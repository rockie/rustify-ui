//! `mbx xtask css [--check]`: the component stylesheet and the SDK's static
//! candidate list, generated and checked in.
//!
//! The product is committed, so an example that only links it builds without
//! running Tailwind at all, and generating it takes the standalone CLI that
//! mise pins (`tailwind.rs`) rather than Node. The price of committing is
//! drift - a class string added without regenerating - and `--check` is what
//! makes drift a failure instead of a missing style nobody notices until a
//! screenshot looks wrong.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub fn input(root: &Path) -> PathBuf {
    root.join("crates/rustify-components/css/rustify.tailwind.css")
}

pub fn output(root: &Path) -> PathBuf {
    root.join("crates/rustify-components/css/rustify.css")
}

fn candidates_output(root: &Path) -> PathBuf {
    root.join("crates/rustify-components/css/candidates.json")
}

const SOURCE_TREES: [(&str, &str); 4] = [
    ("crates/rustify-ui/src", "rs"),
    ("crates/rustify-components/src", "rs"),
    ("crates/rustify-ui/web", "css"),
    ("crates/rustify-components/css", "css"),
];

fn candidate_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for (tree, extension) in SOURCE_TREES {
        source_files(&root.join(tree), extension, &mut files)?;
    }
    // An output must not become its own input: otherwise a deleted class
    // survives through last generation's compiled selector.
    let generated = output(root);
    files.retain(|file| file != &generated);
    files.sort();
    Ok(files)
}

fn source_files(directory: &Path, extension: &str, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in
        std::fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?
    {
        let entry = entry.map_err(|error| format!("{}: {error}", directory.display()))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", entry.path().display()))?;
        if kind.is_dir() {
            if matches!(entry.file_name().to_str(), Some("target" | "ref" | "plan")) {
                continue;
            }
            source_files(&entry.path(), extension, files)?;
        } else if kind.is_file()
            && entry
                .path()
                .extension()
                .is_some_and(|value| value == extension)
        {
            files.push(entry.path());
        }
    }
    Ok(())
}

fn candidates(root: &Path) -> Result<Vec<String>, String> {
    let mut candidates = BTreeSet::new();
    for file in candidate_sources(root)? {
        let content = std::fs::read_to_string(&file)
            .map_err(|error| format!("{}: {error}", file.display()))?;
        let extension = file
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        candidates.extend(rustify_style::scanner::scan(&content, extension).candidates);
    }
    Ok(candidates.into_iter().collect())
}

fn candidate_json(candidates: &[String]) -> Result<String, String> {
    serde_json::to_string_pretty(candidates)
        .map(|json| format!("{json}\n"))
        .map_err(|error| format!("cannot serialize SDK candidates: {error}"))
}

pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let check = args.iter().any(|a| a == "--check");
    let candidates = candidates(&root)?;
    let generated_candidates = candidate_json(&candidates)?;
    let target = if check {
        root.join("target/rustify-css-check.css")
    } else {
        output(&root)
    };

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    crate::tailwind::compile(&input(&root), &target, false)?;

    if !check {
        std::fs::write(candidates_output(&root), &generated_candidates)
            .map_err(|error| format!("{}: {error}", candidates_output(&root).display()))?;
        println!("{}", output(&root).display());
        println!(
            "{} ({} candidates)",
            candidates_output(&root).display(),
            candidates.len()
        );
        return Ok(());
    }

    let committed = std::fs::read_to_string(output(&root))
        .map_err(|e| format!("{}: {e}", output(&root).display()))?;
    let generated =
        std::fs::read_to_string(&target).map_err(|e| format!("{}: {e}", target.display()))?;
    let _ = std::fs::remove_file(&target);
    let committed_candidates = std::fs::read_to_string(candidates_output(&root))
        .map_err(|error| format!("{}: {error}", candidates_output(&root).display()))?;
    let differences = [
        drift(&output(&root), &committed, &generated),
        drift(
            &candidates_output(&root),
            &committed_candidates,
            &generated_candidates,
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    if differences.is_empty() {
        println!(
            "css: no drift ({} bytes; {} SDK candidates)",
            committed.len(),
            candidates.len()
        );
        return Ok(());
    }
    Err(differences.join("\n"))
}

fn drift(path: &Path, committed: &str, generated: &str) -> Option<String> {
    if committed == generated {
        return None;
    }
    let mut message = format!(
        "{} is not what the input produces; run `mbx xtask css` and commit the result \
         ({} bytes committed, {} bytes generated)",
        path.display(),
        committed.len(),
        generated.len()
    );
    if let Some(difference) = first_difference(committed, generated) {
        message.push_str(&format!(
            "\nfirst difference, line {}:\n  committed: {}\n  generated: {}",
            difference.line, difference.committed, difference.generated
        ));
    }
    Some(message)
}

/// The first line two texts disagree on, which usually names the class that
/// was added or dropped. The byte counts alone say only that something moved.
#[derive(Debug, PartialEq)]
struct Difference {
    line: usize,
    committed: String,
    generated: String,
}

const END: &str = "<end of file>";

/// Past this many characters a line is cut to a window around where it first
/// differs, so a long line still shows the part that changed.
const WIDTH: usize = 120;

fn first_difference(committed: &str, generated: &str) -> Option<Difference> {
    // Split on '\n' rather than `lines()`: a lost final newline is drift too.
    let mut ours = committed.split('\n');
    let mut theirs = generated.split('\n');
    let mut line = 0;
    loop {
        line += 1;
        match (ours.next(), theirs.next()) {
            (None, None) => return None,
            (a, b) if a == b => continue,
            (a, b) => {
                let column = match (a, b) {
                    (Some(a), Some(b)) => {
                        a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
                    }
                    _ => 0,
                };
                let excerpt =
                    |side: Option<&str>| side.map_or(END.to_string(), |text| window(text, column));
                return Some(Difference {
                    line,
                    committed: excerpt(a),
                    generated: excerpt(b),
                });
            }
        }
    }
}

fn window(line: &str, column: usize) -> String {
    let start = if column < WIDTH {
        0
    } else {
        column - WIDTH / 2
    };
    let mut chars = line.chars().skip(start);
    let shown: String = chars.by_ref().take(WIDTH).collect();
    format!(
        "{}{shown}{}",
        if start > 0 { "…" } else { "" },
        if chars.next().is_some() { "…" } else { "" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root =
                std::env::temp_dir().join(format!("rustify-css-{}-{nonce}", std::process::id()));
            for (tree, _) in SOURCE_TREES {
                std::fs::create_dir_all(root.join(tree)).unwrap();
            }
            Self(root)
        }

        fn write(&self, path: &str, content: impl AsRef<[u8]>) {
            let path = self.0.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn complete_static_candidates_include_hidden_sdk_and_component_branches() {
        let fixture = Fixture::new();
        fixture.write(
            "crates/rustify-ui/src/overlay.rs",
            "if open { \"fixed hidden opacity-0\" }",
        );
        fixture.write(
            "crates/rustify-components/src/panel.rs",
            "const OPEN: &str = \"aria-expanded:bg-accent focus-visible:ring-2 fixed transition-[width]\";",
        );
        fixture.write(
            "crates/rustify-ui/web/dom-runtime.css",
            ".rustify-floating-state { pointer-events: none; }",
        );
        fixture.write(
            "crates/rustify-components/css/controls.css",
            ".rustify-control-state { transition-property: width; }",
        );
        let extracted = candidates(&fixture.0).unwrap();
        for expected in [
            "fixed",
            "hidden",
            "opacity-0",
            "aria-expanded:bg-accent",
            "focus-visible:ring-2",
            "rustify-floating-state",
            "rustify-control-state",
            "transition-[width]",
        ] {
            assert!(
                extracted.iter().any(|candidate| candidate == expected),
                "missing {expected}"
            );
        }
        assert!(extracted.windows(2).all(|pair| pair[0] < pair[1]));
        let json = candidate_json(&extracted).unwrap();
        assert_eq!(
            serde_json::from_str::<Vec<String>>(&json).unwrap(),
            extracted
        );
        assert!(json.ends_with('\n'));
        assert_eq!(
            candidate_json(&candidates(&fixture.0).unwrap()).unwrap(),
            json
        );
    }

    #[test]
    fn excluded_trees_and_generated_css_are_not_read_or_rescanned() {
        let fixture = Fixture::new();
        fixture.write(
            "crates/rustify-components/src/button.rs",
            "\"inline-flex p-4\"",
        );
        let before = candidates(&fixture.0).unwrap();
        for path in [
            "target/cached.rs",
            "ref/vendor.css",
            "docs/plan/prose.rs",
            "crates/rustify-ui/src/target/cached.rs",
            "crates/rustify-components/src/ref/vendor.rs",
            "crates/rustify-components/css/plan/prose.css",
            "crates/rustify-components/css/rustify.css",
        ] {
            // Reading any of these files would fail, rather than silently
            // yielding a convenient empty candidate set.
            fixture.write(path, [0xff]);
        }
        assert_eq!(candidates(&fixture.0).unwrap(), before);
    }

    #[test]
    fn published_candidates_match_the_sdk_sources() {
        let root = crate::build::repo_root();
        let generated = candidate_json(&candidates(&root).unwrap()).unwrap();
        if let Some(message) = drift(
            &candidates_output(&root),
            rustify_components::CANDIDATES_JSON,
            &generated,
        ) {
            panic!("{message}");
        }
    }

    #[test]
    fn the_first_differing_line_is_reported_with_both_sides() {
        let committed = ".a {\n  display: flex;\n}\n";
        let generated = ".a {\n  display: grid;\n}\n";
        assert_eq!(
            first_difference(committed, generated),
            Some(Difference {
                line: 2,
                committed: "  display: flex;".to_string(),
                generated: "  display: grid;".to_string(),
            })
        );
        assert_eq!(first_difference(committed, committed), None);
    }

    #[test]
    fn a_line_only_one_side_has_is_reported_against_the_end_of_file() {
        let difference = first_difference(".a {}\n", ".a {}\n.b {}\n").unwrap();
        assert_eq!(difference.line, 2);
        assert_eq!(difference.committed, "");
        assert_eq!(difference.generated, ".b {}");
        let difference = first_difference(".a {}\n", ".a {}").unwrap();
        assert_eq!(difference.line, 2);
        assert_eq!(difference.generated, END);
    }

    #[test]
    fn a_long_line_is_cut_around_where_it_differs() {
        let prefix = "≈".repeat(300);
        let difference =
            first_difference(&format!("{prefix}old"), &format!("{prefix}new")).unwrap();
        assert!(difference.committed.starts_with('…'));
        assert!(difference.committed.ends_with("old"));
        assert!(difference.generated.ends_with("new"));
        assert!(difference.committed.chars().count() <= WIDTH + 2);
        assert_eq!(window("short", 3), "short");
    }
}
