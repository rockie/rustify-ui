//! Native flex, ordering, columns and object-position declarations.

use super::value::concat_text;
use super::{
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, resolve_named},
    theme::Theme,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct LayoutOutput {
    pub declarations: Vec<(Text, Text)>,
    pub declaration_count: usize,
}

fn output(property: &str, value: Text) -> LayoutOutput {
    LayoutOutput {
        declarations: vec![(property.into(), value)],
        declaration_count: 1,
    }
}

fn integer_value(
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    default: bool,
    fractions: bool,
) -> Result<Text, super::utilities::FamilyFailure> {
    match value {
        None if default && modifier.is_none() => Ok("1".into()),
        Some(CandidateValue::Arbitrary { value, .. }) if modifier.is_none() => Ok(value.clone()),
        Some(CandidateValue::Named { value }) if canonical_nonnegative_text(value, true) => {
            match modifier {
                None => Ok(value.clone()),
                Some(rhs) if fractions && canonical_nonnegative_text(rhs, true) => {
                    Ok(concat_text(&[
                        &"calc(".into(),
                        value,
                        &"/".into(),
                        rhs,
                        &" * 100%)".into(),
                    ]))
                }
                _ => Err(DiagnosticKind::InvalidValue.into()),
            }
        }
        _ => Err(DiagnosticKind::InvalidValue.into()),
    }
}

pub fn compile(
    root: &str,
    candidate_value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<LayoutOutput>, super::utilities::FamilyFailure> {
    match root {
        "flex" => {
            return integer_value(candidate_value, modifier, false, true)
                .map(|value| Some(output("flex", value)));
        }
        "grow" | "flex-grow" => {
            return integer_value(candidate_value, modifier, true, false)
                .map(|value| Some(output("flex-grow", value)));
        }
        "shrink" | "flex-shrink" => {
            return integer_value(candidate_value, modifier, true, false)
                .map(|value| Some(output("flex-shrink", value)));
        }
        _ => {}
    }
    let (property, namespaces): (&str, &[&str]) = match root {
        "order" | "-order" => ("order", &["order"]),
        "columns" => ("columns", &["columns", "container"]),
        "object" => ("object-position", &["object-position"]),
        _ => return Ok(None),
    };
    let mut value = match candidate_value {
        None => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            {
                let mut resolved = None;
                for namespace in namespaces {
                    resolved = theme.resolve_text(&Text::from(format!("--{namespace}")), used);
                    if resolved.is_some() {
                        break;
                    }
                }
                resolved
            }
            .ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))?
        }
        Some(CandidateValue::Arbitrary { value, .. }) => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            value.clone()
        }
        Some(CandidateValue::Named { value }) => {
            let fraction = modifier
                .filter(|modifier| !modifier.starts_with("[") && !modifier.starts_with("("))
                .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
            if let Some(value) =
                resolve_named(fraction.as_ref().unwrap_or(value), namespaces, theme, used)?
            {
                value
            } else {
                if root == "object"
                    || modifier.is_some()
                    || !canonical_nonnegative_text(value, true)
                {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                value.clone()
            }
        }
    };
    if root == "-order" {
        value = concat_text(&[&"calc(".into(), &value, &" * -1)".into()]);
    }
    Ok(Some(output(property, value)))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};

    fn compiler(css: &str) -> Compiler {
        Compiler::new(
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
                cache_bytes: 1048576,
            },
        )
        .unwrap()
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn layout_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/layout-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = compiler(case["input"].as_str().unwrap());
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| crate::text::Text::from(value.as_str().unwrap()))
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add_text(&candidates).unwrap();
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
    fn columns_precedence_flex_theme_independence_and_legacy_ir_are_preserved() {
        let mut compiler = compiler(
            "@theme { --columns-card: 13rem; --container-card: 17rem; --grow: 19; --flex-1: 23; } @tailwind utilities;",
        );
        let prepared = compiler
            .prepare_add(&[
                "columns-card".into(),
                "grow".into(),
                "flex-1".into(),
                "flex-grow-[2]!".into(),
                "flex-shrink-[3]".into(),
                "-order-7".into(),
            ])
            .unwrap();
        let report = prepared.report();
        assert!(report.css.contains("columns: var(--columns-card);"));
        assert!(!report.css.contains("--container-card:"));
        assert!(report.css.contains("flex-grow: 1;"));
        assert!(report.css.contains("flex: 1;"));
        assert!(!report.css.contains("--grow:"));
        assert!(!report.css.contains("--flex-1:"));
        assert!(!report.css.contains("@property"));
        let program = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "flex-grow-[2]!")
            .unwrap();
        assert_eq!(program.declarations[0].property, "flex-grow");
        assert!(program.declarations[0].important);
    }
}
