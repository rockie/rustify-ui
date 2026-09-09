//! `sources.lock.json`: the frozen record of what was imported into
//! `makepad/` and how Leptos is pinned. Verification reports drift between the
//! frozen fingerprints and the working tree; drift is expected (the fork is
//! first-class source) but must always be attributable.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Deserialize)]
struct SourcesLock {
    makepad: MakepadSection,
    #[serde(default)]
    vendor: Vec<VendorSection>,
    #[serde(default)]
    rust_ui: Option<RustUiSection>,
}

/// One fixed-version third-party browser file set, imported whole so a fresh
/// checkout builds without reaching a registry.
#[derive(Deserialize)]
pub struct VendorSection {
    pub name: String,
    pub version: String,
    pub files: BTreeMap<String, String>,
}

/// The Rust/UI subset, imported file by file.
///
/// A file arrives verbatim and is rewritten in the commit after, so the digest
/// recorded here is of what was *received*: it identifies the origin whether or
/// not our copy still matches it. `state` says which of the two a file is, and
/// only a file still claiming to be verbatim is checked against the digest -
/// a rewritten one that matched would mean the rewrite never happened.
#[derive(Deserialize)]
pub struct RustUiSection {
    pub origin: String,
    pub files: BTreeMap<String, RustUiFile>,
}

#[derive(Deserialize)]
pub struct RustUiFile {
    pub source: String,
    pub sha256: String,
    pub state: String,
}

#[derive(Deserialize)]
struct MakepadSection {
    import_root: String,
    file_list_fingerprint: String,
    files: BTreeMap<String, String>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Drift {
    pub modified: Vec<String>,
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

pub fn drift(frozen: &BTreeMap<String, String>, current: &BTreeMap<String, String>) -> Drift {
    let mut out = Drift::default();
    for (path, digest) in frozen {
        match current.get(path) {
            Some(now) if now == digest => {}
            Some(_) => out.modified.push(path.clone()),
            None => out.removed.push(path.clone()),
        }
    }
    for path in current.keys() {
        if !frozen.contains_key(path) {
            out.added.push(path.clone());
        }
    }
    out
}

/// One digest over the whole sorted list, so the lock can vouch for itself.
pub fn list_fingerprint(files: &BTreeMap<String, String>) -> String {
    let mut hasher = Sha256::new();
    for (path, digest) in files {
        hasher.update(path.as_bytes());
        hasher.update(b" ");
        hasher.update(digest.as_bytes());
        hasher.update(b"\n");
    }
    hex(&hasher.finalize())
}

pub fn fingerprint_tree(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = entry.file_name();
            if path.is_dir() {
                if name != "target" {
                    stack.push(path);
                }
                continue;
            }
            if name == ".DS_Store" {
                continue;
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            out.insert(relative, hex(&Sha256::digest(&bytes)));
        }
    }
    Ok(out)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn run(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("verify") => verify(),
        _ => Err("usage: cargo xtask sources verify".to_string()),
    }
}

fn verify() -> Result<(), String> {
    let root = crate::build::repo_root();
    let lock_path = root.join("sources.lock.json");
    let text =
        std::fs::read_to_string(&lock_path).map_err(|e| format!("{}: {e}", lock_path.display()))?;
    let lock: SourcesLock =
        serde_json::from_str(&text).map_err(|e| format!("sources.lock.json: {e}"))?;
    let makepad = lock.makepad;
    let self_check = list_fingerprint(&makepad.files);
    if self_check != makepad.file_list_fingerprint {
        return Err(format!(
            "sources.lock.json file list fingerprint {} does not match its entries ({})",
            makepad.file_list_fingerprint, self_check
        ));
    }
    let current = fingerprint_tree(&root.join(&makepad.import_root))?;
    let drift = drift(&makepad.files, &current);
    println!(
        "makepad import: {} frozen files, {} modified, {} added, {} removed since import",
        makepad.files.len(),
        drift.modified.len(),
        drift.added.len(),
        drift.removed.len()
    );
    for path in &drift.modified {
        println!("  modified {path}");
    }
    for path in &drift.added {
        println!("  added    {path}");
    }
    for path in &drift.removed {
        println!("  removed  {path}");
    }
    let mut wrong = Vec::new();
    for vendored in &lock.vendor {
        let mut current = BTreeMap::new();
        for path in vendored.files.keys() {
            let bytes = std::fs::read(root.join(path))
                .map_err(|e| format!("{path}: {e}; a vendored file is part of the checkout"))?;
            current.insert(path.clone(), hex(&Sha256::digest(&bytes)));
        }
        let changed = self::drift(&vendored.files, &current);
        println!(
            "vendor {} {}: {} files, {} modified",
            vendored.name,
            vendored.version,
            vendored.files.len(),
            changed.modified.len()
        );
        for path in &changed.modified {
            println!("  modified {path}");
            wrong.push(path.clone());
        }
    }
    if !wrong.is_empty() {
        // A modified fork is ordinary; a modified third-party file is not that
        // version any more, and the version is the whole claim.
        return Err(format!(
            "vendored files no longer match the version they were imported at: {}",
            wrong.join(", ")
        ));
    }

    if let Some(rust_ui) = &lock.rust_ui {
        let mut unmarked = Vec::new();
        let (mut verbatim, mut rewritten) = (0, 0);
        for (path, file) in &rust_ui.files {
            let bytes = std::fs::read(root.join(path))
                .map_err(|e| format!("{path}: {e}; an imported file is part of the checkout"))?;
            let same = hex(&Sha256::digest(&bytes)) == file.sha256;
            match (file.state.as_str(), same) {
                ("verbatim", true) => verbatim += 1,
                ("verbatim", false) => {
                    unmarked.push(format!("{path} changed but is still recorded as verbatim"));
                }
                ("rewritten", false) => rewritten += 1,
                ("rewritten", true) => {
                    unmarked.push(format!("{path} is recorded as rewritten but is unchanged"));
                }
                (other, _) => unmarked.push(format!("{path} has unknown state {other}")),
            }
        }
        println!(
            "rust_ui {}: {} files, {verbatim} verbatim, {rewritten} rewritten",
            rust_ui.origin,
            rust_ui.files.len()
        );
        if !unmarked.is_empty() {
            // The record's only job is to say which files are still the
            // original and which are ours. A wrong mark makes it useless.
            return Err(format!(
                "the Rust/UI import record disagrees with the checkout: {}",
                unmarked.join("; ")
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lock(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn unchanged_tree_reports_no_drift() {
        let frozen = lock(&[("a.rs", "1"), ("b.rs", "2")]);
        let current = lock(&[("a.rs", "1"), ("b.rs", "2")]);
        assert_eq!(drift(&frozen, &current), Drift::default());
    }

    #[test]
    fn modified_added_and_removed_files_are_classified() {
        let frozen = lock(&[("a.rs", "1"), ("b.rs", "2"), ("gone.rs", "3")]);
        let current = lock(&[("a.rs", "1"), ("b.rs", "changed"), ("new.rs", "4")]);
        let d = drift(&frozen, &current);
        assert_eq!(d.modified, vec!["b.rs".to_string()]);
        assert_eq!(d.added, vec!["new.rs".to_string()]);
        assert_eq!(d.removed, vec!["gone.rs".to_string()]);
    }

    #[test]
    fn list_fingerprint_is_order_independent_and_content_sensitive() {
        let a = list_fingerprint(&lock(&[("a", "1"), ("b", "2")]));
        let b = list_fingerprint(&lock(&[("b", "2"), ("a", "1")]));
        let c = list_fingerprint(&lock(&[("a", "1"), ("b", "3")]));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.len(), 64);
    }
}
