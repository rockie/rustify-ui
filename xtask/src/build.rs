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
    /// Where the product will be served from. `/` for the ordinary case.
    pub base: String,
}

#[derive(Serialize)]
pub struct BuildManifest {
    pub example: String,
    pub profile: String,
    /// The path this build's page expects to be served under. Written into the
    /// product because it is baked into the product: `index.html` names its
    /// files absolutely so that a deep link like `/tools/demo/objects/42` does
    /// not resolve them against `/tools/demo/objects/`.
    pub base: String,
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

/// Where a build goes.
///
/// A build for a sub-path is a different product - its page names its own
/// files absolutely - so it gets its own directory. Without that, serving the
/// same example at two bases would mean one directory and two contradictory
/// answers about what is in it.
pub fn app_dir(root: &Path, request: &BuildRequest) -> PathBuf {
    let base = crate::serve::normalize_base(&request.base);
    let name = if base == "/" {
        request.example.clone()
    } else {
        format!(
            "{}@{}",
            request.example,
            base.trim_matches('/').replace('/', "-")
        )
    };
    root.join("target")
        .join("makepad-wasm-app")
        .join(profile(request))
        .join(name)
}

/// Where cargo-makepad writes, which is a fixed path it does not take an
/// option for. It is also the product for the root, so a build for a sub-path
/// finishes this one too rather than leaving it half-made.
fn staging_dir(root: &Path, request: &BuildRequest) -> PathBuf {
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
    let app = staging_dir(&root, request);
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
    for name in ["app.js", "app.css"] {
        copy(&example_dir.join(name), &app.join(name))?;
    }
    let page = std::fs::read_to_string(example_dir.join("index.html"))
        .map_err(|e| format!("{}/index.html: {e}", example_dir.display()))?;
    write(&app.join("index.html"), page.as_bytes())?;
    // Third-party browser files are part of the checkout, at one recorded
    // version (`sources.lock.json`), so the build copies them rather than
    // fetching anything.
    let vendor = example_dir.join("vendor");
    if vendor.is_dir() {
        copy_tree(&vendor, &app.join("vendor"))?;
    }
    // The component stylesheet, for the examples that use the components. It
    // is committed rather than generated here, so this build needs no Node;
    // `cargo xtask css --check` is what keeps it in step with the classes.
    if uses_components(&example_dir)? {
        copy(
            &root.join("crates/rustify-components/css/rustify.css"),
            &app.join("rustify.css"),
        )?;
    }

    finish(&root, request, &app, "/", &bridge.hash.to_string(), &wasm)?;

    let base = crate::serve::normalize_base(&request.base);
    if base == "/" {
        return Ok(app);
    }
    // A sub-path build is a second product: the same files with a page that
    // names them absolutely, so a deep link does not resolve them against the
    // route it was opened at. The root product above stays valid, because
    // cargo-makepad's output directory is fixed and clearing it later would
    // leave whoever serves it with half a build.
    let rebased = app_dir(&root, request);
    if rebased.exists() {
        std::fs::remove_dir_all(&rebased)
            .map_err(|e| format!("cannot clear {}: {e}", rebased.display()))?;
    }
    copy_tree(&app, &rebased)?;
    write(
        &rebased.join("index.html"),
        rebase_index(&page, &base).as_bytes(),
    )?;
    finish(
        &root,
        request,
        &rebased,
        &base,
        &bridge.hash.to_string(),
        &wasm,
    )?;
    Ok(rebased)
}

/// Writes the manifest for a finished product and reports it.
fn finish(
    root: &Path,
    request: &BuildRequest,
    app: &Path,
    base: &str,
    schema_hash: &str,
    wasm: &[u8],
) -> Result<(), String> {
    let files = list_files(app)?;
    let manifest = BuildManifest {
        example: request.example.clone(),
        profile: profile(request).to_string(),
        base: base.to_string(),
        build_id: build_id(wasm),
        schema_hash: schema_hash.to_string(),
        toolchain: toolchain_channel(root)?,
        size_report: size_report(&files),
        files,
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    write(&app.join("build-manifest.json"), json.as_bytes())?;
    println!("built {} -> {}", request.example, app.display());
    println!("{}", format_size_report(&manifest.size_report));
    Ok(())
}

/// The base a built directory was made for, read from its manifest.
///
/// An older build has no `base` in its manifest; it was made before the field
/// existed, and it was made for the root.
pub fn manifest_base(app: &Path) -> Result<String, String> {
    let path = app.join("build-manifest.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(crate::serve::normalize_base(
        value
            .get("base")
            .and_then(|base| base.as_str())
            .unwrap_or("/"),
    ))
}

/// Points a page's own relative references at the path it will be served under.
///
/// `./app.js` resolves against the *document's* directory, so at
/// `/tools/demo/objects/42` it becomes `/tools/demo/objects/app.js` and the
/// page does not load. The policy forbids `<base>` (`base-uri 'none'`), so the
/// references are rewritten instead - which is also the honest thing, because
/// the path a build is deployed under is a property of the build.
pub fn rebase_index(html: &str, base: &str) -> String {
    let base = crate::serve::normalize_base(base);
    if base == "/" {
        return html.to_string();
    }
    html.replace("=\"./", &format!("=\"{base}"))
        .replace("='./", &format!("='{base}"))
}

/// Whether this example draws with the component crate's classes.
///
/// Read from its manifest rather than from a list here: an example that adds
/// the dependency gets the stylesheet without anyone remembering to say so,
/// and one that does not carry an unused file in its product.
fn uses_components(example_dir: &Path) -> Result<bool, String> {
    let manifest = example_dir.join("Cargo.toml");
    let text =
        std::fs::read_to_string(&manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    Ok(text.contains("rustify-components"))
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

pub fn list_files(dir: &Path) -> Result<BTreeMap<String, u64>, String> {
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

/// What a host that compresses would send for each of these files.
///
/// The size report counts bytes on disk, which is what a deployment stores; a
/// static host with compression turned on sends fewer, and the difference is
/// large enough for wasm and fonts that a transfer figure taken without it
/// describes a deployment nobody would run. `gzip -9` is the floor - brotli
/// sends less - so this is the conservative half of the answer.
pub fn compressed_sizes(
    dir: &Path,
    files: &BTreeMap<String, u64>,
) -> Result<BTreeMap<String, u64>, String> {
    let mut out = BTreeMap::new();
    for name in files.keys() {
        let output = std::process::Command::new("gzip")
            .args(["-9", "-c"])
            .stdin(std::fs::File::open(dir.join(name)).map_err(|e| format!("{name}: {e}"))?)
            .output()
            .map_err(|e| format!("cannot run gzip: {e}"))?;
        if !output.status.success() {
            return Err(format!("gzip {name}: exit {}", output.status));
        }
        out.insert(name.clone(), output.stdout.len() as u64);
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
    use super::rebase_index;
    use super::*;

    #[test]
    fn compressed_sizes_are_what_a_compressing_host_would_send() {
        let dir = std::env::temp_dir().join(format!("rustify-size-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("nested")).expect("a temporary directory");
        std::fs::write(dir.join("a.js"), "const x = 1;\n".repeat(500)).expect("a file");
        std::fs::write(
            dir.join("nested/b.bin"),
            (0u8..=255).cycle().take(4096).collect::<Vec<u8>>(),
        )
        .expect("a file");
        let files = list_files(&dir).expect("the listing");

        let compressed = compressed_sizes(&dir, &files).expect("gzip");
        assert_eq!(
            compressed.keys().collect::<Vec<_>>(),
            files.keys().collect::<Vec<_>>()
        );
        // Repetitive text compresses; bytes that are already spread out do not
        // compress much, and neither is allowed to be reported as zero.
        assert!(compressed["a.js"] < files["a.js"] / 10);
        assert!(compressed["nested/b.bin"] > 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

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

    const PAGE: &str = r#"<link rel="stylesheet" href="./runtime.css">
<script type="module" src="./app.js"></script>
<a href="/objects/1">an application link</a>"#;

    #[test]
    fn a_build_for_a_sub_path_has_its_own_directory() {
        // Two bases are two products; one directory could not hold both, and
        // a server started against the wrong one serves a page whose scripts
        // 404.
        let root = Path::new("/repo");
        let at_root = super::app_dir(
            root,
            &super::BuildRequest {
                example: "fusion-basic".to_string(),
                release: true,
                base: "/".to_string(),
            },
        );
        let under = super::app_dir(
            root,
            &super::BuildRequest {
                example: "fusion-basic".to_string(),
                release: true,
                base: "/tools/demo/".to_string(),
            },
        );
        assert!(at_root.ends_with("fusion-basic"));
        assert!(under.ends_with("fusion-basic@tools-demo"));
        assert_ne!(at_root, under);
    }

    #[test]
    fn a_build_for_the_root_is_left_alone() {
        assert_eq!(rebase_index(PAGE, "/"), PAGE);
        assert_eq!(rebase_index(PAGE, ""), PAGE);
    }

    #[test]
    fn a_build_for_a_sub_path_names_its_own_files_absolutely() {
        let rebased = rebase_index(PAGE, "/tools/demo");
        assert!(
            rebased.contains(r#"href="/tools/demo/runtime.css""#),
            "{rebased}"
        );
        assert!(rebased.contains(r#"src="/tools/demo/app.js""#), "{rebased}");
        // Only the page's own references. A link the application wrote is the
        // router's business, and rewriting it here would be a second opinion.
        assert!(rebased.contains(r#"href="/objects/1""#), "{rebased}");
    }

    #[test]
    fn the_base_is_normalised_the_way_serve_normalises_it() {
        // Whatever a person types, the product and the server agree.
        assert_eq!(
            rebase_index(PAGE, "tools/demo/"),
            rebase_index(PAGE, "/tools/demo")
        );
    }
}
