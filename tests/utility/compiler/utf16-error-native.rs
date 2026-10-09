use super::{ast, CompileError, Compiler, CompilerInput, Limits};
use crate::text::Text;
use serde::Deserialize;
use serde_json::Value;

fn limits() -> Limits {
    Limits {
        input_bytes: 262144,
        asset_bytes: 262144,
        candidate_count: 1024,
        ast_depth: 32,
        css_bytes: 262144,
        queued_bytes: 262144,
        cache_bytes: 32_000_000,
    }
}

fn compiler(css: Text) -> Result<Compiler, CompileError> {
    Compiler::new(
        CompilerInput {
            css,
            entry: String::new(),
        },
        Vec::new(),
        limits(),
    )
}

fn units(value: &Value) -> Text {
    Text::from_units(serde_json::from_value(value.clone()).unwrap())
}

#[derive(Deserialize)]
struct ErrorFields {
    kind: String,
    message: Text,
}

fn assert_exception(error: &CompileError, kind: &str, message: &Text) {
    let json = serde_json::to_string(error).unwrap();
    assert_eq!(serde_json::from_str::<CompileError>(&json).unwrap(), *error);
    let fields: ErrorFields = serde_json::from_str(&json).unwrap();
    assert_eq!(fields.kind, kind);
    assert_eq!(fields.message.units(), message.units());
}

#[test]
fn frozen_errors_retain_all_61_setup_build_phases_classes_and_message_units() {
    let fixture: Value = serde_json::from_str(include_str!("utf16-fixtures.json")).unwrap();
    let mut counts = [0; 3];
    for case in fixture["cases"].as_array().unwrap() {
        let Some(expected) = case.get("expectedError") else {
            continue;
        };
        let id = case["id"].as_str().unwrap();
        let name = units(&expected["nameUtf16Units"]);
        let kind = if name == "RangeError" {
            "range_error"
        } else {
            assert_eq!(name, Text::from("Error"), "{id}");
            "syntax"
        };
        let candidates: Vec<String> = case["candidatesUtf16Units"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| units(value).to_utf8().unwrap())
            .collect();
        let (phase, error) = match compiler(units(&case["inputUtf16Units"])) {
            Err(error) => ("compile", error),
            Ok(mut compiler) => (
                "build",
                compiler
                    .prepare_add(&candidates)
                    .err()
                    .unwrap_or_else(|| panic!("{id}: expected build exception")),
            ),
        };
        assert_eq!(phase, expected["phase"].as_str().unwrap(), "{id}");
        assert_exception(&error, kind, &units(&expected["messageUtf16Units"]));
        counts[match (phase, kind) {
            ("compile", "range_error") => 0,
            ("build", "range_error") => 1,
            ("compile", "syntax") => 2,
            _ => panic!("{id}: unexpected error stage/class"),
        }] += 1;
    }
    assert_eq!(counts, [4, 2, 55]);
}

#[test]
fn setup_and_initial_ast_defer_authored_keyframe_and_function_output_usage() {
    for (escape, point) in [("\\110000", 1114112), ("\\ffffff", 16777215)] {
        for css in [
            format!(".p {{ value: var(--custom-{escape}); }} @tailwind utilities;"),
            format!("@theme {{ @keyframes card {{ to {{ value: var(--custom-{escape}); }} }} }} @tailwind utilities;"),
            format!(".p {{ value: theme(missing, var(--custom-{escape})); }} @tailwind utilities;"),
        ] {
            let mut compiler = compiler(css.into()).unwrap();
            compiler.initial_ast().unwrap();
            let initial = compiler.report().clone();
            for _ in 0..2 {
                let error = compiler.prepare_add(&[]).err().unwrap();
                assert_exception(&error, "range_error", &Text::from(format!("Invalid code point {point}")));
                assert_eq!(compiler.report(), &initial);
                assert!(compiler.pending.is_none());
                assert!(compiler.cache.is_empty());
            }
        }
    }
}

#[test]
fn features_none_preserves_original_units_without_variable_usage_analysis() {
    let mut css = Text::from(".p { value: var(--custom-\\110000); content: ");
    css.push_text(&Text::from_units(vec![0xd800, 0xd83d, 0xde00, 0xdc00]));
    css.push_str("; }");
    let mut compiler = compiler(css.clone()).unwrap();
    assert_eq!(compiler.features(), super::public_api::Features::NONE);
    compiler.initial_ast().unwrap();
    let prepared = compiler
        .prepare_add(&["--custom-\\110000".into(), "underline".into()])
        .unwrap();
    assert_eq!(prepared.report().css.units(), css.units());
    assert!(!prepared.report().changed);
    assert!(prepared.report().admitted.is_empty());
    let committed = compiler.commit(prepared).unwrap();
    assert_eq!(committed.revision, 0);
    assert_eq!(committed.css.units(), css.units());
}

#[test]
fn failed_external_variable_build_preserves_report_cache_and_allows_retry() {
    let mut compiler =
        compiler("@theme { --custom-card: red; } @tailwind utilities;".into()).unwrap();
    let first = compiler.prepare_add(&["underline".into()]).unwrap();
    compiler.commit(first).unwrap();
    let committed = compiler.report().clone();
    let entries = compiler.cache.len();
    for _ in 0..2 {
        let error = compiler
            .prepare_add(&["--custom-\\110000".into(), "block".into()])
            .err()
            .unwrap();
        assert_exception(&error, "range_error", &"Invalid code point 1114112".into());
        assert_eq!(compiler.report(), &committed);
        assert_eq!(compiler.cache.len(), entries);
        assert!(!compiler.cache.contains_key(&Text::from("block")));
        assert!(compiler.pending.is_none());
    }
    let next = compiler.prepare_add(&["block".into()]).unwrap();
    let report = compiler.commit(next).unwrap();
    assert!(report.admitted.contains(&"underline".into()));
    assert!(report.admitted.contains(&"block".into()));
}

#[test]
fn theme_dependency_variables_do_not_become_direct_usage_errors() {
    for (options, candidates) in [("static", Vec::new()), ("", vec!["--custom-card".into()])] {
        let css = format!(
            "@theme {options} {{ --custom-card: var(--custom-\\110000); }} @tailwind utilities;"
        );
        let mut compiler = compiler(css.into()).unwrap();
        let prepared = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(
            prepared.report().css.units(),
            Text::from(":root, :host {\n  --custom-card: var(--custom-\\110000);\n}\n").units()
        );
        assert_eq!(
            ast::to_css(&prepared.ast).units(),
            prepared.report().css.units()
        );
    }
}

#[test]
fn range_error_typed_json_preserves_isolated_units_and_reordered_fields() {
    for message in [
        vec![0xd800],
        vec![0xdc00],
        vec![0xd83d, 0xde00],
        vec![0xd800, 0xd801],
        vec![0xdc00, 0xdc01],
        vec![0xdc00, 0xd800],
    ] {
        let text = Text::from_units(message);
        let message = serde_json::to_string(&text).unwrap();
        for json in [
            format!("{{\"kind\":\"range_error\",\"message\":{message}}}"),
            format!("{{\"message\":{message},\"kind\":\"range_error\"}}"),
        ] {
            let error: CompileError = serde_json::from_str(&json).unwrap();
            assert_exception(&error, "range_error", &text);
        }
    }
    let missing = serde_json::from_str::<CompileError>(r#"{"kind":"range_error"}"#).unwrap_err();
    assert!(missing.to_string().contains("missing field `message`"));
}

#[test]
fn range_error_diagnostic_escapes_do_not_change_message_or_cached_capacity() {
    let mut units = Vec::with_capacity(257);
    units.extend([0xd800, 120, 0xdc00, 0xd83d, 0xde00]);
    let expected_allocation = units.capacity() * std::mem::size_of::<u16>();
    let message = Text::from_units(units);
    let logical = message.byte_len();
    let error = CompileError::RangeError { message };
    assert_eq!(error.to_string(), "RangeError: \\ud800x\\udc00😀");
    assert_exception(
        &error,
        "range_error",
        &Text::from_units(vec![0xd800, 120, 0xdc00, 0xd83d, 0xde00]),
    );
    let theme = super::theme::Theme::default();
    theme.legacy.set(Err(error)).unwrap();
    let usage = super::legacy_theme::cache_usage(&theme);
    assert_eq!(usage.string_bytes, logical);
    assert_eq!(
        usage.owned_bytes,
        std::mem::size_of::<CompileError>() + expected_allocation
    );
    assert!(expected_allocation > logical);
}
