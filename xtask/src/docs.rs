//! DOM-only builds of the SDK, and the checks that the default build is one.
//!
//! `build-dom` builds an application the way its own consumers do: Cargo for
//! `wasm32-unknown-unknown`, wasm-bindgen with `--target web`, and the
//! application's own `export-dom-assets` for the loader and the stylesheets.
//! No cargo-makepad and no glue patching, which is what keeps it honest about
//! what an outside consumer needs.
//!
//! `docs-verify --group features` is the evidence that the default really is
//! DOM only: the consumer's whole dependency graph - build dependencies too,
//! on the browser target and the host - has no Makepad package in it, the
//! built product has no Makepad file or import, the GPU API does not compile
//! without the `gpu` feature, and does with it. `--external` does the same
//! outside the repository: the SDK from a git commit, no root patch, plain
//! Cargo and the wasm-bindgen CLI.

use crate::option;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The wasm-bindgen release the SDK pins. The CLI that turns a consumer's wasm
/// into JS has to be the same one, or the glue and the wasm disagree.
const WASM_BINDGEN: &str = "0.2.128";

/// The fixture workspace, relative to the repository root.
const FIXTURE: &str = "tests/dom-only";

const USAGE: &str = "\
mbx xtask build-dom --manifest-path <Cargo.toml> [--output <directory>]
mbx xtask docs-verify --group features [--external [--git <url> --rev <sha>]]
mbx xtask docs-verify --group performance [--report <file>]
mbx xtask docs-verify --group search [--report <file>]
mbx xtask docs-verify --group artifacts
mbx xtask docs-verify --group reliability
mbx xtask docs-verify --group toolchain [--git <url> --rev <sha>]";

pub fn build_dom(args: &[String]) -> Result<(), String> {
    let manifest = option(args, "--manifest-path").ok_or(USAGE)?;
    let manifest = absolute(Path::new(manifest))?;
    let output = option(args, "--output").map(PathBuf::from);
    let built = build(&manifest, output.as_deref())?;
    println!("DOM build: {}", built.display());
    Ok(())
}

pub fn verify(args: &[String]) -> Result<(), String> {
    match option(args, "--group") {
        Some("features") if args.iter().any(|a| a == "--external") => {
            let git = option(args, "--git");
            let rev = option(args, "--rev");
            external(git, rev)
        }
        Some("features") => features(),
        Some("performance") => crate::docs_perf::performance(args),
        Some("search") => crate::docs_search::verify(args),
        Some("toolchain") => toolchain(option(args, "--git"), option(args, "--rev")),
        // What a production build publishes: every address under the base
        // and resolvable, the manifest, the site files, fingerprints that
        // change with their content alone, nothing private; and `new` ->
        // `build` with nothing compiled.
        Some("artifacts") => docs_tests(&[
            &["-p", "rustify-docs", "--test", "generate"],
            &["-p", "rustify-docs-cli", "--test", "artifacts"],
            &["-p", "rustify-docs-cli", "--test", "new_build"],
        ]),
        // The output transaction, its recovery and its lock, under real
        // kills, a full disk and refused permissions; reproducible bytes;
        // failed builds that keep the old output; preview's refusals and
        // address handling; outputs and files a build never touches; author
        // HTML the policy refuses, in every disguise; the machine's paths
        // and values kept out; links to other sites requested only on
        // request, and failures reported as failures.
        Some("reliability") => docs_tests(&[
            &["-p", "rustify-docs", "--lib"],
            &["-p", "rustify-docs-cli", "--bin", "rustify-docs"],
            &["-p", "rustify-docs-cli", "--test", "reliability"],
            &["-p", "rustify-docs-cli", "--test", "faults"],
            &["-p", "rustify-docs-cli", "--test", "external"],
        ]),
        _ => Err(USAGE.to_string()),
    }
}

/// Runs the docs packages' tests, each package on its own: they build with
/// `ssr`, which a workspace-wide run would unify with the browser crates'.
fn docs_tests(runs: &[&[&str]]) -> Result<(), String> {
    for run_args in runs {
        let mut command = Command::new("mbx");
        command.args(["test", "--locked"]).args(*run_args);
        run(&mut command, &format!("mbx test {}", run_args.join(" ")))?;
    }
    Ok(())
}

fn absolute(path: &Path) -> Result<PathBuf, String> {
    std::fs::canonicalize(path).map_err(|error| format!("{}: {error}", path.display()))
}

/// What Cargo says about the package at `manifest`.
struct Package {
    name: String,
    /// The library's file name in the target directory.
    lib: String,
    dir: PathBuf,
    target_dir: PathBuf,
}

fn package(cargo: &mut Command, manifest: &Path) -> Result<Package, String> {
    let output = cargo
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(manifest)
        .output()
        .map_err(|error| format!("cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    let manifest_text = manifest.to_string_lossy();
    let found = metadata["packages"]
        .as_array()
        .and_then(|packages| {
            packages
                .iter()
                .find(|p| p["manifest_path"].as_str() == Some(manifest_text.as_ref()))
        })
        .ok_or_else(|| format!("{} is not a package", manifest.display()))?;
    let lib = found["targets"]
        .as_array()
        .and_then(|targets| {
            targets.iter().find(|t| {
                t["kind"]
                    .as_array()
                    .is_some_and(|kinds| kinds.iter().any(|k| k == "cdylib"))
            })
        })
        .and_then(|t| t["name"].as_str())
        .ok_or_else(|| format!("{} has no cdylib library", manifest.display()))?;
    Ok(Package {
        name: found["name"].as_str().unwrap_or_default().to_string(),
        lib: lib.replace('-', "_"),
        dir: manifest.parent().unwrap_or(Path::new(".")).to_path_buf(),
        target_dir: PathBuf::from(
            metadata["target_directory"]
                .as_str()
                .ok_or("cargo metadata has no target_directory")?,
        ),
    })
}

/// Cargo through mbx, with one target directory for every DOM build. Left to
/// itself the directory depends on whether the caller's environment named
/// one, and a product that moves between runs is a product a check can miss.
fn mbx() -> Command {
    let mut command = Command::new("mbx");
    command.env(
        "CARGO_TARGET_DIR",
        crate::build::repo_root().join("target/dom-only"),
    );
    command
}

fn run(command: &mut Command, what: &str) -> Result<(), String> {
    let status = command
        .status()
        .map_err(|error| format!("{what}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{what} failed: {status}"))
    }
}

/// The browser build's flags: the clipboard needs web-sys's unstable APIs.
fn wasm_rustflags() -> String {
    let current = std::env::var("RUSTFLAGS").unwrap_or_default();
    if current.contains("web_sys_unstable_apis") {
        current
    } else {
        format!("{current} --cfg=web_sys_unstable_apis")
            .trim()
            .to_string()
    }
}

/// Builds the DOM application at `manifest` into `output` (by default
/// `<target>/dom/<package>`): the page's own files from `public/`, the
/// wasm-bindgen output in `pkg/`, and what `export-dom-assets` writes.
fn build(manifest: &Path, output: Option<&Path>) -> Result<PathBuf, String> {
    let package = package(&mut mbx(), manifest)?;
    run(
        mbx()
            .args([
                "build",
                "--locked",
                "--release",
                "--target",
                "wasm32-unknown-unknown",
                "--lib",
                "--manifest-path",
            ])
            .arg(manifest)
            .env("RUSTFLAGS", wasm_rustflags()),
        "the wasm build",
    )?;
    let output = output.map(Path::to_path_buf).unwrap_or_else(|| {
        crate::build::repo_root()
            .join("target/dom")
            .join(&package.name)
    });
    let wasm = package
        .target_dir
        .join("wasm32-unknown-unknown/release")
        .join(format!("{}.wasm", package.lib));
    assemble(&package.dir, &wasm, &output, |out| {
        run(
            mbx()
                .args([
                    "run",
                    "--locked",
                    "--quiet",
                    "--bin",
                    "export-dom-assets",
                    "--manifest-path",
                ])
                .arg(manifest)
                .arg("--")
                .arg(out),
            "export-dom-assets",
        )
    })?;
    Ok(output)
}

/// Lays out one product: `public/`, then `pkg/` from wasm-bindgen, then the
/// exported assets. `pkg/` is replaced whole, so a snippet the glue no longer
/// imports does not linger beside it.
fn assemble(
    package_dir: &Path,
    wasm: &Path,
    output: &Path,
    export: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    let public = package_dir.join("public");
    if public.is_dir() {
        copy_tree(&public, output)?;
    }
    let pkg = output.join("pkg");
    if pkg.exists() {
        std::fs::remove_dir_all(&pkg).map_err(|error| error.to_string())?;
    }
    wasm_bindgen_cli_support::Bindgen::new()
        .input_path(wasm)
        .web(true)
        .map_err(|error| error.to_string())?
        .typescript(false)
        .out_name("app")
        .generate(&pkg)
        .map_err(|error| format!("wasm-bindgen: {error:#}"))?;
    export(output)
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|error| error.to_string())?;
    for entry in std::fs::read_dir(from).map_err(|error| format!("{}: {error}", from.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)
                .map_err(|error| format!("{}: {error}", entry.path().display()))?;
        }
    }
    Ok(())
}

/// Whether a package name is Makepad's: the fork's crates, the integration and
/// its build tool. The fork's copies of `bitflags` and `smallvec` are general
/// libraries under their own names and are reported, not refused.
fn is_makepad(name: &str) -> bool {
    name.starts_with("makepad") || name == "rustify-makepad" || name == "cargo-makepad"
}

/// Every package in `manifest`'s graph of normal and build dependencies, for
/// every target platform, as `name version (source)` lines.
fn graph(cargo: &mut Command, manifest: &Path, package: &str) -> Result<Vec<String>, String> {
    dependency_graph(cargo, manifest, package, None, "{p}")
}

fn feature_graph(
    cargo: &mut Command,
    manifest: &Path,
    package: &str,
    mode: Option<&str>,
) -> Result<Vec<String>, String> {
    dependency_graph(cargo, manifest, package, mode, "{p}\t{f}")
}

fn dependency_graph(
    cargo: &mut Command,
    manifest: &Path,
    package: &str,
    mode: Option<&str>,
    format: &str,
) -> Result<Vec<String>, String> {
    cargo
        .args([
            "tree",
            "--locked",
            "--edges",
            "normal,build",
            "--target",
            "all",
            "--prefix",
            "none",
            "--no-dedupe",
            "--format",
            format,
            "--package",
            package,
            "--manifest-path",
        ])
        .arg(manifest);
    if let Some(mode) = mode {
        cargo.args(["--no-default-features", "--features", mode]);
    }
    let output = cargo
        .output()
        .map_err(|error| format!("cargo tree: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo tree: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let lines: BTreeSet<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim_end_matches(" (*)").trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(lines.into_iter().collect())
}

/// Fails if the graph has a Makepad package in it, and says what it has.
fn check_graph(label: &str, lines: &[String]) -> Result<(), String> {
    let makepad: Vec<&String> = lines
        .iter()
        .filter(|line| is_makepad(line.split_whitespace().next().unwrap_or_default()))
        .collect();
    let copies: Vec<&String> = lines
        .iter()
        .filter(|line| line.contains("makepad/libs/"))
        .collect();
    println!(
        "{label}: {} packages (normal and build dependencies, every target)",
        lines.len()
    );
    for copy in &copies {
        println!("  fork copy of a general library: {copy}");
    }
    if makepad.is_empty() {
        println!("  no makepad*, rustify-makepad or cargo-makepad package");
        Ok(())
    } else {
        Err(format!(
            "{label} links Makepad:\n  {}",
            makepad
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<_>>()
                .join("\n  ")
        ))
    }
}

/// The module names a wasm file imports from.
fn wasm_import_modules(bytes: &[u8]) -> Result<BTreeSet<String>, String> {
    fn leb(bytes: &[u8], at: &mut usize) -> Result<u64, String> {
        let mut value = 0u64;
        let mut shift = 0;
        loop {
            let byte = *bytes.get(*at).ok_or("truncated wasm")?;
            *at += 1;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
        }
    }
    fn name(bytes: &[u8], at: &mut usize) -> Result<String, String> {
        let len = leb(bytes, at)? as usize;
        let text = bytes.get(*at..*at + len).ok_or("truncated wasm")?;
        *at += len;
        Ok(String::from_utf8_lossy(text).into_owned())
    }
    fn limits(bytes: &[u8], at: &mut usize) -> Result<(), String> {
        let flags = leb(bytes, at)?;
        leb(bytes, at)?;
        if flags & 1 != 0 {
            leb(bytes, at)?;
        }
        Ok(())
    }
    if bytes.get(..4) != Some(b"\0asm") {
        return Err("not a wasm module".into());
    }
    let mut modules = BTreeSet::new();
    let mut at = 8;
    while at < bytes.len() {
        let id = bytes[at];
        at += 1;
        let size = leb(bytes, &mut at)? as usize;
        let end = at + size;
        if id == 2 {
            let mut cursor = at;
            for _ in 0..leb(bytes, &mut cursor)? {
                modules.insert(name(bytes, &mut cursor)?);
                name(bytes, &mut cursor)?;
                let kind = *bytes.get(cursor).ok_or("truncated wasm")?;
                cursor += 1;
                match kind {
                    0 => {
                        leb(bytes, &mut cursor)?;
                    }
                    1 => {
                        cursor += 1;
                        limits(bytes, &mut cursor)?;
                    }
                    2 => limits(bytes, &mut cursor)?,
                    3 => cursor += 2,
                    4 => {
                        cursor += 1;
                        leb(bytes, &mut cursor)?;
                    }
                    other => return Err(format!("unknown import kind {other}")),
                }
            }
        }
        at = end;
    }
    Ok(modules)
}

/// Fails if the built product carries anything of Makepad's: a file, an
/// import in the glue, or a wasm import from anywhere but the glue and the
/// SDK's own snippet.
fn check_product(output: &Path) -> Result<(), String> {
    check_product_at(output, &output.join("pkg"))
}

fn check_product_at(output: &Path, bindings: &Path) -> Result<(), String> {
    check_product_with_prefix(output, bindings, "rustify-dom")
}

fn check_product_with_prefix(output: &Path, bindings: &Path, runtime: &str) -> Result<(), String> {
    let snippet_prefix = format!("./snippets/{runtime}-");
    let mut files = Vec::new();
    let mut stack = vec![output.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).map_err(|error| error.to_string())? {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(
                    path.strip_prefix(output)
                        .map_err(|error| error.to_string())?
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    files.sort();
    println!("product {}: {} files", output.display(), files.len());
    for file in &files {
        println!("  {file}");
    }
    let foreign: Vec<&String> = files
        .iter()
        .filter(|file| file.contains("makepad") || file.ends_with("embedded.js"))
        .collect();
    if !foreign.is_empty() {
        return Err(format!("the product has Makepad files: {foreign:?}"));
    }
    let glue = std::fs::read_to_string(bindings.join("app.js"))
        .map_err(|error| format!("app.js: {error}"))?;
    for line in glue
        .lines()
        .filter(|line| line.trim_start().starts_with("import "))
    {
        if line.contains("makepad") || !line.contains(&snippet_prefix) {
            return Err(format!(
                "the glue imports something other than the SDK runtime: {line}"
            ));
        }
    }
    let wasm = std::fs::read(bindings.join("app_bg.wasm"))
        .map_err(|error| format!("app_bg.wasm: {error}"))?;
    let modules = wasm_import_modules(&wasm)?;
    println!("  wasm imports from: {modules:?}");
    let unexpected: Vec<&String> = modules
        .iter()
        .filter(|module| {
            // The glue's own module (`./app_bg.js` from wasm-bindgen 0.2.128,
            // `wbg` before it) and the SDK's snippet; nothing else.
            module.as_str() != "wbg"
                && module.as_str() != "./app_bg.js"
                && !module.starts_with(&snippet_prefix)
        })
        .collect();
    if !unexpected.is_empty() {
        return Err(format!("the wasm imports from {unexpected:?}"));
    }
    if !files
        .iter()
        .any(|file| file.ends_with("web/dom-runtime.js"))
    {
        return Err("the product has no SDK runtime snippet".into());
    }
    for file in files.iter().filter(|file| file.ends_with(".js")) {
        let text = std::fs::read_to_string(output.join(file)).map_err(|error| error.to_string())?;
        for line in text.lines().filter(|line| {
            line.trim_start().starts_with("import ")
                || line.contains(" from ")
                || line.contains("import(")
                || line.contains("import (")
        }) {
            if line.to_ascii_lowercase().contains("makepad") {
                return Err(format!("the product imports Makepad in {file}: {line}"));
            }
        }
    }
    Ok(())
}

/// The GPU API must not compile without the feature, and must with it.
fn check_gpu_switch(cargo: impl Fn() -> Command, fixture: &Path) -> Result<(), String> {
    let negative = fixture.join("gpu-negative/Cargo.toml");
    let output = cargo()
        .args(["check", "--locked", "--manifest-path"])
        .arg(&negative)
        .output()
        .map_err(|error| format!("cargo check: {error}"))?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        return Err("the GPU API compiled without the gpu feature".into());
    }
    for name in ["GpuRegion", "RegionId", "RegionState"] {
        if !stderr.contains(name) {
            return Err(format!(
                "the GPU negative probe failed, but not on {name}:\n{stderr}"
            ));
        }
    }
    println!("GPU API without the gpu feature: refused by the compiler (GpuRegion, RegionId, RegionState)");
    let positive = fixture.join("Cargo.toml");
    for target in [None, Some("wasm32-unknown-unknown")] {
        let mut command = cargo();
        command
            .args(["check", "--locked", "-p", "gpu-opt-in", "--manifest-path"])
            .arg(&positive);
        if let Some(target) = target {
            command
                .args(["--target", target])
                .env("RUSTFLAGS", wasm_rustflags());
        }
        run(&mut command, "the GPU opt-in probe")?;
    }
    println!("GPU API with the gpu feature: compiles on the host and for wasm32");
    Ok(())
}

/// The fork's general-library copies the root patch points at have no change
/// against the commit the tree started from.
fn check_library_copies(root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args([
            "status",
            "--porcelain",
            "--",
            "makepad/libs/bitflags",
            "makepad/libs/smallvec",
        ])
        .current_dir(root)
        .output()
        .map_err(|error| format!("git status: {error}"))?;
    let changed = String::from_utf8_lossy(&output.stdout);
    if !changed.trim().is_empty() {
        return Err(format!("the patched library copies changed:\n{changed}"));
    }
    let lock: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.join("sources.lock.json")).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let frozen = lock["makepad"]["files"]
        .as_object()
        .ok_or("sources.lock.json has no makepad files")?;
    for library in ["libs/bitflags/", "libs/smallvec/"] {
        let current = crate::sources::fingerprint_tree(&root.join("makepad").join(library))?;
        let mut drift = Vec::new();
        for (path, digest) in frozen.iter().filter(|(path, _)| path.starts_with(library)) {
            match current.get(&path[library.len()..]) {
                Some(now) if Some(now.as_str()) == digest.as_str() => {}
                Some(_) => drift.push(format!("modified {path}")),
                None => drift.push(format!("removed {path}")),
            }
        }
        for path in current.keys() {
            if !frozen.contains_key(&format!("{library}{path}")) {
                drift.push(format!("added {library}{path}"));
            }
        }
        println!(
            "makepad/{library}: {} files, drift against sources.lock.json: {}",
            current.len(),
            if drift.is_empty() {
                "none".to_string()
            } else {
                drift.join(", ")
            }
        );
    }
    println!("makepad/libs/bitflags and makepad/libs/smallvec: unchanged against HEAD");
    Ok(())
}

/// The `-sys` packages a docs graph may hold, and why each needs no system
/// library on macOS.
const SYS_ALLOWED: &[(&str, &str)] = &[
    (
        "js-sys",
        "wasm-bindgen's JavaScript bindings: Rust declarations, nothing linked on the host",
    ),
    (
        "web-sys",
        "wasm-bindgen's DOM bindings: Rust declarations, nothing linked on the host",
    ),
    (
        "windows-sys",
        "Windows API declarations, used only when the target is Windows",
    ),
    (
        "fsevent-sys",
        "macOS FSEvents for `dev`'s watcher: links the CoreServices and CoreFoundation frameworks, part of macOS; no build script",
    ),
    (
        "inotify-sys",
        "Linux inotify declarations over libc, used only when the target is Linux or Android",
    ),
    (
        "kqueue-sys",
        "BSD kqueue declarations over libc, used only when the target is a BSD or iOS; no build script",
    ),
];

/// Build tools a docs graph must not pull in: each one is a program or a
/// system library an author would have to install beside Rust.
const TOOL_PACKAGES: &[&str] = &[
    "cmake",
    "pkg-config",
    "vcpkg",
    "bindgen",
    "openssl",
    "openssl-sys",
    "onig",
    "onig_sys",
    "libz-sys",
    "libgit2-sys",
    "curl-sys",
];

/// Features that must not be on in a host-rendering graph.
const BROWSER_FEATURES: &[(&str, &[&str])] = &[
    ("leptos", &["csr", "hydrate"]),
    ("reactive_graph", &["effects"]),
    ("rustify-ui", &["csr", "hydrate", "gpu"]),
    ("rustify-dom", &["csr", "hydrate"]),
    ("rustify-components", &["csr", "hydrate", "gpu"]),
    ("rustify-docs", &["hydrate"]),
];

fn check_feature_rules(
    label: &str,
    lines: &[String],
    rules: &[(&str, &[&str])],
) -> Result<(), String> {
    for (crate_name, forbidden) in rules {
        for line in lines
            .iter()
            .filter(|line| line.split_whitespace().next() == Some(*crate_name))
        {
            let features: BTreeSet<&str> = line
                .split_once('\t')
                .map_or("", |(_, features)| features)
                .split(',')
                .collect();
            let on: Vec<&&str> = forbidden
                .iter()
                .filter(|feature| features.contains(**feature))
                .collect();
            if !on.is_empty() {
                return Err(format!("{label}: {crate_name} has {on:?} on: {line}"));
            }
        }
    }
    Ok(())
}

/// The docs library and the CLI, each on its own: no Makepad, no `-sys`
/// package outside the allowlist, no build tool, and no browser rendering
/// mode switched on.
fn check_docs_graphs(root: &Path) -> Result<(), String> {
    let manifest = root.join("Cargo.toml");
    for package in ["rustify-docs", "rustify-docs-cli"] {
        let lines = graph(&mut mbx(), &manifest, package)?;
        check_graph(&format!("{package} (alone)"), &lines)?;
        let names: BTreeSet<&str> = lines
            .iter()
            .filter_map(|line| line.split_whitespace().next())
            .collect();
        let sys: Vec<&str> = names
            .iter()
            .copied()
            .filter(|name| name.ends_with("-sys") || name.ends_with("_sys"))
            .collect();
        for name in &sys {
            match SYS_ALLOWED.iter().find(|(allowed, _)| allowed == name) {
                Some((_, why)) => println!("  {name}: {why}"),
                None => {
                    return Err(format!(
                        "{package} pulls in {name}, which may need a system library"
                    ))
                }
            }
        }
        let tools: Vec<&&str> = TOOL_PACKAGES
            .iter()
            .filter(|tool| names.contains(**tool))
            .collect();
        if !tools.is_empty() {
            return Err(format!(
                "{package} pulls in {tools:?}, which need a tool or library beside Rust"
            ));
        }
        check_feature_rules(
            package,
            &feature_graph(&mut mbx(), &manifest, package, None)?,
            BROWSER_FEATURES,
        )?;
        println!(
            "  no csr, hydrate, effects or gpu feature; {} -sys package(s), no build tool",
            sys.len()
        );
    }
    Ok(())
}

const SITE_PIPELINE_PACKAGES: &[&str] = &[
    "pulldown-cmark",
    "syntect",
    "two-face",
    "lightningcss",
    "cssparser",
    "html5gum",
    "html5ever",
    "dom_query",
    "serde_yaml_ng",
    "classification-macros",
    "rustify-classification-macros",
    "fast-glob",
    "bexpand",
];

const SITE_PIPELINE_FEATURES: &[(&str, &[&str])] = &[
    ("rustify-docs", &["pipeline"]),
    ("rustify-style", &["compiler", "scanner", "scanner-fs"]),
    ("rustify-ui", &["gpu"]),
    ("rustify-components", &["gpu"]),
];

fn check_site_graph(label: &str, mode: &str, lines: &[String]) -> Result<(), String> {
    let packages: BTreeSet<String> = lines
        .iter()
        .map(|line| {
            line.split_once('\t')
                .map_or(line.as_str(), |(package, _)| package)
                .to_string()
        })
        .collect();
    check_graph(label, &packages.into_iter().collect::<Vec<_>>())?;
    let names: BTreeSet<&str> = lines
        .iter()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let private: Vec<&&str> = SITE_PIPELINE_PACKAGES
        .iter()
        .filter(|name| names.contains(**name))
        .collect();
    if !private.is_empty() {
        return Err(format!(
            "{label} carries private content pipeline packages: {private:?}"
        ));
    }
    check_feature_rules(label, lines, SITE_PIPELINE_FEATURES)?;
    if mode == "ssr" {
        check_feature_rules(label, lines, BROWSER_FEATURES)?;
    } else {
        check_feature_rules(
            label,
            lines,
            &[
                ("leptos", &["csr", "ssr"]),
                ("rustify-ui", &["csr", "ssr"]),
                ("rustify-components", &["csr", "ssr"]),
                ("rustify-docs", &["site-ssr"]),
            ],
        )?;
    }
    let docs_mode = if mode == "ssr" { "site-ssr" } else { "hydrate" };
    if !lines.iter().any(|line| {
        line.split_whitespace().next() == Some("rustify-docs")
            && line.split_once('\t').is_some_and(|(_, features)| {
                features.split(',').any(|feature| feature == docs_mode)
            })
    }) {
        return Err(format!("{label} did not enable rustify-docs/{docs_mode}"));
    }
    println!("  isolated {mode}; no private Markdown/highlighting/CSS compiler/scanner pipeline");
    Ok(())
}

/// Adapt an explicit test consumer with the real under-test CLI's pair.
pub(super) fn pair_project(
    root: &Path,
    manifest: &Path,
    out: Option<&Path>,
) -> Result<serde_json::Value, String> {
    let cli = std::env::var_os("RUSTIFY_DOCS_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/docs-cli/rustify-docs"));
    if !cli.is_file() {
        return Err("build the under-test CLI with `mbx build --locked --profile docs-cli -p rustify-docs-cli`, or set RUSTIFY_DOCS_BIN to that real binary".into());
    }
    let mut command = Command::new("python3");
    command
        .arg(root.join("scripts/docs-pages-prepare.py"))
        .arg("--project")
        .arg(manifest)
        .arg("--cli")
        .arg(cli);
    if let Some(out) = out {
        command.arg("--out").arg(out);
    }
    let output = command.output().map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())
}

/// Exercise the author CLI's public build entry, so the checked glue, wasm,
/// runtime and snippets are the same complete group an author publishes.
fn check_component_site(root: &Path) -> Result<(), String> {
    let fixture = root.join("tests/docs-ssg/fixtures/components");
    let manifest = fixture.join("Cargo.toml");
    for mode in ["ssr", "hydrate"] {
        let lines = feature_graph(&mut mbx(), &manifest, "docs-components-fixture", Some(mode))?;
        check_site_graph(&format!("components fixture/{mode}"), mode, &lines)?;
    }
    run(
        mbx()
            .current_dir(root)
            .env("CARGO_TARGET_DIR", root.join("target"))
            .args([
                "build",
                "--locked",
                "--package",
                "rustify-docs-cli",
                "--profile",
                "docs-cli",
            ]),
        "under-test CLI for the component consumer",
    )?;
    let author = root.join("target/docs-ssg/features/components-author");
    if author.exists() {
        std::fs::remove_dir_all(&author).map_err(|error| error.to_string())?;
    }
    let pair = pair_project(root, &manifest, Some(&author))?;
    let fixture = Path::new(
        pair["project"]
            .as_str()
            .ok_or("pair helper omitted project")?,
    );
    let cli = std::env::var_os("RUSTIFY_DOCS_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/docs-cli/rustify-docs"));
    let output = root.join("target/docs-ssg/features/components");
    run(
        Command::new(cli)
            .current_dir(root)
            .env("RUSTIFY_DOCS_CARGO", "mbx")
            .env("RUSTUP_AUTO_INSTALL", "0")
            .args(["build", "--config"])
            .arg(fixture.join("rustify-docs.toml"))
            .arg("--out")
            .arg(&output),
        "components fixture public CLI build",
    )?;
    let groups = output.join("assets/islands");
    let mut checked = 0;
    for entry in
        std::fs::read_dir(&groups).map_err(|error| format!("{}: {error}", groups.display()))?
    {
        let group = entry.map_err(|error| error.to_string())?.path();
        if !group.is_dir() {
            return Err("island assets must be complete resource-group directories".to_string());
        }
        for required in [
            "app.js",
            "app_bg.wasm",
            "dom-loader.js",
            "dom-runtime.css",
            "islands.js",
        ] {
            if !group.join(required).is_file() {
                return Err(format!("the island resource group is missing {required}"));
            }
        }
        check_product_at(&group, &group)?;
        checked += 1;
    }
    if checked == 0 {
        return Err("the component fixture published no island resource group".to_string());
    }
    println!(
        "component site: {checked} complete island resource group(s) at {}",
        output.display()
    );
    Ok(())
}

fn features() -> Result<(), String> {
    let root = crate::build::repo_root();
    let fixture = root.join(FIXTURE);
    let manifest = fixture.join("consumer/Cargo.toml");
    let output = build(&manifest, None)?;
    check_graph(
        "dom-consumer (in the repository, path dependencies)",
        &graph(&mut mbx(), &manifest, "dom-consumer")?,
    )?;
    check_product(&output)?;
    check_gpu_switch(mbx, &fixture)?;
    check_library_copies(&root)?;
    check_docs_graphs(&root)?;
    build_hydration(&root)?;
    check_component_site(&root)?;
    println!(
        "features: the default build is DOM only; the product is {}",
        output.display()
    );
    Ok(())
}

/// Compiles the same independent view in each rendering mode. The host
/// writes static roots; each browser target has its own feature graph and
/// bindgen output so a CSR union cannot stand in for hydration.
fn build_hydration(root: &Path) -> Result<(), String> {
    let manifest = root.join("tests/dom-only/hydration/Cargo.toml");
    let output = root.join("target/dom/hydration");
    let target = |mode: &str| root.join(format!("target/dom-hydration-{mode}"));
    let command = |mode: &str| {
        let mut command = Command::new("mbx");
        command.env("CARGO_TARGET_DIR", target(mode));
        command
    };
    for mode in ["ssr", "hydrate", "csr"] {
        let mut cargo = command(mode);
        let graph_output = cargo
            .args([
                "tree",
                "--locked",
                "--edges",
                "normal,build",
                "--target",
                "all",
                "--prefix",
                "none",
                "--no-dedupe",
                "--package",
                "hydration-harness",
                "--manifest-path",
            ])
            .arg(&manifest)
            .args(["--no-default-features", "--features", mode])
            .output()
            .map_err(|error| format!("hydration graph: {error}"))?;
        if !graph_output.status.success() {
            return Err(format!(
                "hydration {mode} graph: {}",
                String::from_utf8_lossy(&graph_output.stderr)
            ));
        }
        let lines = String::from_utf8_lossy(&graph_output.stdout)
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();
        check_graph(&format!("hydration-harness/{mode}"), &lines)?;
    }
    run(
        command("ssr")
            .args(["run", "--locked", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .args([
                "--no-default-features",
                "--features",
                "ssr",
                "--bin",
                "export-hydration-assets",
                "--",
            ])
            .arg(&output),
        "hydration host generation",
    )?;
    for mode in ["hydrate", "csr"] {
        run(
            command(mode)
                .args(["build", "--locked", "--release", "--manifest-path"])
                .arg(&manifest)
                .args([
                    "--no-default-features",
                    "--features",
                    mode,
                    "--lib",
                    "--target",
                    "wasm32-unknown-unknown",
                ])
                .env("RUSTFLAGS", wasm_rustflags()),
            &format!("hydration {mode} wasm build"),
        )?;
        let wasm = target(mode).join("wasm32-unknown-unknown/release/hydration_harness.wasm");
        let product = output.join(mode);
        assemble(
            manifest
                .parent()
                .ok_or("hydration manifest has no directory")?,
            &wasm,
            &product,
            |_| Ok(()),
        )?;
        check_product(&product)?;
    }
    println!(
        "hydration: host, hydrate and independent CSR at {}",
        output.display()
    );
    Ok(())
}

/// A scratch directory outside the repository: a package inside it would be
/// taken for a member of the root workspace.
fn scratch() -> PathBuf {
    std::env::temp_dir().join("rustify-docs-verify")
}

/// The consumer's toolchain: the stable `cargo` and `rustc` themselves, with
/// nothing of this repository's in front of them - no mbx, no wrapper, no
/// pinned toolchain variable - and a PATH of that toolchain and the system.
fn plain_cargo(sysroot: &Path) -> Command {
    let bin = sysroot.join("bin");
    let mut command = Command::new(bin.join("cargo"));
    command
        .env_clear()
        .env("HOME", std::env::var_os("HOME").unwrap_or_default())
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("RUSTC", bin.join("rustc"))
        .env("TMPDIR", std::env::temp_dir());
    // Git installation and dependency resolution still need the machine's
    // network route. The author's offline walkthrough has its own environment.
    for name in [
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "NO_PROXY",
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "no_proxy",
    ] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    command
}

fn sysroot() -> Result<PathBuf, String> {
    let output = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .map_err(|error| format!("rustc: {error}"))?;
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

/// Commits the working tree, tracked and untracked files alike but nothing git
/// ignores, into a bare repository outside this one, without touching this
/// one's index, branches or files. Returns its file URL and the commit.
fn candidate(root: &Path, work: &Path) -> Result<(String, String), String> {
    let git = |args: &[&str], index: Option<&Path>| -> Result<String, String> {
        let mut command = Command::new("git");
        command.args(args).current_dir(root);
        if let Some(index) = index {
            command.env("GIT_INDEX_FILE", index);
        }
        let output = command
            .output()
            .map_err(|error| format!("git {}: {error}", args.join(" ")))?;
        if !output.status.success() {
            return Err(format!(
                "git {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    };
    let index = work.join("candidate.index");
    let _ = std::fs::remove_file(&index);
    git(&["read-tree", "HEAD"], Some(&index))?;
    git(&["add", "-A", "."], Some(&index))?;
    let tree = git(&["write-tree"], Some(&index))?;
    let head = git(&["rev-parse", "HEAD"], None)?;
    let commit = git(
        &[
            "commit-tree",
            &tree,
            "-p",
            &head,
            "-m",
            "docs-verify candidate: the working tree as an outside consumer sees it",
        ],
        None,
    )?;
    let bare = work.join("candidate.git");
    if !bare.exists() {
        git(
            &["init", "--bare", "--quiet", &bare.to_string_lossy()],
            None,
        )?;
    }
    git(
        &[
            "push",
            "--quiet",
            "--force",
            &bare.to_string_lossy(),
            &format!("{commit}:refs/heads/candidate"),
        ],
        None,
    )?;
    Ok((format!("file://{}", bare.display()), commit))
}

/// The consumer outside the repository: its manifest with the SDK from `git`
/// at `rev`, its lock carried over so nothing else moves, built with plain
/// Cargo and the wasm-bindgen CLI.
fn external(git: Option<&str>, rev: Option<&str>) -> Result<(), String> {
    let root = crate::build::repo_root();
    let work = scratch();
    std::fs::create_dir_all(&work).map_err(|error| error.to_string())?;
    let (url, rev) = match (git, rev) {
        (Some(url), Some(rev)) => (url.to_string(), rev.to_string()),
        (None, None) => candidate(&root, &work)?,
        _ => return Err("--git and --rev go together".into()),
    };
    println!("SDK source: {url} at {rev}");

    let consumer = work.join("dom-consumer");
    if consumer.exists() {
        std::fs::remove_dir_all(&consumer).map_err(|error| error.to_string())?;
    }
    let fixture = root.join(FIXTURE);
    copy_tree(&fixture.join("consumer/src"), &consumer.join("src"))?;
    copy_tree(&fixture.join("consumer/public"), &consumer.join("public"))?;
    std::fs::copy(fixture.join("Cargo.lock"), consumer.join("Cargo.lock"))
        .map_err(|error| error.to_string())?;
    let manifest = std::fs::read_to_string(fixture.join("consumer/Cargo.toml"))
        .map_err(|error| error.to_string())?;
    let mut rewritten = manifest.clone();
    for (path, name) in [
        ("../../../crates/rustify-ui", "rustify-ui"),
        ("../../../crates/rustify-components", "rustify-components"),
    ] {
        let from = format!("{name} = {{ path = \"{path}\" }}");
        if !rewritten.contains(&from) {
            return Err(format!("the fixture manifest no longer has `{from}`"));
        }
        rewritten = rewritten.replace(
            &from,
            &format!("{name} = {{ git = \"{url}\", rev = \"{rev}\" }}"),
        );
    }
    std::fs::write(consumer.join("Cargo.toml"), rewritten).map_err(|error| error.to_string())?;

    let sysroot = sysroot()?;
    let cargo = || {
        let mut command = plain_cargo(&sysroot);
        command
            .current_dir(&consumer)
            .env("CARGO_TARGET_DIR", consumer.join("target"));
        command
    };
    // Moves the SDK and the components from their paths to the commit and
    // nothing else: every other package keeps the version in the lock.
    run(
        cargo()
            .args(["metadata", "--format-version", "1"])
            .stdout(std::process::Stdio::null()),
        "resolving the git dependencies",
    )?;
    let manifest = consumer.join("Cargo.toml");
    let consumer_graph = graph(&mut cargo(), &manifest, "dom-consumer")?;
    check_graph(
        "dom-consumer (outside the repository, git dependencies, no patch)",
        &consumer_graph,
    )?;
    let runtime = if consumer_graph
        .iter()
        .any(|line| line.starts_with("rustify-dom "))
    {
        "rustify-dom"
    } else {
        "rustify-ui"
    };

    let tools = work.join("tools");
    let bindgen = tools.join("bin/wasm-bindgen");
    let installed = Command::new(&bindgen)
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains(WASM_BINDGEN))
        .unwrap_or(false);
    if !installed {
        run(
            cargo()
                .args([
                    "install",
                    "wasm-bindgen-cli",
                    "--version",
                    WASM_BINDGEN,
                    "--locked",
                    "--root",
                ])
                .arg(&tools),
            "installing wasm-bindgen-cli",
        )?;
    }
    run(
        cargo()
            .args([
                "build",
                "--locked",
                "--release",
                "--target",
                "wasm32-unknown-unknown",
                "--lib",
            ])
            .env("RUSTFLAGS", "--cfg=web_sys_unstable_apis"),
        "the consumer's wasm build",
    )?;
    let public = consumer.join("public");
    run(
        Command::new(&bindgen)
            .arg(consumer.join("target/wasm32-unknown-unknown/release/dom_consumer.wasm"))
            .args(["--target", "web", "--out-name", "app", "--out-dir"])
            .arg(public.join("pkg")),
        "wasm-bindgen",
    )?;
    run(
        cargo().args(["run", "--locked", "--quiet", "--bin", "export-dom-assets"]),
        "export-dom-assets",
    )?;
    check_product_with_prefix(&public, &public.join("pkg"), runtime)?;
    println!(
        "features (outside the repository): the default build is DOM only; the product is {}",
        public.display()
    );
    println!("serve it with: DOM_ONLY_DIR={} npx playwright test -c playwright.docs.config.ts --project=dom-only", public.display());
    Ok(())
}

/// What an author's machine must not need: Node and its package tools, the
/// Tailwind CLI, the wasm build tools, this repository's compiler cache,
/// containers and browsers. The walkthrough's PATH must offer none of them.
const NOT_NEEDED: &[&str] = &[
    "node",
    "npm",
    "npx",
    "tailwindcss",
    "wasm-pack",
    "wasm-bindgen",
    "mbx",
    "docker",
    "podman",
    "chromium",
    "google-chrome",
    "firefox",
];

const SANDBOX: &str = "/usr/bin/sandbox-exec";

/// No network at all, loopback included. A developer's machine may run an
/// HTTP proxy on loopback (this one does), and a request handed to it leaves
/// the machine, so a profile that let loopback through would witness nothing.
const OFFLINE: &str = "(version 1)(allow default)(deny network*)";

/// The servers: listening and answering stay allowed, and the only
/// connections they may open are to loopback and local sockets.
const LOOPBACK: &str = "(version 1)(allow default)(deny network-outbound)\
    (allow network-outbound (remote ip \"localhost:*\"))\
    (allow network-outbound (remote unix-socket))";

/// An address that answers whenever there is a network. Added to the site
/// for the offline link check, so that `unreachable` can only mean that the
/// sandbox held and that the CLI did not pass the failure off as a success.
const PROBE_LINK: &str = "https://www.rust-lang.org/";

/// The production output's own description of itself.
const MANIFEST: &str = "rustify-docs-manifest.json";

/// How long a server may take to print its address, and an HTTP exchange to
/// finish: a hang fails the check instead of stalling it.
const SERVER_START: std::time::Duration = std::time::Duration::from_secs(60);
const HTTP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);

/// The CLI installed the way the documentation says, from a commit: plain
/// Cargo with a cleared environment, `cargo install --git … --locked
/// --profile docs-cli`, into a root outside the repository. The commit is a
/// snapshot of the working tree, or the one `--git` and `--rev` name. The
/// installed binary must report the root workspace's `docs-cli` profile (it
/// unwinds) and recover from a panic, and `cargo install --list` must name
/// the commit it came from. Then an author uses it with nothing but Rust
/// beside it (see `walkthrough`).
fn toolchain(git: Option<&str>, rev: Option<&str>) -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Err(
            "the toolchain group runs on macOS, the one platform the CLI supports; \
             its network witness is sandbox-exec"
                .into(),
        );
    }
    let root = crate::build::repo_root();
    let work = scratch();
    std::fs::create_dir_all(&work).map_err(|error| error.to_string())?;
    let (url, rev) = match (git, rev) {
        (Some(url), Some(rev)) => (url.to_string(), rev.to_string()),
        (None, None) => candidate(&root, &work)?,
        _ => return Err("--git and --rev go together".into()),
    };
    if rev.len() != 40 || !rev.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(
            "--rev must be the complete 40-character Git commit installed by this check".into(),
        );
    }
    let sysroot = sysroot()?;
    environment(&sysroot)?;
    println!("CLI source: {url} at {rev}");
    let install_root = work.join("cli-install");
    if install_root.exists() {
        std::fs::remove_dir_all(&install_root).map_err(|error| error.to_string())?;
    }
    let cargo = || {
        let mut command = plain_cargo(&sysroot);
        // Match an author's install: Cargo owns a fresh temporary target.
        // Reusing the witness's target can accept stale local build artifacts.
        command.current_dir(&work);
        command
    };
    let started = std::time::Instant::now();
    run(
        cargo()
            .args([
                "install",
                "--git",
                &url,
                "--rev",
                &rev,
                "--locked",
                "--profile",
                "docs-cli",
                "--root",
            ])
            .arg(&install_root)
            .arg("rustify-docs-cli"),
        "cargo install --profile docs-cli",
    )?;
    println!("installed in {:.1} s", started.elapsed().as_secs_f64());
    let binary = install_root.join("bin/rustify-docs");
    let output = |args: &[&str]| -> Result<(bool, String), String> {
        let output = Command::new(&binary)
            .args(args)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .output()
            .map_err(|error| format!("{}: {error}", binary.display()))?;
        Ok((
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
        ))
    };
    let (version_ok, version) = output(&["--version"])?;
    let (info_ok, info) = output(&["--build-info"])?;
    let (recovered, probe) = output(&["__panic-probe"])?;
    print!("{version}{info}");
    if !version_ok || !info_ok {
        return Err("the installed CLI did not report its version and build identity".into());
    }
    if !info.contains("panic: unwind") {
        return Err(format!("the installed binary does not unwind:\n{info}"));
    }
    if !recovered || probe.trim() != "recovered" {
        return Err("the installed binary did not recover from a panic".into());
    }
    println!("panic probe: recovered");
    let listed = cargo()
        .args(["install", "--list", "--root"])
        .arg(&install_root)
        .output()
        .map_err(|error| error.to_string())?;
    if !listed.status.success() {
        return Err(format!(
            "cargo install --list failed: {}",
            String::from_utf8_lossy(&listed.stderr)
        ));
    }
    let listed = String::from_utf8_lossy(&listed.stdout).into_owned();
    print!("{listed}");
    verify_cli_source(&info, &listed, &rev)?;
    println!("installed identity: --build-info source and Cargo Git source match {rev}");

    walkthrough(&sysroot, &binary, &work.join("author"))?;
    components_template(&sysroot, &binary, &work.join("components-author"))?;
    println!(
        "toolchain: {} installed from {rev} with the docs-cli profile ({} bytes, unwinds); \
         new, check, build, preview and dev ran with only Rust and the CLI on PATH, \
         single-language and French check/build with no network; nothing was compiled",
        version.trim(),
        std::fs::metadata(&binary)
            .map(|meta| meta.len())
            .unwrap_or(0)
    );
    Ok(())
}

fn verify_cli_source(info: &str, listed: &str, rev: &str) -> Result<(), String> {
    let sources: Vec<_> = info
        .lines()
        .filter_map(|line| line.strip_prefix("source: "))
        .collect();
    if sources != [rev] {
        return Err(format!(
            "the installed --build-info source is {sources:?}, expected the complete commit {rev}"
        ));
    }
    let packages: Vec<_> = listed
        .lines()
        .filter(|line| line.starts_with("rustify-docs-cli v"))
        .collect();
    let [package] = packages.as_slice() else {
        return Err("cargo install --list must name exactly one rustify-docs-cli package".into());
    };
    let source = package
        .split_once(" (")
        .and_then(|(_, source)| source.strip_suffix("):"));
    let revision_query = format!("rev={rev}");
    let matched = source
        .and_then(|source| source.rsplit_once('#'))
        .is_some_and(|(source, short)| {
            short.len() >= 7
                && rev.starts_with(short)
                && source
                    .split_once('?')
                    .is_some_and(|(_, query)| query.split('&').any(|entry| entry == revision_query))
        });
    if !matched {
        return Err(format!(
            "cargo install --list Git revision does not match {rev}: {package}"
        ));
    }
    Ok(())
}

fn quoted_assignment<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    text.split_once(&format!("{field} = \""))?
        .1
        .split_once('"')
        .map(|(value, _)| value)
}

fn cli_framework(info: &str) -> Result<Option<serde_json::Value>, String> {
    let records: Vec<_> = info
        .lines()
        .filter_map(|line| line.strip_prefix("framework: "))
        .collect();
    let distributions: Vec<_> = info
        .lines()
        .filter_map(|line| line.strip_prefix("distribution: "))
        .collect();
    if records.is_empty() && distributions.is_empty() {
        return Ok(None);
    }
    if records.len() != 1
        || distributions.len() != 1
        || !matches!(distributions[0], "source" | "registry")
    {
        return Err("CLI framework/distribution identity is incomplete".into());
    }
    let record: serde_json::Value =
        serde_json::from_str(records[0]).map_err(|error| error.to_string())?;
    if record["schema"] != 1
        || record["registry"] != "crates-io"
        || record["packages"]
            .as_object()
            .is_none_or(|packages| packages.len() != 3)
        || ["rustify-docs", "rustify-dom", "rustify-components"]
            .iter()
            .any(|name| record["packages"][name].as_str().is_none_or(str::is_empty))
        || record["wasm_bindgen"] != WASM_BINDGEN
    {
        return Err("CLI has an invalid framework release record".into());
    }
    Ok(Some(record))
}

fn verify_registry_components(
    manifest: &str,
    lock: &str,
    record: &serde_json::Value,
) -> Result<(), String> {
    for package in ["rustify-docs", "rustify-dom", "rustify-components"] {
        let version = record["packages"][package]
            .as_str()
            .ok_or("framework version missing")?;
        let declarations: Vec<_> = manifest
            .lines()
            .filter(|line| line.starts_with(&format!("{package} = {{")))
            .collect();
        let [declaration] = declarations.as_slice() else {
            return Err(format!("template must declare {package} once"));
        };
        if quoted_assignment(declaration, "version") != Some(format!("={version}").as_str())
            || quoted_assignment(declaration, "path").is_some()
            || quoted_assignment(declaration, "git").is_some()
        {
            return Err(format!(
                "template must declare {package}@={version} from crates.io"
            ));
        }
        let entries: Vec<_> = lock
            .split("[[package]]")
            .filter(|entry| quoted_assignment(entry, "name") == Some(package))
            .collect();
        let [entry] = entries.as_slice() else {
            return Err(format!("template lock must contain {package} once"));
        };
        if quoted_assignment(entry, "version") != Some(version)
            || quoted_assignment(entry, "source")
                != Some("registry+https://github.com/rust-lang/crates.io-index")
            || quoted_assignment(entry, "checksum").is_none_or(|sum| {
                sum.len() != 64 || !sum.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
        {
            return Err(format!(
                "template lock does not match {package}@{version} and its registry checksum"
            ));
        }
    }
    Ok(())
}

fn components_template(sysroot: &Path, binary: &Path, dir: &Path) -> Result<(), String> {
    let author = Author::new(sysroot, binary, dir)?;
    author.check_path()?;
    author.run(OFFLINE, dir, &["new", "site", "--template", "components"])?;
    let site = dir.join("site");
    let manifest =
        std::fs::read_to_string(site.join("Cargo.toml")).map_err(|error| error.to_string())?;
    let lock = std::fs::read(site.join("Cargo.lock")).map_err(|error| error.to_string())?;
    let lock_text = std::str::from_utf8(&lock).map_err(|error| error.to_string())?;
    let output = Command::new(binary)
        .arg("--build-info")
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("installed CLI build-info failed".into());
    }
    let info = String::from_utf8_lossy(&output.stdout);
    let record = cli_framework(&info)?;
    let mut framework = None;
    if let Some(record) = &record {
        verify_registry_components(&manifest, lock_text, record)?;
    } else {
        for package in ["rustify-docs", "rustify-ui", "rustify-components"] {
            let declarations: Vec<_> = manifest
                .lines()
                .filter(|line| line.starts_with(&format!("{package} = {{")))
                .collect();
            let [declaration] = declarations.as_slice() else {
                return Err(format!(
                    "the installed components template must declare {package} once"
                ));
            };
            let revision = quoted_assignment(declaration, "rev")
                .ok_or_else(|| format!("the generated {package} has no Git revision"))?;
            let git = quoted_assignment(declaration, "git")
                .ok_or_else(|| format!("the generated {package} has no Git URL"))?;
            if revision.len() != 40
                || !revision.bytes().all(|byte| byte.is_ascii_hexdigit())
                || framework.is_some_and(|previous| previous != revision)
            {
                return Err("the generated components dependencies do not share one complete framework revision".into());
            }
            framework = Some(revision);
            let entries: Vec<_> = lock_text
                .split("[[package]]")
                .filter(|entry| quoted_assignment(entry, "name") == Some(package))
                .collect();
            let [entry] = entries.as_slice() else {
                return Err(format!("the generated lock must contain {package} once"));
            };
            let expected = format!("git+{git}?rev={revision}#{revision}");
            if quoted_assignment(entry, "source") != Some(expected.as_str()) {
                return Err(format!("the generated {package} lock source does not match its generated Git revision {revision}"));
            }
        }
    }
    let calls = author.calls()?;
    if calls
        != [
            "rustc --version",
            "rustc --print target-libdir --target wasm32-unknown-unknown",
        ]
    {
        return Err(format!(
            "components template creation unexpectedly invoked tools: {calls:?}"
        ));
    }
    if std::fs::read(site.join("Cargo.lock")).map_err(|error| error.to_string())? != lock {
        return Err(
            "the generated components lock changed during the template identity check".into(),
        );
    }
    let pair = record
        .as_ref()
        .and_then(|record| record["library_rev"].as_str())
        .or(framework)
        .unwrap_or_default();
    println!("components template (no network, no build): all three dependencies and the unchanged lock match framework {pair}; lock retained at {} ({} bytes)", site.join("Cargo.lock").display(), lock.len());
    Ok(())
}

/// The machine the check ran on, for the record: the CLI is promised on
/// macOS alone, so the system's version and architecture are part of the
/// evidence, and so is the Rust that built and checks it.
fn environment(sysroot: &Path) -> Result<(), String> {
    let text = |command: &mut Command| -> Result<String, String> {
        let what = command.get_program().to_string_lossy().into_owned();
        let output = command
            .output()
            .map_err(|error| format!("{what}: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "{what}: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    };
    let system = |program: &str, args: &[&str]| {
        let mut command = Command::new(program);
        command.args(args).env_clear().env("PATH", "/usr/bin:/bin");
        command
    };
    let version = text(&mut system("/usr/bin/sw_vers", &["-productVersion"]))?;
    let build = text(&mut system("/usr/bin/sw_vers", &["-buildVersion"]))?;
    let arch = text(&mut system("/usr/bin/uname", &["-m"]))?;
    let mut rustc = Command::new(sysroot.join("bin/rustc"));
    rustc.arg("-vV").env_clear().env("PATH", "/usr/bin:/bin");
    let rustc = text(&mut rustc)?;
    let cargo = text(plain_cargo(sysroot).arg("-V"))?;
    println!("environment: macOS {version} ({build}) on {arch}");
    for line in rustc.lines() {
        println!("  {line}");
    }
    println!("  {cargo}");
    Ok(())
}

/// Quotes `path` for a POSIX shell.
fn shell_quoted(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "'\\''"))
}

/// An author's machine with Rust and the CLI on it and nothing else.
struct Author {
    binary: PathBuf,
    dir: PathBuf,
    home: PathBuf,
    tmp: PathBuf,
    log: PathBuf,
    path: String,
}

impl Author {
    /// Lays the machine out in `dir`: an empty home, a temporary directory,
    /// and stand-ins for `rustc` and `cargo` that log each call and run the
    /// real program, so that whatever the CLI runs is both answered and seen.
    fn new(sysroot: &Path, binary: &Path, dir: &Path) -> Result<Self, String> {
        use std::os::unix::fs::PermissionsExt;
        if dir.exists() {
            std::fs::remove_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
        }
        let home = dir.join("home");
        let tmp = dir.join("tmp");
        let tools = dir.join("tools");
        let log = dir.join("tools.log");
        for created in [&home, &tmp, &tools] {
            std::fs::create_dir_all(created)
                .map_err(|error| format!("{}: {error}", created.display()))?;
        }
        std::fs::write(&log, "").map_err(|error| error.to_string())?;
        for tool in ["rustc", "cargo"] {
            let script = format!(
                "#!/bin/sh\nprintf '%s\\n' \"{tool} $*\" >> {}\nexec {} \"$@\"\n",
                shell_quoted(&log),
                shell_quoted(&sysroot.join("bin").join(tool))
            );
            let stand_in = tools.join(tool);
            std::fs::write(&stand_in, script).map_err(|error| error.to_string())?;
            std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
                .map_err(|error| error.to_string())?;
        }
        let installed = binary
            .parent()
            .ok_or_else(|| format!("{} has no directory", binary.display()))?;
        Ok(Self {
            binary: binary.to_path_buf(),
            dir: dir.to_path_buf(),
            home,
            tmp,
            log,
            path: format!("{}:{}:/usr/bin:/bin", tools.display(), installed.display()),
        })
    }

    /// Fails if any PATH directory offers a program an author must not need.
    fn check_path(&self) -> Result<(), String> {
        let found: Vec<String> = self
            .path
            .split(':')
            .flat_map(|dir| NOT_NEEDED.iter().map(move |tool| Path::new(dir).join(tool)))
            .filter(|path| path.symlink_metadata().is_ok())
            .map(|path| path.display().to_string())
            .collect();
        if found.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "the author's PATH offers what an author must not need: {}",
                found.join(", ")
            ))
        }
    }

    /// The installed CLI under `profile`, with only the author's variables:
    /// nothing of this process's environment, no proxy among it, gets in.
    fn command(&self, profile: &str, cwd: &Path, args: &[&str]) -> Command {
        let mut command = Command::new(SANDBOX);
        command
            .args(["-p", profile])
            .arg(&self.binary)
            .args(args)
            .current_dir(cwd)
            .env_clear()
            .env("PATH", &self.path)
            .env("HOME", &self.home)
            .env("TMPDIR", &self.tmp);
        command
    }

    /// Runs a command that must succeed.
    fn run(
        &self,
        profile: &str,
        cwd: &Path,
        args: &[&str],
    ) -> Result<std::process::Output, String> {
        let output = self
            .command(profile, cwd, args)
            .output()
            .map_err(|error| format!("{SANDBOX}: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "`rustify-docs {}` failed ({}) in {}:\n{}{}",
                args.join(" "),
                output.status,
                cwd.display(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(output)
    }

    fn calls(&self) -> Result<Vec<String>, String> {
        Ok(std::fs::read_to_string(&self.log)
            .map_err(|error| format!("{}: {error}", self.log.display()))?
            .lines()
            .map(str::to_owned)
            .collect())
    }
}

/// The line of a command's report that starts with `prefix`, or its last.
fn conclusion(output: &std::process::Output, prefix: &str) -> String {
    let text = String::from_utf8_lossy(&output.stderr);
    text.lines()
        .find(|line| line.starts_with(prefix))
        .or_else(|| text.lines().rev().find(|line| !line.trim().is_empty()))
        .unwrap_or_default()
        .to_string()
}

/// An author's first session with the installed binary, on a machine that
/// has Rust and the CLI and nothing else (see `Author`). `new`, `check` and
/// `build` run with no network at all and must succeed; `check
/// --external-links` must call every external address unreachable rather
/// than working; the servers, with loopback only, must serve the site; and
/// the only toolchain call is the `rustc --version` check, since a content
/// site compiles nothing. Leaves nothing behind when it passes; when it does
/// not, `dir` stays for a look.
fn walkthrough(sysroot: &Path, binary: &Path, dir: &Path) -> Result<(), String> {
    let author = Author::new(sysroot, binary, dir)?;
    author.check_path()?;
    println!(
        "author: empty HOME, PATH={}, no other variable; none of {} on PATH",
        author.path,
        NOT_NEEDED.join(", ")
    );

    let created = author.run(OFFLINE, dir, &["new", "site"])?;
    let site = dir.join("site");
    println!(
        "new (no network): {}; toolchain check: {}",
        String::from_utf8_lossy(&created.stdout)
            .lines()
            .next()
            .unwrap_or_default(),
        conclusion(&created, "rustc ")
    );
    let checked = author.run(OFFLINE, &site, &["check"])?;
    println!("check (no network): {}", conclusion(&checked, "check: "));
    let built = author.run(OFFLINE, &site, &["build"])?;
    println!("build (no network): {}", conclusion(&built, "build: "));
    published(&site.join("dist"))?;

    let (addresses, _) = offline_link_check(&author, &site)?;
    println!(
        "check --external-links (no network), the site as created: external_addresses: {addresses}; {}",
        if addresses == 0 {
            "exit 0, nothing to request"
        } else {
            "each an error, exit 1"
        }
    );
    let page = site.join("content/guide/writing-pages.md");
    let mut text = std::fs::read_to_string(&page).map_err(|error| error.to_string())?;
    text.push_str(&format!("\nOffline link probe: [Rust]({PROBE_LINK}).\n"));
    std::fs::write(&page, text).map_err(|error| error.to_string())?;
    let (addresses, unreachable) = offline_link_check(&author, &site)?;
    let probe = unreachable
        .iter()
        .find(|(url, _)| url == PROBE_LINK)
        .ok_or_else(|| format!("check --external-links did not find {PROBE_LINK}"))?;
    println!(
        "check --external-links (no network), a link to {PROBE_LINK} added: \
         external_addresses: {addresses}, each an error; exit 1: {}",
        probe.1
    );

    println!(
        "preview (loopback only): {}",
        serve_and_fetch(
            &author,
            &site,
            &["preview", "--port", "0"],
            "preview",
            &[("", 200, &[]), ("guide/getting-started/", 200, &[])]
        )?
    );
    println!(
        "dev (loopback only): {}",
        serve_and_fetch(
            &author,
            &site,
            &["dev", "--port", "0"],
            "dev",
            &[("", 200, &[]), ("guide/getting-started/", 200, &[])]
        )?
    );

    third_language_walkthrough(&author, &site)?;

    let calls = author.calls()?;
    if calls.is_empty() {
        return Err(
            "the toolchain stand-ins logged nothing: `new` and `dev` did not find rustc on PATH"
                .into(),
        );
    }
    if calls.iter().any(|call| call != "rustc --version") {
        return Err(format!(
            "a content site must run nothing but `rustc --version`, and the toolchain stand-ins logged:\n  {}",
            calls.join("\n  ")
        ));
    }
    println!(
        "toolchain calls: {} x `rustc --version` (new and dev check for Rust); no cargo, nothing compiled",
        calls.len()
    );
    std::fs::remove_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    Ok(())
}

fn third_language_walkthrough(author: &Author, site: &Path) -> Result<(), String> {
    let config_file = site.join("rustify-docs.toml");
    let mut config = std::fs::read_to_string(&config_file).map_err(|error| error.to_string())?;
    config.push_str("\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\nmessages='locales/fr.toml'\n");
    std::fs::write(&config_file, config).map_err(|error| error.to_string())?;
    for directory in ["content/fr", "locales"] {
        std::fs::create_dir_all(site.join(directory)).map_err(|error| error.to_string())?;
    }
    std::fs::write(site.join("content/fr/index.md"), "---\ntitle: Accueil français\ndescription: Une vraie traduction.\nlang: fr\n---\n# Accueil français\n\nUne page réellement traduite.\n\n[Commencer](../guide/getting-started.md)\n").map_err(|error| error.to_string())?;
    let messages_file = site.join("locales/fr.toml");
    let messages = "language='Langue'\nuntranslated='Traduction indisponible.'\nnot_found_title='Page introuvable'\nnot_found_body='Aucune page à cette adresse.'\nback_home='Retour à l’accueil'\ncopy='Copier'\n";
    std::fs::write(&messages_file, messages).map_err(|error| error.to_string())?;
    let checked = author.run(OFFLINE, site, &["check", "--format", "json"])?;
    let report: serde_json::Value = serde_json::from_slice(&checked.stdout)
        .map_err(|error| format!("French check JSON: {error}"))?;
    let diagnostics = report["diagnostics"]
        .as_array()
        .ok_or("French check has no diagnostics array")?;
    if report["success"] != true || report["summary"]["errors"] != 0 {
        return Err(format!("French check failed offline: {report}"));
    }
    for rule in ["i18n-missing-page", "i18n-missing-label"] {
        if !diagnostics.iter().any(|issue| {
            issue["rule"] == rule
                && issue["severity"] == "warning"
                && issue["message"]
                    .as_str()
                    .is_some_and(|text| text.contains("`fr`"))
        }) {
            return Err(format!("French check did not report {rule}: {report}"));
        }
    }
    let built = author.run(OFFLINE, site, &["build"])?;
    println!("French check/build (no network): 0 errors with missing-page and missing-label warnings; {}", conclusion(&built, "build: "));
    let dist = site.join("dist");
    let pages: [(&str, &str, u16, &[&str]); 3] = [
        (
            "fr/",
            "fr/index.html",
            200,
            &[
                "<html lang=\"fr\"",
                "<article lang=\"fr\"",
                "Une page réellement traduite.",
            ],
        ),
        (
            "fr/guide/getting-started/",
            "fr/guide/getting-started/index.html",
            200,
            &[
                "<html lang=\"fr\"",
                "<article lang=\"en\"",
                "Traduction indisponible.",
                "data-rd-copy hidden lang=\"fr\"",
                ">Copier</span>",
            ],
        ),
        (
            "fr/missing/deep/",
            "fr/404.html",
            404,
            &[
                "<html lang=\"fr\"",
                "Page introuvable",
                "Aucune page à cette adresse.",
                "href=\"/fr/\"",
            ],
        ),
    ];
    for (_, file, _, markers) in &pages {
        let html =
            std::fs::read_to_string(dist.join(file)).map_err(|error| format!("{file}: {error}"))?;
        if let Some(missing) = markers.iter().find(|marker| !html.contains(**marker)) {
            return Err(format!("French output {file} is missing {missing:?}"));
        }
    }
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(dist.join(MANIFEST)).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    if manifest["locale_not_found"] != serde_json::json!({"": "404.html", "fr": "fr/404.html"}) {
        return Err(format!(
            "French locale_not_found is not the generated language map: {}",
            manifest["locale_not_found"]
        ));
    }
    let before = crate::sources::fingerprint_tree(&dist)?;
    if before
        .keys()
        .any(|file| file.ends_with(".toml") || file.ends_with(".wasm"))
    {
        return Err("the French content site published a configuration input or wasm".into());
    }
    std::fs::write(&messages_file, "copy = [\n").map_err(|error| error.to_string())?;
    let failed = author
        .command(OFFLINE, site, &["build", "--format", "json"])
        .output()
        .map_err(|error| error.to_string())?;
    let failure: serde_json::Value = serde_json::from_slice(&failed.stdout)
        .map_err(|error| format!("invalid French messages build JSON: {error}"))?;
    if failed.status.code() != Some(1)
        || failure["success"] != false
        || !failure["diagnostics"].as_array().is_some_and(|items| {
            items
                .iter()
                .any(|issue| issue["rule"] == "i18n-messages" && issue["file"] == "locales/fr.toml")
        })
    {
        return Err(format!(
            "invalid French messages did not reject the candidate: {failure}"
        ));
    }
    if crate::sources::fingerprint_tree(&dist)? != before {
        return Err("a failed French build changed the last good dist".into());
    }
    let requests: Vec<_> = pages
        .iter()
        .map(|(route, _, status, markers)| (*route, *status, *markers))
        .collect();
    println!(
        "French preview (loopback only, invalid source keeps dist): {}",
        serve_and_fetch(
            author,
            site,
            &["preview", "--port", "0"],
            "preview",
            &requests
        )?
    );
    std::fs::write(&messages_file, messages).map_err(|error| error.to_string())?;
    author.run(OFFLINE, site, &["build"])?;
    if crate::sources::fingerprint_tree(&dist)? != before {
        return Err("restoring French messages did not reproduce the last good dist".into());
    }
    println!(
        "French dev (loopback only, restored messages): {}",
        serve_and_fetch(author, site, &["dev", "--port", "0"], "dev", &requests)?
    );
    println!("French output: real translation, English fallback, per-word language, locale_not_found, input-only messages and unchanged dist on failure; restored build is identical");
    Ok(())
}

/// The production output a build published: its pages, and a manifest that
/// says it is one.
fn published(dist: &Path) -> Result<(), String> {
    for page in ["index.html", "guide/getting-started/index.html", "404.html"] {
        if !dist.join(page).is_file() {
            return Err(format!(
                "the build published no {page} in {}",
                dist.display()
            ));
        }
    }
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(dist.join(MANIFEST))
            .map_err(|error| format!("{}: {error}", dist.join(MANIFEST).display()))?,
    )
    .map_err(|error| format!("{MANIFEST}: {error}"))?;
    if manifest["mode"] != "production" {
        return Err(format!(
            "{MANIFEST} has mode {}, not production",
            manifest["mode"]
        ));
    }
    println!(
        "published: index.html, guide/getting-started/index.html, 404.html; {MANIFEST} mode production, {} files",
        manifest["files"].as_array().map_or(0, Vec::len)
    );
    Ok(())
}

/// `check --external-links --format json` with no network. Every external
/// address it finds must come back as `external-link-unreachable`, an
/// error, and none as answering; with any found the command fails, with
/// none it succeeds. Returns how many it found and, for each one reported,
/// the address and the diagnostic's message.
fn offline_link_check(
    author: &Author,
    site: &Path,
) -> Result<(u64, Vec<(String, String)>), String> {
    let args = ["check", "--external-links", "--format", "json"];
    let output = author
        .command(OFFLINE, site, &args)
        .output()
        .map_err(|error| format!("{SANDBOX}: {error}"))?;
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "`rustify-docs {}` printed no JSON object ({error}):\n{}{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })?;
    let addresses = report["summary"]["external_addresses"]
        .as_u64()
        .ok_or("check --external-links reported no summary.external_addresses")?;
    let mut unreachable: Vec<(String, String)> = Vec::new();
    for diagnostic in report["diagnostics"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let rule = diagnostic["rule"].as_str().unwrap_or_default();
        if !rule.starts_with("external-link") {
            continue;
        }
        let message = diagnostic["message"].as_str().unwrap_or_default();
        if rule != "external-link-unreachable" || diagnostic["severity"] != "error" {
            return Err(format!(
                "with no network, check --external-links reported an answer: {} {rule}: {message}",
                diagnostic["severity"]
            ));
        }
        let url = message.split('`').nth(1).unwrap_or(message).to_string();
        if !unreachable.iter().any(|(seen, _)| *seen == url) {
            unreachable.push((url, message.to_string()));
        }
    }
    if unreachable.len() as u64 != addresses {
        return Err(format!(
            "check --external-links found {addresses} external address(es) but called {} unreachable: {:?}",
            unreachable.len(),
            unreachable
        ));
    }
    let succeeded = output.status.success() && report["success"] == true;
    let failed = !output.status.success() && report["success"] == false;
    if addresses > 0 && !failed {
        return Err(format!(
            "with no network and {addresses} external address(es), check --external-links did not fail ({})",
            output.status
        ));
    }
    if addresses == 0 && !succeeded {
        return Err(format!(
            "with no external address, check --external-links failed ({}):\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    Ok((addresses, unreachable))
}

/// A server that is stopped however the walkthrough leaves it.
struct Server(std::process::Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Starts `rustify-docs <args>` (a server) in `site` with loopback only,
/// reads the address it prints first, fetches the expected pages from it,
/// and stops it. Returns what was fetched, for the record.
fn serve_and_fetch(
    author: &Author,
    site: &Path,
    args: &[&str],
    label: &str,
    pages: &[(&str, u16, &[&str])],
) -> Result<String, String> {
    use std::io::BufRead;
    let what = format!("`rustify-docs {}`", args.join(" "));
    // Its progress goes to a file: a pipe nobody reads could fill and stall it.
    let errors = author.dir.join(format!("{label}.stderr"));
    let errors_file = std::fs::File::create(&errors).map_err(|error| error.to_string())?;
    let child = author
        .command(LOOPBACK, site, args)
        .stdout(std::process::Stdio::piped())
        .stderr(errors_file)
        .spawn()
        .map_err(|error| format!("{SANDBOX}: {error}"))?;
    let mut server = Server(child);
    let stderr = || std::fs::read_to_string(&errors).unwrap_or_default();
    let stdout = server.0.stdout.take().ok_or("the server has no stdout")?;
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let read = std::io::BufReader::new(stdout).read_line(&mut line);
        let _ = sender.send(read.map(|_| line));
    });
    let line = match receiver.recv_timeout(SERVER_START) {
        Ok(Ok(line)) if !line.trim().is_empty() => line.trim().to_string(),
        Ok(Ok(_)) => {
            return Err(format!(
                "{what} ended without printing its address:\n{}",
                stderr()
            ))
        }
        Ok(Err(error)) => return Err(format!("{what}: {error}")),
        Err(_) => {
            return Err(format!(
                "{what} printed no address within {} s:\n{}",
                SERVER_START.as_secs(),
                stderr()
            ))
        }
    };
    let prefix = format!("{label}: http://");
    let rest = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("{what} printed `{line}`, not `{prefix}…`"))?;
    let (authority, base) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let base = if base.is_empty() { "/" } else { base };
    let mut fetched = Vec::new();
    for (page, expected_status, markers) in pages {
        let path = format!("{base}{page}");
        let deadline = std::time::Instant::now() + SERVER_START;
        loop {
            let (status, bytes) = http_get(authority, &path)?;
            let html = String::from_utf8_lossy(&bytes);
            if status == *expected_status && markers.iter().all(|marker| html.contains(*marker)) {
                fetched.push(format!("GET {path} {status} ({} bytes)", bytes.len()));
                break;
            }
            if std::time::Instant::now() >= deadline {
                return Err(format!("{what}: GET {path} did not become {expected_status} with {markers:?}; answered {status}:\n{}", stderr()));
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
    drop(server);
    Ok(format!("{line}; {}; stopped", fetched.join(", ")))
}

/// One GET over HTTP/1.1 that closes the connection. Returns the status and
/// the body.
fn http_get(authority: &str, path: &str) -> Result<(u16, Vec<u8>), String> {
    use std::io::{Read, Write};
    let what = format!("GET http://{authority}{path}");
    let address: std::net::SocketAddr = authority
        .parse()
        .map_err(|error| format!("{what}: {error}"))?;
    let mut stream = std::net::TcpStream::connect_timeout(&address, HTTP_TIMEOUT)
        .map_err(|error| format!("{what}: {error}"))?;
    stream
        .set_read_timeout(Some(HTTP_TIMEOUT))
        .and_then(|()| stream.set_write_timeout(Some(HTTP_TIMEOUT)))
        .map_err(|error| format!("{what}: {error}"))?;
    stream
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .map_err(|error| format!("{what}: {error}"))?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("{what}: {error}"))?;
    let status = String::from_utf8_lossy(&response)
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| format!("{what}: no HTTP status line"))?;
    let body = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|at| response[at + 4..].to_vec())
        .ok_or_else(|| format!("{what}: no HTTP header terminator"))?;
    Ok((status, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_pair_modes_keep_legacy_and_validate_new_registry_templates() {
        assert!(cli_framework("source: old\n").unwrap().is_none());
        let record = serde_json::json!({"schema":1, "registry":"crates-io", "library_rev":"a".repeat(40),
            "packages":{"rustify-docs":"0.2.0", "rustify-dom":"0.2.0", "rustify-components":"0.2.0"}, "wasm_bindgen":WASM_BINDGEN});
        for mode in ["registry", "source"] {
            let info = format!("distribution: {mode}\nframework: {record}\n");
            assert_eq!(cli_framework(&info).unwrap().unwrap(), record);
        }
        assert!(cli_framework(&format!("framework: {record}\n")).is_err());
        assert!(cli_framework("distribution: registry\nframework: broken\n").is_err());
        let mut manifest = String::new();
        let mut lock = String::new();
        for name in ["rustify-docs", "rustify-dom", "rustify-components"] {
            manifest.push_str(&format!(
                "{name} = {{ version = \"=0.2.0\", default-features = false }}\n"
            ));
            lock.push_str(&format!("[[package]]\nname = \"{name}\"\nversion = \"0.2.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"{}\"\n", "a".repeat(64)));
        }
        assert!(verify_registry_components(&manifest, &lock, &record).is_ok());
        assert!(
            verify_registry_components(&manifest.replace("=0.2.0", "=0.1.0"), &lock, &record)
                .is_err()
        );
        assert!(
            verify_registry_components(&manifest, &lock.replace("registry+", "git+"), &record)
                .is_err()
        );
    }

    #[test]
    fn toolchain_identity_matches_the_complete_build_source_and_cargo_revision() {
        let revision = "b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d";
        let info = format!("rustify-docs 0.1.19\npanic: unwind\nsource: {revision}\n");
        let listed = format!("rustify-docs-cli v0.1.19 (file:///tmp/source?rev={revision}#b31e28b7):\n    rustify-docs\n");
        assert!(verify_cli_source(&info, &listed, revision).is_ok());
        for source in [
            "unknown",
            "b31e28b7",
            "b31e28b7f7dd949f7dc89ebc7b5f64380adbb63e",
        ] {
            assert!(verify_cli_source(&format!("source: {source}\n"), &listed, revision).is_err());
        }
    }

    #[test]
    fn toolchain_identity_rejects_unrelated_packages_and_inconsistent_cargo_hashes() {
        let revision = "b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d";
        let info = format!("source: {revision}\n");
        for listed in [
            format!("other-package v1.0.0 (file:///tmp/source?rev={revision}#b31e28b7):"),
            format!("rustify-docs-cli v0.1.19 (file:///tmp/source?rev={revision}#12345678):"),
            format!("rustify-docs-cli v0.1.19 (file:///tmp/source?rev=b31e28b7#b31e28b7):\n    {revision}"),
        ] {
            assert!(verify_cli_source(&info, &listed, revision).is_err(), "{listed}");
        }
    }

    #[test]
    fn makepad_packages_are_named_by_the_rule_and_library_copies_are_not() {
        for name in [
            "makepad-widgets",
            "makepad-platform",
            "rustify-makepad",
            "cargo-makepad",
        ] {
            assert!(is_makepad(name), "{name}");
        }
        for name in ["bitflags", "smallvec", "rustify-ui", "leptos"] {
            assert!(!is_makepad(name), "{name}");
        }
    }

    #[test]
    fn a_graph_with_makepad_in_it_is_refused() {
        let lines = vec![
            "leptos v0.8.22".to_string(),
            "makepad-widgets v1.0.0 (/repo/makepad/widgets)".to_string(),
        ];
        assert!(check_graph("probe", &lines).is_err());
        let lines = vec![
            "leptos v0.8.22".to_string(),
            "smallvec v1.13.2 (/repo/makepad/libs/smallvec)".to_string(),
        ];
        assert!(check_graph("probe", &lines).is_ok());
    }

    fn site_graph(mode: &str) -> Vec<String> {
        let docs = if mode == "ssr" { "site-ssr" } else { "hydrate" };
        vec![
            format!("rustify-docs v0.1.0 (/project with spaces/docs)\t{docs}"),
            format!("rustify-ui v0.1.0\t{mode}"),
            format!("rustify-components v0.1.0\t{mode}"),
            format!("leptos v0.8.22\t{mode}"),
            "toml v1.1.0\tparse".to_string(),
            "rustify-style v0.1.0\tcn,protocol".to_string(),
        ]
    }

    #[test]
    fn site_graph_allows_leptos_configuration_and_runtime_class_merging() {
        assert!(check_site_graph("host", "ssr", &site_graph("ssr")).is_ok());
        assert!(check_site_graph("browser", "hydrate", &site_graph("hydrate")).is_ok());
    }

    #[test]
    fn site_graph_rejects_the_private_pipeline_in_either_rendering_mode() {
        for mode in ["ssr", "hydrate"] {
            for poison in [
                "syntect v5.3.0\tparsing",
                "rustify-docs v0.1.0\tpipeline",
                "rustify-style v0.1.0\tcn,scanner",
                "makepad-widgets v1.0.0\tdefault",
            ] {
                let mut lines = site_graph(mode);
                lines.push(poison.to_string());
                assert!(check_site_graph(mode, mode, &lines).is_err(), "{poison}");
            }
        }
    }

    #[test]
    fn host_site_graph_rejects_browser_and_effect_features_on_dependencies() {
        for poison in [
            "rustify-ui v0.1.0\thydrate",
            "rustify-components v0.1.0\tcsr",
            "reactive_graph v0.2.13\teffects",
            "rustify-ui v0.1.0\tgpu",
        ] {
            let mut lines = site_graph("ssr");
            lines.push(poison.to_string());
            assert!(check_site_graph("host", "ssr", &lines).is_err(), "{poison}");
        }
    }

    #[test]
    fn an_island_group_refuses_a_makepad_import_inside_its_runtime_snippet() {
        let directory = std::env::temp_dir().join(format!(
            "rustify-island-product-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let snippet = directory.join("snippets/rustify-dom-fixture/web/dom-runtime.js");
        std::fs::create_dir_all(snippet.parent().unwrap()).unwrap();
        std::fs::write(&snippet, "export function create_dom_host() {}\n").unwrap();
        std::fs::write(
            directory.join("app.js"),
            "import { create_dom_host } from './snippets/rustify-dom-fixture/web/dom-runtime.js';\n",
        )
        .unwrap();
        std::fs::write(directory.join("app_bg.wasm"), b"\0asm\x01\0\0\0").unwrap();
        let accepted = check_product_at(&directory, &directory);
        std::fs::write(&snippet, "import { draw } from 'makepad-runtime';\n").unwrap();
        let rejected = check_product_at(&directory, &directory);
        std::fs::remove_dir_all(directory).unwrap();
        assert!(accepted.is_ok(), "{accepted:?}");
        assert!(rejected.is_err());
    }

    #[test]
    fn import_modules_are_read_from_the_import_section() {
        // (module (import "wbg" "f" (func)) (import "env" "m" (memory 1)))
        let wasm = [
            0, b'a', b's', b'm', 1, 0, 0, 0, // header
            1, 4, 1, 0x60, 0, 0, // type section: one () -> ()
            2, 18, 2, // import section, two entries
            3, b'w', b'b', b'g', 1, b'f', 0, 0, // wbg.f: func type 0
            3, b'e', b'n', b'v', 1, b'm', 2, 0, 1, // env.m: memory min 1
        ];
        let modules = wasm_import_modules(&wasm).unwrap();
        assert_eq!(
            modules.into_iter().collect::<Vec<_>>(),
            ["env".to_string(), "wbg".to_string()]
        );
    }

    #[test]
    fn stand_in_paths_are_quoted_for_the_shell() {
        assert_eq!(shell_quoted(Path::new("/tmp/a b")), "'/tmp/a b'");
        assert_eq!(shell_quoted(Path::new("/tmp/it's")), "'/tmp/it'\\''s'");
    }
}
