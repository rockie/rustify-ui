use super::{
    ast::{self, AstNode},
    public_api::{
        create_source_map, create_translation_map, LineTable, Position, Source, SourceLocation,
    },
};
use crate::text::Text;
use serde_json::Value;
use std::sync::Arc;

fn every_unit(label: &str) -> Text {
    let mut units: Vec<_> = label.encode_utf16().collect();
    for unit in 0..=u16::MAX {
        // Separate adjacent units so every surrogate is also tested unpaired.
        units.extend([unit, 97]);
    }
    Text::from_units(units)
}

fn decode_twice(json: &str) -> AstNode {
    let decoded: AstNode = serde_json::from_str(json).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    let second: AstNode = serde_json::from_str(&encoded).unwrap();
    assert!(decoded == second, "typed nested AST JSON roundtrip");
    second
}

fn rule(selector: impl Into<Text>, nodes: Vec<AstNode>) -> AstNode {
    AstNode::Rule {
        origin: None,
        selector: selector.into(),
        nodes,
        source: None,
        destination: None,
    }
}

fn at_rule(name: impl Into<Text>, params: impl Into<Text>, nodes: Option<Vec<AstNode>>) -> AstNode {
    AstNode::AtRule {
        origin: None,
        name: name.into(),
        params: params.into(),
        nodes,
        source: None,
        destination: None,
    }
}

fn comment(value: impl Into<Text>) -> AstNode {
    AstNode::Comment {
        origin: None,
        value: value.into(),
        source: None,
        destination: None,
    }
}

#[test]
fn selector_and_comment_decode_every_unit_inside_nested_tag_last_json() {
    let selector = every_unit("selector:");
    let comment = every_unit("comment:");
    let json = format!(
        r#"{{"nodes":[{{"nodes":[{{"value":{},"kind":"comment"}}],"selector":{},"kind":"rule"}}],"kind":"at_root"}}"#,
        serde_json::to_string(&comment).unwrap(),
        serde_json::to_string(&selector).unwrap(),
    );
    let decoded = decode_twice(&json);
    let AstNode::Rule {
        selector: actual,
        nodes,
        ..
    } = &decoded.children().unwrap()[0]
    else {
        panic!("expected nested rule");
    };
    assert!(actual.units() == selector.units(), "selector units");
    let AstNode::Comment { value, .. } = &nodes[0] else {
        panic!("expected nested comment");
    };
    assert!(value.units() == comment.units(), "comment units");
    assert!(nodes[0].source().is_none() && nodes[0].destination().is_none());
}

#[test]
fn at_rule_name_and_params_decode_every_unit_with_nested_and_leaf_forms() {
    let name = every_unit("@name:");
    let params = every_unit("params:");
    let name_json = serde_json::to_string(&name).unwrap();
    let params_json = serde_json::to_string(&params).unwrap();
    let json = format!(
        r#"{{"nodes":[{{"nodes":[{{"params":{params_json},"nodes":null,"name":{name_json},"kind":"at_rule"}}],"params":{params_json},"name":{name_json},"kind":"at_rule"}}],"kind":"at_root"}}"#,
    );
    let decoded = decode_twice(&json);
    let outer = &decoded.children().unwrap()[0];
    for node in [outer, &outer.children().unwrap()[0]] {
        let AstNode::AtRule {
            name: actual_name,
            params: actual_params,
            ..
        } = node
        else {
            panic!("expected at-rule");
        };
        assert!(actual_name.units() == name.units(), "at-rule name units");
        assert!(
            actual_params.units() == params.units(),
            "at-rule params units"
        );
        assert!(node.source().is_none() && node.destination().is_none());
    }
    assert!(outer.children().unwrap()[0].children().is_none());
}

#[test]
fn context_base_and_source_base_decode_every_unit_without_tag_order_dependency() {
    let base = every_unit("base:");
    let source_base = every_unit("source-base:");
    let json = format!(
        r#"{{"nodes":[{{"nodes":[{{"value":"child","kind":"comment"}}],"source_base":{},"reference":false,"base":{},"kind":"context"}}],"kind":"at_root"}}"#,
        serde_json::to_string(&source_base).unwrap(),
        serde_json::to_string(&base).unwrap(),
    );
    let decoded = decode_twice(&json);
    let AstNode::Context {
        base: actual_base,
        source_base: actual_source,
        reference,
        ..
    } = &decoded.children().unwrap()[0]
    else {
        panic!("expected context");
    };
    assert!(actual_base.units() == base.units(), "context base units");
    assert!(
        actual_source.as_ref().unwrap().units() == source_base.units(),
        "context source_base units"
    );
    assert!(!reference);
    let omitted: AstNode =
        serde_json::from_str(r#"{"nodes":[],"base":"base","reference":true,"kind":"context"}"#)
            .unwrap();
    assert_eq!(
        serde_json::to_string(&omitted).unwrap(),
        r#"{"kind":"context","base":"base","reference":true,"nodes":[]}"#,
    );
}

#[test]
fn scalar_ast_json_keeps_existing_field_names_and_omits_provenance() {
    let nodes = vec![
        rule(".p", vec![]),
        at_rule("@media", "screen", None),
        comment("license"),
        AstNode::Context {
            origin: None,
            base: "base".into(),
            source_base: Some("source".into()),
            reference: false,
            nodes: vec![],
        },
    ];
    let encoded: Value = serde_json::from_str(&serde_json::to_string(&nodes).unwrap()).unwrap();
    assert_eq!(
        encoded,
        serde_json::json!([
            {"kind":"rule","selector":".p","nodes":[]},
            {"kind":"at_rule","name":"@media","params":"screen","nodes":null},
            {"kind":"comment","value":"license"},
            {"kind":"context","base":"base","source_base":"source","reference":false,"nodes":[]},
        ])
    );
}

fn fixture_units(case: &Value, key: &str) -> Vec<u16> {
    serde_json::from_value(case[key].clone()).unwrap()
}

fn index_of(units: &[u16], delimiter: &str) -> usize {
    let delimiter: Vec<_> = delimiter.encode_utf16().collect();
    units
        .windows(delimiter.len())
        .position(|part| part == delimiter)
        .unwrap()
}

#[test]
fn direct_selector_comment_and_at_rule_emission_matches_27_frozen_css_observations() {
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    let mut counts = [0; 3];
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let input = fixture_units(case, "inputUtf16Units");
        let nodes = if id.starts_with("utf16:authored:selector:") {
            counts[0] += 1;
            vec![rule(
                Text::from_units(input[..index_of(&input, " {")].to_vec()),
                vec![AstNode::declaration("color", "red")],
            )]
        } else if id.starts_with("utf16:authored:license-comment:") {
            counts[1] += 1;
            vec![
                comment(Text::from_units(input[2..index_of(&input, "*/")].to_vec())),
                rule(".p", vec![AstNode::declaration("color", "red")]),
            ]
        } else if id.starts_with("utf16:authored:at-rule:") {
            counts[2] += 1;
            let start = index_of(&input, ".p { value: ") + ".p { value: ".len();
            let end = start + index_of(&input[start..], ";");
            vec![at_rule(
                "@media",
                Text::from_units(input["@media ".len()..index_of(&input, " {")].to_vec()),
                Some(vec![rule(
                    ".p",
                    vec![AstNode::declaration(
                        "value",
                        Text::from_units(input[start..end].to_vec()),
                    )],
                )]),
            )]
        } else {
            continue;
        };
        assert_eq!(
            ast::to_css(&nodes).units(),
            fixture_units(case, "expectedCssUtf16Units"),
            "{id}: direct AST emitter"
        );
    }
    assert_eq!(counts, [9, 9, 9]);
}

#[test]
fn opaque_at_rule_name_and_leaf_comment_emit_original_units() {
    let nodes = vec![
        at_rule(
            Text::from_units(vec![64, 0xd800]),
            Text::from_units(vec![0xdc00]),
            None,
        ),
        comment(Text::from_units(vec![
            0xdc00, 0xd800, 0xd83d, 0xde00, 0xfffd, 0xe000,
        ])),
    ];
    assert_eq!(
        ast::to_css(&nodes).units(),
        &[
            64, 0xd800, 32, 0xdc00, 59, 10, 47, 42, 0xdc00, 0xd800, 0xd83d, 0xde00, 0xfffd, 0xe000,
            42, 47, 10,
        ]
    );
}

#[test]
fn reference_context_pruning_preserves_visible_opaque_headers_and_sources() {
    let source = Arc::new(Source {
        file: Some("input.css".into()),
        code: Text::from_units(vec![0xd800, 0xdc00]),
    });
    let mut visible = at_rule(
        Text::from_units(vec![64, 0xd800]),
        Text::from_units(vec![0xdc00]),
        Some(vec![rule(
            Text::from_units(vec![38, 0xd800]),
            vec![
                AstNode::declaration(
                    Text::from_units(vec![45, 45, 116, 119, 45, 115, 111, 114, 116, 0xd800]),
                    "keep",
                ),
                comment(Text::from_units(vec![0xd800, 0xdc00])),
            ],
        )]),
    );
    visible.set_source(Some(SourceLocation {
        public: None,
        source: source.clone(),
        start: 0,
        end: 2,
    }));
    let expected = vec![visible.clone()];
    let mut nodes = vec![
        AstNode::Context {
            origin: None,
            base: Text::from_units(vec![0xd800]),
            source_base: Some(Text::from_units(vec![0xdc00])),
            reference: false,
            nodes: vec![visible],
        },
        AstNode::Context {
            origin: None,
            base: Text::from_units(vec![0xfffd]),
            source_base: None,
            reference: true,
            nodes: vec![rule(
                ".hidden",
                vec![AstNode::declaration("display", "none")],
            )],
        },
    ];
    assert_eq!(ast::to_css(&nodes).units(), ast::to_css(&expected).units());
    ast::optimize_rules(&mut nodes);
    assert_eq!(nodes, expected);
    assert!(Arc::ptr_eq(&nodes[0].source().unwrap().source, &source));
}

fn stamp(nodes: &mut [AstNode], location: &SourceLocation) {
    for node in nodes {
        node.set_source(Some(location.clone()));
        node.set_destination(Some(location.clone()));
        if let Some(children) = node.children_mut() {
            stamp(children, location);
        }
    }
}

fn locations<'a>(nodes: &'a [AstNode], output: &mut Vec<&'a SourceLocation>) {
    for node in nodes {
        output.extend([node.source(), node.destination()].into_iter().flatten());
        if let Some(children) = node.children() {
            locations(children, output);
        }
    }
}

#[test]
fn fresh_nested_clones_keep_text_and_spans_with_one_fresh_source_handle() {
    let source = Arc::new(Source {
        file: Some("input.css".into()),
        code: Text::from_units(vec![0xd800, 10, 0xdc00, 0xd83d, 0xde00]),
    });
    let location = SourceLocation {
        public: None,
        source: source.clone(),
        start: 2,
        end: 5,
    };
    let mut nodes = vec![rule(
        Text::from_units(vec![46, 0xd800]),
        vec![at_rule(
            Text::from_units(vec![64, 0xdc00]),
            Text::from_units(vec![0xd800]),
            Some(vec![
                comment(Text::from_units(vec![0xdc00])),
                AstNode::declaration("color", "red"),
            ]),
        )],
    )];
    stamp(&mut nodes, &location);
    let cloned = ast::clone_fresh(&nodes);
    let other = ast::clone_fresh(&nodes);
    assert_eq!(ast::to_css(&cloned).units(), ast::to_css(&nodes).units());
    assert_eq!(cloned, nodes);
    let mut cloned_locations = Vec::new();
    locations(&cloned, &mut cloned_locations);
    assert_eq!(cloned_locations.len(), 8);
    let fresh = cloned_locations[0].source.clone();
    assert!(!Arc::ptr_eq(&fresh, &source));
    assert!(!Arc::ptr_eq(&fresh, &other[0].source().unwrap().source));
    for cloned in cloned_locations {
        assert!(Arc::ptr_eq(&cloned.source, &fresh));
        assert_eq!((cloned.start, cloned.end), (2, 5));
        assert_eq!(cloned.source.code.units(), source.code.units());
    }
}

#[test]
fn remaining_fields_keep_utf16_capacity_and_source_storage_is_deduplicated_by_handle() {
    let mut expected = Vec::new();
    let fields: Vec<_> = (1..=6)
        .map(|index| {
            let mut units = Vec::with_capacity(index * 32);
            units.extend([0xd800, 0xdc00, 0xd800]);
            expected.push(units.capacity() * std::mem::size_of::<u16>());
            Text::from_units(units)
        })
        .collect();
    let mut fields = fields.into_iter();
    let tree = AstNode::Context {
        origin: None,
        base: fields.next().unwrap(),
        source_base: Some(fields.next().unwrap()),
        reference: false,
        nodes: vec![rule(
            fields.next().unwrap(),
            vec![at_rule(
                fields.next().unwrap(),
                fields.next().unwrap(),
                Some(vec![comment(fields.next().unwrap())]),
            )],
        )],
    };
    let AstNode::Context {
        base,
        source_base,
        nodes,
        ..
    } = &tree
    else {
        unreachable!()
    };
    let AstNode::Rule {
        selector, nodes, ..
    } = &nodes[0]
    else {
        unreachable!()
    };
    let AstNode::AtRule {
        name,
        params,
        nodes: Some(nodes),
        ..
    } = &nodes[0]
    else {
        unreachable!()
    };
    let AstNode::Comment { value, .. } = &nodes[0] else {
        unreachable!()
    };
    assert_eq!(
        [
            base,
            source_base.as_ref().unwrap(),
            selector,
            name,
            params,
            value
        ]
        .map(Text::owned_bytes)
        .as_slice(),
        expected
    );
    let mut code = Vec::with_capacity(512);
    code.extend([0xd800, 10, 0xdc00]);
    let source = Arc::new(Source {
        file: None,
        code: Text::from_units(code),
    });
    let expected = std::mem::size_of::<Source>()
        + 2 * std::mem::size_of::<usize>()
        + source.code.owned_bytes();
    let mut tree = tree;
    stamp(
        std::slice::from_mut(&mut tree),
        &SourceLocation {
            public: None,
            source,
            start: 0,
            end: 3,
        },
    );
    assert_eq!(ast::source_owned_bytes(&[tree.clone(), tree]), expected);
}

#[test]
fn public_source_maps_count_units_and_keep_equal_text_origins_distinct() {
    let source = Arc::new(Source {
        file: Some("input.css".into()),
        code: Text::from_units(vec![0xd800, 97, 13, 10, 0xdc00, 0xd83d, 0xde00, 90, 10]),
    });
    let equal_source = Arc::new((*source).clone());
    let generated = Arc::new(Source {
        file: None,
        code: ast::to_css(&[rule(
            Text::from_units(vec![46, 0xdc00, 0xd83d, 0xde00]),
            vec![AstNode::declaration("color", "red")],
        )]),
    });
    let origin = SourceLocation {
        public: None,
        source: source.clone(),
        start: 4,
        end: 8,
    };
    let equal_origin = SourceLocation {
        public: None,
        source: equal_source,
        start: 4,
        end: 8,
    };
    let destination = SourceLocation {
        public: None,
        source: generated.clone(),
        start: 0,
        end: 5,
    };
    let table = LineTable::new(&source.code);
    assert_eq!(table.find(6), Position { line: 2, column: 2 });
    assert_eq!(table.find(7), Position { line: 2, column: 3 });
    let map = create_source_map([
        (Some(&origin), Some(&destination)),
        (Some(&origin), Some(&destination)),
        (Some(&equal_origin), Some(&destination)),
    ]);
    assert_eq!((map.sources.len(), map.mappings.len()), (3, 6));
    assert_eq!(
        map.sources[0].content.as_ref().unwrap().units(),
        source.code.units()
    );
    assert_eq!(
        map.sources[1].content.as_ref().unwrap().units(),
        generated.code.units()
    );
    assert_eq!(map.sources[0], map.sources[2]);
    assert!(!Arc::ptr_eq(&map.sources[0], &map.sources[2]));
    let mut origin_counts = [0; 2];
    for mapping in &map.mappings {
        let original = mapping.original_position.as_ref().unwrap();
        let index = usize::from(Arc::ptr_eq(&original.source, &map.sources[2]));
        assert!(Arc::ptr_eq(
            &original.source,
            &map.sources[if index == 0 { 0 } else { 2 }]
        ));
        origin_counts[index] += 1;
        assert_eq!(original.position.line, 2);
        assert_eq!(
            (original.position.column, mapping.generated_position),
            if original.position.column == 0 {
                (0, Position { line: 1, column: 0 })
            } else {
                (4, Position { line: 1, column: 5 })
            }
        );
    }
    assert_eq!(origin_counts, [4, 2]);
    let translate = create_translation_map(&source.code, &generated.code);
    assert_eq!(
        translate(Some(&origin), Some(&destination)),
        vec![(
            Position { line: 2, column: 0 },
            Position { line: 2, column: 4 },
            Some(Position { line: 1, column: 0 }),
            Some(Position { line: 1, column: 5 }),
        )]
    );
    assert_eq!(
        translate(Some(&origin), None),
        vec![(
            Position { line: 2, column: 0 },
            Position { line: 2, column: 4 },
            None,
            None,
        )]
    );
    assert!(translate(None, Some(&destination)).is_empty());
}

fn append_text(parts: &[&Text]) -> Text {
    let mut result = Text::default();
    for part in parts {
        result.push_text(part);
    }
    result
}

fn assert_same_locations(actual: &[AstNode], original: &[AstNode]) {
    let mut actual_locations = Vec::new();
    let mut original_locations = Vec::new();
    locations(actual, &mut actual_locations);
    locations(original, &mut original_locations);
    assert_eq!(actual_locations.len(), original_locations.len());
    assert!(!original_locations.is_empty());
    for (actual, original) in actual_locations.into_iter().zip(original_locations) {
        assert_eq!((actual.start, actual.end), (original.start, original.end));
        assert!(Arc::ptr_eq(&actual.source, &original.source));
    }
}

#[test]
fn at_rule_function_params_keep_oracle_spacing_units_children_and_provenance() {
    let fixture: Value =
        serde_json::from_str(include_str!("utf16-function-fixtures.json")).unwrap();
    let mut observations = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        if case["id"].as_str().unwrap().starts_with("spacing:") {
            observations.push((
                case["id"].as_str().unwrap().to_string(),
                Text::from_units(fixture_units(case, "valueUtf16Units")),
                Text::from_units(fixture_units(case, "expectedValueUtf16Units")),
            ));
        }
    }
    assert_eq!(observations.len(), 5);
    observations.push((
        "spacing:literal-backslash".into(),
        Text::from(r"--spacing(\ud800)"),
        Text::from(r"calc(var(--spacing, 4px) * \ud800)"),
    ));
    let mut theme_nodes = vec![at_rule(
        "@theme",
        "reference",
        Some(vec![AstNode::declaration("--spacing", "4px")]),
    )];
    let theme = super::theme::Theme::extract(&mut theme_nodes, 32).unwrap();
    let mut count = 0;
    for (id, expression, expected_expression) in observations {
        for name in ["@media", "@custom-media", "@container", "@supports"] {
            // Upstream spacing ignores its source kind, so its fixed value oracle
            // applies here. --theme deliberately tests the distinct at-rule inline path.
            let params = append_text(&[
                &Text::from("(width > "),
                &expression,
                &Text::from(") and (width: --theme(--spacing))"),
            ]);
            let expected_params = append_text(&[
                &Text::from("(width > "),
                &expected_expression,
                &Text::from(") and (width: 4px)"),
            ]);
            let mut original = at_rule(
                name,
                params,
                Some(vec![rule(
                    Text::from_units(vec![46, 0xd800]),
                    vec![
                        comment(Text::from_units(vec![0xdc00])),
                        AstNode::declaration("color", "red"),
                    ],
                )]),
            );
            let source = Arc::new(Source {
                file: Some("authored.css".into()),
                code: ast::to_css(std::slice::from_ref(&original)),
            });
            let source_location = SourceLocation {
                public: None,
                start: 0,
                end: source.code.units().len(),
                source,
            };
            stamp(std::slice::from_mut(&mut original), &source_location);
            let destination = Arc::new(Source {
                file: Some("destination.css".into()),
                code: expected_params.clone(),
            });
            original.set_destination(Some(SourceLocation {
                public: None,
                start: 1,
                end: destination.code.units().len(),
                source: destination,
            }));
            let mut nodes = vec![original.clone()];
            super::functions::substitute(&mut nodes, &theme, 32, 262144).unwrap();
            let AstNode::AtRule {
                name: actual_name,
                params: actual_params,
                ..
            } = &nodes[0]
            else {
                panic!("expected at-rule");
            };
            assert_eq!(actual_name.units(), Text::from(name).units(), "{id}: name");
            assert_eq!(
                actual_params.units(),
                expected_params.units(),
                "{id}: {name} params"
            );
            assert_eq!(nodes[0].children(), original.children(), "{id}: children");
            assert_same_locations(&nodes, std::slice::from_ref(&original));
            count += 1;
        }
    }
    assert_eq!(count, 24);
}

#[test]
fn stylesheet_scope_maps_text_globals_and_animation_without_changing_opaque_fields_or_sources() {
    let samples = [
        Text::from_units(vec![0xd800]),
        Text::from_units(vec![0xdc00]),
        Text::from_units(vec![0xd83d, 0xde00]),
        Text::from_units(vec![0xfffd]),
        Text::from_units(vec![0xe000]),
        Text::from(r"\ud800"),
    ];
    let mut original = Vec::new();
    let mut expected_local = Text::from(
        "@scope ([data-rustify-style-owner=\"alpha\"]) to ([data-rustify-style-owner]) {\n",
    );
    let mut expected_globals = Text::default();
    for sample in &samples {
        let keyframe = append_text(&[&Text::from("spin-"), sample]);
        let property = append_text(&[&Text::from("--tw-token-"), sample]);
        let selector = append_text(&[&Text::from(".opaque-"), sample]);
        let mut group = vec![
            at_rule(
                "@keyframes",
                keyframe.clone(),
                Some(vec![rule("to", vec![AstNode::declaration("opacity", "1")])]),
            ),
            at_rule(
                "@property",
                property.clone(),
                Some(vec![
                    AstNode::declaration("syntax", "\"*\""),
                    AstNode::declaration("inherits", "false"),
                ]),
            ),
            rule(
                selector,
                vec![
                    AstNode::declaration(
                        "animation",
                        append_text(&[&keyframe, &Text::from(" 1s linear")]),
                    ),
                    AstNode::declaration(
                        "animation-name",
                        append_text(&[&keyframe, &Text::from(", none")]),
                    ),
                    AstNode::declaration(
                        "content",
                        append_text(&[&Text::from("\""), &keyframe, &Text::from("\"")]),
                    ),
                    AstNode::declaration(
                        "width",
                        append_text(&[&Text::from("var("), &property, &Text::from(")")]),
                    ),
                ],
            ),
            comment(append_text(&[&Text::from("! "), sample])),
        ];
        let source = Arc::new(Source {
            file: Some("authored.css".into()),
            code: ast::to_css(&group),
        });
        let source_location = SourceLocation {
            public: None,
            start: 0,
            end: source.code.units().len(),
            source,
        };
        stamp(&mut group, &source_location);
        original.extend(group);
        // Expected CSS is specified directly from the ownership contract: local
        // content stays in @scope, while renamed keyframes/property remain global.
        expected_local.push_str("  .opaque-");
        expected_local.push_text(sample);
        expected_local.push_str(" {\n    animation: rustify-alpha-spin-");
        expected_local.push_text(sample);
        expected_local.push_str(" 1s linear;\n    animation-name: rustify-alpha-spin-");
        expected_local.push_text(sample);
        expected_local.push_str(", none;\n    content: \"spin-");
        expected_local.push_text(sample);
        expected_local.push_str("\";\n    width: var(--rustify-alpha-tw-token-");
        expected_local.push_text(sample);
        expected_local.push_str(");\n  }\n  /*! ");
        expected_local.push_text(sample);
        expected_local.push_str("*/\n");
        expected_globals.push_str("@keyframes rustify-alpha-spin-");
        expected_globals.push_text(sample);
        expected_globals
            .push_str(" {\n  to {\n    opacity: 1;\n  }\n}\n@property --rustify-alpha-tw-token-");
        expected_globals.push_text(sample);
        expected_globals.push_str(" {\n  syntax: \"*\";\n  inherits: false;\n}\n");
    }
    expected_local.push_str("}\n");
    expected_local.push_text(&expected_globals);
    let original_css = ast::to_css(&original);
    let scoped = crate::scope::stylesheet(&original, "alpha", 32).unwrap();
    assert_eq!(ast::to_css(&scoped).units(), expected_local.units());
    assert_eq!(ast::to_css(&original).units(), original_css.units());
    assert_eq!(scoped.len(), 13);
    assert!(scoped[0].source().is_none() && scoped[0].destination().is_none());
    let local = scoped[0].children().unwrap();
    assert_eq!(local.len(), 12);
    for index in 0..samples.len() {
        for (actual, expected) in [
            (&local[index * 2], &original[index * 4 + 2]),
            (&local[index * 2 + 1], &original[index * 4 + 3]),
            (&scoped[1 + index * 2], &original[index * 4]),
            (&scoped[2 + index * 2], &original[index * 4 + 1]),
        ] {
            assert_same_locations(std::slice::from_ref(actual), std::slice::from_ref(expected));
        }
    }
}
