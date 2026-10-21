//! The landing fixture generated end to end: landing pages under their
//! profiles' policies beside ordinary pages that keep the strict one, and a
//! site without profiles exactly as strict as before.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use rustify_docs::content::{self, Analysis, FragmentCache, Mode, Project};
use rustify_docs::generation::{self, Generation};
use rustify_docs::highlight::Highlighter;
use rustify_docs::{landing, shell};

/// A copy of a fixture in a fresh temporary directory, removed when dropped.
struct Copy(std::path::PathBuf);

impl Copy {
    fn of(name: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("rustify-docs-landing-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        copy_tree(&fixture(name), &root);
        Self(root)
    }

    /// Replaces `from` with `to` in a file, asserting it was there.
    fn edit(&self, relative: &str, from: &str, to: &str) {
        let path = self.0.join(relative);
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains(from), "{from:?} not in {relative}");
        std::fs::write(path, text.replacen(from, to, 1)).unwrap();
    }
}

impl Drop for Copy {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn analyze(project: &Project) -> (content::Snapshot, Analysis) {
    let (snapshot, problems) = content::snapshot(project);
    assert!(problems.is_empty(), "{problems:#?}");
    let analysis = content::analyze(
        project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        2,
    );
    (snapshot, analysis)
}

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/docs-ssg/fixtures")
        .join(name)
}

fn build(root: &Path) -> (Project, Analysis, Generation) {
    let (project, warnings) = Project::load(root).unwrap_or_else(|errors| panic!("{errors:#?}"));
    assert!(warnings.is_empty(), "{warnings:#?}");
    let (snapshot, analysis) = analyze(&project);
    assert_eq!(analysis.errors(), 0, "{:#?}", analysis.diagnostics);
    let generation = generation::generate(&project, &snapshot, &analysis, &Default::default(), 2)
        .unwrap_or_else(|problems| panic!("{problems:#?}"));
    (project, analysis, generation)
}

fn text(files: &BTreeMap<String, Arc<[u8]>>, path: &str) -> String {
    let bytes = files
        .get(path)
        .unwrap_or_else(|| panic!("{path} is not in the generation"));
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// The `content` of the page's policy `<meta>`.
fn meta_policy(html: &str) -> String {
    let dom = dom_query::Document::from(html);
    dom.select("meta[http-equiv=Content-Security-Policy]")
        .attr("content")
        .expect("a policy meta")
        .to_string()
}

fn html_files(generation: &Generation) -> impl Iterator<Item = (&String, String)> {
    generation
        .files
        .iter()
        .filter(|(path, _)| path.ends_with(".html"))
        .map(|(path, bytes)| (path, String::from_utf8(bytes.to_vec()).unwrap()))
}

#[track_caller]
fn contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "missing {needle:?} in:\n{haystack}"
    );
}

#[test]
fn landing_pages_carry_their_profiles_policy_and_nothing_else_does() {
    let (project, analysis, generation) = build(&fixture("landing"));
    let ads = &project.config.landing["ads"];
    let files = &generation.files;
    let lp = text(files, "lp/index.html");
    let dom = dom_query::Document::from(lp.as_str());

    // The policy, the profile's scripts after the theme's, in order.
    assert_eq!(meta_policy(&lp), landing::meta_policy(Some(ads)));
    let scripts: Vec<(String, bool, bool)> = dom
        .select("head script")
        .nodes()
        .iter()
        .map(|node| {
            (
                node.attr("src").unwrap_or_default().to_string(),
                node.has_attr("async"),
                node.has_attr("defer"),
            )
        })
        .collect();
    let init = files
        .keys()
        .find(|path| path.starts_with("assets/files/init.") && path.ends_with(".js"))
        .expect("the profile's own script, published by content");
    assert_eq!(text(files, init), text(files, "landing/init.js"));
    assert_eq!(scripts.len(), 4, "{scripts:?}");
    assert!(scripts[0].0.starts_with("/docs/assets/mode."));
    assert!(scripts[1].0.starts_with("/docs/assets/site.") && scripts[1].2);
    assert_eq!(
        scripts[2],
        ("http://127.0.0.1:4211/tag.js".into(), true, false)
    );
    assert_eq!(scripts[3], (format!("/docs/{init}"), false, true));

    // A brand that is not a link, no site navigation, the profile's footer.
    for absent in [
        "[data-rd-search]",
        "[data-rd-search-open]",
        ".rd-header-links",
        ".rd-header-nav",
        ".rd-sidebar",
        "[data-rd-mode-switch]",
        "[data-rd-language-switch]",
        ".rd-toc",
        "noscript",
    ] {
        assert!(dom.select(absent).is_empty(), "{absent} in:\n{lp}");
    }
    assert_eq!(dom.select("header .rd-brand").length(), 1);
    assert!(dom.select("header a").is_empty(), "{lp}");
    assert_eq!(
        dom.select("header .rd-brand span").text().as_ref(),
        "Landing Fixture"
    );
    let footer = dom.select("footer.rd-site-footer a");
    assert_eq!(
        (
            footer.length(),
            footer.text().as_ref(),
            footer.attr("href").as_deref()
        ),
        (1, "Privacy", Some("/docs/privacy/"))
    );
    assert_eq!(dom.select("#rd-status").length(), 1);

    // The hero, and what the theme script carries into it. The list is the
    // theme's own, in the head: the root element carries none.
    assert_eq!(
        dom.select("head meta[name=rd-carry]")
            .attr("content")
            .as_deref(),
        Some("gclid utm_source utm_campaign")
    );
    assert!(dom.select("html").attr("data-rd-carry").is_none(), "{lp}");
    assert_eq!(
        dom.select("h1.rd-home-title").text().as_ref(),
        "Ship docs in an afternoon"
    );
    assert_eq!(dom.select(".rd-features li").length(), 3);
    let actions = dom.select(".rd-actions a");
    assert_eq!(actions.length(), 2);
    assert!(actions
        .nodes()
        .iter()
        .all(|node| node.has_attr("data-rd-carry")));
    // Two actions, the page's own link, three hidden fields and frame.
    assert_eq!(dom.select("[data-rd-carry]").length(), 7);

    // The page's form and frame, as written: the profile lets them load.
    let form = dom.select("main form#lead");
    assert_eq!(
        form.attr("action").as_deref(),
        Some("http://127.0.0.1:4211/lead")
    );
    assert_eq!(form.attr("method").as_deref(), Some("post"));
    assert_eq!(
        dom.select("form input[type=hidden][data-rd-carry]")
            .length(),
        3
    );
    let frame = dom.select("main iframe");
    assert_eq!(
        frame.attr("src").as_deref(),
        Some("http://127.0.0.1:4211/embed")
    );
    assert!(frame.attr("title").is_some() && frame.attr("data-rd-carry").is_some());
    assert_eq!(
        dom.select("article a[data-rd-carry]")
            .attr("href")
            .as_deref(),
        Some("https://example.org/signup")
    );

    // Out of every index unless the page asks.
    contains(&lp, "<meta name=\"robots\" content=\"noindex\"");
    let sitemap = text(files, "sitemap.xml");
    contains(&sitemap, "/docs/guide/");
    for route in ["lp/", "thanks/", "offer/"] {
        assert!(
            !sitemap.contains(&format!("/docs/{route}")),
            "{route} in the sitemap"
        );
    }
    let llms = text(files, "llms.txt");
    contains(&llms, "Guide");
    assert!(
        !llms.contains("Ship docs") && !llms.contains("Thirty days"),
        "{llms}"
    );
    let search: String = files
        .iter()
        .filter(|(path, _)| path.starts_with("assets/search.") && path.ends_with(".json"))
        .map(|(_, bytes)| String::from_utf8_lossy(bytes).into_owned())
        .collect();
    contains(&search, "\"route\":\"/guide/\"");
    for route in ["/lp/", "/thanks/", "/offer/"] {
        assert!(
            !search.contains(&format!("\"route\":\"{route}\"")),
            "{route} in search"
        );
    }
    let listed: Vec<&str> = analysis
        .nav
        .iter()
        .flat_map(|group| &group.items)
        .map(|item| item.route.as_str())
        .collect();
    assert!(!listed
        .iter()
        .any(|route| ["/lp/", "/thanks/", "/offer/"].contains(route)));

    // Every other page, the not-found page included, keeps the strict policy.
    for path in [
        "index.html",
        "guide/index.html",
        "privacy/index.html",
        "404.html",
    ] {
        let html = text(files, path);
        assert_eq!(meta_policy(&html), shell::meta_csp(), "{path}");
        assert!(
            dom_query::Document::from(html.as_str())
                .select("meta[name=rd-carry]")
                .is_empty(),
            "{path}"
        );
        assert!(!html.contains("127.0.0.1"), "{path}");
    }
    // A profile that adds no source leaves even its own pages strict.
    let offer = text(files, "offer/index.html");
    assert_eq!(meta_policy(&offer), shell::meta_csp());
    contains(&offer, "<meta name=\"rd-carry\" content=\"gclid\">");
    assert_eq!(
        generation.page_policies,
        BTreeMap::from([
            ("form/index.html".to_owned(), landing::policy(Some(ads))),
            ("lp/index.html".to_owned(), landing::policy(Some(ads))),
            ("thanks/index.html".to_owned(), landing::policy(Some(ads))),
        ])
    );
    for (path, html) in html_files(&generation) {
        let header = generation
            .page_policies
            .get(path)
            .map_or(shell::CSP, String::as_str);
        let meta = header.strip_suffix("; frame-ancestors 'none'").unwrap();
        assert_eq!(meta_policy(&html), meta, "{path}");
    }
}

#[test]
fn landing_generation_is_deterministic_and_within_the_page_budget() {
    let root = fixture("landing");
    let (_, _, first) = build(&root);
    let (_, _, second) = build(&root);
    assert_eq!(first.files, second.files);
    assert_eq!(first.page_policies, second.page_policies);

    // The framework's share of a landing page: its HTML, the site's CSS and
    // the theme's scripts, each as `gzip -9` sends it.
    let gzip = |bytes: &[u8]| {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(9));
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap().len()
    };
    let shared = |prefix: &str, suffix: &str| {
        let (_, bytes) = first
            .files
            .iter()
            .find(|(path, _)| path.starts_with(prefix) && path.ends_with(suffix))
            .unwrap_or_else(|| panic!("{prefix}*{suffix}"));
        gzip(bytes)
    };
    let total = gzip(&first.files["offer/index.html"])
        + shared("assets/site.", ".css")
        + shared("assets/mode.", ".js")
        + shared("assets/site.", ".js");
    assert!(total <= 100_000, "{total} bytes");
}

#[test]
fn a_site_without_profiles_is_as_strict_as_before() {
    let (project, _, generation) = build(&fixture("content"));
    assert!(project.config.landing.is_empty());
    assert!(generation.page_policies.is_empty());
    let mut pages = 0;
    for (path, html) in html_files(&generation) {
        assert_eq!(meta_policy(&html), shell::meta_csp(), "{path}");
        assert!(
            dom_query::Document::from(html.as_str())
                .select("meta[name=rd-carry]")
                .is_empty(),
            "{path}"
        );
        pages += 1;
    }
    assert!(pages > 3);
}

#[test]
fn forms_on_ordinary_pages_stay_as_written_and_landing_targets_must_exist() {
    // A documentation page's form was always blocked by its policy: a
    // warning, and the page as written, endpoints the site does not publish
    // included.
    let site = Copy::of("landing");
    site.edit(
        "content/guide.md",
        "and the strict policy.",
        "and the strict policy.\n\n<form action=\"/api/lead\" method=\"post\"><button formaction=\"/api/x\">Send</button></form>",
    );
    let (project, warnings) = Project::load(&site.0).unwrap();
    assert!(warnings.is_empty());
    let (snapshot, analysis) = analyze(&project);
    assert_eq!(analysis.errors(), 0, "{:#?}", analysis.diagnostics);
    let blocked: Vec<_> = analysis
        .diagnostics
        .iter()
        .map(|d| (d.rule.as_str(), d.file.as_deref()))
        .collect();
    assert_eq!(
        blocked,
        [
            ("form-blocked", Some("content/guide.md")),
            ("form-blocked", Some("content/guide.md"))
        ]
    );
    let generation = generation::generate(&project, &snapshot, &analysis, &Default::default(), 2)
        .unwrap_or_else(|problems| panic!("{problems:#?}"));
    let guide = text(&generation.files, "guide/index.html");
    contains(&guide, "<form action=\"/api/lead\" method=\"post\">");
    contains(&guide, "<button formaction=\"/api/x\">");
    assert_eq!(meta_policy(&guide), shell::meta_csp());

    // A landing page's target on the site is a page that must exist.
    let site = Copy::of("landing");
    site.edit(
        "content/lp.md",
        "action=\"http://127.0.0.1:4211/lead\"",
        "action=\"/missing/\"",
    );
    let (project, _) = Project::load(&site.0).unwrap();
    let (_, analysis) = analyze(&project);
    let errors: Vec<_> = analysis
        .diagnostics
        .iter()
        .filter(|d| d.severity == rustify_docs::diagnostic::Severity::Error)
        .map(|d| (d.rule.as_str(), d.file.as_deref()))
        .collect();
    assert_eq!(errors, [("broken-link", Some("content/lp.md"))]);

    // One on the site that exists is rewritten under the base.
    let site = Copy::of("landing");
    site.edit(
        "content/lp.md",
        "action=\"http://127.0.0.1:4211/lead\"",
        "action=\"thanks.md\"",
    );
    let (_, _, generation) = build(&site.0);
    contains(
        &text(&generation.files, "lp/index.html"),
        "method=\"post\" action=\"/docs/thanks/\"",
    );
}

#[test]
fn a_form_keeps_the_query_it_submits() {
    // A form's target is a page on the site, and the query it submits is
    // part of what the reader sends: it is kept, and a target with only a
    // query is the page it is written on.
    let site = Copy::of("landing");
    site.edit(
        "content/lp.md",
        "action=\"http://127.0.0.1:4211/lead\"",
        "action=\"?campaign=paid\"",
    );
    let (_, _, generation) = build(&site.0);
    contains(
        &text(&generation.files, "lp/index.html"),
        "method=\"post\" action=\"/docs/lp/?campaign=paid\"",
    );

    // A target that names a page on the site keeps the query as well.
    let site = Copy::of("landing");
    site.edit(
        "content/lp.md",
        "action=\"http://127.0.0.1:4211/lead\"",
        "action=\"/thanks/?campaign=paid\"",
    );
    let (_, _, generation) = build(&site.0);
    contains(
        &text(&generation.files, "lp/index.html"),
        "method=\"post\" action=\"/docs/thanks/?campaign=paid\"",
    );

    // Under a root deployment the page is at the root.
    let site = Copy::of("landing");
    site.edit(
        "rustify-docs.toml",
        "url = \"https://example.com/docs/\"",
        "url = \"https://example.com/\"",
    );
    site.edit("rustify-docs.toml", "base = \"/docs/\"", "base = \"/\"");
    site.edit(
        "content/lp.md",
        "action=\"http://127.0.0.1:4211/lead\"",
        "action=\"?campaign=paid\"",
    );
    let (_, _, generation) = build(&site.0);
    contains(
        &text(&generation.files, "lp/index.html"),
        "method=\"post\" action=\"/lp/?campaign=paid\"",
    );
}

#[test]
fn a_page_cannot_write_the_list_of_parameters_the_theme_carries() {
    // The list is the theme's own, in the head: the root element carries
    // none. A `<html>` attribute in the content is merged into the root
    // element by the browser, and the theme does not read it there.
    let (_, _, generation) = build(&fixture("landing"));
    let lp = text(&generation.files, "lp/index.html");
    let root = lp.split("<head>").next().unwrap_or_default();
    assert!(!root.contains("data-rd-carry"), "{root}");
    let dom = dom_query::Document::from(lp.as_str());
    assert_eq!(
        dom.select("head meta[name=rd-carry]")
            .attr("content")
            .as_deref(),
        Some("gclid utm_source utm_campaign")
    );

    // A page whose content writes a list of its own gets none.
    let site = Copy::of("landing");
    site.edit(
        "content/guide.md",
        "and the strict policy.",
        "and the strict policy.\n\n<html data-rd-carry=\"secret\">\n\n<a data-rd-carry href=\"https://example.org/signup\">Continue</a>",
    );
    let (_, _, generation) = build(&site.0);
    let guide = text(&generation.files, "guide/index.html");
    let dom = dom_query::Document::from(guide.as_str());
    assert!(dom.select("meta[name=rd-carry]").is_empty(), "{guide}");
    let root = guide.split("<head>").next().unwrap_or_default();
    assert!(!root.contains("data-rd-carry"), "{root}");
    // The link the content marks is still the one it wrote.
    assert_eq!(
        dom.select("article a[data-rd-carry]")
            .attr("href")
            .as_deref(),
        Some("https://example.org/signup")
    );
}
