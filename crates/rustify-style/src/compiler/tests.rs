use super::*;

pub(super) fn limits() -> Limits {
    Limits {
        input_bytes: 64_000,
        asset_bytes: 256_000,
        candidate_count: 256,
        ast_depth: 32,
        css_bytes: 256_000,
        queued_bytes: 64_000,
        cache_bytes: 256_000,
    }
}
fn compiler() -> Compiler {
    Compiler::new(
        CompilerInput {
            css: "@tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap()
}

#[test]
fn custom_variant_matrix_matches_official_css_and_errors() {
    assert_official_matrix(include_str!(
        "../../../../tests/utility/compiler/custom-variants-fixtures.json"
    ));
}

#[test]
fn parser_error_matrix_matches_official_css_and_errors() {
    assert_official_matrix(include_str!(
        "../../../../tests/utility/compiler/parser-errors-fixtures.json"
    ));
}

#[test]
fn apply_matrix_matches_official_css_and_errors() {
    assert_official_matrix(include_str!(
        "../../../../tests/utility/compiler/apply-fixtures.json"
    ));
}

#[test]
fn variant_parameter_matrix_matches_official_css_and_errors() {
    assert_official_matrix(include_str!(
        "../../../../tests/utility/compiler/variant-parameters-fixtures.json"
    ));
}

#[test]
fn applied_global_resources_do_not_reject_flat_gpu_declarations() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@utility base-card { @apply w-[137px] bg-[#123456]; } @utility applied-card { @apply base-card border-2 border-[#abcdef]; } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        vec![],
        limits(),
    ).unwrap();
    let prepared = compiler.prepare_add(&["applied-card".into()]).unwrap();
    let program = &prepared.report().gpu_programs[0];
    assert!(program.conditions.is_empty(), "{:?}", program.conditions);
    assert!(program
        .declarations
        .iter()
        .any(|declaration| declaration.property == "width"));
    assert!(prepared
        .report()
        .css
        .contains("@property --tw-border-style"));
}

fn assert_official_matrix(fixture: &str) {
    let fixture: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let mut failures = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let assets = case
            .get("assets")
            .map(|value| serde_json::from_value::<Vec<SourceAsset>>(value.clone()).unwrap())
            .unwrap_or_default();
        let result = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: case["entry"].as_str().unwrap_or("input.css").into(),
            },
            assets,
            limits(),
        );
        if let Some(expected) = case["expectedError"]
            .as_str()
            .or_else(|| case["expectedError"]["message"].as_str())
        {
            if !matches!(result, Err(CompileError::Syntax {ref message, ..}) if message == expected)
            {
                failures.push(format!(
                    "{}: expected {expected:?}, got {:?}",
                    case["id"],
                    result.err()
                ));
            }
            continue;
        }
        let mut compiler = match result {
            Ok(compiler) => compiler,
            Err(error) => {
                failures.push(format!("{}: constructor {error:?}", case["id"]));
                continue;
            }
        };
        let steps = case.get("buildSteps").and_then(|value| value.as_array());
        for step in steps
            .map(Vec::as_slice)
            .unwrap_or(std::slice::from_ref(case))
        {
            let candidates: Vec<String> =
                serde_json::from_value(step["candidates"].clone()).unwrap();
            let prepared = match compiler.prepare_add(&candidates) {
                Ok(prepared) => prepared,
                Err(error) => {
                    failures.push(format!("{}: preparation {error:?}", case["id"]));
                    break;
                }
            };
            if prepared.report().css != step["expectedCss"].as_str().unwrap()
                || !prepared.report().unsupported.is_empty()
            {
                failures.push(format!(
                    "{}: actual {:?}, expected {:?}, pending {:?}",
                    case["id"],
                    prepared.report().css,
                    step["expectedCss"],
                    prepared.report().unsupported
                ));
            }
            compiler.commit(prepared).unwrap();
        }
    }
    assert!(
        failures.is_empty(),
        "{} mismatches; first cases: {:#?}",
        failures.len(),
        &failures[..failures.len().min(12)]
    );
}

#[test]
fn custom_variable_candidates_require_an_exact_registered_root() {
    for (candidate, expected) in [
        (
            "probe-(--value)",
            ".probe-\\(--value\\) {\n  color: red;\n}\n",
        ),
        ("probe-child-(--value)", ""),
        ("probe-calc(1)", ""),
    ] {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: "@utility probe-* { color:red; } @tailwind utilities;".into(),
                entry: "input.css".into(),
            },
            vec![],
            limits(),
        )
        .unwrap();
        let prepared = compiler.prepare_add(&[candidate.into()]).unwrap();
        assert_eq!(prepared.report().css, expected, "{candidate}");
    }
}

#[test]
fn removed_custom_sort_declarations_do_not_keep_theme_variables_alive() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --color-brand:#123456; } @utility probe { --tw-sort:var(--color-brand); } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        vec![],
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&["probe".into()]).unwrap();
    assert_eq!(prepared.report().css, "");
}

#[test]
fn reference_custom_branches_do_not_keep_theme_variables_alive() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --color-brand:#123456; } @utility probe { @media reference { color:var(--color-brand); } } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        vec![],
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&["probe".into()]).unwrap();
    assert_eq!(prepared.report().css, "");
}

#[test]
fn reference_custom_branches_do_not_change_rule_sorting() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@utility zulu { color:red; @media reference { width:13px; } } @utility alpha { color:blue; } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        vec![],
        limits(),
    )
    .unwrap();
    let prepared = compiler
        .prepare_add(&["zulu".into(), "alpha".into()])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        ".alpha {\n  color: blue;\n}\n.zulu {\n  color: red;\n}\n"
    );
}

#[test]
fn theme_dependencies_and_keyframes_match_official_css() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/theme-fixtures.json"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "input.css".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let candidates = serde_json::from_value::<Vec<String>>(case["candidates"].clone()).unwrap();
        let prepared = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(
            prepared.report.css,
            case["expectedCss"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert!(prepared.report.unsupported.is_empty(), "{}", case["id"]);
    }
}

#[test]
fn gpu_reports_supply_raw_theme_and_registered_property_bindings() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --color-brand: #123456; --spacing: 4px; } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler
        .prepare_add(&["ring-2".into(), "bg-brand".into(), "p-3".into()])
        .unwrap();
    let report = serde_json::to_value(prepared.report()).unwrap();
    assert_eq!(report["gpu_variables"]["--color-brand"], "#123456");
    assert_eq!(report["gpu_variables"]["--spacing"], "4px");
    assert_eq!(report["gpu_variables"]["--tw-ring-offset-width"], "0px");
    assert_eq!(report["gpu_variables"]["--tw-ring-offset-color"], "#fff");
}

#[test]
fn dynamic_candidates_match_official_smoke_without_a_scanner() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["bg-[#123456]".into(), "w-[137px]".into()])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        include_str!("../../../../tests/utility/compiler/compiler-smoke.css")
    );
    assert_eq!(compiler.report().css, "");
    assert_eq!(compiler.commit(prepared).unwrap().revision, 1);
}

#[test]
fn repeated_candidates_do_not_parse_or_generate_again() {
    let mut compiler = compiler();
    let prepared = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    compiler.commit(prepared).unwrap();
    let repeated = compiler
        .prepare_add(&["w-[137px]".into(), "w-[137px]".into()])
        .unwrap();
    assert_eq!(repeated.report().metrics.parsed_candidates, 0);
    assert_eq!(repeated.report().metrics.generated_candidates, 0);
    assert!(!repeated.report().changed);
    assert_eq!(repeated.report().revision, 1);
}

#[test]
fn preparation_failure_preserves_committed_state_and_does_not_poison_cache() {
    let mut compiler = compiler();
    let prepared = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    compiler.commit(prepared).unwrap();
    let previous = compiler.report().clone();
    let oversized = "x".repeat(64_001);
    assert!(matches!(
        compiler.prepare_add(&[oversized]),
        Err(CompileError::Limit { .. })
    ));
    assert_eq!(compiler.report(), &previous);
    let next = compiler.prepare_add(&["bg-[#123456]".into()]).unwrap();
    assert_eq!(next.report().metrics.parsed_candidates, 1);
}

#[test]
fn discarded_candidates_are_not_committed_or_cached() {
    let mut compiler = compiler();
    let prepared = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    assert!(matches!(
        compiler.prepare_add(&[]),
        Err(CompileError::PendingPrepared)
    ));
    compiler.discard(prepared).unwrap();
    assert_eq!(compiler.revision(), 0);
    let retry = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    assert_eq!(retry.report().metrics.parsed_candidates, 1);
}

#[test]
fn random_batch_order_matches_single_sorted_union() {
    let input = ["bg-[#123456]", "w-[137px]", "p-[12px]", "block"];
    let mut once = compiler();
    let single = once.prepare_add(&input.map(str::to_string)).unwrap();
    let mut incremental = compiler();
    for raw in input.into_iter().rev() {
        let prepared = incremental.prepare_add(&[raw.into()]).unwrap();
        incremental.commit(prepared).unwrap();
    }
    assert_eq!(incremental.report().css, single.report().css);
    assert_eq!(
        incremental.report().gpu_programs,
        single.report().gpu_programs
    );
}

#[test]
fn arbitrary_properties_accept_opacity_modifiers() {
    for (class, css) in [
        (
            "[color:red]/100",
            ".\\[color\\:red\\]\\/100 {\n  color: red;\n}\n",
        ),
        (
            "[padding:7px]/[1]",
            ".\\[padding\\:7px\\]\\/\\[1\\] {\n  padding: 7px;\n}\n",
        ),
    ] {
        let mut compiler = compiler();
        let prepared = compiler.prepare_add(&[class.into()]).unwrap();
        assert_eq!(prepared.report().css, css);
        assert!(prepared.report().unsupported.is_empty());
    }
}

#[test]
fn arbitrary_property_matrix_matches_official_css() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/arbitrary-properties-fixtures.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let candidates = case["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|candidate| candidate.as_str().unwrap().into())
            .collect::<Vec<_>>();
        let prepared = compiler.prepare_add(&candidates).unwrap();
        if prepared.report().css != case["expectedCss"].as_str().unwrap() {
            count += 1;
            if failures.len() < 10 {
                failures.push(format!(
                    "{}: expected {:?}, actual {:?}; rejected {:?}, pending {:?}",
                    case["id"],
                    case["expectedCss"],
                    prepared.report().css,
                    prepared.report().rejected,
                    prepared.report().unsupported
                ));
            }
        }
    }
    assert_eq!(count, 0, "{}", failures.join("\n"));
}

#[test]
fn implemented_branches_are_admitted_and_unknown_roots_are_rejected() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&[
            "mask-[url(example)]".into(),
            "[color:red]/50".into(),
            "nonsense-utility".into(),
        ])
        .unwrap();
    assert!(prepared.report().unsupported.is_empty());
    assert!(prepared
        .report()
        .admitted
        .contains(&"mask-[url(example)]".into()));
    assert!(prepared
        .report()
        .admitted
        .contains(&"[color:red]/50".into()));
    assert_eq!(
        prepared.report().rejected[0].kind,
        DiagnosticKind::UnknownUtility
    );
}

#[test]
fn scope_property_and_pseudo_nesting_match_full_official_fixture() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@scope ([data-utility-probe]) { @tailwind utilities; }".into(),
            entry: "input.css".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler
        .prepare_add(&[
            "w-[137px]".into(),
            "bg-[#123456]".into(),
            "ring-[3px]".into(),
            "before:content-[\"probe\"]".into(),
        ])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        include_str!("../../../../tests/utility/compiler/css-scope-property-nesting.css")
    );
    assert!(prepared.report().unsupported.is_empty());
}

#[test]
fn all_static_utilities_match_each_official_candidate_and_sorted_union() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/static-fixtures.json"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 889);
    for case in cases {
        let raw = case["root"].as_str().unwrap();
        for (candidate, field) in [
            (raw.to_string(), "expectedCss"),
            (format!("{raw}!"), "expectedImportantCss"),
            (format!("hover:{raw}"), "expectedHoverCss"),
        ] {
            let mut compiler = compiler();
            let prepared = compiler.prepare_add(&[candidate]).unwrap();
            assert_eq!(
                prepared.report().css,
                case[field].as_str().unwrap(),
                "{} {field}",
                case["id"].as_str().unwrap()
            );
        }
    }
    for case in fixture["probeCases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let prepared = compiler
            .prepare_add(&[case["root"].as_str().unwrap().into()])
            .unwrap();
        assert_eq!(
            prepared.report().css,
            case["expectedCss"].as_str().unwrap(),
            "{}",
            case["id"].as_str().unwrap()
        );
    }
    let mut large = limits();
    large.candidate_count = 1_000;
    large.css_bytes = 2_000_000;
    large.cache_bytes = 4_000_000;
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        large,
    )
    .unwrap();
    let candidates = cases
        .iter()
        .rev()
        .map(|case| case["root"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let prepared = compiler.prepare_add(&candidates).unwrap();
    assert_eq!(prepared.report().admitted.len(), 889);
    assert_eq!(
        prepared.report().css,
        fixture["union"]["expectedCss"].as_str().unwrap()
    );
}

#[test]
fn ring_shadow_content_and_pseudo_variants_match_official_cases() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/effects-fixtures.json"
    ))
    .unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let candidates = case["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        let prepared = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(
            prepared.report().css,
            case["expectedCss"].as_str().unwrap(),
            "{}",
            case["id"].as_str().unwrap()
        );
    }
}

#[test]
fn all_nonresponsive_static_variants_match_official_cases_and_union() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/variant-fixtures.json"
    ))
    .unwrap();
    assert_eq!(fixtures["staticVariantRoots"].as_array().unwrap().len(), 66);
    for case in fixtures["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let candidates = case["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        let prepared = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(
            prepared.report().css,
            case["expectedCss"].as_str().unwrap(),
            "{}",
            case["id"].as_str().unwrap()
        );
    }
}

#[test]
fn rounded_arbitrary_pixels_match_official_output_and_structured_gpu_value() {
    let mut compiler = compiler();
    let prepared = compiler.prepare_add(&["rounded-[8px]".into()]).unwrap();
    assert_eq!(
        prepared.report().css,
        ".rounded-\\[8px\\] {\n  border-radius: 8px;\n}\n"
    );
    assert_eq!(
        prepared.report().gpu_programs[0].declarations[0].property,
        "border-radius"
    );
    assert_eq!(
        prepared.report().gpu_programs[0].declarations[0].value,
        GpuValue::Pixels { value: "8".into() }
    );
    assert!(prepared.report().unsupported.is_empty());
}

#[test]
fn one_candidate_preserves_all_valid_static_and_functional_interpretations() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --width-full: 13px; } @tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&["w-full".into()]).unwrap();
    assert_eq!(
        prepared.report().css,
        ":root, :host {\n  --width-full: 13px;\n}\n.w-full {\n  width: 100%;\n}\n.w-full {\n  width: var(--width-full);\n}\n"
    );
    assert_eq!(prepared.report().admitted, ["w-full"]);
    assert_eq!(prepared.report().gpu_programs.len(), 2);
}

#[test]
fn all_limits_must_be_positive_and_boundaries_are_inclusive() {
    let mut invalid = limits();
    invalid.ast_depth = 0;
    assert!(matches!(
        Compiler::new(
            CompilerInput {
                css: "".into(),
                entry: "".into()
            },
            Vec::new(),
            invalid
        ),
        Err(CompileError::InvalidLimit { .. })
    ));
    let mut exact = limits();
    exact.candidate_count = 1;
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        exact,
    )
    .unwrap();
    let prepared = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    compiler.commit(prepared).unwrap();
    assert!(
        matches!(compiler.prepare_add(&["bg-[#123456]".into()]), Err(CompileError::Limit { resource, .. }) if resource == "candidate_count")
    );
    assert_eq!(compiler.report().admitted, ["w-[137px]"]);
}

#[test]
fn css_parser_preserves_nested_rules_declaration_order_and_duplicates() {
    let input = "@layer utilities { .x { color: red; color: blue !important; & > p { --data: {a:b;c:d}; content: '};'; } } }";
    let ast = parser::parse(input, 32).unwrap();
    let rendered = ast::to_css(&ast);
    assert_eq!(
        rendered,
        "@layer utilities {\n  .x {\n    color: red;\n    color: blue !important;\n    & > p {\n      --data: {a:b;c:d};\n      content: '};';\n    }\n  }\n}\n"
    );
    assert_eq!(
        parser::parse(&rendered.to_utf8().unwrap(), 32).unwrap(),
        ast
    );
}

#[test]
fn value_parser_preserves_official_unclosed_function_shape() {
    for (input, expected) in [("var(--probe", "var()--probe"), ("f(g(x", "f(g())x")] {
        assert_eq!(value::to_css(&value::parse(input, 32).unwrap()), expected);
    }
}

#[test]
fn value_parser_preserves_functions_quotes_and_separators() {
    let input = "calc(100% - var(--size, 10px)) url('some_file.png')";
    assert_eq!(value::to_css(&value::parse(input, 32).unwrap()), input);
    assert_eq!(
        value::decode_arbitrary("url(some_file.png)_var(--my_token,_red)", 32).unwrap(),
        "url(some_file.png) var(--my_token, red)"
    );
    assert!(matches!(
        value::parse("f(g(h(1)))", 1),
        Err(CompileError::Limit { .. })
    ));
}

#[test]
fn gpu_ir_is_constructed_from_the_same_smoke_declarations() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["w-[137px]".into(), "bg-[#123456]".into()])
        .unwrap();
    let programs = &prepared.report().gpu_programs;
    assert_eq!(
        programs[0].declarations[0].value,
        GpuValue::Pixels {
            value: "137".into()
        }
    );
    assert_eq!(
        programs[1].declarations[0].value,
        GpuValue::Color {
            value: "#123456".into()
        }
    );
}

#[test]
fn javascript_plugin_and_config_execution_is_explicitly_unsupported() {
    for css in ["@plugin 'test';", "@config 'test';"] {
        assert!(matches!(
            Compiler::new(
                CompilerInput {
                    css: css.into(),
                    entry: "".into()
                },
                Vec::new(),
                limits()
            ),
            Err(CompileError::UnsupportedFeature { .. })
        ));
    }
}

#[test]
fn unused_nested_theme_is_pruned_after_rule_optimization() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: ".nested { @theme { --color-card: red; } } @tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&[]).unwrap();
    assert_eq!(
        prepared.report().css,
        ".nested {\n  :root, :host {\n  }\n}\n"
    );
}

#[test]
fn theme_declarations_preserve_source_order_and_unused_tokens_are_pruned() {
    let mut compiler = Compiler::new(CompilerInput { css: "@theme { --spacing: 0.25rem; --color-primary: #123456; --color-unused: red; } @tailwind utilities;".into(), entry: "".into() }, Vec::new(), limits()).unwrap();
    let prepared = compiler
        .prepare_add(&["p-4".into(), "gap-2".into(), "bg-primary".into()])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        ":root, :host {\n  --spacing: 0.25rem;\n  --color-primary: #123456;\n}\n.gap-2 {\n  gap: calc(var(--spacing) * 2);\n}\n.bg-primary {\n  background-color: var(--color-primary);\n}\n.p-4 {\n  padding: calc(var(--spacing) * 4);\n}\n"
    );
}

#[test]
fn implemented_variants_match_official_nesting_and_order() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&[
            "focus-visible:w-[137px]".into(),
            "hover:bg-[#123456]".into(),
        ])
        .unwrap();
    assert!(prepared.report().unsupported.is_empty());
    assert_eq!(
        prepared.report().css,
        ".hover\\:bg-\\[\\#123456\\] {\n  &:hover {\n    @media (hover: hover) {\n      background-color: #123456;\n    }\n  }\n}\n.focus-visible\\:w-\\[137px\\] {\n  &:focus-visible {\n    width: 137px;\n  }\n}\n"
    );
}

#[test]
fn top_level_arbitrary_value_separators_cannot_become_extra_declarations() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&[
            "w-[1px;color:red]".into(),
            "[color:red;display:none]".into(),
        ])
        .unwrap();
    assert!(prepared.report().admitted.is_empty());
    assert_eq!(prepared.report().rejected.len(), 2);
    assert!(prepared.report().css.is_empty());
}

#[test]
fn invalid_spacing_keywords_do_not_emit_css() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["p-auto".into(), "p-full".into(), "gap-full".into()])
        .unwrap();
    assert!(prepared.report().admitted.is_empty());
    assert!(prepared.report().css.is_empty());
}

#[test]
fn prepared_ast_serializes_to_exact_official_css() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["w-[137px]".into(), "bg-[#123456]".into()])
        .unwrap();
    assert_eq!(ast::to_css(prepared.ast()), prepared.report().css);
}

#[test]
fn failed_css_and_cache_limits_preserve_last_committed_snapshot() {
    for resource in ["css_bytes", "cache_bytes"] {
        let mut compiler = compiler();
        let first = compiler.prepare_add(&["w-[1px]".into()]).unwrap();
        compiler.commit(first).unwrap();
        let old = compiler.report().clone();
        match resource {
            "css_bytes" => compiler.limits.css_bytes = old.css.len(),
            _ => compiler.limits.cache_bytes = old.metrics.cache_bytes,
        }
        assert!(
            matches!(compiler.prepare_add(&["bg-[#123456]".into()]), Err(CompileError::Limit { resource: actual, .. }) if actual == resource)
        );
        assert_eq!(compiler.report(), &old);
        let repeated = compiler.prepare_add(&["w-[1px]".into()]).unwrap();
        assert_eq!(repeated.report().metrics.parsed_candidates, 0);
        compiler.discard(repeated).unwrap();
    }
}

#[test]
fn source_asset_bytes_and_input_bytes_are_checked_before_parsing() {
    let mut budget = limits();
    budget.asset_bytes = 8;
    assert!(
        matches!(Compiler::new(CompilerInput { css: "@tailwind utilities;".into(), entry: "".into() }, vec![SourceAsset { id: "x".into(), content: "12345678".into() }], budget), Err(CompileError::Limit { resource, .. }) if resource == "asset_bytes")
    );
    let mut budget = limits();
    budget.input_bytes = 18;
    assert!(
        matches!(Compiler::new(CompilerInput { css: "@tailwind utilities;".into(), entry: "".into() }, Vec::new(), budget), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
    );
}

#[test]
fn a_different_session_cannot_commit_a_private_prepared_token() {
    let mut first = compiler();
    let mut other = compiler();
    let prepared = first.prepare_add(&["w-[137px]".into()]).unwrap();
    assert!(matches!(
        other.commit(prepared),
        Err(CompileError::StalePrepared)
    ));
    assert_eq!(other.revision(), 0);
}

#[test]
fn empty_reference_theme_tokens_do_not_add_a_fallback() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme reference { --radius-empty: ; } @tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&["rounded-empty".into()]).unwrap();
    assert_eq!(
        prepared.report().css,
        ".rounded-empty {\n  border-radius: var(--radius-empty);\n}\n"
    );
}

#[test]
fn inline_theme_and_reference_theme_are_css_native_configurations() {
    for (options, color) in [
        ("inline", "#123456"),
        ("reference", "var(--color-primary, #123456)"),
    ] {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: format!(
                    "@theme {options} {{ --color-primary: #123456; }} @tailwind utilities;"
                )
                .into(),
                entry: "".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        let prepared = compiler.prepare_add(&["bg-primary".into()]).unwrap();
        assert_eq!(
            prepared.report().css,
            format!(".bg-primary {{\n  background-color: {color};\n}}\n")
        );
    }
}

#[test]
fn same_property_rules_use_upstream_alphanumeric_class_order() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["w-[10px]".into(), "w-[2px]".into()])
        .unwrap();
    assert_eq!(prepared.report().admitted, ["w-[2px]", "w-[10px]"]);
}

#[test]
fn named_theme_dependencies_emit_once_in_source_order() {
    let mut compiler = Compiler::new(CompilerInput { css: "@theme { --color-base: red; --color-primary: var(--color-base); } @tailwind utilities;".into(), entry: "".into() }, Vec::new(), limits()).unwrap();
    let prepared = compiler.prepare_add(&["bg-primary".into()]).unwrap();
    assert_eq!(
        prepared.report().css,
        ":root, :host {\n  --color-base: red;\n  --color-primary: var(--color-base);\n}\n.bg-primary {\n  background-color: var(--color-primary);\n}\n"
    );
}

#[test]
fn spacing_and_size_theme_namespaces_match_official_precedence() {
    let css = "@theme { --padding-card: 13px; --margin-hero: 7px; --gap-gutters: 9px; --container-wide: 42rem; --height-row: 5rem; } @tailwind utilities;";
    let mut compiler = Compiler::new(
        CompilerInput {
            css: css.into(),
            entry: "".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler
        .prepare_add(&[
            "p-card".into(),
            "m-hero".into(),
            "gap-gutters".into(),
            "w-wide".into(),
            "max-w-wide".into(),
            "min-h-row".into(),
        ])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        ":root, :host {\n  --padding-card: 13px;\n  --margin-hero: 7px;\n  --gap-gutters: 9px;\n  --container-wide: 42rem;\n  --height-row: 5rem;\n}\n.m-hero {\n  margin: var(--margin-hero);\n}\n.min-h-row {\n  min-height: var(--height-row);\n}\n.w-wide {\n  width: var(--container-wide);\n}\n.max-w-wide {\n  max-width: var(--container-wide);\n}\n.gap-gutters {\n  gap: var(--gap-gutters);\n}\n.p-card {\n  padding: var(--padding-card);\n}\n"
    );
}

#[test]
fn spacing_numbers_require_upstream_canonical_representation() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --spacing: .25rem; } @tailwind utilities;".into(),
            entry: "".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let prepared = compiler
        .prepare_add(&["p-01".into(), "w-2.00".into(), "p-0.5".into(), "p-4".into()])
        .unwrap();
    assert_eq!(prepared.report().admitted, ["p-0.5", "p-4"]);
    assert_eq!(prepared.report().rejected.len(), 2);
}

#[test]
fn arbitrary_math_values_format_binary_operators_and_preserve_unary_signs() {
    let mut compiler = compiler();
    let prepared = compiler
        .prepare_add(&["w-[calc(100%-1rem)]".into()])
        .unwrap();
    assert_eq!(
        prepared.report().css,
        ".w-\\[calc\\(100\\%-1rem\\)\\] {\n  width: calc(100% - 1rem);\n}\n"
    );
    for (input, output) in [
        (
            "calc(-3.4e-2*var(--mywidth))",
            "calc(-3.4e-2 * var(--mywidth))",
        ),
        ("clamp(0px,1px+2px,10px)", "clamp(0px, 1px + 2px, 10px)"),
        ("calc(2*-(1px+2px))", "calc(2 * -(1px + 2px))"),
    ] {
        assert_eq!(value::decode_arbitrary(input, 32).unwrap(), output);
    }
}

#[test]
fn generated_and_input_wrapper_ast_depth_are_bounded_before_commit() {
    let mut small = limits();
    small.ast_depth = 2;
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@layer utilities { @tailwind utilities; }".into(),
            entry: "".into(),
        },
        Vec::new(),
        small,
    )
    .unwrap();
    let before = compiler.report().clone();
    assert!(
        matches!(compiler.prepare_add(&["hover:w-[137px]".into()]), Err(CompileError::Limit { resource, limit: 2 }) if resource == "ast_depth")
    );
    assert_eq!(compiler.report(), &before);
    let prepared = compiler.prepare_add(&["w-[137px]".into()]).unwrap();
    assert_eq!(prepared.report().metrics.parsed_candidates, 1);
    assert_eq!(prepared.report().admitted, ["w-[137px]"]);
}

#[test]
fn external_variable_inputs_match_official_incremental_builds() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/external-variables-fixtures.json"
    ))
    .unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: case["input"].as_str().unwrap().into(),
                entry: "/app/input.css".into(),
            },
            Vec::new(),
            limits(),
        )
        .unwrap();
        for (batch, expected) in case["batches"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["expectedBuilds"].as_array().unwrap())
        {
            let batch: Vec<String> = serde_json::from_value(batch.clone()).unwrap();
            let prepared = compiler.prepare_add(&batch).unwrap();
            assert_eq!(
                prepared.report().css,
                expected.as_str().unwrap(),
                "{}: {batch:?}",
                case["id"]
            );
            assert!(prepared
                .report()
                .rejected
                .iter()
                .all(|diagnostic| !diagnostic.candidate.starts_with("--")));
            compiler.commit(prepared).unwrap();
        }
    }
}

#[test]
fn external_variable_discard_does_not_change_committed_dependencies() {
    let mut compiler = Compiler::new(
        CompilerInput {
            css: "@theme { --color-brand: red; } @tailwind utilities;".into(),
            entry: "input.css".into(),
        },
        Vec::new(),
        limits(),
    )
    .unwrap();
    let before = compiler.report().clone();
    let prepared = compiler.prepare_add(&["--color-brand".into()]).unwrap();
    assert!(prepared.report().css.contains("--color-brand: red"));
    compiler.discard(prepared).unwrap();
    assert_eq!(compiler.report(), &before);
    let empty = compiler.prepare_add(&[]).unwrap();
    assert!(!empty.report().css.contains("--color-brand"));
    compiler.commit(empty).unwrap();
    let retried = compiler.prepare_add(&["--color-brand".into()]).unwrap();
    assert!(retried.report().css.contains("--color-brand: red"));
    compiler.commit(retried).unwrap();
    let repeated = compiler.prepare_add(&["--color-brand".into()]).unwrap();
    assert!(!repeated.report().changed);
    assert_eq!(repeated.report().metrics.parsed_candidates, 0);
}

#[test]
fn external_variable_quota_failures_leave_the_committed_snapshot_unchanged() {
    for resource in ["candidate_count", "cache_bytes", "css_bytes"] {
        let mut bounded = limits();
        match resource {
            "candidate_count" => bounded.candidate_count = 1,
            "cache_bytes" => {}
            "css_bytes" => bounded.css_bytes = 20,
            _ => unreachable!(),
        }
        let mut compiler = Compiler::new(
            CompilerInput {
                css: "@theme { --color-brand: red; --color-other: blue; } @tailwind utilities;"
                    .into(),
                entry: "input.css".into(),
            },
            Vec::new(),
            bounded,
        )
        .unwrap();
        if resource == "cache_bytes" {
            let mut initial_limit = limits();
            initial_limit.cache_bytes = 1;
            assert!(matches!(Compiler::new(
                CompilerInput {
                    css: "@theme { --color-brand: red; --color-other: blue; } @tailwind utilities;".into(),
                    entry: "input.css".into(),
                }, Vec::new(), initial_limit,
            ), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes"));
            compiler.limits.cache_bytes = compiler.report().metrics.cache_bytes;
        }
        let before = compiler.report().clone();
        assert!(
            matches!(compiler.prepare_add(&["--color-brand".into(), "--color-other".into()]),
            Err(CompileError::Limit { resource: actual, .. }) if actual == resource)
        );
        assert_eq!(compiler.report(), &before);
        let prepared = compiler.prepare_add(&[]).unwrap();
        assert_eq!(prepared.report().css, before.css);
        compiler.commit(prepared).unwrap();
    }
}

/// Fixed-seed batches: any partition and order of a candidate set commits the
/// same stylesheet as compiling the union once, re-adding is a no-op, and a
/// discarded preparation leaves no trace in later output.
#[test]
fn fixed_seed_batches_match_the_one_shot_union() {
    fn corpus(file: &str) -> Vec<String> {
        let fixture: serde_json::Value = serde_json::from_str(file).unwrap();
        let mut candidates = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            if let Some(list) = case["candidates"].as_array() {
                candidates.extend(
                    list.iter()
                        .filter_map(|value| value.as_str())
                        .map(str::to_string),
                );
            } else if let Some(root) = case["root"].as_str() {
                candidates.push(root.to_string());
            }
        }
        candidates
    }
    struct Rng(u64);
    impl Rng {
        fn next(&mut self, bound: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % bound as u64) as usize
        }
        fn shuffle(&mut self, values: &mut [String]) {
            for index in (1..values.len()).rev() {
                values.swap(index, self.next(index + 1));
            }
        }
    }
    let large = Limits {
        input_bytes: 1 << 20,
        asset_bytes: 1 << 20,
        candidate_count: 1 << 16,
        ast_depth: 64,
        css_bytes: 64 << 20,
        queued_bytes: 1 << 20,
        cache_bytes: 1 << 30,
    };
    let fresh = || {
        Compiler::new(
            CompilerInput {
                css: "@tailwind utilities;".into(),
                entry: "".into(),
            },
            Vec::new(),
            large.clone(),
        )
        .unwrap()
    };
    let run = |candidates: &[String], seed: u64| {
        let mut rng = Rng(seed);
        let mut order = candidates.to_vec();
        rng.shuffle(&mut order);
        let mut compiler = fresh();
        let mut rest = order.as_slice();
        while !rest.is_empty() {
            let size = 1 + rng.next(rest.len().min(400));
            let (batch, tail) = rest.split_at(size);
            if rng.next(4) == 0 {
                let discarded = compiler.prepare_add(batch).unwrap();
                compiler.discard(discarded).unwrap();
            }
            let prepared = compiler.prepare_add(batch).unwrap();
            compiler.commit(prepared).unwrap();
            rest = tail;
        }
        let css = compiler.report().css.clone();
        let mut again = order[..order.len().min(200)].to_vec();
        rng.shuffle(&mut again);
        let repeat = compiler.prepare_add(&again).unwrap();
        assert!(
            !repeat.report().changed,
            "seed {seed}: re-adding committed candidates"
        );
        assert_eq!(
            repeat.report().css,
            css,
            "seed {seed}: re-adding changed the stylesheet"
        );
        compiler.discard(repeat).unwrap();
        css
    };

    // The official compiler's one-shot output for all 889 static roots.
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/compiler/static-fixtures.json"
    ))
    .unwrap();
    let roots = corpus(include_str!(
        "../../../../tests/utility/compiler/static-fixtures.json"
    ));
    for seed in [
        0x9e37_79b9_7f4a_7c15,
        0x2545_f491_4f6c_dd1d,
        0x1234_5678_9abc_def1,
    ] {
        assert_eq!(
            run(&roots, seed),
            fixture["union"]["expectedCss"].as_str().unwrap()
        );
    }

    // Static, variant, color, typography and border candidates together.
    let mut mixed = roots;
    for file in [
        include_str!("../../../../tests/utility/compiler/dynamic-variant-fixtures.json"),
        include_str!("../../../../tests/utility/compiler/colors-fixtures.json"),
        include_str!("../../../../tests/utility/compiler/typography-fixtures.json"),
        include_str!("../../../../tests/utility/compiler/borders-fixtures.json"),
    ] {
        mixed.extend(corpus(file));
    }
    mixed.sort();
    mixed.dedup();
    assert!(mixed.len() > 5_000, "corpus has {} candidates", mixed.len());
    let mut once = fresh();
    let prepared = once.prepare_add(&mixed).unwrap();
    let union = prepared.report().css.clone();
    for seed in [0x0dd_b1a5_e5ba_d5ee, 0x5bd1_e995_7f4a_7c15] {
        assert_eq!(run(&mixed, seed), union, "seed {seed}");
    }
}
