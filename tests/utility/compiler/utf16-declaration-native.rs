use super::{
    ast::{self, AstNode},
    functions,
    public_api::{Source, SourceLocation},
    Compiler, CompilerInput, Limits,
};
use crate::text::Text;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

fn text(parts: &[&Text]) -> Text {
    let mut output = Text::default();
    for part in parts {
        output.push_text(part);
    }
    output
}

fn fields(node: &AstNode) -> (&Text, &Text, bool, &Arc<()>) {
    match node {
        AstNode::Declaration {
            property,
            value,
            important,
            identity,
            ..
        } => (property, value, *important, identity),
        _ => panic!("expected canonical Declaration"),
    }
}

fn rule(nodes: Vec<AstNode>) -> AstNode {
    AstNode::Rule {
        origin: None,
        selector: ".p".into(),
        nodes,
        source: None,
        destination: None,
    }
}

fn units(case: &Value, field: &str) -> Text {
    Text::from_units(serde_json::from_value(case[field].clone()).unwrap())
}

fn samples() -> Vec<(String, Text)> {
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|case| {
            case["id"]
                .as_str()
                .unwrap()
                .strip_prefix("utf16:authored:value:")
                .map(|id| {
                    let input = units(case, "inputUtf16Units");
                    let start = ".p { value: ".encode_utf16().count();
                    let end = input.units()[start..]
                        .iter()
                        .position(|unit| *unit == 59)
                        .unwrap()
                        + start;
                    (
                        id.into(),
                        Text::from_units(input.units()[start..end].to_vec()),
                    )
                })
        })
        .collect()
}

fn sourced(mut node: AstNode) -> AstNode {
    let (property, value, _, _) = fields(&node);
    let code = text(&[
        &Text::from(".p { "),
        property,
        &Text::from(": "),
        value,
        &Text::from("; }"),
    ]);
    let start = ".p { ".encode_utf16().count();
    let end = start + property.units().len() + 2 + value.units().len();
    let source = Arc::new(Source {
        file: Some("input.css".into()),
        code,
    });
    let destination = SourceLocation {
        public: None,
        source: source.clone(),
        start: 0,
        end: source.code.units().len(),
    };
    node.set_source(Some(SourceLocation {
        public: None,
        source,
        start,
        end,
    }));
    node.set_destination(Some(destination));
    node
}

fn assert_provenance(actual: &AstNode, original: &AstNode) {
    for (actual, original) in [
        (actual.source(), original.source()),
        (actual.destination(), original.destination()),
    ] {
        let actual = actual.unwrap();
        let original = original.unwrap();
        assert_eq!((actual.start, actual.end), (original.start, original.end));
        assert!(Arc::ptr_eq(&actual.source, &original.source));
    }
    assert!(Arc::ptr_eq(fields(actual).3, fields(original).3));
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeclarationJson {
    kind: String,
    property: Text,
    value: Text,
    important: bool,
}

fn json_roundtrip(node: &AstNode, property: &Text, value: &Text, important: bool) {
    let json = serde_json::to_string(node).unwrap();
    let observed: DeclarationJson = serde_json::from_str(&json).unwrap();
    assert_eq!(observed.kind, "declaration");
    assert!(observed.property.units() == property.units());
    assert!(observed.value.units() == value.units());
    assert_eq!(observed.important, important);
    let decoded: AstNode = serde_json::from_str(&json).unwrap();
    let decoded_fields = fields(&decoded);
    assert!(decoded_fields.0.units() == property.units());
    assert!(decoded_fields.1.units() == value.units());
    assert_eq!(decoded_fields.2, important);
    // Existing wire shape omits provenance and identity; JSON reconstruction
    // creates a distinct declaration allocation rather than an Arc graph codec.
    assert!(decoded.source().is_none());
    assert!(decoded.destination().is_none());
    assert!(!Arc::ptr_eq(decoded_fields.3, fields(node).3));
}

#[test]
fn canonical_declaration_json_preserves_scalar_schema_and_every_utf16_unit() {
    let scalar = AstNode::declaration("color", "red");
    assert_eq!(
        serde_json::to_string(&scalar).unwrap(),
        "{\"kind\":\"declaration\",\"property\":\"color\",\"value\":\"red\",\"important\":false}"
    );
    let samples = samples();
    assert_eq!(samples.len(), 9);
    for (_, sample) in samples {
        for important in [false, true] {
            let property = text(&[&Text::from("custom-"), &sample]);
            let value = text(&[&Text::from("outer("), &sample, &Text::from(")")]);
            let mut node = sourced(AstNode::declaration(property.clone(), value.clone()));
            if let AstNode::Declaration {
                important: flag, ..
            } = &mut node
            {
                *flag = important;
            }
            json_roundtrip(&node, &property, &value, important);
        }
    }
    let all = Text::from_units((0..=u16::MAX).collect());
    json_roundtrip(
        &AstNode::declaration(all.clone(), all.clone()),
        &all,
        &all,
        false,
    );
    let json = "{\"value\":\"\\udc00\",\"important\":true,\"property\":\"\\ud800\",\"kind\":\"declaration\"}";
    let node: AstNode = serde_json::from_str(json).unwrap();
    assert_eq!(fields(&node).0.units(), &[0xd800]);
    assert_eq!(fields(&node).1.units(), &[0xdc00]);
    assert!(fields(&node).2);
}

#[test]
fn canonical_declaration_emitter_matches_frozen_property_and_value_units() {
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    let mut count = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let property_case = id.starts_with("utf16:authored:property:");
        if !property_case && !id.starts_with("utf16:authored:value:") {
            continue;
        }
        count += 1;
        let input = units(case, "inputUtf16Units");
        let start = ".p { ".encode_utf16().count();
        let colon = input.units()[start..]
            .iter()
            .position(|unit| *unit == 58)
            .unwrap()
            + start;
        let end = input.units()[colon..]
            .iter()
            .position(|unit| *unit == 59)
            .unwrap()
            + colon;
        let property = Text::from_units(input.units()[start..colon].to_vec());
        let value = Text::from_units(input.units()[colon + 2..end].to_vec());
        let node = AstNode::declaration(property, value);
        assert_eq!(
            ast::to_css(&[rule(vec![node])]).units(),
            units(case, "expectedCssUtf16Units").units(),
            "{id}"
        );
    }
    assert_eq!(count, 18);
    for (_, sample) in samples() {
        let property = text(&[&Text::from("custom-"), &sample]);
        let mut node = AstNode::declaration(property.clone(), sample.clone());
        if let AstNode::Declaration { important, .. } = &mut node {
            *important = true;
        }
        let expected = text(&[
            &property,
            &Text::from(": "),
            &sample,
            &Text::from(" !important;\n"),
        ]);
        assert_eq!(ast::to_css(&[node]).units(), expected.units());
    }
}

#[test]
fn canonical_declaration_clones_preserve_provenance_and_shared_identity_graphs() {
    for (_, sample) in samples() {
        let node = sourced(AstNode::declaration(
            text(&[&Text::from("custom-"), &sample]),
            sample,
        ));
        let cloned = node.clone();
        assert_provenance(&cloned, &node);
        let original = vec![node.clone(), rule(vec![node.clone()])];
        let fresh = ast::clone_fresh(&original);
        let other = ast::clone_fresh(&original);
        let nested = &fresh[1].children().unwrap()[0];
        assert_eq!(fields(&fresh[0]).0.units(), fields(&node).0.units());
        assert_eq!(fields(&fresh[0]).1.units(), fields(&node).1.units());
        assert!(Arc::ptr_eq(fields(&fresh[0]).3, fields(nested).3));
        assert!(!Arc::ptr_eq(fields(&fresh[0]).3, fields(&node).3));
        assert!(!Arc::ptr_eq(fields(&fresh[0]).3, fields(&other[0]).3));
        let source = fresh[0].source().unwrap();
        assert_eq!(source, node.source().unwrap());
        assert_eq!(fresh[0].destination(), node.destination());
        assert!(Arc::ptr_eq(
            &source.source,
            &nested.source().unwrap().source
        ));
        assert!(Arc::ptr_eq(
            &source.source,
            &fresh[0].destination().unwrap().source
        ));
        assert!(!Arc::ptr_eq(&source.source, &node.source().unwrap().source));
        assert!(!Arc::ptr_eq(
            &source.source,
            &other[0].source().unwrap().source
        ));
    }
}

#[test]
fn canonical_declaration_deduplication_keeps_last_equal_units_and_distinct_units() {
    for (_, sample) in samples() {
        let property = text(&[&Text::from("custom-"), &sample]);
        let first = AstNode::declaration(property.clone(), sample.clone());
        let last = sourced(AstNode::declaration(property.clone(), sample.clone()));
        let mut important = AstNode::declaration(property, sample);
        if let AstNode::Declaration {
            important: flag, ..
        } = &mut important
        {
            *flag = true;
        }
        let last_identity = fields(&last).3.clone();
        let important_identity = fields(&important).3.clone();
        assert!(!Arc::ptr_eq(fields(&first).3, &last_identity));
        let mut nodes = vec![
            first,
            rule(vec![AstNode::declaration("color", "red")]),
            last.clone(),
            important,
        ];
        ast::optimize_rule_children(&mut nodes);
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[0], AstNode::Rule { .. }));
        assert!(Arc::ptr_eq(fields(&nodes[1]).3, &last_identity));
        assert!(Arc::ptr_eq(fields(&nodes[2]).3, &important_identity));
        assert_provenance(&nodes[1], &last);
    }
    let samples = samples();
    let mut nodes: Vec<_> = samples
        .iter()
        .map(|(_, sample)| AstNode::declaration("--x", sample.clone()))
        .collect();
    let last = AstNode::declaration("--x", samples[0].1.clone());
    let identity = fields(&last).3.clone();
    nodes.push(last);
    ast::optimize_rule_children(&mut nodes);
    assert_eq!(nodes.len(), 9);
    for (node, (_, expected)) in nodes
        .iter()
        .zip(samples.iter().skip(1).chain(samples.iter().take(1)))
    {
        assert_eq!(fields(node).1.units(), expected.units());
    }
    assert!(Arc::ptr_eq(fields(nodes.last().unwrap()).3, &identity));
}

#[test]
fn canonical_declaration_retains_text_capacity_and_charges_source_allocations() {
    let mut property = Vec::with_capacity(64);
    property.extend([45, 45, 0xd800]);
    let mut value = Vec::with_capacity(128);
    value.extend([0xd83d, 0xde00]);
    let property_bytes = property.capacity() * std::mem::size_of::<u16>();
    let value_bytes = value.capacity() * std::mem::size_of::<u16>();
    let node = AstNode::declaration(Text::from_units(property), Text::from_units(value));
    assert_eq!(
        ast::declaration_identity_bytes(std::slice::from_ref(&node)),
        2 * std::mem::size_of::<usize>()
    );
    assert_eq!(fields(&node).0.owned_bytes(), property_bytes);
    assert_eq!(fields(&node).1.owned_bytes(), value_bytes);
    assert!(property_bytes > fields(&node).0.byte_len());
    assert!(value_bytes > fields(&node).1.byte_len());
    let mut code = Vec::with_capacity(256);
    code.extend([0xd800, 0xdc00]);
    let mut file = String::with_capacity(128);
    file.push_str("same.css");
    let source = Arc::new(Source {
        code: Text::from_units(code),
        file: Some(file),
    });
    let expected = std::mem::size_of::<Source>()
        + 2 * std::mem::size_of::<usize>()
        + source.code.owned_bytes()
        + source.file.as_ref().unwrap().capacity();
    let mut first = node;
    first.set_source(Some(SourceLocation {
        public: None,
        source: source.clone(),
        start: 0,
        end: 2,
    }));
    first.set_destination(first.source().cloned());
    assert_eq!(
        ast::source_owned_bytes(&[first.clone(), first.clone()]),
        expected
    );
    let mut other_code = Vec::with_capacity(256);
    other_code.extend([0xd800, 0xdc00]);
    let mut other_file = String::with_capacity(128);
    other_file.push_str("same.css");
    let other_source = Arc::new(Source {
        code: Text::from_units(other_code),
        file: Some(other_file),
    });
    let other_expected = std::mem::size_of::<Source>()
        + 2 * std::mem::size_of::<usize>()
        + other_source.code.owned_bytes()
        + other_source.file.as_ref().unwrap().capacity();
    let mut second = first.clone();
    second.set_source(Some(SourceLocation {
        public: None,
        source: other_source.clone(),
        start: 0,
        end: 2,
    }));
    second.set_destination(Some(SourceLocation {
        public: None,
        source: other_source,
        start: 0,
        end: 2,
    }));
    assert_eq!(
        ast::source_owned_bytes(&[first, second]),
        expected + other_expected
    );
}

#[test]
fn canonical_declaration_functions_match_all_frozen_outputs_without_losing_metadata() {
    let fixture: Value =
        serde_json::from_str(include_str!("utf16-function-fixtures.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 75);
    for case in cases {
        let compiler = Compiler::new(
            CompilerInput {
                css: case["themeCss"].as_str().unwrap().into(),
                entry: String::new(),
            },
            vec![],
            Limits {
                input_bytes: 65536,
                asset_bytes: 65536,
                candidate_count: 1024,
                ast_depth: 32,
                css_bytes: 262144,
                queued_bytes: 65536,
                cache_bytes: 8_000_000,
            },
        )
        .unwrap();
        let original = sourced(AstNode::declaration(
            "value",
            units(case, "valueUtf16Units"),
        ));
        let mut nodes = vec![original.clone()];
        functions::substitute(&mut nodes, &compiler.theme, 32, 262144).unwrap();
        assert_eq!(fields(&nodes[0]).0.units(), Text::from("value").units());
        assert_eq!(
            fields(&nodes[0]).1.units(),
            units(case, "expectedValueUtf16Units").units(),
            "{}",
            case["id"]
        );
        assert_provenance(&nodes[0], &original);
        json_roundtrip(
            &nodes[0],
            &Text::from("value"),
            &units(case, "expectedValueUtf16Units"),
            false,
        );
    }
}

#[test]
fn canonical_declaration_scope_keeps_raw_units_and_source_identity() {
    for (id, sample) in samples() {
        let property = text(&[&Text::from("--tw-"), &sample]);
        let value = text(&[
            &Text::from("outer(var(--tw-"),
            &sample,
            &Text::from("), \""),
            &sample,
            &Text::from("\", "),
            &sample,
            &Text::from(")"),
        ]);
        let original = sourced(AstNode::declaration(property.clone(), value.clone()));
        let scoped =
            crate::scope::stylesheet(std::slice::from_ref(&original), "alpha", 32).unwrap();
        let declaration = &scoped[0].children().unwrap()[0];
        let expected_property = text(&[&Text::from("--rustify-alpha-tw-"), &sample]);
        let expected_value = text(&[
            &Text::from("outer(var(--rustify-alpha-tw-"),
            &sample,
            &Text::from("), \""),
            &sample,
            &Text::from("\", "),
            &sample,
            &Text::from(")"),
        ]);
        assert_eq!(
            fields(declaration).0.units(),
            expected_property.units(),
            "{id}"
        );
        assert_eq!(
            fields(declaration).1.units(),
            expected_value.units(),
            "{id}"
        );
        assert_provenance(declaration, &original);
        assert_eq!(fields(&original).0.units(), property.units());
        assert_eq!(fields(&original).1.units(), value.units());
        let expected_css = text(&[&Text::from("@scope ([data-rustify-style-owner=\"alpha\"]) to ([data-rustify-style-owner]) {\n  "), &expected_property, &Text::from(": "), &expected_value, &Text::from(";\n}\n")]);
        assert_eq!(ast::to_css(&scoped).units(), expected_css.units(), "{id}");
    }
}
