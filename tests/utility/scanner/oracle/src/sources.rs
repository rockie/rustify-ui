use serde_json::{json, Value};
use std::{fs, path::Path};
use tailwindcss_oxide::{ChangedContent, GlobEntry, PublicSourceEntry, Scanner};

fn string<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}
fn globs(values: Vec<GlobEntry>, root: &Path) -> Vec<String> {
    let prefix = format!("{}/", root.display());
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
            .replace(&prefix, "")
        })
        .collect::<Vec<_>>();
    values.sort();
    values
}
fn write(root: &Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}
fn apply(root: &Path, step: &Value) {
    match string(step, "operation") {
        "put" => {
            std::thread::sleep(std::time::Duration::from_millis(2));
            write(root, string(step, "path"), string(step, "content"));
        }
        "remove" => {
            let path = root.join(string(step, "path"));
            if path.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        }
        "mkdir" => fs::create_dir_all(root.join(string(step, "path"))).unwrap(),
        "symlink" => {
            let link = root.join(string(step, "path"));
            fs::create_dir_all(link.parent().unwrap()).unwrap();
            #[cfg(unix)]
            std::os::unix::fs::symlink(root.join(string(step, "target")), link).unwrap();
        }
        _ => {}
    }
}
pub fn generate(path: &str) {
    let mut fixtures: Vec<Value> = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    for (index, fixture) in fixtures.iter_mut().enumerate() {
        let root = std::env::temp_dir().join(format!(
            "rustify-scanner-oracle-{}-{}-{index}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let root = fs::canonicalize(root).unwrap();
        if let Some(dirs) = fixture["directories"].as_array() {
            for path in dirs {
                fs::create_dir_all(root.join(path.as_str().unwrap())).unwrap();
            }
        }
        if let Some(files) = fixture["files"].as_array() {
            for file in files {
                write(&root, string(file, "path"), string(file, "content"));
            }
        }
        if let Some(links) = fixture["symlinks"].as_array() {
            for link in links {
                let mut step = link.clone();
                step["operation"] = json!("symlink");
                apply(&root, &step);
            }
        }
        let sources = fixture["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| PublicSourceEntry {
                base: root
                    .join(string(source, "base"))
                    .to_string_lossy()
                    .into_owned(),
                pattern: string(source, "pattern").to_owned(),
                negated: source["negated"].as_bool().unwrap_or(false),
            })
            .collect();
        let mut scanner = Scanner::new(sources);
        let mut expected = vec![];
        for step in fixture["steps"].as_array().unwrap() {
            let result = match string(step, "operation") {
                "scan" => json!(scanner.scan()),
                "change" => {
                    let changes = step["changed"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|change| {
                            if change["path"].is_string() {
                                ChangedContent::File(
                                    root.join(string(change, "path")),
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
                    json!(scanner.scan_content(changes))
                }
                "files" => {
                    let mut files = scanner
                        .get_files()
                        .into_iter()
                        .map(|path| path.replace(&format!("{}/", root.display()), ""))
                        .collect::<Vec<_>>();
                    files.sort();
                    json!(files)
                }
                "globs" => json!(globs(scanner.get_globs(), &root)),
                "normalized" => json!(globs(scanner.get_normalized_sources(), &root)),
                "positions" => json!(scanner.get_candidates_with_positions(
                    ChangedContent::Content(
                        string(step, "content").to_owned(),
                        string(step, "extension").to_owned()
                    )
                )),
                _ => {
                    apply(&root, step);
                    Value::Null
                }
            };
            expected.push(result);
        }
        fixture["expected"] = json!(expected);
        fs::remove_dir_all(&root).unwrap();
    }
    println!("{}", serde_json::to_string_pretty(&fixtures).unwrap());
}
