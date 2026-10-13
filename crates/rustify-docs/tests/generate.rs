//! The content fixture analysed and generated end to end, as a build would
//! write it: the final HTML of every page under the `/docs/` base, the files
//! that are and are not published, and the stylesheet's candidates.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use rustify_docs::content::{self, Analysis, FragmentCache, Mode, Project, Snapshot};
use rustify_docs::generation::{self, Generation};
use rustify_docs::highlight::Highlighter;

fn load() -> (Project, Snapshot) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/docs-ssg/fixtures/content");
    let (project, warnings) = Project::load(&root).unwrap_or_else(|errors| panic!("{errors:#?}"));
    assert!(warnings.is_empty(), "{warnings:#?}");
    let (snapshot, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:#?}");
    (project, snapshot)
}

fn build(
    project: &Project,
    snapshot: &Snapshot,
    mode: Mode,
    workers: usize,
) -> (Analysis, Generation) {
    let analysis = content::analyze(
        project,
        snapshot,
        mode,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        workers,
    );
    let generation =
        generation::generate(project, snapshot, &analysis, &Default::default(), workers)
            .expect("styles compile");
    (analysis, generation)
}

fn text(files: &BTreeMap<String, Arc<[u8]>>, path: &str) -> String {
    let bytes = files
        .get(path)
        .unwrap_or_else(|| panic!("{path} is not in the generation"));
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// The page's article, where the Markdown went.
fn article(html: &str) -> &str {
    let start = html.find("<article").expect("an article");
    let end = html.find("</article>").expect("a closed article");
    &html[start..end]
}

#[track_caller]
fn contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "missing {needle:?} in:\n{haystack}"
    );
}

#[test]
fn an_island_marker_in_readable_text_does_not_load_the_island_runtime() {
    let (project, snapshot) = load();
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    let mut articles = generation::resolved_articles(&project, &snapshot, &analysis);
    articles
        .get_mut("/guide/")
        .unwrap()
        .push_str("<p>Attribute syntax: <code>data-rustify-island=&quot;example&quot;</code></p>");
    let extensions = generation::Extensions {
        articles,
        files: BTreeMap::from([(
            "assets/islands/example/islands.js".to_owned(),
            Arc::<[u8]>::from(b"export {};".as_slice()),
        )]),
        island_script: Some("assets/islands/example/islands.js".to_owned()),
        ..Default::default()
    };
    let generation = generation::generate_extended(
        &project,
        &snapshot,
        &analysis,
        &Default::default(),
        1,
        &extensions,
    )
    .unwrap();
    assert!(!text(&generation.files, "guide/index.html").contains("data-rd-islands-module"));
}

#[test]
fn the_fixture_generates_without_a_problem_and_byte_for_byte_again() {
    let (project, snapshot) = load();
    let (analysis, first) = build(&project, &snapshot, Mode::Production, 4);
    assert!(
        analysis.diagnostics.is_empty(),
        "{:#?}",
        analysis.diagnostics
    );
    let (_, second) = build(&project, &snapshot, Mode::Production, 1);
    assert_eq!(
        first.files, second.files,
        "a generation depends on its inputs alone"
    );

    let names: Vec<&str> = first.files.keys().map(String::as_str).collect();
    let fingerprinted = |name: &str, prefix: &str, extension: &str| {
        name.strip_prefix(prefix)
            .and_then(|rest| rest.strip_suffix(extension))
            .is_some_and(|hash| hash.len() == 10 && hash.bytes().all(|b| b.is_ascii_hexdigit()))
    };
    let expected_exact = [
        ".nojekyll",
        "404.html",
        "banner.svg",
        "guide/index.html",
        "guide/setup/index.html",
        "index.html",
        "logo.svg",
        "reference/api/index.html",
        "robots.txt",
        "sitemap.xml",
        "sprite.svg",
    ];
    let expected_fingerprinted = [
        ("assets/files/banner.", ".svg"),
        ("assets/files/logo.", ".svg"),
        ("assets/files/pipeline.", ".svg"),
        ("assets/files/sprite.", ".svg"),
        ("assets/mode.", ".js"),
        ("assets/share.", ".png"),
        ("assets/site.", ".css"),
        ("assets/site.", ".js"),
    ];
    assert_eq!(
        names.len(),
        expected_exact.len() + expected_fingerprinted.len(),
        "{names:#?}"
    );
    for name in expected_exact {
        assert!(names.contains(&name), "{name} in {names:#?}");
    }
    for (prefix, extension) in expected_fingerprinted {
        assert!(
            names.iter().any(|n| fingerprinted(n, prefix, extension)),
            "{prefix}*{extension} in {names:#?}"
        );
    }
    // Not published: the draft, the Markdown, the included source.
    assert!(!names
        .iter()
        .any(|n| n.contains("later") || n.ends_with(".md") || n.ends_with(".rs")));
    assert_eq!(first.pages, 5, "four pages and the not-found page");
}

#[test]
fn merged_home_headings_keep_their_automatic_and_explicit_targets() {
    for base in ["/", "/docs/"] {
        for (heading, id) in [
            ("# Home", "home"),
            ("# Home {#入口}", "入口"),
            ("<h1 id=\"raw\">Home</h1>", "raw"),
        ] {
            let (mut project, mut snapshot) = load();
            project.config.site.base = base.into();
            snapshot.markdown.insert(
                "index.md".into(),
                Arc::from(format!(
                    "---\ntitle: Home\nlayout: home\n---\n{heading}\n\n[Here](#{id})\n"
                )),
            );
            let (analysis, generation) = build(&project, &snapshot, Mode::Production, 1);
            assert_eq!(analysis.errors(), 0, "{:#?}", analysis.diagnostics);
            let home = text(&generation.files, "index.html");
            contains(&home, &format!("id=\"{id}\""));
            assert_eq!(home.matches("<h1").count(), 1, "{home}");
            assert!(!article(&home).contains("<h1"));
        }
    }
}

#[test]
fn home_merge_ends_at_a_close_tag_and_escapes_the_transferred_id() {
    let (project, mut snapshot) = load();
    snapshot.markdown.insert(
        "index.md".into(),
        Arc::from("---\ntitle: Home\nlayout: home\n---\n<h1 id=\"</h1>\">Home</h1>\n\nBody.\n"),
    );
    let (analysis, generation) = build(&project, &snapshot, Mode::Production, 1);
    assert_eq!(analysis.errors(), 0);
    let html = text(&generation.files, "index.html");
    let dom = dom_query::Document::from(html.as_str());
    assert_eq!(dom.select("main h1").attr("id").as_deref(), Some("</h1>"));
    assert_eq!(dom.select("article").text().trim(), "Body.");
    snapshot.markdown.insert("index.md".into(), Arc::from("---\ntitle: Home\nlayout: home\n---\n<h1>Home<template><h1>Hidden</h1></template> tail</h1>\n\nBody.\n"));
    let (_, generation) = build(&project, &snapshot, Mode::Production, 1);
    let html = text(&generation.files, "index.html");
    let dom = dom_query::Document::from(html.as_str());
    assert_eq!(dom.select("article").text().trim(), "Body.");
}

#[test]
fn all_default_pages_have_a_site_footer_without_optional_metadata_or_links() {
    for base in ["/", "/docs/"] {
        let (mut project, mut snapshot) = load();
        project.config.site.base = base.into();
        project.config.nav.top.clear();
        project.config.nav.social.clear();
        project.config.nav.groups.clear();
        project.config.repo = None;
        snapshot.markdown.clear();
        snapshot.markdown.insert(
            "index.md".into(),
            Arc::from("---\ntitle: Home\nlayout: home\n---\n"),
        );
        snapshot.markdown.insert(
            "guide/index.md".into(),
            Arc::from("---\ndescription: d\n---\n# Guide\n"),
        );
        let (analysis, generation) = build(&project, &snapshot, Mode::Production, 1);
        assert_eq!(analysis.errors(), 0, "{:#?}", analysis.diagnostics);
        for file in ["index.html", "guide/index.html", "404.html"] {
            let html = text(&generation.files, file);
            let dom = dom_query::Document::from(html.as_str());
            let footer = dom.select("body > footer");
            assert_eq!(footer.length(), 1, "{file}: {html}");
            assert_eq!(footer.select("nav[aria-label] a").length(), 1);
            assert_eq!(footer.select("a").attr("href").as_deref(), Some(base));
        }
    }
}

#[test]
fn local_metadata_and_header_links_keep_their_conditions_beside_the_site_footer() {
    for edited in [false, true] {
        for dated in [false, true] {
            for header_links in [false, true] {
                let (mut project, snapshot) = load();
                project.config.repo.as_mut().unwrap().edit_url =
                    edited.then(|| "https://example.com/edit".into());
                if !header_links {
                    project.config.nav.top.clear();
                    project.config.nav.social.clear();
                }
                let (analysis, _) = build(&project, &snapshot, Mode::Production, 1);
                let dates = if dated {
                    BTreeMap::from([("content/guide/index.md".into(), "2026-10-12".into())])
                } else {
                    BTreeMap::new()
                };
                let generation =
                    generation::generate(&project, &snapshot, &analysis, &dates, 1).unwrap();
                let html = text(&generation.files, "guide/index.html");
                let dom = dom_query::Document::from(html.as_str());
                assert_eq!(
                    dom.select("body > footer nav a").attr("href").as_deref(),
                    Some("/docs/")
                );
                assert_eq!(
                    dom.select("main .rd-page-meta").length(),
                    usize::from(edited || dated)
                );
                assert_eq!(
                    dom.select("main .rd-page-meta a").length(),
                    usize::from(edited)
                );
                assert_eq!(
                    dom.select("main .rd-page-meta time").length(),
                    usize::from(dated)
                );
                assert_eq!(
                    dom.select(".rd-header-nav").length(),
                    if header_links { 2 } else { 0 }
                );
            }
        }
    }
}

#[test]
fn every_page_has_unique_metadata_and_the_site_files_list_what_may_be_indexed() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let files = &generation.files;
    let share = files
        .keys()
        .find(|name| name.starts_with("assets/share."))
        .unwrap();
    let mut titles = BTreeSet::new();
    for (path, route) in [
        ("index.html", "/docs/"),
        ("guide/index.html", "/docs/guide/"),
        ("guide/setup/index.html", "/docs/guide/setup/"),
        ("reference/api/index.html", "/docs/reference/api/"),
    ] {
        let html = text(files, path);
        let title = html
            .split("<title>")
            .nth(1)
            .unwrap()
            .split("</title>")
            .next()
            .unwrap();
        assert!(titles.insert(title.to_owned()), "{title} twice");
        let canonical = format!("https://example.com{route}");
        contains(
            &html,
            &format!(r#"<link rel="canonical" href="{canonical}">"#),
        );
        contains(
            &html,
            &format!(r#"<meta property="og:url" content="{canonical}">"#),
        );
        contains(
            &html,
            &format!(r#"<meta property="og:image" content="https://example.com/docs/{share}">"#),
        );
        contains(
            &html,
            r#"<meta name="twitter:card" content="summary_large_image">"#,
        );
        contains(&html, r#"<meta property="og:description" content=""#);
        assert!(!html.contains("noindex"), "{path}");
        let kind = if route == "/docs/" {
            "website"
        } else {
            "article"
        };
        contains(
            &html,
            &format!(r#"<meta property="og:type" content="{kind}">"#),
        );
    }
    let api = text(files, "reference/api/index.html");
    contains(&api, "<title>API 参考 | Fixture Docs</title>");
    contains(
        &api,
        r#"<meta name="description" content="中文页面，标题锚点保留中文。">"#,
    );

    // The logo, by content, in every header.
    let logo = files
        .keys()
        .find(|name| name.starts_with("assets/files/logo."))
        .unwrap();
    contains(
        &api,
        &format!(r#"<img src="/docs/{logo}" alt="" width="24" height="24" class="h-6 w-auto">"#),
    );

    let sitemap = text(files, "sitemap.xml");
    let locations: Vec<&str> = sitemap
        .split("<loc>")
        .skip(1)
        .map(|rest| rest.split("</loc>").next().unwrap())
        .collect();
    assert_eq!(
        locations,
        [
            "https://example.com/docs/",
            "https://example.com/docs/guide/",
            "https://example.com/docs/guide/setup/",
            "https://example.com/docs/reference/api/",
        ]
    );
    assert_eq!(
        text(files, "robots.txt"),
        "User-agent: *\nAllow: /\n\nSitemap: https://example.com/docs/sitemap.xml\n"
    );
    assert!(files[".nojekyll"].is_empty());
}

#[test]
fn the_not_found_page_works_from_any_depth() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let html = text(&generation.files, "404.html");
    assert_eq!(html.matches("<h1").count(), 1);
    contains(&html, "Page not found</h1>");
    contains(&html, r#"<meta name="robots" content="noindex">"#);
    assert!(!html.contains("rel=\"canonical\""));
    contains(&html, r#"<a href="/docs/">Back to the home page</a>"#);
    // Every link and file is root-relative under the base.
    for attribute in ["href=\"", "src=\""] {
        for value in html
            .split(attribute)
            .skip(1)
            .map(|rest| rest.split('"').next().unwrap())
        {
            assert!(
                value.starts_with("/docs/")
                    || value.starts_with('#')
                    || value.starts_with("https://"),
                "{attribute}{value}"
            );
        }
    }
}

#[test]
fn links_anchors_images_and_repository_files_resolve_under_the_base() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let files = &generation.files;
    let pipeline = files
        .keys()
        .find(|name| name.starts_with("assets/files/pipeline."))
        .unwrap();

    let guide = text(files, "guide/index.html");
    let body = article(&guide);
    contains(
        body,
        r#"<a href="/docs/guide/setup/#install-the-cli">setup</a>"#,
    );
    // The Chinese anchor, percent-encoded in the URL as browsers send it.
    contains(
        body,
        r#"<a href="/docs/reference/api/#%E6%95%B0%E6%8D%AE-api">reference</a>"#,
    );
    contains(body, r#"<h2 id="steps">Steps</h2>"#);
    contains(body, r#"<h2 id="steps-1">Steps</h2>"#);
    // Raw HTML: its image and its link resolve like Markdown's.
    contains(
        body,
        &format!(r#"<img src="/docs/{pipeline}" alt="The build pipeline" width="120">"#),
    );
    contains(
        body,
        r#"<a href="/docs/guide/setup/#check-the-version">Check the version</a>"#,
    );

    let api = text(files, "reference/api/index.html");
    contains(&api, r#"<html lang="zh-CN""#);
    contains(article(&api), r#"<h1 id="api-参考">API 参考</h1>"#);
    contains(article(&api), r#"<h2 id="数据-api">数据 API</h2>"#);
    contains(article(&api), r#"<a href="/docs/guide/">指南</a>"#);

    let setup = text(files, "guide/setup/index.html");
    let body = article(&setup);
    contains(body, r#"<h2 id="install-the-cli">Install the CLI</h2>"#);
    contains(body, r#"<h3 id="check-the-version">Check the version</h3>"#);
    contains(
        body,
        &format!(r#"<img src="/docs/{pipeline}" alt="The pipeline">"#),
    );
    contains(
        body,
        r#"<a href="https://github.com/example/fixture/blob/main/src/example.rs" rel="noopener">on GitHub</a>"#,
    );
    contains(
        body,
        r#"<a href="https://example.com/releases" rel="noopener">https://example.com/releases</a>"#,
    );

    let home = text(files, "index.html");
    contains(article(&home), r#"<a href="/docs/guide/">guide</a>"#);
    contains(
        article(&home),
        r#"<a href="/docs/reference/api/">API reference</a>"#,
    );
    // The site name is the current page on the home page alone.
    contains(
        &home,
        r#"<a href="/docs/" aria-current="page" class="rd-brand">"#,
    );
    assert!(!setup.contains(r#"<a href="/docs/" aria-current="page""#));
    contains(
        &setup,
        r#"<a href="/docs/guide/setup/" aria-current="page">Setup</a>"#,
    );
}

#[test]
fn files_keep_their_fragment_and_a_class_loads_its_image_from_the_site() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let files = &generation.files;
    let published = |prefix: &str| {
        files
            .keys()
            .find(|name| name.starts_with(prefix))
            .unwrap_or_else(|| panic!("{prefix}* is not published"))
            .clone()
    };
    let sprite = published("assets/files/sprite.");
    let banner = published("assets/files/banner.");
    let guide = text(files, "guide/index.html");
    let body = article(&guide);
    contains(body, &format!(r#"<use href="/docs/{sprite}#mark"></use>"#));
    let class = format!("bg-[url(/docs/{banner})]");
    contains(body, &format!(r#"<div class="h-10 {class} bg-no-repeat""#));
    // The stylesheet loads the published file, and nothing else but data.
    let css = text(files, &generation.stylesheet_path);
    let urls = rustify_docs::style::urls(&css);
    assert!(urls.contains(&format!("/docs/{banner}")), "{urls:?}");
    assert!(
        urls.iter()
            .all(|url| url.starts_with("data:") || url.starts_with("/docs/assets/files/")),
        "{urls:?}"
    );
    // A heading id with a `%` is a whole fragment in the contents.
    contains(
        &guide,
        r##"<a href="#100%25" data-rd-toc-link="">All done</a>"##,
    );
    contains(body, r#"<h2 id="100%">All done</h2>"#);
}

#[test]
fn css_resource_decoding_and_entity_spans_survive_generation() {
    let (project, mut snapshot) = load();
    let red: Arc<[u8]> =
        Arc::from(b"<svg xmlns='http://www.w3.org/2000/svg'><rect fill='red'/></svg>".as_slice());
    let blue: Arc<[u8]> =
        Arc::from(b"<svg xmlns='http://www.w3.org/2000/svg'><rect fill='blue'/></svg>".as_slice());
    snapshot
        .files
        .insert("static/assets/bg one.svg".into(), red.clone());
    snapshot
        .files
        .insert("static/assets/bg_one.svg".into(), blue.clone());
    snapshot
        .files
        .insert("content/=/bg.svg".into(), red.clone());
    let values = [
        "URL('/assets/bg_one.svg')",
        "url('/assets/bg_one.svg')",
        r"URL('/assets/bg\_one.svg')",
        r"url('/assets/bg\_one.svg')",
        "URL('/assets/bg%20one.svg')",
        "url('/assets/bg%20one.svg')",
        "url&#40=/bg.svg)",
        "url&#40;=/bg.svg)",
        "url(&#34=/bg.svg&#34)",
        "url(&#34;=/bg.svg&#34;)",
    ];
    let body = values
        .iter()
        .map(|value| format!("<div class=\"[background-image:{value}]\"></div>\n"))
        .collect::<String>();
    snapshot.markdown.insert(
        "index.md".into(),
        Arc::from(format!(
            "---\ndescription: Resources\n---\n# Resources\n\n{body}"
        )),
    );
    let (analysis, generation) = build(&project, &snapshot, Mode::Production, 2);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let html = text(&generation.files, "index.html");
    let (_, slots) = rustify_docs::html_check::check(article(&html));
    let css = text(&generation.files, &generation.stylesheet_path);
    let loads = rustify_docs::style::urls(&css);
    assert_eq!(slots.len(), values.len());
    for (i, slot) in slots.iter().enumerate() {
        assert!(loads.contains(&slot.value), "{}: {loads:?}", values[i]);
        let path = slot
            .value
            .strip_prefix("/docs/")
            .unwrap()
            .replace("%20", " ");
        assert_eq!(
            generation.files[&path],
            if [1, 2, 3].contains(&i) {
                blue.clone()
            } else {
                red.clone()
            },
            "{}",
            values[i]
        );
    }
}

#[test]
fn a_generation_cannot_load_an_absent_file_under_its_assets_prefix() {
    let (project, snapshot) = load();
    let mut analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0);
    let page = analysis.routes["/guide/"];
    Arc::make_mut(&mut analysis.pages[page].fragment)
        .html
        .push_str("<div class=\"[background-image:URL(/docs/assets/missing.svg)]\"></div>");
    let errors = generation::generate(&project, &snapshot, &analysis, &Default::default(), 1)
        .err()
        .expect("an untracked URL must still be checked against the generation");
    assert!(
        errors
            .iter()
            .any(|d| d.rule == "output-check" && d.message.contains("/docs/assets/missing.svg")),
        "{errors:?}"
    );
}

#[test]
fn header_links_are_on_every_page_and_mark_the_guide() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let files = &generation.files;
    let links = |current: &str| {
        format!(
            r#"<ul class="rd-header-links"><li><a href="/docs/guide/"{current}>Guide</a></li><li><a href="https://github.com/example/fixture" rel="noopener">GitHub</a></li></ul><ul class="rd-header-social"><li><a href="https://mastodon.example/@fixture" rel="noopener">Mastodon</a></li></ul>"#
        )
    };
    for (page, current) in [
        ("index.html", ""),
        ("404.html", ""),
        ("reference/api/index.html", ""),
        ("guide/index.html", r#" aria-current="page""#),
        ("guide/setup/index.html", r#" aria-current="true""#),
    ] {
        let html = text(files, page);
        assert_eq!(html.matches(&links(current)).count(), 2, "{page}:\n{html}");
    }
}

#[test]
fn gfm_alerts_tables_tasks_footnotes_and_code_render_without_inline_style() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let files = &generation.files;
    let guide = text(files, "guide/index.html");
    let setup = text(files, "guide/setup/index.html");
    let both = format!("{}{}", article(&guide), article(&setup));
    for (kind, title) in [
        ("note", "Note"),
        ("tip", "Tip"),
        ("important", "Important"),
        ("warning", "Warning"),
        ("caution", "Caution"),
    ] {
        contains(
            &both,
            &format!(
                "<div class=\"markdown-alert markdown-alert-{kind}\" role=\"note\">\n<p class=\"markdown-alert-title\">{}{title}</p>",
                rustify_docs::icons::alert(kind)
            ),
        );
    }

    let body = article(&setup);
    contains(
        body,
        r#"<thead><tr><th class="text-left">Command</th><th class="text-center">Writes</th><th class="text-right">Network</th></tr></thead>"#,
    );
    contains(
        body,
        r#"<li><input type="checkbox" disabled checked aria-label="Task"> Install</li>"#,
    );
    contains(
        body,
        r##"<sup class="rd-footnote-ref"><a href="#fn-first" id="fnref-first">1</a></sup>"##,
    );
    contains(
        body,
        r#"<li id="fn-first"><p>A page is a Markdown file under <code>content/</code>.</p>"#,
    );

    // A code group: both tabs readable without script, the tab list hidden
    // until the script enhances it.
    contains(
        body,
        r#"<div class="rd-code-group" data-rd-group="install">"#,
    );
    contains(
        body,
        r#"<div class="rd-code-tabs" hidden><div class="rd-code-tablist" role="tablist">"#,
    );
    contains(body, r#"data-lang="sh" data-rd-tab-panel="0""#);
    contains(body, r#"data-lang="toml" data-rd-tab-panel="1""#);
    contains(
        body,
        "<template class=\"rd-code-raw\">cargo install --locked rustify-docs-cli\n</template>",
    );

    // The include: the region's lines, byte for byte, without its markers.
    contains(body, r#"<span class="rd-code-title">src/example.rs</span>"#);
    contains(body, "<template class=\"rd-code-raw\">fn main() {\n    println!(&quot;hello&quot;);\n}\n</template>");
    assert!(!setup.contains("rustify-region") && !setup.contains("rustify-endregion"));

    for (path, bytes) in files.iter().filter(|(path, _)| path.ends_with(".html")) {
        let html = std::str::from_utf8(bytes).unwrap();
        assert!(!html.contains(" style="), "{path} has an inline style");
        assert!(!html.contains("<style"), "{path} has a style element");
        assert!(!html.contains("<script>"), "{path} has an inline script");
        assert!(!html.contains("http://"), "{path} loads over plain HTTP");
    }
}

#[test]
fn highlighting_and_code_text_stay_out_of_the_stylesheet() {
    let (project, snapshot) = load();
    let (_, generation) = build(&project, &snapshot, Mode::Production, 2);
    let mut candidates = BTreeSet::new();
    for (path, bytes) in &generation.files {
        if path.ends_with(".html") {
            rustify_docs::classes::collect(std::str::from_utf8(bytes).unwrap(), &mut candidates);
        }
    }
    assert!(
        candidates.iter().all(|class| !class.starts_with("hl-")),
        "{candidates:?}"
    );
    assert!(
        candidates.contains("text-center"),
        "table alignment is a utility"
    );
    let api = text(&generation.files, "reference/api/index.html");
    contains(article(&api), "bg-red-500");
    let css = text(&generation.files, &generation.stylesheet_path);
    assert!(
        !css.contains("bg-red-500"),
        "code text compiled into a utility"
    );
    assert!(
        css.contains(".text-center{"),
        "an alignment class used by a table has its rule"
    );
    assert!(
        css.contains(".hl-"),
        "the highlight palette ships with the stylesheet"
    );
}

#[test]
fn drafts_render_marked_in_dev_and_nowhere_in_production() {
    let (project, snapshot) = load();
    let (dev, generation) = build(&project, &snapshot, Mode::Dev, 2);
    assert!(dev.diagnostics.is_empty(), "{:#?}", dev.diagnostics);
    let later = text(&generation.files, "guide/later/index.html");
    contains(&later, ">Draft</p>");
    let setup = text(&generation.files, "guide/setup/index.html");
    contains(&setup, "Later · Draft");

    let (_, production) = build(&project, &snapshot, Mode::Production, 2);
    for (path, bytes) in &production.files {
        let text = String::from_utf8_lossy(bytes);
        assert!(
            !text.contains("/guide/later/") && !text.contains("Later · Draft"),
            "{path} mentions the draft"
        );
    }
}
