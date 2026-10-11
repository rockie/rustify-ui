//! One complete generation of a site, rendered from an analysis in parallel
//! and held in memory until it is committed whole.
//!
//! Each page's cached fragment has its URL slots filled for this generation
//! (a link's page route, a resource's published name), is rendered into the
//! theme's shell, and contributes the classes it uses to the one stylesheet.
//! Work is split across at most `available_parallelism` threads; each page
//! renders under its own reactive owner, and nothing mutable is shared
//! between threads but the queue index.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::content::{Analysis, Diagnostic, Project, Snapshot, Target};
use crate::diagnostic::Severity;
use crate::escape::escape_url_into;
use crate::frontmatter::Layout;
use crate::history::Dates;
use crate::imagesize::Size;
use crate::markdown::RefKind;
use crate::shell::{self, AssetUrls, HeaderLink, Home, Logo, Meta, Page, Site};
use crate::style::{self, Brand};

/// The framework's own scripts, shipped with every site.
pub const MODE_SCRIPT: &str = include_str!("../assets/mode.js");
pub const SITE_SCRIPT: &str = include_str!("../assets/site.js");

/// What `dev` adds to the pages it serves (at [`shell::DEV_SCRIPT_URL`] and
/// [`shell::DEV_STYLE_URL`]); no production build contains either.
pub const DEV_SCRIPT: &str = include_str!("../assets/dev.js");
pub const DEV_STYLE: &str = include_str!("../assets/dev.css");

/// Stands in for the stylesheet's URL until the stylesheet, whose name
/// depends on the classes the pages use, exists.
const STYLESHEET_PLACEHOLDER: &str = "/__rustify_docs_stylesheet__.css";

/// Maps `items` through `f` on up to `workers` threads, keeping order.
pub fn parallel_map<T, R, F>(items: &[T], workers: usize, f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    let workers = workers.clamp(1, items.len().max(1));
    if workers == 1 {
        return items.iter().map(f).collect();
    }
    let next = AtomicUsize::new(0);
    let mut results: Vec<(usize, R)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(i) else {
                            break done;
                        };
                        done.push((i, f(item)));
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| match handle.join() {
                Ok(done) => done,
                Err(panic) => std::panic::resume_unwind(panic),
            })
            .collect()
    });
    results.sort_unstable_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, result)| result).collect()
}

/// The worker count a build uses by default.
pub fn default_workers() -> usize {
    std::thread::available_parallelism().map_or(1, usize::from)
}

/// The first 10 hex digits of `bytes`' SHA-256: a file name's fingerprint.
pub fn fingerprint(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest[..5].iter().map(|b| format!("{b:02x}")).collect()
}

/// Where a route's page is written: `/a/b/` → `a/b/index.html`.
pub fn page_path(route: &str) -> String {
    let trimmed = route.trim_matches('/');
    if trimmed.is_empty() {
        "index.html".into()
    } else {
        format!("{trimmed}/index.html")
    }
}

/// The framework's link-preview image, used when `site.image` is not set:
/// a flat 1200×630 card in the default dark background and primary.
pub const SHARE_IMAGE: &[u8] = include_bytes!("../assets/share.png");

/// What a production build writes beside the pages, to say what it is.
pub const MANIFEST_FILE: &str = "rustify-docs-manifest.json";

/// Files the build writes at the site's root, which no asset may take.
pub const SITE_FILES: &[&str] = &[
    "404.html",
    "sitemap.xml",
    "robots.txt",
    ".nojekyll",
    MANIFEST_FILE,
];

/// Where the build's own fingerprinted files go, which no asset may take.
pub const GENERATED_PREFIXES: &[&str] = &[
    "assets/site.",
    "assets/mode.",
    "assets/share.",
    "assets/files/",
];

/// A site's files as a build would write them, by path under the output
/// directory, plus what it took to make them.
pub struct Generation {
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub stylesheet_path: String,
    /// Pages written, the not-found page included.
    pub pages: usize,
    pub candidates: usize,
}

/// Where a file the pages reference is published: under `assets/files/`,
/// named by its name and its content, so it can be cached for good and a
/// change to it renames it (and changes only the pages that reference it).
pub fn resource_output(path: &str, bytes: &[u8]) -> String {
    let name = path.rsplit('/').next().unwrap_or(path);
    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, Some(extension)),
        _ => (name, None),
    };
    let hash = fingerprint(bytes);
    match extension {
        Some(extension) => format!("assets/files/{stem}.{hash}.{extension}"),
        None => format!("assets/files/{stem}.{hash}"),
    }
}

/// What follows a file's path in a URL as written: its `?query` and
/// `#fragment`, which the URL of the file as published keeps.
fn url_suffix(url: &str) -> &str {
    url.find(['?', '#']).map_or("", |at| &url[at..])
}

/// A page's URL with an anchor on it.
fn page_url(site: &Site, route: &str, anchor: Option<&str>) -> String {
    let mut url = site.url(route);
    if let Some(anchor) = anchor.filter(|anchor| !anchor.is_empty()) {
        url.push('#');
        url.push_str(&crate::escape::encode_fragment(anchor));
    }
    url
}

/// The article of page `index`, with every slot filled in. An image whose
/// tag gives no size gets its file's, so the page reserves its box before it
/// loads.
pub fn article(
    analysis: &Analysis,
    index: usize,
    site: &Site,
    outputs: &BTreeMap<String, String>,
    sizes: &BTreeMap<String, Size>,
) -> String {
    let page = &analysis.pages[index];
    let fragment = &page.fragment;
    let html = &fragment.html;
    let mut slots: Vec<&crate::markdown::Slot> = fragment.slots.iter().collect();
    slots.sort_by_key(|slot| slot.html.start);
    let mut out = String::with_capacity(html.len() + 64);
    let mut at = 0;
    let published = |file: &str| {
        outputs
            .get(file)
            .map(|output| site.url(&format!("/{output}")))
    };
    for slot in slots {
        let reference = &page.fragment.references[slot.reference];
        let url = match &page.targets[slot.reference] {
            Target::Keep => continue,
            Target::Page {
                page: target,
                anchor,
            } => page_url(site, &analysis.pages[*target].route, anchor.as_deref()),
            Target::Resource(path) => {
                let Some(url) = published(path) else {
                    continue;
                };
                let url = url + url_suffix(&reference.destination);
                if reference.kind == RefKind::ClassImage {
                    // Inside `url(…)` in a class, where a bracket or a quote
                    // would end the value.
                    out.push_str(&html[at..slot.html.start]);
                    let url = url
                        .replace('(', "%28")
                        .replace(')', "%29")
                        .replace('\'', "%27");
                    escape_url_into(&mut out, &url);
                    at = slot.html.end;
                    continue;
                }
                let image = reference.kind == RefKind::Image;
                let rest = &html[slot.html.end..];
                let tag = &rest[..rest.find('>').unwrap_or(rest.len())];
                match sizes.get(path) {
                    Some(size)
                        if image
                            && size.absolute
                            && rest.starts_with('"')
                            && !tag.contains(" width=")
                            && !tag.contains(" height=") =>
                    {
                        out.push_str(&html[at..slot.html.start]);
                        escape_url_into(&mut out, &url);
                        out.push_str(&format!(
                            "\" width=\"{}\" height=\"{}",
                            size.width, size.height
                        ));
                        at = slot.html.end;
                        continue;
                    }
                    _ => url,
                }
            }
            Target::Repository(url) => {
                // Written as a path, published as a link to another site:
                // it gets the `rel` such links get, unless it has its own.
                out.push_str(&html[at..slot.html.start]);
                escape_url_into(&mut out, url);
                at = slot.html.end;
                let rest = &html[at..];
                let tag_end = rest.find('>').unwrap_or(rest.len());
                if rest.starts_with('"') && !rest[..tag_end].contains(" rel=") {
                    out.push_str("\" rel=\"noopener");
                }
                continue;
            }
            Target::Srcset(candidates) => {
                // Each URL is encoded on its own: a space separates a URL
                // from its descriptor.
                out.push_str(&html[at..slot.html.start]);
                for (i, candidate) in candidates.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let url = candidate
                        .file
                        .as_deref()
                        .and_then(published)
                        .map(|url| url + url_suffix(&candidate.url))
                        .unwrap_or_else(|| candidate.url.clone());
                    escape_url_into(&mut out, &url);
                    if !candidate.descriptor.is_empty() {
                        out.push(' ');
                        crate::escape::escape_html_into(&mut out, &candidate.descriptor);
                    }
                }
                at = slot.html.end;
                continue;
            }
        };
        out.push_str(&html[at..slot.html.start]);
        escape_url_into(&mut out, &url);
        at = slot.html.end;
    }
    out.push_str(&html[at..]);
    out
}

/// Where a resolved target, written as `written`, points: a URL for this
/// generation, or `written` itself for a URL kept as written.
fn target_url(
    analysis: &Analysis,
    target: &Target,
    written: &str,
    site: &Site,
    outputs: &BTreeMap<String, String>,
) -> String {
    match target {
        Target::Page { page, anchor } => {
            page_url(site, &analysis.pages[*page].route, anchor.as_deref())
        }
        Target::Resource(path) => match outputs.get(path) {
            Some(output) => site.url(&format!("/{output}")) + url_suffix(written),
            None => written.to_owned(),
        },
        Target::Repository(url) => url.clone(),
        Target::Keep | Target::Srcset(_) => written.to_owned(),
    }
}

/// The configuration's header links, resolved.
fn header_links(
    analysis: &Analysis,
    links: &[crate::config::NavLink],
    targets: &[Target],
    site: &Site,
    outputs: &BTreeMap<String, String>,
) -> Vec<HeaderLink> {
    links
        .iter()
        .zip(targets)
        .map(|(link, target)| {
            let written = link.link.trim();
            HeaderLink {
                label: link.label.clone(),
                url: target_url(analysis, target, written, site, outputs),
                route: match target {
                    Target::Page { page, .. } => Some(analysis.pages[*page].route.clone()),
                    _ => None,
                },
                external: crate::markdown::is_external(written)
                    || matches!(target, Target::Repository(_)),
            }
        })
        .collect()
}

/// A `<title>` for every page, unique across the site: `Title | Site`, and
/// where two pages share a title, each is told apart by its parent page's
/// title, or failing that by its address.
fn document_titles(
    analysis: &Analysis,
    published: &[usize],
    site_name: &str,
) -> BTreeMap<usize, String> {
    let plain = |i: usize| {
        let title = &analysis.pages[i].title;
        if title == site_name {
            site_name.to_owned()
        } else {
            format!("{title} | {site_name}")
        }
    };
    let mut by_title: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for &i in published {
        by_title.entry(plain(i)).or_default().push(i);
    }
    let parent_title = |i: usize| {
        let route = analysis.pages[i].route.trim_end_matches('/');
        let parent = match route.rfind('/') {
            Some(cut) => &route[..=cut],
            None => "/",
        };
        match analysis.routes.get(parent) {
            Some(&p) if p != i => analysis.pages[p].title.clone(),
            _ => parent
                .trim_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_owned(),
        }
    };
    let mut titles = BTreeMap::new();
    for (title, pages) in by_title {
        if pages.len() == 1 {
            titles.insert(pages[0], title);
            continue;
        }
        let qualified: Vec<(usize, String)> = pages
            .iter()
            .map(|&i| {
                (
                    i,
                    format!(
                        "{} · {} | {site_name}",
                        analysis.pages[i].title,
                        parent_title(i)
                    ),
                )
            })
            .collect();
        let distinct: BTreeSet<&String> = qualified.iter().map(|(_, title)| title).collect();
        for (i, title) in &qualified {
            let title = if distinct.len() == qualified.len() {
                title.clone()
            } else {
                format!(
                    "{} · {} | {site_name}",
                    analysis.pages[*i].title, analysis.pages[*i].route
                )
            };
            titles.insert(*i, title);
        }
    }
    titles
}

/// The site's sitemap: every published page that may be indexed.
fn sitemap(
    analysis: &Analysis,
    published: &[usize],
    absolute: &dyn Fn(&str) -> String,
    dates: &Dates,
) -> String {
    let mut entries: Vec<(&str, usize)> = published
        .iter()
        .filter(|&&i| !analysis.pages[i].draft && analysis.pages[i].front.index != Some(false))
        .map(|&i| (analysis.pages[i].route.as_str(), i))
        .collect();
    entries.sort();
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for (route, i) in entries {
        xml.push_str("<url><loc>");
        crate::escape::escape_html_into(&mut xml, &absolute(route));
        xml.push_str("</loc>");
        if let Some(date) = dates.get(&analysis.pages[i].file) {
            xml.push_str("<lastmod>");
            xml.push_str(date);
            xml.push_str("</lastmod>");
        }
        xml.push_str("</url>\n");
    }
    xml.push_str("</urlset>\n");
    xml
}

/// Renders every published page of `analysis` and the not-found page,
/// compiles the stylesheet for the classes they use, names the shared files
/// by their content and writes the site's files. Problems in the result
/// (a stylesheet that does not compile, a page that breaks the policy) are
/// errors; nothing partial is returned.
pub fn generate(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    dates: &Dates,
    workers: usize,
) -> Result<Generation, Vec<Diagnostic>> {
    let config = &project.config;
    let mode_path = format!("assets/mode.{}.js", fingerprint(MODE_SCRIPT.as_bytes()));
    let site_path = format!("assets/site.{}.js", fingerprint(SITE_SCRIPT.as_bytes()));
    let mut site = Site {
        name: config.site.name.clone(),
        lang: config.site.lang.clone(),
        base: config.site.base.clone(),
        nav: analysis.nav.clone(),
        top: Vec::new(),
        social: Vec::new(),
        assets: AssetUrls::default(),
        logo: None,
    };
    site.assets = AssetUrls {
        stylesheet: STYLESHEET_PLACEHOLDER.into(),
        mode_script: site.url(&format!("/{mode_path}")),
        site_script: site.url(&format!("/{site_path}")),
        dev: analysis.mode == crate::content::Mode::Dev,
    };
    // `https://example.com/docs/` with base `/docs/` is origin
    // `https://example.com`; a root-relative URL appended to it is absolute.
    let origin = config.site.url.as_ref().map(|url| {
        url.strip_suffix(config.site.base.as_str())
            .unwrap_or(url.trim_end_matches('/'))
            .to_owned()
    });
    let absolute = |root_relative: &str| -> Option<String> {
        origin
            .as_ref()
            .map(|origin| format!("{origin}{root_relative}"))
    };

    // The asset directory, published at its own paths, and every file the
    // pages and the theme reference, published by content.
    let mut files: BTreeMap<String, Arc<[u8]>> = BTreeMap::new();
    let assets_prefix = format!("{}/", config.assets_dir);
    for (path, bytes) in &snapshot.files {
        if let Some(output) = path.strip_prefix(&assets_prefix) {
            if !crate::content::is_private(path) {
                files.insert(output.to_owned(), bytes.clone());
            }
        }
    }
    let mut outputs: BTreeMap<String, String> = BTreeMap::new();
    let mut sizes: BTreeMap<String, Size> = BTreeMap::new();
    for path in &analysis.resources {
        // Read and checked by the analysis; nothing else is read here.
        let Some(bytes) = snapshot
            .files
            .get(path)
            .or_else(|| analysis.outside.get(path))
            .cloned()
        else {
            continue;
        };
        let output = resource_output(path, &bytes);
        if let Some(size) = crate::imagesize::size(&bytes) {
            sizes.insert(path.clone(), size);
        }
        outputs.insert(path.clone(), output.clone());
        files.insert(output, bytes);
    }
    site.logo = config.logo.as_ref().and_then(|logo| {
        let url = |path: &String| {
            outputs
                .get(path)
                .map(|output| site.url(&format!("/{output}")))
        };
        // Shown 24 px high; its width follows from its shape.
        let width = sizes
            .get(&logo.light)
            .map(|size| ((24 * size.width + size.height / 2) / size.height).max(1));
        Some(Logo {
            light: url(&logo.light)?,
            dark: url(&logo.dark)?,
            width,
        })
    });
    // A link preview needs the image's absolute address, so it is shown
    // only when the site's URL is known.
    site.top = header_links(analysis, &config.nav.top, &analysis.top, &site, &outputs);
    site.social = header_links(
        analysis,
        &config.nav.social,
        &analysis.social,
        &site,
        &outputs,
    );
    let share_image = match &config.site.image {
        Some((path, _)) => outputs.get(path).cloned(),
        None if origin.is_some() => {
            let output = format!("assets/share.{}.png", fingerprint(SHARE_IMAGE));
            files.insert(output.clone(), Arc::from(SHARE_IMAGE));
            Some(output)
        }
        None => None,
    }
    .and_then(|output| absolute(&site.url(&format!("/{output}"))));

    let published: Vec<usize> = (0..analysis.pages.len())
        .filter(|&i| analysis.mode == crate::content::Mode::Dev || !analysis.pages[i].draft)
        .collect();
    let titles = document_titles(analysis, &published, &site.name);
    let edit_prefix = config.repo.as_ref().and_then(|repo| {
        let edit = repo.edit_url.as_ref()?;
        let path = if repo.path.is_empty() {
            String::new()
        } else {
            format!("{}/", repo.path)
        };
        Some(format!("{}/{path}", edit.trim_end_matches('/')))
    });
    let rendered: Vec<(String, String, BTreeSet<String>)> =
        parallel_map(&published, workers, |&i| {
            let page = &analysis.pages[i];
            let mut article = article(analysis, i, &site, &outputs, &sizes);
            let home = (page.front.layout == Layout::Home).then(|| {
                // The hero carries the page's one H1; a leading `# Title` in
                // the Markdown is that title, not a second heading.
                if article.starts_with("<h1") {
                    if let Some(end) = article.find("</h1>") {
                        let cut = end + "</h1>".len();
                        let cut = cut + usize::from(article[cut..].starts_with('\n'));
                        article.replace_range(..cut, "");
                    }
                }
                Home {
                    tagline: page.front.tagline.as_deref(),
                    actions: page
                        .front
                        .actions
                        .iter()
                        .zip(&page.action_targets)
                        .map(|(action, target)| {
                            let url =
                                target_url(analysis, target, action.link.trim(), &site, &outputs);
                            (action.text.as_str(), url)
                        })
                        .collect(),
                    features: page
                        .front
                        .features
                        .iter()
                        .map(|feature| (feature.title.as_str(), feature.details.as_str()))
                        .collect(),
                }
            });
            let levels = page
                .front
                .toc
                .levels
                .map_or((2, 3), |[low, high]| (low, high));
            let canonical = absolute(&site.url(&page.route));
            let edit_url = edit_prefix
                .as_ref()
                .map(|prefix| format!("{prefix}{}", page.file));
            let html = shell::render_page(
                &site,
                &Page {
                    route: &page.route,
                    document_title: &titles[&i],
                    title: &page.title,
                    description: page.description.as_deref(),
                    lang: &page.lang,
                    draft: page.draft,
                    article: &article,
                    headings: &page.fragment.headings,
                    article_has_h1: page.fragment.h1_count > 0 || home.is_some(),
                    toc_levels: levels,
                    meta: Meta {
                        canonical: canonical.as_deref(),
                        image: share_image.as_deref(),
                        noindex: page.front.index == Some(false),
                        home: page.route == "/",
                        updated: dates.get(&page.file).map(String::as_str),
                        edit_url: edit_url.as_deref(),
                    },
                    home,
                },
            );
            let mut classes = BTreeSet::new();
            crate::classes::collect(&html, &mut classes);
            (page_path(&page.route), html, classes)
        });
    let not_found = shell::render_not_found(&site);
    let mut candidates: BTreeSet<String> = crate::classes::SCRIPT_STATE_CLASSES
        .iter()
        .map(|class| class.to_string())
        .collect();
    crate::classes::collect(&not_found, &mut candidates);
    for (_, _, classes) in &rendered {
        candidates.extend(classes.iter().cloned());
    }
    let brand = match &config.primary {
        Some(primary) => Brand {
            primary_light: primary.light.clone(),
            primary_dark: primary.dark.clone(),
        },
        None => Brand::default(),
    };
    let css = style::site_css(&candidates, &brand).map_err(|error| {
        vec![Diagnostic::general(
            Severity::Error,
            "stylesheet",
            error.to_string(),
        )]
    })?;
    // Check the actual generation, rather than a path prefix that a
    // missing file could share. Document fragments were checked per page.
    let published_urls: BTreeSet<String> = files
        .keys()
        .map(|path| site.url(&format!("/{path}")))
        .collect();
    let fragments: BTreeSet<&str> = analysis
        .published()
        .flat_map(|page| &page.fragment.references)
        .filter(|reference| {
            reference.kind == RefKind::ClassImage && reference.destination.starts_with('#')
        })
        .map(|reference| reference.destination.as_str())
        .collect();
    let strays: Vec<Diagnostic> = style::urls(&css)
        .into_iter()
        .filter(|url| {
            let path = url.split(['?', '#']).next().unwrap_or_default();
            !url.to_ascii_lowercase().starts_with("data:")
                && !fragments.contains(url.as_str())
                && !published_urls.contains(path)
        })
        .map(|url| {
            Diagnostic::general(
                Severity::Error,
                "output-check",
                format!("the generated stylesheet loads `{url}`, which is not a file the site publishes"),
            )
        })
        .collect();
    if !strays.is_empty() {
        return Err(strays);
    }
    let stylesheet_path = format!("assets/site.{}.css", fingerprint(css.as_bytes()));
    let stylesheet_url = site.url(&format!("/{stylesheet_path}"));
    let mut pages: Vec<(String, String)> = rendered
        .into_iter()
        .map(|(path, html, _)| (path, html))
        .collect();
    pages.push(("404.html".to_owned(), not_found));
    let pages: Vec<(String, Arc<[u8]>, Vec<String>)> =
        parallel_map(&pages, workers, |(path, html)| {
            let html = html.replacen(STYLESHEET_PLACEHOLDER, &stylesheet_url, 1);
            let problems = crate::html_check::check_document(&html);
            (path.clone(), Arc::from(html.into_bytes()), problems)
        });
    let mut problems = Vec::new();
    let page_count = pages.len();
    for (path, bytes, found) in pages {
        for problem in found {
            problems.push(Diagnostic::in_file(
                &path,
                Severity::Error,
                "output-check",
                format!("the generated page has {problem}"),
            ));
        }
        files.insert(path, bytes);
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    files.insert(stylesheet_path.clone(), Arc::from(css.into_bytes()));
    files.insert(mode_path, Arc::from(MODE_SCRIPT.as_bytes()));
    files.insert(site_path, Arc::from(SITE_SCRIPT.as_bytes()));

    // The files a host and its crawlers look for at the root.
    let absolute_route = |route: &str| absolute(&site.url(route)).unwrap_or_default();
    let mut robots = String::from("User-agent: *\nAllow: /\n");
    if origin.is_some() {
        let xml = sitemap(analysis, &published, &absolute_route, dates);
        files.insert("sitemap.xml".to_owned(), Arc::from(xml.into_bytes()));
        robots.push_str(&format!("\nSitemap: {}\n", absolute_route("/sitemap.xml")));
    }
    files.insert("robots.txt".to_owned(), Arc::from(robots.into_bytes()));
    // GitHub Pages runs Jekyll on a branch unless told not to.
    files.insert(".nojekyll".to_owned(), Arc::from(&b""[..]));
    Ok(Generation {
        files,
        stylesheet_path,
        pages: page_count,
        candidates: candidates.len(),
    })
}

/// Where [`dev_shell`] puts its page.
pub const DEV_SHELL_PAGE: &str = "index.html";

/// What `dev` serves while nothing has built: the site's shell saying so
/// (at [`DEV_SHELL_PAGE`]) with the stylesheet and scripts it needs, under
/// the configuration's base and in its language and colours.
pub fn dev_shell(project: &Project) -> Result<Generation, Vec<Diagnostic>> {
    let config = &project.config;
    let mode_path = format!("assets/mode.{}.js", fingerprint(MODE_SCRIPT.as_bytes()));
    let site_path = format!("assets/site.{}.js", fingerprint(SITE_SCRIPT.as_bytes()));
    let mut site = Site {
        name: config.site.name.clone(),
        lang: config.site.lang.clone(),
        base: config.site.base.clone(),
        nav: Vec::new(),
        top: Vec::new(),
        social: Vec::new(),
        assets: AssetUrls::default(),
        logo: None,
    };
    site.assets = AssetUrls {
        stylesheet: STYLESHEET_PLACEHOLDER.into(),
        mode_script: site.url(&format!("/{mode_path}")),
        site_script: site.url(&format!("/{site_path}")),
        dev: true,
    };
    let html = shell::render_dev_shell(&site);
    let mut candidates: BTreeSet<String> = crate::classes::SCRIPT_STATE_CLASSES
        .iter()
        .map(|class| class.to_string())
        .collect();
    crate::classes::collect(&html, &mut candidates);
    let brand = match &config.primary {
        Some(primary) => Brand {
            primary_light: primary.light.clone(),
            primary_dark: primary.dark.clone(),
        },
        None => Brand::default(),
    };
    let css = style::site_css(&candidates, &brand).map_err(|error| {
        vec![Diagnostic::general(
            Severity::Error,
            "stylesheet",
            error.to_string(),
        )]
    })?;
    let stylesheet_path = format!("assets/site.{}.css", fingerprint(css.as_bytes()));
    let html = html.replacen(
        STYLESHEET_PLACEHOLDER,
        &site.url(&format!("/{stylesheet_path}")),
        1,
    );
    let mut files: BTreeMap<String, Arc<[u8]>> = BTreeMap::new();
    files.insert(DEV_SHELL_PAGE.to_owned(), Arc::from(html.into_bytes()));
    files.insert(stylesheet_path.clone(), Arc::from(css.into_bytes()));
    files.insert(mode_path, Arc::from(MODE_SCRIPT.as_bytes()));
    files.insert(site_path, Arc::from(SITE_SCRIPT.as_bytes()));
    Ok(Generation {
        files,
        stylesheet_path,
        pages: 1,
        candidates: candidates.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_map_keeps_order_on_any_worker_count() {
        let items: Vec<u32> = (0..257).collect();
        for workers in [1, 2, 7, 64] {
            assert_eq!(
                parallel_map(&items, workers, |x| x * 2),
                items.iter().map(|x| x * 2).collect::<Vec<_>>()
            );
        }
        assert!(parallel_map(&[] as &[u32], 4, |x| *x).is_empty());
    }

    #[test]
    fn page_paths() {
        assert_eq!(page_path("/"), "index.html");
        assert_eq!(page_path("/guide/setup/"), "guide/setup/index.html");
    }
}
