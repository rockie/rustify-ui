use super::*;
use serde_json::{json, Value};
use std::path::Path;
fn string<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}
fn globs(values: Vec<GlobEntry>) -> Vec<String> {
    let mut values = values
        .into_iter()
        .map(|glob| {
            format!(
                "{}{}{}",
                glob.base,
                if glob.pattern.starts_with('/') {
                    ""
                } else {
                    "/"
                },
                glob.pattern
            )
            .replace("/fixture/", "")
        })
        .collect::<Vec<_>>();
    values.sort();
    values
}
fn flatten(result: ScanResult) -> Vec<String> {
    let mut values = result
        .candidates
        .into_iter()
        .chain(result.variables)
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

pub fn run() -> Vec<String> {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("source-fixtures.json")).unwrap();
    let mut errors = vec![];
    for fixture in fixtures {
        let mut tree = VirtualSourceTree::new("/fixture");
        for dir in fixture["directories"].as_array().unwrap() {
            tree.insert_directory(dir.as_str().unwrap());
        }
        for file in fixture["files"].as_array().unwrap() {
            tree.insert_file(
                string(file, "path"),
                string(file, "content").as_bytes().to_vec(),
                0,
            );
        }
        for link in fixture["symlinks"].as_array().unwrap() {
            tree.insert_symlink(
                string(link, "path"),
                Path::new("/fixture").join(string(link, "target")),
            );
        }
        let sources = fixture["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| PublicSourceEntry {
                base: Path::new("/fixture")
                    .join(string(source, "base"))
                    .to_string_lossy()
                    .into_owned(),
                pattern: string(source, "pattern").to_owned(),
                negated: source["negated"].as_bool().unwrap_or(false),
            })
            .collect();
        let mut scanner = Scanner::new(sources, SourceTree::Virtual(tree)).unwrap();
        let mut actual = vec![];
        for (index, step) in fixture["steps"].as_array().unwrap().iter().enumerate() {
            let result = match string(step, "operation") {
                "scan" => {
                    let result = scanner.scan().unwrap();
                    json!(flatten(ScanResult {
                        candidates: result.candidates,
                        variables: result.variables
                    }))
                }
                "change" => {
                    let changed = step["changed"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|change| {
                            if change["path"].is_string() {
                                ChangedContent::File(
                                    Path::new("/fixture").join(string(change, "path")),
                                    string(change, "extension").to_owned(),
                                )
                            } else {
                                ChangedContent::Content(
                                    string(change, "content").to_owned(),
                                    string(change, "extension").to_owned(),
                                )
                            }
                        })
                        .collect();
                    json!(flatten(scanner.scan_content(changed).unwrap()))
                }
                "files" => json!(scanner
                    .get_files()
                    .unwrap()
                    .into_iter()
                    .map(|path| path.replace("/fixture/", ""))
                    .collect::<Vec<_>>()),
                "globs" => json!(globs(scanner.get_globs().unwrap())),
                "normalized" => json!(globs(scanner.get_normalized_sources())),
                "positions" => json!(scanner
                    .get_candidates_with_positions(ChangedContent::Content(
                        string(step, "content").to_owned(),
                        string(step, "extension").to_owned()
                    ))
                    .unwrap()),
                operation => {
                    let tree = scanner.tree_mut().virtual_mut().unwrap();
                    match operation {
                        "put" => tree.insert_file(
                            string(step, "path"),
                            string(step, "content").as_bytes().to_vec(),
                            index as u128 + 1,
                        ),
                        "remove" => tree.remove(string(step, "path")),
                        "mkdir" => tree.insert_directory(string(step, "path")),
                        "symlink" => tree.insert_symlink(
                            string(step, "path"),
                            Path::new("/fixture").join(string(step, "target")),
                        ),
                        _ => panic!("unknown operation"),
                    };
                    Value::Null
                }
            };
            actual.push(result);
        }
        if json!(actual) != fixture["expected"] {
            errors.push(format!(
                "{}: expected {} actual {}",
                fixture["name"],
                fixture["expected"],
                json!(actual)
            ));
        }
    }
    errors
}
#[cfg(test)]
#[test]
fn fixed_upstream_source_steps_match() {
    assert_eq!(run(), Vec::<String>::new());
}
