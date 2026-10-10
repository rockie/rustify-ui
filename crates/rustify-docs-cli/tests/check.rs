//! `rustify-docs check` as a CI job or an author runs it: the real binary,
//! the repository's fixtures and throwaway projects, read through stdout,
//! stderr and the exit status alone.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const BIN: &str = env!("CARGO_BIN_EXE_rustify-docs");

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/docs-ssg/fixtures")
        .join(name)
}

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("the rustify-docs binary runs")
}

fn check_json(config: &Path, extra: &[&str]) -> (Option<i32>, Value, String) {
    let config = config.to_str().unwrap();
    let mut args = vec!["check", "--config", config, "--format", "json"];
    args.extend_from_slice(extra);
    let output = run(&args);
    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    // One complete object and nothing else: a trailing anything fails here.
    let report: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|error| panic!("stdout is not one JSON object ({error}):\n{stdout}"));
    let stderr = String::from_utf8(output.stderr).unwrap();
    (output.status.code(), report, stderr)
}

/// A project in a fresh temporary directory, removed when dropped.
struct Project(PathBuf);

impl Project {
    fn new(files: &[(&str, &str)]) -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("rustify-docs-cli-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (path, text) in files {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        Self(root)
    }

    fn config(&self) -> PathBuf {
        self.0.join("rustify-docs.toml")
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Every file under `root` with its bytes.
fn tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.insert(path.clone(), std::fs::read(&path).unwrap());
            }
        }
    }
    files
}

#[test]
fn three_known_errors_are_exactly_three_located_errors() {
    let (code, report, stderr) = check_json(&fixture("three-errors/rustify-docs.toml"), &[]);
    assert_eq!(code, Some(1));
    assert!(stderr.is_empty(), "JSON mode writes no progress: {stderr}");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["success"], false);
    assert_eq!(report["summary"]["errors"], 3);
    assert_eq!(report["summary"]["warnings"], 0);
    assert_eq!(report["summary"]["pages"], 2);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    let found: Vec<(&str, &str, u64, u64)> = diagnostics
        .iter()
        .map(|d| {
            assert_eq!(d["severity"], "error");
            assert!(
                d["suggestion"].as_str().is_some_and(|s| !s.is_empty()),
                "every error says how to fix it: {d}"
            );
            assert!(d["message"].as_str().is_some_and(|m| !m.is_empty()));
            (
                d["rule"].as_str().unwrap(),
                d["file"].as_str().unwrap(),
                d["line"].as_u64().unwrap(),
                d["column"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        found,
        [
            ("broken-anchor", "content/index.md", 8, 5),
            ("missing-image", "content/index.md", 10, 1),
            ("inline-script", "content/index.md", 12, 1),
        ]
    );
    for d in diagnostics {
        let mut keys: Vec<&str> = d.as_object().unwrap().keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "column",
                "file",
                "line",
                "message",
                "rule",
                "severity",
                "suggestion"
            ]
        );
    }
}

#[test]
fn text_mode_reports_on_stderr_with_locations_and_a_conclusion() {
    let config = fixture("three-errors/rustify-docs.toml");
    let output = run(&["check", "--config", config.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    for location in [
        "--> content/index.md:8:5",
        "--> content/index.md:10:1",
        "--> content/index.md:12:1",
    ] {
        assert!(stderr.contains(location), "{location} in:\n{stderr}");
    }
    assert_eq!(stderr.matches("= help: ").count(), 3, "{stderr}");
    let last = stderr.lines().last().unwrap();
    assert!(
        last.starts_with("check: 2 pages, 3 error(s), 0 warning(s); first: "),
        "{last}"
    );
}

#[test]
fn a_clean_site_passes_and_check_writes_nothing() {
    let root = fixture("content");
    let before = tree(&root);
    let (code, report, _) = check_json(&root.join("rustify-docs.toml"), &["--deny-warnings"]);
    assert_eq!(code, Some(0), "{report:#}");
    assert_eq!(report["success"], true);
    assert_eq!(report["diagnostics"], Value::Array(Vec::new()));
    assert_eq!(report["summary"]["pages"], 4, "the draft is not published");
    assert_eq!(tree(&root), before, "check changed the project");
}

#[test]
fn warnings_fail_only_when_denied() {
    let project = Project::new(&[
        (
            "rustify-docs.toml",
            "schema_version = 1\n[site]\nname = \"W\"\nurl = \"https://example.com/\"\n",
        ),
        (
            "content/index.md",
            "# Home\n\nThe first paragraph stands in for a description.\n",
        ),
    ]);
    let (code, report, _) = check_json(&project.config(), &[]);
    assert_eq!(code, Some(0));
    assert_eq!(report["success"], true);
    let rules: Vec<&str> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| {
            assert_eq!(d["severity"], "warning");
            d["rule"].as_str().unwrap()
        })
        .collect();
    // A temporary directory is outside any repository, so it has no dates.
    assert_eq!(rules, ["no-git-history", "missing-description"]);

    let (code, report, _) = check_json(&project.config(), &["--deny-warnings"]);
    assert_eq!(code, Some(1));
    assert_eq!(report["success"], false);

    let output = run(&[
        "check",
        "--config",
        project.config().to_str().unwrap(),
        "--deny-warnings",
    ]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("(warnings denied)"), "{stderr}");
}

#[test]
fn configuration_errors_are_located_in_the_configuration() {
    let project = Project::new(&[
        (
            "rustify-docs.toml",
            "schema_version = 1\n[site]\nname = \"C\"\ncolour = \"red\"\n",
        ),
        ("content/index.md", "---\ndescription: d\n---\n# Home\n"),
    ]);
    let (code, report, _) = check_json(&project.config(), &[]);
    assert_eq!(code, Some(1));
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 1, "{report:#}");
    let d = &diagnostics[0];
    assert_eq!(d["rule"], "config");
    assert_eq!(d["file"], "rustify-docs.toml");
    assert_eq!(
        (d["line"].as_u64(), d["column"].as_u64()),
        (Some(4), Some(1))
    );
    assert!(d["message"].as_str().unwrap().contains("colour"), "{d}");
    assert_eq!(report["summary"]["pages"], 0);

    let missing = Project::new(&[("content/index.md", "# Home\n")]);
    let (code, report, _) = check_json(&missing.config(), &[]);
    assert_eq!(code, Some(1));
    assert_eq!(report["diagnostics"][0]["rule"], "config-missing");
    assert_eq!(report["diagnostics"][0]["line"], Value::Null);
}

#[test]
fn the_configuration_defaults_to_the_working_directory() {
    let output = Command::new(BIN)
        .args(["check", "--format", "json"])
        .current_dir(fixture("three-errors"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["errors"], 3);
}

#[test]
fn usage_errors_exit_with_two_and_touch_nothing() {
    let config = fixture("three-errors/rustify-docs.toml");
    let config = config.to_str().unwrap();
    for args in [
        vec!["check", "--format", "xml", "--config", config],
        vec!["check", "--config"],
        vec!["check", "--bogus"],
        // Only check requests other sites.
        vec!["build", "--external-links", "--config", config],
        vec!["no-such-command"],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("rustify-docs --help"), "{args:?}: {stderr}");
    }
}

#[test]
fn help_and_version() {
    let output = run(&["--help"]);
    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("check") && help.contains("--deny-warnings"));
    assert!(help.contains("Supported on macOS"));
    let output = run(&["--version"]);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("rustify-docs {}\n", env!("CARGO_PKG_VERSION"))
    );
}
