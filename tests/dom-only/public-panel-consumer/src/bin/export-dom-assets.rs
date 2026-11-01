use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("public"));
    std::fs::create_dir_all(&out)?;
    for (name, contents) in [
        ("rustify-dom-loader.js", rustify_ui::runtime::DOM_LOADER_JS),
        ("rustify-runtime.css", rustify_ui::runtime::DOM_RUNTIME_CSS),
        ("rustify.css", rustify_components::CSS),
    ] {
        std::fs::write(out.join(name), contents)?;
    }
    Ok(())
}
