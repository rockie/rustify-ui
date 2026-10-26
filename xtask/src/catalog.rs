//! `mbx xtask catalog --write <path> [--check]`: the capability document,
//! printed from the table the components are declared in.
//!
//! Keeps the reference inventory, public entries and compatibility categories
//! in the same generated document without depending on local reference trees.

use std::path::PathBuf;

use rustify_components::catalog::{
    catalogue_markdown, CATALOG, COMPONENTS, MARKER, STABLE_REFERENCES,
};

pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let target = crate::option(args, "--write")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
        .ok_or("--write <path> is required (add --check to compare instead of write)")?;
    let check = args.iter().any(|a| a == "--check");

    let document = std::fs::read_to_string(&target).map_err(|e| {
        format!(
            "{}: {e}; the document holds the prose around the table",
            target.display()
        )
    })?;
    let table = catalogue_markdown();
    let (head, tail) = fences(&document, &target)?;
    let next = format!("{head}{MARKER}\n{table}{MARKER}{tail}");

    if check {
        return if document == next {
            println!("catalog: no drift ({} public entries, {} stable reference names, {} legacy categories)", COMPONENTS.len(), STABLE_REFERENCES.len(), CATALOG.len());
            Ok(())
        } else {
            Err(format!(
                "{} is not what the catalogue prints; run `mbx xtask catalog --write {}`",
                target.display(),
                crate::option(args, "--write").unwrap_or_default()
            ))
        };
    }

    std::fs::write(&target, next).map_err(|e| format!("{}: {e}", target.display()))?;
    println!("{}", target.display());
    Ok(())
}

/// The prose before the table and after it. Both markers have to be there:
/// writing a table into a document that never had one would put it wherever
/// the first write happened to land.
fn fences<'a>(document: &'a str, target: &std::path::Path) -> Result<(&'a str, &'a str), String> {
    let (head, rest) = document.split_once(MARKER).ok_or_else(|| {
        format!(
            "{} has no {MARKER} fence; the generated table goes between two of them",
            target.display()
        )
    })?;
    let (_, tail) = rest.split_once(MARKER).ok_or_else(|| {
        format!(
            "{} opens a {MARKER} fence and never closes it",
            target.display()
        )
    })?;
    Ok((head, tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fences_preserve_authored_prose_around_the_complete_generated_catalogue() {
        let document = format!("intro\n{MARKER}\nold table\n{MARKER}\ntail");
        let (head, tail) = fences(&document, std::path::Path::new("guide.md")).unwrap();
        let next = format!("{head}{MARKER}\n{}{MARKER}{tail}", catalogue_markdown());
        assert!(next.starts_with("intro\n"));
        assert!(next.ends_with("\ntail"));
        assert!(next.contains("## Stable reference inventory"));
        assert!(next.contains("## Legacy application categories"));
        assert!(!next.contains("old table"));
    }

    #[test]
    fn a_document_requires_both_generator_markers() {
        assert!(fences("no marker", std::path::Path::new("guide.md")).is_err());
        assert!(fences(MARKER, std::path::Path::new("guide.md")).is_err());
    }
}
