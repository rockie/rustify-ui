//! Source identity shared by the build script and its tests.

use std::path::{Path, PathBuf};

pub struct Identity {
    pub revision: String,
    pub distribution: &'static str,
    pub repository: Option<PathBuf>,
}

pub fn is_revision(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn read(path: &Path) -> Option<String> {
    println!("cargo:rerun-if-changed={}", path.display());
    std::fs::read_to_string(path).ok()
}

fn revision(root: &Path) -> Option<String> {
    let git = root.join(".git");
    let git = if git.is_file() {
        root.join(read(&git)?.trim().strip_prefix("gitdir: ")?)
    } else {
        git
    };
    let head = read(&git.join("HEAD"))?;
    let sha = if let Some(reference) = head.trim().strip_prefix("ref: ") {
        read(&git.join(reference)).or_else(|| {
            // Linked worktrees store shared refs in their common directory.
            let common = read(&git.join("commondir"))
                .map(|path| git.join(path.trim()))
                .unwrap_or_else(|| git.clone());
            read(&common.join(reference)).or_else(|| {
                read(&common.join("packed-refs"))?.lines().find_map(|line| {
                    let (sha, name) = line.split_once(' ')?;
                    (name == reference).then(|| sha.to_owned())
                })
            })
        })?
    } else {
        head
    };
    let sha = sha.trim();
    is_revision(sha).then(|| sha.to_owned())
}

pub fn identify(manifest: &Path) -> Identity {
    let canonical = manifest
        .canonicalize()
        .unwrap_or_else(|_| manifest.to_path_buf());
    let manifest = canonical.as_path();
    let vcs = manifest.join(".cargo_vcs_info.json");
    println!("cargo:rerun-if-changed={}", vcs.display());
    if vcs.exists() {
        let sha = read(&vcs)
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
            .and_then(|value| value.pointer("/git/sha1")?.as_str().map(str::to_owned))
            .filter(|sha| is_revision(sha));
        return Identity {
            revision: sha.unwrap_or_else(|| "unknown".into()),
            distribution: "registry",
            repository: None,
        };
    }
    // Only the actual workspace layout is a source checkout. An archive's
    // unrelated ancestor .git must never authorize local framework paths.
    let repository = manifest.parent().and_then(Path::parent).filter(|root| {
        root.join(".git").exists()
            && manifest
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|name| name == "crates")
            && manifest
                .file_name()
                .is_some_and(|name| name == "rustify-docs-cli")
    });
    let sha = repository.and_then(revision);
    Identity {
        revision: sha.clone().unwrap_or_else(|| "unknown".into()),
        distribution: if sha.is_some() { "source" } else { "unknown" },
        repository: repository.filter(|_| sha.is_some()).map(Path::to_path_buf),
    }
}
