use std::path::Path;

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
            read(&git.join("packed-refs"))?.lines().find_map(|line| {
                let (sha, name) = line.split_once(' ')?;
                (name == reference).then(|| sha.to_owned())
            })
        })?
    } else {
        head
    };
    let sha = sha.trim();
    (sha.len() == 40 && sha.bytes().all(|byte| byte.is_ascii_hexdigit())).then(|| sha.to_owned())
}

fn main() {
    // Cargo's git checkout already has the source identity. Reading it
    // avoids requiring a git executable, network access or an author tool.
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default();
    let revision = Path::new(&manifest)
        .ancestors()
        .find(|root| root.join(".git").exists())
        .and_then(revision)
        .unwrap_or_default();
    println!("cargo:rustc-env=RUSTIFY_DOCS_SOURCE_REVISION={revision}");
}
