#![cfg(feature = "pipeline")]

use rustify_docs::content::{
    self, Diagnostic, FragmentCache, Mode, Project, RegisteredPage, Surfaces,
};
use rustify_docs::generation::{self, Generation};
use rustify_docs::highlight::Highlighter;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const CONFIG: &str = "schema_version=1\n[site]\nname='Docs'\ndescription='A useful library.'\nlang='en'\nurl='https://example.com/docs/'\nbase='/docs/'\n";
const LOCALES: &str = "[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n[[i18n.locales]]\nlang='zh'\nlabel='中文'\nprefix='zh'\n";

struct Fixture(PathBuf);
impl Fixture {
    fn new(config: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "rustify-docs-ai-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let fixture = Self(path);
        fixture.write("rustify-docs.toml", config);
        fixture
    }
    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    fn generate(&self, surfaces: &Surfaces) -> Result<Generation, Vec<Diagnostic>> {
        let (project, _) = Project::load(&self.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        let analysis = content::analyze_with_surfaces(
            &project,
            &snapshot,
            Mode::Production,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
            surfaces,
        );
        if analysis.errors() != 0 {
            return Err(analysis.diagnostics);
        }
        generation::generate_extended(
            &project,
            &snapshot,
            &analysis,
            &Default::default(),
            1,
            &Default::default(),
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn text(generation: &Generation, path: &str) -> String {
    String::from_utf8(
        generation
            .files
            .get(path)
            .unwrap_or_else(|| panic!("missing {path}"))
            .to_vec(),
    )
    .unwrap()
}
fn custom(name: &str, route: &str, description: Option<&str>) -> RegisteredPage {
    RegisteredPage {
        name: name.into(),
        route: route.into(),
        title: name.into(),
        description: description.map(str::to_owned),
        lang: None,
        draft: None,
        index: None,
        search: None,
        ai: None,
        html: format!("<h1>{name}</h1><p>Never infer a registered description from HTML.</p>"),
        file: "src/pages.rs".into(),
    }
}

#[test]
fn ai_uses_language_navigation_source_addresses_and_independent_policies() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}\n[[nav.groups]]\nlabel='Guides'\nitems=['second.md','first.md']\n[[nav.groups]]\nlabel='Registered'\nroutes=['/custom/','/undescribed/']\n"));
    fixture.write("content/index.md", "---\nlayout: home\ntitle: Home\n---\n");
    fixture.write(
        "content/first.md",
        "---\ndescription: First description\nsearch: false\nindex: false\n---\n# First\n",
    );
    fixture.write(
        "content/second.md",
        "---\ndescription: Second description\n---\n# Second\n",
    );
    fixture.write(
        "content/zh/second.md",
        "---\ndescription: 中文介绍\n---\n# 第二页\n",
    );
    fixture.write(
        "content/other.md",
        "---\ndescription: Unlisted description\n---\n# Unlisted\n",
    );
    fixture.write(
        "content/excluded.md",
        "---\ndescription: AI_SECRET\nai: false\n---\n# Excluded\n",
    );
    fixture.write(
        "content/draft.md",
        "---\ndescription: DRAFT_SECRET\ndraft: true\n---\n# Draft\n",
    );
    fixture.write(
        "content/zh/draft.md",
        "---\ndescription: TRANSLATED_DRAFT_SECRET\n---\n# 草稿\n",
    );
    let generation = fixture
        .generate(&Surfaces {
            pages: vec![
                custom("Custom", "/custom/", Some("Custom description")),
                custom("Undescribed", "/undescribed/", Some("  \n  ")),
            ],
            ..Default::default()
        })
        .unwrap();
    let en = text(&generation, "llms.txt");
    let zh = text(&generation, "zh/llms.txt");
    assert!(en.starts_with("# Docs\n\n> A useful library.\n"));
    assert!(en.find("[Second]").unwrap() < en.find("[First]").unwrap());
    assert!(
        en.contains("## Guides") && en.contains("## Registered") && en.contains("## Other pages")
    );
    assert!(en.contains("[First](https://example.com/docs/first/): First description"));
    assert!(zh.contains("https://example.com/docs/zh/second/"));
    assert!(zh.contains("https://example.com/docs/first/"));
    assert!(!zh.contains("https://example.com/docs/zh/first/"));
    assert!(zh.contains("en") && zh.contains("尚未翻译"));
    for index in [&en, &zh] {
        assert_eq!(
            index.lines().filter(|line| line.starts_with("- [")).count(),
            5
        );
        for private in [
            "SECRET",
            "Undescribed",
            "404",
            ".md",
            "src/",
            "file:",
            fixture.0.to_str().unwrap(),
        ] {
            assert!(!index.contains(private), "{private}: {index}");
        }
    }
}

#[test]
fn ai_home_description_candidates_trim_and_fall_back_without_changing_page_metadata() {
    for (metadata, body, expected) in [
        ("", "", "Homepage of Docs."),
        ("description: ''\n", "", "Homepage of Docs."),
        (
            "description: '   '\ntagline: '  '\n",
            "",
            "Homepage of Docs.",
        ),
        ("tagline: A home tagline\n", "", "A home tagline"),
        (
            "description: '  '\ntagline: Tagline\n",
            "A first paragraph.\n",
            "A first paragraph.",
        ),
        (
            "description: Explicit description\ntagline: Tagline\n",
            "A first paragraph.\n",
            "Explicit description",
        ),
    ] {
        let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
        fixture.write(
            "content/index.md",
            &format!("---\nlayout: home\ntitle: Home\n{metadata}---\n{body}"),
        );
        let generation = fixture.generate(&Surfaces::default()).unwrap();
        for path in ["llms.txt", "zh/llms.txt"] {
            let output = text(&generation, path);
            assert!(output.contains(expected), "{metadata}: {output}");
            assert_eq!(
                output
                    .lines()
                    .filter(|line| line.starts_with("- ["))
                    .count(),
                1
            );
        }
        let dom = dom_query::Document::from(text(&generation, "index.html").as_str());
        if metadata.is_empty() && body.is_empty() {
            assert_eq!(
                dom.select("meta[name=description]")
                    .attr("content")
                    .as_deref(),
                Some("")
            );
        }
    }
}

#[test]
fn ai_home_fallback_description_uses_the_real_source_language() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.write("content/index.md", "---\nlayout: home\ntitle: Home\n---\n");
    fixture.write(
        "content/zh/index.md",
        "---\nlayout: home\ntitle: 首页\ndescription: '  '\n---\n",
    );
    let generation = fixture.generate(&Surfaces::default()).unwrap();
    assert!(text(&generation, "llms.txt").contains("Homepage of Docs."));
    let zh = text(&generation, "zh/llms.txt");
    assert!(zh.contains("Docs首页"));
    assert!(zh.contains("https://example.com/docs/zh/"));
    assert!(!zh.contains("Homepage of Docs."));
}

#[test]
fn ai_empty_registered_description_is_excluded_even_at_home() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    let generation = fixture
        .generate(&Surfaces {
            pages: vec![custom("Registered home", "/", None)],
            ..Default::default()
        })
        .unwrap();
    for path in ["llms.txt", "zh/llms.txt"] {
        assert!(!text(&generation, path).contains("- ["));
    }
}

#[test]
fn ai_non_home_blank_description_requires_a_paragraph_or_opt_out() {
    let fixture = Fixture::new(CONFIG);
    fixture.write(
        "content/index.md",
        "---\ntitle: Empty\ndescription: '   '\n---\n# Empty\n",
    );
    let errors = match fixture.generate(&Surfaces::default()) {
        Ok(_) => panic!("blank non-Home must fail"),
        Err(errors) => errors,
    };
    assert!(errors
        .iter()
        .any(|error| error.rule == "missing-description"
            && error.file.as_deref() == Some("content/index.md")));
    fixture.write(
        "content/index.md",
        "---\ntitle: Empty\ndescription: '   '\nai: false\n---\n# Empty\n",
    );
    assert!(fixture.generate(&Surfaces::default()).is_ok());
}

#[test]
fn ai_root_relative_addresses_and_author_text_stay_single_safe_entries() {
    let fixture = Fixture::new(&CONFIG.replace("url='https://example.com/docs/'\n", ""));
    fixture.write("content/index.md", "---\nlayout: home\ntitle: Home\n---\n");
    fixture.write("content/file (x).md", "---\ntitle: 'Title [brackets] (parentheses)'\ndescription: |-\n  First line.\n  - fake item [escape](https://evil.invalid/)\n---\n");
    let first = fixture.generate(&Surfaces::default()).unwrap();
    let output = text(&first, "llms.txt");
    assert!(output.contains("/docs/file%20%28x%29/"), "{output}");
    assert!(output.contains("Title \\[brackets\\]"), "{output}");
    assert_eq!(
        output
            .lines()
            .filter(|line| line.starts_with("- ["))
            .count(),
        2
    );
    assert!(!output.contains("\n- fake"));
    assert_eq!(
        first.files,
        fixture.generate(&Surfaces::default()).unwrap().files
    );
}

#[test]
fn ai_rebuild_removes_deleted_pages_and_changed_language_outputs() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}"));
    fixture.write("content/index.md", "---\nlayout: home\ntitle: Home\n---\n");
    fixture.write(
        "content/gone.md",
        "---\ndescription: Gone description\n---\n# Gone\n",
    );
    let before = fixture.generate(&Surfaces::default()).unwrap();
    assert!(text(&before, "zh/llms.txt").contains("Gone"));
    std::fs::remove_file(fixture.0.join("content/gone.md")).unwrap();
    fixture.write("rustify-docs.toml", CONFIG);
    let after = fixture.generate(&Surfaces::default()).unwrap();
    assert!(!after.files.contains_key("zh/llms.txt"));
    assert!(!text(&after, "llms.txt").contains("Gone"));
}

#[test]
fn ai_home_uses_the_source_catalog_and_normalizes_its_description() {
    let fixture = Fixture::new(&format!("{CONFIG}{LOCALES}\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\nmessages='locales/fr.toml'\n"));
    fixture.write("locales/fr.toml", "[ai]\nhome_description=\"Accueil de {site}.\\n   Une documentation.\"\nother_pages='Autres pages'\n");
    fixture.write("content/index.md", "---\nlayout: home\ntitle: Home\n---\n");
    fixture.write(
        "content/fr/index.md",
        "---\nlayout: home\ntitle: Accueil\n---\n",
    );
    let generation = fixture.generate(&Surfaces::default()).unwrap();
    let output = text(&generation, "fr/llms.txt");
    assert!(
        output.contains(": Accueil de Docs. Une documentation.\n"),
        "{output}"
    );
    assert!(output.contains("## Autres pages"));
    assert!(text(&generation, "zh/llms.txt").contains("Homepage of Docs."));
}
