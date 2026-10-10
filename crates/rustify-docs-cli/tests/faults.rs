//! What a build does when the machine works against it, on real files and
//! real processes: killed at each step of publishing, a full disk, a
//! refused permission, a damaged or foreign output, two writers at once,
//! outputs it must not touch, files that must never ship, author HTML the
//! policy refuses, and the machine's own paths and values.

mod support;

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use support::*;

/// The fixture site under `site/`, built once into `site/dist`.
fn built() -> (Temp, BTreeMap<String, Vec<u8>>) {
    let temp = copy_fixture("content");
    succeeded(&run_in(&temp.path("site"), &["build"]));
    let tree = tree(&temp.path("site/dist"));
    assert!(tree.contains_key("index.html"));
    (temp, tree)
}

/// The private area of `out`: `<parent>/.rustify-docs/<name>-<hash>/`.
fn area(out: &Path) -> PathBuf {
    let parent = out.parent().unwrap().join(".rustify-docs");
    let name = out.file_name().unwrap().to_string_lossy().into_owned();
    std::fs::read_dir(&parent)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(&format!("{name}-"))
        })
        .expect("the output's private area")
}

/// What the area holds besides its lock.
fn leftovers(out: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(area(out))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name != "lock")
        .collect();
    names.sort();
    names
}

/// Starts `build` stopped at `step`, and returns it once it says so.
fn stopped_build(site: &Path, step: &str) -> std::process::Child {
    let mut child = Command::new(BIN)
        .arg("build")
        .current_dir(site)
        .env("RUSTIFY_DOCS_FAULT", format!("stop:{step}"))
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = BufReader::new(child.stderr.take().unwrap()).lines();
    let marker = format!("build: stopped at {step}");
    loop {
        let line = lines
            .next()
            .unwrap_or_else(|| panic!("build ended before {step}"))
            .unwrap();
        if line.starts_with(&marker) {
            break;
        }
    }
    // Keep draining, so the child never blocks on a full pipe.
    std::thread::spawn(move || for _ in lines {});
    child
}

#[test]
fn a_build_killed_at_each_step_of_publishing_is_recovered_by_the_next() {
    for (step, recovered) in [
        ("staged", "discarded"),
        ("moved-aside", "undone and the previous output restored"),
        ("swapped-in", "completed"),
    ] {
        let (temp, before) = built();
        let site = temp.path("site");
        let out = site.join("dist");
        temp.edit(
            "site/content/guide/index.md",
            "Read the setup page first.",
            "Read the setup page first, again.",
        );
        let mut build = stopped_build(&site, step);
        build.kill().unwrap();
        build.wait().unwrap();

        // What the kill left: exactly what the step says.
        match step {
            "staged" => assert_eq!(tree(&out), before, "{step}"),
            "moved-aside" => assert!(!out.exists(), "{step}"),
            _ => {
                let after = tree(&out);
                assert_ne!(after, before, "{step}");
                assert!(text_of(&after, "guide/index.html").contains("again"));
            }
        }
        let refused = Preview::start(&site, &[]).err().expect("preview refuses");
        assert!(refused.contains("stopped half-way"), "{step}: {refused}");

        let next = run_in(&site, &["build"]);
        succeeded(&next);
        let said = stderr(&next);
        assert!(said.contains(recovered), "{step}: {said}");
        assert!(text_of(&tree(&out), "guide/index.html").contains("again"));
        assert_eq!(leftovers(&out), Vec::<String>::new(), "{step}");
    }
}

#[test]
fn one_writer_at_a_time_and_a_killed_build_lets_go() {
    let (temp, before) = built();
    let site = temp.path("site");
    let mut first = stopped_build(&site, "staged");
    let second = run_in(&site, &["build"]);
    assert_eq!(second.status.code(), Some(1));
    assert!(
        stderr(&second).contains("in use by another build"),
        "{}",
        stderr(&second)
    );
    let preview = Preview::start(&site, &[]).err().expect("preview refuses");
    assert!(preview.contains("being built"), "{preview}");
    assert_eq!(tree(&site.join("dist")), before);
    first.kill().unwrap();
    first.wait().unwrap();
    // The system released the dead build's lock with its process.
    let third = run_in(&site, &["build"]);
    succeeded(&third);
    assert!(stderr(&third).contains("discarded"));
}

/// A small volume of its own, detached when dropped.
struct Volume {
    mount: PathBuf,
}

impl Volume {
    fn new(temp: &Temp, megabytes: u32) -> Self {
        let image = temp.path("volume.dmg");
        let mount = temp.path("volume");
        std::fs::create_dir_all(&mount).unwrap();
        let ok = |command: &mut Command| {
            let output = command.output().unwrap();
            assert!(output.status.success(), "{}", stderr(&output));
        };
        ok(Command::new("hdiutil")
            .args([
                "create",
                "-quiet",
                "-fs",
                "HFS+",
                "-volname",
                "rustify-full",
                "-size",
            ])
            .arg(format!("{megabytes}m"))
            .arg(&image));
        ok(Command::new("hdiutil")
            .args([
                "attach",
                "-quiet",
                "-nobrowse",
                "-noautoopen",
                "-mountpoint",
            ])
            .arg(&mount)
            .arg(&image));
        Self { mount }
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        let _ = Command::new("hdiutil")
            .args(["detach", "-quiet", "-force"])
            .arg(&self.mount)
            .status();
    }
}

#[test]
fn a_full_disk_keeps_the_previous_output() {
    let temp = copy_fixture("content");
    let site = temp.path("site");
    let volume = Volume::new(&temp, 4);
    let out = volume.mount.join("dist");
    let out_arg = out.to_str().unwrap();
    succeeded(&run_in(&site, &["build", "--out", out_arg]));
    let before = tree(&out);
    // More than the volume holds, published at its own path.
    std::fs::write(site.join("static/large.bin"), vec![7u8; 6 << 20]).unwrap();
    let full = run_in(&site, &["build", "--out", out_arg, "--format", "json"]);
    assert_eq!(full.status.code(), Some(1), "{}", stderr(&full));
    let report: serde_json::Value = serde_json::from_slice(&full.stdout).unwrap();
    let write = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["rule"] == "write")
        .unwrap_or_else(|| panic!("{report}"));
    assert!(
        write["message"].as_str().unwrap().contains("No space left"),
        "{write}"
    );
    assert_eq!(tree(&out), before, "the full disk cost the old output");
    assert_eq!(leftovers(&out), Vec::<String>::new());
    // Room again: the next build publishes.
    std::fs::remove_file(site.join("static/large.bin")).unwrap();
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Read it first.",
    );
    succeeded(&run_in(&site, &["build", "--out", out_arg]));
    assert!(text_of(&tree(&out), "guide/index.html").contains("Read it first."));
}

/// Restores a directory's mode when dropped, so a failed test cleans up.
struct Mode(PathBuf, u32);

impl Mode {
    fn set(path: &Path, mode: u32) -> Self {
        let old = std::fs::metadata(path).unwrap().permissions().mode();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
        Self(path.to_path_buf(), old)
    }
}

impl Drop for Mode {
    fn drop(&mut self) {
        let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(self.1));
    }
}

#[test]
fn a_refused_permission_keeps_the_previous_output() {
    let temp = copy_fixture("content");
    let site = temp.path("site");
    let parent = temp.path("published");
    std::fs::create_dir_all(&parent).unwrap();
    let out = parent.join("dist");
    let out_arg = out.to_str().unwrap();
    succeeded(&run_in(&site, &["build", "--out", out_arg]));
    let before = tree(&out);
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Changed.",
    );

    // The output's directory refuses the swap's renames.
    {
        let _locked = Mode::set(&parent, 0o555);
        let refused = run_in(&site, &["build", "--out", out_arg]);
        assert_eq!(refused.status.code(), Some(1));
        assert!(
            stderr(&refused).contains("moving aside"),
            "{}",
            stderr(&refused)
        );
    }
    assert_eq!(tree(&out), before);
    assert_eq!(leftovers(&out), Vec::<String>::new());

    // The private area refuses its journal.
    {
        let _locked = Mode::set(&area(&out), 0o555);
        let refused = run_in(&site, &["build", "--out", out_arg]);
        assert_eq!(refused.status.code(), Some(1));
        assert!(
            stderr(&refused).contains("Permission denied"),
            "{}",
            stderr(&refused)
        );
    }
    assert_eq!(tree(&out), before);
    succeeded(&run_in(&site, &["build", "--out", out_arg]));
    assert!(text_of(&tree(&out), "guide/index.html").contains("Changed."));
}

#[test]
fn an_output_whose_owner_cannot_be_confirmed_is_never_replaced() {
    let (temp, _) = built();
    let site = temp.path("site");
    let manifest = site.join("dist/rustify-docs-manifest.json");
    for (text, says) in [
        ("{", "is damaged"),
        (
            "{\"schema_version\":1,\"mode\":\"dev\",\"base\":\"/docs/\",\"cli_version\":\"0\",\"build_id\":\"x\",\"files\":[]}",
            "not a production build",
        ),
    ] {
        std::fs::write(&manifest, text).unwrap();
        let before = tree(&site.join("dist"));
        let refused = run_in(&site, &["build"]);
        assert_eq!(refused.status.code(), Some(1));
        assert!(stderr(&refused).contains(says), "{}", stderr(&refused));
        assert_eq!(tree(&site.join("dist")), before);
    }
    // A journal of garbage is someone's interrupted build: discarded, and
    // nothing outside the private area is touched.
    std::fs::remove_dir_all(site.join("dist")).unwrap();
    std::fs::write(area(&site.join("dist")).join("transaction"), "?!\n").unwrap();
    let next = run_in(&site, &["build"]);
    succeeded(&next);
    assert!(stderr(&next).contains("discarded"));
}

#[test]
fn outputs_that_are_not_a_build_output_are_refused() {
    let (temp, _) = built();
    let site = temp.path("site");
    std::fs::create_dir_all(site.join(".git/objects")).unwrap();
    std::fs::write(temp.path("a-file"), "x").unwrap();
    std::os::unix::fs::symlink(site.join("dist"), temp.path("linked")).unwrap();
    let cases = [
        ("/", "would hold the project"),
        ("..", "would hold the project"),
        ("../..", "would hold the project"),
        (".git", "repository's or this tool's own directory"),
        (".git/site", "repository's or this tool's own directory"),
        (".rustify-docs", "repository's or this tool's own directory"),
        ("../a-file", "Not a directory"),
        ("../linked", "symbolic link"),
    ];
    let before = tree(&temp.0);
    for (out, says) in cases {
        let refused = run_in(&site, &["build", "--out", out]);
        assert_eq!(refused.status.code(), Some(1), "{out}");
        assert!(
            stderr(&refused).contains(says),
            "{out}: {}",
            stderr(&refused)
        );
    }
    let after = tree(&temp.0);
    let changed: Vec<&String> = after
        .keys()
        .filter(|path| before.get(*path) != after.get(*path))
        .chain(before.keys().filter(|path| !after.contains_key(*path)))
        .filter(|path| !path.contains("/.rustify-docs/") && !path.starts_with(".rustify-docs/"))
        .collect();
    assert!(changed.is_empty(), "{changed:?}");
}

#[test]
fn private_files_and_links_out_of_the_project_never_ship() {
    let temp = copy_fixture("content");
    let site = temp.path("site");
    temp.write("site/static/.git/config", "[core]\n");
    temp.write("site/static/.DS_Store", "x");
    temp.write("site/static/.env.production", "TOKEN=x\n");
    temp.write("site/static/notes/.env", "TOKEN=x\n");
    // A link inside the project is followed; one out of it is an error.
    std::os::unix::fs::symlink(
        "../content/guide/pipeline.svg",
        site.join("static/pipeline-link.svg"),
    )
    .unwrap();
    temp.write("outside.txt", "secret\n");
    std::os::unix::fs::symlink(temp.path("outside.txt"), site.join("static/outside.txt")).unwrap();
    let refused = run_in(&site, &["build", "--format", "json"]);
    assert_eq!(refused.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&refused.stdout).unwrap();
    let errors: Vec<(&str, &str)> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "error")
        .map(|d| {
            (
                d["rule"].as_str().unwrap(),
                d["file"].as_str().unwrap_or(""),
            )
        })
        .collect();
    assert_eq!(errors, [("symlink-outside", "static/outside.txt")]);
    assert!(!site.join("dist").exists());

    std::fs::remove_file(site.join("static/outside.txt")).unwrap();
    succeeded(&run_in(&site, &["build"]));
    let built = tree(&site.join("dist"));
    assert!(built.contains_key("pipeline-link.svg"));
    for path in built.keys() {
        assert!(
            !path.contains(".git") && !path.contains(".env") && !path.contains(".DS_Store"),
            "{path} shipped"
        );
    }
}

#[test]
fn author_html_the_policy_refuses_fails_check_and_build_where_written() {
    let (temp, before) = built();
    let site = temp.path("site");
    let text = temp.read("site/content/reference/api.md");
    temp.write(
        "site/content/reference/api.md",
        &format!(
            "{text}\n<p onclick=\"go()\">click</p>\n\n<style>p {{ color: red }}</style>\n\n<img src=\"https://cdn.example/a.png\" alt=\"a\">\n\n<a href=\"https://example.org\" target=\"_blank\">away</a>\n"
        ),
    );
    for command in ["check", "build"] {
        let output = run_in(&site, &[command, "--format", "json"]);
        assert_eq!(output.status.code(), Some(1), "{command}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let mut found: Vec<(String, u64)> = report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|d| d["severity"] == "error")
            .map(|d| {
                assert_eq!(d["file"], "content/reference/api.md", "{d}");
                assert!(d["column"].as_u64().is_some(), "{d}");
                (
                    d["rule"].as_str().unwrap().to_owned(),
                    d["line"].as_u64().unwrap(),
                )
            })
            .collect();
        found.sort();
        let rules: Vec<&str> = found.iter().map(|(rule, _)| rule.as_str()).collect();
        assert_eq!(
            rules,
            [
                "cross-origin-resource",
                "event-handler",
                "inline-style",
                "new-window-opener"
            ],
            "{command}"
        );
    }
    assert_eq!(tree(&site.join("dist")), before);
}

#[test]
fn no_machine_path_user_or_environment_value_reaches_the_output() {
    let temp = copy_fixture("content");
    let site = temp.path("site");
    let canary = format!("canary-{}", std::process::id());
    let home = temp.path(&format!("home-{canary}"));
    std::fs::create_dir_all(&home).unwrap();
    let output = Command::new(BIN)
        .arg("build")
        .current_dir(&site)
        .env("HOME", &home)
        .env("USER", format!("user-{canary}"))
        .env("LOGNAME", format!("user-{canary}"))
        .env("RUSTIFY_DOCS_CANARY", &canary)
        .output()
        .unwrap();
    succeeded(&output);
    let root = temp.0.to_string_lossy().into_owned();
    let real_user = std::env::var("USER").unwrap_or_default();
    for (path, bytes) in tree(&site.join("dist")) {
        let text = String::from_utf8_lossy(&bytes);
        assert!(
            !text.contains(&canary),
            "{path} carries an environment value"
        );
        assert!(!text.contains(&root), "{path} carries the project's path");
        assert!(
            !text.contains("/Users/") && !text.contains("/private/"),
            "{path} carries a machine path"
        );
        if real_user.len() > 3 {
            assert!(
                !text.contains(&format!("/{real_user}/")),
                "{path} carries the user's name"
            );
        }
    }
    // Diagnostics name files from the project's root, not the machine's.
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "[x](nowhere.md)",
    );
    let check = run_in(&site, &["check", "--format", "json"]);
    let json = String::from_utf8_lossy(&check.stdout);
    assert!(
        json.contains("\"file\":\"content/guide/index.md\""),
        "{json}"
    );
    assert!(!json.contains(&root), "{json}");
}
