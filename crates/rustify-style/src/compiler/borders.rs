//! Native border/divide/outline/stroke widths, decoration thickness/offset and opacity.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, resolve_named},
    dependencies::property,
    theme::Theme,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct BorderOutput {
    pub declarations: Vec<(Text, Text)>,
    pub globals: Vec<AstNode>,
    pub selector: Option<&'static str>,
    pub sort_property: Option<&'static str>,
    pub declaration_count: usize,
}

fn output(declarations: Vec<(Text, Text)>, globals: Vec<AstNode>) -> BorderOutput {
    BorderOutput {
        declaration_count: declarations.len(),
        declarations,
        globals,
        selector: None,
        sort_property: None,
    }
}

fn border_properties(root: &str) -> Option<(&'static str, &'static str)> {
    Some(match root {
        "border" => ("border-style", "border-width"),
        "border-x" => ("border-inline-style", "border-inline-width"),
        "border-y" => ("border-block-style", "border-block-width"),
        "border-s" => ("border-inline-start-style", "border-inline-start-width"),
        "border-e" => ("border-inline-end-style", "border-inline-end-width"),
        "border-t" => ("border-top-style", "border-top-width"),
        "border-r" => ("border-right-style", "border-right-width"),
        "border-b" => ("border-bottom-style", "border-bottom-width"),
        "border-l" => ("border-left-style", "border-left-width"),
        _ => return None,
    })
}

fn generic_value(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Text, super::utilities::FamilyFailure> {
    let namespaces: &[&str] = match root {
        "divide-x" | "divide-y" => &["divide-width", "border-width"],
        "outline-offset" => &["outline-offset"],
        "underline-offset" => &["text-underline-offset"],
        "opacity" => &["opacity"],
        _ => return Err(DiagnosticKind::InvalidValue.into()),
    };
    match value {
        None => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            if root.starts_with("divide-") {
                Ok(theme
                    .raw_text(&Text::from("--default-border-width"), used)
                    .unwrap_or_else(|| "1px".into()))
            } else {
                theme
                    .resolve_text(&Text::from(format!("--{}", namespaces[0])), used)
                    .ok_or(super::utilities::FamilyFailure::Candidate(
                        DiagnosticKind::InvalidValue,
                    ))
            }
        }
        Some(CandidateValue::Arbitrary { value, .. }) => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            Ok(value.clone())
        }
        Some(CandidateValue::Named { value }) => {
            let fraction = modifier
                .filter(|modifier| !(modifier.starts_with("[") || modifier.starts_with("(")))
                .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
            if let Some(value) =
                resolve_named(fraction.as_ref().unwrap_or(value), namespaces, theme, used)?
            {
                return Ok(value);
            }
            if modifier.is_some() || !canonical_nonnegative_text(value, root != "opacity") {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            Ok(concat_text(&[
                value,
                &if root == "opacity" { "%" } else { "px" }.into(),
            ]))
        }
    }
}

fn width_value(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    let Some(value) = value else {
        if modifier.is_some() || matches!(root, "stroke" | "decoration") {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        let key = if root == "outline" {
            "--default-outline-width"
        } else {
            "--default-border-width"
        };
        return Ok(Some(
            theme
                .raw_text(&Text::from(key), used)
                .unwrap_or_else(|| "1px".into()),
        ));
    };
    // Resolve color vs width through the already ported source branch rules,
    // without retaining dependencies from the interpretation that loses.
    match super::colors::compile(root, Some(value), modifier, theme, &mut BTreeSet::new()) {
        Ok(Some(_)) => return Ok(None),
        Err(super::utilities::FamilyFailure::Candidate(DiagnosticKind::PendingFamily)) => {}
        Err(kind) => return Err(kind),
        Ok(None) => return Ok(None),
    }
    if modifier.is_some() && !(root == "stroke" && matches!(value, CandidateValue::Named { .. })) {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    let css = match value {
        CandidateValue::Arbitrary { value, .. } => value.clone(),
        CandidateValue::Named { value } => {
            let namespace = match root {
                "outline" => "outline-width",
                "stroke" => "stroke-width",
                "decoration" => "text-decoration-thickness",
                _ => "border-width",
            };
            if let Some(value) = resolve_named(value, &[namespace], theme, used)? {
                value
            } else if canonical_nonnegative_text(value, true) {
                concat_text(&[value, &if root == "stroke" { "" } else { "px" }.into()])
            } else {
                return Err(DiagnosticKind::InvalidValue.into());
            }
        }
    };
    Ok(Some(css))
}

fn mark_variables(
    value: &Text,
    used: &mut BTreeSet<Text>,
) -> Result<(), super::utilities::FamilyFailure> {
    fn walk(nodes: &[super::value::ValueNode], used: &mut BTreeSet<Text>) {
        for node in nodes {
            if let super::value::ValueNode::Function { name, nodes } = node {
                if name == "var" {
                    if let Some(super::value::ValueNode::Word { value }) = nodes.first() {
                        if value.starts_with("--") {
                            used.insert(value.clone());
                        }
                    }
                }
                walk(nodes, used);
            }
        }
    }
    match super::value::parse_text(value, usize::MAX) {
        Ok(nodes) => walk(&nodes, used),
        Err(super::CompileError::Syntax { .. }) => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<BorderOutput>, super::utilities::FamilyFailure> {
    let negative = root.starts_with('-');
    let root = root.trim_start_matches('-');
    if negative && !matches!(root, "outline-offset" | "underline-offset") {
        return Ok(None);
    }
    let result = if let Some((style, width)) = border_properties(root) {
        let Some(value) = width_value(root, value, modifier, theme, used)? else {
            return Ok(None);
        };
        output(
            vec![
                (style.into(), "var(--tw-border-style)".into()),
                (width.into(), value),
            ],
            vec![property("--tw-border-style", Some("solid"), "*")],
        )
    } else if matches!(root, "outline" | "stroke" | "decoration") {
        let Some(value) = width_value(root, value, modifier, theme, used)? else {
            return Ok(None);
        };
        if root == "outline" {
            output(
                vec![
                    ("outline-style".into(), "var(--tw-outline-style)".into()),
                    ("outline-width".into(), value),
                ],
                vec![property("--tw-outline-style", Some("solid"), "*")],
            )
        } else {
            output(
                vec![(
                    if root == "stroke" {
                        "stroke-width"
                    } else {
                        "text-decoration-thickness"
                    }
                    .into(),
                    value,
                )],
                Vec::new(),
            )
        }
    } else if matches!(root, "divide-x" | "divide-y") {
        let value = generic_value(root, value, modifier, theme, used)?;
        let (variable, style, start, end, sort) = if root == "divide-x" {
            (
                "--tw-divide-x-reverse",
                "border-inline-style",
                "border-inline-start-width",
                "border-inline-end-width",
                "divide-x-width",
            )
        } else {
            (
                "--tw-divide-y-reverse",
                "border-bottom-style",
                "border-top-width",
                "border-bottom-width",
                "divide-y-width",
            )
        };
        let mut declarations = vec![
            (variable.into(), "0".into()),
            (style.into(), "var(--tw-border-style)".into()),
        ];
        if root == "divide-y" {
            declarations.push(("border-top-style".into(), "var(--tw-border-style)".into()));
        }
        let mut start_value = Text::from("calc(");
        start_value.push_text(&value);
        start_value.push_str(&format!(" * var({variable}))"));
        declarations.push((start.into(), start_value));
        let mut end_value = Text::from("calc(");
        end_value.push_text(&value);
        end_value.push_str(&format!(" * calc(1 - var({variable})))"));
        declarations.push((end.into(), end_value));
        let mut result = output(
            declarations,
            vec![
                property(variable, Some("0"), "*"),
                property("--tw-border-style", Some("solid"), "*"),
            ],
        );
        result.selector = Some(":where(& > :not(:last-child))");
        result.sort_property = Some(sort);
        result.declaration_count += 1;
        result
    } else if matches!(root, "outline-offset" | "underline-offset" | "opacity") {
        let value = generic_value(root, value, modifier, theme, used)?;
        let value = if negative {
            let mut output = Text::from("calc(");
            output.push_text(&value);
            output.push_str(" * -1)");
            output
        } else {
            value
        };
        output(
            vec![(
                if root == "underline-offset" {
                    "text-underline-offset"
                } else {
                    root
                }
                .into(),
                value,
            )],
            Vec::new(),
        )
    } else {
        return Ok(None);
    };
    for (_, value) in &result.declarations {
        mark_variables(value, used)?;
    }
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};
    use crate::program::{GpuCondition, GpuValue};

    #[test]
    fn divide_width_wrappers_preserve_non_scalar_units() {
        use super::{compile, CandidateValue, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let mut width = Text::from("var(--width-");
        width.push_text(&Text::from_units(vec![0xdc00]));
        width.push_str(")");
        let mut css = Text::from("@theme inline { --divide-width-brand: ");
        css.push_text(&width);
        css.push_str("; }");
        let mut nodes = crate::compiler::parser::parse_text(&css, 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let candidate = CandidateValue::Named {
            value: "brand".into(),
        };
        let mut used = BTreeSet::new();
        let result = compile("divide-x", Some(&candidate), None, &theme, &mut used)
            .unwrap()
            .unwrap();
        let mut expected = Text::from("calc(");
        expected.push_text(&width);
        expected.push_str(" * var(--tw-divide-x-reverse))");
        assert_eq!(result.declarations[2].1.units(), expected.units());
        let mut variable = Text::from("--width-");
        variable.push_text(&Text::from_units(vec![0xdc00]));
        assert!(used.contains(&variable));
    }

    #[test]
    fn border_branches_match_official_fixtures() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/borders-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = Compiler::new(
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
            )
            .unwrap();
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            let report = prepared.report();
            if report.css != case["expectedCss"].as_str().unwrap() || !report.unsupported.is_empty()
            {
                failures.push(format!(
                    "{}: actual={:?} expected={:?} pending={:?}",
                    case["id"].as_str().unwrap(),
                    report.css,
                    case["expectedCss"].as_str().unwrap(),
                    report.unsupported
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first cases: {:#?}",
            failures.len(),
            &failures[..failures.len().min(10)]
        );
    }

    #[test]
    fn widths_and_opacity_emit_programs_and_divide_keeps_its_relationship() {
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
                css_bytes: 262144,
                queued_bytes: 65536,
                cache_bytes: 1048576,
            },
        )
        .unwrap();
        let prepared = compiler
            .prepare_add(&["border-2".into(), "opacity-50".into(), "divide-x-2".into()])
            .unwrap();
        assert_eq!(
            prepared
                .report()
                .gpu_variables
                .get(&crate::text::Text::from("--tw-border-style"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("solid")
        );
        assert_eq!(
            prepared
                .report()
                .gpu_variables
                .get(&crate::text::Text::from("--tw-divide-x-reverse"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("0")
        );
        let programs = &prepared.report().gpu_programs;
        let border = programs
            .iter()
            .find(|program| program.candidate == "border-2")
            .unwrap();
        assert!(border
            .declarations
            .iter()
            .any(|declaration| declaration.property == "border-width"
                && declaration.value == GpuValue::Pixels { value: "2".into() }));
        let opacity = programs
            .iter()
            .find(|program| program.candidate == "opacity-50")
            .unwrap();
        assert!(opacity
            .declarations
            .iter()
            .any(|declaration| declaration.property == "opacity"));
        let divide = programs
            .iter()
            .find(|program| program.candidate == "divide-x-2")
            .unwrap();
        assert!(divide.conditions.iter().any(
            |condition| matches!(condition, GpuCondition::Unsupported { feature }
            if feature.contains(":where(& > :not(:last-child))"))
        ));
    }
}
