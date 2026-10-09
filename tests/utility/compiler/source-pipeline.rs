use super::{Compiler, CompilerInput, SourceAsset};
use crate::scanner::{ChangedContent, SourceTree, VirtualSourceTree};

#[test]
fn css_selected_sources_and_variables_match_official_scanner_compiler_pipeline() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("source-pipeline-fixtures.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: case["entry"].as_str().unwrap().into(),
            },
            serde_json::from_value::<Vec<SourceAsset>>(case["assets"].clone()).unwrap(),
            super::tests::limits(),
        )
        .unwrap();
        let mut tree = VirtualSourceTree::new("/");
        for path in case["directories"].as_array().unwrap() {
            tree.insert_directory(path.as_str().unwrap());
        }
        for file in case["files"].as_array().unwrap() {
            tree.insert_file(
                file["path"].as_str().unwrap(),
                file["content"].as_str().unwrap().as_bytes().to_vec(),
                1,
            );
        }
        let mut scanner = compiler
            .scanner(case["base"].as_str().unwrap(), SourceTree::Virtual(tree))
            .unwrap();
        for (index, step) in case["steps"].as_array().unwrap().iter().enumerate() {
            let result = match step["operation"].as_str().unwrap() {
                "scan" => {
                    let result = scanner.scan().unwrap();
                    Some((result.candidates, result.variables))
                }
                "change" => {
                    let result = scanner
                        .scan_content(
                            step["changed"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|content| {
                                    ChangedContent::Content(
                                        content["content"].as_str().unwrap().into(),
                                        content["extension"].as_str().unwrap().into(),
                                    )
                                })
                                .collect(),
                        )
                        .unwrap();
                    Some((result.candidates, result.variables))
                }
                operation => {
                    #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
                    let tree = match scanner.tree_mut() {
                        SourceTree::Virtual(tree) => tree,
                        SourceTree::Native(_) => panic!("virtual fixture"),
                    };
                    #[cfg(not(all(feature = "scanner-fs", not(target_arch = "wasm32"))))]
                    let SourceTree::Virtual(tree) = scanner.tree_mut();
                    match operation {
                        "put" => tree.insert_file(
                            step["path"].as_str().unwrap(),
                            step["content"].as_str().unwrap().as_bytes().to_vec(),
                            index as u128 + 2,
                        ),
                        "remove" => tree.remove(step["path"].as_str().unwrap()),
                        _ => panic!("unknown fixture operation"),
                    }
                    None
                }
            };
            if let Some((mut candidates, variables)) = result {
                candidates.extend(variables);
                candidates.sort();
                candidates.dedup();
                assert_eq!(
                    serde_json::json!(candidates),
                    step["expectedCandidates"],
                    "{} operation {index}",
                    case["id"]
                );
                let prepared = compiler.prepare_add(&candidates).unwrap();
                assert_eq!(
                    prepared.report().css,
                    step["expectedCss"].as_str().unwrap(),
                    "{} operation {index}",
                    case["id"]
                );
                compiler.commit(prepared).unwrap();
            }
        }
    }
}
