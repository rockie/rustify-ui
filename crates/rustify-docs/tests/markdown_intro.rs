use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;
use rustify_docs::content::{self, Analysis, FragmentCache, Mode, Project, Snapshot, Surfaces};
use rustify_docs::highlight::Highlighter;

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "rustify-markdown-intro-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::create_dir_all(root.join("static")).unwrap();
        std::fs::write(root.join("rustify-docs.toml"), "schema_version=1\n[site]\nname='Site'\nbase='/docs/'\n[content]\ndir='docs'\ninclude=['guide.md']\n[repo]\nurl='https://github.com/example/sdk'\nref='main'\n").unwrap();
        std::fs::write(
            root.join("docs/guide.md"),
            "# Guide\n\n## Installation\n\nRead here.\n",
        )
        .unwrap();
        Self(root)
    }

    fn analyze(&self) -> (Project, Snapshot, Analysis) {
        let (project, _) = Project::load(&self.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Production,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
        );
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        (project, snapshot, analysis)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn introduction_uses_original_markdown_and_source_links_without_the_readme_heading() {
    let fixture = Fixture::new();
    let original = "# Name\r\n\r\nRésumé — [SDK](docs/guide.md#installation) and [repository quick start](#quick-start).\r\n\r\n```rust\r\n## This is code\r\n```\r\n\r\n## Quick start\r\n\r\nDo not include this section.\r\n";
    std::fs::write(fixture.0.join("README.md"), original).unwrap();
    let (project, snapshot, mut analysis) = fixture.analyze();
    let intro = content::markdown_intro(
        &project,
        &snapshot,
        &analysis,
        "README.md",
        &Highlighter::new(),
    )
    .unwrap();
    assert!(intro.diagnostics.is_empty(), "{:?}", intro.diagnostics);
    assert!(intro.html.contains("Résumé —"));
    assert!(intro.html.contains("href=\"/docs/guide/#installation\""));
    assert!(intro
        .html
        .contains("https://github.com/example/sdk/blob/main/README.md#quick-start"));
    assert!(!intro.html.contains("<h1") && !intro.html.contains("Do not include this section"));
    analysis.inputs.extend(intro.inputs);
    assert!(content::validate_inputs(&project, &snapshot, &analysis).is_empty());
    assert!(!analysis.resources.contains("README.md"));
}

#[test]
fn the_original_read_survives_reanalysis_and_rejects_a_later_readme_change() {
    let fixture = Fixture::new();
    std::fs::write(
        fixture.0.join("README.md"),
        "# Name\n\nOriginal introduction.\n\n## Quick start\n",
    )
    .unwrap();
    let (project, snapshot, analysis) = fixture.analyze();
    let intro = content::markdown_intro(
        &project,
        &snapshot,
        &analysis,
        "README.md",
        &Highlighter::new(),
    )
    .unwrap();
    std::fs::write(
        fixture.0.join("README.md"),
        "# Name\n\nA later introduction.\n\n## Quick start\n",
    )
    .unwrap();
    let mut rebuilt = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    rebuilt.inputs.extend(intro.inputs);
    assert!(content::validate_inputs(&project, &snapshot, &rebuilt)
        .iter()
        .any(|problem| problem.rule == "render-generation-changed"));
}

#[test]
fn nested_source_images_enter_the_common_resource_graph_and_keep_the_first_read() {
    let fixture = Fixture::new();
    std::fs::create_dir_all(fixture.0.join("notes/media")).unwrap();
    std::fs::write(
        fixture.0.join("notes/README.md"),
        "# Name\n\n![Probe](media/probe.svg)\n\n## Quick start\n",
    )
    .unwrap();
    let image = fixture.0.join("notes/media/probe.svg");
    std::fs::write(
        &image,
        "<svg xmlns='http://www.w3.org/2000/svg' width='8' height='8'></svg>",
    )
    .unwrap();
    let (project, snapshot, analysis) = fixture.analyze();
    let intro = content::markdown_intro(
        &project,
        &snapshot,
        &analysis,
        "notes/README.md",
        &Highlighter::new(),
    )
    .unwrap();
    assert!(intro.diagnostics.is_empty(), "{:?}", intro.diagnostics);
    assert!(intro.html.contains("src=\"notes/media/probe.svg\""));
    std::fs::write(
        &image,
        "<svg xmlns='http://www.w3.org/2000/svg' width='9' height='9'></svg>",
    )
    .unwrap();
    let surfaces = Surfaces {
        pages: vec![content::RegisteredPage {
            lang: None,
            name: "Home".into(),
            route: "/".into(),
            title: "Home".into(),
            description: None,
            html: intro.html,
            file: "site/src/pages.rs".into(),
        }],
        ..Surfaces::default()
    };
    let mut rebuilt = content::analyze_with_surfaces(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
        &surfaces,
    );
    rebuilt.inputs.extend(intro.inputs);
    assert!(rebuilt.resources.contains("notes/media/probe.svg"));
    assert!(!rebuilt.resources.contains("notes/README.md"));
    assert!(content::validate_inputs(&project, &snapshot, &rebuilt)
        .iter()
        .any(|problem| problem.rule == "render-generation-changed"));
}

#[test]
fn introduction_diagnostics_keep_the_original_file_line() {
    let fixture = Fixture::new();
    std::fs::write(
        fixture.0.join("README.md"),
        "# Name\n\nBad [link](docs/missing.md).\n\n## Quick start\n",
    )
    .unwrap();
    let (project, snapshot, analysis) = fixture.analyze();
    let intro = content::markdown_intro(
        &project,
        &snapshot,
        &analysis,
        "README.md",
        &Highlighter::new(),
    )
    .unwrap();
    assert!(
        intro
            .diagnostics
            .iter()
            .any(|problem| problem.file.as_deref() == Some("README.md")
                && problem.line == Some(3)
                && problem.rule == "broken-link"),
        "{:?}",
        intro.diagnostics
    );
}

#[test]
fn private_markdown_cannot_be_used_as_an_introduction() {
    let fixture = Fixture::new();
    std::fs::write(
        fixture.0.join(".env.md"),
        "# Private\n\nDo not expose this.\n",
    )
    .unwrap();
    let (project, snapshot, analysis) = fixture.analyze();
    assert!(content::markdown_intro(
        &project,
        &snapshot,
        &analysis,
        ".env.md",
        &Highlighter::new()
    )
    .is_err());
}

#[test]
fn the_helper_emits_only_the_source_request_and_a_static_placeholder() {
    let registry = rustify_docs::SiteRegistry::new(serde_json::json!({})).page("Home", |context| {
        view! { <section>{context.markdown_intro("README.md")}</section> }
    });
    let result = registry.render(
        &rustify_docs::site::RenderRequest {
            key: "home".into(),
            seed: "home".into(),
            target: rustify_docs::site::RenderTarget::Page {
                name: "Home".into(),
                route: "/".into(),
            },
        },
        &rustify_docs::site::SiteEnvironment::default(),
    );
    assert!(result.diagnostics.is_empty());
    assert_eq!(result.markdown_intros.len(), 1);
    assert_eq!(result.markdown_intros[0].path, "README.md");
    assert!(result
        .html
        .unwrap()
        .contains("data-rustify-markdown-placeholder"));
    assert!(result.demos.is_empty());
}
