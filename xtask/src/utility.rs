//! Independent single-threaded style WASM probes, without Makepad glue patches.

use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Command;

mod seed;

fn run(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} failed: {status}", args.join(" ")))
    }
}

fn facade(root: &Path, output: &Path, features: &str) -> Result<(), String> {
    run(
        root,
        "mbx",
        &[
            "build",
            "--locked",
            "-p",
            "rustify-style-wasm",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            features,
        ],
    )?;
    let metadata = Command::new("mbx")
        .args([
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ])
        .current_dir(root)
        .output()
        .map_err(|error| format!("style Cargo metadata: {error}"))?;
    if !metadata.status.success() {
        return Err(format!(
            "style Cargo metadata: {}",
            String::from_utf8_lossy(&metadata.stderr)
        ));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&metadata.stdout).map_err(|error| error.to_string())?;
    let target = metadata["target_directory"]
        .as_str()
        .ok_or("style Cargo metadata has no target_directory")?;
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    wasm_bindgen_cli_support::Bindgen::new()
        .input_path(
            Path::new(target).join("wasm32-unknown-unknown/release/rustify_style_wasm.wasm"),
        )
        .web(true)
        .map_err(|error| error.to_string())?
        .typescript(false)
        .out_name("style")
        .generate(output)
        .map_err(|error| format!("style wasm-bindgen: {error:#}"))
}

pub fn product_bundle(
    root: &Path,
    app: &Path,
    wasm: &[u8],
    scanner: bool,
    static_css: Option<&Path>,
) -> Result<(), String> {
    let module = wasmi::Module::new(&wasmi::Engine::default(), wasm)
        .map_err(|error| format!("utility feature detection: {error}"))?;
    if !module
        .exports()
        .any(|export| export.name() == "rustify_utility_protocol_version")
    {
        return if scanner {
            Err("--utility-scanner requires an application with the utility feature".into())
        } else {
            Ok(())
        };
    }
    let output = app.join("utility");
    facade(
        root,
        &output.join("compiler"),
        if scanner {
            "compiler,scanner"
        } else {
            "compiler"
        },
    )?;
    std::fs::copy(
        root.join("web/utility-worker.js"),
        output.join("utility-worker.js"),
    )
    .map_err(|error| error.to_string())?;
    let candidates = static_css
        .map(|path| std::fs::read_to_string(path).map_err(|error| error.to_string()))
        .transpose()?
        .map(|css| seed::candidates(&css))
        .transpose()?
        .unwrap_or_default();
    std::fs::write(
        output.join("static-candidates.json"),
        serde_json::to_vec(&candidates).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let files = crate::sources::fingerprint_tree(&output)?;
    let build_id = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&files).map_err(|error| error.to_string())?)
    );
    std::fs::write(
        output.join("build-manifest.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "protocol_version": 1, "build_id": build_id, "files_sha256": files, "scanner_enabled": scanner,
            "static_candidates": "static-candidates.json",
        }))
        .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

pub fn build(args: &[String]) -> Result<(), String> {
    if let Some(group @ ("compiler" | "compiler-api" | "upstream-units")) =
        crate::option(args, "--group")
    {
        let root = crate::build::repo_root();
        let output = crate::option(args, "--output")
            .map(|output| root.join(output))
            .unwrap_or_else(|| root.join("target/utility").join(group));
        return facade(&root, &output, group);
    }
    if crate::option(args, "--group") != Some("cn") {
        return Err(
            "utility-build requires --group cn, compiler, compiler-api or upstream-units".into(),
        );
    }
    let root = crate::build::repo_root();
    let output = root.join(crate::option(args, "--output").unwrap_or("target/utility/cn"));
    facade(&root, &output, "cn")?;
    for name in ["cn-node.mjs", "cn-cli.mjs"] {
        std::fs::copy(
            root.join("crates/rustify-style-wasm/host").join(name),
            output.join(name),
        )
        .map_err(|error| error.to_string())?;
    }
    std::fs::copy(
        root.join("crates/rustify-style/upstream/cn/LICENSE"),
        output.join("LICENSE"),
    )
    .map_err(|error| error.to_string())?;
    let manifest = output.join("build-manifest.json");
    if manifest.exists() {
        std::fs::remove_file(&manifest).map_err(|error| error.to_string())?;
    }
    let files = crate::sources::fingerprint_tree(&output)?;
    std::fs::write(
        manifest,
        serde_json::to_vec_pretty(&serde_json::json!({
            "protocol_version": 1, "group": "cn", "profile": "release", "files_sha256": files,
        }))
        .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

/// V-11 measurements on this host: the Rust cn and compiler natively and
/// behind their JS facades, beside the fixed JS references run in the same
/// session, on identical workloads. Measurement only; no threshold is applied.
pub fn bench(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let out = root.join(crate::option(args, "--out").unwrap_or("target/utility-bench"));
    std::fs::create_dir_all(&out).map_err(|error| error.to_string())?;
    let path = |name: &str| out.join(name).to_string_lossy().into_owned();
    run(
        &root,
        "mbx",
        &[
            "run",
            "--locked",
            "--release",
            "-p",
            "rustify-style",
            "--features",
            "compiler",
            "--example",
            "utility-bench",
            "--",
            "tests/utility/cn/runtime-fixtures.json",
            "tests/utility/compiler",
            &path("native.json"),
        ],
    )?;
    let cn = root.join("target/utility/cn");
    let compiler = root.join("target/utility/compiler");
    facade(&root, &cn, "cn")?;
    facade(&root, &compiler, "compiler")?;
    run(
        &root,
        "node",
        &[
            "tests/utility/bench-wasm.mjs",
            &cn.to_string_lossy(),
            &compiler.to_string_lossy(),
            &path("wasm.json"),
        ],
    )?;
    run(
        &root,
        "node",
        &[
            "tests/utility/cn/bench.mjs",
            "--out",
            &path("cn-reference.json"),
        ],
    )?;
    run(
        &root,
        "node",
        &[
            "tests/utility/compiler/bench.mjs",
            "--output",
            &path("compiler-reference.json"),
        ],
    )?;
    let read = |name: &str| -> Result<serde_json::Value, String> {
        serde_json::from_str(
            &std::fs::read_to_string(out.join(name)).map_err(|error| format!("{name}: {error}"))?,
        )
        .map_err(|error| format!("{name}: {error}"))
    };
    let (native, wasm) = (read("native.json")?, read("wasm.json")?);
    let (cn_reference, compiler_reference) =
        (read("cn-reference.json")?, read("compiler-reference.json")?);
    let mut rows = Vec::new();
    for name in [
        "cnVariadicWarm",
        "cnSingleWarm",
        "mergeStringWarm",
        "mergeUncached",
        "clsxJoinOnly",
        "createConfigAndFirstMerge",
    ] {
        let p50 = |report: &serde_json::Value, path: &str| {
            report.pointer(&format!("{path}/{name}/p50")).cloned()
        };
        rows.push(serde_json::json!({
            "workload": format!("cn.{name}"), "unit": "ns/call",
            "jsReference": p50(&cn_reference, "/measurements"),
            "rustNative": p50(&native, "/cn/measurements"),
            "rustWasmCrossing": p50(&wasm, "/cn/measurements"),
        }));
    }
    for name in ["cold", "warm", "noOp"] {
        let p50 = |report: &serde_json::Value, path: &str| {
            report.pointer(&format!("{path}/{name}/p50Ms")).cloned()
        };
        rows.push(serde_json::json!({
            "workload": format!("compiler.{name}"), "unit": "ms",
            "jsReference": p50(&compiler_reference, "/metrics"),
            "rustNative": p50(&native, "/compiler/metrics"),
            "rustWasmSession": p50(&wasm, "/compiler/metrics"),
        }));
    }
    rows.push(serde_json::json!({
        "workload": format!("compiler.batch({} candidates)", native["batch"]["candidates"]),
        "unit": "ms", "rustNative": native["batch"]["p50Ms"], "rustWasmSession": wasm["batch"]["p50Ms"],
    }));
    let summary = serde_json::json!({ "schemaVersion": 1, "p50": rows });
    std::fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(&summary).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    for row in &rows {
        println!("{row}");
    }
    Ok(())
}

/// Executes the fixed upstream test files against the Rust facade and checks
/// the committed per-test dispositions; the official TypeScript run is the
/// harness control. Needs `ref/tailwindcss` at the fixed commit and the
/// executor's own locked dependencies.
fn upstream_tests(root: &Path) -> Result<(), String> {
    let executor = root.join("tests/utility/compiler/upstream-exec");
    if !executor.join("node_modules/vitest").is_dir() {
        return Err(
            "upstream executor dependencies are missing; run `npm ci --prefix tests/utility/compiler/upstream-exec`"
                .into(),
        );
    }
    run(
        root,
        "node",
        &["tests/utility/compiler/upstream-test-ledger.mjs", "--check"],
    )?;
    facade(
        root,
        &root.join("target/upstream-exec/upstream-units"),
        "upstream-units",
    )?;
    for implementation in ["official", "rust"] {
        run(
            root,
            "node",
            &[
                "tests/utility/compiler/upstream-exec/run.mjs",
                "--impl",
                implementation,
            ],
        )?;
    }
    run(
        root,
        "node",
        &["tests/utility/compiler/upstream-exec/ledger.mjs", "--check"],
    )?;
    run(
        root,
        "node",
        &["tests/utility/compiler/upstream-exec/internal-entries.mjs"],
    )
}

pub fn conformance(args: &[String]) -> Result<(), String> {
    if !args.iter().any(|arg| arg == "--check") {
        return Err("utility-conformance requires --check".into());
    }
    let group = crate::option(args, "--group").unwrap_or("compiler-full");
    if group == "cn-native" {
        let root = crate::build::repo_root();
        facade(&root, &root.join("target/utility-conformance/cn"), "cn")?;
        return if args.iter().any(|arg| arg == "--build-only") {
            Ok(())
        } else {
            run(&root, "node", &["tests/utility/cn/run-wasm-native.mjs"])
        };
    }
    if group == "scanner-pipeline" {
        let root = crate::build::repo_root();
        for oracle in ["external-variables.mjs", "source-pipeline.mjs"] {
            run(
                &root,
                "node",
                &[&format!("tests/utility/compiler/{oracle}"), "--check"],
            )?;
        }
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "-p",
                "rustify-style-wasm",
                "--no-default-features",
                "--features",
                "compiler,scanner",
                "--lib",
            ],
        )?;
        facade(
            &root,
            &root.join("target/utility-conformance/scanner-pipeline"),
            "compiler,scanner",
        )?;
        return run(
            &root,
            "node",
            &["tests/utility/compiler/run-scanner-pipeline-wasm.mjs"],
        );
    }
    if group == "compiler-api" {
        let root = crate::build::repo_root();
        run(
            &root,
            "node",
            &["tests/utility/compiler/upstream-test-ledger.mjs", "--check"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/upstream-test-ledger.test.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/public-api.mjs", "--check"],
        )?;
        run(
            &root,
            "node",
            &[
                "tests/utility/compiler/design-system-completions.mjs",
                "--check",
            ],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/utf16-functions.mjs", "--check"],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::public_api::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_value_tests::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_declaration_tests::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_theme_tests::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_ast_fields_tests::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_parser_tests::",
            ],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_error_tests::",
            ],
        )?;
        facade(
            &root,
            &root.join("target/utility-conformance/compiler-api"),
            "compiler-api",
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-api-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-input-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-candidates-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-object-order-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-registration-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-media-source-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-ownership-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &[
                "tests/utility/compiler/run-public-ast-graph-wasm.mjs",
                "--input-mutations",
                "--apply-locations",
                "--apply-multi-locations",
            ],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-mutations-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-setup-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-apply-input-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-custom-input-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-custom-origin-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-import-input-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-failed-setup-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-apply-failure-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-function-order-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-import-failure-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-async-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-public-ast-budget-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-functions-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-values-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-theme-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-theme-sources-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-ast-fields-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-parser-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-errors-wasm.mjs"],
        )?;
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::utf16_family_tests::",
            ],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/utf16-families.mjs", "--check"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-families-wasm.mjs"],
        )?;
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-shadow-budget-wasm.mjs"],
        )?;
        return run(
            &root,
            "node",
            &["tests/utility/compiler/run-utf16-animations-wasm.mjs"],
        );
    }
    if group == "compiler-full" {
        for part in ["compiler-api", "compiler-slice", "compiler-upstream"] {
            conformance(&["--check".into(), "--group".into(), part.into()])?;
        }
        return Ok(());
    }
    if group == "compiler-upstream" {
        return upstream_tests(&crate::build::repo_root());
    }
    if !["compiler-smoke", "compiler-slice"].contains(&group) {
        return Err(format!("unknown utility conformance group {group}"));
    }
    let root = crate::build::repo_root();
    if group == "compiler-slice" {
        for oracle in [
            "static.mjs",
            "effects.mjs",
            "variants.mjs",
            "functional-spacing.mjs",
            "dynamic-variants.mjs",
            "variant-parameters.mjs",
            "colors.mjs",
            "typography.mjs",
            "borders.mjs",
            "backgrounds.mjs",
            "gradients.mjs",
            "transforms.mjs",
            "theme.mjs",
            "filters.mjs",
            "shadows.mjs",
            "prefix.mjs",
            "motion.mjs",
            "grid.mjs",
            "layout.mjs",
            "masks.mjs",
            "miscellaneous.mjs",
            "structural-spacing.mjs",
            "effects-complete.mjs",
            "arbitrary-properties.mjs",
            "custom-utilities.mjs",
            "custom-variants.mjs",
            "parser-errors.mjs",
            "apply.mjs",
            "functions.mjs",
            "legacy-theme.mjs",
            "imports.mjs",
            "source-directives.mjs",
            "external-variables.mjs",
        ] {
            run(
                &root,
                "node",
                &[&format!("tests/utility/compiler/{oracle}"), "--check"],
            )?;
        }
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "compiler",
                "--lib",
                "compiler::",
            ],
        )?;
    }
    run(
        &root,
        "node",
        &["tests/utility/compiler/reference.mjs", "--check"],
    )?;
    run(
        &root,
        "mbx",
        &[
            "test",
            "--locked",
            "-p",
            "rustify-style-wasm",
            "--no-default-features",
            "--features",
            "compiler",
            "--lib",
        ],
    )?;
    facade(
        &root,
        &root.join("target/utility-conformance/compiler"),
        "compiler",
    )?;
    if group == "compiler-slice" {
        run(
            &root,
            "node",
            &["tests/utility/compiler/run-wasm.mjs", "--slice"],
        )
    } else {
        run(&root, "node", &["tests/utility/compiler/run-wasm.mjs"])
    }
}

pub fn baseline(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let output = root.join("target/utility-baseline");
    if args.iter().any(|arg| arg == "--serve") {
        if !output.join("build-manifest.json").is_file() {
            return Err("run `mbx xtask utility-baseline --build-only` first".into());
        }
        let port = crate::option(args, "--port")
            .unwrap_or("0")
            .parse()
            .map_err(|_| "invalid --port")?;
        return crate::serve::serve(crate::serve::ServeConfig {
            root: output,
            base: "/".into(),
            port,
            csp: crate::serve::CspMode::Strict,
            spa: false,
            fault: std::sync::Mutex::new(None),
        });
    }
    let group = crate::option(args, "--group").unwrap_or("all");
    if !["worker", "css", "extractor", "compiler", "cn", "all"].contains(&group) {
        return Err("--group must be worker, css, extractor, compiler, cn or all".into());
    }
    std::fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    if !["extractor", "cn"].contains(&group) {
        facade(&root, &output.join("worker"), "baseline")?;
    }
    if group == "cn" || group == "all" {
        // The JS facade in the page realm, with the fixed runtime corpus and
        // the same pure fixture interpreter the Node gate uses.
        facade(&root, &output.join("cn"), "cn")?;
        for name in ["runtime-fixtures.json", "recipes.mjs", "runtime-check.mjs"] {
            std::fs::copy(
                root.join("tests/utility/cn").join(name),
                output.join("cn").join(name),
            )
            .map_err(|error| format!("copy {name}: {error}"))?;
        }
    }
    if group == "extractor" || group == "all" {
        run(
            &root,
            "mbx",
            &[
                "test",
                "--locked",
                "-p",
                "rustify-style",
                "--no-default-features",
                "--features",
                "scanner",
                "--lib",
            ],
        )?;
        facade(&root, &output.join("extractor"), "scanner")?;
        std::fs::copy(
            root.join("tests/utility/scanner/fixtures.json"),
            output.join("scanner-fixtures.json"),
        )
        .map_err(|error| error.to_string())?;
    }
    if group == "compiler" || group == "all" {
        facade(&root, &output.join("compiler"), "compiler")?;
        std::fs::copy(
            root.join("web/utility-worker.js"),
            output.join("utility-worker.js"),
        )
        .map_err(|error| error.to_string())?;
    }
    std::fs::write(output.join("static-candidates.json"), b"[]")
        .map_err(|error| error.to_string())?;
    for name in ["index.html", "app.css", "app.js", "worker.js", "probe.css"] {
        std::fs::copy(
            root.join("tests/utility/baseline").join(name),
            output.join(name),
        )
        .map_err(|error| format!("copy {name}: {error}"))?;
    }
    std::fs::copy(
        root.join("tests/utility/compiler/css-scope-property-nesting.css"),
        output.join("official.css"),
    )
    .map_err(|error| error.to_string())?;
    let manifest_path = output.join("build-manifest.json");
    if manifest_path.exists() {
        std::fs::remove_file(&manifest_path).map_err(|error| error.to_string())?;
    }
    let files = crate::sources::fingerprint_tree(&output)?;
    let build_id = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&files).map_err(|error| error.to_string())?)
    );
    let manifest = serde_json::json!({
        "protocol_version": 1,
        "profile": "release",
        "base": "/",
        "group": group,
        "build_id": build_id,
        "files_sha256": files,
        "static_candidates": "static-candidates.json",
    });
    std::fs::write(
        manifest_path,
        serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    if args.iter().any(|arg| arg == "--build-only") {
        return Ok(());
    }
    let pattern = match group {
        "worker" => "scanner-free",
        "css" => "strict CSP",
        "extractor" => "extractor WASM",
        "compiler" => "Rust compiler Worker",
        "cn" => "cn WASM facade",
        _ => ".",
    };
    run(
        &root,
        "npx",
        &[
            "playwright",
            "test",
            "--project=utility-baseline",
            "--workers=1",
            "utility-baseline.spec.ts",
            "-g",
            pattern,
        ],
    )
}
