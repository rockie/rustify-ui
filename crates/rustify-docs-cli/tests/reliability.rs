//! A build's promises when things go wrong or happen twice: the same input
//! writes the same bytes, a failed build leaves the previous output exactly
//! as it was and says why, one output has one writer, a preview serves only
//! a production build of the base it was asked for, and a directory that is
//! not an output is never replaced.

mod support;

use support::*;

fn content() -> Temp {
    copy_fixture("content")
}

#[test]
fn the_same_input_builds_the_same_bytes() {
    let temp = content();
    let site = temp.path("site");
    succeeded(&run_in(&site, &["build", "--out", "../one"]));
    succeeded(&run_in(&site, &["build", "--out", "../two"]));
    let (one, two) = (tree(&temp.path("one")), tree(&temp.path("two")));
    assert!(one.len() > 10);
    assert_eq!(one, two);
}

#[test]
fn a_failed_build_keeps_the_previous_output_and_reports_why() {
    let temp = content();
    let site = temp.path("site");
    succeeded(&run_in(&site, &["build"]));
    let before = tree(&site.join("dist"));

    temp.edit(
        "site/content/guide/index.md",
        "](setup.md#install-the-cli)",
        "](setup.md#no-such-heading)",
    );
    temp.write(
        "site/content/broken.md",
        "---\ndescription: d\n---\n# Broken\n\n<p style=\"color: red\">x</p>\n",
    );
    let failed = run_in(&site, &["build"]);
    assert_eq!(failed.status.code(), Some(1));
    let text = stderr(&failed);
    let last = text.lines().last().unwrap();
    assert!(
        last.starts_with("build failed: 2 error(s), ")
            && last.contains("; first: ")
            && last.ends_with("the output was not changed"),
        "{last}"
    );
    assert!(text.contains("--> content/broken.md:6:4"), "{text}");
    assert_eq!(
        tree(&site.join("dist")),
        before,
        "the old output, byte for byte"
    );

    let json = run_in(&site, &["build", "--format", "json"]);
    assert_eq!(json.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(report["success"], false);
    assert_eq!(report["summary"]["errors"], 2);
    assert!(json.stderr.is_empty(), "{}", stderr(&json));
    assert_eq!(tree(&site.join("dist")), before);
}

/// `build --format json`'s report: exactly one JSON object on stdout.
fn json_report(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout is not one JSON object ({error}):\n{}\nstderr:\n{}",
            stdout(output),
            stderr(output)
        )
    })
}

#[test]
fn a_refused_output_is_reported_in_json_as_well() {
    let temp = content();
    let site = temp.path("site");
    let refused = run_in(&site, &["build", "--format", "json", "--out", "content"]);
    assert_eq!(refused.status.code(), Some(1));
    let report = json_report(&refused);
    assert_eq!(report["success"], false);
    assert_eq!(report["summary"]["errors"], 1);
    assert_eq!(report["diagnostics"][0]["rule"], "output");
    assert!(report["diagnostics"][0]["message"]
        .as_str()
        .unwrap()
        .contains("overlaps the content directory"));

    // One being previewed is busy.
    succeeded(&run_in(&site, &["build"]));
    let preview = Preview::start(&site, &[]).unwrap();
    let busy = run_in(&site, &["build", "--format", "json"]);
    assert_eq!(busy.status.code(), Some(1));
    let report = json_report(&busy);
    assert_eq!(report["diagnostics"][0]["rule"], "output-busy");
    drop(preview);

    // Text mode says the same on stderr, as before.
    let text = run_in(&site, &["build", "--out", "content"]);
    assert_eq!(text.status.code(), Some(1));
    assert!(stdout(&text).is_empty());
    assert!(stderr(&text).contains("overlaps the content directory"));
}

#[test]
fn a_site_left_without_pages_fails_and_keeps_what_was_published() {
    let temp = content();
    let site = temp.path("site");
    succeeded(&run_in(&site, &["build"]));
    let before = tree(&site.join("dist"));
    std::fs::remove_dir_all(site.join("content")).unwrap();
    std::fs::create_dir_all(site.join("content")).unwrap();
    temp.write(
        "site/content/later.md",
        "---\ndescription: d\ndraft: true\n---\n# Later\n",
    );
    let failed = run_in(&site, &["build", "--format", "json"]);
    assert_eq!(failed.status.code(), Some(1));
    let report = json_report(&failed);
    let rules: Vec<&str> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "error")
        .map(|d| d["rule"].as_str().unwrap())
        .collect();
    assert!(rules.contains(&"no-pages"), "{report:#}");
    assert_eq!(
        tree(&site.join("dist")),
        before,
        "the old output, byte for byte"
    );
}

#[test]
fn a_successful_build_reports_its_size_and_largest_pages() {
    let temp = content();
    let output = run_in(&temp.path("site"), &["build", "--format", "json"]);
    succeeded(&output);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let summary = &report["summary"];
    assert_eq!(summary["pages"], 4);
    assert_eq!(summary["errors"], 0);
    let files = tree(&temp.path("site/dist"));
    assert_eq!(summary["files"], files.len());
    assert_eq!(
        summary["bytes"],
        files.values().map(Vec::len).sum::<usize>()
    );
    let largest = summary["largest"].as_array().unwrap();
    assert_eq!(largest.len(), 5, "four pages and the not-found page");
    let sizes: Vec<u64> = largest
        .iter()
        .map(|p| p["bytes"].as_u64().unwrap())
        .collect();
    assert!(sizes.windows(2).all(|w| w[0] >= w[1]), "{sizes:?}");
}

#[test]
fn one_output_has_one_writer_and_previews_need_a_finished_production_build() {
    let temp = content();
    let site = temp.path("site");
    let refused = Preview::start(&site, &[])
        .err()
        .expect("nothing to preview yet");
    assert!(refused.contains("not a production build"), "{refused}");

    succeeded(&run_in(&site, &["build"]));
    let preview = Preview::start(&site, &[]).unwrap();
    assert!(preview.url.ends_with("/docs/"), "{}", preview.url);
    let (status, headers, _) = get(&preview.url);
    assert_eq!(status, 200);
    assert!(headers["content-security-policy"].ends_with("frame-ancestors 'none'"));

    let blocked = run_in(&site, &["build"]);
    assert_eq!(blocked.status.code(), Some(1));
    assert!(
        stderr(&blocked).contains("is in use by another build or a preview"),
        "{}",
        stderr(&blocked)
    );
    let second = Preview::start(&site, &[]).expect("previews share an output");
    drop(second);
    drop(preview);
    succeeded(&run_in(&site, &["build"]));

    let wrong = Preview::start(&site, &["--base", "/other/"]).err().unwrap();
    assert!(
        wrong.contains("built for base /docs/, not /other/"),
        "{wrong}"
    );

    // A build that stopped half-way blocks a preview until a build
    // recovers it.
    let area = std::fs::read_dir(site.join(".rustify-docs"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    std::fs::write(area.join("transaction"), "staging\n").unwrap();
    std::fs::create_dir_all(area.join("staging")).unwrap();
    let stopped = Preview::start(&site, &[]).err().unwrap();
    assert!(stopped.contains("stopped half-way"), "{stopped}");
    let recovered = run_in(&site, &["build"]);
    succeeded(&recovered);
    assert!(
        stderr(&recovered).contains("an interrupted build of"),
        "{}",
        stderr(&recovered)
    );
    assert!(!area.join("staging").exists() && !area.join("transaction").exists());
    drop(Preview::start(&site, &[]).unwrap());

    // Anything but a production manifest is refused.
    let manifest = site.join("dist/rustify-docs-manifest.json");
    let text = std::fs::read_to_string(&manifest).unwrap();
    std::fs::write(&manifest, text.replace("\"production\"", "\"dev\"")).unwrap();
    let dev = Preview::start(&site, &[]).err().unwrap();
    assert!(dev.contains("is a `dev` output"), "{dev}");
    std::fs::write(&manifest, "{").unwrap();
    let damaged = Preview::start(&site, &[]).err().unwrap();
    assert!(damaged.contains("is damaged"), "{damaged}");
}

#[test]
fn an_output_never_replaces_the_project_or_a_stranger() {
    let temp = content();
    let site = temp.path("site");
    for out in [".", "..", "content", "content/guide", "static", "../"] {
        let refused = run_in(&site, &["build", "--out", out]);
        assert_eq!(refused.status.code(), Some(1), "--out {out}");
        assert!(
            stderr(&refused).contains("a build replaces its output whole"),
            "--out {out}: {}",
            stderr(&refused)
        );
    }
    temp.write("site/src/notes.txt", "mine");
    let refused = run_in(&site, &["build", "--out", "src"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(
        stderr(&refused).contains("holds files that no build wrote"),
        "{}",
        stderr(&refused)
    );
    assert_eq!(temp.read("site/src/notes.txt"), "mine");
    assert_eq!(
        tree(&site.join("src")).len(),
        2,
        "the example source and the notes, untouched"
    );
}
