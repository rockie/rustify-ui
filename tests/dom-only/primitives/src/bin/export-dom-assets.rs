//! Writes only runtime and structural assets, never a theme or component skin.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .ok_or("output directory required")?;
    std::fs::create_dir_all(&out)?;
    for (name, content) in [
        ("rustify-dom-loader.js", rustify_ui::runtime::DOM_LOADER_JS),
        ("rustify-runtime.css", rustify_ui::runtime::DOM_RUNTIME_CSS),
        (
            "primitives.css",
            rustify_components::primitives::PRIMITIVES_CSS,
        ),
    ] {
        std::fs::write(out.join(name), content)?;
    }
    Ok(())
}
