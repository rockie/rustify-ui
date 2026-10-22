//! The CLI's frozen framework pair and Cargo source boundary.

use serde_json::Value;
use std::path::Path;

#[cfg(test)]
mod source;

pub const FRAMEWORKS: [&str; 3] = ["rustify-docs", "rustify-dom", "rustify-components"];
pub const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";
pub const PAIR_ADVICE: &str = "Upgrade the CLI, all three dependencies and Cargo.lock together. For a 0.1.32 Git project replace rustify-ui with rustify-dom (or alias package = \"rustify-dom\") and use the registry versions in this CLI's --build-info; keep the old manifest, lock and 0.1.32 CLI to roll back.";
const RECORD: &str = include_str!("../assets/framework-release.json");

#[derive(Debug)]
pub struct Framework {
    pub value: Value,
}

impl Framework {
    pub fn parse(text: &str) -> Result<Self, String> {
        let value: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
        if value["schema"] != 1
            || value["registry"] != "crates-io"
            || value["packages"]
                .as_object()
                .is_none_or(|p| p.len() != FRAMEWORKS.len())
            || FRAMEWORKS
                .iter()
                .any(|name| value["packages"][name].as_str().is_none_or(str::is_empty))
            || value["wasm_bindgen"].as_str().is_none_or(str::is_empty)
            || (!value["library_rev"].is_null()
                && value["library_rev"]
                    .as_str()
                    .is_none_or(|r| r.len() != 40 || !r.bytes().all(|b| b.is_ascii_hexdigit())))
        {
            return Err("invalid frozen framework release record".into());
        }
        Ok(Self { value })
    }

    pub fn embedded() -> Result<Self, String> {
        Self::parse(RECORD)
    }

    pub fn version(&self, name: &str) -> &str {
        self.value["packages"][name].as_str().unwrap_or_default()
    }
}

pub fn context() -> Value {
    use sha2::{Digest, Sha256};
    let mut framework = serde_json::from_str::<Value>(RECORD).unwrap_or(Value::Null);
    if env!("RUSTIFY_DOCS_DISTRIBUTION") == "source" {
        framework["development_checkout"] = Value::String(format!(
            "{:x}",
            Sha256::digest(env!("RUSTIFY_DOCS_REPOSITORY").as_bytes())
        ));
    }
    serde_json::json!({"distribution":env!("RUSTIFY_DOCS_DISTRIBUTION"), "framework":framework,
        "font_resolution":"css", "template_lock":format!("{:x}", Sha256::digest(include_bytes!("../assets/components-Cargo.lock")))})
}

pub fn validate(metadata: &Value, author: &str) -> Result<(), String> {
    validate_pair(
        metadata,
        author,
        &Framework::embedded()?,
        env!("RUSTIFY_DOCS_DISTRIBUTION"),
        Some(Path::new(env!("RUSTIFY_DOCS_REPOSITORY"))),
    )
}

fn validate_pair(
    metadata: &Value,
    author: &str,
    record: &Framework,
    distribution: &str,
    repository: Option<&Path>,
) -> Result<(), String> {
    let packages = metadata["packages"]
        .as_array()
        .ok_or("Cargo metadata has no packages")?;
    let direct = metadata
        .pointer("/resolve/nodes")
        .and_then(Value::as_array)
        .and_then(|nodes| nodes.iter().find(|node| node["id"] == author))
        .and_then(|node| node["dependencies"].as_array())
        .ok_or("Cargo metadata has no resolved author dependency graph")?;
    let mut source_mode = None;
    for name in FRAMEWORKS {
        let found: Vec<_> = packages.iter().filter(|p| p["name"] == name).collect();
        let expected = format!("{name}@{} from crates.io", record.version(name));
        let [package] = found.as_slice() else {
            return Err(format!(
                "Expected exactly one {expected} in the resolved graph; found {}. {PAIR_ADVICE}",
                found.len()
            ));
        };
        let actual = format!(
            "{name}@{} from {}",
            package["version"].as_str().unwrap_or("unknown"),
            package["source"].as_str().unwrap_or("path")
        );
        let registry = package["source"].as_str() == Some(CRATES_IO);
        if source_mode.is_some_and(|previous| previous != registry) {
            return Err(format!("Expected {expected} consistently with the other framework packages; found {actual}. {PAIR_ADVICE}"));
        }
        source_mode = Some(registry);
        if !direct.contains(&package["id"]) || package["version"] != record.version(name) {
            return Err(format!(
                "Expected a direct {expected}; found {actual}. {PAIR_ADVICE}"
            ));
        }
        match package["source"].as_str() {
            Some(CRATES_IO) => {}
            None if distribution == "source" && package["source"].is_null() => {
                let root = repository.ok_or("cannot identify this source CLI's checkout")?;
                let expected_path =
                    std::fs::canonicalize(root.join("crates").join(name).join("Cargo.toml"))
                        .map_err(|error| {
                            format!("cannot verify {expected}: {error}. {PAIR_ADVICE}")
                        })?;
                let actual_path = package["manifest_path"]
                    .as_str()
                    .and_then(|path| std::fs::canonicalize(path).ok());
                if actual_path.as_deref() != Some(expected_path.as_path()) {
                    return Err(format!("Expected {expected} or its canonical manifest in this source CLI's checkout; found {actual}. {PAIR_ADVICE}"));
                }
            }
            _ => {
                return Err(format!(
                    "Expected {expected}; found {actual}. {PAIR_ADVICE}"
                ))
            }
        }
    }
    let bindings: Vec<_> = packages
        .iter()
        .filter(|p| p["name"] == "wasm-bindgen")
        .collect();
    if bindings.is_empty()
        || bindings
            .iter()
            .any(|p| p["version"] != record.value["wasm_bindgen"])
    {
        return Err(format!(
            "Expected wasm-bindgen@{}; found {:?}. {PAIR_ADVICE}",
            record.value["wasm_bindgen"],
            bindings.iter().map(|p| &p["version"]).collect::<Vec<_>>()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn validate_pair(
        graph: &Value,
        author: &str,
        record: &Framework,
        identity: &source::Identity,
    ) -> Result<(), String> {
        super::validate_pair(
            graph,
            author,
            record,
            identity.distribution,
            identity.repository.as_deref(),
        )
    }

    fn record() -> Framework {
        Framework::parse(&json!({
            "schema": 1, "registry": "crates-io", "library_rev": "a".repeat(40),
            "wasm_bindgen": "0.2.128",
            "packages": {"rustify-docs":"0.2.0", "rustify-dom":"0.2.0", "rustify-components":"0.2.0"}
        }).to_string()).unwrap()
    }

    fn metadata() -> Value {
        let mut packages: Vec<_> = FRAMEWORKS
            .iter()
            .map(|name| {
                json!({
                    "id": name, "name": name, "version":"0.2.0", "source": CRATES_IO,
                })
            })
            .collect();
        packages.push(json!({"id":"bindings", "name":"wasm-bindgen", "version":"0.2.128", "source":CRATES_IO}));
        json!({"packages":packages,"resolve":{"nodes":[{"id":"author","dependencies":FRAMEWORKS}]}})
    }

    fn registry() -> source::Identity {
        source::Identity {
            revision: "b".repeat(40),
            distribution: "registry",
            repository: None,
        }
    }

    #[test]
    fn registry_pair_accepts_exact_direct_packages_independently_of_aliases() {
        let mut graph = metadata();
        graph["resolve"]["nodes"][0]["deps"] = json!([
            {"name":"docs_alias", "pkg":"rustify-docs"},
            {"name":"ui_alias", "pkg":"rustify-dom"},
            {"name":"controls", "pkg":"rustify-components"}
        ]);
        assert!(validate_pair(&graph, "author", &record(), &registry()).is_ok());
    }

    #[test]
    fn registry_pair_rejects_wrong_versions_sources_indirect_and_duplicate_core() {
        for (field, value) in [
            ("version", json!("0.1.32")),
            (
                "source",
                json!("git+https://github.com/rockie/rustify-ui?rev=old#old"),
            ),
            ("source", Value::Null),
            ("source", json!("registry+https://example.com/index")),
        ] {
            let mut graph = metadata();
            graph["packages"][1][field] = value;
            let error = validate_pair(&graph, "author", &record(), &registry()).unwrap_err();
            assert!(
                error.contains("rustify-dom") && error.contains("0.2.0"),
                "{error}"
            );
        }
        let mut graph = metadata();
        graph["resolve"]["nodes"][0]["dependencies"] =
            json!(["rustify-docs", "rustify-components"]);
        assert!(validate_pair(&graph, "author", &record(), &registry()).is_err());
        let mut graph = metadata();
        let mut duplicate = graph["packages"][1].clone();
        duplicate["id"] = json!("second-dom");
        graph["packages"].as_array_mut().unwrap().push(duplicate);
        assert!(validate_pair(&graph, "author", &record(), &registry()).is_err());
        let mut graph = metadata();
        graph["packages"][3]["version"] = json!("0.2.127");
        assert!(validate_pair(&graph, "author", &record(), &registry()).is_err());
    }

    struct Fixture(std::path::PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "rustify-release-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn write(&self, path: &str, bytes: &str) {
            let path = self.0.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, bytes).unwrap();
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn registry_pair_development_requires_source_distribution_and_canonical_checkout() {
        let fixture = Fixture::new();
        let mut graph = metadata();
        for (index, name) in FRAMEWORKS.iter().enumerate() {
            let path = format!("crates/{name}/Cargo.toml");
            fixture.write(&path, "[package]\n");
            graph["packages"][index]["source"] = Value::Null;
            graph["packages"][index]["manifest_path"] = json!(fixture.0.join(path));
        }
        let identity = source::Identity {
            revision: "b".repeat(40),
            distribution: "source",
            repository: Some(fixture.0.clone()),
        };
        assert!(validate_pair(&graph, "author", &record(), &identity).is_ok());
        assert!(validate_pair(&graph, "author", &record(), &registry()).is_err());
        fixture.write("other/Cargo.toml", "[package]\n");
        graph["packages"][1]["manifest_path"] = json!(fixture.0.join("other/Cargo.toml"));
        assert!(validate_pair(&graph, "author", &record(), &identity).is_err());
    }

    #[test]
    fn archive_identity_precedes_git_and_does_not_leak_an_ancestor_checkout() {
        let fixture = Fixture::new();
        fixture.write(".git/HEAD", &"c".repeat(40));
        fixture.write("crates/rustify-docs-cli/Cargo.toml", "[package]\n");
        let manifest = fixture.0.join("crates/rustify-docs-cli");
        let identity = source::identify(&manifest);
        assert_eq!(identity.distribution, "source");
        assert_eq!(identity.revision, "c".repeat(40));
        fixture.write(
            "crates/rustify-docs-cli/.cargo_vcs_info.json",
            &json!({"git":{"sha1":"a".repeat(40)}}).to_string(),
        );
        let identity = source::identify(&manifest);
        assert_eq!(identity.distribution, "registry");
        assert_eq!(identity.revision, "a".repeat(40));
        assert!(identity.repository.is_none());
        fixture.write("unpacked/rustify-docs-cli/Cargo.toml", "[package]\n");
        assert_eq!(
            source::identify(&fixture.0.join("unpacked/rustify-docs-cli")).revision,
            "unknown"
        );
        fixture.write("crates/rustify-docs-cli/.cargo_vcs_info.json", "not JSON");
        assert_eq!(source::identify(&manifest).revision, "unknown");
    }
}
