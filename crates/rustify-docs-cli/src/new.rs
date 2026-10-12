//! `rustify-docs new`: a site that previews and builds as it is.
//!
//! The toolchain is checked first and the target directory second, both
//! before anything is written. The site is written into a temporary sibling
//! of the target and renamed into place only when every file is there, so a
//! failure leaves nothing behind and an existing directory is never written
//! into.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The `content` template, as `(path, text)`; `{{name}}` is the site's name.
const CONTENT: &[(&str, &str)] = &[
    (
        "rustify-docs.toml",
        include_str!("../templates/content/rustify-docs.toml"),
    ),
    ("README.md", include_str!("../templates/content/README.md")),
    // Kept without its dot in this repository, where it would be a real
    // ignore file.
    (".gitignore", include_str!("../templates/content/gitignore")),
    (
        "content/index.md",
        include_str!("../templates/content/content/index.md"),
    ),
    (
        "content/guide/getting-started.md",
        include_str!("../templates/content/content/guide/getting-started.md"),
    ),
    (
        "content/guide/writing-pages.md",
        include_str!("../templates/content/content/guide/writing-pages.md"),
    ),
    (
        "static/logo.svg",
        include_str!("../templates/content/static/logo.svg"),
    ),
];

/// The Rust toolchain, as `rustc --version` reports it. A content site
/// compiles nothing; sites with components of their own will, so `new` and
/// `dev` check for it up front rather than half-way into a session.
pub fn doctor() -> Result<String, String> {
    let missing = || {
        "Rust is not installed (`rustc --version` did not run).\n\
         Install it from https://rustup.rs, then run `rustup toolchain install stable`."
            .to_owned()
    };
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .map_err(|_| missing())?;
    let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if !output.status.success() || !version.starts_with("rustc ") {
        return Err(missing());
    }
    Ok(version)
}

/// Escapes `text` for a double-quoted TOML or YAML string.
fn quoted(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// The template with the site's name in it: escaped where the template
/// quotes it, as written elsewhere.
fn fill(template: &str, name: &str) -> String {
    template
        .replace("{{version}}", crate::VERSION)
        .replace("{{install-version}}", crate::install_version())
        .replace("{{revision}}", crate::install_revision())
        .replace("{{install}}", &crate::install())
        .replace("{{rollback}}", &crate::rollback())
        .replace("\"{{name}}\"", &format!("\"{}\"", quoted(name)))
        .replace("{{name}}", name)
}

fn is_empty_dir(dir: &Path) -> std::io::Result<bool> {
    Ok(std::fs::read_dir(dir)?.next().is_none())
}

pub fn run(dir: &Path, template: &str) -> Result<String, String> {
    if template != "content" {
        return Err(format!(
            "template `{template}` is not in this release; the one template is `content`"
        ));
    }
    let version = doctor()?;
    // An existing directory may be named as `.` or `..`; its real path has
    // the name.
    let resolved = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let dir = resolved.as_path();
    let name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| format!("{} does not end in a directory name", dir.display()))?
        .to_owned();
    let parent = match dir.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    };
    if !parent.is_dir() {
        return Err(format!("{} does not exist", parent.display()));
    }
    let existed = match std::fs::symlink_metadata(dir) {
        Ok(meta) if meta.is_dir() => {
            if !is_empty_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
                return Err(format!(
                    "{} is not empty; `new` only writes into a new or empty directory",
                    dir.display()
                ));
            }
            true
        }
        Ok(_) => return Err(format!("{} exists and is not a directory", dir.display())),
        Err(_) => false,
    };

    let staging = parent.join(format!(".{name}.rustify-docs-new-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    let written = (|| -> std::io::Result<()> {
        for (path, text) in CONTENT {
            let target = staging.join(path);
            std::fs::create_dir_all(target.parent().expect("template paths have a parent"))?;
            std::fs::write(target, fill(text, &name))?;
        }
        if existed {
            std::fs::remove_dir(dir)?;
        }
        std::fs::rename(&staging, dir)
    })();
    if let Err(error) = written {
        let _ = std::fs::remove_dir_all(&staging);
        if existed && !dir.exists() {
            let _ = std::fs::create_dir(dir);
        }
        return Err(format!("writing {}: {error}", dir.display()));
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_quoted_where_the_template_quotes_them() {
        let toml = fill("name = \"{{name}}\"\n", "a \"b\"");
        assert_eq!(toml, "name = \"a \\\"b\\\"\"\n");
        let markdown = fill("title: \"{{name}}\"\n\n# {{name}}\n", "a \"b\"");
        assert_eq!(markdown, "title: \"a \\\"b\\\"\"\n\n# a \"b\"\n");
        let named = fill("name = \"{{name}}\"\n# {{name}}\n", "site{{version}}");
        assert_eq!(named, "name = \"site{{version}}\"\n# site{{version}}\n");
    }
}
