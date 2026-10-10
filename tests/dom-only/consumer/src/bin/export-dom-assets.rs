//! Writes the files the page needs beside the wasm-bindgen output: the SDK's
//! DOM loader and runtime stylesheet, and the components' stylesheet. They
//! come from the crates this program was compiled with, which are the ones the
//! wasm was compiled with, so the loader and the wasm always agree.
//!
//! `cargo run --bin export-dom-assets [-- <directory>]`; `public` by default.

use std::path::PathBuf;

fn main() {
    let out = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("public"));
    let files = [
        ("rustify-dom-loader.js", rustify_ui::runtime::DOM_LOADER_JS),
        ("rustify-runtime.css", rustify_ui::runtime::DOM_RUNTIME_CSS),
        ("rustify.css", rustify_components::CSS),
    ];
    if let Err(error) = std::fs::create_dir_all(&out) {
        eprintln!("cannot create {}: {error}", out.display());
        std::process::exit(1);
    }
    for (name, contents) in files {
        let path = out.join(name);
        if let Err(error) = std::fs::write(&path, contents) {
            eprintln!("cannot write {}: {error}", path.display());
            std::process::exit(1);
        }
        println!("{}", path.display());
    }
}
