//! Native border spacing and sibling spacing declarations.

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

pub struct SpaceOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<SpaceOutput>, super::utilities::FamilyFailure> {
    let negative = root.starts_with('-');
    let unsigned = root.strip_prefix('-').unwrap_or(root);
    let reverse = matches!(root, "space-x-reverse" | "space-y-reverse");
    let px = unsigned.ends_with("-px");
    let base = if reverse {
        unsigned.strip_suffix("-reverse").unwrap_or(unsigned)
    } else if px {
        unsigned.strip_suffix("-px").unwrap_or(unsigned)
    } else {
        unsigned
    };
    if !matches!(
        base,
        "border-spacing" | "border-spacing-x" | "border-spacing-y" | "space-x" | "space-y"
    ) || negative && base.starts_with("border-")
    {
        return Ok(None);
    }
    let resolved = if reverse {
        "1".into()
    } else if px {
        if negative { "-1px" } else { "1px" }.into()
    } else {
        let resolved = match value {
            Some(CandidateValue::Arbitrary { value, .. }) if modifier.is_none() => value.clone(),
            Some(CandidateValue::Named { value }) => {
                let fraction = modifier
                    .filter(|modifier| !modifier.starts_with("[") && !modifier.starts_with("("))
                    .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
                let namespaces = if base.starts_with("border-") {
                    &["border-spacing", "spacing"][..]
                } else {
                    &["space", "spacing"][..]
                };
                if let Some(value) =
                    resolve_named(fraction.as_ref().unwrap_or(value), namespaces, theme, used)?
                {
                    value
                } else {
                    if modifier.is_some() || !canonical_nonnegative_text(value, false) {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    let multiplier = theme.resolve_text(&Text::from("--spacing"), used).ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?;
                    return Ok(Some(output(
                        base,
                        &concat_text(&[
                            &"calc(".into(),
                            &multiplier,
                            &Text::from(if negative { " * -" } else { " * " }),
                            value,
                            &")".into(),
                        ]),
                        false,
                    )));
                }
            }
            _ => return Err(DiagnosticKind::InvalidValue.into()),
        };
        if negative {
            concat_text(&[&"calc(".into(), &resolved, &" * -1)".into()])
        } else {
            resolved
        }
    };
    Ok(Some(output(base, &resolved, reverse)))
}

fn output(base: &str, value: &Text, reverse: bool) -> SpaceOutput {
    if base.starts_with("border-") {
        let mut nodes = Vec::new();
        if base != "border-spacing-y" {
            nodes.push(AstNode::declaration("--tw-border-spacing-x", value.clone()));
        }
        if base != "border-spacing-x" {
            nodes.push(AstNode::declaration("--tw-border-spacing-y", value.clone()));
        }
        nodes.push(AstNode::declaration(
            "border-spacing",
            "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
        ));
        SpaceOutput {
            declaration_count: nodes.len(),
            nodes,
            globals: vec![
                property("--tw-border-spacing-x", Some("0"), "<length>"),
                property("--tw-border-spacing-y", Some("0"), "<length>"),
            ],
        }
    } else {
        let (variable, axis) = if base == "space-x" {
            ("--tw-space-x-reverse", "inline")
        } else {
            ("--tw-space-y-reverse", "block")
        };
        let mut children = vec![AstNode::declaration(
            variable,
            if reverse { "1" } else { "0" },
        )];
        if !reverse {
            children.push(AstNode::declaration(
                format!("margin-{axis}-start"),
                concat_text(&[
                    &"calc(".into(),
                    value,
                    &Text::from(format!(" * var({variable}))")),
                ]),
            ));
            children.push(AstNode::declaration(
                format!("margin-{axis}-end"),
                concat_text(&[
                    &"calc(".into(),
                    value,
                    &Text::from(format!(" * calc(1 - var({variable})))")),
                ]),
            ));
        }
        SpaceOutput {
            // The source's virtual --tw-sort counts toward ordering but is not CSS.
            declaration_count: children.len() + 1,
            nodes: vec![AstNode::Rule {
                origin: None,
                source: None,
                destination: None,
                selector: ":where(& > :not(:last-child))".into(),
                nodes: children,
            }],
            globals: vec![property(variable, Some("0"), "*")],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::{ast::AstNode, Compiler, CompilerInput, Limits};

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
    fn structural_spacing_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/structural-spacing-fixtures.json"
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
    fn sibling_ast_keeps_important_local_and_resources_first_seen_without_virtual_sort_css() {
        let mut compiler = compiler("@theme { --spacing: .25rem; } @tailwind utilities;");
        let prepared = compiler
            .prepare_add(&[
                "space-x-2!".into(),
                "space-y-3".into(),
                "border-spacing-4!".into(),
            ])
            .unwrap();
        let properties = prepared
            .ast()
            .iter()
            .filter_map(|node| match node {
                AstNode::AtRule { name, params, .. } if name == "@property" => Some(params.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            properties,
            [
                "--tw-border-spacing-x",
                "--tw-border-spacing-y",
                "--tw-space-y-reverse",
                "--tw-space-x-reverse"
            ]
        );
        let children = prepared
            .ast()
            .iter()
            .find_map(|node| match node {
                AstNode::Rule {
                    selector, nodes, ..
                } if selector == ".space-x-2\\!" => nodes.first().and_then(|node| match node {
                    AstNode::Rule {
                        selector, nodes, ..
                    } if selector == ":where(& > :not(:last-child))" => Some(nodes),
                    _ => None,
                }),
                _ => None,
            })
            .unwrap();
        assert_eq!(children.len(), 3);
        assert!(children.iter().all(|node| matches!(
            node,
            AstNode::Declaration {
                important: true,
                ..
            }
        )));
        assert!(!prepared.report().css.contains("--tw-sort"));
        assert!(!prepared
            .report()
            .css
            .contains("initial-value: 0 !important"));
        assert!(prepared
            .report()
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "space-x-2!")
            .unwrap()
            .conditions
            .iter()
            .any(|condition| matches!(
                condition,
                crate::compiler::program::GpuCondition::Unsupported { .. }
            )));
    }
}
