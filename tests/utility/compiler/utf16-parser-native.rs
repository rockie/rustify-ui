use super::{
    ast::{self, AstNode},
    parser,
    public_api::{self, CompileOptions, Features, LineTable, Position, Source, Stylesheet},
    theme::Theme,
    CompileError, Compiler, CompilerInput, Limits, SourceAsset,
};
use crate::text::Text;
use serde_json::Value;
use std::sync::Arc;

fn fixture() -> Value {
    serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap()
}

fn units(case: &Value, field: &str) -> Text {
    Text::from_units(serde_json::from_value(case[field].clone()).unwrap())
}

fn limits() -> Limits {
    Limits {
        input_bytes: 2_000_000,
        asset_bytes: 2_000_000,
        candidate_count: 1024,
        ast_depth: 32,
        css_bytes: 2_000_000,
        queued_bytes: 2_000_000,
        cache_bytes: 32_000_000,
    }
}

fn scalar_candidates(case: &Value) -> Option<Vec<String>> {
    case["candidatesUtf16Units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| {
            let units: Vec<u16> = serde_json::from_value(candidate.clone()).unwrap();
            String::from_utf16(&units).ok()
        })
        .collect()
}

fn without_banner(css: &Text) -> &[u16] {
    let banner: Vec<_> = "/*! tailwindcss v4.1.13 | MIT License | https://tailwindcss.com */\n"
        .encode_utf16()
        .collect();
    css.units()
        .strip_prefix(banner.as_slice())
        .unwrap_or(css.units())
}

fn assert_syntax_message(error: CompileError, expected: &Text, id: &str) {
    let CompileError::Syntax { message, .. } = error else {
        panic!("{id}: expected Syntax, received {error:?}");
    };
    assert_eq!(
        message.units(),
        expected.units(),
        "{id}: original error units"
    );
}

#[test]
fn production_parser_reads_all_92_authored_fixture_inputs_without_scalar_staging() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !id.starts_with("utf16:authored:") {
            continue;
        }
        let input = units(case, "inputUtf16Units");
        let nodes =
            parser::parse_text(&input, 32).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert!(
            matches!(nodes.last(), Some(AstNode::AtRule { name, params, .. })
            if name == "@tailwind" && params == "utilities"),
            "{id}: original directive parsed"
        );
        // The frozen compiler output consumes this directive. Only the observed
        // AST output excludes it; the production parser receives the whole input.
        assert_eq!(
            ast::to_css(&nodes[..nodes.len() - 1]).units(),
            units(case, "expectedCssUtf16Units").units(),
            "{id}: production parser output",
        );
        count += 1;
    }
    assert_eq!(count, 92);
}

#[test]
fn production_parser_errors_match_18_frozen_messages_with_original_surrogates() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if ![
            "utf16:error:missing-closing-rule:",
            "utf16:error:invalid-declaration:",
            "utf16:error:unterminated-string:",
        ]
        .iter()
        .any(|prefix| id.starts_with(prefix))
        {
            continue;
        }
        assert_eq!(case["expectedError"]["phase"], "compile");
        let error = parser::parse_text(&units(case, "inputUtf16Units"), 32).unwrap_err();
        assert_syntax_message(
            error,
            &units(&case["expectedError"], "messageUtf16Units"),
            id,
        );
        count += 1;
    }
    assert_eq!(count, 18);
}

#[test]
fn compile_text_matches_156_authored_map_and_error_cases_with_scalar_candidates() {
    let fixture = fixture();
    let mut counts = [0; 3];
    let mut css_count = 0;
    let mut error_count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let layer = if id.starts_with("utf16:authored:") {
            0
        } else if id.starts_with("utf16:map:") {
            1
        } else if id.starts_with("utf16:error:") {
            2
        } else {
            continue;
        };
        // The candidate API remains String. This declares a fixture subset,
        // rather than changing an original candidate or accepting a lossy decode.
        let Some(candidates) = scalar_candidates(case) else {
            continue;
        };
        let result = match public_api::compile_text(
            &units(case, "inputUtf16Units"),
            CompileOptions::default(),
            vec![],
            limits(),
        ) {
            Ok(mut compiler) => compiler
                .build(&candidates)
                .map_err(|error| ("build", error)),
            Err(error) => Err(("compile", error)),
        };
        if let Some(expected_error) = case.get("expectedError") {
            let (phase, error) = result.unwrap_err();
            assert_eq!(
                phase,
                expected_error["phase"].as_str().unwrap(),
                "{id}: phase"
            );
            assert_eq!(
                units(expected_error, "nameUtf16Units").units(),
                Text::from("Error").units()
            );
            assert_syntax_message(error, &units(expected_error, "messageUtf16Units"), id);
            error_count += 1;
        } else {
            let css = result.unwrap_or_else(|(phase, error)| panic!("{id}: {phase}: {error:?}"));
            assert_eq!(
                without_banner(&css),
                units(case, "expectedCssUtf16Units").units(),
                "{id}: CSS units"
            );
            css_count += 1;
        }
        counts[layer] += 1;
    }
    assert_eq!(counts, [92, 9, 55]);
    assert_eq!((css_count, error_count), (102, 54));
}

#[test]
fn production_parsed_map_blocks_match_all_10_frozen_raw_theme_observations() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !id.starts_with("utf16:map:") {
            continue;
        }
        let mut nodes = parser::parse_text(&units(case, "inputUtf16Units"), 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let observed: Vec<_> = theme
            .ordered_tokens()
            .map(|(key, token)| (theme.prefix_key_text(key), token.value.clone()))
            .collect();
        let expected: Vec<_> = case["expectedThemeEntries"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| {
                (
                    units(entry, "keyUtf16Units"),
                    units(entry, "valueUtf16Units"),
                )
            })
            .collect();
        assert_eq!(observed, expected, "{id}: raw theme order and key identity");
        count += 1;
    }
    assert_eq!(count, 10);
}

#[test]
fn raw_license_comment_parser_preserves_every_unit_and_full_source_span() {
    let mut payload = Text::from("! ");
    for unit in 0..=u16::MAX {
        payload.push_text(&Text::from_units(vec![unit, 122]));
    }
    let mut input = Text::from("/*");
    input.push_text(&payload);
    input.push_str("*/");
    let source = Arc::new(Source {
        file: Some("all-units.css".into()),
        code: input.clone(),
    });
    let nodes = parser::parse_with_source(&input, 32, Some(source.clone())).unwrap();
    let [AstNode::Comment { value, .. }] = nodes.as_slice() else {
        panic!("expected license comment");
    };
    assert!(
        value.units() == payload.units(),
        "all 65,536 original units, including lone surrogates"
    );
    for location in [nodes[0].source(), nodes[0].destination()] {
        let location = location.unwrap();
        assert_eq!((location.start, location.end), (0, input.units().len()));
        assert!(Arc::ptr_eq(&location.source, &source));
    }
    let mut expected = input;
    expected.push_str("\n");
    assert!(ast::to_css(&nodes).units() == expected.units());
}

#[test]
fn parser_preserves_custom_property_comments_but_removes_ordinary_comments() {
    let mut input = Text::from(".p { --custom: { ");
    input.push_text(&Text::from_units(vec![0xd800]));
    input.push_str("; nested: [");
    input.push_text(&Text::from_units(vec![0xdc00]));
    input.push_str("]; /* ");
    input.push_text(&Text::from_units(vec![0xd800]));
    input.push_str(" */ }; value: a/* ");
    input.push_text(&Text::from_units(vec![0xdc00]));
    input.push_str(" */b; }");
    let nodes = parser::parse_text(&input, 32).unwrap();
    let children = nodes[0].children().unwrap();
    let AstNode::Declaration {
        property, value, ..
    } = &children[0]
    else {
        panic!("custom property");
    };
    assert_eq!(property.units(), Text::from("--custom").units());
    let mut expected = Text::from("{ ");
    expected.push_text(&Text::from_units(vec![0xd800]));
    expected.push_str("; nested: [");
    expected.push_text(&Text::from_units(vec![0xdc00]));
    expected.push_str("]; /* ");
    expected.push_text(&Text::from_units(vec![0xd800]));
    expected.push_str(" */ }");
    assert_eq!(value.units(), expected.units());
    assert!(
        matches!(&children[1], AstNode::Declaration { property, value, .. } if property == "value" && value == "ab")
    );
    assert_eq!(children.len(), 2);
}

#[test]
fn bom_crlf_and_license_hoisting_keep_original_unit_offsets_and_source_identity() {
    let fixture = fixture();
    let case = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "utf16:authored:bom-crlf-high")
        .unwrap();
    let input = units(case, "inputUtf16Units");
    let source = Arc::new(Source {
        file: Some("input.css".into()),
        code: input.clone(),
    });
    let nodes = parser::parse_with_source(&input, 32, Some(source.clone())).unwrap();
    let declaration = &nodes[0].children().unwrap()[0];
    for (node, span) in [
        (&nodes[0], (1, 5)),
        (declaration, (10, 18)),
        (&nodes[1], (24, 43)),
    ] {
        for location in [node.source(), node.destination()] {
            let location = location.unwrap();
            assert_eq!((location.start, location.end), span);
            assert!(Arc::ptr_eq(&location.source, &source));
            assert_eq!(location.source.code.units(), input.units());
        }
    }
    let table = LineTable::new(&input);
    assert_eq!(table.find(10), Position { line: 3, column: 2 });
    assert_eq!(
        table.find(18),
        Position {
            line: 3,
            column: 10
        }
    );
    let mut hoisted = Text::from(".p { color: red; }\r\n/*! ");
    hoisted.push_text(&Text::from_units(vec![0xd800, 0xd83d, 0xde00]));
    hoisted.push_str(" */");
    let hoisted_source = Arc::new(Source {
        file: None,
        code: hoisted.clone(),
    });
    let nodes = parser::parse_with_source(&hoisted, 32, Some(hoisted_source.clone())).unwrap();
    assert!(
        matches!(&nodes[0], AstNode::Comment { value, .. } if value.units() == [33, 32, 0xd800, 0xd83d, 0xde00, 32])
    );
    assert_eq!(
        (
            nodes[0].source().unwrap().start,
            nodes[0].source().unwrap().end
        ),
        (20, 30)
    );
    assert!(Arc::ptr_eq(
        &nodes[0].source().unwrap().source,
        &hoisted_source
    ));
    assert_eq!(
        (
            nodes[1].source().unwrap().start,
            nodes[1].source().unwrap().end
        ),
        (0, 3)
    );
}

#[test]
fn parser_grammar_transforms_newlines_and_reports_unit_offsets_without_replacement() {
    let nodes = parser::parse_text(&Text::from(".p { value: a\r\nb; }"), 32).unwrap();
    assert!(
        matches!(&nodes[0].children().unwrap()[0], AstNode::Declaration { value, .. } if value == "a b")
    );
    let mut input = Text::from("@media (x: ");
    input.push_text(&Text::from_units(vec![0xd800, 0xdc00, 0xd800]));
    input.push_str(") { value: red;");
    let error = parser::parse_text(&input, 32).unwrap_err();
    let CompileError::Syntax { offset, message } = error else {
        panic!("expected missing closing at-rule");
    };
    assert_eq!(offset, input.units().len());
    let mut expected = Text::from("Missing closing } at @media (x: ");
    expected.push_text(&Text::from_units(vec![0xd800, 0xdc00, 0xd800]));
    expected.push_str(")");
    assert_eq!(message.units(), expected.units());
    let mut string = Text::from(".p { content: \"");
    string.push_text(&Text::from_units(vec![0xd83d, 0xde00, 0xd800]));
    string.push_str("\r\nend\"; }");
    let CompileError::Syntax { offset, message } = parser::parse_text(&string, 32).unwrap_err()
    else {
        panic!("expected unterminated string");
    };
    assert_eq!(offset, ".p { content: \"".len() + 3);
    let mut expected = Text::from("Unterminated string: \"");
    expected.push_text(&Text::from_units(vec![0xd83d, 0xde00, 0xd800]));
    expected.push_str("\"");
    assert_eq!(message.units(), expected.units());
}

#[test]
fn plain_compile_keeps_original_non_scalar_source_and_maps_without_normalizing_css() {
    let mut input = Text::from("\u{feff}.p {\r\n  value: ");
    input.push_text(&Text::from_units(vec![0xd800, 0xd83d, 0xde00, 0xdc00]));
    input.push_str("; /* ordinary */\r\n}\r\n");
    let mut compiler = public_api::compile_text(
        &input,
        CompileOptions {
            from: Some("plain.css".into()),
            ..CompileOptions::default()
        },
        vec![],
        limits(),
    )
    .unwrap();
    assert_eq!(compiler.features(), Features::NONE);
    assert_eq!(compiler.build(&[]).unwrap().units(), input.units());
    let map = compiler.build_source_map().unwrap();
    assert_eq!(map.sources.len(), 1);
    assert_eq!(map.sources[0].url.as_deref(), Some("plain.css"));
    assert_eq!(
        map.sources[0].content.as_ref().unwrap().units(),
        input.units()
    );
    assert!(!map.mappings.is_empty());
    for mapping in &map.mappings {
        let original = mapping.original_position.as_ref().unwrap();
        assert!(Arc::ptr_eq(&original.source, &map.sources[0]));
        assert_eq!(original.position, mapping.generated_position);
    }
}

#[test]
fn text_input_and_virtual_asset_content_reach_both_native_and_public_import_parsers() {
    let mut content = Text::from(".");
    content.push_text(&Text::from_units(vec![0xd800]));
    content.push_str(" { value: ");
    content.push_text(&Text::from_units(vec![0xdc00]));
    content.push_str("; }");
    let input = Text::from("@import \"opaque.css\";");
    let asset = SourceAsset {
        id: "opaque.css".into(),
        content: content.clone(),
    };
    let mut compiler = Compiler::new(
        CompilerInput {
            css: input.clone(),
            entry: "entry.css".into(),
        },
        vec![asset.clone()],
        limits(),
    )
    .unwrap();
    let prepared = compiler.prepare_add(&[]).unwrap();
    let native = compiler.commit(prepared).unwrap().css;
    let stylesheet: Stylesheet = asset.into();
    assert_eq!(stylesheet.content.units(), content.units());
    let mut public = public_api::compile_text(
        &input,
        CompileOptions::default(),
        vec![stylesheet],
        limits(),
    )
    .unwrap();
    let public = public.build(&[]).unwrap();
    let mut expected = Text::from(".");
    expected.push_text(&Text::from_units(vec![0xd800]));
    expected.push_str(" {\n  value: ");
    expected.push_text(&Text::from_units(vec![0xdc00]));
    expected.push_str(";\n}\n");
    assert_eq!(native.units(), expected.units());
    assert_eq!(without_banner(&public), expected.units());
}

#[test]
fn scalar_convenience_entries_agree_with_canonical_text_parser_and_compiler() {
    for input in [
        ".p { value: 😀; }",
        "@media screen { .p { color: red; } } @tailwind utilities;",
        "/*! scalar */\r\n.p { --custom: { a; b: [c]; }; }",
    ] {
        let text = Text::from(input);
        assert_eq!(
            parser::parse(input, 32).unwrap(),
            parser::parse_text(&text, 32).unwrap()
        );
        let mut scalar =
            public_api::compile(input, CompileOptions::default(), vec![], limits()).unwrap();
        let mut canonical =
            public_api::compile_text(&text, CompileOptions::default(), vec![], limits()).unwrap();
        assert_eq!(
            scalar.build(&[]).unwrap().units(),
            canonical.build(&[]).unwrap().units()
        );
    }
}

#[test]
fn canonical_parser_depth_and_text_input_byte_limits_are_enforced() {
    let mut input = Text::from(".p { .");
    input.push_text(&Text::from_units(vec![0xd800]));
    input.push_str(" { value: red; } }");
    assert!(
        matches!(parser::parse_text(&input, 1), Err(CompileError::Limit { resource, limit: 1 }) if resource == "ast_depth")
    );
    let mut limits = limits();
    limits.input_bytes = input.byte_len() - 1;
    assert!(
        matches!(public_api::compile_text(&input, CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
    );
}

#[test]
fn scalar_candidate_theme_index_results_keep_original_non_scalar_declarations() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !id.starts_with("utf16:candidate:primitive-index:") {
            continue;
        }
        let mut compiler = Compiler::new(
            CompilerInput {
                css: units(case, "inputUtf16Units"),
                entry: String::new(),
            },
            vec![],
            limits(),
        )
        .unwrap();
        let prepared = compiler
            .prepare_add(&scalar_candidates(case).unwrap())
            .unwrap();
        assert_eq!(
            prepared.report.css.units(),
            units(case, "expectedCssUtf16Units").units(),
            "{id}"
        );
        let non_scalar = units(case, "expectedCssUtf16Units").to_utf8().is_err();
        assert!(!prepared.report.gpu_programs.is_empty(), "{id}");
        for program in &prepared.report.gpu_programs {
            assert_eq!(program.conditions.iter().any(|condition| matches!(condition,
                super::program::GpuCondition::Unsupported { feature } if feature == "non-scalar UTF-16 GPU declaration")), non_scalar, "{id}");
        }
        count += 1;
    }
    assert_eq!(count, 9);
}

#[test]
fn theme_keyframe_usage_keeps_original_animation_name_units() {
    for units in [
        vec![0xd800],
        vec![0xdc00],
        vec![0xd800, 0xd800],
        vec![0xdc00, 0xdc00],
        vec![0xdc00, 0xd800],
        vec![0xd83d, 0xde00],
        vec![0xfffd],
        vec![99, 97, 114, 100],
    ] {
        let name = Text::from_units(units);
        for themed in [false, true] {
            let mut input = Text::from("@theme { ");
            let mut expected = Text::default();
            if themed {
                input.push_str("--animate-custom: ");
                input.push_text(&name);
                input.push_str(" 1s; ");
                expected.push_str(":root, :host {\n  --animate-custom: ");
                expected.push_text(&name);
                expected.push_str(" 1s;\n}\n");
            }
            input.push_str("@keyframes ");
            input.push_text(&name);
            input.push_str(" { to { opacity: 0; } } } .p { animation: ");
            expected.push_str(".p {\n  animation: ");
            if themed {
                input.push_str("var(--animate-custom)");
                expected.push_str("var(--animate-custom)");
            } else {
                input.push_text(&name);
                input.push_str(" 1s");
                expected.push_text(&name);
                expected.push_str(" 1s");
            }
            input.push_str("; } @tailwind utilities;");
            expected.push_str(";\n}\n@keyframes ");
            expected.push_text(&name);
            expected.push_str(" {\n  to {\n    opacity: 0;\n  }\n}\n");
            let mut compiler =
                public_api::compile_text(&input, CompileOptions::default(), vec![], limits())
                    .unwrap();
            assert_eq!(
                without_banner(&compiler.build(&[]).unwrap()),
                expected.units(),
                "name {name:?}, themed {themed}"
            );
        }
    }
}

#[test]
fn theme_keyframe_usage_splits_ecmascript_whitespace_and_distinct_names() {
    for (separator, retained) in [
        (0xfeff, true),
        (0x0085, false),
        (9, true),
        (32, true),
        (0x2028, true),
        (0x2007, true),
    ] {
        let mut input =
            Text::from("@theme { @keyframes card { to { opacity: 0; } } } .p { animation: card");
        input.push_text(&Text::from_units(vec![separator]));
        input.push_str("1s; } @tailwind utilities;");
        let mut compiler =
            public_api::compile_text(&input, CompileOptions::default(), vec![], limits()).unwrap();
        assert_eq!(
            compiler.build(&[]).unwrap().contains("@keyframes card"),
            retained,
            "separator {separator:04x}"
        );
    }
    let name = Text::from_units(vec![0xd800]);
    let mut input = Text::from("@theme { @keyframes ");
    input.push_text(&name);
    input
        .push_str(" { to { opacity: 0; } } @keyframes � { to { opacity: 1; } } } .p { animation: ");
    input.push_text(&name);
    input.push_str(" 1s, � 2s; } @tailwind utilities;");
    let mut compiler =
        public_api::compile_text(&input, CompileOptions::default(), vec![], limits()).unwrap();
    let css = compiler.build(&[]).unwrap();
    let mut keyframe = Text::from("@keyframes ");
    keyframe.push_text(&name);
    assert!(css
        .units()
        .windows(keyframe.units().len())
        .any(|part| part == keyframe.units()));
    assert!(css.contains("@keyframes �"));
}
