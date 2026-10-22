//! Native gradient constructors and stops with ordered structural AST output.

use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{
        as_color, canonical_nonnegative_text, inferred_type_text, is_number_text, resolve_named,
    },
    dependencies::property,
    theme::Theme,
    value::concat_text,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct GradientOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub sort_property: Option<&'static str>,
    pub declaration_count: usize,
}

fn output(nodes: Vec<AstNode>, declaration_count: usize) -> GradientOutput {
    GradientOutput {
        nodes,
        globals: Vec::new(),
        sort_property: None,
        declaration_count,
    }
}

fn interpolation(modifier: Option<&Text>) -> Result<Text, super::utilities::FamilyFailure> {
    let Some(modifier) = modifier else {
        return Ok("in oklab".into());
    };
    if let Some(value) = modifier
        .strip_prefix("[")
        .and_then(|value| value.strip_suffix("]"))
    {
        return super::value::decode_arbitrary_text(&value, usize::MAX).map_err(
            |error| match error {
                super::CompileError::Syntax { .. } => DiagnosticKind::InvalidValue.into(),
                error => super::utilities::FamilyFailure::Resource(error),
            },
        );
    }
    if let Some(value) = modifier
        .strip_prefix("(")
        .and_then(|value| value.strip_suffix(")"))
    {
        return Ok(concat_text(&[&"var(".into(), &value, &")".into()]));
    }
    Ok(
        if ["longer", "shorter", "increasing", "decreasing"]
            .iter()
            .any(|name| modifier == *name)
        {
            concat_text(&[&"in oklch ".into(), modifier, &" hue".into()])
        } else {
            concat_text(&[&"in ".into(), modifier])
        },
    )
}

fn angle(value: &Text) -> bool {
    ["deg", "rad", "grad", "turn"].iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| !number.contains("(") && is_number_text(&number))
    })
}

fn direction(value: &Text) -> Option<&'static str> {
    Some(match value {
        value if value == "to-t" => "to top",
        value if value == "to-tr" => "to top right",
        value if value == "to-r" => "to right",
        value if value == "to-br" => "to bottom right",
        value if value == "to-b" => "to bottom",
        value if value == "to-bl" => "to bottom left",
        value if value == "to-l" => "to left",
        value if value == "to-tl" => "to top left",
        _ => return None,
    })
}

fn stop_properties() -> Vec<AstNode> {
    [
        ("--tw-gradient-position", None, "*"),
        ("--tw-gradient-from", Some("#0000"), "<color>"),
        ("--tw-gradient-via", Some("#0000"), "<color>"),
        ("--tw-gradient-to", Some("#0000"), "<color>"),
        ("--tw-gradient-stops", None, "*"),
        ("--tw-gradient-via-stops", None, "*"),
        (
            "--tw-gradient-from-position",
            Some("0%"),
            "<length-percentage>",
        ),
        (
            "--tw-gradient-via-position",
            Some("50%"),
            "<length-percentage>",
        ),
        (
            "--tw-gradient-to-position",
            Some("100%"),
            "<length-percentage>",
        ),
    ]
    .into_iter()
    .map(|(name, initial, syntax)| property(name, initial, syntax))
    .collect()
}

fn stop(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<GradientOutput, super::utilities::FamilyFailure> {
    let value = value.ok_or(super::utilities::FamilyFailure::Candidate(
        DiagnosticKind::InvalidValue,
    ))?;
    let (value, position) = match value {
        CandidateValue::Arbitrary { value, data_type } => {
            if matches!(
                data_type
                    .as_ref()
                    .map(super::colors::known_type)
                    .or_else(|| inferred_type_text("decoration", value)),
                Some("length" | "percentage")
            ) {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                (value.clone(), true)
            } else {
                (
                    as_color(value.clone(), modifier, theme, used)?.ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?,
                    false,
                )
            }
        }
        CandidateValue::Named { value } => {
            let color = match value {
                value if value == "current" => Some("currentcolor".into()),
                value if value == "inherit" || value == "transparent" => Some(value.clone()),
                _ => resolve_named(value, &["background-color", "color"], theme, used)?,
            }
            .filter(|value| !value.is_empty())
            .map(|value| as_color(value, modifier, theme, used))
            .transpose()?
            .flatten();
            if let Some(color) = color {
                (color, false)
            } else {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                let position =
                    resolve_named(value, &["gradient-color-stop-positions"], theme, used)?
                        .or_else(|| {
                            value
                                .strip_suffix("%")
                                .filter(|number| canonical_nonnegative_text(number, true))
                                .map(|_| value.clone())
                        })
                        .ok_or(super::utilities::FamilyFailure::Candidate(
                            DiagnosticKind::InvalidValue,
                        ))?;
                (position, true)
            }
        }
    };
    let mut result = if position {
        output(
            vec![AstNode::declaration(
                format!("--tw-gradient-{root}-position"),
                value,
            )],
            1,
        )
    } else {
        let mut nodes = vec![AstNode::declaration(format!("--tw-gradient-{root}"), value)];
        if root == "via" {
            nodes.push(AstNode::declaration("--tw-gradient-via-stops", "var(--tw-gradient-position), var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-via) var(--tw-gradient-via-position), var(--tw-gradient-to) var(--tw-gradient-to-position)"));
            nodes.push(AstNode::declaration(
                "--tw-gradient-stops",
                "var(--tw-gradient-via-stops)",
            ));
        } else {
            nodes.push(AstNode::declaration("--tw-gradient-stops", "var(--tw-gradient-via-stops, var(--tw-gradient-position), var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-to) var(--tw-gradient-to-position))"));
        }
        let count = nodes.len() + 1; // The source --tw-sort marker is counted, then omitted from CSS.
        let mut result = output(nodes, count);
        result.sort_property = Some(match root {
            "from" => "--tw-gradient-from",
            "via" => "--tw-gradient-via",
            _ => "--tw-gradient-to",
        });
        result
    };
    result.globals = stop_properties();
    Ok(result)
}

fn constructor(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    negative: bool,
) -> Result<GradientOutput, super::utilities::FamilyFailure> {
    if let Some(CandidateValue::Arbitrary { value, data_type }) = value {
        if modifier.is_some() {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        let is_angle = data_type
            .as_ref()
            .map(super::colors::known_type)
            .map_or_else(|| angle(value), |hint| hint == "angle");
        if root == "bg-linear" && negative && !is_angle {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        let value = if root == "bg-linear" && negative {
            concat_text(&[&"calc(".into(), value, &" * -1)".into()])
        } else {
            value.clone()
        };
        let function = match root {
            "bg-linear" => "linear-gradient",
            "bg-conic" => "conic-gradient",
            _ => "radial-gradient",
        };
        return Ok(output(
            vec![
                AstNode::declaration("--tw-gradient-position", value.clone()),
                AstNode::declaration(
                    "background-image",
                    concat_text(&[
                        &format!("{function}(var(--tw-gradient-stops,").into(),
                        &value,
                        &"))".into(),
                    ]),
                ),
            ],
            2,
        ));
    }
    let method = interpolation(modifier)?;
    if value.is_none() && matches!(root, "bg-conic" | "bg-radial") {
        let function = if root == "bg-conic" {
            "conic-gradient"
        } else {
            "radial-gradient"
        };
        return Ok(output(
            vec![
                AstNode::declaration("--tw-gradient-position", method),
                AstNode::declaration(
                    "background-image",
                    format!("{function}(var(--tw-gradient-stops))"),
                ),
            ],
            2,
        ));
    }
    let Some(CandidateValue::Named { value }) = value else {
        return Err(DiagnosticKind::InvalidValue.into());
    };
    if root == "bg-radial" {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    let value = if root == "bg-linear" && !negative && direction(value).is_some() {
        direction(value).unwrap_or_default().into()
    } else if canonical_nonnegative_text(value, true) {
        if negative {
            concat_text(&[&"calc(".into(), value, &"deg * -1)".into()])
        } else {
            concat_text(&[value, &"deg".into()])
        }
    } else {
        return Err(DiagnosticKind::InvalidValue.into());
    };
    if root == "bg-conic" {
        return Ok(output(
            vec![
                AstNode::declaration(
                    "--tw-gradient-position",
                    concat_text(&[&"from ".into(), &value, &" ".into(), &method]),
                ),
                AstNode::declaration(
                    "background-image",
                    "conic-gradient(var(--tw-gradient-stops))",
                ),
            ],
            2,
        ));
    }
    Ok(output(
        vec![
            AstNode::declaration("--tw-gradient-position", value.clone()),
            AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@supports".into(),
                params: "(background-image: linear-gradient(in lab, red, red))".into(),
                nodes: Some(vec![AstNode::declaration(
                    "--tw-gradient-position",
                    concat_text(&[&value, &" ".into(), &method]),
                )]),
            },
            AstNode::declaration(
                "background-image",
                "linear-gradient(var(--tw-gradient-stops))",
            ),
        ],
        3,
    ))
}

fn mark_variables(
    nodes: &[AstNode],
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<(), super::utilities::FamilyFailure> {
    fn walk(nodes: &[super::value::ValueNode], used: &mut BTreeSet<crate::text::Text>) {
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
    for node in nodes {
        if let AstNode::Declaration { value, .. } = node {
            let nodes = super::value::parse_text(value, usize::MAX)?;
            walk(&nodes, used);
        }
        if let Some(children) = node.children() {
            mark_variables(children, used)?;
        }
    }
    Ok(())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<GradientOutput>, super::utilities::FamilyFailure> {
    let negative = root.starts_with('-');
    let root = root.trim_start_matches('-');
    let result = if !negative && matches!(root, "from" | "via" | "to") {
        stop(root, value, modifier, theme, used)?
    } else if matches!(root, "bg-linear" | "bg-conic") || root == "bg-radial" && !negative {
        constructor(root, value, modifier, negative)?
    } else {
        return Ok(None);
    };
    mark_variables(&result.nodes, used)?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{ast::AstNode, Compiler, CompilerInput, Limits};
    use crate::program::GpuCondition;

    #[test]
    #[cfg(rustify_repo_tests)]
    fn gradient_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/gradients-fixtures.json"
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

    #[test]
    fn linear_interpolation_keeps_nested_ast_and_rejects_flat_gpu_projection() {
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
            .prepare_add(&["bg-linear-to-r/longer!".into(), "from-[red]".into()])
            .unwrap();
        let nodes = prepared
            .ast()
            .iter()
            .find_map(|node| match node {
                AstNode::Rule {
                    selector, nodes, ..
                } if selector.starts_with(".bg-linear-to-r") => Some(nodes),
                _ => None,
            })
            .unwrap();
        assert!(
            matches!(&nodes[0], AstNode::Declaration { property, value, important: true, .. }
            if property == "--tw-gradient-position" && value == "to right")
        );
        assert!(
            matches!(&nodes[1], AstNode::AtRule { name, nodes: Some(children), .. }
            if name == "@supports" && matches!(&children[0], AstNode::Declaration { property, value, important: true, .. }
                if property == "--tw-gradient-position" && value == "to right in oklch longer hue"))
        );
        let linear = prepared
            .report()
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "bg-linear-to-r/longer!")
            .unwrap();
        assert_eq!(
            linear
                .declarations
                .iter()
                .filter(|declaration| declaration.property == "--tw-gradient-position")
                .count(),
            1
        );
        assert!(linear
            .conditions
            .iter()
            .any(|condition| matches!(condition, GpuCondition::Unsupported { .. })));
        assert_eq!(
            prepared
                .report()
                .gpu_variables
                .get(&crate::text::Text::from("--tw-gradient-from"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("#0000")
        );
    }
}
