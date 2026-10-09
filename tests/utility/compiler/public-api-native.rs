use super::*;
use crate::compiler::CompilerInput;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("public-api-fixtures.json")).unwrap()
}

fn compiler(case: &Value) -> Result<CssCompiler, CompileError> {
    compile(
        case["input"].as_str().unwrap(),
        serde_json::from_value(case["options"].clone()).unwrap(),
        case.get("assets")
            .map(|assets| serde_json::from_value(assets.clone()).unwrap())
            .unwrap_or_default(),
        crate::compiler::tests::limits(),
    )
}

#[test]
fn discovery_base_is_independent_of_source_path() {
    for case in fixture()["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("options:"))
    {
        let api = compiler(case).unwrap();
        assert_eq!(
            serde_json::to_value(api.sources()).unwrap(),
            serde_json::json!({"sources": case["expectedSources"], "root": case["expectedRoot"]}),
            "{}",
            case["id"]
        );
    }
}

#[test]
fn feature_masks_match_fixed_upstream() {
    for case in fixture()["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("features:"))
    {
        let api = compiler(case).unwrap();
        assert_eq!(
            api.features().0,
            case["expectedFeatures"].as_u64().unwrap() as u32,
            "{}",
            case["id"]
        );
    }
}

#[test]
fn polyfill_flags_match_fixed_upstream_css() {
    for case in fixture()["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("polyfills:"))
    {
        let mut api = compiler(case).unwrap();
        for (candidates, expected) in case["batches"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["expectedBuilds"].as_array().unwrap())
        {
            let css = api
                .build(&serde_json::from_value::<Vec<String>>(candidates.clone()).unwrap())
                .unwrap();
            assert_eq!(css, expected["css"].as_str().unwrap(), "{}", case["id"]);
        }
    }
}

#[test]
fn warning_messages_match_fixed_upstream() {
    let f = fixture();
    let case = f["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "warnings:unsupported-bare-type")
        .unwrap();
    let api = compiler(case).unwrap();
    assert_eq!(
        serde_json::to_value(api.warnings()).unwrap(),
        case["expectedWarnings"]
    );
}

#[test]
fn every_string_api_build_matches_fixed_upstream_css() {
    for case in fixture()["compileCases"].as_array().unwrap() {
        if case.get("expectedError").is_some() {
            continue;
        }
        let mut api = compiler(case).unwrap();
        for (index, (candidates, expected)) in case["batches"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["expectedBuilds"].as_array().unwrap())
            .enumerate()
        {
            let css = api
                .build(&serde_json::from_value::<Vec<String>>(candidates.clone()).unwrap())
                .unwrap();
            assert_eq!(
                css.to_utf8().unwrap(),
                expected["css"].as_str().unwrap(),
                "{} batch {index}",
                case["id"]
            );
        }
    }
}

#[test]
fn missing_public_loader_reports_the_official_error() {
    let f = fixture();
    let case = f["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case.get("expectedError").is_some())
        .unwrap();
    let error = compiler(case).err().unwrap();
    assert!(
        matches!(error, CompileError::Syntax { message, .. } if message == case["expectedError"].as_str().unwrap()),
        "wrong public error classification"
    );
}

#[test]
fn virtual_stylesheet_loader_arguments_match_fixed_upstream() {
    for case in fixture()["compileCases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case.get("assets").is_some())
    {
        let api = compiler(case).unwrap();
        assert_eq!(
            serde_json::to_value(api.stylesheet_loads()).unwrap(),
            case["expectedLoadCalls"],
            "{}",
            case["id"]
        );
    }
}

#[test]
fn public_banner_counts_toward_the_output_limit_before_commit() {
    let mut limits = crate::compiler::tests::limits();
    limits.css_bytes = 1;
    let mut api = compile(
        "@tailwind utilities;",
        CompileOptions::default(),
        vec![],
        limits,
    )
    .unwrap();
    assert!(
        matches!(api.build(&[]), Err(CompileError::Limit { resource, limit: 1 }) if resource == "css_bytes")
    );
    assert_eq!(api.compiler.revision(), 0);
}

#[test]
fn public_function_helper_checks_unchanged_text_against_output_bytes() {
    let mut limits = crate::compiler::tests::limits();
    limits.css_bytes = 1;
    let compiler = Compiler::new(
        CompilerInput {
            css: String::new().into(),
            entry: String::new(),
        },
        vec![],
        limits,
    )
    .unwrap();
    assert!(
        matches!(compiler.substitute_functions_in_value(&crate::text::Text::from("é"), false), Err(CompileError::Limit { resource, limit: 1 }) if resource == "css_bytes")
    );
}

fn source_map_snapshot(map: &DecodedSourceMap) -> Value {
    let references = map
        .mappings
        .iter()
        .map(|mapping| {
            mapping.original_position.as_ref().map(|original| {
                map.sources
                    .iter()
                    .position(|source| std::sync::Arc::ptr_eq(source, &original.source))
                    .expect("mapping shares a decoded source handle")
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({"decoded":serde_json::to_value(map).unwrap(),"sourceReferences":references})
}

#[test]
fn every_initial_source_map_matches_fixed_upstream() {
    let mut failures = Vec::new();
    for case in fixture()["compileCases"].as_array().unwrap() {
        if case.get("expectedError").is_some() {
            continue;
        }
        let api = compiler(case).unwrap();
        let actual = source_map_snapshot(&api.build_source_map().unwrap());
        if actual != case["expectedInitialSourceMap"] {
            failures.push(format!(
                "{}: actual {} expected {}",
                case["id"], actual, case["expectedInitialSourceMap"]
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} map mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn every_built_source_map_matches_fixed_upstream() {
    let mut failures = Vec::new();
    for case in fixture()["compileCases"].as_array().unwrap() {
        if case.get("expectedError").is_some() {
            continue;
        }
        let mut api = compiler(case).unwrap();
        for (index, (batch, expected)) in case["batches"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["expectedBuilds"].as_array().unwrap())
            .enumerate()
        {
            api.build(&serde_json::from_value::<Vec<String>>(batch.clone()).unwrap())
                .unwrap();
            let actual = source_map_snapshot(&api.build_source_map().unwrap());
            if actual != expected["sourceMap"] {
                let a = actual["decoded"]["mappings"].as_array().unwrap();
                let b = expected["sourceMap"]["decoded"]["mappings"]
                    .as_array()
                    .unwrap();
                let first = a.iter().zip(b).position(|(a, b)| a != b);
                failures.push(format!("{} batch {index}: mappings {}/{}; first {:?}: actual {} expected {}; sources actual {} expected {}",case["id"],a.len(),b.len(),first,first.map(|i|&a[i]).unwrap_or(&Value::Null),first.map(|i|&b[i]).unwrap_or(&Value::Null),actual["decoded"]["sources"],expected["sourceMap"]["decoded"]["sources"]));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} build map mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn source_map_limit_keeps_revision_and_previous_map_on_repeated_failure() {
    let mut api = compile(
        ".a { color:red; } @tailwind utilities;",
        CompileOptions {
            from: Some("a.css".into()),
            ..Default::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&["flex".into()]).unwrap();
    let previous = source_map_snapshot(&api.build_source_map().unwrap());
    let revision = api.compiler.revision();
    let limit = api.compiler.committed.metrics.cache_bytes;
    api.compiler.limits.cache_bytes = limit;
    for _ in 0..2 {
        assert!(
            matches!(api.build_source_map(),Err(CompileError::Limit{resource,..}) if resource=="cache_bytes")
        );
    }
    assert_eq!(api.compiler.revision(), revision);
    api.compiler.limits.cache_bytes = crate::compiler::tests::limits().cache_bytes;
    assert_eq!(
        source_map_snapshot(&api.build_source_map().unwrap()),
        previous
    );
}

#[test]
fn failed_output_ast_budget_keeps_previous_source_map_and_can_retry() {
    let mut api = compile(
        "@tailwind utilities;",
        CompileOptions {
            from: Some("a.css".into()),
            ..Default::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&["flex".into()]).unwrap();
    let previous = source_map_snapshot(&api.build_source_map().unwrap());
    let revision = api.compiler.revision();
    api.compiler.limits.cache_bytes = api.compiler.committed.metrics.cache_bytes;
    assert!(
        matches!(api.build(&["block".into()]),Err(CompileError::Limit{resource,..}) if resource=="cache_bytes")
    );
    assert_eq!(api.compiler.revision(), revision);
    api.compiler.limits.cache_bytes = crate::compiler::tests::limits().cache_bytes;
    assert_eq!(
        source_map_snapshot(&api.build_source_map().unwrap()),
        previous
    );
    assert!(api
        .build(&["block".into()])
        .unwrap()
        .contains("display: block"));
}

#[test]
fn untracked_imports_and_candidates_do_not_allocate_source_objects() {
    let mut api = compile(
        "@import \"a.css\"; @tailwind utilities;",
        CompileOptions::default(),
        vec![Stylesheet {
            id: "a.css".into(),
            path: Some("a.css".into()),
            base: None,
            content: "@utility probe {color:red;}".into(),
        }],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&["probe".into()]).unwrap();
    assert!(api.mapped_ast.is_empty());
    assert_eq!(
        crate::compiler::ast::source_owned_bytes(&api.compiler.input_ast),
        0
    );
    assert!(api.build_source_map().unwrap().mappings.is_empty());
}

#[test]
fn tracked_custom_candidates_charge_each_structured_cloned_source() {
    let mut api = compile(
        "@utility probe {color:red;} @tailwind utilities;",
        CompileOptions {
            from: Some("a.css".into()),
            ..Default::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&["probe".into(), "hover:probe".into()]).unwrap();
    let sources = api
        .compiler
        .cache
        .values()
        .filter_map(|entry| match &entry.outcome {
            crate::compiler::Outcome::Accepted(nodes) => Some(nodes),
            _ => None,
        })
        .flat_map(|nodes| nodes.iter())
        .map(|compiled| {
            crate::compiler::ast::source_owned_bytes_many([
                std::slice::from_ref(&compiled.node),
                compiled.globals.as_slice(),
            ])
        })
        .collect::<Vec<_>>();
    assert_eq!(sources.len(), 2);
    assert!(sources.iter().all(|bytes| *bytes > 0));
    let map = api.build_source_map().unwrap();
    assert_eq!(
        map.sources
            .iter()
            .filter(|source| source.url.as_deref() == Some("a.css"))
            .count(),
        3
    );
}

#[test]
fn repeated_noop_builds_and_maps_do_not_accumulate_internal_budget() {
    let mut api = compile(
        ".a {color:red;} @tailwind utilities;",
        CompileOptions {
            from: Some("a.css".into()),
            ..Default::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&["flex".into()]).unwrap();
    let cache_bytes = api.compiler.committed.metrics.cache_bytes;
    let map = source_map_snapshot(&api.build_source_map().unwrap());
    for _ in 0..3 {
        api.build(&[]).unwrap();
        assert_eq!(api.compiler.committed.metrics.cache_bytes, cache_bytes);
        assert_eq!(source_map_snapshot(&api.build_source_map().unwrap()), map);
    }
}

#[test]
fn each_apply_candidate_overrides_custom_utility_provenance_with_its_own_span() {
    let css =
        "@utility probe {width:13px;} .a { @apply flex probe underline; } @tailwind utilities;";
    let mut api = compile(
        css,
        CompileOptions {
            from: Some("a.css".into()),
            ..Default::default()
        },
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    api.build(&[]).unwrap();
    let rule=api.mapped_ast.iter().find(|node|matches!(node,crate::compiler::ast::AstNode::Rule{selector,..} if selector==".a")).unwrap();
    let spans = rule
        .children()
        .unwrap()
        .iter()
        .map(|node| {
            let source = node.source().unwrap();
            String::from_utf16(&source.source.code.units()[source.start..source.end]).unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(spans, vec!["flex", "probe", "underline"]);
    let map = api.build_source_map().unwrap();
    for mapping in &map.mappings {
        assert!(map.sources.iter().any(|source| std::sync::Arc::ptr_eq(
            source,
            &mapping.original_position.as_ref().unwrap().source
        )));
    }
}

#[test]
fn utf16_function_errors_match_fixed_oracle_units() {
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    let mut limits = crate::compiler::tests::limits();
    limits.cache_bytes = 8_000_000;
    let compiler = Compiler::new(
        CompilerInput {
            css: String::new().into(),
            entry: String::new(),
        },
        vec![],
        limits,
    )
    .unwrap();
    let mut failures = Vec::new();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| {
        [
            "utf16:error:missing-theme:",
            "utf16:error:missing-css-theme:",
            "utf16:error:alpha-message:",
        ]
        .iter()
        .any(|prefix| case["id"].as_str().unwrap().starts_with(prefix))
    }) {
        count += 1;
        let input: Vec<u16> = serde_json::from_value(case["inputUtf16Units"].clone()).unwrap();
        let start = ".p { value: ".encode_utf16().count();
        let end = input[start..].iter().position(|unit| *unit == 59).unwrap() + start;
        let expression = crate::text::Text::from_units(input[start..end].to_vec());
        let error = compiler
            .substitute_functions_in_value(&expression, false)
            .unwrap_err();
        let serialized = serde_json::to_string(&error).unwrap();
        let roundtrip: CompileError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtrip, error, "{}: typed error JSON", case["id"]);
        match error {
            CompileError::Syntax { message, .. } => {
                let expected: Vec<u16> =
                    serde_json::from_value(case["expectedError"]["messageUtf16Units"].clone())
                        .unwrap();
                if message.units() != expected {
                    failures.push(format!("{}: message unit mismatch", case["id"]));
                }
            }
            other => failures.push(format!(
                "{}: expected Syntax, received {other:?}",
                case["id"]
            )),
        }
    }
    assert_eq!(count, 18);
    assert!(
        failures.is_empty(),
        "{} mismatches: {}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn syntax_error_display_preserves_scalars_and_escapes_isolated_units() {
    let scalar = CompileError::Syntax {
        offset: 7,
        message: crate::text::Text::from("message 😀 \\ud800"),
    };
    assert_eq!(scalar.to_string(), "CSS syntax at 7: message 😀 \\ud800");
    assert_eq!(
        serde_json::to_string(&scalar).unwrap(),
        "{\"kind\":\"syntax\",\"offset\":7,\"message\":\"message 😀 \\\\ud800\"}"
    );
    let isolated = CompileError::Syntax {
        offset: 3,
        message: crate::text::Text::from_units(vec![0xd800, 32, 0xdc00, 32, 0xd83d, 0xde00]),
    };
    assert_eq!(isolated.to_string(), "CSS syntax at 3: \\ud800 \\udc00 😀");
    let json = serde_json::to_string(&isolated).unwrap();
    assert_eq!(
        json,
        "{\"kind\":\"syntax\",\"offset\":3,\"message\":\"\\ud800 \\udc00 😀\"}"
    );
    assert_eq!(
        serde_json::from_str::<CompileError>(&json).unwrap(),
        isolated
    );
}

#[test]
fn error_json_keeps_all_existing_variant_fields_and_accepts_reordered_syntax() {
    let errors = [
        CompileError::Limit {
            resource: "input_bytes".into(),
            limit: 3,
        },
        CompileError::InvalidLimit {
            resource: "input_bytes".into(),
        },
        CompileError::UnsupportedFeature {
            feature: "plugin".into(),
        },
        CompileError::PendingFeature {
            feature: "config".into(),
        },
        CompileError::PendingPrepared,
        CompileError::StalePrepared,
        CompileError::RevisionExhausted,
        CompileError::DuplicateAsset { id: "a.css".into() },
        CompileError::MissingAsset {
            id: "a.css".into(),
            base: "/app".into(),
        },
    ];
    for error in errors {
        let json = serde_json::to_string(&error).unwrap();
        assert_eq!(serde_json::from_str::<CompileError>(&json).unwrap(), error);
    }
    let reordered = serde_json::from_str::<CompileError>(
        "{\"message\":\"\\ud800\",\"offset\":4,\"kind\":\"syntax\"}",
    )
    .unwrap();
    assert_eq!(
        reordered,
        CompileError::Syntax {
            offset: 4,
            message: crate::text::Text::from_units(vec![0xd800]),
        }
    );
}

#[test]
fn cached_syntax_error_charges_utf16_allocation_capacity() {
    let theme = crate::compiler::theme::Theme::default();
    let mut units = Vec::with_capacity(64);
    units.extend([0xd800, 0xdc00]);
    let message = crate::text::Text::from_units(units);
    let owned = message.owned_bytes();
    let logical = message.byte_len();
    theme
        .legacy
        .set(Err(CompileError::Syntax { offset: 0, message }))
        .unwrap();
    let usage = crate::compiler::legacy_theme::cache_usage(&theme);
    assert_eq!(usage.string_bytes, logical);
    assert_eq!(
        usage.owned_bytes,
        std::mem::size_of::<CompileError>() + owned
    );
    assert!(owned > logical);
}
