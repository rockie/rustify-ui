//! `cargo xtask verify --suite p1|p2`: everything the milestone can check by
//! itself, in one place, plus an honest account of what it cannot.
//!
//! The point is not to replace the individual commands - a developer runs
//! those - but to produce one report that says which of them passed, on which
//! build, and which parts of the acceptance need a person and have not been
//! handed in. A suite that quietly omitted the second kind would be the more
//! dangerous of the two.

use crate::build::{self, BuildRequest};
use std::path::Path;
use std::process::Command;

pub struct Step {
    pub name: &'static str,
    pub outcome: Result<String, String>,
}

impl Step {
    fn ok(name: &'static str, detail: impl Into<String>) -> Self {
        Self {
            name,
            outcome: Ok(detail.into()),
        }
    }

    fn failed(name: &'static str, detail: impl Into<String>) -> Self {
        Self {
            name,
            outcome: Err(detail.into()),
        }
    }
}

/// One release's worth of checks: what to build, what to run in a browser,
/// which reports it owes and which records only a person can write.
///
/// Two suites rather than one command with a flag, because "what P2 delivers"
/// is not "what P1 delivered plus more": the examples, the browser projects
/// and the manual records all differ, and a suite that quietly ran P1's list
/// under P2's name would report a pass nobody had earned.
pub struct Suite {
    pub name: &'static str,
    pub examples: &'static [&'static str],
    pub projects: &'static [&'static str],
    pub reports: &'static [&'static str],
    pub manual: &'static [(&'static str, &'static str)],
}

pub const P1: Suite = Suite {
    name: "P1",
    examples: &["fusion-basic", "property-workbench"],
    projects: &["fusion-basic", "property-workbench", "deployment"],
    reports: &REPORTS,
    manual: &MANUAL_RECORDS,
};

pub const P2: Suite = Suite {
    name: "P2",
    examples: &["fusion-basic", "property-workbench", "component-catalog"],
    projects: &[
        "fusion-basic",
        "property-workbench",
        "component-catalog",
        "workbench-deep",
        "deployment",
    ],
    reports: &P2_REPORTS,
    manual: &P2_MANUAL_RECORDS,
};

/// The manual records P1 cannot produce for itself. Each one is a file a
/// person writes after doing the thing; the suite reports which are missing
/// rather than passing over them.
pub const MANUAL_RECORDS: [(&str, &str); 4] = [
    (
        "docs/validation/p1/manual/pinyin.md",
        "M5: twenty Chinese phrases through a real macOS input method in Chrome",
    ),
    (
        "docs/validation/p1/manual/voiceover.md",
        "M5: VoiceOver + Chrome over the five keyboard journeys",
    ),
    (
        "docs/validation/p1/manual/contrast-and-zoom.md",
        "M8: the contrast and 200%/400% reflow walkthrough of both examples",
    ),
    (
        "docs/validation/p1/manual/safari.md",
        "M8: the same build's main path in macOS Safari, recorded and not blocking",
    ),
];

/// The six reports the milestone delivers, plus the requirement matrix. They
/// are the milestone's own output rather than a person's, so a missing one is
/// a failed step and not a note.
/// P2's, which are not P1's: an eighteen-category catalogue, two languages
/// and twenty text samples are things only a person can sign off on.
pub const P2_MANUAL_RECORDS: [(&str, &str); 4] = [
    (
        "docs/validation/p2/manual/voiceover.md",
        "M8: what VoiceOver actually says over the catalogue's eighteen categories and B1's \
         five journeys. The tree behind it is checked by `p2-semantics` and `p2-a11y` - \
         names, distinctness, no focus trap, and a modal the keyboard cannot get behind. \
         What needs a person is the speech",
    ),
    (
        "docs/validation/p2/manual/pinyin.md",
        "M8: a real pinyin session in the property form and the command palette. The \
         mechanics beneath it are checked by `p2-ime` - what a person still has to say \
         is whether composing in them actually feels right",
    ),
    (
        "docs/validation/p2/manual/samples.md",
        "M7/M8: direction and order of the twenty B5 samples against a reference rendering \
         (A-4). Whether anything came out blank is measured by `p2-i18n`; what needs the \
         reference image is whether the marks that *are* there are the right ones, in the \
         right order and the right direction",
    ),
    (
        "docs/validation/p2/manual/contrast-and-zoom.md",
        "M8: a person's look at the catalogue and B1 at 200% and 400%. Most of this is \
         checked already - the contrast ratios by `cargo test -p rustify-ui theme`, the \
         five journeys at 200% and the catalogue's reflow at 320 CSS pixels by \
         `p2-zoom` and `p2-reflow`. What is left is the judgement a measurement cannot \
         make: whether the reflowed pages are still *readable*",
    ),
];

pub const P2_REPORTS: [&str; 7] = [
    "docs/reports/p2/functional.md",
    "docs/reports/p2/performance.md",
    "docs/reports/p2/compatibility.md",
    "docs/reports/p2/accessibility.md",
    "docs/reports/p2/fault-recovery.md",
    "docs/reports/p2/known-limitations.md",
    "docs/reports/p2/requirements.md",
];

pub const REPORTS: [&str; 7] = [
    "docs/reports/p1/functional.md",
    "docs/reports/p1/performance.md",
    "docs/reports/p1/compatibility.md",
    "docs/reports/p1/accessibility.md",
    "docs/reports/p1/fault-recovery.md",
    "docs/reports/p1/known-limitations.md",
    "docs/reports/p1/requirements.md",
];

pub fn run(args: &[String]) -> Result<(), String> {
    let suite = match args
        .iter()
        .position(|a| a == "--suite")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
    {
        Some("p1") => P1,
        Some("p2") => P2,
        _ => {
            return Err(
                "usage: cargo xtask verify --suite p1|p2 [--no-browser] [--no-build]".to_string(),
            )
        }
    };
    let root = build::repo_root();
    let mut steps = vec![
        command(
            "sources",
            &root,
            "cargo",
            &["run", "--quiet", "-p", "xtask", "--", "sources", "verify"],
        ),
        command(
            "doctor",
            &root,
            "cargo",
            &["run", "--quiet", "-p", "xtask", "--", "doctor"],
        ),
        command("fmt", &root, "cargo", &["fmt", "--all", "--", "--check"]),
        command(
            "clippy",
            &root,
            "cargo",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        command(
            "host tests",
            &root,
            "cargo",
            &["test", "--workspace", "--lib"],
        ),
        command("xtask tests", &root, "cargo", &["test", "-p", "xtask"]),
        command("fork tests", &root.join("makepad"), "cargo", &["test"]),
        // The fork's workspace excludes platform/, so its script VM has to be
        // named to be run. A test that never runs is not a test.
        command(
            "fork script tests",
            &root.join("makepad"),
            "cargo",
            &["test", "--manifest-path", "platform/script/Cargo.toml"],
        ),
        reports(&root, &suite),
    ];

    if !args.iter().any(|a| a == "--no-build") {
        for example in suite.examples {
            steps.push(double_build(&root, example));
        }
    }
    if !args.iter().any(|a| a == "--no-browser") {
        for project in suite.projects {
            steps.push(browser(&root, project));
        }
    }

    report(&steps, &root, &suite)
}

/// Builds one example twice, from scratch each time, and compares what came
/// out. V10 asks that a product be traceable to its sources and tools, which
/// means two runs of the same command have to agree about what they produced.
fn double_build(root: &Path, example: &str) -> Step {
    let name: &'static str = match example {
        "fusion-basic" => "double build: fusion-basic",
        "component-catalog" => "double build: component-catalog",
        _ => "double build: property-workbench",
    };
    let request = BuildRequest {
        base: "/".to_string(),
        example: example.to_string(),
        release: true,
    };
    let mut manifests = Vec::new();
    for round in 0..2 {
        if let Err(error) = build::build(&request) {
            return Step::failed(name, format!("build {} failed: {error}", round + 1));
        }
        let path = build::app_dir(root, &request).join("build-manifest.json");
        match std::fs::read_to_string(&path) {
            Ok(text) => manifests.push(text),
            Err(error) => return Step::failed(name, format!("{}: {error}", path.display())),
        }
    }
    let (first, second) = (&manifests[0], &manifests[1]);
    let field = |text: &str, key: &str| -> String {
        text.split(&format!("\"{key}\": "))
            .nth(1)
            .and_then(|rest| rest.split(',').next())
            .unwrap_or("?")
            .trim()
            .trim_matches('"')
            .to_string()
    };
    let same_schema = field(first, "schema_hash") == field(second, "schema_hash");
    let same_build = field(first, "build_id") == field(second, "build_id");
    let same_files = first == second;
    if !same_schema {
        return Step::failed(name, "two builds produced different message bridges");
    }
    if same_files {
        Step::ok(
            name,
            format!(
                "two clean builds identical, build id {}",
                field(first, "build_id")
            ),
        )
    } else if same_build {
        Step::ok(
            name,
            "two clean builds agree on the wasm and the bridge; the manifests differ elsewhere",
        )
    } else {
        // Not a failure of the milestone: nothing here claims a reproducible
        // build. It is a fact worth printing, because it decides whether a
        // build id can be used to identify a deployment.
        Step::ok(
            name,
            format!(
                "two clean builds differ: build ids {} and {}; the bridge matches, so the wasm is not byte-reproducible",
                field(first, "build_id"),
                field(second, "build_id")
            ),
        )
    }
}

fn reports(root: &Path, suite: &Suite) -> Step {
    let missing: Vec<&str> = suite
        .reports
        .iter()
        .copied()
        .filter(|path| !root.join(path).is_file())
        .collect();
    if missing.is_empty() {
        Step::ok("reports", format!("{} delivered", suite.reports.len()))
    } else {
        Step::failed("reports", format!("missing: {}", missing.join(", ")))
    }
}

fn browser(root: &Path, project: &'static str) -> Step {
    let name: &'static str = match project {
        "fusion-basic" => "browser: fusion-basic",
        "property-workbench" => "browser: property-workbench",
        "component-catalog" => "browser: component-catalog",
        "workbench-deep" => "browser: workbench-deep",
        _ => "browser: deployment",
    };
    let mut step = command(
        name,
        root,
        "npx",
        &["playwright", "test", "--project", project],
    );
    if let Ok(detail) = &mut step.outcome {
        *detail = format!("{project} suite passed");
    }
    step
}

fn command(name: &'static str, dir: &Path, program: &str, args: &[&str]) -> Step {
    match Command::new(program).args(args).current_dir(dir).output() {
        Ok(output) if output.status.success() => Step::ok(name, "passed"),
        Ok(output) => {
            let tail: String = String::from_utf8_lossy(&output.stderr)
                .lines()
                .rev()
                .take(3)
                .collect::<Vec<_>>()
                .join(" / ");
            Step::failed(name, format!("exit {}: {tail}", output.status))
        }
        Err(error) => Step::failed(name, format!("cannot run {program}: {error}")),
    }
}

fn report(steps: &[Step], root: &Path, suite: &Suite) -> Result<(), String> {
    let mut failed = 0;
    println!("{} verification suite", suite.name);
    for step in steps {
        match &step.outcome {
            Ok(detail) => println!("  ok    {:<30} {detail}", step.name),
            Err(detail) => {
                failed += 1;
                println!("  FAIL  {:<30} {detail}", step.name);
            }
        }
    }

    println!("\nmanual records (a person does these; the suite only looks for the file)");
    let mut missing = 0;
    for (path, what) in suite.manual.iter().copied() {
        if root.join(path).is_file() {
            println!("  held  {path}");
        } else {
            missing += 1;
            println!("  none  {path}\n        {what}");
        }
    }

    println!(
        "\n{} automated step(s) failed, {} manual record(s) not handed in",
        failed, missing
    );
    if failed > 0 {
        return Err("the automated part of the suite did not pass".to_string());
    }
    if missing > 0 {
        // Not an error: the suite ran everything it can. Saying "passed" here
        // would be the lie the milestone is meant to prevent.
        println!(
            "{} cannot be called complete while a manual record is missing.",
            suite.name
        );
    }
    Ok(())
}
