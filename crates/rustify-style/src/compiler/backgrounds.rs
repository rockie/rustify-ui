//! Native background image/position/size branches; gradient roots are separate.

use super::{candidate::CandidateValue, theme::Theme, DiagnosticKind};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct BackgroundOutput {
    pub declarations: Vec<(Text, Text)>,
    pub declaration_count: usize,
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
) -> Result<Option<BackgroundOutput>, super::utilities::FamilyFailure> {
    let (property, css) = match root {
        "bg-size" | "bg-position" => {
            // These explicit roots have no theme keys or bare-value handler.
            // The shared functionalUtility passes through every arbitrary hint.
            let Some(CandidateValue::Arbitrary { value, .. }) = value else {
                return Err(DiagnosticKind::InvalidValue.into());
            };
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            (
                if root == "bg-size" {
                    "background-size"
                } else {
                    "background-position"
                },
                value.clone(),
            )
        }
        "bg" => {
            let value = value.ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))?;
            // A successful color interpretation always precedes named images.
            // Probe without leaking dependencies into the competing branch.
            match super::colors::compile(root, Some(value), modifier, theme, &mut BTreeSet::new()) {
                Ok(Some(_)) => return Ok(None),
                Err(super::utilities::FamilyFailure::Candidate(DiagnosticKind::PendingFamily)) => {}
                Err(kind) => return Err(kind),
                Ok(None) => return Ok(None),
            }
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            match value {
                CandidateValue::Named { value } => (
                    "background-image",
                    super::colors::resolve_named(value, &["background-image"], theme, used)?
                        .ok_or(super::utilities::FamilyFailure::Candidate(
                            DiagnosticKind::InvalidValue,
                        ))?,
                ),
                CandidateValue::Arbitrary { value, data_type } => {
                    let property = match data_type
                        .as_ref()
                        .map(super::colors::known_type)
                        .or_else(|| super::colors::inferred_type_text(root, value))
                    {
                        Some("percentage" | "position") => "background-position",
                        Some("bg-size" | "length" | "size") => "background-size",
                        Some("image" | "url") => "background-image",
                        _ => return Err(DiagnosticKind::InvalidValue.into()),
                    };
                    (property, value.clone())
                }
            }
        }
        _ => return Ok(None),
    };
    mark_variables(&css, used)?;
    Ok(Some(BackgroundOutput {
        declarations: vec![(property.into(), css)],
        declaration_count: 1,
    }))
}

#[cfg(test)]
mod tests {
    #[cfg(rustify_repo_tests)]
    use crate::compiler::{Compiler, CompilerInput, Limits};

    #[test]
    fn inline_background_image_preserves_non_scalar_units() {
        use super::{compile, CandidateValue, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let mut image = Text::from("url('");
        image.push_text(&Text::from_units(vec![0xd800]));
        image.push_str("')");
        let mut css = Text::from("@theme inline { --background-image-brand: ");
        css.push_text(&image);
        css.push_str("; }");
        let mut nodes = crate::compiler::parser::parse_text(&css, 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let candidate = CandidateValue::Named {
            value: "brand".into(),
        };
        let result = compile("bg", Some(&candidate), None, &theme, &mut BTreeSet::new())
            .unwrap()
            .unwrap();
        assert_eq!(
            result.declarations,
            vec![(Text::from("background-image"), image)]
        );
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn background_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/backgrounds-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let compiler = Compiler::new(
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
            let mut compiler = compiler.unwrap();
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
}
