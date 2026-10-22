#[path = "src/release/source.rs"]
mod source;

fn main() {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default();
    let identity = source::identify(std::path::Path::new(&manifest));
    println!(
        "cargo:rustc-env=RUSTIFY_DOCS_SOURCE_REVISION={}",
        identity.revision
    );
    println!(
        "cargo:rustc-env=RUSTIFY_DOCS_DISTRIBUTION={}",
        identity.distribution
    );
    println!(
        "cargo:rustc-env=RUSTIFY_DOCS_REPOSITORY={}",
        identity.repository.unwrap_or_default().display()
    );
}
