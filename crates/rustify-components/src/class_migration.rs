use serde_json::Value;

#[test]
#[cfg(rustify_repo_tests)]
fn captured_component_inputs_match_the_fixed_cn_oracle() {
    let historical: Value = serde_json::from_str(include_str!(
        "../../../tests/utility/migration/fixtures.json"
    ))
    .unwrap();
    let oracle: Value = serde_json::from_str(include_str!(
        "../../../tests/utility/migration/cn-expected.json"
    ))
    .unwrap();
    for (input, expected) in historical["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(oracle["cases"].as_array().unwrap())
    {
        assert_eq!(input["id"], expected["id"]);
        let caller = input["callerClass"].as_str().unwrap();
        // The captured inputs freeze cn behavior, independently of today's skin.
        let raw: Vec<_> = input["rawFragments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item.as_str().unwrap())
            .collect();
        let actual = crate::macros::merge(crate::cn::join_text(&raw), caller);
        assert_eq!(
            actual,
            expected["expected"].as_str().unwrap(),
            "{}",
            input["id"]
        );
    }
}
