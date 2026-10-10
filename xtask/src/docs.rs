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
mbx xtask docs-verify --group features [--external [--git <url> --rev <sha>]]";

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
        _ => Err(USAGE.to_string()),
    }
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
    let output = cargo
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
            package,
            "--manifest-path",
        ])
        .arg(manifest)
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
    let glue = std::fs::read_to_string(output.join("pkg/app.js"))
        .map_err(|error| format!("pkg/app.js: {error}"))?;
    for line in glue
        .lines()
        .filter(|line| line.trim_start().starts_with("import "))
    {
        if line.contains("makepad") || !line.contains("./snippets/rustify-ui-") {
            return Err(format!(
                "the glue imports something other than the SDK runtime: {line}"
            ));
        }
    }
    let wasm = std::fs::read(output.join("pkg/app_bg.wasm"))
        .map_err(|error| format!("pkg/app_bg.wasm: {error}"))?;
    let modules = wasm_import_modules(&wasm)?;
    println!("  wasm imports from: {modules:?}");
    let unexpected: Vec<&String> = modules
        .iter()
        .filter(|module| {
            // The glue's own module (`./app_bg.js` from wasm-bindgen 0.2.128,
            // `wbg` before it) and the SDK's snippet; nothing else.
            module.as_str() != "wbg"
                && module.as_str() != "./app_bg.js"
                && !module.starts_with("./snippets/rustify-ui-")
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
    println!(
        "features: the default build is DOM only; the product is {}",
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
    check_graph(
        "dom-consumer (outside the repository, git dependencies, no patch)",
        &graph(&mut cargo(), &manifest, "dom-consumer")?,
    )?;

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
    check_product(&public)?;
    println!(
        "features (outside the repository): the default build is DOM only; the product is {}",
        public.display()
    );
    println!("serve it with: DOM_ONLY_DIR={} npx playwright test -c playwright.docs.config.ts --project=dom-only", public.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
