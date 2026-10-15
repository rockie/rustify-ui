#![cfg(feature = "pipeline")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use rustify_docs::content::{
    self, Analysis, Diagnostic, FragmentCache, Mode, Project, RegisteredPage, Snapshot, Surfaces,
};
use rustify_docs::diagnostic::Severity;
use rustify_docs::generation::{self, Extensions};
use rustify_docs::highlight::Highlighter;

const CONFIG: &str =
    "schema_version=1\n[site]\nname='Docs'\nbase='/docs/'\nurl='https://example.com/docs/'\n";
const INCLUDE_HOME: &str = "[content]\ndir='content'\ninclude=['index.md']\n";
const REPO: &str = "[repo]\nurl='https://github.com/example/library'\nref='main'\n";
const HOME: &str = "---\ndescription: Home page\n---\n# Home\n";
const EXCLUDED_LINK: &str =
    "---\ndescription: Home page\n---\n# Home\n\n[Excluded](excluded.md#not-a-published-heading)\n";
const EXCLUDED_URL: &str =
    "https://github.com/example/library/blob/main/content/excluded.md#not-a-published-heading";

struct Fixture(PathBuf);

impl Fixture {
    fn new(config: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rustify-docs-content-include-{}-{time}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("content")).unwrap();
        let fixture = Self(root);
        fixture.write("rustify-docs.toml", config.as_bytes());
        fixture
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        let path = self.0.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }

    fn read(&self) -> (Project, Snapshot) {
        let (project, warnings) =
            Project::load(&self.0).unwrap_or_else(|errors| panic!("{errors:?}"));
        assert!(warnings.is_empty(), "{warnings:?}");
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        (project, snapshot)
    }

    fn selection_diagnostics(&self) -> Vec<Diagnostic> {
        match Project::load(&self.0) {
            Err(errors) => errors,
            Ok((project, mut warnings)) => {
                warnings.extend(content::snapshot(&project).1);
                warnings
            }
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn analyze(project: &Project, snapshot: &Snapshot) -> Analysis {
    content::analyze(
        project,
        snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    )
}

#[test]
fn absent_include_keeps_recursive_markdown_discovery() {
    let fixture = Fixture::new(CONFIG);
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write(
        "content/guide/deep/setup.md",
        b"---\ndescription: Setup page\n---\n# Setup\n",
    );

    let (_, snapshot) = fixture.read();

    assert_eq!(
        snapshot
            .markdown
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["guide/deep/setup.md", "index.md"]
    );
}

#[test]
fn explicit_include_avoids_unselected_markdown_and_unreferenced_binary_reads() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}"));
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write("content/bad-utf8.md", &[0xff, 0xfe]);
    fixture.write(
        "content/bad-frontmatter.md",
        b"---\nunknown_frontmatter_field: true\n---\n# Excluded\n",
    );
    fixture.write("content/unused.bin", &[0, 0xff, 1, 0xfe]);

    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);

    assert_eq!(
        snapshot
            .markdown
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["index.md"]
    );
    assert!(snapshot.files.is_empty(), "{:?}", snapshot.files.keys());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    fixture.write("content/bad-utf8.md", &[0xfd]);
    fixture.write("content/bad-frontmatter.md", b"---\nunclosed: [\n");
    fixture.write("content/unused.bin", &[2, 3, 0xff]);
    assert!(
        content::validate_inputs(&project, &snapshot, &analysis).is_empty(),
        "unselected bytes must not become rendering dependencies"
    );
}

#[test]
fn an_explicit_empty_include_can_publish_only_a_registered_page() {
    let fixture = Fixture::new(&format!("{CONFIG}[content]\ndir='content'\ninclude=[]\n"));
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write("content/ignored.md", &[0xff]);
    let (project, snapshot) = fixture.read();
    let surfaces = Surfaces {
        pages: vec![RegisteredPage {
            name: "Landing".into(),
            route: "/".into(),
            lang: None,
            title: "Landing".into(),
            description: Some("Registered landing page".into()),
            html: "<p>Registered landing page.</p>".into(),
            file: "src/pages.rs".into(),
        }],
        ..Default::default()
    };
    let analysis = content::analyze_with_surfaces(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
        &surfaces,
    );
    assert!(snapshot.markdown.is_empty());
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    let generation = generation::generate_extended(
        &project,
        &snapshot,
        &analysis,
        &Default::default(),
        1,
        &Extensions::default(),
    )
    .unwrap();
    let html = String::from_utf8(generation.files["index.html"].to_vec()).unwrap();
    assert!(html.contains("Registered landing page."));
    assert!(!generation.files.contains_key("ignored/index.html"));
}

#[cfg(unix)]
#[test]
fn include_rejects_globs_even_when_a_literal_matching_name_exists() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ndir='content'\ninclude=[\n  'index.md',\n  'guide/*.md',\n]\n"
    ));
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write("content/guide/*.md", HOME.as_bytes());

    let diagnostics = fixture.selection_diagnostics();

    assert!(
        diagnostics.iter().any(|d| d.severity == Severity::Error
            && d.file.as_deref() == Some("rustify-docs.toml")
            && d.line == Some(10)
            && d.column.is_some_and(|column| column >= 3)),
        "the glob entry needs its original location: {diagnostics:?}"
    );
}

#[test]
fn include_rejects_a_file_outside_the_content_directory_at_its_config_entry() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ndir='content'\ninclude=[\n  'index.md',\n  '../outside.md',\n]\n"
    ));
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write("outside.md", HOME.as_bytes());

    let diagnostics = fixture.selection_diagnostics();

    assert!(
        diagnostics.iter().any(|d| d.severity == Severity::Error
            && d.file.as_deref() == Some("rustify-docs.toml")
            && d.line == Some(10)
            && d.column.is_some_and(|column| column >= 3)),
        "the escaping entry needs its original location: {diagnostics:?}"
    );
}

#[test]
fn include_rejects_a_private_path_at_its_config_entry() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ndir='content'\ninclude=[\n  'index.md',\n  '.env/private.md',\n]\n"
    ));
    fixture.write("content/index.md", HOME.as_bytes());
    fixture.write("content/.env/private.md", HOME.as_bytes());

    let diagnostics = fixture.selection_diagnostics();

    assert!(
        diagnostics.iter().any(|d| d.severity == Severity::Error
            && d.file.as_deref() == Some("rustify-docs.toml")
            && d.line == Some(10)
            && d.column.is_some_and(|column| column >= 3)),
        "the private entry needs its original location: {diagnostics:?}"
    );
}

#[test]
fn include_rejects_the_second_duplicate_at_its_original_location() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ndir='content'\ninclude=[\n  'index.md',\n  'index.md',\n]\n"
    ));
    fixture.write("content/index.md", HOME.as_bytes());

    let diagnostics = fixture.selection_diagnostics();

    assert!(
        diagnostics.iter().any(|d| d.severity == Severity::Error
            && d.file.as_deref() == Some("rustify-docs.toml")
            && d.line == Some(10)
            && d.column.is_some_and(|column| column >= 3)),
        "the duplicate entry needs its original location: {diagnostics:?}"
    );
}

#[test]
fn include_rejects_a_missing_file_at_its_config_entry() {
    let fixture = Fixture::new(&format!(
        "{CONFIG}[content]\ndir='content'\ninclude=[\n  'index.md',\n  'missing.md',\n]\n"
    ));
    fixture.write("content/index.md", HOME.as_bytes());

    let diagnostics = fixture.selection_diagnostics();

    assert!(
        diagnostics.iter().any(|d| d.severity == Severity::Error
            && d.file.as_deref() == Some("rustify-docs.toml")
            && d.line == Some(10)
            && d.column.is_some_and(|column| column >= 3)),
        "the missing entry needs its original location: {diagnostics:?}"
    );
}

#[test]
fn excluded_markdown_links_to_the_repository_without_binding_its_body() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}{REPO}"));
    fixture.write("content/index.md", EXCLUDED_LINK.as_bytes());
    fixture.write("content/excluded.md", &[0xff, 0xfe]);
    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(!snapshot.files.contains_key("content/excluded.md"));
    assert!(!analysis.outside.contains_key("content/excluded.md"));
    assert!(!analysis.resources.contains("content/excluded.md"));
    let generation =
        generation::generate(&project, &snapshot, &analysis, &Default::default(), 1).unwrap();
    let html = String::from_utf8(generation.files["index.html"].to_vec()).unwrap();
    let dom = dom_query::Document::from(html.as_str());
    assert_eq!(
        dom.select("article a").attr("href").as_deref(),
        Some(EXCLUDED_URL)
    );
    assert!(!generation.files.contains_key("excluded/index.html"));
    assert!(!generation.files.keys().any(|path| path.ends_with(".md")));

    fixture.write(
        "content/excluded.md",
        b"---\ninvalid: [\nnew excluded body\n",
    );
    let problems = content::validate_inputs(&project, &snapshot, &analysis);
    assert!(problems.is_empty(), "{problems:?}");
}

#[test]
fn deleting_an_excluded_repository_link_target_invalidates_the_original_analysis() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}{REPO}"));
    fixture.write("content/index.md", EXCLUDED_LINK.as_bytes());
    fixture.write("content/excluded.md", b"excluded body");
    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);

    std::fs::remove_file(fixture.0.join("content/excluded.md")).unwrap();
    let problems = content::validate_inputs(&project, &snapshot, &analysis);

    assert!(
        problems
            .iter()
            .any(|d| d.severity == Severity::Error && d.rule == "render-generation-changed"),
        "a repository link still needs its original target: {problems:?}"
    );
}

#[cfg(unix)]
#[test]
fn retargeting_an_excluded_symlink_invalidates_even_identical_target_bytes() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}{REPO}"));
    fixture.write("content/index.md", EXCLUDED_LINK.as_bytes());
    fixture.write("content/first.md", b"identical excluded body");
    fixture.write("content/second.md", b"identical excluded body");
    let link = fixture.0.join("content/excluded.md");
    std::os::unix::fs::symlink("first.md", &link).unwrap();
    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);

    std::fs::remove_file(&link).unwrap();
    std::os::unix::fs::symlink("second.md", &link).unwrap();
    let problems = content::validate_inputs(&project, &snapshot, &analysis);

    assert!(
        problems
            .iter()
            .any(|d| d.severity == Severity::Error && d.rule == "render-generation-changed"),
        "existence must remain bound to the original target: {problems:?}"
    );
}

#[test]
fn an_excluded_markdown_link_without_repo_is_located_in_the_selected_page() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}"));
    fixture.write("content/index.md", EXCLUDED_LINK.as_bytes());
    fixture.write("content/excluded.md", &[0xff]);
    let (project, snapshot) = fixture.read();

    let analysis = analyze(&project, &snapshot);

    assert!(
        analysis
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error
                && d.rule == "repo-link"
                && d.file.as_deref() == Some("content/index.md")
                && d.line == Some(6)
                && d.column.is_some()),
        "the selected-page link needs [repo]: {:?}",
        analysis.diagnostics
    );
}

#[test]
fn raw_html_links_to_excluded_markdown_without_reading_or_publishing_it() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}{REPO}"));
    fixture.write(
        "content/index.md",
        format!(
            "{HOME}\n<a data-excluded=\"\" href=\"excluded.md#not-a-published-heading\">Excluded</a>\n"
        )
        .as_bytes(),
    );
    fixture.write("content/excluded.md", &[0xff]);
    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(!analysis.outside.contains_key("content/excluded.md"));
    assert!(!analysis.resources.contains("content/excluded.md"));
    let generation =
        generation::generate(&project, &snapshot, &analysis, &Default::default(), 1).unwrap();
    let html = String::from_utf8(generation.files["index.html"].to_vec()).unwrap();
    let dom = dom_query::Document::from(html.as_str());
    assert_eq!(
        dom.select("a[data-excluded]").attr("href").as_deref(),
        Some(EXCLUDED_URL)
    );
    assert!(!generation.files.contains_key("excluded/index.html"));
    assert!(!generation.files.keys().any(|path| path.ends_with(".md")));

    fixture.write("content/excluded.md", b"new excluded body");
    let problems = content::validate_inputs(&project, &snapshot, &analysis);
    assert!(problems.is_empty(), "{problems:?}");
}

#[test]
fn a_selected_pages_image_is_read_published_and_bound_to_its_original_bytes() {
    let fixture = Fixture::new(&format!("{CONFIG}{INCLUDE_HOME}"));
    fixture.write(
        "content/index.md",
        format!("{HOME}\n![Selected image](images/picture.svg)\n").as_bytes(),
    );
    let image = b"<svg xmlns='http://www.w3.org/2000/svg' width='2' height='3'><rect width='2' height='3' fill='blue'/></svg>";
    fixture.write("content/images/picture.svg", image);
    fixture.write("content/images/unreferenced.bin", &[0xff, 0, 0xfe]);
    let (project, snapshot) = fixture.read();
    let analysis = analyze(&project, &snapshot);
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(analysis.resources.contains("content/images/picture.svg"));
    let bytes = snapshot
        .files
        .get("content/images/picture.svg")
        .or_else(|| analysis.outside.get("content/images/picture.svg"))
        .expect("the selected page's image was actually read");
    assert_eq!(bytes.as_ref(), image);
    assert!(!snapshot
        .files
        .contains_key("content/images/unreferenced.bin"));
    let generation =
        generation::generate(&project, &snapshot, &analysis, &Default::default(), 1).unwrap();
    let html = String::from_utf8(generation.files["index.html"].to_vec()).unwrap();
    let dom = dom_query::Document::from(html.as_str());
    let src = dom.select("article img").attr("src").unwrap();
    let output = src.strip_prefix("/docs/").unwrap();
    assert_eq!(generation.files[output].as_ref(), image);

    fixture.write(
        "content/images/picture.svg",
        b"<svg xmlns='http://www.w3.org/2000/svg' width='2' height='3'><rect width='2' height='3' fill='red'/></svg>",
    );
    let problems = content::validate_inputs(&project, &snapshot, &analysis);
    assert!(
        problems
            .iter()
            .any(|d| d.severity == Severity::Error && d.rule == "render-generation-changed"),
        "published image bytes remain rendering inputs: {problems:?}"
    );
}
