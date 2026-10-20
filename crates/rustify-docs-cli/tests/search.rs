mod support;
use support::*;

#[test]
fn ai_files_are_deterministic_and_failed_candidates_leave_the_old_directory() {
    let fixture = copy_fixture("content");
    let site = fixture.path("site");
    succeeded(&run_in(&site, &["build"]));
    let first = tree(&site.join("dist"));
    assert!(first.contains_key("llms.txt"));
    succeeded(&run_in(&site, &["build"]));
    assert_eq!(first, tree(&site.join("dist")));
    fixture.write(
        "site/content/empty.md",
        "---\ntitle: Empty\ndescription: '   '\n---\n# Empty\n",
    );
    for command in ["check", "build"] {
        let output = run_in(&site, &[command, "--format", "json"]);
        assert_eq!(output.status.code(), Some(1));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|error| error["rule"] == "missing-description"
                && error["file"] == "content/empty.md"));
        assert_eq!(first, tree(&site.join("dist")));
    }
}

#[test]
fn ai_empty_home_check_and_build_still_succeed_with_a_nonempty_index_description() {
    let fixture = copy_fixture("content");
    fixture.write(
        "site/content/index.md",
        "---\nlayout: home\ntitle: Home\ndescription: '   '\n---\n",
    );
    for command in ["check", "build"] {
        succeeded(&run_in(&fixture.path("site"), &[command]));
    }
    let text = fixture.read("site/dist/llms.txt");
    assert!(text.contains("Homepage of "), "{text}");
}

/// The repository's own documentation site, analysed as a production build
/// sees it, with the registered pages standing in for the Rust surfaces.
struct Official {
    project: rustify_docs::content::Project,
    snapshot: rustify_docs::content::Snapshot,
    manifest: rustify_docs_syntax::Manifest,
    analysis: rustify_docs::content::Analysis,
}

fn official() -> Official {
    use rustify_docs::content::{self, FragmentCache, Mode, Project, RegisteredPage, Surfaces};
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (project, _) = Project::load(&repository).unwrap();
    let (snapshot, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let registry = std::fs::read_to_string(repository.join("site/src/lib.rs")).unwrap();
    let manifest = rustify_docs_syntax::parse_registry(&registry)
        .unwrap()
        .manifest();
    let surfaces = Surfaces {
        pages: manifest
            .pages
            .iter()
            .map(|page| RegisteredPage {
                name: page.name.clone(),
                route: page.route.clone(),
                lang: page.lang.clone(),
                title: page.title.clone(),
                description: page.description.clone(),
                draft: page.draft,
                index: page.index,
                search: page.search,
                ai: page.ai,
                html: format!("<h1>{}</h1>", page.title),
                file: "site/src/lib.rs".into(),
            })
            .collect(),
        ..Default::default()
    };
    let analysis = content::analyze_with_surfaces(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &rustify_docs::highlight::Highlighter::new(),
        1,
        &surfaces,
    );
    Official {
        project,
        snapshot,
        manifest,
        analysis,
    }
}

#[test]
fn official_sources_have_only_two_real_chinese_pages_and_keep_the_existing_catalog() {
    let Official {
        project,
        snapshot,
        manifest,
        analysis,
    } = official();
    assert_eq!(
        project
            .config
            .locales
            .iter()
            .map(|locale| locale.lang.as_str())
            .collect::<Vec<_>>(),
        ["en", "zh"]
    );
    assert_eq!(snapshot.markdown.len(), 13);
    assert_eq!(
        snapshot
            .markdown
            .keys()
            .filter(|path| path.starts_with("zh/"))
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["zh/quickstart.md"]
    );
    assert_eq!(manifest.demos.len(), 24);
    assert_eq!(manifest.pages.len(), 26);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.published().count(), 74);
    let zh = &analysis.locales["zh"];
    let real: Vec<_> = zh
        .pages
        .values()
        .map(|&i| &analysis.pages[i])
        .filter(|page| !page.locale.is_fallback)
        .map(|page| page.locale.logical_route.as_str())
        .collect();
    assert_eq!(real, ["/", "/quickstart/"]);
    assert_eq!(
        zh.nav
            .iter()
            .map(|group| group.label.as_str())
            .collect::<Vec<_>>(),
        ["指南", "组件"]
    );
    assert_eq!(zh.nav[0].items.len(), 12);
    assert_eq!(zh.nav[1].items.len(), 24);
    assert!(zh
        .nav
        .iter()
        .flat_map(|group| &group.items)
        .all(|item| item.route.starts_with("/zh/")));
    let pages = analysis
        .published()
        .map(|page| page.file.as_str())
        .collect::<Vec<_>>();
    assert!(pages.iter().all(|file| !file.contains("docs/plan/")
        && !file.contains("docs/validation/")
        && !file.contains("PRD")));
}

#[test]
fn official_guides_declare_one_sentence_descriptions() {
    let Official { analysis, .. } = official();
    let guides = [
        "quickstart",
        "architecture",
        "navigation",
        "workspace",
        "forms",
        "i18n",
        "data",
        "themes",
        "utility",
        "compatibility",
        "components",
        "vellum",
    ]
    .map(|name| format!("docs/{name}.md"));
    let english = |file: &str| {
        analysis
            .pages
            .iter()
            .find(|page| page.file == file && page.locale.ui_lang == "en")
            .unwrap_or_else(|| panic!("no English page for {file}"))
    };
    for file in &guides {
        let page = english(file);
        let description = page
            .front
            .description
            .as_deref()
            .unwrap_or_else(|| panic!("{file} has no `description:`"));
        let body = description.strip_suffix('.').unwrap_or_else(|| {
            panic!("{file}: the description does not end with a full stop: {description:?}")
        });
        assert!(
            ![". ", "? ", "! "].iter().any(|stop| body.contains(stop)),
            "{file}: more than one sentence: {description:?}"
        );
        assert!(
            description.chars().count() <= 200,
            "{file}: longer than 200 characters: {description:?}"
        );
        assert_ne!(description, page.title, "{file} repeats its title");
        assert!(
            !description.contains(": ") && !description.contains(" #"),
            "{file}: not a plain YAML scalar: {description:?}"
        );
        assert!(
            !description.as_bytes().windows(3).any(|window| {
                window[0].is_ascii_digit() && window[1] == b'.' && window[2].is_ascii_digit()
            }),
            "{file}: names a version: {description:?}"
        );
    }
    assert!(!analysis.diagnostics.iter().any(|diagnostic| {
        diagnostic.rule == "missing-description"
            && diagnostic
                .file
                .as_ref()
                .is_some_and(|file| guides.contains(file))
    }));
    let fallbacks: Vec<_> = analysis
        .pages
        .iter()
        .filter(|page| page.locale.ui_lang == "zh" && page.locale.is_fallback)
        .filter(|page| guides.contains(&page.file))
        .collect();
    assert_eq!(fallbacks.len(), 11);
    for page in fallbacks {
        assert_eq!(
            page.front.description,
            english(&page.file).front.description,
            "{} falls back to another description",
            page.route
        );
    }
}
