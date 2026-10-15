#![cfg(feature = "pipeline")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use rustify_docs::content::{self, FragmentCache, Mode, Project, RegisteredPage, Surfaces};
use rustify_docs::generation::{self, Generation};
use rustify_docs::highlight::Highlighter;
use rustify_docs::site::Slot;
use serde_json::Value;

const CONFIG: &str = "schema_version=1\n[site]\nname='Docs'\nlang='en'\nbase='/docs/'\nurl='https://example.com/docs/'\n\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n[[i18n.locales]]\nlang='zh'\nlabel='中文'\nprefix='zh'\n";

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "rustify-docs-search-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let fixture = Self(root);
        fixture.write("rustify-docs.toml", CONFIG);
        fixture.page("index.md", "Home", "", "Welcome.");
        fixture
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    fn page(&self, path: &str, title: &str, metadata: &str, body: &str) {
        self.write(
            &format!("content/{path}"),
            &format!("---\ndescription: {title} description\n{metadata}---\n# {title}\n\n{body}\n"),
        );
    }

    fn generate(&self, surfaces: &Surfaces) -> Generation {
        self.generate_mode(surfaces, Mode::Production)
    }

    fn generate_mode(&self, surfaces: &Surfaces, mode: Mode) -> Generation {
        let (project, _) = Project::load(&self.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        let analysis = content::analyze_with_surfaces(
            &project,
            &snapshot,
            mode,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
            surfaces,
        );
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        generation::generate_extended(
            &project,
            &snapshot,
            &analysis,
            &Default::default(),
            1,
            &Default::default(),
        )
        .unwrap_or_else(|problems| panic!("{problems:?}"))
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn index(generation: &Generation, lang: &str) -> Value {
    let prefix = format!("assets/search.{lang}.");
    let files: Vec<_> = generation
        .files
        .iter()
        .filter(|(path, _)| path.starts_with(&prefix) && path.ends_with(".json"))
        .collect();
    assert_eq!(
        files.len(),
        1,
        "one current-language index: {:?}",
        generation.files.keys()
    );
    serde_json::from_slice(files[0].1).unwrap()
}

fn custom(route: &str, html: &str) -> RegisteredPage {
    RegisteredPage {
        draft: None,
        index: None,
        search: None,
        ai: None,
        name: "Custom".into(),
        route: route.into(),
        title: "Real page title".into(),
        lang: None,
        description: Some("Custom description".into()),
        html: html.into(),
        file: "src/pages.rs".into(),
    }
}

#[test]
fn extraction_uses_current_language_routes_and_independent_public_switches() {
    let fixture = Fixture::new();
    fixture.page(
        "guide.md",
        "Guide",
        "index: false\nai: false\n",
        "English body.",
    );
    fixture.page("zh/guide.md", "指南", "", "中文正文。");
    fixture.page(
        "hidden.md",
        "Excluded search",
        "search: false\n",
        "SEARCH_SENTINEL",
    );
    fixture.page("draft.md", "Draft", "draft: true\n", "DRAFT_SENTINEL");
    fixture.page("missing.md", "Fallback", "", "Fallback words.");
    let generation = fixture.generate(&Surfaces::default());
    let en = index(&generation, "en");
    let zh = index(&generation, "zh");
    assert_eq!(en["schema_version"], 1);
    assert_eq!(zh["ui_lang"], "zh");
    let pages = zh["pages"].as_array().unwrap();
    assert_eq!(pages.len(), 3);
    assert!(pages
        .iter()
        .all(|page| page["route"].as_str().unwrap().starts_with("/zh/")));
    let guide = pages
        .iter()
        .find(|page| page["route"] == "/zh/guide/")
        .unwrap();
    assert_eq!(guide["title"], "指南");
    assert_eq!(guide["source_lang"], "zh");
    assert_eq!(guide["is_fallback"], false);
    let fallback = pages
        .iter()
        .find(|page| page["route"] == "/zh/missing/")
        .unwrap();
    assert_eq!(fallback["source_lang"], "en");
    assert_eq!(fallback["is_fallback"], true);
    for data in [en.to_string(), zh.to_string()] {
        for private in [
            "SEARCH_SENTINEL",
            "DRAFT_SENTINEL",
            "404",
            "content/",
            fixture.0.to_str().unwrap(),
        ] {
            assert!(!data.contains(private), "{private}");
        }
    }
}

#[test]
fn extraction_legacy_page_languages_receive_reachable_current_language_indexes() {
    let fixture = Fixture::new();
    fixture.write(
        "rustify-docs.toml",
        CONFIG.split("[[i18n.locales]]").next().unwrap(),
    );
    fixture.page("guide.md", "中文指南", "lang: zh-CN\n", "中文正文。");
    let generation = fixture.generate(&Surfaces::default());
    assert_eq!(
        index(&generation, "en")["pages"].as_array().unwrap().len(),
        1
    );
    let chinese = index(&generation, "zh-CN");
    assert_eq!(chinese["pages"][0]["route"], "/guide/");
    let document = dom_query::Document::from(
        std::str::from_utf8(&generation.files["guide/index.html"]).unwrap(),
    );
    let url = document
        .select("dialog[data-rd-search]")
        .attr("data-rd-search-index")
        .unwrap()
        .to_string();
    assert!(url.starts_with("/docs/assets/search.zh-CN."));
    assert!(generation
        .files
        .contains_key(url.strip_prefix("/docs/").unwrap()));
    assert!(generation.files.contains_key("llms.txt"));
    assert!(!generation.files.contains_key("zh-CN/llms.txt"));
}

#[test]
fn extraction_walks_final_main_with_reachable_heading_paths_and_readable_text() {
    let fixture = Fixture::new();
    let surfaces = Surfaces {
        pages: vec![custom("/custom/", "<h1>Custom heading</h1><p>First <em>inline</em> words.</p><p>Next paragraph.</p><h2 id='setup'>Setup</h2><p>Use <code>ThemeScope</code>.</p><h3 id='second'>Second</h3><p>Nested body</p><h2>No anchor</h2><p>Unanchored body</p><div class='rd-code'><button class='rd-copy'>CONTROL_SENTINEL</button><pre><code>fn example() {}</code></pre><template>RAW_SENTINEL</template></div>")],
        slots: [("/custom/".into(), [(Slot::BeforeContent, "<p>SLOT_SENTINEL</p>".into())].into())].into(),
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces);
    let data = index(&generation, "en");
    let page = data["pages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["route"] == "/custom/")
        .unwrap();
    assert_eq!(page["title"], "Real page title");
    let sections = page["sections"].as_array().unwrap();
    assert_eq!(sections.len(), 3);
    assert!(sections[0]["anchor"].is_null());
    assert!(sections[0]["text"]
        .as_str()
        .unwrap()
        .contains("First inline words. Next paragraph."));
    assert_eq!(sections[1]["anchor"], "setup");
    assert_eq!(
        sections[2]["path"],
        serde_json::json!(["Real page title", "Setup", "Second"])
    );
    assert!(sections[2]["text"]
        .as_str()
        .unwrap()
        .contains("No anchor Unanchored body"));
    assert!(sections[2]["text"]
        .as_str()
        .unwrap()
        .contains("fn example() {}"));
    for private in [
        "CONTROL_SENTINEL",
        "RAW_SENTINEL",
        "SLOT_SENTINEL",
        "Copy",
        "Previous",
        "src/pages.rs",
        "<p>",
    ] {
        assert!(!page.to_string().contains(private), "{private}");
    }
}

#[test]
fn extraction_preserves_home_features_and_component_initial_text_but_not_source_chrome() {
    let fixture = Fixture::new();
    fixture.page("index.md", "Home", "layout: home\ntagline: Home tagline\nfeatures:\n  - title: Feature title\n    details: Feature details\n", "Home body.");
    let surfaces = Surfaces { pages: vec![custom("/custom/", "<h1>Component</h1><section class='rd-demo'><fieldset class='rd-demo-canvas'><button>Initial component</button><p>Initial value seven</p></fieldset><div class='rd-demo-status'>STATUS_SENTINEL</div><div class='rd-demo-source'><p class='rd-demo-path'>PATH_SENTINEL</p><div class='rd-code-group'><div role='tablist'>TAB_SENTINEL</div><div class='rd-code-panel'><pre><code>fn visible_function() {}</code></pre></div><div class='rd-code-panel' hidden><pre>FULL_FILE_SENTINEL</pre></div></div></div></section>")], ..Default::default() };
    let generation = fixture.generate(&surfaces);
    let data = index(&generation, "en").to_string();
    for public in [
        "Home tagline",
        "Feature title",
        "Feature details",
        "Home body.",
        "Initial component",
        "Initial value seven",
        "visible_function",
    ] {
        assert!(data.contains(public), "{public}");
    }
    for private in [
        "STATUS_SENTINEL",
        "PATH_SENTINEL",
        "TAB_SENTINEL",
        "FULL_FILE_SENTINEL",
    ] {
        assert!(!data.contains(private), "{private}");
    }
}

#[test]
fn extraction_is_deterministic_and_rebuilds_removed_translations_and_switches() {
    let fixture = Fixture::new();
    fixture.page("guide.md", "Guide", "", "English body.");
    fixture.page("zh/guide.md", "指南", "", "中文正文。");
    let first = fixture.generate(&Surfaces::default());
    let second = fixture.generate(&Surfaces::default());
    assert_eq!(first.files, second.files);
    std::fs::remove_file(fixture.0.join("content/zh/guide.md")).unwrap();
    fixture.page("guide.md", "Guide", "search: false\n", "English body.");
    let changed = fixture.generate(&Surfaces::default());
    assert_eq!(index(&changed, "zh")["pages"].as_array().unwrap().len(), 1);
    assert!(first
        .files
        .keys()
        .filter(|path| path.starts_with("assets/search.zh."))
        .all(|path| !changed.files.contains_key(path)));
}

#[test]
fn extraction_rejects_reserved_language_files_and_directory_routes() {
    for path in [
        "llms.txt",
        "zh/llms.txt",
        "zh/llms.txt/index.html",
        "assets/search.en.fake.json",
        "assets/search-worker.fake.js",
    ] {
        let fixture = Fixture::new();
        fixture.write(&format!("static/{path}"), "reserved");
        let (project, _) = Project::load(&fixture.0).unwrap();
        let (snapshot, _) = content::snapshot(&project);
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Production,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
        );
        assert!(
            analysis
                .diagnostics
                .iter()
                .any(|d| d.rule == "output-conflict"),
            "{path}: {:?}",
            analysis.diagnostics
        );
    }
}

#[test]
fn extraction_orders_navigation_first_and_includes_unlisted_pages() {
    let fixture = Fixture::new();
    fixture.write(
        "rustify-docs.toml",
        &format!("{CONFIG}\n[[nav.groups]]\nlabel='Guides'\nitems=['b.md','a.md']\n"),
    );
    fixture.page(
        "a.md",
        "A",
        "sidebar:\n  label: Navigation alias\n",
        "Body A",
    );
    fixture.page("b.md", "B", "", "Body B");
    fixture.page("unlisted.md", "Unlisted", "", "Outside navigation");
    let data = index(&fixture.generate(&Surfaces::default()), "en");
    let pages = data["pages"].as_array().unwrap();
    assert_eq!(
        pages
            .iter()
            .map(|page| page["route"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["/b/", "/a/", "/", "/unlisted/"]
    );
    assert_eq!(pages[1]["title"], "A");
    assert!(pages
        .iter()
        .enumerate()
        .all(|(i, page)| page["nav_order"] == i));
}

#[test]
fn extraction_registered_policies_reach_the_graph_and_all_fallbacks() {
    let fixture = Fixture::new();
    let mut hidden = custom("/hidden/", "<h1>Hidden</h1><p>SEARCH_SECRET</p>");
    hidden.search = Some(false);
    let mut draft = custom("/draft/", "<h1>Draft</h1><p>DRAFT_SECRET</p>");
    draft.name = "Draft".into();
    draft.title = "Draft".into();
    draft.draft = Some(true);
    let mut translation = custom("/draft/", "<h1>草稿译文</h1><p>TRANSLATED_SECRET</p>");
    translation.name = "DraftZh".into();
    translation.title = "草稿译文".into();
    translation.lang = Some("zh".into());
    let surfaces = Surfaces {
        pages: vec![hidden, draft, translation],
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces);
    for lang in ["en", "zh"] {
        let data = index(&generation, lang);
        assert_eq!(data["pages"].as_array().unwrap().len(), 1);
        assert!(!data.to_string().contains("SECRET"));
    }
    assert!(!generation.files.contains_key("draft/index.html"));
    assert!(!generation.files.contains_key("zh/draft/index.html"));
    assert!(generation.files.contains_key("zh/hidden/index.html"));
    let dev = fixture.generate_mode(&surfaces, Mode::Dev);
    assert!(dev.files.contains_key("zh/draft/index.html"));
    assert_eq!(index(&dev, "zh")["pages"].as_array().unwrap().len(), 1);
}

#[test]
fn extraction_tracks_heading_levels_when_a_custom_parent_has_no_id() {
    let fixture = Fixture::new();
    let generation = fixture.generate(&Surfaces { pages: vec![custom("/custom/", "<h1>Custom</h1><h2 id='old'>Old parent</h2><p>Old body.</p><h2>New parent</h2><p>Unanchored body.</p><h3 id='first'>First child</h3><p>First body.</p><h3 id='second'>Second child</h3><p>Second body.</p>")], ..Default::default() });
    let data = index(&generation, "en");
    let page = data["pages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|page| page["route"] == "/custom/")
        .unwrap();
    assert_eq!(
        page["sections"][2]["path"],
        serde_json::json!(["Real page title", "New parent", "First child"])
    );
    assert_eq!(
        page["sections"][3]["path"],
        serde_json::json!(["Real page title", "New parent", "Second child"])
    );
    assert!(page["sections"][1]["text"]
        .as_str()
        .unwrap()
        .contains("New parent Unanchored body."));
}

#[test]
fn extraction_preserves_direct_registered_component_actions_and_labels() {
    let fixture = Fixture::new();
    let generation = fixture.generate(&Surfaces { pages: vec![custom("/custom/", "<h1>Custom</h1><button type='button'>Direct component action</button><label for='custom-value'>Field label</label><input id='custom-value' value='7'>")], ..Default::default() });
    let data = index(&generation, "en").to_string();
    assert!(data.contains("Direct component action"));
    assert!(data.contains("Field label"));
    assert!(data.contains("Direct component action Field label"));
}

#[test]
fn shell_search_is_static_accessible_and_uses_the_current_language_on_every_page() {
    let fixture = Fixture::new();
    let generation = fixture.generate(&Surfaces {
        pages: vec![custom(
            "/custom/",
            "<h1>Custom</h1><p>Readable without scripts.</p>",
        )],
        ..Default::default()
    });
    for (path, bytes) in generation
        .files
        .iter()
        .filter(|(path, _)| path.ends_with(".html"))
    {
        let html = std::str::from_utf8(bytes).unwrap();
        let dom = dom_query::Document::from(html);
        let lang = if path.starts_with("zh/") { "zh" } else { "en" };
        assert_eq!(
            dom.select("button[data-rd-search-open][disabled]").length(),
            1,
            "{path}"
        );
        assert_eq!(dom.select("dialog[data-rd-search]").length(), 1, "{path}");
        assert_eq!(dom.select("dialog h1").length(), 0);
        assert_eq!(
            dom.select(
                "input[role=combobox][aria-controls=rd-search-results][aria-expanded=false]"
            )
            .length(),
            1
        );
        assert_eq!(dom.select("label[for=rd-search-input]").length(), 1);
        assert_eq!(dom.select("#rd-search-results[role=listbox]").length(), 1);
        assert_eq!(
            dom.select("dialog[data-rd-search]")
                .attr("data-rd-search-lang")
                .as_deref(),
            Some(lang)
        );
        for (attribute, prefix, suffix) in [
            (
                "data-rd-search-index",
                &format!("/docs/assets/search.{lang}."),
                ".json",
            ),
            (
                "data-rd-search-worker",
                &"/docs/assets/search-worker.".to_string(),
                ".js",
            ),
            (
                "data-rd-search-module",
                &"/docs/assets/search.".to_string(),
                ".js",
            ),
        ] {
            let url = dom
                .select("dialog[data-rd-search]")
                .attr(attribute)
                .unwrap()
                .to_string();
            assert!(
                url.starts_with(prefix) && url.ends_with(suffix),
                "{path}: {url}"
            );
            assert!(generation
                .files
                .contains_key(url.strip_prefix("/docs/").unwrap()));
        }
        assert!(html.contains(if lang == "zh" {
            "启用 JavaScript"
        } else {
            "Enable JavaScript"
        }));
        assert!(!html.contains("/__rustify_docs_search_"));
        assert_eq!(dom.select("script[src*='search.'],script[src*='search-worker.'],link[rel=preload][href*='search']").length(), 0);
    }
}

#[test]
fn shell_search_marks_each_third_language_control_with_its_actual_catalog_language() {
    let fixture = Fixture::new();
    fixture.write("rustify-docs.toml", &format!("{CONFIG}\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\nmessages='locales/fr.toml'\n"));
    fixture.write("locales/fr.toml", "[search]\nopen='Rechercher'\n");
    let generation = fixture.generate(&Surfaces::default());
    let document =
        dom_query::Document::from(std::str::from_utf8(&generation.files["fr/index.html"]).unwrap());
    assert_eq!(
        document
            .select("dialog[data-rd-search]")
            .attr("lang")
            .as_deref(),
        Some("fr")
    );
    assert_eq!(
        document.select("#rd-search-input").attr("lang").as_deref(),
        Some("en")
    );
    assert_eq!(
        document
            .select("#rd-search-results")
            .attr("lang")
            .as_deref(),
        Some("en")
    );
    assert_eq!(
        document
            .select("label[for=rd-search-input]")
            .attr("lang")
            .as_deref(),
        Some("en")
    );
}
