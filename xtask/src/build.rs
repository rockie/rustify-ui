//! `cargo xtask build-web`: the complete static build of one example.
//!
//! cargo-makepad produces the wasm, the bindgen glue and Makepad's own JS and
//! resources. This step adds everything the runtime needs on top: the static
//! message bridge, the embedded host, the loader and the example page, then
//! records what was built in a manifest.

use crate::bridge::{extract_bridge, render_bridge_module};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct BuildRequest {
    pub example: String,
    pub release: bool,
}

#[derive(Serialize)]
pub struct BuildManifest {
    pub example: String,
    pub profile: String,
    pub build_id: String,
    pub schema_hash: String,
    pub toolchain: String,
    pub files: BTreeMap<String, u64>,
    pub size_report: SizeReport,
}

#[derive(Serialize, Default, Debug, PartialEq, Eq)]
pub struct SizeReport {
    pub wasm: u64,
    pub js: u64,
    pub css: u64,
    pub fonts: u64,
    pub images: u64,
    pub data: u64,
    pub total: u64,
}

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one level below the repository root")
        .to_path_buf()
}

pub fn app_dir(root: &Path, request: &BuildRequest) -> PathBuf {
    root.join("target")
        .join("makepad-wasm-app")
        .join(profile(request))
        .join(&request.example)
}

fn profile(request: &BuildRequest) -> &'static str {
    if request.release {
        "release"
    } else {
        "debug"
    }
}

pub fn build(request: &BuildRequest) -> Result<PathBuf, String> {
    let root = repo_root();
    let example_dir = root.join("examples").join(&request.example);
    if !example_dir.join("Cargo.toml").is_file() {
        return Err(format!("no example at {}", example_dir.display()));
    }
    let app = app_dir(&root, request);
    if app.exists() {
        std::fs::remove_dir_all(&app)
            .map_err(|e| format!("cannot clear {}: {e}", app.display()))?;
    }
    run_cargo_makepad(&root, request)?;

    let wasm_path = app.join(format!("{}.wasm", request.example));
    let wasm = std::fs::read(&wasm_path).map_err(|e| format!("{}: {e}", wasm_path.display()))?;
    let bridge = extract_bridge(&wasm)?;
    let bridge_dir = app.join("rustify_makepad");
    std::fs::create_dir_all(&bridge_dir).map_err(|e| e.to_string())?;
    write(
        &bridge_dir.join("message_bridge.js"),
        render_bridge_module(&bridge.source, bridge.hash).as_bytes(),
    )?;
    copy(
        &root.join("crates/rustify-makepad/web/embedded.js"),
        &bridge_dir.join("embedded.js"),
    )?;
    copy(&root.join("web/loader.js"), &app.join("loader.js"))?;
    copy(&root.join("web/runtime.css"), &app.join("runtime.css"))?;
    for name in ["index.html", "app.js", "app.css"] {
        copy(&example_dir.join(name), &app.join(name))?;
    }
    // Third-party browser files are part of the checkout, at one recorded
    // version (`sources.lock.json`), so the build copies them rather than
    // fetching anything.
    let vendor = example_dir.join("vendor");
    if vendor.is_dir() {
        copy_tree(&vendor, &app.join("vendor"))?;
    }

    let files = list_files(&app)?;
    let manifest = BuildManifest {
        example: request.example.clone(),
        profile: profile(request).to_string(),
        build_id: build_id(&wasm),
        schema_hash: bridge.hash.to_string(),
        toolchain: toolchain_channel(&root)?,
        size_report: size_report(&files),
        files,
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    write(&app.join("build-manifest.json"), json.as_bytes())?;
    println!("built {} -> {}", request.example, app.display());
    println!("{}", format_size_report(&manifest.size_report));
    Ok(app)
}

fn run_cargo_makepad(root: &Path, request: &BuildRequest) -> Result<(), String> {
    let mut args = vec![
        "run",
        "--quiet",
        "--release",
        "--manifest-path",
        "makepad/tools/cargo_makepad/Cargo.toml",
        "--",
        "wasm",
        "build",
    ];
    if request.release {
        args.push("--release");
    }
    args.extend(["-p", request.example.as_str()]);
    let status = Command::new("cargo")
        .args(&args)
        .current_dir(root)
        .status()
        .map_err(|e| format!("cannot run cargo: {e}"))?;
    if !status.success() {
        return Err("cargo-makepad wasm build failed".to_string());
    }
    Ok(())
}

fn toolchain_channel(root: &Path) -> Result<String, String> {
    let text =
        std::fs::read_to_string(root.join("rust-toolchain.toml")).map_err(|e| e.to_string())?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("channel"))
        .and_then(|rest| rest.split('"').nth(1))
        .map(str::to_string)
        .ok_or_else(|| "rust-toolchain.toml has no channel".to_string())
}

fn build_id(wasm: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(wasm);
    digest.iter().take(8).map(|b| format!("{b:02x}")).collect()
}

fn list_files(dir: &Path) -> Result<BTreeMap<String, u64>, String> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in std::fs::read_dir(&current).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let relative = path
                    .strip_prefix(dir)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/");
                out.insert(relative, entry.metadata().map_err(|e| e.to_string())?.len());
            }
        }
    }
    Ok(out)
}

/// Splits raw bytes into the six categories the plan reports.
pub fn size_report(files: &BTreeMap<String, u64>) -> SizeReport {
    let mut report = SizeReport::default();
    for (name, size) in files {
        let ext = name.rsplit('.').next().unwrap_or("");
        let bucket = match ext {
            "wasm" => &mut report.wasm,
            // A module is JavaScript; the third-party file is shipped as one.
            "js" | "mjs" => &mut report.js,
            "css" => &mut report.css,
            "ttf" | "otf" | "woff" | "woff2" => &mut report.fonts,
            "png" | "jpg" | "jpeg" | "svg" | "webp" | "gif" => &mut report.images,
            _ => &mut report.data,
        };
        *bucket += size;
        report.total += size;
    }
    report
}

pub fn format_size_report(report: &SizeReport) -> String {
    format!(
        "size report (bytes): wasm {} | js {} | css {} | fonts {} | images {} | data {} | total {}",
        report.wasm, report.js, report.css, report.fonts, report.images, report.data, report.total
    )
}

/// Copies a directory as it stands. Used for the vendored files, which are
/// taken whole or not at all.
fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            copy_tree(&path, &target)?;
        } else {
            copy(&path, &target)?;
        }
    }
    Ok(())
}

fn copy(from: &Path, to: &Path) -> Result<(), String> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::copy(from, to)
        .map(|_| ())
        .map_err(|e| format!("copy {} -> {}: {e}", from.display(), to.display()))
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_bucketed_by_extension() {
        let mut files = BTreeMap::new();
        files.insert("a.wasm".to_string(), 10);
        files.insert("b.js".to_string(), 1);
        files.insert("c/d.js".to_string(), 2);
        files.insert("vendor/x.mjs".to_string(), 7);
        files.insert("e.css".to_string(), 3);
        files.insert("f.ttf".to_string(), 4);
        files.insert("g.png".to_string(), 5);
        files.insert("h.json".to_string(), 6);
        let report = size_report(&files);
        assert_eq!(
            report,
            SizeReport {
                wasm: 10,
                js: 10,
                css: 3,
                fonts: 4,
                images: 5,
                data: 6,
                total: 38
            }
        );
    }
}
