//! The actual compiler, helper and in-memory dev generation across saves.
mod support;

use std::fs;
use std::path::Path;
use support::*;

fn project() -> Temp {
    let temp = Temp::new("rust-dev");
    let from = fixture("components");
    for directory in ["src", "content", "static"] {
        copy_tree(
            &from.join(directory),
            &temp.path(&format!("site/{directory}")),
        );
    }
    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "rustify-docs.toml",
        "theme.json",
    ] {
        fs::copy(from.join(file), temp.path(&format!("site/{file}"))).unwrap();
    }
    let repo = fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")).unwrap();
    let mut manifest = temp.read("site/Cargo.toml");
    for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
        let old = format!("path = \"../../../../crates/{library}\"");
        assert_eq!(manifest.matches(&old).count(), 1);
        manifest = manifest.replacen(
            &old,
            &format!(
                "path = {}",
                serde_json::to_string(&repo.join("crates").join(library)).unwrap()
            ),
            1,
        );
    }
    temp.write("site/Cargo.toml", &manifest);
    pair_component_project(&temp.path("site"));
    temp
}

fn next_build(dev: &Dev, previous: u64) -> serde_json::Value {
    dev.events("/docs/", previous)
        .until(|state| state["building"] == false && state["revision"].as_u64().unwrap() > previous)
}

#[test]
fn real_rust_dev_serves_before_compile_reuses_content_and_preserves_failed_generations() {
    use std::os::unix::fs::PermissionsExt;
    let temp = project();
    let compiler = temp.path("cargo-observe");
    let calls = temp.path("compiler-calls");
    let quote = |path: &Path| format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"));
    fs::write(
        &compiler,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$1\" >> {}\nexec mbx \"$@\"\n",
            quote(&calls)
        ),
    )
    .unwrap();
    fs::set_permissions(&compiler, fs::Permissions::from_mode(0o755)).unwrap();
    let compiler_text = compiler.to_str().unwrap();
    let dev = Dev::start_pending(
        &temp.path("site"),
        &["--port", "0"],
        &[("RUSTIFY_DOCS_CARGO", compiler_text)],
    )
    .unwrap();
    let lock = fs::read(temp.path("site/Cargo.lock")).unwrap();
    let (status, initial) = get_text(&dev.url);
    assert_eq!(status, 200);
    assert!(initial.contains("Building your site"));
    let mut events = dev.events("/docs/", 0);
    let first = events.until(|state| state["building"] == false);
    assert_eq!(first["ok"], true, "{}\n{}", first, dev.log());
    assert_eq!(first["compile"]["host"]["profile"], "docs-host");
    assert_eq!(first["compile"]["wasm"]["profile"], "docs-wasm");
    assert!(first["compile"]["completed_at_ms"].as_u64().unwrap() > 0);
    let count = || fs::read_to_string(&calls).unwrap().lines().count();
    let compiled = count();
    let mut revision = first["revision"].as_u64().unwrap();

    temp.edit(
        "site/content/index.md",
        "# Component fixture",
        "# Component fixture\n\nA content save reuses Rust.",
    );
    let state = next_build(&dev, revision);
    assert_eq!(state["ok"], true, "{state}\n{}", dev.log());
    assert_eq!(
        count(),
        compiled,
        "Markdown must not invoke Cargo, including metadata"
    );
    assert!(get_text(&dev.url).1.contains("A content save reuses Rust."));
    revision = state["revision"].as_u64().unwrap();

    temp.edit(
        "site/rustify-docs.toml",
        "name = \"Component fixture\"",
        "name = \"Updated component docs\"",
    );
    let state = next_build(&dev, revision);
    assert_eq!(state["ok"], true, "{state}\n{}", dev.log());
    assert_eq!(
        count(),
        compiled,
        "site configuration must reuse the compiled pair"
    );
    revision = state["revision"].as_u64().unwrap();

    let theme = temp.read("site/theme.json");
    temp.write("site/theme.json", &format!("{theme}\n"));
    let sequence = state["sequence"].as_u64().unwrap();
    let state = dev.events("/docs/", revision).until(|state| {
        state["building"] == false && state["sequence"].as_u64().unwrap() > sequence
    });
    assert_eq!(state["ok"], true, "{state}\n{}", dev.log());
    assert_eq!(count(), compiled, "theme-only reads do not invoke Cargo");
    revision = state["revision"].as_u64().unwrap();

    let original = temp.read("site/src/demos.rs");
    temp.write(
        "site/src/demos.rs",
        &format!("{original}\npub fn invalid_dev_save() -> i32 {{ \"wrong type\" }}\n"),
    );
    let good = get_text(&dev.url).1;
    let failed = next_build(&dev, revision);
    assert_eq!(failed["ok"], false, "{failed}\n{}", dev.log());
    assert_eq!(
        get_text(&dev.url).1,
        good,
        "the whole last-good HTML remains served"
    );
    assert_eq!(failed["compile"]["host"]["success"], false);
    assert_eq!(failed["compile"]["wasm"]["success"], false);
    assert!(failed["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["file"] == "src/demos.rs" && item["line"].as_u64().unwrap_or(0) > 0));
    assert!(count() > compiled);
    revision = failed["revision"].as_u64().unwrap();

    temp.write(
        "site/src/demos.rs",
        &format!("{original}\n// A repaired, distinct source generation.\n"),
    );
    let repaired = next_build(&dev, revision);
    assert_eq!(repaired["ok"], true, "{repaired}\n{}", dev.log());
    assert_ne!(
        repaired["compile"]["generation"],
        first["compile"]["generation"]
    );
    assert_eq!(fs::read(temp.path("site/Cargo.lock")).unwrap(), lock);
    assert!(!temp.path("site/dist").exists());
}

fn counters(dev: &Dev) -> Vec<String> {
    let (status, html) = get_text(&format!("{}counter/", dev.url));
    assert_eq!(status, 200);
    dom_query::Document::from(html.as_str())
        .select("[data-testid=counter-value]")
        .nodes()
        .iter()
        .map(|node| node.text().to_string())
        .collect()
}

#[test]
fn an_external_path_dependency_is_read_watched_and_restored_after_deletion() {
    let temp = project();
    temp.write(
        "author-values/Cargo.toml",
        "[package]\nname = \"author-values\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    temp.write("author-values/src/lib.rs", "pub const VALUE: i32 = 7;\n");
    temp.edit(
        "site/Cargo.toml",
        "[dependencies]\n",
        "[dependencies]\nauthor-values = { path = \"../author-values\" }\n",
    );
    temp.edit(
        "site/src/demos.rs",
        "        7,\n",
        "        author_values::VALUE,\n",
    );
    // Adding an author dependency is an explicit lock update, before dev.
    // Every observed dev round below remains locked and offline.
    let metadata = std::process::Command::new("mbx")
        .args([
            "metadata",
            "--offline",
            "--format-version",
            "1",
            "--no-default-features",
            "--features",
            "ssr",
        ])
        .current_dir(temp.path("site"))
        .output()
        .unwrap();
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let lock = fs::read(temp.path("site/Cargo.lock")).unwrap();
    let dev = Dev::start(
        &temp.path("site"),
        &[],
        &[("RUSTIFY_DOCS_CARGO", "mbx"), ("CARGO_NET_OFFLINE", "true")],
    )
    .unwrap();
    let first = dev
        .events("/docs/counter/", 0)
        .until(|state| state["building"] == false);
    assert_eq!(first["ok"], true, "{first}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "7"]);
    temp.write("author-values/src/lib.rs", "pub const VALUE: i32 = 11;\n");
    let changed = next_build(&dev, first["revision"].as_u64().unwrap());
    assert_eq!(changed["ok"], true, "{changed}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "11"]);
    assert_ne!(
        changed["compile"]["generation"],
        first["compile"]["generation"]
    );
    let good = get_text(&format!("{}counter/", dev.url)).1;

    fs::rename(temp.path("author-values"), temp.path("values-away")).unwrap();
    let missing = next_build(&dev, changed["revision"].as_u64().unwrap());
    assert_eq!(missing["ok"], false, "{missing}\n{}", dev.log());
    assert_eq!(get_text(&format!("{}counter/", dev.url)).1, good);
    fs::rename(temp.path("values-away"), temp.path("author-values")).unwrap();
    let restored = next_build(&dev, missing["revision"].as_u64().unwrap());
    assert_eq!(restored["ok"], true, "{restored}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "11"]);
    assert_eq!(fs::read(temp.path("site/Cargo.lock")).unwrap(), lock);
    eprintln!(
        "external-dependency-evidence {}",
        serde_json::json!({"first":first,"changed":changed,"missing":missing,"restored":restored,"cli_log":dev.log()})
    );
}

#[test]
fn build_generated_includes_and_saves_during_compile_publish_only_the_latest_complete_view() {
    use std::os::unix::fs::PermissionsExt;
    use std::time::{Duration, Instant};
    let temp = project();
    temp.write("site/build-input.txt", "7\n");
    temp.write(
        "site/build.rs",
        "use std::{env, fs, path::PathBuf};\nfn main() {\n println!(\"cargo:rerun-if-changed=build-input.txt\");\n let value = fs::read_to_string(\"build-input.txt\").unwrap();\n let out = PathBuf::from(env::var_os(\"OUT_DIR\").unwrap());\n fs::write(out.join(\"counter_value.rs\"), format!(\"pub const BUILD_VALUE: i32 = {};\\n\", value.trim())).unwrap();\n}\n",
    );
    temp.edit(
        "site/src/demos.rs",
        "use leptos::prelude::*;\n",
        "use leptos::prelude::*;\ninclude!(concat!(env!(\"OUT_DIR\"), \"/counter_value.rs\"));\n",
    );
    temp.edit(
        "site/src/demos.rs",
        "        7,\n",
        "        BUILD_VALUE,\n",
    );
    let original = temp.read("site/src/demos.rs");
    let compiler = temp.path("compile-gate");
    let gate = temp.path("hold-build");
    let entered = temp.path("build-entered");
    let quote = |path: &Path| format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"));
    fs::write(&compiler, format!("#!/bin/sh\nif [ \"$1\" = build ] && [ -f {} ]; then printf 'entered\\n' > {}; while [ -f {} ]; do /bin/sleep 0.02; done; fi\nexec mbx \"$@\"\n",quote(&gate),quote(&entered),quote(&gate))).unwrap();
    fs::set_permissions(&compiler, fs::Permissions::from_mode(0o755)).unwrap();
    let dev = Dev::start(
        &temp.path("site"),
        &[],
        &[
            ("RUSTIFY_DOCS_CARGO", compiler.to_str().unwrap()),
            ("CARGO_NET_OFFLINE", "true"),
        ],
    )
    .unwrap();
    let first = dev
        .events("/docs/counter/", 0)
        .until(|state| state["building"] == false);
    assert_eq!(first["ok"], true, "{first}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "7"]);
    temp.write("site/build-input.txt", "11\n");
    let include = next_build(&dev, first["revision"].as_u64().unwrap());
    assert_eq!(include["ok"], true, "{include}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "11"]);
    let good = get_text(&format!("{}counter/", dev.url)).1;
    fs::write(&gate, "held").unwrap();
    temp.write(
        "site/src/demos.rs",
        &original.replace("BUILD_VALUE,", "BUILD_VALUE + 1,"),
    );
    let began = Instant::now();
    while !entered.exists() {
        assert!(began.elapsed() < Duration::from_secs(30), "{}", dev.log());
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(get_text(&format!("{}counter/", dev.url)).1, good);
    temp.write(
        "site/src/demos.rs",
        &original.replace("BUILD_VALUE,", "BUILD_VALUE + 2,"),
    );
    temp.write(
        "site/src/demos.rs",
        &original.replace("BUILD_VALUE,", "BUILD_VALUE + 3,"),
    );
    fs::remove_file(&gate).unwrap();
    let latest = next_build(&dev, include["revision"].as_u64().unwrap());
    assert_eq!(latest["ok"], true, "{latest}\n{}", dev.log());
    assert_eq!(counters(&dev), ["2", "2", "2", "14"]);
    assert!(get_text(&format!("{}counter/", dev.url))
        .1
        .contains("BUILD_VALUE + 3"));
    assert!(dev
        .log()
        .contains("inputs changed; discarding the candidate and rebuilding"));
    eprintln!(
        "build-include-burst-evidence {}",
        serde_json::json!({"first":first,"include_changed":include,"latest":latest,"cli_log":dev.log()})
    );
}
