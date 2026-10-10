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
        "guide/index.html",
        "guide/setup/index.html",
        "index.html",
        "logo.svg",
        "reference/api/index.html",
        "robots.txt",
        "sitemap.xml",
    ];
    let expected_fingerprinted = [
        ("assets/files/logo.", ".svg"),
        ("assets/files/pipeline.", ".svg"),
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
