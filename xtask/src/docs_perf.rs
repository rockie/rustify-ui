//! `docs-verify --group performance`: the docs site generator measured on
//! reference site B.
//!
//! This stage runs the risk gate that precedes the real `build` and `dev`:
//! B is generated fresh from the frozen profile, `rustify-docs`'s `b_poc`
//! harness (built with the `docs-cli` profile the CLI ships with) renders a
//! cold generation and five warm re-renders after a configuration change,
//! and the written pages are measured with the same `gzip -9` the size
//! report uses. The thresholds are the plan's; a miss fails the group.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::build::{compressed_sizes, repo_root};

/// Cold render of all of B with its stylesheet.
const COLD_LIMIT_MS: f64 = 20_000.0;
/// A configuration change re-rendering every page, committed in memory.
const HOT_CONFIG_LIMIT_MS: f64 = 1_000.0;
/// HTML, CSS and every framework script of one page, each `gzip -9`.
const PAGE_LIMIT_BYTES: u64 = 100_000;
/// `rustify-docs build` of all of B into an empty output.
const BUILD_LIMIT_MS: f64 = 20_000.0;
/// Full builds timed; the median is reported.
const BUILD_ROUNDS: usize = 5;

fn mbx() -> Command {
    let mut command = Command::new("mbx");
    // A child build lands in the repository's own target directory, whatever
    // this process inherited.
    command.env("CARGO_TARGET_DIR", repo_root().join("target"));
    command.current_dir(repo_root());
    command
}

fn run(command: &mut Command, what: &str) -> Result<String, String> {
    let output = command
        .output()
        .map_err(|error| format!("{what}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{what} failed:\n{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn files_under(dir: &Path, prefix: &str, out: &mut BTreeMap<String, u64>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        let path = entry.path();
        if path.is_dir() {
            files_under(&path, &format!("{name}/"), out)?;
        } else {
            out.insert(name, entry.metadata().map_err(|e| e.to_string())?.len());
        }
    }
    Ok(())
}

/// What machine and toolchain the numbers belong to.
fn environment() -> serde_json::Value {
    let text = |program: &str, args: &[&str]| {
        Command::new(program)
            .args(args)
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
            .unwrap_or_default()
    };
    serde_json::json!({
        "os": format!("{} {}", text("sw_vers", &["-productName"]), text("sw_vers", &["-productVersion"])),
        "arch": std::env::consts::ARCH,
        "cpu": text("sysctl", &["-n", "machdep.cpu.brand_string"]),
        "logical_cpus": std::thread::available_parallelism().map(usize::from).unwrap_or(0),
        "memory_bytes": text("sysctl", &["-n", "hw.memsize"]),
        "rustc": text("rustc", &["--version"]),
        "load_average": text("sysctl", &["-n", "vm.loadavg"]),
    })
}

pub fn performance(args: &[String]) -> Result<(), String> {
    let root = repo_root();
    let work: PathBuf = root.join("target/docs-ssg/perf");
    if work.exists() {
        std::fs::remove_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    }
    let site = work.join("b");
    let manifest = crate::docs_corpus::generate(&root, &site, crate::docs_corpus::DEFAULT_SEED)?;
    println!(
        "reference site B: {} pages, digest {}",
        manifest.pages, manifest.digest
    );

    run(
        mbx().args([
            "build",
            "--locked",
            "--profile",
            "docs-cli",
            "-p",
            "rustify-docs",
            "--example",
            "b_poc",
        ]),
        "building the b_poc harness",
    )?;
    let harness = root.join("target/docs-cli/examples/b_poc");
    let out = work.join("site");
    let report = run(
        Command::new(&harness)
            .arg(&site)
            .args(["--out"])
            .arg(&out)
            .args(["--rounds", "5"]),
        "running the b_poc harness",
    )?;
    let timing: serde_json::Value = serde_json::from_str(report.trim())
        .map_err(|e| format!("the harness report: {e}\n{report}"))?;

    let mut files = BTreeMap::new();
    files_under(&out, "", &mut files)?;
    let gzipped = compressed_sizes(&out, &files)?;
    let shared: u64 = gzipped
        .iter()
        .filter(|(name, _)| {
            name.starts_with("assets/") && (name.ends_with(".css") || name.ends_with(".js"))
        })
        .map(|(_, size)| size)
        .sum();
    let largest = |filter: &dyn Fn(&str) -> bool| {
        gzipped
            .iter()
            .filter(|(name, _)| name.ends_with(".html") && filter(name))
            .max_by_key(|(name, _)| (files[*name], (*name).clone()))
            .map(|(name, size)| (name.clone(), files[name], *size))
    };
    let home = largest(&|name| name == "index.html").ok_or("no home page")?;
    let content = largest(&|name| name != "index.html").ok_or("no content page")?;
    let page_total = |page: &(String, u64, u64)| page.2 + shared;

    // The real command, as an author runs it: `build` of all of B into an
    // empty output, timed from outside the process, five times.
    run(
        mbx().args([
            "build",
            "--locked",
            "--profile",
            "docs-cli",
            "-p",
            "rustify-docs-cli",
        ]),
        "building the rustify-docs CLI",
    )?;
    let cli = root.join("target/docs-cli/rustify-docs");
    let mut build_ms = Vec::with_capacity(BUILD_ROUNDS);
    let mut build_report = serde_json::Value::Null;
    for round in 0..BUILD_ROUNDS {
        let out = work.join(format!("build-{round}"));
        let started = std::time::Instant::now();
        let output = run(
            Command::new(&cli)
                .args(["build", "--format", "json", "--config"])
                .arg(site.join("rustify-docs.toml"))
                .arg("--out")
                .arg(&out),
            "rustify-docs build of B",
        )?;
        build_ms.push(started.elapsed().as_secs_f64() * 1000.0);
        build_report = serde_json::from_str(output.trim())
            .map_err(|e| format!("the build report: {e}\n{output}"))?;
        if round > 0 {
            std::fs::remove_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
        }
    }
    let mut sorted = build_ms.clone();
    sorted.sort_by(f64::total_cmp);
    let build_median = sorted[sorted.len() / 2];
    let build_summary = &build_report["summary"];
    println!(
        "rustify-docs build of B into an empty output: median {build_median:.1} ms of {build_ms:?}; {} pages, {} files, {} bytes",
        build_summary["pages"], build_summary["files"], build_summary["bytes"]
    );

    let cold_ms = timing["cold"]["render_and_style_ms"]
        .as_f64()
        .ok_or("cold timing")?;
    let hot_ms = timing["hot_config_median_ms"]
        .as_f64()
        .ok_or("hot timing")?;
    let errors = timing["errors"].as_u64().ok_or("error count")?;
    let warnings = timing["warnings"].as_u64().ok_or("warning count")?;
    let checks = [
        (
            "B analyses without a problem, so the timings are of a site that would publish",
            errors == 0 && warnings == 0,
            format!("{errors} error(s), {warnings} warning(s)"),
        ),
        ("cold render and stylesheet of 500 pages", cold_ms <= COLD_LIMIT_MS, format!("{cold_ms:.1} ms (limit {COLD_LIMIT_MS} ms)")),
        (
            "rustify-docs build of 500 pages into an empty output (median of 5)",
            build_median <= BUILD_LIMIT_MS && build_summary["pages"] == 500 && build_report["success"] == true,
            format!("{build_median:.1} ms (limit {BUILD_LIMIT_MS} ms), {} pages", build_summary["pages"]),
        ),
        (
            "configuration change, all pages re-rendered and committed (median of 5)",
            hot_ms < HOT_CONFIG_LIMIT_MS,
            format!("{hot_ms:.1} ms (limit < {HOT_CONFIG_LIMIT_MS} ms; {:.1} ms left for watch, SSE and the browser)", HOT_CONFIG_LIMIT_MS - hot_ms),
        ),
        (
            "largest content page: HTML + CSS + scripts, gzip -9",
            page_total(&content) <= PAGE_LIMIT_BYTES,
            format!("{} bytes ({} = {} raw / {} gzip + {shared} shared; limit {PAGE_LIMIT_BYTES})", page_total(&content), content.0, content.1, content.2),
        ),
        (
            "home page: HTML + CSS + scripts, gzip -9",
            page_total(&home) <= PAGE_LIMIT_BYTES,
            format!("{} bytes ({} raw / {} gzip + {shared} shared; limit {PAGE_LIMIT_BYTES})", page_total(&home), home.1, home.2),
        ),
    ];
    let mut failed = Vec::new();
    for (name, passed, detail) in &checks {
        println!("{} {name}: {detail}", if *passed { "pass" } else { "FAIL" });
        if !passed {
            failed.push(*name);
        }
    }
    let summary = serde_json::json!({
        "environment": environment(),
        "corpus": { "pages": manifest.pages, "seed": manifest.seed, "profile_sha256": manifest.profile_sha256, "digest": manifest.digest },
        "timing": timing,
        "build": {
            "rounds_ms": build_ms,
            "median_ms": build_median,
            "pages": build_summary["pages"],
            "files": build_summary["files"],
            "bytes": build_summary["bytes"],
            "largest": build_summary["largest"],
            "warnings": build_summary["warnings"],
        },
        "sizes": {
            "shared_gzip": shared,
            "files_gzip": gzipped.iter().filter(|(name, _)| name.starts_with("assets/")).collect::<BTreeMap<_, _>>(),
            "largest_content": { "path": content.0, "raw": content.1, "gzip": content.2, "total": page_total(&content) },
            "home": { "path": home.0, "raw": home.1, "gzip": home.2, "total": page_total(&home) },
        },
        "checks": checks.iter().map(|(name, passed, detail)| serde_json::json!({ "check": name, "passed": passed, "detail": detail })).collect::<Vec<_>>(),
    });
    let report_path = crate::option(args, "--report")
        .map(PathBuf::from)
        .unwrap_or_else(|| work.join("report.json"));
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(
        &report_path,
        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())? + "\n",
    )
    .map_err(|e| format!("{}: {e}", report_path.display()))?;
    println!("report: {}\nsite: {}", report_path.display(), out.display());
    if failed.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "performance: {} check(s) failed: {}",
            failed.len(),
            failed.join("; ")
        ))
    }
}
