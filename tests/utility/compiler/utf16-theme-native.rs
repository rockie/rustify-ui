use super::{
    ast::{self, AstNode},
    public_api::{Source, SourceLocation},
    theme::{used_variables, Theme},
    value,
};
use crate::text::Text;
use serde_json::Value;
use std::{collections::BTreeSet, sync::Arc};

fn text(parts: &[&Text]) -> Text {
    let mut result = Text::default();
    for part in parts {
        result.push_text(part);
    }
    result
}

fn units(case: &Value, field: &str) -> Text {
    Text::from_units(serde_json::from_value(case[field].clone()).unwrap())
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap()
}

fn trim(units: &[u16]) -> &[u16] {
    fn whitespace(unit: u16) -> bool {
        matches!(unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279)
    }
    let start = units
        .iter()
        .position(|unit| !whitespace(*unit))
        .unwrap_or(units.len());
    let end = units
        .iter()
        .rposition(|unit| !whitespace(*unit))
        .map_or(start, |index| index + 1);
    &units[start..end]
}

fn block(options: &str, declarations: Vec<AstNode>) -> AstNode {
    AstNode::AtRule {
        origin: None,
        source: None,
        destination: None,
        name: "@theme".into(),
        params: options.into(),
        nodes: Some(declarations),
    }
}

// This fixture adapter reads only flat @theme declaration blocks in the frozen
// observation cases. It does not invoke or implement the production CSS parser.
fn authored_blocks(input: &Text) -> Vec<AstNode> {
    let marker = Text::from("@theme");
    let input = input.units();
    let mut offset = 0;
    let mut blocks = Vec::new();
    while let Some(start) = input[offset..]
        .windows(marker.units().len())
        .position(|part| part == marker.units())
    {
        let start = offset + start + marker.units().len();
        let open = input[start..].iter().position(|unit| *unit == 123).unwrap() + start;
        let close = input[open..].iter().position(|unit| *unit == 125).unwrap() + open;
        assert!(!input[open + 1..close].contains(&123));
        let options = Text::from_units(trim(&input[start..open]).to_vec())
            .to_utf8()
            .unwrap();
        let declarations = input[open + 1..close]
            .split(|unit| *unit == 59)
            .filter(|part| !trim(part).is_empty())
            .map(|part| {
                let colon = part.iter().position(|unit| *unit == 58).unwrap();
                AstNode::declaration(
                    Text::from_units(trim(&part[..colon]).to_vec()),
                    Text::from_units(trim(&part[colon + 1..]).to_vec()),
                )
            })
            .collect();
        blocks.push(block(&options, declarations));
        offset = close + 1;
    }
    blocks
}

fn extract(mut nodes: Vec<AstNode>) -> Theme {
    Theme::extract(&mut nodes, 32).unwrap()
}

fn samples() -> Vec<(String, Text)> {
    fixture()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|case| {
            let id = case["id"]
                .as_str()
                .unwrap()
                .strip_prefix("utf16:authored:value:")?;
            let input = units(case, "inputUtf16Units");
            let start = ".p { value: ".encode_utf16().count();
            let end = input.units()[start..]
                .iter()
                .position(|unit| *unit == 59)
                .unwrap()
                + start;
            Some((
                id.into(),
                Text::from_units(input.units()[start..end].to_vec()),
            ))
        })
        .collect()
}

// Frozen samples need only the identifier backslash escape; all other sample
// units are retained literally by the fixed upstream CSS escape algorithm.
fn authored_key(raw: &Text) -> Text {
    let mut units = Vec::new();
    for unit in raw.units() {
        if *unit == 92 {
            units.push(92);
        }
        units.push(*unit);
    }
    Text::from_units(units)
}

fn declaration(key: &Text, value: &Text) -> AstNode {
    AstNode::declaration(authored_key(key), value.clone())
}

fn ordered(theme: &Theme) -> Vec<(Text, Text)> {
    theme
        .ordered_tokens()
        .map(|(key, token)| (key.clone(), token.value.clone()))
        .collect()
}

#[test]
fn every_frozen_theme_observation_keeps_order_aliases_and_raw_units() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case.get("expectedThemeEntries").is_some())
    {
        count += 1;
        let theme = extract(authored_blocks(&units(case, "inputUtf16Units")));
        let actual: Vec<_> = theme
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
        assert_eq!(actual, expected, "{}", case["id"]);
        assert_eq!(theme.tokens.len(), expected.len(), "{}", case["id"]);
        let json = serde_json::to_string(&actual).unwrap();
        let decoded: Vec<(Text, Text)> = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, expected, "{}: typed observation JSON", case["id"]);
    }
    assert_eq!(count, 111);
}

#[test]
fn frozen_modes_resolve_raw_and_reference_fallbacks_without_changing_units() {
    let fixture = fixture();
    let mut count = 0;
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("utf16:theme:"))
    {
        count += 1;
        let mode = case["id"].as_str().unwrap().split(':').nth(2).unwrap();
        let theme = extract(authored_blocks(&units(case, "inputUtf16Units")));
        let entry = &case["expectedThemeEntries"][2];
        let observed_key = units(entry, "keyUtf16Units");
        let key = if mode.contains("prefix") {
            text(&[
                &Text::from("--"),
                &Text::from_units(observed_key.units()[5..].to_vec()),
            ])
        } else {
            observed_key
        };
        let raw = units(entry, "valueUtf16Units");
        let css = units(case, "expectedCssUtf16Units");
        let marker = Text::from("  direct: ");
        let start = css
            .units()
            .windows(marker.units().len())
            .position(|part| part == marker.units())
            .unwrap()
            + marker.units().len();
        let end = css.units()[start..]
            .iter()
            .position(|unit| *unit == 59)
            .unwrap()
            + start;
        let expected = Text::from_units(css.units()[start..end].to_vec());
        let mut used = BTreeSet::new();
        assert_eq!(
            theme.resolve_text(&key, &mut used).unwrap().units(),
            expected.units(),
            "{}",
            case["id"]
        );
        assert_eq!(
            theme.raw_text(&key, &mut BTreeSet::new()).unwrap().units(),
            raw.units()
        );
        let token = theme.tokens.get(&key).unwrap();
        assert_eq!(token.inline, mode == "inline");
        assert_eq!(token.reference, mode == "reference");
        assert_eq!(token.always_emit, mode == "static");
        assert_eq!(token.default, mode.starts_with("default"));
        if matches!(mode, "inline" | "reference") {
            assert!(used.is_empty());
        } else {
            let name = Text::from_units(expected.units()[4..expected.units().len() - 1].to_vec());
            assert_eq!(used, BTreeSet::from([name]));
        }
        if mode == "static" {
            let declarations = theme.declarations(&BTreeSet::new()).unwrap();
            let root = AstNode::Rule {
                origin: None,
                selector: ":root, :host".into(),
                nodes: declarations,
                source: None,
                destination: None,
            };
            let end = css.units().iter().position(|unit| *unit == 125).unwrap() + 2;
            assert_eq!(ast::to_css(&[root]).units(), &css.units()[..end]);
        }
    }
    assert_eq!(count, 63);
}

#[test]
fn theme_default_overwrite_reset_and_ignored_namespaces_share_one_text_map() {
    for (_, sample) in samples() {
        let namespace = text(&[&Text::from("--custom-"), &sample]);
        let a = text(&[&namespace, &Text::from("-a")]);
        let b = text(&[&namespace, &Text::from("-b")]);
        let outside = Text::from("--outside");
        let reset = text(&[&namespace, &Text::from("-*")]);
        let blocks = vec![
            block(
                "default",
                vec![
                    declaration(&a, &Text::from("7px")),
                    declaration(&b, &Text::from("11px")),
                ],
            ),
            block(
                "",
                vec![
                    declaration(&a, &Text::from("13px")),
                    declaration(&outside, &Text::from("31px")),
                ],
            ),
            block("default", vec![declaration(&a, &Text::from("17px"))]),
            block(
                "",
                vec![
                    declaration(&a, &Text::from("19px")),
                    declaration(&b, &Text::from("initial")),
                ],
            ),
            block("default", vec![declaration(&b, &Text::from("23px"))]),
        ];
        let theme = extract(blocks.clone());
        assert_eq!(
            ordered(&theme),
            vec![
                (a.clone(), Text::from("19px")),
                (outside.clone(), Text::from("31px")),
                (b.clone(), Text::from("23px"))
            ]
        );
        let mut reset_blocks = blocks;
        reset_blocks.push(block("", vec![declaration(&reset, &Text::from("initial"))]));
        reset_blocks.push(block("default", vec![declaration(&a, &Text::from("29px"))]));
        assert_eq!(
            ordered(&extract(reset_blocks.clone())),
            vec![(outside, Text::from("31px")), (a, Text::from("29px"))]
        );
        reset_blocks.push(block("", vec![AstNode::declaration("--*", "initial")]));
        assert!(extract(reset_blocks).tokens.is_empty());
        for (namespace, ignored) in [
            ("--font", "--font-weight"),
            ("--inset", "--inset-shadow"),
            ("--text", "--text-shadow"),
        ] {
            let removed = text(&[&Text::from(namespace), &Text::from("-"), &sample]);
            let kept = text(&[&Text::from(ignored), &Text::from("-"), &sample]);
            let theme = extract(vec![block(
                "",
                vec![
                    declaration(&removed, &Text::from("a")),
                    declaration(&kept, &Text::from("b")),
                    AstNode::declaration(format!("{namespace}-*"), "initial"),
                ],
            )]);
            assert_eq!(ordered(&theme), vec![(kept, Text::from("b"))]);
        }
    }
}

#[test]
fn theme_dependency_words_and_cycles_preserve_non_scalar_keys() {
    for (_, sample) in samples() {
        let a = text(&[&Text::from("--a-"), &sample]);
        let b = text(&[&Text::from("--b-"), &sample]);
        let c = text(&[&Text::from("--c-"), &sample]);
        let expression = text(&[
            &Text::from("outside(--ignored, var("),
            &a,
            &Text::from(", fn("),
            &b,
            &Text::from(", var("),
            &c,
            &Text::from("))), var("),
            &a,
            &Text::from("))"),
        ]);
        assert_eq!(
            used_variables(&value::parse_text(&expression, 32).unwrap()),
            vec![a.clone(), b.clone(), c.clone(), a.clone()]
        );
        let inline = extract(vec![block(
            "inline",
            vec![AstNode::declaration("--inline", expression.clone())],
        )]);
        let mut inline_used = BTreeSet::new();
        assert_eq!(
            inline
                .resolve_text(&Text::from("--inline"), &mut inline_used)
                .unwrap()
                .units(),
            expression.units()
        );
        assert_eq!(
            inline_used,
            BTreeSet::from([a.clone(), b.clone(), c.clone()])
        );
        let ca = authored_key(&a);
        let cb = authored_key(&b);
        let cc = authored_key(&c);
        let va = text(&[&Text::from("var("), &cb, &Text::from(")")]);
        let vb = text(&[&Text::from("var("), &ca, &Text::from(")")]);
        let vc = text(&[&Text::from("var("), &ca, &Text::from(")")]);
        let theme = extract(vec![block(
            "",
            vec![
                declaration(&a, &va),
                declaration(&b, &vb),
                declaration(&c, &vc),
                AstNode::declaration("--unused", "red"),
            ],
        )]);
        let emitted = theme.declarations(&BTreeSet::new()).unwrap();
        let expected = text(&[
            &ca,
            &Text::from(": "),
            &va,
            &Text::from(";\n"),
            &cb,
            &Text::from(": "),
            &vb,
            &Text::from(";\n"),
        ]);
        assert_eq!(ast::to_css(&emitted).units(), expected.units());
        let emitted = theme.declarations(&BTreeSet::from([cc.clone()])).unwrap();
        let expected = text(&[&expected, &cc, &Text::from(": "), &vc, &Text::from(";\n")]);
        assert_eq!(ast::to_css(&emitted).units(), expected.units());
    }
}

#[test]
fn prepared_theme_values_keep_raw_text_and_reference_output_separate() {
    for (_, sample) in samples() {
        let base = text(&[&Text::from("--base-"), &sample]);
        let alias = text(&[&Text::from("--alias-"), &sample]);
        let expression = text(&[&Text::from("--theme("), &base, &Text::from(")")]);
        let css_base = authored_key(&base);
        let expected = text(&[
            &Text::from("var("),
            &css_base,
            &Text::from(", "),
            &sample,
            &Text::from(")"),
        ]);
        let mut theme = extract(vec![
            block("reference", vec![declaration(&base, &sample)]),
            block("static", vec![declaration(&alias, &expression)]),
        ]);
        theme.prepare_functions(32, 65536, 8_000_000).unwrap();
        let token = theme.tokens.get(&alias).unwrap();
        assert_eq!(token.value.units(), expression.units());
        assert_eq!(token.output_value().units(), expected.units());
        assert_eq!(token.dependencies, vec![css_base.clone()]);
        let mut used = BTreeSet::new();
        assert_eq!(
            theme.raw_text(&alias, &mut used).unwrap().units(),
            expression.units()
        );
        assert_eq!(used, BTreeSet::from([css_base]));
        let emitted = theme.declarations(&BTreeSet::new()).unwrap();
        let expected_css = text(&[
            &authored_key(&alias),
            &Text::from(": "),
            &expected,
            &Text::from(";\n"),
        ]);
        assert_eq!(ast::to_css(&emitted).units(), expected_css.units());
        let owned = theme.owned_bytes();
        assert!(theme.prepare_functions(32, 65536, 1).is_err());
        assert_eq!(theme.owned_bytes(), owned);
        assert_eq!(
            theme.tokens.get(&alias).unwrap().output_value().units(),
            expected.units()
        );
    }
}

#[test]
fn theme_overwrites_keep_selected_source_metadata_through_emission() {
    let key = text(&[&Text::from("--custom-"), &Text::from_units(vec![0xd800])]);
    let old_source = Arc::new(Source {
        file: Some("old.css".into()),
        code: Text::from_units(vec![0xd800]),
    });
    let new_source = Arc::new(Source {
        file: Some("new.css".into()),
        code: Text::from_units(vec![0xdc00]),
    });
    let old = SourceLocation {
        public: None,
        source: old_source,
        start: 0,
        end: 1,
    };
    let selected = SourceLocation {
        public: None,
        source: new_source,
        start: 0,
        end: 1,
    };
    let mut first = AstNode::declaration("--custom-\\d800", "old");
    first.set_source(Some(old));
    let mut last = declaration(&key, &Text::from_units(vec![0xdc00]));
    last.set_source(Some(selected.clone()));
    let theme = extract(vec![
        block("static", vec![first, last]),
        block(
            "default",
            vec![AstNode::declaration("--custom-\\D800", "ignored")],
        ),
    ]);
    let token = theme.tokens.get(&key).unwrap();
    assert!(Arc::ptr_eq(
        &token.source.as_ref().unwrap().source,
        &selected.source
    ));
    let emitted = theme.declarations(&BTreeSet::new()).unwrap();
    assert_eq!(emitted.len(), 1);
    assert_eq!(emitted[0].source(), Some(&selected));
    assert!(Arc::ptr_eq(
        &emitted[0].source().unwrap().source,
        &selected.source
    ));
    let AstNode::Declaration {
        property, value, ..
    } = &emitted[0]
    else {
        panic!("expected canonical Declaration")
    };
    assert_eq!(property.units(), key.units());
    assert_eq!(value.units(), &[0xdc00]);
}

#[test]
fn theme_text_storage_and_prepared_cache_charge_real_utf16_capacity() {
    let key = Text::from_units(vec![45, 45, 0xd800]);
    let mut theme = extract(vec![block(
        "static",
        vec![declaration(&key, &Text::from("initial-value"))],
    )]);
    let mut storage = Vec::with_capacity(2048);
    storage.extend("--theme(--missing, ".encode_utf16());
    storage.extend("x".repeat(512).encode_utf16());
    storage.extend([0xdc00, 41]);
    let capacity_bytes = storage.capacity() * std::mem::size_of::<u16>();
    theme.tokens.get_mut(&key).unwrap().value = Text::from_units(storage);
    assert_eq!(
        theme.tokens.get(&key).unwrap().value.owned_bytes(),
        capacity_bytes
    );
    let before = theme.owned_bytes();
    assert!(before >= capacity_bytes);
    let expected = text(&[
        &Text::from("x".repeat(512)),
        &Text::from_units(vec![0xdc00]),
    ]);
    theme.prepare_functions(32, 65536, 8_000_000).unwrap();
    let token = theme.tokens.get(&key).unwrap();
    assert_eq!(token.value.owned_bytes(), capacity_bytes);
    assert_eq!(token.output_value().units(), expected.units());
    assert!(token.output_value().owned_bytes() > token.output_value().byte_len());
    let after = theme.owned_bytes();
    assert!(after >= before + token.output_value().owned_bytes());
    let json = serde_json::to_string(token.output_value()).unwrap();
    assert_eq!(
        serde_json::from_str::<Text>(&json).unwrap().units(),
        expected.units()
    );
    assert!(theme.prepare_functions(32, 65536, 1).is_err());
    assert_eq!(theme.owned_bytes(), after);
    assert_eq!(
        theme.tokens.get(&key).unwrap().output_value().units(),
        expected.units()
    );
}
