//! Native spacingUtility families; CSS and GPU consume these declarations.

use super::value::concat_text;
use super::{candidate::CandidateValue, theme::Theme, DiagnosticKind};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct FunctionalOutput {
    pub declarations: Vec<(Text, Text)>,
    pub sort_property: Option<&'static str>,
    pub declaration_count: usize,
}

struct Family {
    properties: &'static [&'static str],
    namespaces: &'static [&'static str],
    negative: bool,
    fractions: bool,
}

fn family(root: &str) -> Option<Family> {
    let (properties, namespaces, negative, fractions): (
        &'static [&'static str],
        &'static [&'static str],
        bool,
        bool,
    ) = match root {
        "inset" => (&["inset"], &["inset", "spacing"], true, true),
        "inset-x" => (&["inset-inline"], &["inset", "spacing"], true, true),
        "inset-y" => (&["inset-block"], &["inset", "spacing"], true, true),
        "start" => (&["inset-inline-start"], &["inset", "spacing"], true, true),
        "end" => (&["inset-inline-end"], &["inset", "spacing"], true, true),
        "top" => (&["top"], &["inset", "spacing"], true, true),
        "right" => (&["right"], &["inset", "spacing"], true, true),
        "bottom" => (&["bottom"], &["inset", "spacing"], true, true),
        "left" => (&["left"], &["inset", "spacing"], true, true),
        "m" => (&["margin"], &["margin", "spacing"], true, false),
        "mx" => (&["margin-inline"], &["margin", "spacing"], true, false),
        "my" => (&["margin-block"], &["margin", "spacing"], true, false),
        "ms" => (
            &["margin-inline-start"],
            &["margin", "spacing"],
            true,
            false,
        ),
        "me" => (&["margin-inline-end"], &["margin", "spacing"], true, false),
        "mt" => (&["margin-top"], &["margin", "spacing"], true, false),
        "mr" => (&["margin-right"], &["margin", "spacing"], true, false),
        "mb" => (&["margin-bottom"], &["margin", "spacing"], true, false),
        "ml" => (&["margin-left"], &["margin", "spacing"], true, false),
        "p" => (&["padding"], &["padding", "spacing"], false, false),
        "px" => (&["padding-inline"], &["padding", "spacing"], false, false),
        "py" => (&["padding-block"], &["padding", "spacing"], false, false),
        "ps" => (
            &["padding-inline-start"],
            &["padding", "spacing"],
            false,
            false,
        ),
        "pe" => (
            &["padding-inline-end"],
            &["padding", "spacing"],
            false,
            false,
        ),
        "pt" => (&["padding-top"], &["padding", "spacing"], false, false),
        "pr" => (&["padding-right"], &["padding", "spacing"], false, false),
        "pb" => (&["padding-bottom"], &["padding", "spacing"], false, false),
        "pl" => (&["padding-left"], &["padding", "spacing"], false, false),
        "scroll-m" => (
            &["scroll-margin"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-mx" => (
            &["scroll-margin-inline"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-my" => (
            &["scroll-margin-block"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-ms" => (
            &["scroll-margin-inline-start"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-me" => (
            &["scroll-margin-inline-end"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-mt" => (
            &["scroll-margin-top"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-mr" => (
            &["scroll-margin-right"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-mb" => (
            &["scroll-margin-bottom"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-ml" => (
            &["scroll-margin-left"],
            &["scroll-margin", "spacing"],
            true,
            false,
        ),
        "scroll-p" => (
            &["scroll-padding"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-px" => (
            &["scroll-padding-inline"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-py" => (
            &["scroll-padding-block"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-ps" => (
            &["scroll-padding-inline-start"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-pe" => (
            &["scroll-padding-inline-end"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-pt" => (
            &["scroll-padding-top"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-pr" => (
            &["scroll-padding-right"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-pb" => (
            &["scroll-padding-bottom"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "scroll-pl" => (
            &["scroll-padding-left"],
            &["scroll-padding", "spacing"],
            false,
            false,
        ),
        "size" => (&["width", "height"], &["size", "spacing"], false, true),
        "w" => (&["width"], &["width", "spacing", "container"], false, true),
        "min-w" => (
            &["min-width"],
            &["min-width", "spacing", "container"],
            false,
            true,
        ),
        "max-w" => (
            &["max-width"],
            &["max-width", "spacing", "container"],
            false,
            true,
        ),
        "h" => (&["height"], &["height", "spacing"], false, true),
        "min-h" => (
            &["min-height"],
            &["min-height", "height", "spacing"],
            false,
            true,
        ),
        "max-h" => (
            &["max-height"],
            &["max-height", "height", "spacing"],
            false,
            true,
        ),
        "basis" => (
            &["flex-basis"],
            &["flex-basis", "spacing", "container"],
            false,
            true,
        ),
        "gap" => (&["gap"], &["gap", "spacing"], false, false),
        "gap-x" => (&["column-gap"], &["gap", "spacing"], false, false),
        "gap-y" => (&["row-gap"], &["gap", "spacing"], false, false),
        _ => return None,
    };
    Some(Family {
        properties,
        namespaces,
        negative,
        fractions,
    })
}

// Upstream Number/string round-trips reject leading/trailing zeroes and require
// ECMAScript's scientific notation at 1e21. Multipliers cannot be below 0.25,
// so the small-number scientific notation branch is irrelevant here.
fn canonical_nonnegative(value: &str, integer: bool) -> bool {
    let Ok(number) = value.parse::<f64>() else {
        return false;
    };
    if !number.is_finite()
        || number < 0.0
        || if integer {
            number.fract() != 0.0
        } else {
            number % 0.25 != 0.0
        }
    {
        return false;
    }
    let ordinary = if number == 0.0 {
        "0".into()
    } else {
        number.to_string()
    };
    if number < 1e21 {
        return ordinary == value;
    }
    let significant = ordinary.trim_end_matches('0');
    let mut chars = significant.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let rest = chars.as_str();
    let mantissa = if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first}.{rest}")
    };
    format!("{mantissa}e+{}", ordinary.len() - 1) == value
}

fn canonical_nonnegative_text(value: &Text, integer: bool) -> bool {
    if !value
        .units()
        .iter()
        .all(|unit| matches!(unit, 43 | 45..=57 | 69 | 101))
    {
        return false;
    }
    let scalar: String = value
        .units()
        .iter()
        .map(|unit| char::from(*unit as u8))
        .collect();
    canonical_nonnegative(&scalar, integer)
}

fn resolve_named(
    value: &Text,
    namespaces: &[&str],
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    for namespace in namespaces {
        // --inset-shadow and --inset-ring are independent namespaces.
        if *namespace == "inset"
            && ["shadow", "ring"].iter().any(|prefix| {
                value == prefix
                    || value
                        .strip_prefix(prefix)
                        .is_some_and(|rest| rest.starts_with("-"))
            })
        {
            continue;
        }
        for key in [
            concat_text(&[&Text::from(format!("--{namespace}-")), value]),
            concat_text(&[
                &Text::from(format!("--{namespace}-")),
                &value.replace_ascii(b'.', "_"),
            ]),
        ] {
            if let Some(css) = theme.resolve_text(&key, used) {
                return Ok(Some(css));
            }
            // Theme extraction currently retains escaped property spellings;
            // token use and emitted var() must retain that same spelling.
            let escaped = super::candidate::escape_text(&key);
            if escaped != key {
                if let Some(css) = theme.resolve_text(&escaped, used) {
                    return Ok(Some(css));
                }
            }
        }
    }
    Ok(None)
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<FunctionalOutput>, super::utilities::FamilyFailure> {
    let (root, negative) = root
        .strip_prefix('-')
        .map_or((root, false), |root| (root, true));
    let Some(family) = family(root).filter(|family| !negative || family.negative) else {
        return Ok(None);
    };
    let Some(value) = value else {
        return Err(DiagnosticKind::InvalidValue.into());
    };
    let mut already_negative = false;
    let css = match value {
        CandidateValue::Arbitrary { value, .. } => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            value.clone()
        }
        CandidateValue::Named { value } => {
            // The slash denotes a fraction only for named modifiers, but a
            // resolved theme value accepts an arbitrary modifier upstream.
            let fraction = modifier
                .filter(|modifier| !modifier.starts_with("[") && !modifier.starts_with("("))
                .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
            if let Some(css) = resolve_named(
                fraction.as_ref().unwrap_or(value),
                family.namespaces,
                theme,
                used,
            )? {
                css
            } else if let Some(fraction) = fraction.filter(|_| family.fractions) {
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
                if !canonical_nonnegative_text(value, false) {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                let Some(spacing) = theme.resolve_text(&Text::from("--spacing"), used) else {
                    return Err(DiagnosticKind::InvalidValue.into());
                };
                already_negative = negative;
                let css = if negative {
                    concat_text(&[
                        &"calc(".into(),
                        &spacing,
                        &" * -".into(),
                        value,
                        &")".into(),
                    ])
                } else {
                    concat_text(&[&"calc(".into(), &spacing, &" * ".into(), value, &")".into()])
                };
                if modifier.is_some() && !css.contains("/") {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                css
            }
        }
    };
    let css = if negative && !already_negative {
        concat_text(&[&"calc(".into(), &css, &" * -1)".into()])
    } else {
        css
    };
    let sort_property = (root == "size").then_some("size");
    Ok(Some(FunctionalOutput {
        declarations: family
            .properties
            .iter()
            .map(|property| ((*property).into(), css.clone()))
            .collect(),
        sort_property,
        declaration_count: family.properties.len() + usize::from(sort_property.is_some()),
    }))
}

#[cfg(test)]
mod tests {
    #[cfg(rustify_repo_tests)]
    use crate::compiler::{Compiler, CompilerInput, GpuValue, Limits};

    #[test]
    fn inline_theme_values_remain_original_utf16_across_layout_families() {
        use crate::{
            compiler::{ast::AstNode, candidate::CandidateValue, theme::Theme},
            text::Text,
        };
        use std::collections::BTreeSet;
        let value = Text::from_units(vec![0xd800, 0xfffd, 92, 117, 100, 56, 48, 48]);
        let mut nodes = vec![AstNode::AtRule {
            origin: None,
            name: "@theme".into(),
            params: "inline".into(),
            source: None,
            destination: None,
            nodes: Some(
                [
                    "--spacing-card",
                    "--grid-column-card",
                    "--order-card",
                    "--transition-duration-card",
                    "--skew-card",
                    "--space-card",
                    "--z-index-card",
                ]
                .into_iter()
                .map(|name| AstNode::declaration(name, value.clone()))
                .collect(),
            ),
        }];
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let candidate = Some(CandidateValue::Named {
            value: "card".into(),
        });
        let mut used = BTreeSet::new();
        let outputs = [
            super::compile("p", candidate.as_ref(), None, &theme, &mut used)
                .unwrap()
                .unwrap()
                .declarations[0]
                .1
                .clone(),
            crate::compiler::grid::compile("col", candidate.as_ref(), None, &theme, &mut used)
                .unwrap()
                .unwrap()
                .declarations[0]
                .1
                .clone(),
            crate::compiler::layout::compile("order", candidate.as_ref(), None, &theme, &mut used)
                .unwrap()
                .unwrap()
                .declarations[0]
                .1
                .clone(),
            crate::compiler::motion::compile(
                "duration",
                candidate.as_ref(),
                None,
                &theme,
                &mut used,
            )
            .unwrap()
            .unwrap()
            .declarations[0]
                .1
                .clone(),
        ];
        assert!(outputs.iter().all(|output| output == &value));
        let transform = crate::compiler::transforms::compile(
            "-skew-x",
            candidate.as_ref(),
            None,
            &theme,
            &mut used,
        )
        .unwrap()
        .unwrap();
        let mut expected = Text::from("skewX(calc(");
        expected.push_text(&value);
        expected.push_str(" * -1))");
        assert_eq!(transform.declarations[0].1, expected);
        let space = crate::compiler::structural_spacing::compile(
            "space-x",
            candidate.as_ref(),
            None,
            &theme,
            &mut used,
        )
        .unwrap()
        .unwrap();
        let AstNode::Rule { nodes, .. } = &space.nodes[0] else {
            panic!("expected sibling rule")
        };
        let AstNode::Declaration { value: actual, .. } = &nodes[1] else {
            panic!("expected spacing declaration")
        };
        let mut expected = Text::from("calc(");
        expected.push_text(&value);
        expected.push_str(" * var(--tw-space-x-reverse))");
        assert_eq!(actual, &expected);
        let miscellaneous = crate::compiler::miscellaneous::compile(
            "z",
            candidate.as_ref(),
            None,
            &theme,
            &mut used,
        )
        .unwrap()
        .unwrap();
        let AstNode::Declaration { value: actual, .. } = &miscellaneous.nodes[0] else {
            panic!("expected z-index declaration")
        };
        assert_eq!(actual, &value);
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn spacing_layout_functional_families_match_official_boundary_fixtures() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/functional-spacing-fixtures.json"
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
                .map(|value| crate::text::Text::from(value.as_str().unwrap()))
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add_text(&candidates).unwrap();
            if case["id"] == "functional:size:important" {
                let programs = &prepared.report().gpu_programs;
                assert_eq!(programs.len(), 1);
                assert_eq!(programs[0].declarations.len(), 2);
                for (declaration, property) in
                    programs[0].declarations.iter().zip(["width", "height"])
                {
                    assert_eq!(declaration.property, property);
                    assert_eq!(declaration.value, GpuValue::Pixels { value: "13".into() });
                    assert!(declaration.important);
                }
            }
            if prepared.report().css != case["expectedCss"].as_str().unwrap()
                || !prepared.report().unsupported.is_empty()
            {
                failures.push(format!(
                    "{}: actual={:?} expected={:?} pending={:?}",
                    case["id"].as_str().unwrap(),
                    prepared.report().css,
                    case["expectedCss"].as_str().unwrap(),
                    prepared.report().unsupported
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first cases: {:#?}; last cases: {:#?}",
            failures.len(),
            &failures[..failures.len().min(10)],
            &failures[failures.len().saturating_sub(5)..]
        );
    }
}
