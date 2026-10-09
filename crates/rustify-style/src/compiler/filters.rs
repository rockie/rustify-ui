//! Native filter families with structural drop-shadow fallbacks.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{
        as_color, canonical_nonnegative_text, is_color_text, js_number_text, js_string,
        resolve_named,
    },
    dependencies::property,
    shadows::{join_text, replace_shadow_colors, segment},
    theme::Theme,
    value::ValueNode,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct FilterOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

const FILTER_VALUE: &str = "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)";
const BACKDROP_VALUE: &str = "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)";

fn properties(backdrop: bool) -> Vec<AstNode> {
    if backdrop {
        [
            "blur",
            "brightness",
            "contrast",
            "grayscale",
            "hue-rotate",
            "invert",
            "opacity",
            "saturate",
            "sepia",
        ]
        .iter()
        .map(|root| property(&format!("--tw-backdrop-{root}"), None, "*"))
        .collect()
    } else {
        [
            "blur",
            "brightness",
            "contrast",
            "grayscale",
            "hue-rotate",
            "invert",
            "opacity",
            "saturate",
            "sepia",
            "drop-shadow",
            "drop-shadow-color",
            "drop-shadow-alpha",
            "drop-shadow-size",
        ]
        .iter()
        .map(|root| {
            property(
                &format!("--tw-{root}"),
                (*root == "drop-shadow-alpha").then_some("100%"),
                if *root == "drop-shadow-alpha" {
                    "<percentage>"
                } else {
                    "*"
                },
            )
        })
        .collect()
    }
}

fn output(nodes: Vec<AstNode>, globals: Vec<AstNode>) -> FilterOutput {
    fn count(nodes: &[AstNode]) -> usize {
        nodes
            .iter()
            .map(|node| {
                usize::from(matches!(node, AstNode::Declaration { .. }))
                    + node.children().map_or(0, count)
            })
            .sum()
    }
    FilterOutput {
        declaration_count: count(&nodes),
        nodes,
        globals,
    }
}

fn filter_declarations(backdrop: bool, value: &Text) -> Vec<AstNode> {
    if backdrop {
        vec![
            AstNode::declaration("-webkit-backdrop-filter", value.clone()),
            AstNode::declaration("backdrop-filter", value.clone()),
        ]
    } else {
        vec![AstNode::declaration("filter", value.clone())]
    }
}

fn generic_value(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Text, super::utilities::FamilyFailure> {
    let effect = root.strip_prefix("backdrop-").unwrap_or(root);
    let namespaces: &[&str] = match root {
        "blur" => &["blur"],
        "backdrop-blur" => &["backdrop-blur", "blur"],
        "brightness" => &["brightness"],
        "backdrop-brightness" => &["backdrop-brightness", "brightness"],
        "contrast" => &["contrast"],
        "backdrop-contrast" => &["backdrop-contrast", "contrast"],
        "grayscale" => &["grayscale"],
        "backdrop-grayscale" => &["backdrop-grayscale", "grayscale"],
        "hue-rotate" => &["hue-rotate"],
        "backdrop-hue-rotate" => &["backdrop-hue-rotate", "hue-rotate"],
        "invert" => &["invert"],
        "backdrop-invert" => &["backdrop-invert", "invert"],
        "saturate" => &["saturate"],
        "backdrop-saturate" => &["backdrop-saturate", "saturate"],
        "sepia" => &["sepia"],
        "backdrop-sepia" => &["backdrop-sepia", "sepia"],
        _ => &["backdrop-opacity", "opacity"],
    };
    match value {
        None => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            if matches!(effect, "grayscale" | "invert" | "sepia") {
                return Ok("100%".into());
            }
            for namespace in namespaces {
                if let Some(value) =
                    theme.resolve_text(&Text::from(&format!("--{namespace}")), used)
                {
                    return Ok(value);
                }
            }
            Err(DiagnosticKind::InvalidValue.into())
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
            if effect == "blur"
                || !canonical_nonnegative_text(value, effect != "opacity")
                || modifier.is_some()
            {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            Ok(concat_text(&[
                value,
                &if effect == "hue-rotate" { "deg" } else { "%" }.into(),
            ]))
        }
    }
}

fn alpha_modifier(
    modifier: Option<&Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    let Some(modifier) = modifier else {
        return Ok(None);
    };
    if let Some(value) = modifier
        .strip_prefix("[")
        .and_then(|value| value.strip_suffix("]"))
    {
        return super::value::decode_arbitrary_text(&value, usize::MAX)
            .map(Some)
            .map_err(|error| match error {
                super::CompileError::Syntax { .. } => DiagnosticKind::InvalidValue.into(),
                error => super::utilities::FamilyFailure::Resource(error),
            });
    }
    if let Some(value) = modifier
        .strip_prefix("(")
        .and_then(|value| value.strip_suffix(")"))
    {
        return Ok(Some(concat_text(&[&"var(".into(), &value, &")".into()])));
    }
    Ok(canonical_nonnegative_text(modifier, true).then(|| concat_text(&[modifier, &"%".into()])))
}

fn alpha_percentage(alpha: &Text) -> Text {
    js_number_text(alpha).map_or_else(
        || alpha.clone(),
        |number| format!("{}%", js_string(number * 100.0)).into(),
    )
}

fn with_alpha(color: &Text, alpha: &Text) -> Text {
    let alpha = alpha_percentage(alpha);
    if alpha == "100%" {
        color.clone()
    } else {
        concat_text(&[
            &"color-mix(in oklab, ".into(),
            color,
            &" ".into(),
            &alpha,
            &", transparent)".into(),
        ])
    }
}

fn replaced_shadow_nodes(
    value: &Text,
    alpha: Option<&Text>,
    max_bytes: usize,
) -> Result<Vec<AstNode>, super::utilities::FamilyFailure> {
    let mut fallback = false;
    let mut shadows = Vec::new();
    let mut bytes: usize = 0;
    for shadow in segment(value, ',')? {
        let shadow = replace_shadow_colors(&shadow, max_bytes, |color| {
            let color = if let Some(alpha) = alpha {
                if color.starts_with("current") {
                    with_alpha(color, alpha)
                } else {
                    fallback |= color.starts_with("var(") || alpha.starts_with("var(");
                    concat_text(&[
                        &"oklab(from ".into(),
                        color,
                        &" l a b / ".into(),
                        &alpha_percentage(alpha),
                        &")".into(),
                    ])
                }
            } else {
                color.clone()
            };
            concat_text(&[&"var(--tw-drop-shadow-color, ".into(), &color, &")".into()])
        })?;
        bytes = bytes
            .saturating_add(shadow.byte_len())
            .saturating_add(13)
            .saturating_add(usize::from(!shadows.is_empty()));
        super::check_limit(bytes, max_bytes, "css_bytes")?;
        shadows.push(concat_text(&[&"drop-shadow(".into(), &shadow, &")".into()]));
    }
    let replaced = join_text(&shadows, " ");
    if fallback {
        let mut shadows = Vec::new();
        let mut bytes: usize = 0;
        for shadow in segment(value, ',')? {
            let shadow = replace_shadow_colors(&shadow, max_bytes, |color| {
                concat_text(&[&"var(--tw-drop-shadow-color, ".into(), color, &")".into()])
            })?;
            bytes = bytes
                .saturating_add(shadow.byte_len())
                .saturating_add(13)
                .saturating_add(usize::from(!shadows.is_empty()));
            super::check_limit(bytes, max_bytes, "css_bytes")?;
            shadows.push(concat_text(&[&"drop-shadow(".into(), &shadow, &")".into()]));
        }
        let shadows = join_text(&shadows, " ");
        Ok(vec![
            AstNode::declaration("--tw-drop-shadow-size", shadows),
            AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@supports".into(),
                params: "(color: lab(from red l a b))".into(),
                nodes: Some(vec![AstNode::declaration(
                    "--tw-drop-shadow-size",
                    replaced,
                )]),
            },
        ])
    } else {
        Ok(vec![AstNode::declaration(
            "--tw-drop-shadow-size",
            replaced,
        )])
    }
}

fn drop_shadow(
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
    max_bytes: usize,
) -> Result<FilterOutput, super::utilities::FamilyFailure> {
    let alpha = alpha_modifier(modifier)?;
    let (raw, resolved) = match value {
        None => {
            let raw = theme
                .raw_text(&Text::from("--drop-shadow"), used)
                .ok_or(DiagnosticKind::InvalidValue)?;
            let resolved = theme
                .resolve_text(&Text::from("--drop-shadow"), used)
                .ok_or(DiagnosticKind::InvalidValue)?;
            (raw, Some(resolved))
        }
        Some(CandidateValue::Arbitrary { value, data_type }) => {
            if data_type
                .as_ref()
                .map(super::colors::known_type)
                .map_or_else(|| is_color_text(value), |hint| hint == "color")
            {
                let color = as_color(value.clone(), modifier, theme, used)?
                    .ok_or(DiagnosticKind::InvalidValue)?;
                return Ok(output(
                    vec![
                        AstNode::declaration(
                            "--tw-drop-shadow-color",
                            with_alpha(&color, &"var(--tw-drop-shadow-alpha)".into()),
                        ),
                        AstNode::declaration("--tw-drop-shadow", "var(--tw-drop-shadow-size)"),
                    ],
                    properties(false),
                ));
            }
            if modifier.is_some() && alpha.is_none() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            (value.clone(), None)
        }
        Some(CandidateValue::Named { value }) => {
            let key = concat_text(&[&"--drop-shadow-".into(), value]);
            let raw = theme.raw_text(&key, used).filter(|value| !value.is_empty());
            // A size probe can lose to the color branch or be inlined for alpha.
            // Only var() references in the emitted nodes mark its token as used.
            let resolved = resolve_named(value, &["drop-shadow"], theme, &mut BTreeSet::new())?
                .filter(|value| !value.is_empty());
            if let Some((raw, resolved)) = raw.zip(resolved) {
                if modifier.is_some() && alpha.is_none() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                (raw, alpha.is_none().then_some(resolved))
            } else {
                let color = match value {
                    value if value == "current" => Some("currentcolor".into()),
                    value if value == "inherit" || value == "transparent" => Some(value.clone()),
                    _ => resolve_named(value, &["drop-shadow-color", "color"], theme, used)?,
                }
                .filter(|value| !value.is_empty())
                .ok_or(DiagnosticKind::InvalidValue)?;
                let color =
                    as_color(color, modifier, theme, used)?.ok_or(DiagnosticKind::InvalidValue)?;
                return Ok(output(
                    vec![
                        AstNode::declaration(
                            "--tw-drop-shadow-color",
                            if color == "inherit" {
                                color
                            } else {
                                with_alpha(&color, &"var(--tw-drop-shadow-alpha)".into())
                            },
                        ),
                        AstNode::declaration("--tw-drop-shadow", "var(--tw-drop-shadow-size)"),
                    ],
                    properties(false),
                ));
            }
        }
    };
    let mut nodes = Vec::new();
    if let Some(alpha) = &alpha {
        nodes.push(AstNode::declaration(
            "--tw-drop-shadow-alpha",
            alpha.clone(),
        ));
    }
    nodes.extend(replaced_shadow_nodes(&raw, alpha.as_ref(), max_bytes)?);
    let value = if let Some(resolved) = resolved {
        join_text(
            &segment(&resolved, ',')?
                .iter()
                .map(|value| concat_text(&[&"drop-shadow(".into(), value, &")".into()]))
                .collect::<Vec<_>>(),
            " ",
        )
    } else {
        "var(--tw-drop-shadow-size)".into()
    };
    nodes.push(AstNode::declaration("--tw-drop-shadow", value));
    nodes.push(AstNode::declaration("filter", FILTER_VALUE));
    Ok(output(nodes, properties(false)))
}

fn mark_variables(
    nodes: &[AstNode],
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
    max_bytes: usize,
) -> Result<Option<FilterOutput>, super::utilities::FamilyFailure> {
    let (root, negative) = root
        .strip_prefix('-')
        .map_or((root, false), |root| (root, true));
    let backdrop = root.starts_with("backdrop-");
    let effect = root.strip_prefix("backdrop-").unwrap_or(root);
    if !matches!(root, "drop-shadow" | "backdrop-opacity")
        && !matches!(
            effect,
            "filter"
                | "blur"
                | "brightness"
                | "contrast"
                | "grayscale"
                | "hue-rotate"
                | "invert"
                | "saturate"
                | "sepia"
        )
        || negative && effect != "hue-rotate"
    {
        return Ok(None);
    }
    let result = if root == "drop-shadow" {
        drop_shadow(value, modifier, theme, used, max_bytes)?
    } else if effect == "filter" {
        if modifier.is_some() {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        let (value, globals) = match value {
            None => (
                if backdrop {
                    Text::from(BACKDROP_VALUE)
                } else {
                    Text::from(FILTER_VALUE)
                },
                properties(backdrop),
            ),
            Some(CandidateValue::Arbitrary { value, .. }) => (value.clone(), Vec::new()),
            Some(CandidateValue::Named { value }) if value == "none" => {
                (Text::from("none"), Vec::new())
            }
            _ => return Err(DiagnosticKind::InvalidValue.into()),
        };
        output(filter_declarations(backdrop, &value), globals)
    } else {
        let value = generic_value(root, value, modifier, theme, used)?;
        let value = if negative {
            concat_text(&[&"calc(".into(), &value, &" * -1)".into()])
        } else {
            value
        };
        let mut nodes = vec![AstNode::declaration(
            format!("--tw-{root}"),
            concat_text(&[&format!("{effect}(").into(), &value, &")".into()]),
        )];
        nodes.extend(filter_declarations(
            backdrop,
            &Text::from(if backdrop {
                BACKDROP_VALUE
            } else {
                FILTER_VALUE
            }),
        ));
        output(nodes, properties(backdrop))
    };
    mark_variables(&result.nodes, used)?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{ast::AstNode, Compiler, CompilerInput, Limits};
    use crate::program::GpuCondition;

    #[test]
    fn filter_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/filters-fixtures.json"
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
    fn dynamic_shadow_alpha_preserves_supports_ast_and_backdrop_vendor_pair() {
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
        let shadow_candidate = "drop-shadow-[0_1px_red]/[var(--opacity)]!";
        let prepared = compiler
            .prepare_add(&[shadow_candidate.into(), "backdrop-blur-[13px]".into()])
            .unwrap();
        let nodes = prepared
            .ast()
            .iter()
            .find_map(|node| match node {
                AstNode::Rule {
                    selector, nodes, ..
                } if selector.starts_with(".drop-shadow-") => Some(nodes),
                _ => None,
            })
            .unwrap();
        assert!(
            matches!(&nodes[2], AstNode::AtRule { name, params, nodes: Some(children) , .. }
            if name == "@supports" && params == "(color: lab(from red l a b))"
            && matches!(&children[0], AstNode::Declaration { property, value, important: true, .. }
                if property == "--tw-drop-shadow-size" && value.contains("oklab(from red l a b / var(--opacity))")))
        );
        let report = prepared.report();
        let shadow = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == shadow_candidate)
            .unwrap();
        assert_eq!(
            shadow
                .declarations
                .iter()
                .filter(|declaration| declaration.property == "--tw-drop-shadow-size")
                .count(),
            1
        );
        assert!(shadow
            .conditions
            .iter()
            .any(|condition| matches!(condition, GpuCondition::Unsupported { .. })));
        assert_eq!(
            report
                .gpu_variables
                .get(&crate::text::Text::from("--tw-drop-shadow-alpha"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("100%")
        );
        let backdrop = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "backdrop-blur-[13px]")
            .unwrap();
        assert_eq!(backdrop.declarations[1].property, "-webkit-backdrop-filter");
        assert_eq!(backdrop.declarations[2].property, "backdrop-filter");
        assert_eq!(
            backdrop.declarations[1].value,
            backdrop.declarations[2].value
        );
    }
}
