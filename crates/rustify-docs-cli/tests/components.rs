//! One real author project across successful, refused and recovered builds.
//! Cargo and the helper run through the public CLI, with one persistent cache.

mod support;

use dom_query::Document;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use support::*;

const DEMO: &str = concat!(
    "/// The function and its containing file are displayed verbatim by the docs.\n",
    "pub fn counter_demo() -> impl IntoView {\n",
    "    counter(\n",
    "        7,\n",
    "        \"Résumé — a real registered counter\".to_string(),\n",
    "        view! { <p>\"This child is part of the same view on both targets.\"</p> }.into_any(),\n",
    "    )\n",
    "}\n",
);

const COLD_ERRORS: &str = concat!(
    "---\ndescription: Independent cold errors\n---\n# Independent errors\n\n",
    "```rustify-component\nname: Counter\nprops:\n  unexpected: 1\n```\n\n",
    "```rustify-component\nname: UnregisteredCounter\n```\n\n",
    "```rustify-demo\nname: MissingDemo\n```\n",
);

const DATA_ERRORS: &str = concat!(
    "---\ndescription: Located property and YAML errors\n---\n# Property errors\n\n",
    "```rustify-component\nname: Counter\nprops:\n  'initial': not-an-integer\n```\n\n",
    "```rustify-component\nname: Counter\nprops: {initial: 1, initial: 2}\n```\n\n",
    "```rustify-component\nname: Counter\nchildren: |\n  wrong\nprops: {label: wrong, initial: wrong}\n```\n\n",
    "```rustify-component\nname: Swatch\nprops:\n  palette: {accent: 3}\n```\n\n",
    "```rustify-component\nname: Swatch\nprops:\n  labels: wrong\n  palette: {accent: red}\n```\n\n",
    "```rustify-component\nname: Swatch\n```\n",
);

/// Chinese text before the error on its line, with CRLF line ends.
const ZH_CRLF_ERRORS: &str = concat!(
    "---\r\ndescription: 中文与 CRLF\r\n---\r\n# 属性错误\r\n\r\n",
    "中文前缀：下面的计数器初始值写错了。\r\n\r\n",
    "```rustify-component\r\nname: Counter\r\nprops: {label: 中文标签, initial: 错误}\r\n```\r\n",
);

/// An author composite and an alias, whose omission rules only the
/// registration knows, beside the fixture's own components.
const AUTHOR_TYPES: &str = concat!(
    "\n/// Author prop types for located props diagnostics.\n",
    "#[derive(Clone, Debug, PartialEq, rustify_docs::__private::serde::Serialize, rustify_docs::__private::serde::Deserialize)]\n",
    "#[serde(crate = \"rustify_docs::__private::serde\")]\n",
    "pub struct Palette {\n    pub accent: String,\n}\n\n",
    "pub type Labels = Vec<String>;\n",
);

const REGISTRY_END: &str = "        }\n        demos {";

const INITIAL_REPAIR: &str =
    "Write `initial: 0` for declared type `i32` (default expression `2`). Reported props path: `initial`.";

fn project() -> Temp {
    let temp = Temp::new("components");
    let from = fixture("components");
    // Copy only author inputs. The fixture's hidden target/cache can be huge.
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
    let root = fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")).unwrap();
    let mut manifest = temp.read("site/Cargo.toml");
    for library in ["rustify-ui", "rustify-components", "rustify-docs"] {
        let old = format!("path = \"../../../../crates/{library}\"");
        assert_eq!(
            manifest.matches(&old).count(),
            1,
            "the fixture's direct {library} dependency"
        );
        let path = root.join("crates").join(library);
        let quoted = serde_json::to_string(path.to_str().unwrap()).unwrap();
        manifest = manifest.replacen(&old, &format!("path = {quoted}"), 1);
    }
    temp.write("site/Cargo.toml", &manifest);
    pair_component_project(&temp.path("site"));
    temp
}

#[test]
fn pair_helper_preserves_manifest_bytes_and_refuses_mismatched_generated_pairs() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new("python3")
        .args([
            "-c",
            r#"
from pathlib import Path
import runpy, sys, tomllib
root = Path(sys.argv[1])
helpers = runpy.run_path(str(root / 'scripts/docs-pages-prepare.py'))
cli = root / 'crates/rustify-docs-cli'
original = (root / 'tests/docs-ssg/fixtures/components/Cargo.toml').read_bytes()
package = tomllib.loads(original.decode())['package']['name']
embedded = (cli / 'assets/components-Cargo.lock').read_bytes()
packages = tomllib.loads(embedded.decode())['package']
source = next(p['source'] for p in packages if p['name'] == 'rustify-docs')
revision = source.rsplit('#', 1)[1]
generated = (cli / 'templates/components/Cargo.toml.in').read_bytes()
generated = generated.replace(b'{{package}}', package.encode()).replace(b'{{framework-revision}}', revision.encode())
lock = embedded.replace(b'name = "docs-site"\nversion = "0.1.0"\n',
                        f'name = "{package}"\nversion = "0.1.0"\n'.encode(), 1)
project = helpers['project_bytes']
projected = project(original, generated, lock, revision)
restored = projected
for name in helpers['FRAMEWORKS']:
    old = next(line for line in original.splitlines(keepends=True) if line.startswith(f'{name} = '.encode()))
    new = next(line for line in generated.splitlines(keepends=True) if line.startswith(f'{name} = '.encode()))
    restored = restored.replace(new, old, 1)
assert restored == original, 'lib, profiles and every other original byte must survive'
for bad_manifest, bad_lock, label in (
    (generated.replace(revision.encode(), b'0' * 40, 1), lock, 'dependency revision'),
    (generated, lock.replace(source.encode(), source.replace(revision, '0' * 40).encode(), 1), 'lock revision'),
    (generated, lock.replace(f'name = "{package}"\n'.encode(), b'name = "wrong-author"\n', 1), 'root package'),
    (generated, lock.replace(b'name = "wasm-bindgen"\nversion = "0.2.128"', b'name = "wasm-bindgen"\nversion = "0.2.127"', 1), 'binding version'),
):
    try:
        project(original, bad_manifest, bad_lock, revision)
    except ValueError:
        pass
    else:
        raise AssertionError(f'{label} mismatch must refuse projection')
print('pair projection: exact manifest roundtrip and four identity refusals passed')
"#,
        ])
        .arg(repository)
        .output()
        .unwrap();
    succeeded(&output);
}

fn source_version(original: &str, initial: i32, marker: &str) -> (String, String) {
    assert_eq!(
        original.matches(DEMO).count(),
        1,
        "the fixture's direct source function"
    );
    let function = DEMO
        .replacen("        7,", &format!("        {initial},"), 1)
        .replacen("Résumé — a real registered counter", marker, 1);
    let file = format!(
        "\u{feff}{}",
        original.replacen(DEMO, &function, 1).replace('\n', "\r\n")
    );
    (file, function.replace('\n', "\r\n"))
}

fn short(text: &str) -> String {
    text.chars().take(1800).collect()
}

fn diagnostics(report: &Value) -> String {
    report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            format!(
                "{} {}:{}:{} {}",
                error["rule"],
                error["file"],
                error["line"],
                error["column"],
                short(error["message"].as_str().unwrap_or(""))
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn build(temp: &Temp, stage: &str, lock: &[u8], success: bool) -> Value {
    let output = Command::new(BIN)
        .current_dir(temp.path("site"))
        .args(["build", "--format", "json"])
        .env("RUSTIFY_DOCS_CARGO", "mbx")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .output()
        .expect("the real CLI runs");
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "{stage}: stdout is not one JSON response: {error}\n{}\n{}",
            short(&stdout(&output)),
            short(&stderr(&output))
        )
    });
    let evidence = temp.path(&format!("evidence/{stage}"));
    fs::create_dir_all(&evidence).unwrap();
    fs::write(evidence.join("cli.json"), &output.stdout).unwrap();
    fs::write(evidence.join("cli.stderr"), &output.stderr).unwrap();
    for file in [
        "docs-host-release-host.stdout",
        "docs-host-release-host.stderr",
        "release-wasm.stdout",
        "release-wasm.stderr",
    ] {
        let log = temp.path(&format!("site/.rustify-docs/logs/{file}"));
        if log.is_file() {
            fs::copy(log, evidence.join(file)).unwrap();
        }
    }
    assert_eq!(
        fs::read(temp.path("site/Cargo.lock")).unwrap(),
        lock,
        "{stage}: --locked preserves every lock byte"
    );
    assert_eq!(
        output.status.code(),
        Some(i32::from(!success)),
        "{stage}: {}\nstderr: {}",
        diagnostics(&report),
        short(&stderr(&output))
    );
    assert_eq!(
        report["success"],
        success,
        "{stage}: {}",
        diagnostics(&report)
    );
    println!(
        "components/{stage}: success={}, errors={}, compiler={}",
        report["success"], report["summary"]["errors"], report["summary"]["rust_build"]
    );
    report
}

fn diagnostic<'a>(report: &'a Value, rule: &str, file: &str) -> &'a Value {
    report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|error| {
            error["severity"] == "error" && error["rule"] == rule && error["file"] == file
        })
        .unwrap_or_else(|| panic!("missing {rule} at {file}: {}", diagnostics(report)))
}

fn written_line(text: &str, needle: &str) -> u64 {
    (text.lines().position(|line| line.contains(needle)).unwrap() + 1) as u64
}

/// The 1-based line and character column of `token` on the `nth` line
/// containing `needle`.
fn written_at(text: &str, needle: &str, nth: usize, token: &str) -> (u64, u64) {
    let (index, line) = text
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(needle))
        .nth(nth)
        .unwrap();
    let at = line.find(token).unwrap();
    (index as u64 + 1, line[..at].chars().count() as u64 + 1)
}

/// Each error in `file` as (rule, line, column, suggestion), sorted.
fn located_repairs(report: &Value, file: &str) -> Vec<(String, u64, u64, String)> {
    let mut found: Vec<_> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| error["severity"] == "error" && error["file"] == file)
        .map(|error| {
            (
                error["rule"].as_str().unwrap().to_owned(),
                error["line"].as_u64().unwrap(),
                error["column"].as_u64().unwrap(),
                error["suggestion"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect();
    found.sort();
    found
}

fn check(temp: &Temp, args: &[&str]) -> std::process::Output {
    Command::new(BIN)
        .current_dir(temp.path("site"))
        .arg("check")
        .args(args)
        .env("RUSTIFY_DOCS_CARGO", "mbx")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .output()
        .expect("the real CLI runs")
}

fn check_flags(report: &Value) {
    let compiled = &report["summary"]["rust_build"];
    assert_eq!(compiled["generation"].as_str().unwrap().len(), 64);
    for (target, profile, feature) in [
        ("host", "docs-host-release", "ssr"),
        ("wasm", "release", "hydrate"),
    ] {
        let timing = &compiled[target];
        assert_eq!(timing["success"], true);
        assert_eq!(timing["profile"], profile);
        assert_eq!(timing["compiler"], "mbx");
        assert!(timing["completed_at_ms"].as_u64().unwrap() > 0);
        let flags: Vec<_> = timing["flags"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert!(flags.contains(&"--locked"));
        assert!(flags.windows(2).any(|pair| pair == ["--features", feature]));
        assert!(flags.windows(2).any(|pair| pair == ["--profile", profile]));
        if target == "host" {
            assert!(flags
                .windows(2)
                .any(|pair| pair == ["--bin", "docs-render"]));
            assert!(timing["target"].is_null());
        } else {
            assert!(flags.contains(&"--lib"));
            assert!(flags
                .windows(2)
                .any(|pair| pair == ["--target", "wasm32-unknown-unknown"]));
        }
    }
    assert!(
        compiled["wasm"]["completed_at_ms"].as_u64().unwrap()
            >= compiled["host"]["completed_at_ms"].as_u64().unwrap()
    );
}

fn both_targets_failed(temp: &Temp) {
    for (file, feature) in [
        ("docs-host-release-host.stdout", "site-ssr"),
        ("release-wasm.stdout", "hydrate"),
    ] {
        let messages: Vec<Value> = temp
            .read(&format!("site/.rustify-docs/logs/{file}"))
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();
        assert!(messages.iter().any(|message| message["reason"] == "build-finished" && message["success"] == false), "{file}: actual Cargo failure");
        assert!(
            messages
                .iter()
                .any(|message| message["reason"] == "compiler-message"
                    && message["message"]["code"]["code"] == "E0308"),
            "{file}: the source's actual type error"
        );
        assert!(
            messages
                .iter()
                .any(|message| message["reason"] == "compiler-artifact"
                    && message["target"]["name"] == "rustify_docs"
                    && message["features"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|flag| flag == feature)),
            "{file}: the expected target feature graph ran"
        );
    }
}

#[cfg(unix)]
fn overlapping_describe(temp: &Temp, lock: &[u8], good: &BTreeMap<String, Vec<u8>>) {
    use std::os::unix::fs::PermissionsExt;
    let barrier = temp.path("helper-barrier");
    fs::create_dir_all(&barrier).unwrap();
    // The helper and wasm compiler must each arrive before either continues.
    // A synchronous describe after wasm would time out at this barrier.
    temp.write(
        "site/src/bin/docs-render.rs",
        r#"fn main() {
    let barrier = std::env::var_os("RUSTIFY_HELPER_BARRIER").map(std::path::PathBuf::from);
    if let Some(directory) = &barrier {
        let started = std::time::Instant::now();
        while !directory.join("wasm-started").exists() {
            assert!(started.elapsed().as_secs() < 15, "wasm did not reach helper barrier");
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        std::fs::write(directory.join("helper-started"), b"started").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let result = rustify_docs::serve_site(docs_components_fixture::site());
    if let Some(directory) = &barrier {
        std::fs::write(directory.join("helper-finished"), b"finished").unwrap();
    }
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
"#,
    );
    let compiler = temp.path("cargo-barrier");
    fs::write(
        &compiler,
        r#"#!/bin/sh
if [ "$1" = build ]; then
    case " $* " in
        *" --target wasm32-unknown-unknown "*)
            printf '%s\n' wasm-start >> "$RUSTIFY_HELPER_BARRIER/order"
            touch "$RUSTIFY_HELPER_BARRIER/wasm-started"
            rounds=0
            while [ ! -f "$RUSTIFY_HELPER_BARRIER/helper-started" ]; do
                rounds=$((rounds + 1))
                [ "$rounds" -lt 150 ] || exit 76
                sleep 0.1
            done
            [ ! -f "$RUSTIFY_HELPER_BARRIER/fail-wasm" ] || exit 77
            mbx "$@"
            status=$?
            printf '%s\n' wasm-end >> "$RUSTIFY_HELPER_BARRIER/order"
            exit "$status"
            ;;
        *)
            printf '%s\n' host-start >> "$RUSTIFY_HELPER_BARRIER/order"
            mbx "$@"
            status=$?
            printf '%s\n' host-end >> "$RUSTIFY_HELPER_BARRIER/order"
            exit "$status"
            ;;
    esac
fi
exec mbx "$@"
"#,
    )
    .unwrap();
    fs::set_permissions(&compiler, fs::Permissions::from_mode(0o755)).unwrap();
    let run = |failed: bool| {
        let output = Command::new(BIN)
            .current_dir(temp.path("site"))
            .args(["build", "--format", "json"])
            .env("RUSTIFY_DOCS_CARGO", &compiler)
            .env("RUSTIFY_HELPER_BARRIER", &barrier)
            .output()
            .unwrap();
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            output.status.code(),
            Some(i32::from(failed)),
            "{}",
            diagnostics(&report)
        );
        assert!(
            barrier.join("helper-finished").exists(),
            "the describe worker must be joined before CLI exit"
        );
        assert_eq!(fs::read(temp.path("site/Cargo.lock")).unwrap(), lock);
        report
    };
    let report = run(false);
    assert_eq!(report["success"], true);
    let order = fs::read_to_string(barrier.join("order")).unwrap();
    assert_eq!(order, "host-start\nhost-end\nwasm-start\nwasm-end\n");
    let overlapped = tree(&temp.path("site/dist"));
    for (path, bytes) in good.iter().filter(|(path, _)| path.starts_with("assets/")) {
        assert_eq!(
            overlapped.get(path),
            Some(bytes),
            "helper preparation preserves reader resource `{path}`"
        );
    }
    for file in ["helper-started", "helper-finished", "wasm-started", "order"] {
        fs::remove_file(barrier.join(file)).unwrap();
    }
    fs::write(barrier.join("fail-wasm"), b"fail").unwrap();
    let report = run(true);
    assert_eq!(report["success"], false);
    assert!(diagnostics(&report).contains("wasm Cargo build failed"));
    assert_eq!(
        tree(&temp.path("site/dist")),
        overlapped,
        "wasm failure cannot publish the worker's partial result"
    );
}

fn published_source(
    files: &BTreeMap<String, Vec<u8>>,
    file: &str,
    function: &str,
    initial: i32,
    marker: &str,
    project_root: &Path,
) -> String {
    let html = text_of(files, "counter/index.html");
    let document = Document::from(html.as_str());
    let source = document.select(".rd-demo-source");
    assert_eq!(source.length(), 1);
    assert_eq!(
        source
            .select("[data-rd-source-path]")
            .attr("data-rd-source-path")
            .as_deref(),
        Some("src/demos.rs")
    );
    let raw: Vec<String> = source
        .select("template.rd-code-raw")
        .nodes()
        .iter()
        .map(|node| {
            let content = node.element_ref().unwrap().template_contents.unwrap();
            node.tree.get(&content).unwrap().text().to_string()
        })
        .collect();
    assert_eq!(
        raw,
        [function, file],
        "original UTF-8, BOM and CRLF bytes survive the raw templates"
    );
    assert!(document.select(".rd-demo-canvas").text().contains(marker));
    let initial_text = initial.to_string();
    assert!(document
        .select(".rd-demo-canvas [data-testid=counter-value]")
        .iter()
        .any(|value| value.text().as_ref() == initial_text.as_str()));
    assert!(
        !html.contains(project_root.to_str().unwrap()),
        "published HTML carries project-relative provenance"
    );
    assert!(
        !files.contains_key("src/demos.rs"),
        "raw source is not published as a separate file"
    );
    let wasm: Vec<_> = files
        .iter()
        .filter(|(path, _)| path.starts_with("assets/islands/") && path.ends_with("/app_bg.wasm"))
        .collect();
    assert_eq!(wasm.len(), 1);
    let (path, bytes) = wasm[0];
    assert!(bytes.starts_with(b"\0asm"));
    assert!(
        bytes
            .windows(marker.len())
            .any(|part| part == marker.as_bytes()),
        "the same source marker is linked into the actual wasm"
    );
    let manifest: Value = serde_json::from_slice(&files["rustify-docs-manifest.json"]).unwrap();
    let entry = manifest["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["path"] == path.as_str())
        .unwrap();
    assert_eq!(entry["sha256"], sha256(bytes));
    let group = path.strip_suffix("app_bg.wasm").unwrap();
    for required in ["app.js", "dom-loader.js", "dom-runtime.css", "islands.js"] {
        assert!(files.contains_key(&format!("{group}{required}")));
    }
    assert!(files
        .keys()
        .any(|path| path.starts_with(&format!("{group}snippets/"))
            && path.ends_with("web/dom-runtime.js")));
    sha256(bytes)
}

#[test]
fn real_component_builds_report_independent_errors_preserve_good_bytes_and_recover_one_generation()
{
    let temp = project();
    let site = temp.path("site");
    let original = temp.read("site/src/demos.rs");
    let lock = fs::read(site.join("Cargo.lock")).unwrap();
    let marker_a = "M2 source A — café Ω <&>";
    let (file_a, function_a) = source_version(&original, 7, marker_a);
    temp.write("site/src/demos.rs", &file_a);

    let first = build(&temp, "A-good", &lock, true);
    check_flags(&first);
    let good = tree(&site.join("dist"));
    // `check` compiles the development profiles; warm them while the site is
    // good. Under mbx a project's first development generation can find
    // build-script outputs among its inputs and discard itself, and only then.
    let warm = check(&temp, &["--format", "json"]);
    if warm.status.code() != Some(0) {
        let report: Value = serde_json::from_slice(&warm.stdout).unwrap();
        let errors: Vec<_> = report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|error| error["severity"] == "error")
            .map(|error| error["rule"].as_str().unwrap())
            .collect();
        assert_eq!(
            errors,
            ["rust-generation-changed"],
            "{}",
            diagnostics(&report)
        );
        let again = check(&temp, &["--format", "json"]);
        let report: Value = serde_json::from_slice(&again.stdout).unwrap();
        assert_eq!(again.status.code(), Some(0), "{}", diagnostics(&report));
    }
    let wasm_a = published_source(&good, &file_a, &function_a, 7, marker_a, &site);

    let registry = temp.read("site/src/lib.rs");
    assert_eq!(registry.matches(REGISTRY_END).count(), 1);
    temp.write(
        "site/src/lib.rs",
        &registry.replacen(
            REGISTRY_END,
            &format!("            Tabs {{ tabs: Vec<String> }} => |props, children| demos::counter(props.tabs.len() as i32, \"Reserved\".to_string(), children),\n{REGISTRY_END}"),
            1,
        ),
    );
    temp.write("site/content/errors.md", COLD_ERRORS);
    temp.write(
        "site/src/demos.rs",
        &format!(
            "{file_a}\r\npub fn integration_compile_error() -> i32 {{ \"not an integer\" }}\r\n"
        ),
    );
    let failed = build(&temp, "B-cold-and-rust-errors", &lock, false);
    let field = diagnostic(&failed, "unknown-property", "content/errors.md");
    assert_eq!(
        (field["line"].as_u64(), field["column"].as_u64()),
        (Some(written_line(COLD_ERRORS, "unexpected:")), Some(3))
    );
    assert_eq!(
        field["suggestion"],
        "Use one of `initial` (`i32`; default expression `2`), `label` (`String`; default expression `\"Counter\".to_string()`) for `Counter`; rename or remove `unexpected`."
    );
    let unknown = diagnostic(&failed, "unregistered-component", "content/errors.md");
    assert_eq!(
        unknown["line"],
        written_line(COLD_ERRORS, "name: UnregisteredCounter") - 1
    );
    assert_eq!(unknown["column"], 1);
    assert_eq!(
        unknown["suggestion"],
        "Use a component from `BrowserCounter`, `CardGrid`, `Counter`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`."
    );
    let demo = diagnostic(&failed, "unregistered-demo", "content/errors.md");
    assert_eq!(
        (demo["line"].as_u64(), demo["column"].as_u64()),
        (
            Some(written_line(COLD_ERRORS, "name: MissingDemo") - 1),
            Some(1)
        )
    );
    assert_eq!(
        demo["suggestion"],
        "Use a demo from `AsyncAfterAttachDemo`, `CatalogProxy`, `CounterDemo`, `FailureDemo`, `ThemeProbeDemo`; register demos in `site!`."
    );
    let reserved = diagnostic(&failed, "reserved-component", "src/lib.rs");
    assert_eq!(
        (reserved["line"].as_u64(), reserved["column"].as_u64()),
        (None, None)
    );
    assert_eq!(
        reserved["suggestion"],
        "Rename `Tabs` in `site!` and its Markdown references; builtin component names are reserved."
    );
    let rust = diagnostic(&failed, "rust-compile", "src/demos.rs");
    assert!(rust["line"].as_u64().unwrap() > written_line(&file_a, "pub fn counter_demo"));
    assert!(rust["column"].as_u64().unwrap() > 0);
    both_targets_failed(&temp);
    assert_eq!(
        tree(&site.join("dist")),
        good,
        "compile errors leave the whole successful output byte-for-byte"
    );

    temp.write("site/src/demos.rs", &file_a);
    temp.write(
        "site/src/lib.rs",
        &format!("{registry}{AUTHOR_TYPES}").replacen(
            REGISTRY_END,
            &format!("            Swatch {{ palette: crate::Palette, labels: crate::Labels = Vec::new(), limit: Option<u32> }} =>\n                |props, children| demos::counter(props.limit.unwrap_or(0) as i32 + props.labels.len() as i32, props.palette.accent, children),\n{REGISTRY_END}"),
            1,
        ),
    );
    temp.write("site/content/errors.md", DATA_ERRORS);
    temp.write("site/content/errors-zh.md", ZH_CRLF_ERRORS);
    let failed = build(&temp, "C-props-and-yaml", &lock, false);
    let typed = diagnostic(&failed, "invalid_props", "content/errors.md");
    assert_eq!(
        (typed["line"].as_u64(), typed["column"].as_u64()),
        (Some(written_line(DATA_ERRORS, "'initial':")), Some(3))
    );
    let duplicate = diagnostic(&failed, "embed-payload", "content/errors.md");
    assert!(duplicate["message"].as_str().unwrap().contains("duplicate"));
    let at = |needle: &str, nth: usize, token: &str| written_at(DATA_ERRORS, needle, nth, token);
    let fence = |nth: usize| (at("name: Swatch", nth, "name").0 - 1, 1);
    let palette = "Check `palette` of declared type `crate::Palette` (no declared default; confirm omission semantics in the registration) in `src/lib.rs`; follow the reported serde requirement.";
    let mut expected = vec![
        ("invalid_props", at("'initial':", 0, "'initial'"), INITIAL_REPAIR.to_owned()),
        ("embed-payload", at("props: {initial: 1", 0, "{"), "Keep each YAML mapping key once; remove or rename the duplicate key.".to_owned()),
        ("invalid_props", at("props: {label: wrong", 0, "initial"), INITIAL_REPAIR.to_owned()),
        ("invalid_props", at("palette: {accent: 3}", 0, "palette"), format!("{palette} Reported props path: `palette.accent`.")),
        ("invalid_props", at("labels: wrong", 0, "labels"), "Check `labels` of declared type `crate::Labels` (default expression `Vec::new()`) in `src/lib.rs`; follow the reported serde requirement. Reported props path: `labels`.".to_owned()),
        ("invalid_props", fence(2), "Add or correct `props` using the declared fields `labels` (`crate::Labels`; default expression `Vec::new()`), `limit` (`Option<u32>`; optional; omission is null), `palette` (`crate::Palette`; no declared default; confirm omission semantics in the registration); check the registration in `src/lib.rs`.".to_owned()),
    ]
    .into_iter()
    .map(|(rule, (line, column), repair)| (rule.to_owned(), line, column, repair))
    .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(located_repairs(&failed, "content/errors.md"), expected);
    let (line, column) = written_at(ZH_CRLF_ERRORS, "props: {label: 中文标签", 0, "initial");
    assert_eq!(
        located_repairs(&failed, "content/errors-zh.md"),
        [(
            "invalid_props".to_owned(),
            line,
            column,
            INITIAL_REPAIR.to_owned()
        )]
    );
    assert_eq!(
        tree(&site.join("dist")),
        good,
        "data errors cannot publish a partial generation"
    );
    // `check` reports the same refusals with the same repairs, as JSON and
    // as text, from its own development generation.
    let checked = check(&temp, &["--format", "json"]);
    assert_eq!(checked.status.code(), Some(1));
    let checked: Value = serde_json::from_slice(&checked.stdout).unwrap();
    for file in ["content/errors.md", "content/errors-zh.md"] {
        assert_eq!(
            located_repairs(&checked, file),
            located_repairs(&failed, file),
            "{file}: {}",
            diagnostics(&checked)
        );
    }
    let text = String::from_utf8(check(&temp, &[]).stderr).unwrap();
    for file in ["content/errors.md", "content/errors-zh.md"] {
        for (_, line, column, repair) in located_repairs(&failed, file) {
            let located = format!("  --> {file}:{line}:{column}\n  = help: {repair}\n");
            assert!(text.contains(&located), "{located}\n{text}");
        }
    }
    fs::remove_file(site.join("content/errors-zh.md")).unwrap();
    temp.write("site/src/lib.rs", &registry);

    fs::remove_file(site.join("content/errors.md")).unwrap();
    let marker_b = "M2 source B — réparé λ <&>";
    let (file_b, function_b) = source_version(&original, 31, marker_b);
    temp.write("site/src/demos.rs", &file_b);
    let repaired = build(&temp, "D-repaired", &lock, true);
    check_flags(&repaired);
    let recovered = tree(&site.join("dist"));
    let wasm_b = published_source(&recovered, &file_b, &function_b, 31, marker_b, &site);
    assert_ne!(
        wasm_b, wasm_a,
        "the recovered source must be linked into a new wasm"
    );
    assert_ne!(
        repaired["summary"]["rust_build"]["generation"],
        first["summary"]["rust_build"]["generation"]
    );

    let warm = build(&temp, "E-unchanged", &lock, true);
    check_flags(&warm);
    for target in ["host", "wasm"] {
        assert_eq!(
            warm["summary"]["rust_build"][target]["compiled_artifacts"],
            0
        );
        assert!(
            warm["summary"]["rust_build"][target]["fresh_artifacts"]
                .as_u64()
                .unwrap()
                > 0
        );
    }
    assert_eq!(
        tree(&site.join("dist")),
        recovered,
        "the same complete input writes the same complete bytes"
    );
    #[cfg(unix)]
    overlapping_describe(&temp, &lock, &recovered);
}
