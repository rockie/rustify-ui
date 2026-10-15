use std::sync::Arc;

use rustify_docs::content::{self, FragmentCache, Mode, Project, Surfaces};
use rustify_docs::diagnostic::{LineIndex, Severity};
use rustify_docs::highlight::Highlighter;

use super::{Fixture, CONFIG, LOCALES};

fn config_with_messages(path: &str) -> String {
    format!("{CONFIG}{LOCALES}messages='{path}'\n")
}

#[test]
fn projection_messages_are_separate_declared_inputs_with_one_shared_receipt() {
    let config = config_with_messages("locales/shared.toml")
        .replace("prefix='zh'", "prefix='zh'\nmessages='locales/shared.toml'");
    let fixture = Fixture::new(&config);
    fixture.page("index.md", "Home", "");
    fixture.write("locales/shared.toml", "menu='Lire'\n");
    fixture.write("locales/unconfigured.toml", "this is not valid TOML");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(snapshot.messages.len(), 1);
    assert_eq!(
        snapshot.messages["locales/shared.toml"].as_ref(),
        "menu='Lire'\n"
    );
    assert_eq!(
        snapshot
            .inputs
            .byte_paths()
            .filter(|(alias, _)| *alias == "locales/shared.toml")
            .count(),
        1
    );
    assert!(!snapshot
        .inputs
        .byte_paths()
        .any(|(alias, _)| alias == "locales/unconfigured.toml"));
    assert!(!snapshot.files.contains_key("locales/shared.toml"));
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.catalogs["fr"].get("menu").text, "Lire");
    assert_eq!(analysis.catalogs["fr"].get("menu").lang, "fr");
    assert_eq!(analysis.catalogs["zh"].get("menu").text, "Lire");
    assert_eq!(analysis.catalogs["zh"].get("menu").lang, "zh");
    assert!(!analysis.resources.contains("locales/shared.toml"));
    assert!(!analysis.outside.contains_key("locales/shared.toml"));
}

#[test]
fn projection_messages_do_not_expand_the_content_allowlist() {
    let config =
        format!("{CONFIG}[content]\ninclude=['index.md']\n{LOCALES}messages='locales/fr.toml'\n");
    let fixture = Fixture::new(&config);
    fixture.page("index.md", "Home", "");
    fixture.write("content/zh/index.md", "---\nbroken: [\n");
    fixture.write("content/unselected.toml", "this is not valid TOML");
    fixture.write("locales/fr.toml", "menu='Lire'\n");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(
        snapshot
            .markdown
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["index.md"]
    );
    assert!(snapshot.files.is_empty());
    assert!(snapshot.messages.contains_key("locales/fr.toml"));
    assert!(!snapshot
        .inputs
        .byte_paths()
        .any(|(alias, _)| alias == "content/zh/index.md" || alias == "content/unselected.toml"));
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(analysis.pages[analysis.routes["/zh/"]].locale.is_fallback);
    assert_eq!(analysis.catalogs["fr"].get("menu").text, "Lire");
}

#[test]
fn projection_snapshot_message_bytes_bind_analysis_and_generation_validation() {
    let fixture = Fixture::new(&config_with_messages("locales/fr.toml"));
    fixture.page("index.md", "Home", "");
    fixture.write("locales/fr.toml", "menu='Lire'\n");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (before, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    fixture.write("locales/fr.toml", "menu='Ouvrir'\n");
    let analysis = content::analyze(
        &project,
        &before,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert_eq!(analysis.catalogs["fr"].get("menu").text, "Lire");
    assert!(content::validate_inputs(&project, &before, &analysis)
        .iter()
        .any(|problem| problem.rule == "render-generation-changed"
            && problem.file.as_deref() == Some("locales/fr.toml")));
}

#[test]
fn projection_message_changes_refresh_controls_and_keep_other_locale_cache_entries() {
    let fixture = Fixture::new(&config_with_messages("locales/fr.toml"));
    fixture.write(
        "content/index.md",
        "---\ndescription: Home\n---\n# Home\n\n```text\nunchanged code\n```\n",
    );
    fixture.write("locales/fr.toml", "copy='Copier'\n");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (before, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let highlighter = Highlighter::new();
    let mut cache = FragmentCache::default();
    let first = content::analyze(
        &project,
        &before,
        Mode::Production,
        &mut cache,
        &highlighter,
        1,
    );
    assert_eq!(first.errors(), 0, "{:?}", first.diagnostics);
    let first_fr = &first.pages[first.routes["/fr/"]].fragment;
    let first_en = &first.pages[first.routes["/"]].fragment;
    assert!(first_fr.html.contains("Copier"), "{}", first_fr.html);
    let cached = content::analyze(
        &project,
        &before,
        Mode::Production,
        &mut cache,
        &highlighter,
        1,
    );
    assert!(Arc::ptr_eq(
        first_fr,
        &cached.pages[cached.routes["/fr/"]].fragment
    ));
    fixture.write("locales/fr.toml", "copy='Recopier'\n");
    assert!(content::validate_inputs(&project, &before, &cached)
        .iter()
        .any(|problem| problem.rule == "render-generation-changed"
            && problem.file.as_deref() == Some("locales/fr.toml")));
    let (after, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let next = content::analyze(
        &project,
        &after,
        Mode::Production,
        &mut cache,
        &highlighter,
        1,
    );
    assert_eq!(next.errors(), 0, "{:?}", next.diagnostics);
    let next_fr = &next.pages[next.routes["/fr/"]].fragment;
    assert!(next_fr.html.contains("Recopier"), "{}", next_fr.html);
    assert!(!Arc::ptr_eq(first_fr, next_fr));
    assert!(Arc::ptr_eq(
        first_en,
        &next.pages[next.routes["/"]].fragment
    ));
    assert!(content::validate_inputs(&project, &after, &next).is_empty());
}

#[test]
fn projection_message_source_bytes_invalidate_the_cache_even_with_identical_labels() {
    let fixture = Fixture::new(&config_with_messages("locales/fr.toml"));
    fixture.page("index.md", "Home", "");
    fixture.write("locales/fr.toml", "menu='Lire'\n");
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (before, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let highlighter = Highlighter::new();
    let mut cache = FragmentCache::default();
    let first = content::analyze(
        &project,
        &before,
        Mode::Production,
        &mut cache,
        &highlighter,
        1,
    );
    assert_eq!(first.errors(), 0, "{:?}", first.diagnostics);
    fixture.write(
        "locales/fr.toml",
        "# same labels, different input\nmenu='Lire'\n",
    );
    let (after, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let next = content::analyze(
        &project,
        &after,
        Mode::Production,
        &mut cache,
        &highlighter,
        1,
    );
    assert_eq!(next.errors(), 0, "{:?}", next.diagnostics);
    assert_eq!(first.catalogs["fr"].digest(), next.catalogs["fr"].digest());
    assert!(!Arc::ptr_eq(
        &first.pages[first.routes["/fr/"]].fragment,
        &next.pages[next.routes["/fr/"]].fragment,
    ));
    assert!(Arc::ptr_eq(
        &first.pages[first.routes["/"]].fragment,
        &next.pages[next.routes["/"]].fragment,
    ));
}

#[test]
fn projection_messages_in_scanned_content_or_assets_are_refused_as_resources() {
    for path in [
        "content/messages.toml",
        "static/messages.toml",
        "content/catalog.md",
    ] {
        let fixture = Fixture::new(&config_with_messages(path));
        fixture.page("index.md", "Home", "");
        fixture.write(path, "menu='Lire'\n");
        let (project, _) = Project::load(&fixture.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        assert!(
            problems
                .iter()
                .any(|problem| problem.rule == "i18n-messages"
                    && problem.severity == Severity::Error),
            "{path}: {problems:?}"
        );
        assert!(snapshot.messages.contains_key(path));
        assert!(!snapshot.files.contains_key(path));
        assert!(!snapshot.markdown.contains_key("catalog.md"));
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Production,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
        );
        assert!(!analysis.resources.contains(path));
        assert!(!analysis.outside.contains_key(path));
    }
}

#[test]
fn projection_explicit_message_resource_links_are_errors_and_never_become_resources() {
    let fixture = Fixture::new(&config_with_messages("locales/fr.toml"));
    fixture.write("locales/fr.toml", "menu='Lire'\n");
    fixture.write(
        "content/index.md",
        "---\ndescription: Home\n---\n# Home\n\n[Raw messages](../locales/fr.toml)\n",
    );
    let analysis = fixture.analyze(Mode::Production, &Surfaces::default());
    assert!(
        analysis
            .diagnostics
            .iter()
            .any(|problem| problem.rule == "i18n-messages"
                && problem.severity == Severity::Error
                && problem.file.as_deref() == Some("content/index.md")),
        "{:?}",
        analysis.diagnostics
    );
    assert!(!analysis.resources.contains("locales/fr.toml"));
    assert!(!analysis.outside.contains_key("locales/fr.toml"));
}

#[test]
fn projection_unreadable_message_inputs_report_errors_at_the_messages_declaration() {
    for kind in ["missing", "directory", "not-utf8"] {
        let config = config_with_messages("locales/fr.toml");
        let fixture = Fixture::new(&config);
        fixture.page("index.md", "Home", "");
        match kind {
            "directory" => std::fs::create_dir_all(fixture.0.join("locales/fr.toml")).unwrap(),
            "not-utf8" => {
                fixture.write("locales/fr.toml", "");
                std::fs::write(fixture.0.join("locales/fr.toml"), [0xff, 0xfe]).unwrap();
            }
            _ => {}
        }
        let (project, _) = Project::load(&fixture.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        let (line, column) =
            LineIndex::new(&config).position(config.find("'locales/fr.toml'").unwrap());
        assert!(
            problems
                .iter()
                .any(|problem| problem.rule == "i18n-messages"
                    && problem.severity == Severity::Error
                    && problem.file.as_deref() == Some("rustify-docs.toml")
                    && problem.line == Some(line)
                    && problem.column == Some(column)),
            "{kind}: {problems:?}"
        );
        assert!(!snapshot.messages.contains_key("locales/fr.toml"));
        assert!(!snapshot.files.contains_key("locales/fr.toml"));
    }
}

#[cfg(unix)]
#[test]
fn projection_message_symlinks_keep_original_targets_when_equal_bytes_are_rebound() {
    let config = config_with_messages("locales/second.toml")
        .replace("prefix='zh'", "prefix='zh'\nmessages='locales/first.toml'");
    let fixture = Fixture::new(&config);
    fixture.page("index.md", "Home", "");
    fixture.write("inputs/a.toml", "menu='Lire'\n");
    fixture.write("inputs/b.toml", "menu='Lire'\n");
    std::fs::create_dir_all(fixture.0.join("locales")).unwrap();
    std::os::unix::fs::symlink("../inputs/a.toml", fixture.0.join("locales/first.toml")).unwrap();
    std::os::unix::fs::symlink("../inputs/b.toml", fixture.0.join("locales/second.toml")).unwrap();
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (before, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    let original = std::fs::canonicalize(fixture.0.join("inputs/a.toml")).unwrap();
    assert!(before
        .inputs
        .byte_paths()
        .any(|(alias, real)| alias == "locales/first.toml" && real == original.as_path()));
    assert!(before.linked.contains("inputs/a.toml"));
    assert!(before.linked.contains("inputs/b.toml"));
    let analysis = content::analyze(
        &project,
        &before,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(content::validate_inputs(&project, &before, &analysis).is_empty());
    std::fs::remove_file(fixture.0.join("locales/first.toml")).unwrap();
    std::fs::remove_file(fixture.0.join("locales/second.toml")).unwrap();
    std::os::unix::fs::symlink("../inputs/b.toml", fixture.0.join("locales/first.toml")).unwrap();
    std::os::unix::fs::symlink("../inputs/a.toml", fixture.0.join("locales/second.toml")).unwrap();
    let (after, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(before.messages, after.messages);
    assert_eq!(before.linked, after.linked);
    let changed = content::validate_inputs(&project, &before, &analysis);
    for path in ["locales/first.toml", "locales/second.toml"] {
        assert!(
            changed
                .iter()
                .any(|problem| problem.rule == "render-generation-changed"
                    && problem.file.as_deref() == Some(path)),
            "{changed:?}"
        );
    }
}

#[cfg(unix)]
#[test]
fn projection_message_symlinks_cannot_read_private_or_external_targets() {
    for private in [true, false] {
        let config = config_with_messages("locales/fr.toml");
        let fixture = Fixture::new(&config);
        let outside = Fixture::new(CONFIG);
        fixture.page("index.md", "Home", "");
        fixture.write(".env/fr.toml", "menu='Private marker'\n");
        outside.write("fr.toml", "menu='Outside marker'\n");
        std::fs::create_dir_all(fixture.0.join("locales")).unwrap();
        let target = if private {
            fixture.0.join(".env/fr.toml")
        } else {
            outside.0.join("fr.toml")
        };
        std::os::unix::fs::symlink(target, fixture.0.join("locales/fr.toml")).unwrap();
        let (project, _) = Project::load(&fixture.0).unwrap();
        let (snapshot, problems) = content::snapshot(&project);
        let (line, column) =
            LineIndex::new(&config).position(config.find("'locales/fr.toml'").unwrap());
        assert!(
            problems
                .iter()
                .any(|problem| problem.rule == "i18n-messages"
                    && problem.severity == Severity::Error
                    && problem.file.as_deref() == Some("rustify-docs.toml")
                    && problem.line == Some(line)
                    && problem.column == Some(column)),
            "private={private}: {problems:?}"
        );
        assert!(snapshot.messages.is_empty());
        assert!(!snapshot.files.contains_key("locales/fr.toml"));
    }
}

#[cfg(unix)]
#[test]
fn projection_message_aliases_cannot_bypass_resource_or_scanning_refusals() {
    let fixture = Fixture::new(&config_with_messages("locales/fr.toml"));
    fixture.write("locales/fr.toml", "menu='Lire'\n");
    fixture.write(
        "content/index.md",
        "---\ndescription: Home\n---\n# Home\n\n[Raw alias](../downloads/raw.txt)\n",
    );
    std::fs::create_dir_all(fixture.0.join("downloads")).unwrap();
    std::fs::create_dir_all(fixture.0.join("static")).unwrap();
    std::os::unix::fs::symlink("../locales/fr.toml", fixture.0.join("downloads/raw.txt")).unwrap();
    std::os::unix::fs::symlink("../locales/fr.toml", fixture.0.join("static/exposed.txt")).unwrap();
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, problems) = content::snapshot(&project);
    assert!(
        problems
            .iter()
            .any(|problem| problem.rule == "i18n-messages"),
        "{problems:?}"
    );
    assert!(!snapshot.files.contains_key("static/exposed.txt"));
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
            .any(|problem| problem.rule == "i18n-messages"
                && problem.file.as_deref() == Some("content/index.md")),
        "{:?}",
        analysis.diagnostics
    );
    assert!(!analysis.resources.contains("downloads/raw.txt"));
    assert!(!analysis.resources.contains("static/exposed.txt"));
    assert!(!analysis.outside.contains_key("downloads/raw.txt"));
}
