//! `mbx xtask css [--check]`: the component stylesheet, generated and
//! checked in.
//!
//! The product is committed rather than built during `build-web`, so building
//! the wasm never needs Node. The price of that is drift - a class string added
//! without regenerating - and `--check` is what makes drift a failure instead
//! of a missing style nobody notices until a screenshot looks wrong.

use std::path::{Path, PathBuf};
use std::process::Command;

pub fn input(root: &Path) -> PathBuf {
    root.join("crates/rustify-components/css/rustify.tailwind.css")
}

pub fn output(root: &Path) -> PathBuf {
    root.join("crates/rustify-components/css/rustify.css")
}

pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let check = args.iter().any(|a| a == "--check");
    let target = if check {
        root.join("target/rustify-css-check.css")
    } else {
        output(&root)
    };

    let cli = root.join("node_modules/.bin/tailwindcss");
    if !cli.is_file() {
        return Err(format!(
            "{} is not installed; run `npm ci` (the Tailwind CLI is a dev dependency and is never needed to build the wasm)",
            cli.display()
        ));
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let out = Command::new(&cli)
        .args([
            "-i",
            &input(&root).to_string_lossy(),
            "-o",
            &target.to_string_lossy(),
        ])
        .current_dir(&root)
        .output()
        .map_err(|e| format!("cannot run {}: {e}", cli.display()))?;
    if !out.status.success() {
        return Err(format!(
            "tailwindcss exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }

    if !check {
        println!("{}", output(&root).display());
        return Ok(());
    }

    let committed = std::fs::read_to_string(output(&root))
        .map_err(|e| format!("{}: {e}", output(&root).display()))?;
    let generated =
        std::fs::read_to_string(&target).map_err(|e| format!("{}: {e}", target.display()))?;
    let _ = std::fs::remove_file(&target);
    if committed == generated {
        println!("css: no drift ({} bytes)", committed.len());
        Ok(())
    } else {
        Err(format!(
            "{} is not what the input produces; run `mbx xtask css` and commit the result \
             ({} bytes committed, {} bytes generated)",
            output(&root).display(),
            committed.len(),
            generated.len()
        ))
    }
}
