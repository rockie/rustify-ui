use super::{value, CompileError, Compiler, CompilerInput, Limits};
use crate::text::Text;
use serde_json::Value;
use value::ValueNode;

fn text(parts: &[&Text]) -> Text {
    let mut output = Text::default();
    for part in parts {
        output.push_text(part);
    }
    output
}

fn word(value: &Text) -> ValueNode {
    ValueNode::Word {
        value: value.clone(),
    }
}

fn separator(value: &str) -> ValueNode {
    ValueNode::Separator {
        value: value.into(),
    }
}

fn function(name: &Text, nodes: Vec<ValueNode>) -> ValueNode {
    ValueNode::Function {
        name: name.clone(),
        nodes,
    }
}

fn json_roundtrip(nodes: &[ValueNode], id: &str) {
    let json = serde_json::to_string(nodes).unwrap();
    let decoded: Vec<ValueNode> = serde_json::from_str(&json)
        .unwrap_or_else(|error| panic!("{id}: typed node JSON: {error}"));
    assert!(
        decoded == nodes,
        "{id}: typed node JSON changed units or structure"
    );
}

fn assert_parse(input: &Text, expected: Vec<ValueNode>, expected_css: &Text, id: &str) {
    let actual: Vec<ValueNode> =
        value::parse_text(input, 32).unwrap_or_else(|error| panic!("{id}: {error}"));
    assert_eq!(actual, expected, "{id}: value AST");
    assert_eq!(
        value::to_css_text(&actual).units(),
        expected_css.units(),
        "{id}: printed units"
    );
    json_roundtrip(&expected, id);
}

fn fixtures() -> Value {
    serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap()
}

fn units(case: &Value, field: &str) -> Text {
    Text::from_units(serde_json::from_value(case[field].clone()).unwrap())
}

fn declaration(css: &Text, property: &str) -> Text {
    let marker = Text::from(format!("{property}: "));
    let start = css
        .units()
        .windows(marker.units().len())
        .position(|units| units == marker.units())
        .unwrap()
        + marker.units().len();
    let end = css.units()[start..]
        .iter()
        .position(|unit| *unit == 59)
        .unwrap()
        + start;
    Text::from_units(css.units()[start..end].to_vec())
}

fn samples() -> Vec<(String, Text)> {
    fixtures()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|case| {
            let id = case["id"].as_str().unwrap();
            id.strip_prefix("utf16:authored:value:").map(|name| {
                (
                    name.into(),
                    declaration(&units(case, "inputUtf16Units"), "value"),
                )
            })
        })
        .collect()
}

#[test]
fn frozen_utf16_authored_values_keep_expected_units_and_node_shapes() {
    let fixture = fixtures();
    let mut cases = 0;
    let mut declarations = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let Some(context) = ["value", "quoted", "function"]
            .into_iter()
            .find(|context| id.starts_with(&format!("utf16:authored:{context}:")))
        else {
            continue;
        };
        cases += 1;
        let input = units(case, "inputUtf16Units");
        let css = units(case, "expectedCssUtf16Units");
        let properties: &[&str] = if context == "quoted" {
            &["content", "value"]
        } else {
            &["value"]
        };
        for property in properties {
            declarations += 1;
            let expression = declaration(&input, property);
            let expected_css = declaration(&css, property);
            let expected = match (context, *property) {
                ("function", _) => {
                    let start = "outer(".encode_utf16().count();
                    let end = expression.units()[start..]
                        .iter()
                        .position(|unit| *unit == 44)
                        .unwrap()
                        + start;
                    let sample = Text::from_units(expression.units()[start..end].to_vec());
                    vec![function(
                        &Text::from("outer"),
                        vec![
                            word(&sample),
                            separator(", "),
                            function(&Text::from("inner"), vec![word(&sample)]),
                        ],
                    )]
                }
                ("quoted", "value") => vec![function(
                    &Text::from("url"),
                    vec![word(&Text::from_units(
                        expression.units()[4..expression.units().len() - 1].to_vec(),
                    ))],
                )],
                _ => vec![word(&expression)],
            };
            assert_parse(
                &expression,
                expected,
                &expected_css,
                &format!("{id}:{property}"),
            );
        }
    }
    assert_eq!(cases, 27);
    assert_eq!(declarations, 36);
}

#[test]
fn frozen_utf16_function_helpers_keep_inputs_outputs_and_typed_node_json() {
    let fixture: Value =
        serde_json::from_str(include_str!("utf16-function-fixtures.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 75);
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let input = units(case, "valueUtf16Units");
        let expected = units(case, "expectedValueUtf16Units");
        let nodes: Vec<ValueNode> = value::parse_text(&input, 32).unwrap();
        assert_eq!(
            value::to_css_text(&nodes).units(),
            input.units(),
            "{id}: input units"
        );
        json_roundtrip(&nodes, id);
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
        let output = compiler
            .substitute_functions_in_value(&input, false)
            .unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            output.units(),
            expected.units(),
            "{id}: helper output units"
        );
        let output_json = serde_json::to_string(&output).unwrap();
        assert_eq!(
            serde_json::from_str::<Text>(&output_json).unwrap().units(),
            expected.units(),
            "{id}: output JSON units"
        );
        let output_nodes: Vec<ValueNode> = value::parse_text(&expected, 32).unwrap();
        assert_eq!(
            value::to_css_text(&output_nodes).units(),
            expected.units(),
            "{id}: output AST units"
        );
        json_roundtrip(&output_nodes, id);
    }
}

#[test]
fn utf16_function_names_and_words_keep_frozen_sample_units() {
    let samples = samples();
    assert_eq!(samples.len(), 9);
    for (id, sample) in samples {
        let input = text(&[
            &sample,
            &Text::from("("),
            &sample,
            &Text::from(", inner("),
            &sample,
            &Text::from("))"),
        ]);
        let expected = vec![function(
            &sample,
            vec![
                word(&sample),
                separator(", "),
                function(&Text::from("inner"), vec![word(&sample)]),
            ],
        )];
        assert_parse(&input, expected, &input, &id);
    }
}

#[test]
fn utf16_parser_edge_shapes_follow_fixed_reference() {
    // Expected ASTs follow the pinned value-parser.ts buffer/stack transitions,
    // including its root-level remainder for an unclosed function.
    for (id, sample) in samples() {
        let escaped = text(&[&sample, &Text::from("\\")]);
        let escaped_css = text(&[&sample, &Text::from("\\undefined")]);
        assert_parse(
            &escaped,
            vec![word(&escaped_css)],
            &escaped_css,
            &format!("{id}:trailing-escape"),
        );
        assert_parse(
            &text(&[&sample, &Text::from(")tail")]),
            vec![word(&Text::from("tail"))],
            &Text::from("tail"),
            &format!("{id}:stray-close"),
        );
        assert_parse(
            &text(&[&Text::from("outer("), &sample]),
            vec![function(&Text::from("outer"), vec![]), word(&sample)],
            &text(&[&Text::from("outer()"), &sample]),
            &format!("{id}:unclosed"),
        );
        assert_parse(
            &text(&[&Text::from("outer(inner("), &sample]),
            vec![
                function(
                    &Text::from("outer"),
                    vec![function(&Text::from("inner"), vec![])],
                ),
                word(&sample),
            ],
            &text(&[&Text::from("outer(inner())"), &sample]),
            &format!("{id}:nested-unclosed"),
        );
        let quote = text(&[&Text::from("\""), &sample]);
        assert_parse(
            &quote,
            vec![word(&quote)],
            &quote,
            &format!("{id}:unclosed-quote"),
        );
        let crlf = text(&[&sample, &Text::from("\r\n:,\t="), &sample]);
        let normalized = text(&[&sample, &Text::from("\n:,\t="), &sample]);
        assert_parse(
            &crlf,
            vec![word(&sample), separator("\n:,\t="), word(&sample)],
            &normalized,
            &format!("{id}:crlf"),
        );
        let control = text(&[&sample, &Text::from("\r\u{c}"), &sample]);
        assert_parse(
            &control,
            vec![word(&control)],
            &control,
            &format!("{id}:other-controls"),
        );
        let quote = text(&[&Text::from("\""), &sample, &Text::from("\\\"x\"")]);
        assert_parse(
            &quote,
            vec![word(&quote)],
            &quote,
            &format!("{id}:escaped-quote"),
        );
    }
}

#[test]
fn typed_value_nodes_roundtrip_all_code_units_in_every_text_field() {
    let all = Text::from_units((0..=u16::MAX).collect());
    let expected = vec![
        word(&all),
        ValueNode::Separator { value: all.clone() },
        function(&all, vec![function(&all, vec![word(&all)])]),
    ];
    json_roundtrip(&expected, "all-65536-code-units");
    let scalar = vec![function(
        &Text::from("outer"),
        vec![
            word(&Text::from("😀")),
            separator(", "),
            word(&Text::from("x")),
        ],
    )];
    assert_eq!(serde_json::to_string(&scalar).unwrap(),
        "[{\"kind\":\"function\",\"name\":\"outer\",\"nodes\":[{\"kind\":\"word\",\"value\":\"😀\"},{\"kind\":\"separator\",\"value\":\", \"},{\"kind\":\"word\",\"value\":\"x\"}]}]");
}

#[test]
fn utf16_segmentation_keeps_delimiters_inside_escaped_quoted_and_nested_text() {
    for (id, sample) in samples() {
        let expected = vec![
            sample.clone(),
            text(&[
                &Text::from("outer("),
                &sample,
                &Text::from(", inner("),
                &sample,
                &Text::from("))"),
            ]),
            text(&[&Text::from("["), &sample, &Text::from(",x]")]),
            text(&[&Text::from("{"), &sample, &Text::from(",y}")]),
            text(&[&Text::from("\""), &sample, &Text::from(",z\"")]),
            text(&[&sample, &Text::from("\\,"), &sample]),
            Text::default(),
        ];
        let mut input = Text::default();
        for (index, part) in expected.iter().enumerate() {
            if index > 0 {
                input.push_str(",");
            }
            input.push_text(part);
        }
        let actual = value::segment_text(&input, 44, 32).unwrap();
        assert_eq!(actual, expected, "{id}: segmentation units");
    }
    assert!(
        matches!(value::segment_text(&Text::from("outer(inner(x))"), 44, 1),
        Err(CompileError::Limit { resource, limit: 1 }) if resource == "ast_depth")
    );
}

fn instantiate(pattern: &str, sample: &Text) -> Text {
    let mut output = Text::default();
    for (index, part) in pattern.split("{u}").enumerate() {
        if index > 0 {
            output.push_text(sample);
        }
        output.push_str(part);
    }
    output
}

fn assert_decode(patterns: &[(&str, &str)]) {
    let samples = samples();
    assert_eq!(samples.len(), 9);
    for (id, sample) in samples {
        for (input_pattern, expected_pattern) in patterns {
            let input = instantiate(input_pattern, &sample);
            let expected = instantiate(expected_pattern, &sample);
            let actual = value::decode_arbitrary_text(&input, 32)
                .unwrap_or_else(|error| panic!("{id}:{input_pattern}: {error}"));
            assert_eq!(actual.units(), expected.units(), "{id}:{input_pattern}");
            let json = serde_json::to_string(&actual).unwrap();
            assert_eq!(
                serde_json::from_str::<Text>(&json).unwrap().units(),
                expected.units(),
                "{id}:{input_pattern}: decoded JSON units"
            );
            if let (Ok(scalar_input), Ok(scalar_expected)) = (input.to_utf8(), expected.to_utf8()) {
                assert_eq!(
                    value::decode_arbitrary(&scalar_input, 32).unwrap(),
                    scalar_expected,
                    "{id}:{input_pattern}: scalar adapter"
                );
            }
        }
    }
}

#[test]
fn utf16_arbitrary_decoding_preserves_url_and_first_argument_units() {
    // Underscore rules are independently transcribed from the fixed
    // decode-arbitrary-value.ts and its URL/var/theme tests.
    let patterns = [
        ("_{u}_x", " {u} x"),
        (r"_{u}\_x_", " {u}_x "),
        (r"url(./{u}_file\_name.jpg)", r"url(./{u}_file\_name.jpg)"),
        (
            r"no-repeat_url(./{u}_file\_name.jpg)",
            r"no-repeat url(./{u}_file\_name.jpg)",
        ),
        ("var(--{u}_1_5,_1rem)", "var(--{u}_1_5, 1rem)"),
        (
            "theme(padding.{u}_1_5,_1rem)",
            "theme(padding.{u}_1_5, 1rem)",
        ),
        (
            r"var(--{u}\_1_5,_var(--x_2_5,_{u}))",
            "var(--{u}_1_5, var(--x_2_5, {u}))",
        ),
        (
            r"theme(padding.{u}\_1_5,_theme(padding.x_2_5,_{u}))",
            "theme(padding.{u}_1_5, theme(padding.x_2_5, {u}))",
        ),
        ("_{u}_fn(_{u}_,_{u})", " {u} fn( {u} , {u})"),
        ("_{u}_var(--x_1_5,_{u})", " {u} var(--x_1_5, {u})"),
        (
            "_{u}_theme(padding.x_1_5,_{u})",
            " {u} theme(padding.x_1_5, {u})",
        ),
        ("var(_{u},_{u})", "var(_{u}, {u})"),
    ];
    assert_eq!(patterns.len(), 12);
    assert_decode(&patterns);
}

#[test]
fn utf16_arbitrary_math_formatting_keeps_raw_units_and_reference_operator_rules() {
    // Math spacing expectations come from the fixed decode test table and
    // math-operators.ts: numeric operands, scientific exponents, unary signs,
    // commas, nested functions, and nonnumeric keyword tokens are distinct.
    let patterns = [
        ("calc({u}+2)", "calc({u} + 2)"),
        ("calc(2+{u})", "calc(2 + {u})"),
        ("calc(var(--{u}_x)*100)", "calc(var(--{u}_x) * 100)"),
        ("min({u},calc(100%-20px))", "min({u}, calc(100% - 20px))"),
        ("max({u},1+2)", "max({u}, 1 + 2)"),
        (
            "clamp(-3px+4px,var(--{u}_x),-3px+4px)",
            "clamp(-3px + 4px, var(--{u}_x), -3px + 4px)",
        ),
        (
            "calc(1px*(7--12/24)+var(--{u}_x))",
            "calc(1px * (7 - -12 / 24) + var(--{u} x))",
        ),
        (
            "min(-3.4e-2-var(--{u}_x),calc-size(auto))",
            "min(-3.4e-2 - var(--{u} x), calc-size(auto))",
        ),
        (
            "clamp(-10e3-var(--{u}_x),calc-size(max-content),var(--{u}_x)+-10e3)",
            "clamp(-10e3 - var(--{u} x), calc-size(max-content), var(--{u}_x) + -10e3)",
        ),
        (
            "calc(var(--{u}_x)*calc(100%+50%))",
            "calc(var(--{u}_x) * calc(100% + 50%))",
        ),
        (
            "calc(env(safe-area-{u},calc(10px+20px))+5px)",
            "calc(env(safe-area-{u},calc(10px + 20px)) + 5px)",
        ),
        (
            "min(theme(spacing.{u}_bar),fit-content,calc(20*calc(40-30)))",
            "min(theme(spacing.{u}_bar), fit-content, calc(20 * calc(40 - 30)))",
        ),
        ("minmax({u}-content,25%)", "minmax({u}-content,25%)"),
        ("round(to-zero,{u},1+3)", "round(to-zero, {u}, 1 + 3)"),
        ("calc({u}xé+{u}yé)", "calc({u}xé+{u}yé)"),
        ("calc({u}x + {u}y)", "calc({u}x + {u}y)"),
        ("atan(1_+_-infinity)_{u}", "atan(1 + -infinity) {u}"),
        (
            "fit-content(min(max-content,max(min-content,calc(20px+1em))),{u})",
            "fit-content(min(max-content, max(min-content, calc(20px + 1em))),{u})",
        ),
        (
            "var(--{u}_x,_calc(100%+1rem))",
            "var(--{u}_x, calc(100% + 1rem))",
        ),
        ("calc(24px_+_(-1rem))_{u}", "calc(24px + (-1rem)) {u}"),
    ];
    assert_eq!(patterns.len(), 20);
    assert_decode(&patterns);
}

#[test]
fn utf16_arbitrary_math_trim_end_uses_ecmascript_whitespace_units() {
    // Fixed math-operators.ts uses JS trimEnd: FEFF is whitespace, 0085 is not.
    // The original whitespace remains in output; only the previous-token
    // classification controls the spaces inserted around the operator.
    let patterns = [
        ("calc(1\u{feff}+{u})", "calc(1\u{feff} + {u})"),
        ("calc(1\u{85}+{u})", "calc(1\u{85}+{u})"),
        ("calc(var(--x)\u{feff}+{u})", "calc(var(--x)\u{feff} + {u})"),
        ("calc(var(--x)\u{85}+{u})", "calc(var(--x)\u{85}+{u})"),
    ];
    assert_eq!(patterns.len(), 4);
    assert_decode(&patterns);
}
