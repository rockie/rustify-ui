//! Local fonts through the real content-only CLI and output transaction.
mod support;

use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};
use support::{tree, Temp, BIN};

fn project(base: &str, theme: &str) -> Temp {
    let site = Temp::new("fonts");
    site.write("rustify-docs.toml", &format!("schema_version=1\n[site]\nname='Local fonts'\nurl='https://example.test{base}'\nbase='{base}'\n{theme}"));
    site.write(
        "content/index.md",
        "---\ndescription: Author fonts\n---\n# Local fonts\n\nA local font.\n",
    );
    site.write(
        "theme.json",
        include_str!("../templates/theme.example.json"),
    );
    let directory = site.path("fonts");
    fs::create_dir_all(&directory).unwrap();
    for name in ["author-sans.woff2", "author-text.ttf"] {
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/docs-ssg/fixtures/fonts")
                .join(name),
            directory.join(name),
        )
        .unwrap();
    }
    for args in [
        vec!["init", "--quiet"],
        vec!["add", "."],
        vec![
            "-c",
            "user.name=Font fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "Initial author content",
        ],
    ] {
        let output = Command::new("git")
            .args(args)
            .current_dir(&site.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    site
}

fn run(site: &Temp, command: &str, extra: &[&str]) -> (bool, Value) {
    let output = Command::new(BIN)
        .arg(command)
        .args(["--format", "json"])
        .args(extra)
        .current_dir(&site.0)
        .output()
        .unwrap();
    let report = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "{error}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    });
    (output.status.success(), report)
}

fn css(site: &Temp) -> String {
    tree(&site.path("dist"))
        .iter()
        .filter(|(path, _)| path.ends_with(".css"))
        .map(|(_, bytes)| String::from_utf8(bytes.clone()).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

const FONT_CONFIG: &str = "[theme]\ndocument='theme.json'\n[[theme.fonts]]\nfamily='Author Sans'\nsrc='fonts/author-sans.woff2'\nweight='100 900'\n[[theme.fonts]]\nfamily='Author Text'\nsrc='fonts/author-text.ttf'\nstyle='italic'\n";

#[test]
fn default_and_generic_themes_generate_no_font_requests() {
    for declaration in ["", "[theme]\ndocument='theme.json'\n"] {
        let site = project("/", declaration);
        let (success, report) = run(&site, "build", &["--deny-warnings"]);
        assert!(success, "{report}");
        assert!(!css(&site).contains("@font-face"));
        assert!(!tree(&site.path("dist"))
            .keys()
            .any(|path| path.starts_with("assets/fonts/")));
    }
}

#[test]
fn declared_fonts_publish_exact_bytes_formats_and_base_urls() {
    for base in ["/", "/docs/"] {
        let site = project(base, FONT_CONFIG);
        let (success, report) = run(&site, "build", &["--deny-warnings"]);
        assert!(success, "{report}");
        let css = css(&site);
        for (file, format) in [
            ("author-sans.woff2", "woff2"),
            ("author-text.ttf", "truetype"),
        ] {
            let bytes = fs::read(site.path(&format!("fonts/{file}"))).unwrap();
            let extension = file.rsplit('.').next().unwrap();
            let path = format!("assets/fonts/{:x}.{extension}", Sha256::digest(&bytes));
            assert_eq!(fs::read(site.path(&format!("dist/{path}"))).unwrap(), bytes);
            assert!(css.contains(&format!("{base}{path}")), "{css}");
            assert!(
                css.contains(&format!("format(\"{format}\")"))
                    || css.contains(&format!("format({format})")),
                "{css}"
            );
        }
        assert_eq!(
            tree(&site.path("dist"))
                .keys()
                .filter(|path| path.starts_with("assets/fonts/"))
                .count(),
            2
        );
        assert!(css.contains("swap"));
        assert!(!css.contains("makepad_widgets"));
    }
}

#[test]
fn missing_and_empty_fonts_fail_consistently_and_keep_the_previous_output() {
    let site = project("/", FONT_CONFIG);
    assert!(run(&site, "build", &[]).0);
    let previous = tree(&site.path("dist"));
    let path = site.path("fonts/author-sans.woff2");
    let original = fs::read(&path).unwrap();
    for bytes in [None, Some(Vec::new())] {
        match bytes {
            None => fs::remove_file(&path).unwrap(),
            Some(bytes) => fs::write(&path, bytes).unwrap(),
        }
        for command in ["check", "build"] {
            let (success, report) = run(&site, command, &[]);
            assert!(!success, "{command}: {report}");
            assert!(report.to_string().contains("theme-font"), "{report}");
            assert_eq!(tree(&site.path("dist")), previous);
        }
        fs::write(&path, &original).unwrap();
    }
    assert!(run(&site, "build", &[]).0);
    assert_eq!(tree(&site.path("dist")), previous);
}

#[test]
fn unknown_named_fonts_warn_once_and_deny_warnings_rejects_them() {
    let site = project("/", "[theme]\ndocument='theme.json'\n");
    let mut document: Value = serde_json::from_str(&site.read("theme.json")).unwrap();
    for mode in ["light", "dark"] {
        for slot in ["font-sans", "font-serif", "font-mono"] {
            document["styles"][mode][slot] = "Uninstalled Author Face".into();
        }
    }
    site.write(
        "theme.json",
        &serde_json::to_string_pretty(&document).unwrap(),
    );
    let (success, report) = run(&site, "check", &[]);
    assert!(success, "{report}");
    let warnings = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| issue["rule"] == "theme-font-system")
        .count();
    assert_eq!(warnings, 1, "{report}");
    assert!(!run(&site, "check", &["--deny-warnings"]).0);
    assert!(run(&site, "build", &[]).0);
    assert!(!tree(&site.path("dist"))
        .keys()
        .any(|path| path.starts_with("assets/fonts/")));
    assert!(css(&site).contains("Uninstalled Author Face"));
}

#[cfg(unix)]
#[test]
fn font_symlinks_cannot_escape_read_private_files_or_generated_output() {
    use std::os::unix::fs::symlink;
    let site = project("/", FONT_CONFIG);
    let outside = Temp::new("outside-fonts");
    outside.write("font.woff2", "outside bytes");
    site.write(".env.fonts/font.woff2", "private bytes");
    site.write("dist/font.woff2", "generated bytes");
    let font = site.path("fonts/author-sans.woff2");
    fs::remove_file(&font).unwrap();
    for target in [
        outside.path("font.woff2"),
        site.path(".env.fonts/font.woff2"),
        site.path("dist/font.woff2"),
    ] {
        symlink(target, &font).unwrap();
        let (success, report) = run(&site, "check", &[]);
        assert!(!success, "{report}");
        assert!(report.to_string().contains("theme-font"), "{report}");
        fs::remove_file(&font).unwrap();
    }
}
