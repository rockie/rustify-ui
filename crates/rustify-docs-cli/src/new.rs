//! `rustify-docs new`: a site that previews and builds as it is.
//!
//! The toolchain is checked first and the target directory second, both
//! before anything is written. The site is written into a temporary sibling
//! of the target and renamed into place only when every file is there, so a
//! failure leaves nothing behind and an existing directory is never written
//! into.

use rustify_docs::diagnostic_text::{code_span, literal};
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

const COMPONENTS: &[(&str, &str)] = &[
    (
        "Cargo.toml",
        include_str!("../templates/components/Cargo.toml.in"),
    ),
    (
        "rustify-docs.toml",
        include_str!("../templates/components/rustify-docs.toml"),
    ),
    (
        "README.md",
        include_str!("../templates/components/README.md"),
    ),
    (
        ".gitignore",
        include_str!("../templates/components/gitignore"),
    ),
    (
        "src/lib.rs",
        include_str!("../templates/components/src/lib.rs"),
    ),
    (
        "src/demos.rs",
        include_str!("../templates/components/src/demos.rs"),
    ),
    (
        "src/bin/docs-render.rs",
        include_str!("../templates/components/src/bin/docs-render.rs"),
    ),
    (
        "content/index.md",
        include_str!("../templates/components/content/index.md"),
    ),
    (
        "content/guide/getting-started.md",
        include_str!("../templates/components/content/guide/getting-started.md"),
    ),
    (
        "content/guide/writing-demos.md",
        include_str!("../templates/components/content/guide/writing-demos.md"),
    ),
];

// The pair supplies a complete lock; generation only renames its root package.
const COMPONENTS_LOCK: &str = include_str!("../assets/components-Cargo.lock");

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

/// Checks the wasm standard library used by the active Rust compiler.
pub fn doctor_components() -> Result<String, String> {
    let version = doctor()?;
    let missing = || {
        "The wasm32-unknown-unknown target is unavailable to the active Rust compiler.\n\
         Run `rustup target add wasm32-unknown-unknown`, then retry with that toolchain."
            .to_owned()
    };
    let output = Command::new("rustc")
        .args([
            "--print",
            "target-libdir",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .map_err(|_| missing())?;
    if !output.status.success() {
        return Err(missing());
    }
    let libdir = String::from_utf8_lossy(&output.stdout);
    let installed = std::fs::read_dir(libdir.trim())
        .map_err(|_| missing())?
        .filter_map(Result::ok)
        .any(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.starts_with("libstd-") && name.ends_with(".rlib") && entry.path().is_file()
        });
    if !installed {
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

fn package_name(name: &str) -> String {
    let mut package = String::new();
    let mut separator = false;
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !package.is_empty() {
                package.push('-');
            }
            package.push(character.to_ascii_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }
    if package.is_empty() {
        "docs-site".to_owned()
    } else if !package.as_bytes()[0].is_ascii_alphabetic() {
        format!("docs-{package}")
    } else {
        package
    }
}

fn is_revision(revision: &str) -> bool {
    revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn framework_revision() -> Result<&'static str, String> {
    let install = "Install a published CLI/framework pair with `cargo install --git https://github.com/rockie/rustify-ui --rev <CLI-C> --locked --profile docs-cli rustify-docs-cli`.";
    if !is_revision(crate::SOURCE_REVISION) {
        return Err(format!(
            "The components template requires a traceable CLI source revision.\n{install}"
        ));
    }
    let framework = crate::EXPECTED_FRAMEWORK_REV.trim();
    if !is_revision(framework) {
        return Err(format!(
            "The components template has no paired framework revision in this CLI.\n{install}"
        ));
    }
    Ok(framework)
}

fn fill_components(template: &str, name: &str, package: &str, framework: &str) -> String {
    let install = format!(
        "cargo install --git https://github.com/rockie/rustify-ui --rev {} --locked --profile docs-cli rustify-docs-cli",
        crate::SOURCE_REVISION
    );
    let directory = format!("'{}'", name.replace('\'', "'\\''"));
    template
        .replace("{{version}}", crate::VERSION)
        .replace("{{revision}}", crate::SOURCE_REVISION)
        .replace("{{framework-revision}}", framework)
        .replace("{{install}}", &install)
        .replace("{{package}}", package)
        .replace("{{directory}}", &directory)
        .replace("\"{{name}}\"", &format!("\"{}\"", quoted(name)))
        .replace("{{name}}", name)
}

fn named_lock(lock: &str, package: &str) -> Result<String, String> {
    let mut renamed = String::with_capacity(lock.len() + package.len());
    let mut roots = 0;
    for (index, record) in lock.split("\n[[package]]\n").enumerate() {
        if index > 0 {
            renamed.push_str("\n[[package]]\n");
        }
        if record.starts_with("name = \"docs-site\"\n")
            && !record.lines().any(|line| line.starts_with("source = "))
        {
            roots += 1;
            renamed.push_str(&record.replacen(
                "name = \"docs-site\"\n",
                &format!("name = \"{package}\"\n"),
                1,
            ));
        } else {
            renamed.push_str(record);
        }
    }
    if roots != 1 {
        return Err("The embedded components Cargo.lock must contain one source-free `docs-site` root package; reinstall a published paired CLI.".to_owned());
    }
    Ok(renamed)
}

fn is_empty_dir(dir: &Path) -> std::io::Result<bool> {
    Ok(std::fs::read_dir(dir)?.next().is_none())
}

pub fn run(dir: &Path, template: &str) -> Result<String, String> {
    let (files, version) = match template {
        "content" => (CONTENT, doctor()?),
        "components" => (COMPONENTS, doctor_components()?),
        _ => {
            return Err(format!(
                "template {} is not in this release; choose `content` or `components`",
                code_span(template)
            ));
        }
    };
    // An existing directory may be named as `.` or `..`; its real path has
    // the name.
    let resolved = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let dir = resolved.as_path();
    let name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| {
            format!(
                "{} does not end in a directory name",
                literal(&dir.display().to_string())
            )
        })?
        .to_owned();
    let parent = match dir.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    };
    if !parent.is_dir() {
        return Err(format!(
            "{} does not exist",
            literal(&parent.display().to_string())
        ));
    }
    let existed = match std::fs::symlink_metadata(dir) {
        Ok(meta) if meta.is_dir() => {
            if !is_empty_dir(dir)
                .map_err(|e| format!("{}: {e}", literal(&dir.display().to_string())))?
            {
                return Err(format!(
                    "{} is not empty; `new` only writes into a new or empty directory",
                    literal(&dir.display().to_string())
                ));
            }
            true
        }
        Ok(_) => {
            return Err(format!(
                "{} exists and is not a directory",
                literal(&dir.display().to_string())
            ))
        }
        Err(_) => false,
    };

    let components = if template == "components" {
        let framework = framework_revision()?;
        let package = package_name(&name);
        let lock = named_lock(COMPONENTS_LOCK, &package)?;
        Some((framework, package, lock))
    } else {
        None
    };

    let staging = parent.join(format!(".{name}.rustify-docs-new-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    let written = (|| -> std::io::Result<()> {
        for (path, text) in files {
            let target = staging.join(path);
            std::fs::create_dir_all(target.parent().expect("template paths have a parent"))?;
            let rendered = match &components {
                Some((framework, package, _)) => fill_components(text, &name, package, framework),
                None => fill(text, &name),
            };
            std::fs::write(target, rendered)?;
        }
        if let Some((_, _, lock)) = &components {
            std::fs::write(staging.join("Cargo.lock"), lock)?;
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
        return Err(format!(
            "writing {}: {error}",
            literal(&dir.display().to_string())
        ));
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

    #[test]
    fn directory_names_become_valid_package_names() {
        for (name, package) in [
            ("My Docs", "my-docs"),
            ("  My__Docs---2026! ", "my-docs-2026"),
            ("中文 🦀", "docs-site"),
            ("123 Notes", "docs-123-notes"),
            ("_9__", "docs-9"),
            ("self", "self"),
        ] {
            assert_eq!(package_name(name), package);
        }
    }

    #[test]
    fn components_names_are_literal_in_commands_and_display_text() {
        let text = fill_components(
            "cd {{directory}}\nname = \"{{name}}\"\npackage = \"{{package}}\"\n# {{name}}\n",
            "My 'Docs' $(touch nope) {{package}}",
            "my-docs-touch-nope-package",
            "framework",
        );
        assert_eq!(
            text,
            "cd 'My '\\''Docs'\\'' $(touch nope) {{package}}'\nname = \"My 'Docs' $(touch nope) {{package}}\"\npackage = \"my-docs-touch-nope-package\"\n# My 'Docs' $(touch nope) {{package}}\n"
        );
    }

    #[test]
    fn renaming_the_lock_changes_only_the_source_free_root_record() {
        let lock = "# Locked dependencies\nversion = 4\n\n[[package]]\nname = \"docs-site\"\nversion = \"1.0.0\"\nsource = \"registry+https://example.com/index\"\n\n[[package]]\nname = \"docs-site\"\nversion = \"0.1.0\"\ndependencies = [\n \"docs-site 1.0.0\",\n \"rustify-docs\",\n]\n\n[[package]]\nname = \"rustify-docs\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/rockie/rustify-ui?rev=framework#framework\"\n";
        let renamed = named_lock(lock, "my-docs").unwrap();
        assert_eq!(
            renamed,
            lock.replace(
                "name = \"docs-site\"\nversion = \"0.1.0\"",
                "name = \"my-docs\"\nversion = \"0.1.0\""
            )
        );
        assert!(named_lock("version = 4\n", "my-docs").is_err());
        assert!(named_lock(&(lock.to_owned() + lock), "my-docs").is_err());
    }

    #[test]
    fn source_identities_must_be_complete_git_revisions() {
        assert!(is_revision("0123456789abcdef0123456789abcdef01234567"));
        for revision in [
            "",
            "main",
            "01234567",
            "g123456789abcdef0123456789abcdef01234567",
        ] {
            assert!(!is_revision(revision));
        }
    }
}
