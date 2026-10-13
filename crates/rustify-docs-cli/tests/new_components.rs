//! Components generation checks the active toolchain before touching the
//! destination, and requires the embedded CLI/framework/lock pair.

mod support;

use support::*;

fn target_toolchain(temp: &Temp, with_std: bool) -> Toolchain {
    let tools = Toolchain::new(temp, true);
    let libdir = temp.path("wasm-lib");
    std::fs::create_dir(&libdir).unwrap();
    temp.write("wasm-lib/libcore-test.rlib", "core");
    if with_std {
        temp.write("wasm-lib/libstd-test.rlib", "std");
    }
    std::fs::write(
        tools.bin.join("rustc"),
        format!(
            "#!/bin/sh\necho \"rustc $*\" >> '{}'\ncase \"$*\" in\n'--version') echo 'rustc 1.99.0 (test compiler)';;\n'--print target-libdir --target wasm32-unknown-unknown') echo '{}';;\n*) exit 1;;\nesac\n",
            tools.log.display(),
            libdir.display(),
        ),
    )
    .unwrap();
    tools
}

fn assert_no_staging(temp: &Temp) {
    let staging: Vec<_> = std::fs::read_dir(&temp.0)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .filter(|name| name.to_string_lossy().contains("rustify-docs-new"))
        .collect();
    assert!(staging.is_empty(), "{staging:?}");
}

#[test]
fn components_without_rust_stop_before_checking_the_directory() {
    let temp = Temp::new("components-no-rust");
    let tools = Toolchain::new(&temp, false);
    temp.write("full/notes.txt", "mine");
    let before = tree(&temp.path("full"));
    let refused = tools.run_in(&temp.0, &["new", "full", "--template", "components"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(stderr(&refused).contains("rustup toolchain install stable"));
    assert_eq!(tree(&temp.path("full")), before);
    assert!(tools.calls().is_empty());
    assert_no_staging(&temp);
}

#[test]
fn components_require_wasm_std_from_the_active_compiler_before_directory_checks() {
    let temp = Temp::new("components-no-target");
    let tools = target_toolchain(&temp, false);
    temp.write("full/notes.txt", "mine");
    let refused = tools.run_in(&temp.0, &["new", "full", "--template", "components"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(
        stderr(&refused).contains("rustup target add wasm32-unknown-unknown"),
        "{}",
        stderr(&refused)
    );
    assert_eq!(temp.read("full/notes.txt"), "mine");
    assert_eq!(
        tools.calls(),
        [
            "rustc --version",
            "rustc --print target-libdir --target wasm32-unknown-unknown"
        ]
    );
    assert_no_staging(&temp);
}

#[test]
fn components_toolchain_check_handles_an_unsupported_target_query() {
    let temp = Temp::new("components-target-query");
    let tools = Toolchain::new(&temp, true);
    let refused = tools.run_in(&temp.0, &["new", "site", "--template", "components"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(stderr(&refused).contains("rustup target add wasm32-unknown-unknown"));
    assert!(!temp.path("site").exists());
    assert_no_staging(&temp);
}

#[test]
fn components_never_write_into_a_nonempty_directory() {
    let temp = Temp::new("components-nonempty");
    let tools = target_toolchain(&temp, true);
    temp.write("full/notes.txt", "mine");
    let before = tree(&temp.path("full"));
    let refused = tools.run_in(&temp.0, &["new", "full", "--template", "components"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(stderr(&refused).contains("is not empty"));
    assert_eq!(tree(&temp.path("full")), before);
    assert_no_staging(&temp);
}

#[test]
fn components_ship_a_complete_paired_lock_or_refuse_without_writing() {
    let temp = Temp::new("components-pair");
    let tools = target_toolchain(&temp, true);
    let framework = include_str!("../assets/framework-revision.txt").trim();
    let cli = env!("RUSTIFY_DOCS_SOURCE_REVISION");
    let revision =
        |value: &str| value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
    let created = tools.run_in(&temp.0, &["new", "My Docs", "--template", "components"]);
    if !revision(framework) || !revision(cli) {
        assert_eq!(created.status.code(), Some(1));
        let message = stderr(&created);
        assert!(message.contains("components template"), "{message}");
        assert!(
            message.contains("paired") || message.contains("traceable CLI source revision"),
            "{message}"
        );
        assert!(message.contains("cargo install --git https://github.com/rockie/rustify-ui"));
        assert!(!temp.path("My Docs").exists());
    } else {
        succeeded(&created);
        let site = temp.path("My Docs");
        assert_eq!(
            tree(&site).into_keys().collect::<Vec<_>>(),
            [
                ".gitignore",
                "Cargo.lock",
                "Cargo.toml",
                "README.md",
                "content/guide/getting-started.md",
                "content/guide/writing-demos.md",
                "content/index.md",
                "rustify-docs.toml",
                "src/bin/docs-render.rs",
                "src/demos.rs",
                "src/lib.rs",
            ]
        );
        let manifest = temp.read("My Docs/Cargo.toml");
        assert!(manifest.starts_with("[package]\nname = \"my-docs\"\n"));
        assert!(manifest.contains("[workspace]"));
        assert!(manifest.contains("name = \"site_docs\""));
        assert_eq!(
            manifest.matches(&format!("rev = \"{framework}\"")).count(),
            3
        );
        assert!(!manifest.contains("path = \"../../"));
        let embedded = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/components-Cargo.lock"),
        )
        .unwrap();
        let lock = temp.read("My Docs/Cargo.lock");
        assert_eq!(
            lock,
            embedded.replacen("name = \"docs-site\"", "name = \"my-docs\"", 1)
        );
        assert!(temp
            .read("My Docs/rustify-docs.toml")
            .contains("name = \"My Docs\""));
        let readme = temp.read("My Docs/README.md");
        assert!(readme.contains("new 'My Docs' --template components"));
        assert!(readme.contains(cli) && readme.contains(framework));
        assert!(!readme.contains("{{"));
    }
    assert_eq!(
        tools.calls(),
        [
            "rustc --version",
            "rustc --print target-libdir --target wasm32-unknown-unknown"
        ]
    );
    assert_no_staging(&temp);
}
