#![cfg(feature = "pipeline")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use rustify_docs::content::{
    self, Analysis, FragmentCache, Mode, PageOrigin, Project, RegisteredPage, Surfaces,
};
use rustify_docs::highlight::Highlighter;

const CONFIG: &str = "schema_version=1\n[site]\nname='Docs'\nlang='en'\nurl='https://example.com/docs/'\nbase='/docs/'\n";
const LOCALES: &str = "\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n\n[[i18n.locales]]\nlang='zh'\nlabel='中文'\nprefix='zh'\n\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\n";

#[path = "i18n/input_cases.rs"]
mod inputs;

struct Fixture(PathBuf);

impl Fixture {
    fn new(config: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "rustify-docs-i18n-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let fixture = Self(root);
        fixture.write("rustify-docs.toml", config);
        fixture
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    fn page(&self, path: &str, title: &str, extra: &str) {
        self.write(
            &format!("content/{path}"),
            &format!("---\ndescription: {title} description\n{extra}---\n# {title}\n"),
        );
    }

    fn analyze(&self, mode: Mode, surfaces: &Surfaces) -> Analysis {
        let (project, warnings) = Project::load(&self.0).unwrap();
        assert!(warnings.is_empty(), "{warnings:?}");
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        content::analyze_with_surfaces(
            &project,
            &snapshot,
            mode,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
            surfaces,
        )
    }

    fn generate(&self, surfaces: &Surfaces) -> rustify_docs::generation::Generation {
        let (project, _) = Project::load(&self.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        let analysis = self.analyze(Mode::Production, surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        rustify_docs::generation::generate_extended(
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

#[test]
fn graph_emits_every_locale_at_one_logical_address() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("guide/start.md", "Start", "");
    fixture.page("zh/guide/start.md", "开始", "lang: zh\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let routes: Vec<_> = analysis.routes.keys().map(String::as_str).collect();
    assert_eq!(
        routes,
        [
            "/",
            "/fr/",
            "/fr/guide/start/",
            "/guide/start/",
            "/zh/",
            "/zh/guide/start/"
        ]
    );
    let page = &analysis.pages[analysis.routes["/fr/guide/start/"]];
    assert_eq!(page.path, "guide/start.md");
    assert_eq!(page.lang, "fr");
    assert_eq!(page.title, "Start");
    assert_eq!(page.locale.logical_route, "/guide/start/");
    assert_eq!(page.locale.source_lang, "en");
    assert_eq!(page.locale.source_route, "/guide/start/");
    assert!(page.locale.is_fallback);
    assert!(
        !analysis.pages[analysis.routes["/zh/guide/start/"]]
            .locale
            .is_fallback
    );
    assert_eq!(analysis.locales["fr"].root_route, "/fr/");
    assert_eq!(
        analysis.locales["fr"].pages["/guide/start/"],
        analysis.routes["/fr/guide/start/"]
    );
}

#[test]
fn graph_keeps_legacy_frontmatter_language_without_translation_pairing() {
    let fixture = Fixture::new(CONFIG);
    fixture.page("index.md", "Home", "lang: fr\n");
    fixture.page("zh/guide.md", "独立文章", "lang: zh\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.pages.len(), 2);
    assert_eq!(analysis.pages[analysis.routes["/"]].lang, "fr");
}

#[test]
fn graph_default_draft_hides_all_versions_and_translation_draft_falls_back() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("secret.md", "Default secret", "draft: true\n");
    fixture.page("zh/secret.md", "Chinese secret", "");
    fixture.page("guide.md", "Public guide", "");
    fixture.page("zh/guide.md", "Unpublished translation", "draft: true\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(analysis
        .published()
        .all(|page| !page.route.contains("secret")));
    let translated = &analysis.pages[analysis.routes["/zh/guide/"]];
    assert!(!translated.draft);
    assert_eq!(translated.title, "Public guide");
    assert_eq!(translated.path, "guide.md");
    let dev = fixture.analyze(Mode::Dev, &Surfaces::default());
    assert_eq!(
        dev.pages[dev.routes["/zh/guide/"]].title,
        "Unpublished translation"
    );
}

#[test]
fn graph_rejects_an_orphan_translation_and_conflicting_content_language() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("zh/orphan.md", "Orphan", "");
    fixture.page("zh/index.md", "中文首页", "lang: fr\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert!(analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "i18n-orphan"
            && issue.file.as_deref() == Some("content/zh/orphan.md")));
    assert!(analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "i18n-content-lang"
            && issue.file.as_deref() == Some("content/zh/index.md")
            && issue.line.is_some()));
}

#[test]
fn graph_checks_conflicts_and_reserved_not_found_for_every_locale() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("zh/404.md", "Reserved", "");
    fixture.page("zh/duplicate.md", "First", "");
    fixture.page("zh/duplicate/index.md", "Second", "");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert!(analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "reserved-route"));
    assert!(analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "route-conflict"));
}

fn registered(name: &str, route: &str, lang: Option<&str>) -> RegisteredPage {
    RegisteredPage {
        draft: None,
        index: None,
        search: None,
        ai: None,
        name: name.into(),
        route: route.into(),
        lang: lang.map(String::from),
        title: name.into(),
        description: Some(format!("{name} description")),
        html: format!("<h1>{name}</h1><p>{name} body</p>"),
        file: "site/src/lib.rs".into(),
    }
}

#[test]
fn graph_pairs_markdown_and_registered_versions_without_cloning_declarations() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("plain.md", "Plain", "");
    fixture.page("zh/component.md", "中文组件", "");
    let surfaces = Surfaces {
        pages: vec![
            registered("EnglishComponent", "/component/", None),
            registered("ChinesePlain", "/plain/", Some("ZH")),
        ],
        ..Default::default()
    };
    let analysis = fixture.analyze(Mode::Production, &surfaces);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let zh = &analysis.pages[analysis.routes["/zh/component/"]];
    assert_eq!(zh.origin, PageOrigin::Markdown);
    assert_eq!(zh.locale.source_route, "/zh/component/");
    let fr = &analysis.pages[analysis.routes["/fr/component/"]];
    assert_eq!(fr.origin, PageOrigin::Registered("EnglishComponent".into()));
    assert_eq!(fr.path, "/component/");
    assert!(fr.locale.is_fallback);
    assert_eq!(surfaces.pages.len(), 2);
    assert_eq!(analysis.pages[analysis.routes["/zh/plain/"]].lang, "zh");
}

#[test]
fn graph_registered_pages_reject_unknown_languages_orphans_and_reserved_prefixes() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    let surfaces = Surfaces {
        pages: vec![
            registered("Unknown", "/unknown/", Some("de")),
            registered("Orphan", "/orphan/", Some("fr")),
            registered("Prefixed", "/zh/guide/", None),
            registered("Conflict", "/", Some("en")),
        ],
        ..Default::default()
    };
    let analysis = fixture.analyze(Mode::Production, &surfaces);
    for rule in [
        "i18n-config",
        "i18n-orphan",
        "reserved-route",
        "route-conflict",
    ] {
        assert!(
            analysis.diagnostics.iter().any(|issue| issue.rule == rule),
            "{rule}: {:?}",
            analysis.diagnostics
        );
    }
}

#[test]
fn graph_rendered_overrides_keep_real_translation_identity() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    let mut surfaces = Surfaces {
        pages: vec![registered("Home", "/", None)],
        ..Default::default()
    };
    surfaces
        .rendered_pages
        .insert("/fr/".into(), "<h1>French environment</h1>".into());
    let analysis = fixture.analyze(Mode::Production, &surfaces);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let fr = &analysis.pages[analysis.routes["/fr/"]];
    assert!(fr.locale.is_fallback);
    assert_eq!(fr.locale.source_lang, "en");
    assert!(fr.fragment.html.contains("French environment"));
    assert_eq!(surfaces.pages.len(), 1);
}

#[test]
fn graph_preserves_configured_tag_spelling_for_markdown_and_registered_pages() {
    let config = format!("{}{}", CONFIG.replace("lang='en'", "lang='EN'"), LOCALES);
    let fixture = Fixture::new(&config);
    fixture.page("index.md", "Home", "lang: EN\n");
    let surfaces = Surfaces {
        pages: vec![registered("Guide", "/guide/", Some("EN"))],
        ..Default::default()
    };
    let analysis = fixture.analyze(Mode::Production, &surfaces);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.pages[analysis.routes["/"]].lang, "en");
    assert_eq!(analysis.pages[analysis.routes["/guide/"]].lang, "en");
    assert_eq!(
        analysis.pages[analysis.routes["/fr/guide/"]]
            .locale
            .source_lang,
        "en"
    );
}

#[test]
fn projection_links_follow_the_ui_language_and_resources_follow_the_real_source() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("guide/target.md", "Target", "");
    fixture.page("zh/guide/target.md", "目标", "");
    fixture.write("content/guide/from.md", "---\ndescription: Links\n---\n# Links\n\n[Relative](target.md) [Logical](/guide/target/) [Explicit](/zh/guide/target/) ![Image](picture.svg)\n");
    fixture.write("content/guide/picture.svg", "<svg/>");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    for (route, expected) in [
        ("/zh/guide/from/", "/zh/guide/target/"),
        ("/fr/guide/from/", "/fr/guide/target/"),
    ] {
        let page = &analysis.pages[analysis.routes[route]];
        for target in &page.targets[..2] {
            assert!(
                matches!(target, content::Target::Page {page, ..} if analysis.pages[*page].route == expected),
                "{route}: {target:?}"
            );
        }
        assert!(
            matches!(&page.targets[2], content::Target::Page {page, ..} if analysis.pages[*page].route == "/zh/guide/target/")
        );
        assert_eq!(
            page.targets[3],
            content::Target::Resource("content/guide/picture.svg".into())
        );
    }
}

#[test]
fn projection_checks_anchors_against_the_actual_target_version() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.write(
        "content/index.md",
        "---\ndescription: Home\n---\n# Home\n\n[Section](target.md#section)\n",
    );
    fixture.write(
        "content/target.md",
        "---\ndescription: Target\n---\n# Target\n\n## Section\n",
    );
    fixture.page("zh/target.md", "目标", "");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert!(
        analysis
            .diagnostics
            .iter()
            .any(|issue| issue.rule == "broken-anchor" && issue.message.contains("zh/target.md")),
        "{:?}",
        analysis.diagnostics
    );
}

#[test]
fn projection_navigation_uses_logical_paths_and_the_current_version_titles() {
    let config = format!("{CONFIG}[[nav.groups]]\nlabel='Guide'\nitems=['guide/a.md','guide/b.md']\n[nav]\ntop=[{{label='Guide',link='guide/a.md'}}]\nsocial=[{{label='Home',link='index.md'}}]\n{LOCALES}\n[i18n.locales.nav]\ngroups=[{{label='Guides',items=['guide/b.md','guide/a.md'],scope='/guide/'}}]\ntop=[{{label='Lire',link='guide/a.md'}}]\nsocial=[{{label='Accueil',link='index.md'}}]\n");
    let fixture = Fixture::new(&config);
    fixture.page("index.md", "Home", "layout: home\n");
    fixture.page("guide/a.md", "A", "sidebar:\n  group: Guide\n");
    fixture.page("guide/b.md", "B", "sidebar:\n  group: Guide\n");
    fixture.page("zh/guide/a.md", "甲", "");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.nav[0].items.len(), 2);
    let zh = &analysis.locales["zh"];
    assert_eq!(zh.nav[0].items[0].route, "/zh/guide/a/");
    assert_eq!(zh.nav[0].items[0].title, "甲");
    assert_eq!(zh.nav[0].items[1].title, "B");
    let fr = &analysis.locales["fr"];
    assert_eq!(fr.nav[0].prefix, "/fr/guide/");
    assert_eq!(fr.nav[0].label, "Guides");
    assert_eq!(fr.nav[0].items[0].route, "/fr/guide/b/");
    assert!(
        matches!(fr.top[0], content::Target::Page {page, ..} if analysis.pages[page].route == "/fr/guide/a/")
    );
    assert!(
        matches!(fr.social[0], content::Target::Page {page, ..} if analysis.pages[page].route == "/fr/")
    );
}

#[test]
fn projection_allowlist_does_not_discover_unselected_translations() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ninclude=['index.md']\n{LOCALES}"
    ));
    fixture.page("index.md", "Home", "");
    fixture.write("content/zh/index.md", "---\nbroken: [\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(analysis.pages[analysis.routes["/zh/"]].locale.is_fallback);
    assert_eq!(
        analysis.pages[analysis.routes["/zh/"]].file,
        "content/index.md"
    );
}

#[test]
fn projection_cache_keys_include_both_languages_catalog_and_included_bytes() {
    let includes = rustify_docs::markdown::IncludeFiles::default();
    let key =
        |source, ui, digest| content::FragmentKey::of("# Source", 0, source, ui, digest, &includes);
    let base = key("en", "fr", "first");
    assert_ne!(base, key("en", "zh", "first"));
    assert_ne!(base, key("zh", "fr", "first"));
    assert_ne!(base, key("en", "fr", "changed"));
}

#[test]
fn projection_translated_source_links_to_missing_translation_in_its_own_language() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("guide/from.md", "From", "");
    fixture.page("guide/target.md", "Target", "");
    fixture.write(
        "content/zh/guide/from.md",
        "---\ndescription: 来源\n---\n# 来源\n\n[目标](target.md) [显式英文](/guide/target/)\n",
    );
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let page = &analysis.pages[analysis.routes["/zh/guide/from/"]];
    assert!(page.targets.iter().all(|target| matches!(target, content::Target::Page{page,..} if analysis.pages[*page].route == "/zh/guide/target/")));
}

#[test]
fn projection_code_group_tab_contract_is_scoped_to_each_ui_language() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}messages='locales/fr.toml'\n"));
    fixture.write("locales/fr.toml", "plain_text='Texte'\n");
    fixture.write("content/index.md", "---\ndescription: Home\n---\n# Home\n\n<!-- rustify:code-group name=\"example\" -->\n\n```\nplain\n```\n\n```rust title=\"Rust\"\nfn main() {}\n```\n\n<!-- rustify:end-code-group -->\n");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert_eq!(
        analysis.errors(),
        0,
        "{:?}",
        analysis
            .diagnostics
            .iter()
            .filter(|issue| issue.severity == rustify_docs::diagnostic::Severity::Error)
            .collect::<Vec<_>>()
    );
    assert!(analysis.pages[analysis.routes["/fr/"]]
        .fragment
        .html
        .contains("Texte"));
}

#[test]
fn projection_static_assets_cannot_take_a_configured_languages_error_file() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.write("static/zh/404.html", "<p>Not a generated error</p>");
    fixture.write("static/foo/404.html", "<p>An ordinary asset</p>");
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert!(analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "output-conflict"
            && issue.file.as_deref() == Some("static/zh/404.html")));
    assert!(!analysis
        .diagnostics
        .iter()
        .any(|issue| issue.rule == "output-conflict"
            && issue.file.as_deref() == Some("static/foo/404.html")));
}

fn generated_html(generation: &rustify_docs::generation::Generation, path: &str) -> String {
    String::from_utf8(generation.files[path].to_vec()).unwrap()
}

#[test]
fn metadata_real_versions_interlink_and_fallbacks_point_to_their_source() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    fixture.page("zh/index.md", "首页", "");
    fixture.page("guide.md", "Guide", "");
    let generation = fixture.generate(&Surfaces::default());
    for (path, canonical, noindex) in [
        ("index.html", "https://example.com/docs/", false),
        ("zh/index.html", "https://example.com/docs/zh/", false),
        ("fr/index.html", "https://example.com/docs/", true),
        (
            "zh/guide/index.html",
            "https://example.com/docs/guide/",
            true,
        ),
    ] {
        let html = generated_html(&generation, path);
        let document = dom_query::Document::from(html.as_str());
        assert_eq!(
            document
                .select("link[rel=canonical]")
                .attr("href")
                .as_deref(),
            Some(canonical),
            "{path}"
        );
        assert_eq!(
            document
                .select("meta[name=robots][content=noindex]")
                .length(),
            usize::from(noindex),
            "{path}"
        );
        let alternates = document.select("link[rel=alternate]");
        if path.ends_with("guide/index.html") {
            assert_eq!(alternates.length(), 1);
        } else {
            assert_eq!(alternates.length(), 2, "{path}");
            assert_eq!(
                document
                    .select("link[rel=alternate][hreflang=en]")
                    .attr("href")
                    .as_deref(),
                Some("https://example.com/docs/")
            );
            assert_eq!(
                document
                    .select("link[rel=alternate][hreflang=zh]")
                    .attr("href")
                    .as_deref(),
                Some("https://example.com/docs/zh/")
            );
        }
        assert_eq!(
            document.select("link[rel=alternate][hreflang=fr]").length(),
            0
        );
    }
    let sitemap = generated_html(&generation, "sitemap.xml");
    assert_eq!(sitemap.matches("<loc>").count(), 3);
    assert!(!sitemap.contains("/fr/"));
    assert!(!sitemap.contains("/zh/guide/"));
}

#[test]
fn metadata_without_a_public_url_omits_absolute_metadata_in_all_languages() {
    let config = CONFIG.replace("url='https://example.com/docs/'\n", "");
    let fixture = Fixture::new(&format!("{config}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    let generation = fixture.generate(&Surfaces::default());
    assert!(!generation.files.contains_key("sitemap.xml"));
    for path in ["index.html", "zh/index.html", "fr/index.html"] {
        let html = generated_html(&generation, path);
        let document = dom_query::Document::from(html.as_str());
        assert_eq!(
            document
                .select("link[rel=canonical], link[rel=alternate]")
                .length(),
            0
        );
    }
}

#[test]
fn metadata_language_errors_are_complete_and_root_templates_are_inert() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    let surfaces = Surfaces {
        slots: ["/404/", "/zh/404/", "/fr/404/"]
            .into_iter()
            .map(|route| {
                (
                    route.into(),
                    std::collections::BTreeMap::from([(
                        rustify_docs::site::Slot::BeforeContent,
                        format!("<p id='shared-error-slot'>{route}</p>"),
                    )]),
                )
            })
            .collect(),
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces);
    assert_eq!(
        generation.locale_not_found,
        std::collections::BTreeMap::from([
            ("".into(), "404.html".into()),
            ("zh".into(), "zh/404.html".into()),
            ("fr".into(), "fr/404.html".into()),
        ])
    );
    for (lang, path, home) in [
        ("en", "404.html", "/docs/"),
        ("zh", "zh/404.html", "/docs/zh/"),
        ("fr", "fr/404.html", "/docs/fr/"),
    ] {
        let html = generated_html(&generation, path);
        let document = dom_query::Document::from(html.as_str());
        assert_eq!(document.select("html").attr("lang").as_deref(), Some(lang));
        assert_eq!(
            document
                .select("head link[rel=canonical], head link[rel=alternate]")
                .length(),
            0
        );
        assert_eq!(
            document
                .select("meta[name=robots][content=noindex]")
                .length(),
            1
        );
        assert!(html.contains(&format!("href=\"{home}\"")));
        assert!(
            rustify_docs::html_check::check_document(&html).is_empty(),
            "{path}"
        );
    }
    let root = generated_html(&generation, "404.html");
    assert_eq!(root.matches("data-rd-not-found-prefix=").count(), 2);
    for home in ["/docs/", "/docs/zh/", "/docs/fr/"] {
        assert!(root.contains(&format!("href=\"{home}\"")));
    }
}

#[test]
fn metadata_first_build_shell_has_each_languages_own_catalog() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}messages='locales/fr.toml'\n"));
    fixture.write(
        "locales/fr.toml",
        "[dev]\nshell_title='Site indisponible'\nshell_body='Corrigez les erreurs.'\n",
    );
    let (project, _) = Project::load(&fixture.0).unwrap();
    let generation = rustify_docs::generation::dev_shell(&project).unwrap();
    assert_eq!(generation.locale_not_found["fr"], "fr/404.html");
    let html = generated_html(&generation, "fr/404.html");
    let document = dom_query::Document::from(html.as_str());
    assert_eq!(document.select("html").attr("lang").as_deref(), Some("fr"));
    assert_eq!(document.select("h1").text().as_ref(), "Site indisponible");
    assert_eq!(document.select("h1").attr("lang").as_deref(), Some("fr"));
    assert!(html.contains("Corrigez les erreurs."));
    let script = document.select("script[data-labels]");
    let words: serde_json::Value =
        serde_json::from_str(&script.attr("data-labels").unwrap()).unwrap();
    let languages: serde_json::Value =
        serde_json::from_str(&script.attr("data-label-langs").unwrap()).unwrap();
    assert_eq!(words["shell_title"], "Site indisponible");
    assert_eq!(languages["shell_title"], "fr");
    assert_eq!(languages["building"], "en");
}

#[test]
fn metadata_final_gate_checks_generated_absolute_page_addresses() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.page("index.md", "Home", "");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, _) = content::snapshot(&project);
    let mut analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    let home = analysis.routes["/"];
    analysis.pages[home].locale.source_route = "/not-published/".into();
    let problems =
        rustify_docs::generation::generate(&project, &snapshot, &analysis, &Default::default(), 1)
            .err()
            .expect("an absolute canonical must name an emitted page");
    assert!(problems
        .iter()
        .any(|issue| issue.rule == "output-check" && issue.message.contains("not-published")));
}
