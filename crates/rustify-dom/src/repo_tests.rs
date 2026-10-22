//! Repository-only corpus availability check.
use std::io::Read;

#[test]
fn repo_corpus_sentinel() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in [
        "crates/rustify-components/css/sdk.css",
        "crates/rustify-components/css/theme-v4.css",
        "docs/theme-document.schema.json",
    ] {
        let path = root.join(file);
        let mut input = std::fs::File::open(&path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        input
            .read_exact(&mut [0])
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }
}
