use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let out = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    fs::create_dir_all(&out)?;
    for (name, contents) in [
        ("rustify-dom-loader.js", rustify_ui::runtime::DOM_LOADER_JS),
        ("rustify-runtime.css", rustify_ui::runtime::DOM_RUNTIME_CSS),
    ] {
        fs::write(out.join(name), contents)?;
    }
    Ok(())
}
