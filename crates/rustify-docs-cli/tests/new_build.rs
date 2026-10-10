//! `new` and `build` as an author first meets them: with nothing on `PATH`
//! but stand-ins for the Rust toolchain, which record anything the CLI runs.
//! A content site must be created and built without compiling anything.

mod support;

use support::*;

#[test]
fn a_new_site_builds_and_nothing_is_compiled() {
    let temp = Temp::new("new");
    let tools = Toolchain::new(&temp, true);
    let created = tools.run_in(&temp.0, &["new", "my-docs"]);
    succeeded(&created);
    assert!(
        stderr(&created).contains("rustc 1.99.0"),
        "{}",
        stderr(&created)
    );
    let site = temp.path("my-docs");
    let files: Vec<String> = tree(&site).into_keys().collect();
    assert_eq!(
        files,
        [
            ".gitignore",
            "README.md",
            "content/guide/getting-started.md",
            "content/guide/writing-pages.md",
            "content/index.md",
            "rustify-docs.toml",
            "static/logo.svg",
        ]
    );
    assert!(temp
        .read("my-docs/rustify-docs.toml")
        .contains("name = \"my-docs\""));

    let built = tools.run_in(&site, &["build"]);
    succeeded(&built);
    let text = stderr(&built);
    let conclusion = text
        .lines()
        .find(|line| line.starts_with("build: "))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(
        conclusion.starts_with("build: 3 pages and 404.html in "),
        "{conclusion}"
    );
    assert!(
        conclusion.contains(" files, 2 warning(s), "),
        "{conclusion}"
    );
    // The largest pages close the report.
    let tail: Vec<&str> = text.lines().rev().take(4).collect();
    assert!(
        tail.iter()
            .all(|line| line.ends_with("index.html") || line.ends_with("404.html")),
        "{text}"
    );
    for page in [
        "index.html",
        "guide/getting-started/index.html",
        "guide/writing-pages/index.html",
        "404.html",
    ] {
        assert!(site.join("dist").join(page).is_file(), "{page}");
    }
    // Only the toolchain check ran: no cargo, no compiler, no linker.
    assert_eq!(tools.calls(), ["rustc --version"]);

    let checked = tools.run_in(&site, &["check", "--format", "json"]);
    succeeded(&checked);
    assert_eq!(tools.calls(), ["rustc --version"]);
}

#[test]
fn new_writes_into_an_empty_directory_and_never_into_a_full_one() {
    let temp = Temp::new("new-dirs");
    let tools = Toolchain::new(&temp, true);
    std::fs::create_dir_all(temp.path("empty")).unwrap();
    succeeded(&tools.run_in(&temp.0, &["new", "empty"]));
    assert!(temp.path("empty/rustify-docs.toml").is_file());
    std::fs::create_dir_all(temp.path("here")).unwrap();
    succeeded(&tools.run_in(&temp.path("here"), &["new", "."]));
    assert!(temp
        .read("here/rustify-docs.toml")
        .contains("name = \"here\""));

    temp.write("full/notes.txt", "mine");
    let refused = tools.run_in(&temp.0, &["new", "full"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(
        stderr(&refused).contains("is not empty"),
        "{}",
        stderr(&refused)
    );
    assert_eq!(tree(&temp.path("full")).len(), 1);
    assert_eq!(temp.read("full/notes.txt"), "mine");

    let other = tools.run_in(&temp.0, &["new", "next", "--template", "components"]);
    assert_eq!(other.status.code(), Some(1));
    assert!(stderr(&other).contains("template `components` is not in this release"));
    assert!(!temp.path("next").exists());

    // Nothing half-written is left beside them.
    let names: Vec<String> = std::fs::read_dir(&temp.0)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.contains("rustify-docs-new"))
        .collect();
    assert!(names.is_empty(), "{names:?}");
}

#[test]
fn without_rust_new_stops_before_writing_anything() {
    let temp = Temp::new("no-rustc");
    let tools = Toolchain::new(&temp, false);
    let refused = tools.run_in(&temp.0, &["new", "my-docs"]);
    assert_eq!(refused.status.code(), Some(1));
    let text = stderr(&refused);
    assert!(
        text.contains("https://rustup.rs") && text.contains("rustup toolchain install stable"),
        "{text}"
    );
    assert!(!temp.path("my-docs").exists());
    let leftovers: Vec<_> = std::fs::read_dir(&temp.0)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .filter(|name| name.to_string_lossy().starts_with('.'))
        .collect();
    assert!(leftovers.is_empty(), "{leftovers:?}");
}

#[test]
fn a_failed_first_build_leaves_no_output() {
    let temp = Temp::new("first-failure");
    let tools = Toolchain::new(&temp, true);
    succeeded(&tools.run_in(&temp.0, &["new", "site"]));
    let site = temp.path("site");
    temp.edit(
        "site/content/guide/writing-pages.md",
        "Put images next to",
        "![Missing](nowhere.png)\n\nPut images next to",
    );
    let failed = tools.run_in(&site, &["build"]);
    assert_eq!(failed.status.code(), Some(1));
    let last = stderr(&failed).lines().last().unwrap().to_owned();
    assert!(
        last.starts_with("build failed: 1 error(s), ")
            && last
                .contains("first: `nowhere.png` does not exist (content/guide/writing-pages.md:"),
        "{last}"
    );
    assert!(!site.join("dist").exists());
    // The private area holds its lock file and nothing else.
    let area: Vec<String> = tree(&site.join(".rustify-docs")).into_keys().collect();
    assert_eq!(area.len(), 1, "{area:?}");
    assert!(area[0].ends_with("/lock"), "{area:?}");
}

/// The generated `.gitignore` keeps a build out of version control, and a
/// site in a repository gets its pages' dates from the history.
#[test]
fn a_built_site_leaves_the_repository_clean_and_dated() {
    let temp = Temp::new("git");
    let tools = Toolchain::new(&temp, true);
    succeeded(&tools.run_in(&temp.0, &["new", "site"]));
    let site = temp.path("site");
    let git = |args: &[&str]| {
        let output = std::process::Command::new("git")
            .args(args)
            .current_dir(&site)
            .env("GIT_AUTHOR_DATE", "2026-10-09T12:00:00+00:00")
            .env("GIT_COMMITTER_DATE", "2026-10-09T12:00:00+00:00")
            .output()
            .expect("git runs");
        assert!(output.status.success(), "git {args:?}: {}", stderr(&output));
        stdout(&output)
    };
    git(&["init", "-q"]);
    git(&[
        "-c",
        "user.name=Test",
        "-c",
        "user.email=test@example.com",
        "add",
        "-A",
    ]);
    git(&[
        "-c",
        "user.name=Test",
        "-c",
        "user.email=test@example.com",
        "commit",
        "-q",
        "-m",
        "site",
    ]);

    // The real PATH here: git must be found for the dates.
    let built = run_in(&site, &["build", "--format", "json"]);
    succeeded(&built);
    let report: serde_json::Value = serde_json::from_slice(&built.stdout).unwrap();
    let rules: Vec<&str> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["rule"].as_str().unwrap())
        .collect();
    assert_eq!(
        rules,
        ["missing-site-url"],
        "dated from the history: {report:#}"
    );
    assert_eq!(report["summary"]["pages"], 3);
    let page = temp.read("site/dist/guide/getting-started/index.html");
    assert!(
        page.contains("<time datetime=\"2026-10-09\">2026-10-09</time>"),
        "{page}"
    );

    assert_eq!(git(&["status", "--porcelain", "--ignored=no"]), "");
    let ignored = git(&["status", "--porcelain", "--ignored"]);
    assert!(
        ignored.contains("!! .rustify-docs/") && ignored.contains("!! dist/"),
        "{ignored}"
    );
}
