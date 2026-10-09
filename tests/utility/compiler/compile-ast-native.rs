use super::*;
use serde_json::Value;

fn scalar_public_ast(nodes: Vec<Value>) -> PublicAst {
    Arc::new(serde_json::from_str(&serde_json::to_string(&nodes).unwrap()).unwrap())
}

#[test]
fn every_public_ast_build_matches_fixed_upstream_structure_and_identity() {
    let fixture: Value = serde_json::from_str(include_str!("public-api-fixtures.json")).unwrap();
    let mut builds = 0;
    let cases = fixture["compileAstCases"].as_array().unwrap();
    assert_eq!(cases.len(), 9);
    for case in cases {
        let input: PublicAst =
            Arc::new(serde_json::from_str(&case["inputAst"].to_string()).unwrap());
        let mut api = compile_ast(
            input.clone(),
            serde_json::from_value(case["options"].clone()).unwrap(),
            case.get("assets")
                .map(|v| serde_json::from_value(v.clone()).unwrap())
                .unwrap_or_default(),
            crate::compiler::tests::limits(),
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", case["id"]));
        assert_eq!(
            api.features().0 as u64,
            case["expectedFeatures"].as_u64().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            serde_json::to_value(api.sources()).unwrap(),
            serde_json::json!({"sources": case["expectedSources"], "root": case["expectedRoot"]}),
            "{}",
            case["id"]
        );
        assert_eq!(
            serde_json::to_value(api.stylesheet_loads()).unwrap(),
            case["expectedLoadCalls"],
            "{}",
            case["id"]
        );
        let mut previous: Option<PublicAst> = None;
        let batches = case["batches"].as_array().unwrap();
        let expected = case["expectedBuilds"].as_array().unwrap();
        assert_eq!(batches.len(), expected.len());
        for (index, (batch, expected)) in batches.iter().zip(expected).enumerate() {
            let actual = api
                .build(&serde_json::from_value::<Vec<String>>(batch.clone()).unwrap())
                .unwrap();
            assert_eq!(
                serde_json::to_value(actual.as_ref()).unwrap(),
                expected["ast"],
                "{} batch {index}",
                case["id"]
            );
            assert_eq!(
                Arc::ptr_eq(&actual, &input),
                expected["sameAsInput"].as_bool().unwrap(),
                "{} batch {index} input identity",
                case["id"]
            );
            assert_eq!(
                previous.as_ref().is_some_and(|p| Arc::ptr_eq(p, &actual)),
                expected["sameAsPrevious"].as_bool().unwrap(),
                "{} batch {index} previous identity",
                case["id"]
            );
            previous = Some(actual);
            builds += 1;
        }
    }
    assert_eq!(builds, 61);
}

fn utilities_input() -> PublicAst {
    scalar_public_ast(vec![
        serde_json::json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":[]}),
    ])
}

#[test]
fn failed_ast_build_preserves_the_last_array_and_candidate_commit() {
    let mut limits = crate::compiler::tests::limits();
    limits.css_bytes = 110;
    let mut api =
        compile_ast(utilities_input(), CompileOptions::default(), vec![], limits).unwrap();
    let previous = api.build(&["flex".into()]).unwrap();
    let revision = api.compiler.revision();
    assert!(
        matches!(api.build(&["w-[137px]".into()]), Err(CompileError::Limit { resource, .. }) if resource == "css_bytes")
    );
    assert_eq!(api.compiler.revision(), revision);
    assert!(Arc::ptr_eq(
        &previous,
        &api.build(&["flex".into()]).unwrap()
    ));
    assert_eq!(api.compiler.report().admitted, vec!["flex"]);
}

#[test]
fn public_ast_input_and_retained_objects_obey_limits() {
    let input = utilities_input();
    let mut limits = crate::compiler::tests::limits();
    limits.input_bytes = 1;
    assert!(
        matches!(compile_ast(input.clone(), CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
    );
    let mut limits = crate::compiler::tests::limits();
    limits.cache_bytes = 1;
    assert!(
        matches!(compile_ast(input, CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
    );
}

#[test]
fn deeply_nested_public_input_is_rejected_before_serializing_or_cloning() {
    let mut node = serde_json::json!({"kind":"declaration", "property":"color", "value":"red", "important":false});
    for _ in 0..8 {
        node = serde_json::json!({"kind":"rule", "selector":"&", "nodes":[node]});
    }
    let mut limits = crate::compiler::tests::limits();
    limits.ast_depth = 2;
    assert!(
        matches!(compile_ast(scalar_public_ast(vec![node]), CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, limit:2 }) if resource == "ast_depth")
    );
}

#[test]
fn unused_virtual_assets_do_not_contribute_locations_or_syntax_errors() {
    let assets = vec![Stylesheet {
        id: "unused.css".into(),
        path: None,
        base: None,
        content: ".invalid {".into(),
    }];
    let mut api = compile_ast(
        utilities_input(),
        CompileOptions::default(),
        assets,
        crate::compiler::tests::limits(),
    )
    .unwrap();
    assert!(api.stylesheet_loads().is_empty());
    assert_eq!(api.build(&[]).unwrap().len(), 1);
}

#[test]
fn oversized_public_input_is_rejected_before_ast_conversion() {
    let mut limits = crate::compiler::tests::limits();
    limits.input_bytes = 8;
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"invalid", "payload":"large text should be rejected before inspecting its AST kind"}),
    ]);
    assert!(
        matches!(compile_ast(input, CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, limit:8 }) if resource == "input_bytes")
    );
}

#[test]
fn duplicate_selectors_keep_each_rule_and_apply_source() {
    let location = |file: &str| serde_json::json!([{"file": file, "code":".same { @apply underline; }"}, 0, 6]);
    let first = location("first.css");
    let second = location("second.css");
    let first_apply =
        serde_json::json!([{"file":"first.css", "code":".same { @apply underline; }"}, 8, 24]);
    let second_apply =
        serde_json::json!([{"file":"second.css", "code":".same { @apply underline; }"}, 8, 24]);
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"rule", "selector":".same", "src": first, "nodes":[{"kind":"at-rule", "name":"@apply", "params":"underline", "nodes":[], "src": first_apply}]}),
        serde_json::json!({"kind":"rule", "selector":".same", "src": second, "nodes":[{"kind":"at-rule", "name":"@apply", "params":"underline", "nodes":[], "src": second_apply}]}),
    ]);
    let mut api = compile_ast(
        input,
        CompileOptions::default(),
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    let output = serde_json::to_value(api.build(&[]).unwrap().as_ref()).unwrap();
    assert_eq!(output[1]["src"], first);
    assert_eq!(output[2]["src"], second);
    let candidate_src = |source: &Value| {
        let mut source = source.clone();
        source[1] = serde_json::json!(15);
        source[2] = serde_json::json!(24);
        source
    };
    assert_eq!(output[1]["nodes"][0]["src"], candidate_src(&first_apply));
    assert_eq!(output[2]["nodes"][0]["src"], candidate_src(&second_apply));
}

#[test]
fn removed_rule_does_not_shift_duplicate_selector_sources() {
    let first = serde_json::json!([{"file":"empty.css", "code":".same {}"}, 0, 6]);
    let second = serde_json::json!([{"file":"visible.css", "code":".same { color:red; }"}, 0, 6]);
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"rule", "selector":".same", "src":first, "nodes":[]}),
        serde_json::json!({"kind":"rule", "selector":".same", "src":second, "nodes":[{"kind":"declaration", "property":"color", "value":"red", "important":false}]}),
        serde_json::json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":[]}),
    ]);
    let mut api = compile_ast(
        input,
        CompileOptions::default(),
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(api.build(&[]).unwrap().as_ref()).unwrap()[1]["src"],
        second
    );
}

#[test]
fn nested_identical_selectors_keep_inner_and_outer_sources() {
    let outer =
        serde_json::json!([{"file":"outer.css", "code":".same { .same { color:red; } }"}, 0, 6]);
    let inner = serde_json::json!([{"file":"inner.css", "code":".same { color:red; }"}, 0, 6]);
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"rule", "selector":".same", "src":outer, "nodes":[{"kind":"rule", "selector":".same", "src":inner, "nodes":[{"kind":"declaration", "property":"color", "value":"red", "important":false}]}]}),
        serde_json::json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":[]}),
    ]);
    let mut api = compile_ast(
        input,
        CompileOptions {
            from: Some("entry.css".into()),
            ..CompileOptions::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    let output = serde_json::to_value(api.build(&[]).unwrap().as_ref()).unwrap();
    assert_eq!(output[1]["src"], outer);
    assert_eq!(output[1]["nodes"][0]["src"], inner);
    assert!(output[1]["nodes"][0]["nodes"][0].get("src").is_none());
}

#[test]
fn public_input_budget_counts_escaped_json_and_from_without_allocating_output() {
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"comment", "value":"quotes\" newline\n 😀"}),
    ]);
    let from = "/😀.css";
    let expected = serde_json::to_string(input.as_ref()).unwrap().len() + from.len();
    let mut limits = crate::compiler::tests::limits();
    limits.input_bytes = expected;
    check_input_budget(&input, Some(from), &limits).unwrap();
    limits.input_bytes -= 1;
    assert!(
        matches!(check_input_budget(&input, Some(from), &limits), Err(CompileError::Limit { resource, limit }) if resource == "input_bytes" && limit == expected - 1)
    );
}

#[test]
fn duplicate_at_rule_headers_keep_distinct_sources() {
    let first = serde_json::json!([{"file":"first.css", "code":"@media (hover: hover) { .a { color:red; } }"}, 0, 22]);
    let second = serde_json::json!([{"file":"second.css", "code":"@media (hover: hover) { .a { color:blue; } }"}, 0, 22]);
    let input = scalar_public_ast(vec![
        serde_json::json!({"kind":"at-rule", "name":"@media", "params":"(hover: hover)", "src":first, "nodes":[{"kind":"rule", "selector":".a", "nodes":[{"kind":"declaration", "property":"color", "value":"red", "important":false}]}]}),
        serde_json::json!({"kind":"at-rule", "name":"@media", "params":"(hover: hover)", "src":second, "nodes":[{"kind":"rule", "selector":".a", "nodes":[{"kind":"declaration", "property":"color", "value":"blue", "important":false}]}]}),
        serde_json::json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":[]}),
    ]);
    let mut api = compile_ast(
        input,
        CompileOptions::default(),
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    let output = serde_json::to_value(api.build(&[]).unwrap().as_ref()).unwrap();
    assert_eq!(output[1]["src"], first);
    assert_eq!(output[2]["src"], second);
}
