//! Native transform families with ordered declarations and property resources.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, is_number_text, resolve_named},
    dependencies::property,
    theme::Theme,
    value::ValueNode,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct TransformOutput {
    pub declarations: Vec<(Text, Text)>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

const TRANSFORM_VALUE: &str = "var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,)";
const TRANSLATE_VALUE: &str = "var(--tw-translate-x) var(--tw-translate-y)";
const SCALE_VALUE: &str = "var(--tw-scale-x) var(--tw-scale-y)";

fn properties(kind: &str) -> Vec<AstNode> {
    let (names, initial): (&[&str], Option<&str>) = match kind {
        "translate" => (
            &["--tw-translate-x", "--tw-translate-y", "--tw-translate-z"],
            Some("0"),
        ),
        "scale" => (&["--tw-scale-x", "--tw-scale-y", "--tw-scale-z"], Some("1")),
        _ => (
            &[
                "--tw-rotate-x",
                "--tw-rotate-y",
                "--tw-rotate-z",
                "--tw-skew-x",
                "--tw-skew-y",
            ],
            None,
        ),
    };
    names
        .iter()
        .map(|name| property(name, initial, "*"))
        .collect()
}

fn output(declarations: Vec<(Text, Text)>, globals: Vec<AstNode>) -> TransformOutput {
    TransformOutput {
        declaration_count: declarations.len(),
        declarations,
        globals,
    }
}

fn negate(value: Text, negative: bool) -> Text {
    if negative {
        concat_text(&[&"calc(".into(), &value, &" * -1)".into()])
    } else {
        value
    }
}

fn generic_value(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    negative: bool,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Text, super::utilities::FamilyFailure> {
    let namespace = match root {
        "origin" => "transform-origin",
        "perspective-origin" => "perspective-origin",
        "perspective" => "perspective",
        root if root.starts_with("translate") => "translate",
        root if root.starts_with("scale") => "scale",
        root if root.starts_with("rotate") => "rotate",
        _ => "skew",
    };
    let translate = namespace == "translate";
    let mut already_negative = false;
    let value = match value {
        None => {
            if translate || modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            theme
                .resolve_text(&Text::from(format!("--{namespace}")), used)
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
            let namespaces: &[&str] = if translate {
                &["translate", "spacing"]
            } else {
                &[namespace]
            };
            if let Some(value) =
                resolve_named(fraction.as_ref().unwrap_or(value), namespaces, theme, used)?
            {
                value
            } else if let Some(fraction) = fraction.filter(|_| translate && root != "translate-z") {
                let Some((lhs, rhs)) = fraction.split_once("/") else {
                    return Err(DiagnosticKind::InvalidValue.into());
                };
                if !canonical_nonnegative_text(&lhs, true)
                    || !canonical_nonnegative_text(&rhs, true)
                {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                concat_text(&[&"calc(".into(), &fraction, &" * 100%)".into()])
            } else {
                if translate {
                    if !canonical_nonnegative_text(value, false) {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    let spacing = theme.resolve_text(&Text::from("--spacing"), used).ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?;
                    already_negative = negative;
                    let value = concat_text(&[
                        &"calc(".into(),
                        &spacing,
                        &Text::from(if negative { " * -" } else { " * " }),
                        value,
                        &")".into(),
                    ]);
                    if modifier.is_some() && !value.contains("/") {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    value
                } else {
                    if !matches!(namespace, "scale" | "rotate" | "skew")
                        || modifier.is_some()
                        || !canonical_nonnegative_text(value, true)
                    {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    concat_text(&[
                        value,
                        &Text::from(if namespace == "scale" { "%" } else { "deg" }),
                    ])
                }
            }
        }
    };
    Ok(negate(value, negative && !already_negative))
}

fn vector(value: &Text) -> bool {
    if value.starts_with(" ")
        || value.starts_with("\t")
        || value.ends_with(" ")
        || value.ends_with("\t")
    {
        return false;
    }
    let parts = value
        .units()
        .split(|unit| *unit == 32)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.contains(&40) && is_number_text(&Text::from_units(part.to_vec())))
}

fn simple_rotate_or_scale(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    negative: bool,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<TransformOutput, super::utilities::FamilyFailure> {
    if modifier.is_some() {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    let value = value.ok_or(super::utilities::FamilyFailure::Candidate(
        DiagnosticKind::InvalidValue,
    ))?;
    if let CandidateValue::Arbitrary { value, data_type } = value {
        let value = if root == "rotate"
            && data_type
                .as_ref()
                .map_or_else(|| vector(value), |hint| hint == "vector")
        {
            concat_text(&[value, &" var(--tw-rotate)".into()])
        } else {
            negate(value.clone(), negative)
        };
        return Ok(output(vec![(root.into(), value)], Vec::new()));
    }
    let CandidateValue::Named { value } = value else {
        return Err(DiagnosticKind::InvalidValue.into());
    };
    let value = resolve_named(value, &[root], theme, used)?
        .or_else(|| {
            canonical_nonnegative_text(value, true).then(|| {
                concat_text(&[
                    value,
                    &Text::from(if root == "rotate" { "deg" } else { "%" }),
                ])
            })
        })
        .ok_or(super::utilities::FamilyFailure::Candidate(
            DiagnosticKind::InvalidValue,
        ))?;
    let value = negate(value, negative);
    if root == "rotate" {
        return Ok(output(vec![("rotate".into(), value)], Vec::new()));
    }
    Ok(output(
        vec![
            ("--tw-scale-x".into(), value.clone()),
            ("--tw-scale-y".into(), value.clone()),
            ("--tw-scale-z".into(), value),
            ("scale".into(), SCALE_VALUE.into()),
        ],
        properties("scale"),
    ))
}

fn mark_variables(
    declarations: &[(Text, Text)],
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<(), super::utilities::FamilyFailure> {
    fn walk(nodes: &[ValueNode], used: &mut BTreeSet<crate::text::Text>) {
        for node in nodes {
            if let ValueNode::Function { name, nodes } = node {
                if name == "var" {
                    if let Some(ValueNode::Word { value }) = nodes.first() {
                        if value.starts_with("--") {
                            used.insert(value.clone());
                        }
                    }
                }
                walk(nodes, used);
            }
        }
    }
    for (_, value) in declarations {
        let nodes = super::value::parse_text(value, usize::MAX)?;
        walk(&nodes, used);
    }
    Ok(())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<TransformOutput>, super::utilities::FamilyFailure> {
    let (root, negative) = root
        .strip_prefix('-')
        .map_or((root, false), |root| (root, true));
    if !matches!(
        root,
        "origin"
            | "perspective-origin"
            | "perspective"
            | "translate"
            | "translate-x"
            | "translate-y"
            | "translate-z"
            | "scale"
            | "scale-x"
            | "scale-y"
            | "scale-z"
            | "rotate"
            | "rotate-x"
            | "rotate-y"
            | "rotate-z"
            | "skew"
            | "skew-x"
            | "skew-y"
            | "transform"
    ) || negative
        && matches!(
            root,
            "origin" | "perspective-origin" | "perspective" | "transform"
        )
    {
        return Ok(None);
    }
    let result = if matches!(root, "scale" | "rotate") {
        simple_rotate_or_scale(root, value, modifier, negative, theme, used)?
    } else if root == "transform" {
        if modifier.is_some() {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        let value = match value {
            None => TRANSFORM_VALUE.into(),
            Some(CandidateValue::Arbitrary { value, .. }) => value.clone(),
            _ => return Err(DiagnosticKind::InvalidValue.into()),
        };
        output(vec![("transform".into(), value)], properties("transform"))
    } else {
        let value = generic_value(root, value, modifier, negative, theme, used)?;
        match root {
            "origin" => output(vec![("transform-origin".into(), value)], Vec::new()),
            "perspective-origin" | "perspective" => output(vec![(root.into(), value)], Vec::new()),
            "translate" => output(
                vec![
                    ("--tw-translate-x".into(), value.clone()),
                    ("--tw-translate-y".into(), value),
                    ("translate".into(), TRANSLATE_VALUE.into()),
                ],
                properties("translate"),
            ),
            "translate-x" | "translate-y" | "translate-z" => output(
                vec![
                    (Text::from(format!("--tw-{root}")), value),
                    (
                        "translate".into(),
                        if root == "translate-z" {
                            Text::from(format!("{TRANSLATE_VALUE} var(--tw-translate-z)"))
                        } else {
                            TRANSLATE_VALUE.into()
                        },
                    ),
                ],
                properties("translate"),
            ),
            "scale-x" | "scale-y" | "scale-z" => output(
                vec![
                    (Text::from(format!("--tw-{root}")), value),
                    (
                        "scale".into(),
                        if root == "scale-z" {
                            Text::from(format!("{SCALE_VALUE} var(--tw-scale-z)"))
                        } else {
                            SCALE_VALUE.into()
                        },
                    ),
                ],
                properties("scale"),
            ),
            "rotate-x" | "rotate-y" | "rotate-z" => output(
                vec![
                    (
                        Text::from(format!("--tw-{root}")),
                        concat_text(&[
                            &Text::from(format!(
                                "rotate{}(",
                                root.chars()
                                    .last()
                                    .map(|axis| axis.to_ascii_uppercase())
                                    .unwrap_or('X')
                            )),
                            &value,
                            &")".into(),
                        ]),
                    ),
                    ("transform".into(), TRANSFORM_VALUE.into()),
                ],
                properties("transform"),
            ),
            "skew" => output(
                vec![
                    (
                        "--tw-skew-x".into(),
                        concat_text(&[&"skewX(".into(), &value, &")".into()]),
                    ),
                    (
                        "--tw-skew-y".into(),
                        concat_text(&[&"skewY(".into(), &value, &")".into()]),
                    ),
                    ("transform".into(), TRANSFORM_VALUE.into()),
                ],
                properties("transform"),
            ),
            _ => output(
                vec![
                    (
                        Text::from(format!("--tw-{root}")),
                        concat_text(&[
                            &Text::from(if root == "skew-x" { "skewX(" } else { "skewY(" }),
                            &value,
                            &")".into(),
                        ]),
                    ),
                    ("transform".into(), TRANSFORM_VALUE.into()),
                ],
                properties("transform"),
            ),
        }
    };
    mark_variables(&result.declarations, used)?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};
    use crate::program::GpuValue;

    #[test]
    fn generated_transform_variables_track_original_non_scalar_names() {
        let mut value = crate::text::Text::from("var(--custom-");
        value.push_text(&crate::text::Text::from_units(vec![0xd800]));
        value.push_str(")");
        let mut used = std::collections::BTreeSet::new();
        super::mark_variables(&[("transform".into(), value)], &mut used).unwrap();
        let mut expected = crate::text::Text::from("--custom-");
        expected.push_text(&crate::text::Text::from_units(vec![0xd800]));
        assert_eq!(used.into_iter().collect::<Vec<_>>(), vec![expected]);
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn transform_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/transforms-fixtures.json"
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
            "{} mismatches; ids={:?}; first cases: {:#?}",
            failures.len(),
            failures
                .iter()
                .map(|failure| failure.split(": actual=").next().unwrap())
                .collect::<Vec<_>>(),
            &failures[..failures.len().min(10)]
        );
    }

    #[test]
    fn vector_rotation_and_arbitrary_scale_keep_direct_gpu_values_without_extra_resources() {
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
            .prepare_add(&[
                "-rotate-[1_2_3]!".into(),
                "scale-[2_1.5_3]".into(),
                "translate-z-[13px]".into(),
                "origin-[13px_27px]".into(),
            ])
            .unwrap();
        let report = prepared.report();
        let rotation = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "-rotate-[1_2_3]!")
            .unwrap();
        assert_eq!(rotation.declarations.len(), 1);
        assert_eq!(rotation.declarations[0].property, "rotate");
        assert_eq!(
            rotation.declarations[0].value,
            GpuValue::CssExpression {
                value: "1 2 3 var(--tw-rotate)".into()
            }
        );
        assert!(rotation.declarations[0].important);
        let scale = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "scale-[2_1.5_3]")
            .unwrap();
        assert_eq!(scale.declarations.len(), 1);
        assert_eq!(scale.declarations[0].property, "scale");
        let origin = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "origin-[13px_27px]")
            .unwrap();
        assert_eq!(
            origin.declarations[0].value,
            GpuValue::CssExpression {
                value: "13px 27px".into()
            }
        );
        assert_eq!(
            report
                .gpu_variables
                .get(&crate::text::Text::from("--tw-translate-z"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("0")
        );
        assert!(!report
            .gpu_variables
            .contains_key(&crate::text::Text::from("--tw-scale-x")));
        assert!(!report
            .gpu_variables
            .contains_key(&crate::text::Text::from("--tw-rotate-x")));
    }
}
