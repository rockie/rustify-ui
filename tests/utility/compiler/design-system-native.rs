use super::*;
use sha2::{Digest, Sha256};

fn fixtures() -> Vec<Value> {
    let all: Value = serde_json::from_str(include_str!("public-api-fixtures.json")).unwrap();
    all["designSystemCases"].as_array().unwrap().clone()
}

fn design(case: &Value) -> DesignSystem {
    load_design_system(
        case["input"].as_str().unwrap(),
        CompileOptions::default(),
        vec![],
        {
            let mut limits = crate::compiler::tests::limits();
            limits.cache_bytes = 8_000_000;
            limits.candidate_count = 1024;
            limits
        },
    )
    .unwrap()
}

#[test]
fn candidate_parse_print_and_compile_ast_nodes_match_fixed_public_api() {
    for case in fixtures() {
        let mut api = design(&case);
        assert_eq!(api.important, case["important"].as_bool().unwrap());
        for result in case["candidateResults"].as_array().unwrap() {
            let parsed = api
                .parse_candidate(result["raw"].as_str().unwrap())
                .unwrap();
            assert_eq!(
                json!(parsed),
                result["parsed"],
                "{} parse {}",
                case["id"],
                result["raw"]
            );
            for (index, candidate) in parsed.iter().enumerate() {
                assert_eq!(
                    json!(api.print_candidate(candidate).unwrap()),
                    result["printed"][index]
                );
                assert_eq!(
                    json!(api.compile_ast_nodes(candidate, 0).unwrap()),
                    result["compiled"][index]["flagsNone"]
                );
                assert_eq!(
                    json!(api.compile_ast_nodes(candidate, 1).unwrap()),
                    result["compiled"][index]["flagsRespectImportant"]
                );
            }
        }
    }
}

#[test]
fn variant_parse_print_and_selector_completions_match_fixed_public_api() {
    for case in fixtures() {
        let mut api = design(&case);
        for result in case["variantResults"].as_array().unwrap() {
            let parsed = api.parse_variant(result["raw"].as_str().unwrap()).unwrap();
            assert_eq!(
                parsed, result["parsed"],
                "{} parse {}",
                case["id"], result["raw"]
            );
            if !parsed.is_null() {
                assert_eq!(
                    json!(api.print_variant(&parsed).unwrap()),
                    result["printed"]
                );
            }
        }
        let mut variants = api.get_variants().unwrap();
        for variant in &mut variants {
            let name = variant["name"].as_str().unwrap().to_owned();
            let value = variant["values"][0].as_str().unwrap_or("card").to_owned();
            let mut probes = Vec::new();
            for options in [
                json!({}),
                json!({"value": value}),
                json!({"value":"[state=open]", "modifier":"named"}),
            ] {
                let selectors = api.variant_selectors(&name, &options).unwrap();
                probes.push(json!({"options":options,"selectors":selectors}));
            }
            variant["selectorProbes"] = json!(probes);
        }
        assert_eq!(
            variants.len(),
            case["expectedVariants"].as_array().unwrap().len()
        );
        for (actual, expected) in variants
            .iter()
            .zip(case["expectedVariants"].as_array().unwrap())
        {
            assert_eq!(
                actual, expected,
                "{} variant {}",
                case["id"], actual["name"]
            );
        }
    }
}

#[test]
fn all_class_completions_match_the_fixed_upstream_digest() {
    for case in fixtures() {
        let list = design(&case).get_class_list().unwrap();
        assert_eq!(
            list.as_array().unwrap().len(),
            case["expectedClassList"]["count"].as_u64().unwrap() as usize,
            "{} class count",
            case["id"]
        );
        let digest = format!("{:x}", Sha256::digest(serde_json::to_vec(&list).unwrap()));
        assert_eq!(
            digest,
            case["expectedClassList"]["sha256"].as_str().unwrap(),
            "{} class digest",
            case["id"]
        );
    }
}

#[test]
fn class_order_css_and_cached_variant_order_match_fixed_public_api() {
    for case in fixtures() {
        let mut api = design(&case);
        for candidate in case["candidateResults"].as_array().unwrap() {
            api.parse_candidate(candidate["raw"].as_str().unwrap())
                .unwrap();
        }
        for variant in case["variantResults"].as_array().unwrap() {
            api.parse_variant(variant["raw"].as_str().unwrap()).unwrap();
        }
        for variant in api.get_variants().unwrap() {
            let name = variant["name"].as_str().unwrap();
            let value = variant["values"][0].as_str().unwrap_or("card");
            for options in [
                json!({}),
                json!({"value":value}),
                json!({"value":"[state=open]","modifier":"named"}),
            ] {
                api.variant_selectors(name, &options).unwrap();
            }
        }
        let classes: Vec<String> = serde_json::from_value(case["classOrderInput"].clone()).unwrap();
        assert_eq!(
            api.get_class_order(&classes).unwrap(),
            case["expectedClassOrder"],
            "{} class order",
            case["id"]
        );
        let candidates: Vec<String> = case["candidateResults"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["raw"].as_str().unwrap().into())
            .collect();
        assert_eq!(
            api.candidates_to_css(&candidates).unwrap(),
            case["expectedCandidatesToCss"],
            "{} css",
            case["id"]
        );
        assert_eq!(
            api.get_variant_order().unwrap(),
            case["expectedVariantOrder"],
            "{} variant order",
            case["id"]
        );
        assert_eq!(
            api.invalid_candidates().unwrap(),
            case["expectedInvalidCandidates"]
        );
    }
}

#[test]
fn theme_resolution_and_variable_tracking_match_fixed_public_api() {
    for case in fixtures() {
        let api = design(&case);
        for result in case["resolveResults"].as_array().unwrap() {
            assert_eq!(
                api.resolve_theme_value(
                    result["path"].as_str().unwrap(),
                    result["forceInline"].as_bool().unwrap_or(true)
                )
                .unwrap(),
                result["result"]
            );
        }
        let mut api = design(&case);
        assert_eq!(
            serde_json::to_value(api.theme_entries().unwrap()).unwrap(),
            case["trackUsedVariables"]["beforeTracking"]
        );
        api.track_used_variables(case["trackUsedVariables"]["input"].as_str().unwrap())
            .unwrap();
        assert_eq!(
            serde_json::to_value(api.theme_entries().unwrap()).unwrap(),
            case["trackUsedVariables"]["afterTracking"]
        );
        api.track_used_variables(case["trackUsedVariables"]["input"].as_str().unwrap())
            .unwrap();
        assert_eq!(
            serde_json::to_value(api.theme_entries().unwrap()).unwrap(),
            case["trackUsedVariables"]["repeatedTracking"]
        );
    }
}

#[test]
fn theme_entries_keep_all_original_utf16_keys_and_values() {
    #[derive(serde::Deserialize)]
    struct Entry {
        value: crate::text::Text,
        options: u32,
        src: Value,
    }
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    let mut checked = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let Some(expected) = case.get("expectedThemeEntries") else {
            continue;
        };
        let input = crate::text::Text::from_units(
            serde_json::from_value(case["inputUtf16Units"].clone()).unwrap(),
        );
        let mut limits = crate::compiler::tests::limits();
        limits.input_bytes = 262144;
        limits.css_bytes = 262144;
        limits.cache_bytes = 8_000_000;
        let api =
            load_design_system_text(&input, CompileOptions::default(), vec![], limits).unwrap();
        let entries = api.theme_entries().unwrap();
        let json = serde_json::to_string(&entries).unwrap();
        let entries: Vec<(crate::text::Text, Entry)> = serde_json::from_str(&json).unwrap();
        assert_eq!(
            entries.len(),
            expected.as_array().unwrap().len(),
            "{}",
            case["id"]
        );
        for ((key, entry), expected) in entries.iter().zip(expected.as_array().unwrap()) {
            assert_eq!(
                key.units(),
                serde_json::from_value::<Vec<u16>>(expected["keyUtf16Units"].clone()).unwrap(),
                "{} key",
                case["id"]
            );
            assert_eq!(
                entry.value.units(),
                serde_json::from_value::<Vec<u16>>(expected["valueUtf16Units"].clone()).unwrap(),
                "{} value",
                case["id"]
            );
            assert_eq!(entry.src, json!({"undefined":true}));
            assert!(entry.options <= 31);
        }
        checked += 1;
    }
    assert_eq!(checked, 111);
}

#[test]
fn theme_entry_output_obeys_the_shared_cache_budget() {
    let mut api = design(&fixtures()[0]);
    api.compiler.limits.cache_bytes = api.retained_bytes();
    assert!(matches!(
        api.theme_entries(),
        Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes"
    ));
}

#[test]
fn theme_tracking_decodes_escaped_keys_and_reports_invalid_code_points() {
    for (options, prefix) in [("", ""), ("prefix(tw)", "tw-")] {
        let input = format!(
            "@theme {options} {{ --color-\\d800: red; --color-�: blue; }} @tailwind utilities;"
        );
        let mut api = load_design_system(
            &input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        api.track_used_variables(&format!("var(--{prefix}color-\\d800)"))
            .unwrap();
        let entries = api.theme_entries().unwrap();
        assert_eq!(entries[0].1.options, 16);
        assert_eq!(entries[1].1.options, 0);
        let mut raw = crate::text::Text::from(format!("var(--{prefix}color-"));
        raw.push_text(&crate::text::Text::from_units(vec![0xd800]));
        raw.push_str(")");
        api.track_used_variables_text(&raw).unwrap();
        assert_eq!(api.theme_entries().unwrap(), entries);
        assert!(
            matches!(api.track_used_variables(&format!("var(--{prefix}missing-\\110000)")), Err(CompileError::RangeError { message }) if message == "Invalid code point 1114112")
        );
    }
}

#[test]
fn theme_entries_preserve_import_sources_without_tracking_design_system_root() {
    let mut content = crate::text::Text::from("@theme static { --first: red; --second: ");
    content.push_text(&crate::text::Text::from_units(vec![0xd800]));
    content.push_str("; }");
    for path in [Some("/virtual/a.css".to_owned()), Some(String::new()), None] {
        for from in [None, Some(String::new()), Some("entry.css".into())] {
            let api = load_design_system(
                "@theme static { --root: root; } @import \"a.css\"; @tailwind utilities;",
                CompileOptions {
                    from: from.clone(),
                    ..Default::default()
                },
                vec![Stylesheet {
                    id: "a.css".into(),
                    path: path.clone(),
                    base: Some("/virtual".into()),
                    content: content.clone(),
                }],
                crate::compiler::tests::limits(),
            )
            .unwrap();
            let entries = api.theme_entries().unwrap();
            assert!(entries[0].1.src.is_none());
            if from.is_some() && path.as_deref() != Some("") {
                let first = entries[1].1.src.as_ref().unwrap();
                let second = entries[2].1.src.as_ref().unwrap();
                assert!(std::sync::Arc::ptr_eq(&first.source, &second.source));
                assert_eq!(
                    first.source.file.as_deref(),
                    Some(path.as_deref().unwrap_or("a.css"))
                );
                assert_eq!(first.source.code, content);
                assert_eq!((first.start, first.end), (16, 28));
                assert_eq!((second.start, second.end), (30, 41));
            } else {
                assert!(entries.iter().all(|(_, entry)| entry.src.is_none()));
            }
            #[derive(serde::Deserialize)]
            struct Source {
                file: Option<String>,
                code: crate::text::Text,
            }
            let json = serde_json::to_string(&entries).unwrap();
            if from.is_some() && path.as_deref() != Some("") {
                // Read the tuple directly; buffering its code through Value would lose units.
                #[derive(serde::Deserialize)]
                struct Entry {
                    value: crate::text::Text,
                    options: u32,
                    src: Box<serde_json::value::RawValue>,
                }
                let parsed: Vec<(crate::text::Text, Entry)> = serde_json::from_str(&json).unwrap();
                let (source, start, end): (Source, usize, usize) =
                    serde_json::from_str(parsed[2].1.src.get()).unwrap();
                assert_eq!(
                    source.file.as_deref(),
                    Some(path.as_deref().unwrap_or("a.css"))
                );
                assert_eq!(source.code, content);
                assert_eq!(parsed[2].1.value.units(), [0xd800]);
                assert_eq!(parsed[2].1.options, 8);
                assert_eq!((start, end), (30, 41));
            }
        }
    }
}

#[test]
fn variant_order_preserves_distinct_raw_cache_keys_with_equal_json_values() {
    let mut api = design(&fixtures()[0]);
    let first = api.parse_variant("data-[a_b]").unwrap();
    let second = api.parse_variant("data-[a b]").unwrap();
    assert_eq!(first, second);
    let entries = api.variant_order_entries().unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|(raw, _, order)| (raw.as_str(), *order))
            .collect::<Vec<_>>(),
        [("data-[a_b]", 0), ("data-[a b]", 1)]
    );
    assert_eq!(api.parse_variant("data-[a_b]").unwrap(), first);
    assert_eq!(api.variant_order_entries().unwrap(), entries);
}

#[test]
fn custom_functional_completions_resolve_literals_spacing_and_theme_namespaces() {
    let api = load_design_system(
        "@theme { --spacing:.25rem; --color-one: red; --color-two: blue; } @utility probe-* { a: --value('literal', --color-*); b: --spacing(--value(integer)); c: --modifier('option', --color-*); }",
        CompileOptions::default(), vec![], { let mut limits = crate::compiler::tests::limits(); limits.cache_bytes = 8_000_000; limits },
    ).unwrap();
    let list = api.get_class_list().unwrap();
    let probes = list
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry[0].as_str().unwrap().starts_with("probe-"))
        .cloned()
        .collect::<Vec<_>>();
    let mut expected = SPACING_SUGGESTIONS
        .iter()
        .filter(|value| !value.contains('.'))
        .map(|value| format!("probe-{value}"))
        .chain([
            "probe-literal".into(),
            "probe-one".into(),
            "probe-two".into(),
        ])
        .collect::<Vec<_>>();
    expected
        .sort_by(|a, b| crate::compiler::compare_classes(&a.as_str().into(), &b.as_str().into()));
    assert_eq!(
        probes,
        expected
            .into_iter()
            .map(|name| json!([name,{"modifiers":["option","one","two"]}]))
            .collect::<Vec<_>>()
    );
}

#[test]
fn respect_important_can_be_disabled_without_disabling_candidate_important() {
    let mut api = design(&fixtures()[0]);
    api.important = true;
    let ordinary = api.parse_candidate("w-[1px]").unwrap().remove(0);
    let explicit = api.parse_candidate("w-[1px]!").unwrap().remove(0);
    assert_eq!(
        api.compile_ast_nodes(&ordinary, 0).unwrap()[0]["node"]["nodes"][0]["important"],
        false
    );
    assert_eq!(
        api.compile_ast_nodes(&ordinary, 1).unwrap()[0]["node"]["nodes"][0]["important"],
        true
    );
    assert_eq!(
        api.compile_ast_nodes(&explicit, 0).unwrap()[0]["node"]["nodes"][0]["important"],
        true
    );
}

#[test]
fn completion_quota_rejects_before_metadata_allocation_and_preserves_cached_candidates() {
    let mut api = design(&fixtures()[0]);
    let parsed = api.parse_candidate("flex").unwrap();
    api.compiler.limits.cache_bytes = api.retained_bytes + 4096;
    assert!(
        matches!(api.get_class_list(), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
    );
    assert_eq!(api.parse_candidate("flex").unwrap(), parsed);
    assert_eq!(
        api.candidates_to_css(&["flex".into()]).unwrap(),
        json!([".flex {\n  display: flex;\n}\n"])
    );
}

#[test]
fn cached_public_values_share_the_session_budget_before_cloning() {
    let mut api = design(&fixtures()[0]);
    api.parse_candidate("hover:flex").unwrap();
    api.compiler.limits.cache_bytes = api.retained_bytes;
    assert!(
        matches!(api.parse_candidate("hover:flex"), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
    );
    assert!(
        matches!(api.parse_variant("hover"), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
    );
}

#[test]
fn candidate_arbitrary_variant_preserves_the_original_cache_key() {
    let mut api = design(&fixtures()[0]);
    api.parse_candidate("[&_p]:flex").unwrap();
    assert_eq!(
        api.variant_order_entries()
            .unwrap()
            .iter()
            .map(|(raw, _, _)| raw.as_str())
            .collect::<Vec<_>>(),
        ["[&_p]"]
    );
    api.parse_variant("[& p]").unwrap();
    assert_eq!(api.variant_order_entries().unwrap().len(), 2);
}

#[test]
fn edited_candidate_payload_obeys_input_limits_before_utility_generation() {
    let mut api = design(&fixtures()[0]);
    let mut candidate = api.parse_candidate("w-[1px]").unwrap().remove(0);
    api.compiler.limits.input_bytes = 1024;
    candidate["value"]["value"] = json!("x".repeat(1025));
    assert!(
        matches!(api.compile_ast_nodes(&candidate, 1), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
    );
    assert!(
        matches!(api.print_candidate(&candidate), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
    );
}

#[test]
fn theme_tracking_preserves_encounter_order_before_range_failure() {
    for (options, prefix) in [("", ""), ("prefix(tw)", "tw-")] {
        for valid_first in [true, false] {
            let mut api = load_design_system(
                &format!("@theme {options} {{ --z: 1; }} @tailwind utilities;"),
                CompileOptions::default(),
                vec![],
                crate::compiler::tests::limits(),
            )
            .unwrap();
            let valid = format!("var(--{prefix}z)");
            let invalid = format!("var(--{prefix}\\110000)");
            let raw = if valid_first {
                format!("{valid} {invalid}")
            } else {
                format!("{invalid} {valid}")
            };
            assert!(matches!(
                api.track_used_variables(&raw),
                Err(CompileError::RangeError { .. })
            ));
            assert_eq!(
                api.theme_entries().unwrap()[0].1.options,
                if valid_first { 16 } else { 0 }
            );
            api.track_used_variables(&valid).unwrap();
            assert_eq!(api.theme_entries().unwrap()[0].1.options, 16);
        }
    }
}

#[test]
fn theme_tracking_limit_preserves_all_used_bits_and_retained_bytes() {
    let mut api = load_design_system(
        "@theme { --a: red; --b: blue; } @tailwind utilities;",
        CompileOptions::default(),
        vec![],
        crate::compiler::tests::limits(),
    )
    .unwrap();
    let entries = api.theme_entries().unwrap();
    let retained = api.retained_bytes();
    let original_limit = api.compiler.limits.cache_bytes;
    // The old per-key check admits --a, then fails on --b after committing --a.
    api.compiler.limits.cache_bytes = retained + 160;
    assert!(matches!(
        api.track_used_variables("var(--a) var(--b)"),
        Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes"
    ));
    assert_eq!(api.retained_bytes(), retained);
    api.compiler.limits.cache_bytes = original_limit;
    assert_eq!(api.theme_entries().unwrap(), entries);
    api.track_used_variables("var(--a) var(--b)").unwrap();
    assert!(api
        .theme_entries()
        .unwrap()
        .iter()
        .all(|(_, token)| token.options == 16));
}

#[test]
fn theme_tracking_budgets_unknown_variables_and_small_parser_allocations() {
    for raw in [
        "var(--missing) ".repeat(1024),
        "a".into(),
        "\\".into(),
        "(".into(),
        "a,b,a,b,a,b".into(),
        "((((((((((((((((x))))))))))))))))".into(),
    ] {
        let mut api = load_design_system(
            "@theme { --z: red; } @tailwind utilities;",
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let entries = api.theme_entries().unwrap();
        let retained = api.retained_bytes();
        let original_limit = api.compiler.limits.cache_bytes;
        api.compiler.limits.cache_bytes = retained + 1;
        assert!(
            matches!(
                api.track_used_variables(&raw),
                Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes"
            ),
            "temporary parser allocations must be budgeted: {} units",
            raw.encode_utf16().count()
        );
        assert_eq!(api.retained_bytes(), retained);
        api.compiler.limits.cache_bytes = original_limit;
        assert_eq!(api.theme_entries().unwrap(), entries);
    }
}
