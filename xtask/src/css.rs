//! `mbx xtask css [--check]`: the component stylesheet, generated and
//! checked in.
//!
//! The product is committed, so an example that only links it builds without
//! running Tailwind at all, and generating it takes the standalone CLI that
//! mise pins (`tailwind.rs`) rather than Node. The price of committing is
//! drift - a class string added without regenerating - and `--check` is what
//! makes drift a failure instead of a missing style nobody notices until a
//! screenshot looks wrong.

use std::path::{Path, PathBuf};

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

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    crate::tailwind::compile(&input(&root), &target, false)?;

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
        return Ok(());
    }
    let mut message = format!(
        "{} is not what the input produces; run `mbx xtask css` and commit the result \
         ({} bytes committed, {} bytes generated)",
        output(&root).display(),
        committed.len(),
        generated.len()
    );
    if let Some(difference) = first_difference(&committed, &generated) {
        message.push_str(&format!(
            "\nfirst difference, line {}:\n  committed: {}\n  generated: {}",
            difference.line, difference.committed, difference.generated
        ));
    }
    Err(message)
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
