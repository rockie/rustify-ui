//! Native miscellaneous layout, list, cursor and containment utility branches.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, resolve_named},
    theme::Theme,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct MiscOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

fn output(nodes: Vec<AstNode>) -> MiscOutput {
    MiscOutput {
        declaration_count: nodes.len(),
        nodes,
        globals: Vec::new(),
    }
}

pub fn compile(
    root: &str,
    candidate_value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<MiscOutput>, super::utilities::FamilyFailure> {
    if root == "@container" {
        let value = match candidate_value {
            None => Text::from("inline-size"),
            Some(CandidateValue::Arbitrary { value, .. }) => value.clone(),
            Some(CandidateValue::Named { value }) if value == "normal" => Text::from("normal"),
            _ => return Err(DiagnosticKind::InvalidValue.into()),
        };
        let mut nodes = vec![AstNode::declaration("container-type", value)];
        if let Some(modifier) = modifier {
            let modifier = if let Some(value) = modifier
                .strip_prefix("[")
                .and_then(|value| value.strip_suffix("]"))
            {
                match super::value::decode_arbitrary_text(&value, usize::MAX) {
                    Ok(value) => value,
                    Err(super::CompileError::Syntax { .. }) => {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    Err(error) => return Err(error.into()),
                }
            } else if let Some(value) = modifier
                .strip_prefix("(")
                .and_then(|value| value.strip_suffix(")"))
            {
                concat_text(&[&"var(".into(), &value, &")".into()])
            } else {
                modifier.clone()
            };
            nodes.push(AstNode::declaration("container-name", modifier));
        }
        return Ok(Some(output(nodes)));
    }
    if root == "max-w-screen" {
        let Some(CandidateValue::Named { value }) = candidate_value else {
            return Err(DiagnosticKind::InvalidValue.into());
        };
        let value = resolve_named(value, &["breakpoint"], theme, used)?.ok_or(
            super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
        )?;
        return Ok(Some(output(vec![AstNode::declaration("max-width", value)])));
    }
    let (property, namespaces): (&str, &[&str]) = match root {
        "line-clamp" => ("-webkit-line-clamp", &["line-clamp"]),
        "list" => ("list-style-type", &["list-style-type"]),
        "list-image" => ("list-style-image", &["list-style-image"]),
        "indent" | "-indent" => ("text-indent", &["text-indent", "spacing"]),
        "align" => ("vertical-align", &[]),
        "z" | "-z" => ("z-index", &["z-index"]),
        "aspect" => ("aspect-ratio", &["aspect"]),
        "cursor" => ("cursor", &["cursor"]),
        "will-change" => ("will-change", &[]),
        "contain" => ("contain", &[]),
        _ => return Ok(None),
    };
    let negative = root.starts_with('-');
    let mut directly_negated = false;
    let mut value = match candidate_value {
        None => {
            if modifier.is_some() || matches!(root, "indent" | "-indent") {
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
            } else if root == "aspect" {
                let Some(rhs) = modifier else {
                    return Err(DiagnosticKind::InvalidValue.into());
                };
                if !canonical_nonnegative_text(value, true)
                    || !canonical_nonnegative_text(rhs, true)
                {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                concat_text(&[value, &"/".into(), rhs])
            } else {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                if matches!(root, "z" | "-z" | "line-clamp")
                    && canonical_nonnegative_text(value, true)
                {
                    value.clone()
                } else if matches!(root, "indent" | "-indent")
                    && canonical_nonnegative_text(value, false)
                {
                    let multiplier = theme.resolve_text(&Text::from("--spacing"), used).ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?;
                    directly_negated = negative;
                    concat_text(&[
                        &"calc(".into(),
                        &multiplier,
                        &Text::from(if negative { " * -" } else { " * " }),
                        value,
                        &")".into(),
                    ])
                } else {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
            }
        }
    };
    if negative && !directly_negated {
        value = concat_text(&[&"calc(".into(), &value, &" * -1)".into()]);
    }
    let mut nodes = if root == "line-clamp" {
        vec![
            AstNode::declaration("overflow", "hidden"),
            AstNode::declaration("display", "-webkit-box"),
            AstNode::declaration("-webkit-box-orient", "vertical"),
        ]
    } else {
        Vec::new()
    };
    nodes.push(AstNode::declaration(property, value));
    Ok(Some(output(nodes)))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{ast::AstNode, candidate, Compiler, CompilerInput, Limits};

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
    fn miscellaneous_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/miscellaneous-fixtures.json"
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
    fn container_modifier_and_spacing_ir_keep_source_semantics() {
        let mut compiler = compiler(
            "@theme { --spacing: .25rem; --breakpoint-card: 43rem; --container: 99px; --container-normal: 101px; } @tailwind utilities;",
        );
        let prepared = compiler
            .prepare_add(&[
                "@container-normal/[a_b]!".into(),
                "-indent-2".into(),
                "max-w-screen-card/ignored".into(),
                "line-clamp-4".into(),
            ])
            .unwrap();
        let declarations = |raw: &str| {
            let selector = super::concat_text(&[
                &".".into(),
                &candidate::escape_text(&crate::text::Text::from(raw)),
            ]);
            prepared
                .ast()
                .iter()
                .find_map(|node| match node {
                    AstNode::Rule {
                        selector: current,
                        nodes,
                        ..
                    } if *current == selector => Some(
                        nodes
                            .iter()
                            .filter_map(|node| match node {
                                AstNode::Declaration {
                                    property,
                                    value,
                                    important,
                                    ..
                                } => Some((property.clone(), value.clone(), *important)),
                                _ => None,
                            })
                            .collect::<Vec<_>>(),
                    ),
                    _ => None,
                })
                .unwrap()
        };
        assert_eq!(
            declarations("@container-normal/[a_b]!"),
            [
                (
                    crate::text::Text::from("container-type"),
                    crate::text::Text::from("normal"),
                    true
                ),
                (
                    crate::text::Text::from("container-name"),
                    crate::text::Text::from("a b"),
                    true
                )
            ]
        );
        assert_eq!(
            declarations("-indent-2"),
            [(
                crate::text::Text::from("text-indent"),
                crate::text::Text::from("calc(var(--spacing) * -2)"),
                false
            )]
        );
        assert_eq!(
            declarations("max-w-screen-card/ignored"),
            [(
                crate::text::Text::from("max-width"),
                crate::text::Text::from("var(--breakpoint-card)"),
                false
            )]
        );
        assert_eq!(
            declarations("line-clamp-4"),
            [
                (
                    crate::text::Text::from("overflow"),
                    crate::text::Text::from("hidden"),
                    false
                ),
                (
                    crate::text::Text::from("display"),
                    crate::text::Text::from("-webkit-box"),
                    false
                ),
                (
                    crate::text::Text::from("-webkit-box-orient"),
                    crate::text::Text::from("vertical"),
                    false
                ),
                (
                    crate::text::Text::from("-webkit-line-clamp"),
                    crate::text::Text::from("4"),
                    false
                )
            ]
        );
        assert!(prepared.report().unsupported.is_empty());
    }
}
