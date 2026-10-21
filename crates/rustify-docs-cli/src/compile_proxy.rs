//! Test-only docs-host/docs-wasm compile-to-render/load proxy.
//!
//! Root hook: `#[cfg(test)] mod compile_proxy;`
//! Run the ignored test with RUSTIFY_COMPILE_PROXY_WORK naming a fresh directory,
//! serve its dist/ through tests/docs-ssg/static-host.mjs, and run
//! harness/measure-compile-islands.mjs against that same directory and origin.
//! The CLI is installed and measured with docs-cli:
//! `mbx test --locked --profile docs-cli -p rustify-docs-cli --bin rustify-docs
//! compile_proxy::compile_to_islands_proxy -- --ignored --exact --nocapture`.

use rustify_docs::content::{Diagnostic, Project};
use rustify_docs::diagnostic::Severity;
use rustify_docs::publish::{self, Area};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const SAMPLES: usize = 5;
const PROXY: &str = concat!(
    "pub fn catalog_proxy() -> impl IntoView {\n",
    "    let scope = rustify_ui::id::next(\"catalog\").unwrap_or_else(|| \"catalog\".to_string());\n",
    "    view! { <crate::catalog::CatalogView scope=scope /> }\n",
    "}\n",
);

fn epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn atomic_json(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let staging = path.with_extension("tmp");
    fs::write(&staging, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    fs::rename(staging, path).unwrap();
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn prepare(repo: &Path, work: &Path) -> (PathBuf, String, Vec<u8>) {
    let fixture = repo.join("tests/docs-ssg/fixtures/components");
    let site = work.join("site");
    assert!(!site.exists(), "choose a fresh proxy work directory");
    for directory in ["src", "content", "static"] {
        copy_tree(&fixture.join(directory), &site.join(directory));
    }
    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "rustify-docs.toml",
        "theme.json",
    ] {
        fs::copy(fixture.join(file), site.join(file)).unwrap();
    }
    let mut manifest = fs::read_to_string(site.join("Cargo.toml")).unwrap();
    for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
        let old = format!("path = \"../../../../crates/{library}\"");
        assert_eq!(manifest.matches(&old).count(), 1);
        let path = repo.join("crates").join(library);
        manifest = manifest.replacen(
            &old,
            &format!(
                "path = {}",
                serde_json::to_string(path.to_str().unwrap()).unwrap()
            ),
            1,
        );
    }
    fs::write(site.join("Cargo.toml"), manifest).unwrap();
    // Use the actual CLI binary of this test build; an installed C can also
    // be selected by the existing browser-harness RUSTIFY_DOCS_BIN variable.
    let cli = std::env::var_os("RUSTIFY_DOCS_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("rustify-docs")
        });
    assert!(
        cli.is_file(),
        "build the same-profile rustify-docs binary first, or set RUSTIFY_DOCS_BIN to the real under-test C"
    );
    let output = Command::new("python3")
        .arg(repo.join("scripts/docs-pages-prepare.py"))
        .arg("--project")
        .arg(site.join("Cargo.toml"))
        .arg("--cli")
        .arg(cli)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let pair: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(pair["candidate_sha"], super::SOURCE_REVISION);
    assert_eq!(
        pair["framework_revision"],
        super::EXPECTED_FRAMEWORK_REV.trim()
    );
    let source = fs::read_to_string(site.join("src/demos.rs")).unwrap();
    assert_eq!(source.matches(PROXY).count(), 1);
    let lock = fs::read(site.join("Cargo.lock")).unwrap();
    (site, source, lock)
}

fn view_source(original: &str, index: usize, marker: &str) -> String {
    let literal = serde_json::to_string(marker).unwrap();
    let function = format!(
        r#"pub fn catalog_proxy() -> impl IntoView {{
    let scope = rustify_ui::id::next("catalog").unwrap_or_else(|| "catalog".to_string());
    let proof = RwSignal::new({index}_i32);
    view! {{
        <div data-compile-proxy-version={literal}>
            <crate::catalog::CatalogView scope=scope />
            <button class="sr-only" data-testid="compile-proxy-change" on:click=move |_| proof.update(|value| *value += {step})>
                "Compile proxy input"
            </button>
            <output class="sr-only" data-testid="compile-proxy-value">{{move || proof.get()}}</output>
        </div>
    }}
}}
"#,
        step = 100 + index
    );
    original.replacen(PROXY, &function, 1)
}

fn wait_json(path: &Path, session: &str, sample: Option<usize>) -> Value {
    let started = Instant::now();
    loop {
        if let Ok(bytes) = fs::read(path) {
            let value: Value = serde_json::from_slice(&bytes).expect("atomic proxy JSON is valid");
            assert_eq!(
                value["session"], session,
                "another run wrote this proxy handshake"
            );
            if let Some(sample) = sample {
                assert_eq!(value["sample"], sample);
            }
            return value;
        }
        assert!(
            started.elapsed() < Duration::from_secs(120),
            "proxy browser acknowledgement timed out: {}",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn history(site: &Path) -> BTreeSet<PathBuf> {
    fs::read_dir(site.join(".rustify-docs/logs/history"))
        .into_iter()
        .flatten()
        .map(|entry| entry.unwrap().path())
        .collect()
}

fn describe_history(
    work: &Path,
    site: &Path,
    sample: usize,
    before: &BTreeSet<PathBuf>,
) -> Vec<Value> {
    let mut records = Vec::new();
    for directory in history(site).difference(before) {
        let destination = work
            .join("evidence")
            .join(format!("sample-{sample}"))
            .join("history")
            .join(directory.file_name().unwrap());
        let mut timing: Value =
            serde_json::from_slice(&fs::read(directory.join("timing.json")).unwrap()).unwrap();
        timing["history_directory"] =
            json!(destination.strip_prefix(work).unwrap().to_str().unwrap());
        records.push(timing);
    }
    records.sort_by_key(|record| record["completed_at_ms"].as_u64().unwrap());
    records
}

fn preserve_history(work: &Path, site: &Path, sample: usize, before: &BTreeSet<PathBuf>) {
    for directory in history(site).difference(before) {
        let destination = work
            .join("evidence")
            .join(format!("sample-{sample}"))
            .join("history")
            .join(directory.file_name().unwrap());
        copy_tree(directory, &destination);
    }
}

fn publication(
    project: &Project,
    out: &Path,
    generation: &mut rustify_docs::generation::Generation,
) -> Result<(), String> {
    publish::check_out(project, out)?;
    let area = Area::of(out)?;
    let lock = publish::lock(&area, true).map_err(|error| format!("{error:?}"))?;
    publish::recover(&lock)?;
    publish::replaceable(&area.out)?;
    publish::add_manifest(
        &mut generation.files,
        &project.config.site.base,
        super::VERSION,
        &generation.locale_not_found,
        &generation.page_policies,
    );
    publish::publish(&lock, &generation.files)
}

fn diagnostic_json(diagnostics: &[Diagnostic]) -> Vec<Value> {
    diagnostics
        .iter()
        .map(|error| {
            json!({
                "severity":error.severity.as_str(),"rule":error.rule,"file":error.file,
                "line":error.line,"column":error.column,"message":error.message,
            })
        })
        .collect()
}

#[test]
#[ignore = "requires the matching compile-islands Chromium sampler and static host"]
fn compile_to_islands_proxy() {
    let work = std::env::var_os("RUSTIFY_COMPILE_PROXY_WORK")
        .map(PathBuf::from)
        .expect("set RUSTIFY_COMPILE_PROXY_WORK to a fresh dedicated directory");
    fs::create_dir_all(&work).unwrap();
    let work = fs::canonicalize(work).unwrap();
    let repo = fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")).unwrap();
    let (site, original, lock) = prepare(&repo, &work);
    let hostname = Command::new("hostname")
        .output()
        .expect("hostname identifies the same measurement machine");
    assert!(hostname.status.success());
    let hostname = String::from_utf8(hostname.stdout)
        .unwrap()
        .trim()
        .to_string();
    let session = format!("{}-{}", std::process::id(), epoch_ms());
    let cli_build = json!({
        "required_cargo_profile":"docs-cli","profile_env":option_env!("PROFILE"),
        "debug_assertions":cfg!(debug_assertions),"panic_strategy":if cfg!(panic="unwind") {"unwind"} else {"abort"},
        "test_executable":std::env::current_exe().unwrap(),
        "profile_evidence":"Root's actual mbx test --profile docs-cli invocation log; no runtime profile override",
    });
    atomic_json(
        &work.join("session.json"),
        &json!({
            "schema_version":1,"session":session,"samples":SAMPLES,"hostname":hostname,
            "os":std::env::consts::OS,"arch":std::env::consts::ARCH,"created_epoch_ms":epoch_ms(),
            "base":"/docs/","page":"/docs/proxy/","profiles":["docs-host","docs-wasm"],
        "cli_build":cli_build,
            "scope":"compile/render/load proxy with current theme/slots; original catalog and counters; not the M4 dev endpoint or A/B largest-page gate",
        }),
    );
    println!("compile proxy work: {}", work.display());
    let ready = wait_json(&work.join("browser-ready.json"), &session, None);
    assert_eq!(
        ready["ready"], true,
        "browser sampler setup failed: {ready}"
    );
    let mut results = Vec::new();
    for index in 0..=SAMPLES {
        let marker = format!("compile-proxy-sample-{index}");
        let source = view_source(&original, index, &marker);
        fs::write(site.join("src/demos.rs"), &source).unwrap();
        let before_history = history(&site);
        let start_epoch = epoch_ms();
        let started = Instant::now();
        let (project, diagnostics) = Project::load(&site).expect("the real author project loads");
        let (mut diagnostics, mut generation, _, _, rust_build, guard) = super::analyse(
            &project,
            diagnostics,
            false,
            super::rust_site::BuildMode::Development,
        );
        if let Some(guard) = &guard {
            if let Err(errors) = guard.validate_current() {
                diagnostics.extend(errors);
                generation = None;
            }
        }
        let mut published = false;
        if !diagnostics
            .iter()
            .any(|error| error.severity == Severity::Error)
        {
            if let Some(generation) = &mut generation {
                match publication(&project, &work.join("dist"), generation) {
                    Ok(()) => published = true,
                    Err(error) => diagnostics.push(Diagnostic::general(
                        Severity::Error,
                        "compile-proxy-publication",
                        error,
                    )),
                }
            }
        }
        let finished_epoch = epoch_ms();
        let elapsed = started.elapsed().as_millis() as u64;
        let attempts = describe_history(&work, &site, index, &before_history);
        assert_eq!(
            fs::read(site.join("Cargo.lock")).unwrap(),
            lock,
            "every actual build is --locked"
        );
        let source_unchanged = fs::read(site.join("src/demos.rs")).unwrap() == source.as_bytes();
        let end = [
            rust_build["host"]["completed_at_ms"].as_u64(),
            rust_build["wasm"]["completed_at_ms"].as_u64(),
        ]
        .into_iter()
        .flatten()
        .max();
        let mut resources = serde_json::Map::new();
        let mut page_sha = None;
        let mut page_bytes = None;
        let mut wasm_path = None;
        if published {
            let files = &generation.as_ref().unwrap().files;
            page_sha = Some(digest(&files["proxy/index.html"]));
            page_bytes = Some(files["proxy/index.html"].len());
            for (path, bytes) in files.iter().filter(|(path, _)| path.starts_with("assets/")) {
                resources.insert(
                    path.clone(),
                    json!({"sha256":digest(bytes),"bytes":bytes.len()}),
                );
                if path.ends_with("/app_bg.wasm") {
                    assert!(wasm_path.replace(path.clone()).is_none());
                }
            }
        }
        let mut descriptor = json!({
            "schema_version":1,"session":session,"sample":index,"kind":if index==0 {"cold"} else {"incremental"},
            "cli_build":cli_build,
            "success":published&&source_unchanged,"marker":marker,"initial":index,"after_input":100+index*2,
            "generation":rust_build["generation"],"host":rust_build["host"],"wasm":rust_build["wasm"],
            "compile_complete_epoch_ms":end,"pipeline_finished_epoch_ms":finished_epoch,
            "native_clock":{"epoch_before":start_epoch,"epoch_after":finished_epoch,"elapsed_monotonic_ms":elapsed,
                "wall_monotonic_difference_ms":finished_epoch as i128-start_epoch as i128-elapsed as i128},
            "source_sha256":digest(source.as_bytes()),"source_unchanged":source_unchanged,
            "page":"/docs/proxy/","base":"/docs/","page_sha256":page_sha,"page_bytes":page_bytes,"wasm_path":wasm_path,
            "resources":resources,"compile_attempts":attempts,"diagnostics":diagnostic_json(&diagnostics),
        });
        descriptor["descriptor_written_epoch_ms"] = json!(epoch_ms());
        atomic_json(
            &work
                .join("descriptors")
                .join(format!("sample-{index}.json")),
            &descriptor,
        );
        let acknowledgement = wait_json(
            &work.join("acks").join(format!("sample-{index}.json")),
            &session,
            Some(index),
        );
        // Large raw Cargo transcripts are already retained under site/ and
        // copied for evidence after the measured browser endpoint.
        preserve_history(&work, &site, index, &before_history);
        println!(
            "compile proxy/{index}: native={}, browser={}, compile-end={}",
            descriptor["success"],
            acknowledgement["valid"],
            descriptor["compile_complete_epoch_ms"]
        );
        results.push(json!({"native":descriptor,"browser":acknowledgement}));
    }
    atomic_json(
        &work.join("native-results.json"),
        &json!({"session":session,"samples":results}),
    );
    assert!(results.iter().all(|sample| sample["native"]["success"] == true && sample["browser"]["valid"] == true),
        "all raw samples remain in {}", work.display());
}
