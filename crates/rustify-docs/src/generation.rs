//! One complete generation of a site, rendered from an analysis in parallel
//! and held in memory until it is committed whole.
//!
//! Each page's cached fragment has its URL slots filled for this generation
//! (a link's page route, a resource's published name), is rendered into the
//! theme's shell, and contributes the classes it uses to the one stylesheet.
//! Work is split across at most `available_parallelism` threads; each page
//! renders under its own reactive owner, and nothing mutable is shared
//! between threads but the queue index.

use crate::diagnostic_text::{code_span, literal};
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
use crate::landing::{Profile, ScriptSrc};
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
    "llms.txt",
    ".nojekyll",
    MANIFEST_FILE,
];

/// Where the build's own fingerprinted files go, which no asset may take.
pub const GENERATED_PREFIXES: &[&str] = &[
    "assets/site.",
    "assets/mode.",
    "assets/share.",
    "assets/files/",
    "assets/search.",
    "assets/search-worker.",
];

/// A site's files as a build would write them, by path under the output
/// directory, plus what it took to make them.
pub struct Generation {
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub locale_not_found: BTreeMap<String, String>,
    /// The header policy of each page served under another than
    /// [`shell::CSP`], by its path under the output directory.
    pub page_policies: BTreeMap<String, String>,
    pub stylesheet_path: String,
    /// Pages written, the not-found page included.
    pub pages: usize,
    pub candidates: usize,
}

/// Results and immutable assets from the site's registered Rust renderers.
/// Content-only callers keep using [`generate`] and never need this value.
#[derive(Default)]
pub struct Extensions {
    pub articles: BTreeMap<String, String>,
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub candidates: BTreeSet<String>,
    pub css: String,
    /// Optional validated @theme mappings consumed by the utility compiler.
    pub theme_utilities: String,
    /// Native CSS projected after the reading theme's incumbent defaults.
    pub theme_css: String,
    /// A project-relative resource path, loaded only by pages with islands.
    pub island_script: Option<String>,
    /// Routes where registered views produced an interactive island.
    pub island_routes: BTreeSet<String>,
}

/// Resolves Markdown child URLs before the one batch of Rust views runs.
pub fn resolved_articles(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
) -> BTreeMap<String, String> {
    let site = link_site(project);
    let mut outputs = BTreeMap::new();
    let mut sizes = BTreeMap::new();
    for path in &analysis.resources {
        if let Some(bytes) = snapshot
            .files
            .get(path)
            .or_else(|| analysis.outside.get(path))
        {
            outputs.insert(path.clone(), resource_output(path, bytes));
            if let Some(size) = crate::imagesize::size(bytes) {
                sizes.insert(path.clone(), size);
            }
        }
    }
    analysis
        .pages
        .iter()
        .enumerate()
        .map(|(index, page)| {
            (
                page.route.clone(),
                article(analysis, index, &site, &outputs, &sizes),
            )
        })
        .collect()
}

fn link_site(project: &Project) -> Site {
    Site {
        name: project.config.site.name.clone(),
        lang: project.config.site.lang.clone(),
        home_route: "/".into(),
        catalog: Arc::new(crate::labels::resolve(
            &project.config.site.lang,
            &BTreeMap::new(),
        )),
        base: project.config.site.base.clone(),
        nav: Vec::new(),
        top: Vec::new(),
        social: Vec::new(),
        assets: AssetUrls::default(),
        logo: None,
    }
}

pub(crate) fn site_origin(project: &Project) -> Option<String> {
    project.config.site.url.as_ref().map(|url| {
        url.strip_suffix(project.config.site.base.as_str())
            .unwrap_or(url.trim_end_matches('/'))
            .to_owned()
    })
}

pub(crate) fn source_fragment(
    project: &Project,
    analysis: &Analysis,
    fragment: &crate::markdown::Fragment,
    targets: &[Target],
) -> String {
    // Resource paths stay project-relative until the enclosing surface enters
    // the common resource graph; page and repository links are already resolved.
    resolve_fragment(
        analysis,
        fragment,
        targets,
        &link_site(project),
        |file| Some(file.to_owned()),
        &BTreeMap::new(),
    )
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

/// A page's URL with an optional query and anchor on it.
fn page_url(site: &Site, route: &str, query: Option<&str>, anchor: Option<&str>) -> String {
    let mut url = site.url(route);
    if let Some(query) = query.filter(|query| !query.is_empty()) {
        url.push('?');
        url.push_str(query);
    }
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
    resolve_fragment(
        analysis,
        &page.fragment,
        &page.targets,
        site,
        |file| {
            outputs
                .get(file)
                .map(|output| site.url(&format!("/{output}")))
        },
        sizes,
    )
}

/// Resolves the same checked URL slots for an article or a layout surface.
fn resolve_fragment(
    analysis: &Analysis,
    fragment: &crate::markdown::Fragment,
    targets: &[Target],
    site: &Site,
    published: impl Fn(&str) -> Option<String>,
    sizes: &BTreeMap<String, Size>,
) -> String {
    let html = &fragment.html;
    let mut slots: Vec<&crate::markdown::Slot> = fragment.slots.iter().collect();
    slots.sort_by_key(|slot| slot.html.start);
    let mut out = String::with_capacity(html.len() + 64);
    let mut at = 0;
    for slot in slots {
        let reference = &fragment.references[slot.reference];
        let url = match &targets[slot.reference] {
            Target::Keep => continue,
            Target::Page {
                page: target,
                anchor,
            } => page_url(
                site,
                &analysis.pages[*target].route,
                None,
                anchor.as_deref(),
            ),
            Target::Form {
                page,
                query,
                anchor,
            } => page_url(
                site,
                &analysis.pages[*page].route,
                query.as_deref(),
                anchor.as_deref(),
            ),
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
                    let mut encoded = String::new();
                    escape_url_into(&mut encoded, &url);
                    // A rewritten file name is literal, even in functions
                    // where the utility compiler treats '_' as a space.
                    out.push_str(&encoded.replace('_', "\\_"));
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
                        .and_then(&published)
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

fn layout_slots(
    analysis: &Analysis,
    route: &str,
    site: &Site,
    outputs: &BTreeMap<String, String>,
    sizes: &BTreeMap<String, Size>,
) -> shell::LayoutSlots {
    analysis
        .slots
        .get(route)
        .into_iter()
        .flat_map(|slots| slots.iter())
        .map(|(slot, content)| {
            (
                *slot,
                resolve_fragment(
                    analysis,
                    &content.fragment,
                    &content.targets,
                    site,
                    |file| {
                        outputs
                            .get(file)
                            .map(|output| site.url(&format!("/{output}")))
                    },
                    sizes,
                ),
            )
        })
        .collect()
}

/// Source literals remain class selectors when a hidden browser view appears.
fn resolve_candidate_css(
    css: &mut String,
    analysis: &Analysis,
    site: &Site,
    outputs: &BTreeMap<String, String>,
) {
    if analysis.candidate_urls.is_empty() {
        return;
    }
    for (written, range) in style::url_ranges(css, false).into_iter().rev() {
        let Some(Target::Resource(path)) = analysis.candidate_urls.get(&written) else {
            continue;
        };
        let Some(output) = outputs.get(path) else {
            continue;
        };
        let url = site.url(&format!("/{output}")) + url_suffix(&written);
        let mut escaped = String::new();
        for character in url.chars() {
            if character.is_ascii_whitespace() || matches!(character, '\'' | '"' | '(' | ')' | '\\')
            {
                escaped.push_str(&format!("%{:02X}", character as u32));
            } else {
                escaped.push(character);
            }
        }
        css.replace_range(range, &escaped);
    }
}

fn island_module(html: &mut String, route: &str, site: &Site, extensions: &Extensions) {
    if !extensions.island_routes.contains(route) {
        return;
    }
    if let Some(script) = &extensions.island_script {
        let mut tag = String::from("<script type=\"module\" data-rd-islands-module src=\"");
        escape_url_into(&mut tag, &site.url(&format!("/{script}")));
        tag.push_str("\"></script>");
        *html = html.replacen("</body>", &format!("{tag}</body>"), 1);
    }
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
            page_url(site, &analysis.pages[*page].route, None, anchor.as_deref())
        }
        Target::Form {
            page,
            query,
            anchor,
        } => page_url(
            site,
            &analysis.pages[*page].route,
            query.as_deref(),
            anchor.as_deref(),
        ),
        Target::Resource(path) => match outputs.get(path) {
            Some(output) => site.url(&format!("/{output}")) + url_suffix(written),
            None => written.to_owned(),
        },
        Target::Repository(url) => url.clone(),
        Target::Keep | Target::Srcset(_) => written.to_owned(),
    }
}

/// The profile a landing page loads under; `None` for every other page.
fn landing_profile<'a>(
    config: &'a crate::config::Config,
    page: &crate::content::Page,
) -> Option<&'a Profile> {
    if page.front.layout != Layout::Landing {
        return None;
    }
    config.landing.get(page.front.landing.as_deref()?)
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
                lang: site.lang.clone(),
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

/// A `<title>` for every page, unique within its configured language: `Title | Site`, and
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
    let mut by_title: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for &i in published {
        let lang = if analysis.locales.is_empty() {
            String::new()
        } else {
            analysis.pages[i].lang.clone()
        };
        by_title.entry((lang, plain(i))).or_default().push(i);
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
    for ((_, title), pages) in by_title {
        if pages.len() == 1
            || pages.iter().any(|&i| {
                matches!(
                    analysis.pages[i].origin,
                    crate::content::PageOrigin::Registered(_)
                )
            })
        {
            for i in pages {
                titles.insert(i, title.clone());
            }
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

fn title_diagnostics(
    analysis: &Analysis,
    titles: &BTreeMap<usize, String>,
    site: &Site,
) -> Vec<Diagnostic> {
    let mut groups = BTreeMap::<(&str, &str), Vec<usize>>::new();
    for (&i, title) in titles {
        let lang = if analysis.locales.is_empty() {
            ""
        } else {
            analysis.pages[i].lang.as_str()
        };
        groups.entry((lang, title)).or_default().push(i);
    }
    groups.into_iter().filter_map(|((_, title), pages)| {
        let catalog = &analysis.catalogs[&analysis.pages[pages[0]].lang];
        let not_found = format!("{} | {}", catalog.get("not_found_title").text, site.name);
        let registered = pages.iter().find(|&&i| matches!(analysis.pages[i].origin, crate::content::PageOrigin::Registered(_)))?;
        if pages.len() == 1 && title != not_found { return None; }
        let routes = pages.iter().map(|&i| literal(&analysis.pages[i].route)).collect::<Vec<_>>().join(", ");
        Some(Diagnostic::in_file(&analysis.pages[*registered].file, Severity::Error,
            "document-title", format!("registered page document title {} is duplicated by {routes}{}; give the page a unique title", code_span(title), if title == not_found { " and the generated not-found page" } else { "" })))
    }).collect()
}

fn insert_output(
    files: &mut BTreeMap<String, Arc<[u8]>>,
    path: String,
    bytes: Arc<[u8]>,
) -> Result<(), Vec<Diagnostic>> {
    match files.entry(path) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(bytes);
            Ok(())
        }
        std::collections::btree_map::Entry::Occupied(entry) => Err(vec![Diagnostic::in_file(
            entry.key(),
            Severity::Error,
            "output-conflict",
            "two generated or authored files have the same published path",
        )]),
    }
}

/// Check the actual document trees and the entire output namespace together.
fn final_documents(
    files: &BTreeMap<String, Arc<[u8]>>,
    sources: &BTreeMap<String, String>,
    site: &Site,
    origin: Option<&str>,
    workers: usize,
) -> Vec<Diagnostic> {
    let mut problems = Vec::new();
    for path in files.keys() {
        let mut parent = path.as_str();
        while let Some((prefix, _)) = parent.rsplit_once('/') {
            if files.contains_key(prefix) {
                problems.push(Diagnostic::in_file(
                    sources.get(path).map_or(path.as_str(), String::as_str),
                    Severity::Error,
                    "output-conflict",
                    format!(
                        "published file {} is also a directory required by {}",
                        code_span(prefix),
                        code_span(path)
                    ),
                ));
            }
            parent = prefix;
        }
    }
    let mut urls = BTreeMap::new();
    for path in files.keys() {
        urls.insert(
            crate::content::percent_decode(&site.url(&format!("/{path}"))),
            path.as_str(),
        );
        if let Some(directory) = path.strip_suffix("index.html") {
            let route = crate::content::percent_decode(&site.url(&format!("/{directory}")));
            urls.insert(route.trim_end_matches('/').to_owned(), path.as_str());
            urls.insert(route, path.as_str());
        }
    }
    let pages: Vec<_> = sources.iter().collect();
    let documents = parallel_map(&pages, workers, |&(path, source)| {
        let html = String::from_utf8_lossy(&files[path]);
        let (ids, slots) = crate::html_check::document_references(&html);
        (path, source, ids, slots)
    });
    let mut ids = BTreeMap::new();
    for (path, source, anchors, _) in &documents {
        let mut found = BTreeSet::new();
        for id in anchors {
            if !found.insert(id.clone()) {
                problems.push(Diagnostic::in_file(
                    source,
                    Severity::Error,
                    "output-check",
                    format!(
                        "generated {} has duplicate document id {}",
                        code_span(path),
                        code_span(&format!("#{id}"))
                    ),
                ));
            }
        }
        ids.insert(path.as_str(), found);
    }
    for (path, source, _, slots) in documents {
        for slot in slots {
            // A form submits to an endpoint, not to a file the site publishes;
            // a landing page's own targets were checked with its page.
            if slot.role == crate::html_check::UrlRole::FormTarget {
                continue;
            }
            let references = if slot.role == crate::html_check::UrlRole::Srcset {
                crate::html_check::parse_srcset(&slot.value)
                    .into_iter()
                    .map(|(url, _)| url)
                    .collect()
            } else {
                vec![slot.value]
            };
            for url in references {
                let url = url.trim();
                let url = if slot.element == "link" {
                    origin
                        .and_then(|origin| url.strip_prefix(origin))
                        .filter(|path| path.starts_with('/'))
                        .unwrap_or(url)
                } else {
                    url
                };
                if url.starts_with("//") || crate::content::has_scheme(url) {
                    continue;
                }
                let (location, fragment) = url
                    .split_once('#')
                    .map_or((url, None), |(path, anchor)| (path, Some(anchor)));
                let location = location.split('?').next().unwrap_or_default();
                let target = if location.is_empty() {
                    Some(path.as_str())
                } else {
                    let location = crate::content::percent_decode(location);
                    let absolute = if location.starts_with('/') {
                        Some(location)
                    } else {
                        let current = site.url(&format!("/{path}"));
                        let directory = current.rsplit_once('/').map_or("", |(dir, _)| dir);
                        crate::content::join(directory, &location).map(|path| format!("/{path}"))
                    };
                    absolute
                        .as_ref()
                        .and_then(|absolute| urls.get(absolute).copied())
                };
                let missing = match target {
                    None => Some(format!(
                        "references {}, which is not a file or route the site publishes",
                        code_span(url)
                    )),
                    Some(target) => {
                        fragment
                            .filter(|anchor| !anchor.is_empty())
                            .and_then(|anchor| {
                                let anchor = crate::content::percent_decode(anchor);
                                ids.get(target)
                                    .filter(|ids| !ids.contains(&anchor))
                                    .map(|_| {
                                        format!(
                                            "references {}, but the final {} document has no id {}",
                                            code_span(url),
                                            code_span(target),
                                            code_span(&format!("#{anchor}"))
                                        )
                                    })
                            })
                    }
                };
                if let Some(missing) = missing {
                    problems.push(Diagnostic::in_file(
                        source,
                        Severity::Error,
                        "output-check",
                        format!("generated {} {missing}", code_span(path)),
                    ));
                }
            }
        }
    }
    problems
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
        .filter(|&&i| {
            !analysis.pages[i].draft
                && !analysis.pages[i].locale.is_fallback
                && analysis.pages[i].front.indexed()
        })
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
    generate_extended(
        project,
        snapshot,
        analysis,
        dates,
        workers,
        &Extensions::default(),
    )
}

fn not_found_body(html: &str) -> String {
    let body = html
        .find("<body")
        .and_then(|start| html[start..].find('>').map(|offset| start + offset + 1))
        .unwrap_or(0);
    let end = html.rfind("</body>").unwrap_or(html.len());
    let mut body = html[body..end].to_owned();
    while let Some(start) = body.find("<script") {
        let Some(close) = body[start..].find("</script>") else {
            break;
        };
        body.replace_range(start..start + close + "</script>".len(), "");
    }
    body
}

/// Merges registered views into the ordinary shell and final policy checks.
pub fn generate_extended(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    dates: &Dates,
    workers: usize,
    extensions: &Extensions,
) -> Result<Generation, Vec<Diagnostic>> {
    let config = &project.config;
    let mode_path = format!("assets/mode.{}.js", fingerprint(MODE_SCRIPT.as_bytes()));
    let site_path = format!("assets/site.{}.js", fingerprint(SITE_SCRIPT.as_bytes()));
    let mut site = Site {
        name: config.site.name.clone(),
        lang: config.site.lang.clone(),
        home_route: "/".into(),
        catalog: analysis
            .catalogs
            .iter()
            .find(|(lang, _)| lang.eq_ignore_ascii_case(&config.site.lang))
            .map(|(_, catalog)| catalog.clone())
            .unwrap_or_else(|| {
                Arc::new(crate::labels::resolve(&config.site.lang, &BTreeMap::new()))
            }),
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
    let origin = site_origin(project);
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
            if !crate::content::is_private(path)
                && !crate::content::is_theme_document(project, path)
            {
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
    for (path, bytes) in &extensions.files {
        insert_output(&mut files, path.clone(), bytes.clone())?;
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
    let mut problems = title_diagnostics(analysis, &titles, &site);
    let edit_prefix = config.repo.as_ref().and_then(|repo| {
        let edit = repo.edit_url.as_ref()?;
        let path = if repo.path.is_empty() {
            String::new()
        } else {
            format!("{}/", repo.path)
        };
        Some(format!("{}/{path}", edit.trim_end_matches('/')))
    });
    let localized_sites: BTreeMap<_, _> = analysis
        .catalogs
        .iter()
        .map(|(lang, catalog)| {
            let mut localized = site.clone();
            localized.lang = lang.clone();
            localized.catalog = catalog.clone();
            if let Some(view) = analysis.locales.get(lang) {
                localized.home_route = view.root_route.clone();
                localized.nav = view.nav.clone();
                let nav = config
                    .locales
                    .iter()
                    .find(|locale| locale.lang == *lang)
                    .and_then(|locale| locale.nav.as_ref())
                    .unwrap_or(&config.nav);
                localized.top = header_links(analysis, &nav.top, &view.top, &localized, &outputs);
                localized.social =
                    header_links(analysis, &nav.social, &view.social, &localized, &outputs);
                let label_lang = if config
                    .locales
                    .iter()
                    .any(|locale| locale.lang == *lang && locale.nav.is_some())
                {
                    lang
                } else {
                    &config.site.lang
                };
                for link in localized.top.iter_mut().chain(&mut localized.social) {
                    link.lang = label_lang.clone();
                }
            }
            (lang.clone(), localized)
        })
        .collect();
    let rendered: Vec<(String, String, BTreeSet<String>)> =
        parallel_map(&published, workers, |&i| {
            let page = &analysis.pages[i];
            let site = &localized_sites[&page.lang];
            let language_links: Vec<_> = config
                .locales
                .iter()
                .filter_map(|locale| {
                    let &target = analysis.locales[&locale.lang]
                        .pages
                        .get(&page.locale.logical_route)?;
                    let target = &analysis.pages[target];
                    Some(shell::LanguageLink {
                        lang: locale.lang.clone(),
                        label: locale.label.clone(),
                        href: site.url(&target.route),
                        is_fallback: target.locale.is_fallback,
                        fallback_label: analysis.catalogs[&locale.lang].get("untranslated").clone(),
                        anchors: target
                            .fragment
                            .anchors
                            .iter()
                            .cloned()
                            .chain(
                                analysis
                                    .slots
                                    .get(&target.route)
                                    .into_iter()
                                    .flat_map(|slots| slots.values())
                                    .flat_map(|slot| slot.fragment.anchors.iter().cloned()),
                            )
                            .collect::<BTreeSet<_>>()
                            .into_iter()
                            .collect(),
                    })
                })
                .collect();
            let mut article = extensions
                .articles
                .get(&page.route)
                .cloned()
                .unwrap_or_else(|| article(analysis, i, site, &outputs, &sizes));
            let custom_home = page.locale.logical_route == "/"
                && matches!(page.origin, crate::content::PageOrigin::Registered(_));
            let profile = landing_profile(config, page);
            let carry = profile.map_or(&[][..], |profile| &profile.carry);
            let landing = (page.front.layout == Layout::Landing).then(|| shell::Landing {
                meta_policy: crate::landing::meta_policy(profile),
                scripts: profile
                    .into_iter()
                    .flat_map(|profile| &profile.scripts)
                    .filter_map(|script| {
                        let url = match &script.src {
                            ScriptSrc::Local(path) => site.url(&format!("/{}", outputs.get(path)?)),
                            ScriptSrc::External(src) => src.clone(),
                        };
                        Some((url, script.load))
                    })
                    .collect(),
                carry,
                footer: profile
                    .map(|profile| {
                        header_links(
                            analysis,
                            &profile.footer,
                            &analysis.landing_footers[&profile.name],
                            site,
                            &outputs,
                        )
                    })
                    .unwrap_or_default(),
            });
            let home = (page.front.layout.has_hero() || custom_home).then(|| {
                if custom_home {
                    return Home {
                        custom: true,
                        ..Home::default()
                    };
                }
                // The hero carries the page's one H1; a leading `# Title` in
                // the Markdown is that title, not a second heading.
                if page.fragment.leading_h1.is_some() {
                    if let Some(cut) = crate::html_check::leading_h1_end(&article) {
                        let cut = cut + usize::from(article[cut..].starts_with('\n'));
                        article.replace_range(..cut, "");
                    }
                }
                Home {
                    custom: false,
                    heading_id: page
                        .fragment
                        .leading_h1
                        .as_deref()
                        .filter(|id| !id.is_empty()),
                    tagline: page.front.tagline.as_deref(),
                    actions: page
                        .front
                        .actions
                        .iter()
                        .zip(&page.action_targets)
                        .map(|(action, target)| {
                            let url =
                                target_url(analysis, target, action.link.trim(), site, &outputs);
                            (action.text.as_str(), url)
                        })
                        .collect(),
                    features: page
                        .front
                        .features
                        .iter()
                        .map(|feature| (feature.title.as_str(), feature.details.as_str()))
                        .collect(),
                    carry: !carry.is_empty(),
                }
            });
            let levels = page
                .front
                .toc
                .levels
                .map_or((2, 3), |[low, high]| (low, high));
            let canonical = absolute(&site.url(&page.locale.source_route));
            let alternates: Vec<_> = if origin.is_some() && !analysis.locales.is_empty() {
                analysis
                    .locales
                    .values()
                    .filter_map(|view| view.pages.get(&page.locale.logical_route))
                    .map(|&i| &analysis.pages[i])
                    .filter(|target| {
                        !target.locale.is_fallback && !target.draft && target.front.indexed()
                    })
                    .map(|target| shell::AlternateLink {
                        lang: target.lang.clone(),
                        href: absolute(&site.url(&target.route)).unwrap_or_default(),
                    })
                    .collect()
            } else {
                Vec::new()
            };
            let edit_url = edit_prefix
                .as_ref()
                .map(|prefix| format!("{prefix}{}", page.file));
            let slots = layout_slots(analysis, &page.route, site, &outputs, &sizes);
            let mut html = shell::render_page_with_slots(
                site,
                &Page {
                    route: &page.route,
                    document_title: &titles[&i],
                    title: &page.title,
                    description: page.description.as_deref(),
                    lang: &page.lang,
                    source_lang: &page.locale.source_lang,
                    language_links: &language_links,
                    draft: page.draft,
                    article: &article,
                    headings: &page.fragment.headings,
                    article_has_h1: page.fragment.h1_count > 0
                        || home.as_ref().is_some_and(|home| !home.custom),
                    toc_levels: levels,
                    meta: Meta {
                        canonical: canonical.as_deref(),
                        image: share_image.as_deref(),
                        noindex: !page.front.indexed() || page.locale.is_fallback,
                        home: page.locale.logical_route == "/",
                        updated: dates.get(&page.file).map(String::as_str),
                        edit_url: edit_url.as_deref(),
                        alternates: &alternates,
                    },
                    home,
                    landing,
                },
                &slots,
            );
            island_module(&mut html, &page.route, site, extensions);
            let mut classes = BTreeSet::new();
            crate::classes::collect(&html, &mut classes);
            (page_path(&page.route), html, classes)
        });
    let homes: Vec<_> = config
        .locales
        .iter()
        .map(|locale| shell::LanguageLink {
            lang: locale.lang.clone(),
            label: locale.label.clone(),
            href: site.url(&analysis.locales[&locale.lang].root_route),
            is_fallback: false,
            fallback_label: analysis.catalogs[&locale.lang].get("untranslated").clone(),
            anchors: Vec::new(),
        })
        .collect();
    let mut locale_not_found = BTreeMap::new();
    let mut not_found_pages = BTreeMap::new();
    if config.locales.is_empty() {
        let slots = layout_slots(analysis, "/404/", &site, &outputs, &sizes);
        let mut html = shell::render_not_found_with_slots(&site, &slots);
        island_module(&mut html, "/404/", &site, extensions);
        not_found_pages.insert("404.html".to_owned(), html);
    } else {
        for locale in &config.locales {
            let site = &localized_sites[&locale.lang];
            let route = crate::content::locale_route(config, &locale.lang, "/404/");
            let slots = layout_slots(analysis, &route, site, &outputs, &sizes);
            let mut html = shell::render_not_found_with_languages(site, &slots, &homes);
            html = html.replacen("data-rd-base=", "data-rd-not-found=\"\" data-rd-base=", 1);
            island_module(&mut html, &route, site, extensions);
            let path = if locale.prefix.is_empty() {
                "404.html".into()
            } else {
                format!("{}/404.html", locale.prefix)
            };
            locale_not_found.insert(locale.prefix.clone(), path.clone());
            not_found_pages.insert(path, html);
        }
        let mut templates = String::new();
        for (prefix, path) in &locale_not_found {
            if prefix.is_empty() {
                continue;
            }
            let locale = config
                .locales
                .iter()
                .find(|locale| locale.prefix == *prefix)
                .expect("validated language prefix");
            let catalog = &analysis.catalogs[&locale.lang];
            let words: BTreeMap<_, _> = catalog
                .iter()
                .filter_map(|(key, value)| key.strip_prefix("dev.").map(|key| (key, &value.text)))
                .collect();
            let languages: BTreeMap<_, _> = catalog
                .iter()
                .filter_map(|(key, value)| key.strip_prefix("dev.").map(|key| (key, &value.lang)))
                .collect();
            templates.push_str("<template");
            for (name, value) in [
                ("data-rd-not-found-prefix", prefix.clone()),
                ("data-rd-not-found-lang", locale.lang.clone()),
                (
                    "data-rd-not-found-title",
                    format!("{} | {}", catalog.get("not_found_title").text, site.name),
                ),
                (
                    "data-rd-dev-labels",
                    serde_json::to_string(&words).unwrap_or_default(),
                ),
                (
                    "data-rd-dev-label-langs",
                    serde_json::to_string(&languages).unwrap_or_default(),
                ),
            ] {
                templates.push_str(&format!(" {name}=\""));
                crate::escape::escape_html_into(&mut templates, &value);
                templates.push('"');
            }
            templates.push('>');
            templates.push_str(&not_found_body(&not_found_pages[path]));
            templates.push_str("</template>");
        }
        if let Some(mut root) = not_found_pages.remove("404.html") {
            if !root.contains("data-rd-islands-module") {
                if let Some(route) = config
                    .locales
                    .iter()
                    .map(|locale| crate::content::locale_route(config, &locale.lang, "/404/"))
                    .find(|route| extensions.island_routes.contains(route))
                {
                    island_module(&mut root, &route, &site, extensions);
                }
            }
            if let Some(at) = root.rfind("</body>") {
                root.insert_str(at, &templates);
            }
            not_found_pages.insert("404.html".into(), root);
        }
    }
    let mut candidates: BTreeSet<String> = crate::classes::SCRIPT_STATE_CLASSES
        .iter()
        .map(|class| class.to_string())
        .collect();
    for html in not_found_pages.values() {
        crate::classes::collect(html, &mut candidates);
    }
    for (_, _, classes) in &rendered {
        candidates.extend(classes.iter().cloned());
    }
    candidates.extend(extensions.candidates.iter().cloned());
    let brand = match &config.primary {
        Some(primary) => Brand {
            primary_light: primary.light.clone(),
            primary_dark: primary.dark.clone(),
        },
        None => Brand::default(),
    };
    let mut css = style::site_css_with_theme(
        &candidates,
        &brand,
        &extensions.theme_utilities,
        &extensions.theme_css,
    )
    .map_err(|error| {
        vec![Diagnostic::general(
            Severity::Error,
            "stylesheet",
            error.to_string(),
        )]
    })?;
    css.push_str(&extensions.css);
    resolve_candidate_css(&mut css, analysis, &site, &outputs);
    // Check the actual generation, rather than a path prefix that a
    // missing file could share. Document fragments were checked per page.
    let published_urls: BTreeSet<String> = files
        .keys()
        .map(|path| site.url(&format!("/{path}")))
        .collect();
    let fragments: BTreeSet<&str> = analysis
        .published()
        .map(|page| page.fragment.as_ref())
        .chain(
            analysis
                .slots
                .values()
                .flat_map(|slots| slots.values().map(|slot| slot.fragment.as_ref())),
        )
        .flat_map(|fragment| &fragment.references)
        .filter(|reference| {
            reference.kind == RefKind::ClassImage && reference.destination.starts_with('#')
        })
        .map(|reference| reference.destination.as_str())
        .chain(
            analysis
                .candidate_urls
                .keys()
                .filter(|url| url.starts_with('#'))
                .map(String::as_str),
        )
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
                format!(
                    "the generated stylesheet loads {}, which is not a file the site publishes",
                    code_span(&url)
                ),
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
    pages.extend(not_found_pages);
    let documents = pages
        .iter()
        .map(|(path, html)| (path.clone(), Arc::from(html.as_bytes())))
        .collect();
    let search = crate::search::build(project, analysis, &documents)?;
    for (_, html) in &mut pages {
        for (lang, url) in &search.indexes {
            *html = html.replace(
                &format!(
                    "data-rd-search-index=\"{}\"",
                    crate::search::index_placeholder(lang)
                ),
                &format!(
                    "data-rd-search-index=\"{}\"",
                    crate::escape::escape_html(url)
                ),
            );
        }
        for (attribute, placeholder, url) in [
            (
                "module",
                crate::search::MODULE_PLACEHOLDER,
                &search.module_url,
            ),
            (
                "worker",
                crate::search::WORKER_PLACEHOLDER,
                &search.worker_url,
            ),
        ] {
            *html = html.replace(
                &format!("data-rd-search-{attribute}=\"{placeholder}\""),
                &format!(
                    "data-rd-search-{attribute}=\"{}\"",
                    crate::escape::escape_html(url)
                ),
            );
        }
    }
    for (path, bytes) in search.files {
        insert_output(&mut files, path, bytes)?;
    }
    for (path, bytes) in crate::ai::build(project, analysis)? {
        insert_output(&mut files, path, bytes)?;
    }
    // The page and its header carry the same policy; the not-found pages
    // are never landing pages.
    let profiles: BTreeMap<String, &Profile> = published
        .iter()
        .filter_map(|&i| {
            let page = &analysis.pages[i];
            Some((page_path(&page.route), landing_profile(config, page)?))
        })
        .collect();
    let page_policies: BTreeMap<String, String> = profiles
        .iter()
        .map(|(path, profile)| (path.clone(), crate::landing::policy(Some(profile))))
        .filter(|(_, policy)| policy != shell::CSP)
        .collect();
    let pages: Vec<(String, Arc<[u8]>, Vec<String>)> =
        parallel_map(&pages, workers, |(path, html)| {
            let html = html.replacen(STYLESHEET_PLACEHOLDER, &stylesheet_url, 1);
            let problems =
                crate::html_check::check_document_under(&html, profiles.get(path).copied());
            (path.clone(), Arc::from(html.into_bytes()), problems)
        });
    let mut sources: BTreeMap<String, String> = published
        .iter()
        .map(|&i| {
            let page = &analysis.pages[i];
            (page_path(&page.route), page.file.clone())
        })
        .collect();
    for path in std::iter::once("404.html").chain(locale_not_found.values().map(String::as_str)) {
        sources.insert(
            path.into(),
            config
                .rust
                .as_ref()
                .map_or(path, |rust| rust.registry.as_str())
                .to_owned(),
        );
    }
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
        insert_output(&mut files, path, bytes)?;
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    insert_output(
        &mut files,
        stylesheet_path.clone(),
        Arc::from(css.into_bytes()),
    )?;
    insert_output(&mut files, mode_path, Arc::from(MODE_SCRIPT.as_bytes()))?;
    insert_output(&mut files, site_path, Arc::from(SITE_SCRIPT.as_bytes()))?;

    // The files a host and its crawlers look for at the root.
    let absolute_route = |route: &str| absolute(&site.url(route)).unwrap_or_default();
    let mut robots = String::from("User-agent: *\nAllow: /\n");
    if origin.is_some() {
        let xml = sitemap(analysis, &published, &absolute_route, dates);
        insert_output(
            &mut files,
            "sitemap.xml".to_owned(),
            Arc::from(xml.into_bytes()),
        )?;
        robots.push_str(&format!("\nSitemap: {}\n", absolute_route("/sitemap.xml")));
    }
    insert_output(
        &mut files,
        "robots.txt".to_owned(),
        Arc::from(robots.into_bytes()),
    )?;
    // GitHub Pages runs Jekyll on a branch unless told not to.
    insert_output(&mut files, ".nojekyll".to_owned(), Arc::from(&b""[..]))?;
    problems.extend(final_documents(
        &files,
        &sources,
        &site,
        origin.as_deref(),
        workers,
    ));
    if !problems.is_empty() {
        return Err(problems);
    }
    Ok(Generation {
        files,
        locale_not_found,
        page_policies,
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
    dev_shell_with_status(project, false)
}

/// A static first-build page using the same assets as the diagnostic shell.
pub fn dev_shell_with_status(
    project: &Project,
    building: bool,
) -> Result<Generation, Vec<Diagnostic>> {
    let (snapshot, _) = crate::content::snapshot(project);
    dev_shell_from_snapshot(project, &snapshot, building)
}

/// Uses the failed generation's original message reads for its diagnostic shell.
#[doc(hidden)]
pub fn dev_shell_from_snapshot(
    project: &Project,
    snapshot: &Snapshot,
    building: bool,
) -> Result<Generation, Vec<Diagnostic>> {
    let config = &project.config;
    let mode_path = format!("assets/mode.{}.js", fingerprint(MODE_SCRIPT.as_bytes()));
    let site_path = format!("assets/site.{}.js", fingerprint(SITE_SCRIPT.as_bytes()));
    let catalogs = crate::content::resolve_catalogs(project, snapshot, &mut Vec::new());
    let mut site = Site {
        name: config.site.name.clone(),
        lang: config.site.lang.clone(),
        home_route: "/".into(),
        catalog: catalogs
            .iter()
            .find(|(lang, _)| lang.eq_ignore_ascii_case(&config.site.lang))
            .map(|(_, catalog)| catalog.clone())
            .unwrap_or_else(|| {
                Arc::new(crate::labels::resolve(&config.site.lang, &BTreeMap::new()))
            }),
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
    let mut pages = BTreeMap::new();
    let mut locale_not_found = BTreeMap::new();
    if config.locales.is_empty() {
        pages.insert(
            DEV_SHELL_PAGE.to_owned(),
            shell::render_dev_shell_with_status(&site, building),
        );
    } else {
        for locale in &config.locales {
            let mut localized = site.clone();
            localized.lang = locale.lang.clone();
            localized.home_route = crate::content::locale_route(config, &locale.lang, "/");
            localized.catalog = catalogs[&locale.lang].clone();
            let html = shell::render_dev_shell_with_status(&localized, building);
            let path = if locale.prefix.is_empty() {
                "404.html".into()
            } else {
                format!("{}/404.html", locale.prefix)
            };
            if locale.prefix.is_empty() {
                pages.insert(DEV_SHELL_PAGE.to_owned(), html.clone());
            }
            locale_not_found.insert(locale.prefix.clone(), path.clone());
            pages.insert(path, html);
        }
    }
    let mut candidates: BTreeSet<String> = crate::classes::SCRIPT_STATE_CLASSES
        .iter()
        .map(|class| class.to_string())
        .collect();
    for html in pages.values() {
        crate::classes::collect(html, &mut candidates);
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
    let stylesheet_path = format!("assets/site.{}.css", fingerprint(css.as_bytes()));
    let mut files: BTreeMap<String, Arc<[u8]>> = BTreeMap::new();
    let page_count = pages.len();
    for (path, html) in pages {
        let html = html.replacen(
            STYLESHEET_PLACEHOLDER,
            &site.url(&format!("/{stylesheet_path}")),
            1,
        );
        files.insert(path, Arc::from(html.into_bytes()));
    }
    files.insert(stylesheet_path.clone(), Arc::from(css.into_bytes()));
    files.insert(mode_path, Arc::from(MODE_SCRIPT.as_bytes()));
    files.insert(site_path, Arc::from(SITE_SCRIPT.as_bytes()));
    Ok(Generation {
        files,
        locale_not_found,
        page_policies: BTreeMap::new(),
        stylesheet_path,
        pages: page_count,
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
