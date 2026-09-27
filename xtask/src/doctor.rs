//! `mbx xtask doctor`: reports the fixed toolchain and the repository's
//! provenance records. It never installs anything.

use std::path::Path;
use std::process::Command;

struct Check {
    name: &'static str,
    result: Result<String, String>,
}

pub fn run() -> Result<(), String> {
    let root = crate::build::repo_root();
    let checks = vec![
        toolchain(&root),
        mbx(&root),
        wasm_bindgen_lock(&root),
        leptos_lock(&root),
        sources_lock(&root),
        vendored(&root),
        licenses(&root),
        tailwind(),
        node(),
        playwright(&root),
        chrome(),
    ];
    let mut failed = 0;
    for check in &checks {
        match &check.result {
            Ok(detail) => println!("ok    {:<18} {detail}", check.name),
            Err(detail) => {
                failed += 1;
                println!("FAIL  {:<18} {detail}", check.name);
            }
        }
    }
    if failed > 0 {
        return Err(format!("{failed} check(s) failed"));
    }
    Ok(())
}

fn toolchain(root: &Path) -> Check {
    let result = (|| {
        let pinned = mise_pin(root, "rust")?;
        let version = capture("rustc", &["--version"])?;
        let version = version.trim();
        if !version.starts_with(&format!("rustc {pinned} ")) {
            return Err(format!(
                "{version} is active but mise.toml pins rust {pinned}; run `mise install`"
            ));
        }
        let sysroot = capture("rustc", &["--print", "sysroot"])?;
        let rustlib = Path::new(sysroot.trim()).join("lib/rustlib");
        if !rustlib.join("wasm32-unknown-unknown").is_dir() {
            return Err(format!(
                "rust {pinned} lacks wasm32-unknown-unknown; run \
                 `rustup target add wasm32-unknown-unknown --toolchain {pinned}`"
            ));
        }
        // Started the way mbx starts it: with no library path, which rustup's
        // proxy (and Cargo, for this very process) would otherwise supply.
        let verbose = capture("rustc", &["-vV"])?;
        let host = verbose
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .ok_or("rustc -vV names no host")?;
        let lld = Command::new(rustlib.join(host).join("bin/rust-lld"))
            .args(["-flavor", "wasm", "--version"])
            .env_remove("DYLD_FALLBACK_LIBRARY_PATH")
            .env_remove("DYLD_LIBRARY_PATH")
            .env_remove("LD_LIBRARY_PATH")
            .output();
        if !lld.is_ok_and(|output| output.status.success()) {
            return Err(
                "rust-lld cannot start on its own, so the wasm link fails under \
                 mbx; run `sh scripts/link-libllvm.sh`"
                    .to_string(),
            );
        }
        Ok(format!("{version}, wasm32-unknown-unknown, rust-lld"))
    })();
    Check {
        name: "toolchain",
        result,
    }
}

/// Every build goes through mbx, so a missing one is a build that cannot run.
fn mbx(root: &Path) -> Check {
    let result = (|| {
        let pinned = mise_pin(root, "mr-boxington")?;
        let version = capture("mbx", &["--version"])?;
        let version = version.trim();
        if version != format!("mbx {pinned}") {
            return Err(format!(
                "{version} is active but mise.toml pins mr-boxington {pinned}; run `mise install`"
            ));
        }
        Ok(version.to_string())
    })();
    Check {
        name: "mbx",
        result,
    }
}

/// The version `mise.toml` pins for `tool`, written either as a bare string
/// or as the `version` of an inline table.
pub fn mise_pin(root: &Path, tool: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(root.join("mise.toml")).map_err(|e| e.to_string())?;
    parse_mise_pin(&text, tool).ok_or_else(|| format!("mise.toml pins no version of {tool}"))
}

fn parse_mise_pin(text: &str, tool: &str) -> Option<String> {
    let value = text.lines().find_map(|line| {
        line.trim()
            .strip_prefix(tool)
            .and_then(|rest| rest.trim_start().strip_prefix('='))
    })?;
    let value = value.trim();
    let quoted = match value.strip_prefix('{') {
        Some(table) => table
            .split("version")
            .nth(1)?
            .trim_start()
            .strip_prefix('=')?,
        None => value,
    };
    quoted.split('"').nth(1).map(str::to_string)
}

fn wasm_bindgen_lock(root: &Path) -> Check {
    let result = (|| {
        let app_lock =
            std::fs::read_to_string(root.join("Cargo.lock")).map_err(|e| e.to_string())?;
        let tool_lock =
            std::fs::read_to_string(root.join("makepad/Cargo.lock")).map_err(|e| e.to_string())?;
        let crate_version = lock_version(&app_lock, "wasm-bindgen")
            .ok_or("wasm-bindgen missing from Cargo.lock")?;
        let cli_version = lock_version(&tool_lock, "wasm-bindgen-cli-support")
            .ok_or("wasm-bindgen-cli-support missing from makepad/Cargo.lock")?;
        if crate_version != cli_version {
            return Err(format!(
                "wasm-bindgen {crate_version} but cli-support {cli_version}; they must match"
            ));
        }
        Ok(format!(
            "wasm-bindgen crate and cli-support both {crate_version}"
        ))
    })();
    Check {
        name: "wasm-bindgen",
        result,
    }
}

fn leptos_lock(root: &Path) -> Check {
    let result = (|| {
        let app_lock =
            std::fs::read_to_string(root.join("Cargo.lock")).map_err(|e| e.to_string())?;
        let version = lock_version(&app_lock, "leptos").ok_or("leptos missing from Cargo.lock")?;
        if version != "0.8.20" {
            return Err(format!("leptos is {version}, expected the pinned 0.8.20"));
        }
        Ok(format!("leptos {version} from crates.io"))
    })();
    Check {
        name: "leptos",
        result,
    }
}

fn sources_lock(root: &Path) -> Check {
    let path = root.join("sources.lock.json");
    let result = std::fs::read_to_string(&path)
        .map_err(|e| format!("{}: {e}", path.display()))
        .and_then(|text| {
            serde_json::from_str::<serde_json::Value>(&text).map_err(|e| e.to_string())
        })
        .and_then(|value| {
            let files = value["makepad"]["files"]
                .as_object()
                .ok_or("sources.lock.json has no makepad.files")?;
            Ok(format!(
                "{} imported files recorded; run `mbx xtask sources verify` for drift",
                files.len()
            ))
        });
    Check {
        name: "sources.lock.json",
        result,
    }
}

/// The third-party browser files are part of the checkout, at one version, with
/// their licence beside them. A build never fetches them.
fn vendored(root: &Path) -> Check {
    let path = root.join("sources.lock.json");
    let result = std::fs::read_to_string(&path)
        .map_err(|e| format!("{}: {e}", path.display()))
        .and_then(|text| {
            serde_json::from_str::<serde_json::Value>(&text).map_err(|e| e.to_string())
        })
        .and_then(|value| {
            let entries = value["vendor"]
                .as_array()
                .ok_or("sources.lock.json has no vendor list")?
                .clone();
            let mut named = Vec::new();
            for entry in &entries {
                let name = entry["name"].as_str().unwrap_or("?");
                let version = entry["version"].as_str().unwrap_or("?");
                let files = entry["files"]
                    .as_object()
                    .ok_or_else(|| format!("vendor {name} has no files"))?;
                for file in files.keys() {
                    if !root.join(file).is_file() {
                        return Err(format!("vendor {name} {version}: {file} is missing"));
                    }
                }
                named.push(format!("{name} {version}"));
            }
            Ok(format!(
                "{}; run `mbx xtask sources verify` for digests",
                named.join(", ")
            ))
        });
    Check {
        name: "vendored",
        result,
    }
}

fn licenses(root: &Path) -> Check {
    let required = [
        "makepad/LICENSE",
        "makepad/widgets/resources/FONT-LICENSES.md",
        "examples/property-workbench/vendor/nouislider/LICENSE.md",
        "crates/rustify-components/LICENSE-RUST-UI",
    ];
    let missing: Vec<&str> = required
        .iter()
        .copied()
        .filter(|path| !root.join(path).is_file())
        .collect();
    Check {
        name: "licenses",
        result: if missing.is_empty() {
            Ok(required.join(", "))
        } else {
            Err(format!("missing {}", missing.join(", ")))
        },
    }
}

/// The Tailwind CLI that generates the component stylesheet. mise pins the
/// standalone binary, so it needs no `npm ci`.
fn tailwind() -> Check {
    Check {
        name: "tailwind",
        result: crate::tailwind::cli().map(|cli| {
            format!(
                "tailwindcss v{} at {}",
                crate::tailwind::VERSION,
                cli.display()
            )
        }),
    }
}

fn node() -> Check {
    Check {
        name: "node",
        result: capture("node", &["--version"]).map(|v| v.trim().to_string()),
    }
}

fn playwright(root: &Path) -> Check {
    let result = (|| {
        let package =
            std::fs::read_to_string(root.join("package.json")).map_err(|e| e.to_string())?;
        let pinned = package
            .lines()
            .find(|line| line.contains("\"@playwright/test\""))
            .and_then(|line| line.split('"').nth(3))
            .ok_or("package.json does not pin @playwright/test")?
            .to_string();
        if !root.join("node_modules/@playwright/test").is_dir() {
            return Err(format!(
                "@playwright/test {pinned} pinned but not installed; run `npm ci`"
            ));
        }
        Ok(format!("@playwright/test {pinned} installed"))
    })();
    Check {
        name: "playwright",
        result,
    }
}

fn chrome() -> Check {
    let binary = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
    let result = if Path::new(binary).is_file() {
        capture(binary, &["--version"]).map(|v| v.trim().to_string())
    } else {
        Err("Google Chrome not found in /Applications (the P1 pass gate browser)".to_string())
    };
    Check {
        name: "chrome",
        result,
    }
}

fn capture(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run {program}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Version of `name` in a Cargo.lock, by its `[[package]]` block.
pub fn lock_version(lock: &str, name: &str) -> Option<String> {
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line.trim() == format!("name = \"{name}\"") {
            return lines
                .next()
                .and_then(|next| next.trim().strip_prefix("version = \""))
                .and_then(|rest| rest.strip_suffix('"'))
                .map(str::to_string);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_version_reads_the_package_block() {
        let lock = "[[package]]\nname = \"wasm-bindgen\"\nversion = \"0.2.128\"\n\n[[package]]\nname = \"wasm-bindgen-shared\"\nversion = \"0.2.128\"\n";
        assert_eq!(
            lock_version(lock, "wasm-bindgen").as_deref(),
            Some("0.2.128")
        );
        assert_eq!(
            lock_version(lock, "wasm-bindgen-shared").as_deref(),
            Some("0.2.128")
        );
        assert_eq!(lock_version(lock, "leptos"), None);
    }

    #[test]
    fn mise_pin_reads_a_string_and_an_inline_table() {
        let text = "[tools]\nmr-boxington = \"1.15.0\"\nrust = { version = \"1.98.1\", mr_boxington = true, targets = \"wasm32-unknown-unknown\" }\n";
        assert_eq!(parse_mise_pin(text, "rust").as_deref(), Some("1.98.1"));
        assert_eq!(
            parse_mise_pin(text, "mr-boxington").as_deref(),
            Some("1.15.0")
        );
        assert_eq!(parse_mise_pin(text, "node"), None);
    }
}
