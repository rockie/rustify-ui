//! `cargo xtask doctor`: reports the fixed toolchain and the repository's
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
        wasm_bindgen_lock(&root),
        leptos_lock(&root),
        sources_lock(&root),
        vendored(&root),
        licenses(&root),
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
        let text =
            std::fs::read_to_string(root.join("rust-toolchain.toml")).map_err(|e| e.to_string())?;
        let channel = text
            .lines()
            .find_map(|line| line.trim().strip_prefix("channel"))
            .and_then(|rest| rest.split('"').nth(1))
            .ok_or("rust-toolchain.toml has no channel")?
            .to_string();
        let version = capture("rustup", &["run", &channel, "rustc", "--version"])?;
        let components = capture(
            "rustup",
            &["component", "list", "--installed", "--toolchain", &channel],
        )?;
        for needed in ["rust-src", "rust-std-wasm32-unknown-unknown"] {
            if !components.lines().any(|line| line.starts_with(needed)) {
                return Err(format!(
                    "{channel} lacks {needed}; run `rustup component add` for it"
                ));
            }
        }
        Ok(format!("{channel} ({})", version.trim()))
    })();
    Check {
        name: "toolchain",
        result,
    }
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
                "{} imported files recorded; run `cargo xtask sources verify` for drift",
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
                "{}; run `cargo xtask sources verify` for digests",
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
}
