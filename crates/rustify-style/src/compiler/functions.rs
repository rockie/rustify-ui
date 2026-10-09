//! Tailwind CSS function substitution shares the candidate and authored CSS path.

use super::{
    ast::AstNode,
    theme::Theme,
    value::{self, ValueNode},
    CompileError,
};
use crate::text::Text;
use std::collections::BTreeSet;

fn syntax(message: impl Into<Text>) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

fn known(name: &str) -> bool {
    matches!(name, "--alpha" | "--spacing" | "--theme" | "theme")
}

#[cfg(test)]
pub(super) fn contains(nodes: &[AstNode]) -> bool {
    nodes.iter().any(|node| {
        (match node {
            AstNode::Declaration { value, .. } => ["--alpha(", "--spacing(", "--theme(", "theme("]
                .iter()
                .any(|name| value.contains(name)),
            AstNode::AtRule { name, params, .. }
                if ["@media", "@custom-media", "@container", "@supports"]
                    .iter()
                    .any(|label| name == label) =>
            {
                ["--alpha(", "--spacing(", "--theme(", "theme("]
                    .iter()
                    .any(|function| params.contains(function))
            }
            _ => false,
        }) || node.children().is_some_and(contains)
    })
}

fn trim(text: &Text) -> Text {
    fn whitespace(unit: u16) -> bool {
        matches!(unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279)
    }
    let units = text.units();
    let start = units
        .iter()
        .position(|unit| !whitespace(*unit))
        .unwrap_or(units.len());
    let end = units
        .iter()
        .rposition(|unit| !whitespace(*unit))
        .map_or(start, |index| index + 1);
    Text::from_units(units[start..end].to_vec())
}

fn join(parts: &[Text], separator: &str) -> Text {
    let mut text = Text::default();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push_str(separator);
        }
        text.push_text(part);
    }
    text
}

fn unquote(path: &Text) -> Text {
    let units = path.units();
    let Some(&quote @ (39 | 34)) = units.first() else {
        return path.clone();
    };
    let mut output = Vec::new();
    let mut offset = 1;
    while offset < units.len().saturating_sub(1) {
        let unit = units[offset];
        if unit == 92
            && units
                .get(offset + 1)
                .is_some_and(|next| *next == quote || *next == 92)
        {
            offset += 1;
        }
        output.push(units[offset]);
        offset += 1;
    }
    Text::from_units(output)
}

fn inject_fallback(nodes: &mut [ValueNode], fallback: &Text) {
    for node in nodes {
        if let ValueNode::Function { name, nodes } = node {
            if name == "var" || name == "theme" || name == "--theme" {
                if nodes.len() == 1 {
                    let mut value = Text::from(", ");
                    value.push_text(fallback);
                    nodes.push(ValueNode::Word { value });
                } else if let Some(ValueNode::Word { value }) = nodes.last_mut() {
                    if value == "initial" {
                        *value = fallback.clone();
                    }
                }
            }
            inject_fallback(nodes, fallback);
        }
    }
}

fn with_alpha(color: &Text, alpha: &Text) -> Text {
    let alpha = alpha
        .to_utf8()
        .ok()
        .and_then(|alpha| super::colors::js_number(&alpha))
        .map_or_else(
            || alpha.clone(),
            |number| Text::from(format!("{}%", super::colors::js_string(number * 100.0))),
        );
    if alpha == "100%" {
        return color.clone();
    }
    let mut result = Text::from("color-mix(in oklab, ");
    result.push_text(color);
    result.push_str(" ");
    result.push_text(&alpha);
    result.push_str(", transparent)");
    result
}

fn resolve_theme(theme: &Theme, path: &Text, inline: bool) -> Result<Option<Text>, CompileError> {
    let slash = path.units().iter().rposition(|unit| *unit == 47);
    let (path, modifier) = slash.map_or((path.clone(), None), |index| {
        (
            trim(&Text::from_units(path.units()[..index].to_vec())),
            Some(trim(&Text::from_units(path.units()[index + 1..].to_vec()))),
        )
    });
    let resolved = if path.starts_with("--") {
        if inline {
            theme.raw_text(&path, &mut BTreeSet::new())
        } else {
            theme.resolve_text(&path, &mut BTreeSet::new())
        }
    } else {
        super::legacy_theme::resolve_path_text(theme, &path)?
    };
    Ok(resolved.map(|resolved| {
        modifier
            .filter(|alpha| !alpha.is_empty())
            .map_or_else(|| resolved.clone(), |alpha| with_alpha(&resolved, &alpha))
    }))
}

fn evaluate(
    name: &str,
    args: &Text,
    theme: &Theme,
    at_rule: bool,
    depth: usize,
) -> Result<Text, CompileError> {
    let args = value::segment_text(&trim(args), 44, depth)?;
    let args: Vec<_> = args.iter().map(trim).collect();
    let empty = Text::default();
    let first = args.first().unwrap_or(&empty);
    match name {
        "--alpha" => {
            let parts = value::segment_text(first, 47, depth)?;
            let color = parts.first().map_or_else(Text::default, trim);
            let alpha = parts.get(1).map_or_else(Text::default, trim);
            if color.is_empty() || alpha.is_empty() || args.len() > 1 {
                let mut message = Text::from(if color.is_empty() || alpha.is_empty() {
                    "The --alpha(…) function requires a color and an alpha value, e.g.: "
                } else {
                    "The --alpha(…) function only accepts one argument, e.g.: "
                });
                message.push_str("`--alpha(");
                if color.is_empty() {
                    message.push_str("var(--my-color)");
                } else {
                    message.push_text(&color);
                }
                message.push_str(" / ");
                if alpha.is_empty() {
                    message.push_str("50%");
                } else {
                    message.push_text(&alpha);
                }
                message.push_str(")`");
                return Err(syntax(message));
            }
            Ok(with_alpha(&color, &alpha))
        }
        "--spacing" => {
            if first.is_empty() {
                return Err(syntax(
                    "The --spacing(…) function requires an argument, but received none.",
                ));
            }
            if args.len() > 1 {
                return Err(syntax(format!(
                    "The --spacing(…) function only accepts a single argument, but received {}.",
                    args.len()
                )));
            }
            let multiplier = theme.resolve_text(&Text::from("--spacing"), &mut BTreeSet::new()).ok_or_else(|| syntax("The --spacing(…) function requires that the `--spacing` theme variable exists, but it was not found."))?;
            let mut output = Text::from("calc(");
            output.push_text(&multiplier);
            output.push_str(" * ");
            output.push_text(first);
            output.push_str(")");
            Ok(output)
        }
        "--theme" | "theme" => {
            let legacy = name == "theme";
            let mut path = if legacy {
                unquote(first)
            } else {
                first.clone()
            };
            if !legacy && !path.starts_with("--") {
                return Err(syntax(
                    "The --theme(…) function can only be used with CSS variables from your theme.",
                ));
            }
            let mut inline = legacy || at_rule;
            if !legacy && path.units().ends_with(&[32, 105, 110, 108, 105, 110, 101]) {
                inline = true;
                path = Text::from_units(path.units()[..path.units().len() - 7].to_vec());
            }
            let fallback = join(&args[1..], ", ");
            let Some(resolved) =
                resolve_theme(theme, &path, inline)?.filter(|value| !value.is_empty())
            else {
                if args.len() > 1 {
                    return Ok(fallback);
                }
                let mut message = Text::from("Could not resolve value for theme function: `theme(");
                message.push_text(&path);
                message.push_str(")`. Consider checking if the ");
                message.push_str(if legacy { "path" } else { "variable name" });
                message.push_str(" is correct or provide a fallback value to silence this error.");
                return Err(syntax(message));
            };
            if legacy || args.len() == 1 || fallback == "initial" {
                return Ok(resolved);
            }
            if resolved == "initial" {
                return Ok(fallback);
            }
            if resolved.starts_with("var(")
                || resolved.starts_with("theme(")
                || resolved.starts_with("--theme(")
            {
                let mut ast = value::parse_text(&resolved, depth)?;
                inject_fallback(&mut ast, &fallback);
                return Ok(value::to_css_text(&ast));
            }
            Ok(resolved)
        }
        _ => unreachable!(),
    }
}

pub(super) fn in_text_value(
    value: &Text,
    theme: &Theme,
    at_rule: bool,
    depth: usize,
    max_bytes: usize,
) -> Result<Text, CompileError> {
    if !["--alpha(", "--spacing(", "--theme(", "theme("]
        .iter()
        .any(|name| value.contains(name))
    {
        return Ok(value.clone());
    }
    fn visit(
        nodes: &[ValueNode],
        theme: &Theme,
        at_rule: bool,
        budget: usize,
        limit: usize,
        max_bytes: usize,
    ) -> Result<Text, CompileError> {
        let mut output = Text::default();
        for node in nodes {
            if budget == 0 && matches!(node, ValueNode::Function { .. }) {
                return Err(CompileError::Limit {
                    resource: "ast_depth".into(),
                    limit,
                });
            }
            let rendered = match node {
                ValueNode::Function { name, nodes }
                    if name.to_utf8().is_ok_and(|name| known(&name)) =>
                {
                    let result = evaluate(
                        &name
                            .to_utf8()
                            .map_err(|_| syntax("Non-scalar CSS function name"))?,
                        &value::to_css_text(nodes),
                        theme,
                        at_rule,
                        limit,
                    )?;
                    super::check_limit(result.len(), max_bytes, "css_bytes")?;
                    visit(
                        &value::parse_text(&result, limit)?,
                        theme,
                        at_rule,
                        budget - 1,
                        limit,
                        max_bytes,
                    )?
                }
                ValueNode::Function { name, nodes } => {
                    let children = visit(nodes, theme, at_rule, budget - 1, limit, max_bytes)?;
                    super::check_limit(
                        name.len().saturating_add(children.len()).saturating_add(2),
                        max_bytes,
                        "css_bytes",
                    )?;
                    let mut result = name.clone();
                    result.push_str("(");
                    result.push_text(&children);
                    result.push_str(")");
                    result
                }
                ValueNode::Word { value } | ValueNode::Separator { value } => value.clone(),
            };
            super::check_limit(
                output.len().saturating_add(rendered.len()),
                max_bytes,
                "css_bytes",
            )?;
            output.push_text(&rendered);
        }
        Ok(output)
    }
    visit(
        &value::parse_text(value, depth)?,
        theme,
        at_rule,
        depth,
        depth,
        max_bytes,
    )
}

pub(super) fn substitute(
    nodes: &mut [AstNode],
    theme: &Theme,
    depth: usize,
    max_bytes: usize,
) -> Result<(), CompileError> {
    let mut edits = super::origin::Edits::from_nodes(nodes);
    substitute_with_edits(nodes, theme, depth, max_bytes, &mut edits)?;
    edits.finish().bind(nodes);
    Ok(())
}

pub(super) fn substitute_current(
    root: &super::public_api::PublicAst,
    theme: &Theme,
    current: &mut super::origin::current::Current,
    allowance: &mut super::origin::current::Allowance,
    limits: &super::Limits,
    effects: &mut Vec<super::public_api::InputMutation>,
) -> Result<bool, CompileError> {
    use super::{origin::current::Action, public_api::PublicValue as Value};
    let mut found = false;
    current.walk(root, allowance, limits.ast_depth, &mut |node, cursor| {
        let field = if node["kind"] == "declaration" {
            Some("value")
        } else if node["kind"] == "at-rule"
            && ["@media", "@custom-media", "@container", "@supports"]
                .iter()
                .any(|name| node["name"] == *name)
        {
            Some("params")
        } else {
            None
        };
        if let Some((field, value)) = field
            .and_then(|field| {
                node.get(field)
                    .and_then(Value::as_text)
                    .map(|value| (field, value))
            })
            .filter(|(_, value)| has_function(value))
        {
            found = true;
            let value = Value::from(in_text_value(
                value,
                theme,
                field == "params",
                limits.ast_depth,
                limits.css_bytes,
            )?);
            let captured = cursor.assign(
                node,
                field,
                &value,
                4 * std::mem::size_of::<super::public_api::InputMutation>() + field.len() * 2,
            )?;
            effects.push(super::public_api::InputMutation::Set {
                target: node.clone(),
                field: field.into(),
                value: captured.anchor,
            });
        }
        Ok(Action::Continue)
    })?;
    Ok(found)
}

pub(super) fn substitute_with_edits(
    nodes: &mut [AstNode],
    theme: &Theme,
    depth: usize,
    max_bytes: usize,
    edits: &mut super::origin::Edits,
) -> Result<(), CompileError> {
    edits.include_nodes(nodes);
    for node in nodes {
        edits.refresh(node);
        let replacement = match node {
            AstNode::Declaration { value, .. } if has_function(value) => Some((
                "value",
                in_text_value(value, theme, false, depth, max_bytes)?,
            )),
            AstNode::AtRule { name, params, .. }
                if ["@media", "@custom-media", "@container", "@supports"]
                    .iter()
                    .any(|label| name == label)
                    && has_function(params) =>
            {
                Some((
                    "params",
                    in_text_value(params, theme, true, depth, max_bytes)?,
                ))
            }
            _ => None,
        };
        if let Some((field, value)) = replacement {
            edits.assign(node, field, value)?;
        }
        if let Some(children) = node.children_mut() {
            substitute_with_edits(children, theme, depth, max_bytes, edits)?;
        }
    }
    Ok(())
}
fn has_function(value: &Text) -> bool {
    ["--alpha(", "--spacing(", "--theme(", "theme("]
        .iter()
        .any(|name| value.contains(name))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{CompileError, Compiler, CompilerInput, Limits};

    #[test]
    fn utf16_declarations_substitute_functions_without_losing_identity_or_units() {
        let mut input =
            super::super::parser::parse("@theme reference { --spacing: 4px; }", 32).unwrap();
        let theme = super::super::theme::Theme::extract(&mut input, 32).unwrap();
        let mut value = crate::text::Text::from("--spacing(");
        value.push_text(&crate::text::Text::from_units(vec![0xd800]));
        value.push_str(")");
        let mut nodes = vec![super::AstNode::declaration("value", value)];
        let original = nodes.clone();
        assert!(super::contains(&nodes));
        super::substitute(&mut nodes, &theme, 32, 262144).unwrap();
        let super::AstNode::Declaration {
            value, identity, ..
        } = &nodes[0]
        else {
            panic!("expected Declaration");
        };
        let super::AstNode::Declaration { identity: old, .. } = &original[0] else {
            panic!("expected Declaration");
        };
        assert!(std::sync::Arc::ptr_eq(identity, old));
        let mut expected = crate::text::Text::from("calc(var(--spacing, 4px) * ");
        expected.push_text(&crate::text::Text::from_units(vec![0xd800]));
        expected.push_str(")");
        assert_eq!(value, &expected);
    }

    #[test]
    fn native_utf16_function_arguments_match_the_fixed_official_compiler() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/utf16-function-fixtures.json"
        ))
        .unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let css = case["themeCss"].as_str().unwrap();
            let compiler = Compiler::new(
                CompilerInput {
                    css: css.into(),
                    entry: String::new(),
                },
                Vec::new(),
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
            let units = serde_json::from_value(case["valueUtf16Units"].clone()).unwrap();
            let input = crate::text::Text::from_units(units);
            let result = compiler
                .substitute_functions_in_value(&input, false)
                .unwrap();
            let expected: Vec<u16> =
                serde_json::from_value(case["expectedValueUtf16Units"].clone()).unwrap();
            assert_eq!(result.units(), expected, "{}", case["id"]);
        }
    }

    #[test]
    fn scalar_authored_utf16_escape_and_primitive_outputs_match_the_official_matrix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/utf16-fixtures.json"
        ))
        .unwrap();
        let mut checked = 0;
        for case in fixture["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            if !id.starts_with("utf16:escape:theme-key:") && !id.starts_with("utf16:primitive:") {
                continue;
            }
            let units: Vec<u16> = serde_json::from_value(case["inputUtf16Units"].clone()).unwrap();
            let Ok(css) = String::from_utf16(&units) else {
                continue;
            };
            let result = Compiler::new(
                CompilerInput {
                    css: css.into(),
                    entry: String::new(),
                },
                Vec::new(),
                Limits {
                    input_bytes: 65536,
                    asset_bytes: 65536,
                    candidate_count: 1024,
                    ast_depth: 32,
                    css_bytes: 262144,
                    queued_bytes: 65536,
                    cache_bytes: 4194304,
                },
            );
            checked += 1;
            if case.get("expectedError").is_some() {
                let expected: Vec<u16> =
                    serde_json::from_value(case["expectedError"]["messageUtf16Units"].clone())
                        .unwrap();
                let (name, message) = match result.err().unwrap() {
                    CompileError::Syntax { message, .. } => ("Error", message),
                    CompileError::RangeError { message } => ("RangeError", message),
                    error => panic!("{id}: unexpected {error:?}"),
                };
                assert_eq!(case["expectedError"]["phase"], "compile", "{id}");
                assert_eq!(
                    crate::text::Text::from(name).units(),
                    serde_json::from_value::<Vec<u16>>(
                        case["expectedError"]["nameUtf16Units"].clone()
                    )
                    .unwrap(),
                    "{id}: name"
                );
                assert_eq!(message.units(), expected, "{id}: message");
            } else {
                let compiler = result.unwrap_or_else(|error| panic!("{id}: {error:?}"));
                let expected: Vec<u16> =
                    serde_json::from_value(case["expectedCssUtf16Units"].clone()).unwrap();
                assert_eq!(compiler.report().css.units(), expected, "{id}");
            }
        }
        assert_eq!(checked, 76);
    }

    #[test]
    fn official_non_scalar_utf16_outputs_are_lossless() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/legacy-theme-boundaries.json"
        ))
        .unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let result = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
                    entry: String::new(),
                },
                Vec::new(),
                Limits {
                    input_bytes: 65536,
                    asset_bytes: 65536,
                    candidate_count: 1024,
                    ast_depth: 32,
                    css_bytes: 262144,
                    queued_bytes: 65536,
                    cache_bytes: 1048576,
                },
            );
            let compiler = result.unwrap_or_else(|error| panic!("{}: {error:?}", case["id"]));
            let expected: Vec<u16> =
                serde_json::from_value(case["expectedCssUtf16Units"].clone()).unwrap();
            assert_eq!(compiler.report().css.units(), expected, "{}", case["id"]);
        }
    }

    #[test]
    fn evaluated_theme_aliases_are_available_to_gpu_and_cached_candidates() {
        let mut compiler = Compiler::new(
            CompilerInput { css: "@theme { --color-brand: #123456; --color-alias: --theme(--color-brand); --spacing: 4px; } @tailwind utilities;".into(), entry: String::new() },
            Vec::new(),
            Limits { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024,
                ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 },
        ).unwrap();
        let candidates = ["bg-alias".into(), "w-[--spacing(3)]".into()];
        let first = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(
            first.report().gpu_variables[&crate::text::Text::from("--color-alias")],
            "var(--color-brand)"
        );
        assert!(
            matches!(&first.report().gpu_programs.iter().find(|program| program.candidate == candidates[1]).unwrap().declarations[0].value,
            crate::compiler::GpuValue::CssExpression { value } if value == "calc(var(--spacing) * 3)")
        );
        compiler.commit(first).unwrap();
        let repeated = compiler.prepare_add(&candidates).unwrap();
        assert_eq!(repeated.report().metrics.parsed_candidates, 0);
        assert_eq!(repeated.report().metrics.generated_candidates, 0);
        assert_eq!(
            repeated.report().gpu_variables[&crate::text::Text::from("--color-alias")],
            "var(--color-brand)"
        );
    }

    #[test]
    fn function_expansion_is_bounded_before_materializing_repeated_theme_values() {
        let mut theme = super::Theme::extract(
            &mut crate::compiler::parser::parse(
                "@theme inline { --font-value: abcdefghijklmnopqrstuvwxyz; }",
                32,
            )
            .unwrap(),
            32,
        )
        .unwrap();
        theme.prepare_functions(32, 40, 1048576).unwrap();
        let actual = super::in_text_value(
            &crate::text::Text::from("--theme(--font-value) --theme(--font-value)"),
            &theme,
            false,
            32,
            40,
        );
        assert!(
            matches!(actual, Err(CompileError::Limit { resource, limit: 40 }) if resource == "css_bytes")
        );
    }

    #[test]
    fn oversized_function_expansion_rejects_the_batch_without_committing_candidates() {
        let mut compiler = Compiler::new(
            CompilerInput {
                css: "@tailwind utilities;".into(),
                entry: String::new(),
            },
            Vec::new(),
            Limits {
                input_bytes: 65536,
                asset_bytes: 65536,
                candidate_count: 1024,
                ast_depth: 32,
                css_bytes: 80,
                queued_bytes: 65536,
                cache_bytes: 1048576,
            },
        )
        .unwrap();
        let initial = compiler.prepare_add(&["block".into()]).unwrap();
        compiler.commit(initial).unwrap();
        let css = compiler.report().css.clone();
        let revision = compiler.revision();
        let result = compiler.prepare_add(&[
            "hidden".into(),
            "[value:theme(fontFamily.sans)_theme(fontFamily.sans)]".into(),
        ]);
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 80 }) if resource == "css_bytes")
        );
        assert_eq!(compiler.revision(), revision);
        assert_eq!(compiler.report().css, css);
        assert!(!compiler.report().admitted.contains(&"hidden".into()));
    }

    #[test]
    fn cached_theme_function_outputs_obey_the_configuration_cache_budget() {
        let result = Compiler::new(
            CompilerInput { css: "@theme inline { --font-value: abcdefghijklmnopqrstuvwxyz; --font-a: --theme(--font-value); --font-b: --theme(--font-value); } @tailwind utilities;".into(), entry: String::new() },
            Vec::new(),
            Limits { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024,
                ast_depth: 32, css_bytes: 1000, queued_bytes: 65536, cache_bytes: 40 },
        );
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 40 }) if resource == "cache_bytes")
        );
    }

    #[test]
    fn css_functions_match_official_authored_candidate_and_theme_cases() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/functions-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let result = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
                    entry: String::new(),
                },
                Vec::new(),
                Limits {
                    input_bytes: 65536,
                    asset_bytes: 65536,
                    candidate_count: 1024,
                    ast_depth: 32,
                    css_bytes: 262144,
                    queued_bytes: 65536,
                    cache_bytes: 1048576,
                },
            );
            let actual = if let Some(expected) = case["expectedError"].as_str() {
                match result {
                    Err(CompileError::Syntax { message, .. }) if message == expected => continue,
                    Err(error) => format!("{error:?}"),
                    Ok(_) => "unexpected success".into(),
                }
            } else {
                match result {
                    Ok(mut compiler) => {
                        let candidates = case["candidates"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|value| value.as_str().unwrap().to_owned())
                            .collect::<Vec<_>>();
                        match compiler.prepare_add(&candidates) {
                            Ok(prepared)
                                if prepared.report().css
                                    == case["expectedCss"].as_str().unwrap()
                                    && prepared.report().unsupported.is_empty() =>
                            {
                                continue;
                            }
                            Ok(prepared) => format!(
                                "CSS={:?}, pending={:?}",
                                prepared.report().css,
                                prepared.report().unsupported
                            ),
                            Err(error) => format!("{error:?}"),
                        }
                    }
                    Err(error) => format!("{error:?}"),
                }
            };
            failures.push(format!(
                "{}: {actual}; expected={}",
                case["id"],
                case["expectedError"]
                    .as_str()
                    .or(case["expectedCss"].as_str())
                    .unwrap()
            ));
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first: {:#?}",
            failures.len(),
            &failures[..failures.len().min(8)]
        );
    }
}
