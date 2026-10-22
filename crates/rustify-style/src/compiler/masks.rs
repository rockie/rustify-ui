//! Native mask image, positioning, sizing, edge and gradient stop branches.

use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{as_color, canonical_nonnegative_text, is_color_text, is_number_text, resolve_named},
    dependencies::property,
    theme::Theme,
    value::concat_text,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct MaskOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

const MASK_IMAGE: &str = "var(--tw-mask-linear), var(--tw-mask-radial), var(--tw-mask-conic)";
const EDGE_IMAGE: &str =
    "var(--tw-mask-left), var(--tw-mask-right), var(--tw-mask-bottom), var(--tw-mask-top)";

fn output(declarations: Vec<(Text, Text)>, globals: Vec<AstNode>) -> MaskOutput {
    MaskOutput {
        declaration_count: declarations.len(),
        nodes: declarations
            .into_iter()
            .map(|(name, value)| AstNode::declaration(name, value))
            .collect(),
        globals,
    }
}

fn common() -> Vec<(Text, Text)> {
    vec![
        ("mask-image".into(), MASK_IMAGE.into()),
        ("mask-composite".into(), "intersect".into()),
    ]
}

fn resources(values: &[(&str, &str)]) -> Vec<AstNode> {
    values
        .iter()
        .map(|(name, initial)| property(name, Some(initial), "*"))
        .collect()
}

fn gradient_resources() -> Vec<AstNode> {
    resources(&[
        ("--tw-mask-linear", "linear-gradient(#fff, #fff)"),
        ("--tw-mask-radial", "linear-gradient(#fff, #fff)"),
        ("--tw-mask-conic", "linear-gradient(#fff, #fff)"),
    ])
}

fn kind_resources(kind: &str) -> Vec<AstNode> {
    let mut values = Vec::new();
    if kind != "radial" {
        values.push((format!("--tw-mask-{kind}-position"), "0deg"));
    }
    for (suffix, initial) in [
        ("from-position", "0%"),
        ("to-position", "100%"),
        ("from-color", "black"),
        ("to-color", "transparent"),
    ] {
        values.push((format!("--tw-mask-{kind}-{suffix}"), initial));
    }
    if kind == "radial" {
        for (suffix, initial) in [
            ("shape", "ellipse"),
            ("size", "farthest-corner"),
            ("position", "center"),
        ] {
            values.push((format!("--tw-mask-{kind}-{suffix}"), initial));
        }
    }
    values
        .iter()
        .map(|(name, initial)| property(name, Some(initial), "*"))
        .collect()
}

fn math(value: &Text) -> bool {
    value.contains("(") && is_number_text(value)
}
fn number(value: &Text) -> bool {
    !value.contains("(") && is_number_text(value)
}
fn percentage(value: &Text) -> bool {
    value
        .strip_suffix("%")
        .is_some_and(|number_value| number(&number_value))
        || math(value)
}
fn length(value: &Text) -> bool {
    [
        "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
        "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi",
        "cqb", "cqmin", "cqmax",
    ]
    .iter()
    .any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number_value| number(&number_value))
    }) || math(value)
}
fn segments(value: &Text, separator: char) -> Result<Vec<Text>, super::utilities::FamilyFailure> {
    Ok(super::value::segment_text(
        value,
        separator as u16,
        usize::MAX,
    )?)
}
fn url(value: &Text) -> bool {
    value.starts_with("url(")
        && value.ends_with(")")
        && !value
            .units()
            .iter()
            .any(|unit| matches!(unit, 10 | 13 | 0x2028 | 0x2029))
}
fn image(value: &Text) -> Result<bool, super::utilities::FamilyFailure> {
    let mut count = 0;
    for part in segments(value, ',')? {
        if part.starts_with("var(") {
            continue;
        }
        if url(&part)
            || [
                "element(",
                "image(",
                "cross-fade(",
                "image-set(",
                "linear-gradient(",
                "radial-gradient(",
                "conic-gradient(",
                "repeating-linear-gradient(",
                "repeating-radial-gradient(",
                "repeating-conic-gradient(",
            ]
            .iter()
            .any(|prefix| part.starts_with(prefix))
        {
            count += 1;
        } else {
            return Ok(false);
        }
    }
    Ok(count > 0)
}
fn position(value: &Text) -> Result<bool, super::utilities::FamilyFailure> {
    let mut count = 0;
    for part in segments(value, ' ')? {
        if part.starts_with("var(") {
            continue;
        }
        if ["center", "top", "right", "bottom", "left"]
            .iter()
            .any(|keyword| part == *keyword)
            || length(&part)
            || percentage(&part)
        {
            count += 1;
        } else {
            return Ok(false);
        }
    }
    Ok(count > 0)
}
fn size(value: &Text) -> Result<bool, super::utilities::FamilyFailure> {
    let mut count = 0;
    for part in segments(value, ',')? {
        if part == "cover" || part == "contain" {
            count += 1;
            continue;
        }
        let parts = segments(&part, ' ')?;
        if parts.len() != 1 && parts.len() != 2 {
            return Ok(false);
        }
        if parts
            .iter()
            .all(|value| value == "auto" || length(value) || percentage(value))
        {
            count += 1;
        }
    }
    Ok(count > 0)
}
fn mask_type(value: &Text) -> Result<Option<&'static str>, super::utilities::FamilyFailure> {
    if value.starts_with("var(") {
        return Ok(None);
    }
    Ok(if image(value)? {
        Some("image")
    } else if percentage(value) {
        Some("percentage")
    } else if position(value)? {
        Some("position")
    } else if size(value)? {
        Some("bg-size")
    } else if length(value) {
        Some("length")
    } else if url(value) {
        Some("url")
    } else {
        None
    })
}
fn stop_type(value: &Text) -> Option<&'static str> {
    if value.starts_with("var(") {
        return None;
    }
    if length(value) {
        Some("length")
    } else if percentage(value) {
        Some("percentage")
    } else if is_color_text(value) {
        Some("color")
    } else {
        None
    }
}

// Upstream percentage validation slices its last UTF-16 code unit even for an
// explicit type hint. A supplementary final character cannot leave an integer.
fn percentage_integer(value: &Text) -> bool {
    value.units().len().checked_sub(1).is_some_and(|end| {
        canonical_nonnegative_text(&Text::from_units(value.units()[..end].to_vec()), true)
    })
}

fn stop_value(
    value: &CandidateValue,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<(Text, &'static str), super::utilities::FamilyFailure> {
    if let CandidateValue::Arbitrary { value, data_type } = value {
        let kind = data_type
            .as_ref()
            .map(super::colors::known_type)
            .or_else(|| stop_type(value));
        if kind == Some("color") {
            return as_color(value.clone(), modifier, theme, used)?
                .map(|value| (value, "color"))
                .ok_or(super::utilities::FamilyFailure::Candidate(
                    DiagnosticKind::InvalidValue,
                ));
        }
        if modifier.is_some() || (kind == Some("percentage") && !percentage_integer(value)) {
            return Err(DiagnosticKind::InvalidValue.into());
        }
        return Ok((value.clone(), "position"));
    }
    let CandidateValue::Named { value } = value else {
        unreachable!()
    };
    let color = match value {
        value if value == "current" => Some("currentcolor".into()),
        value if value == "inherit" || value == "transparent" => Some(value.clone()),
        _ => resolve_named(value, &["background-color", "color"], theme, used)?
            .filter(|value| !value.is_empty()),
    };
    if let Some(color) = color {
        if let Some(color) = as_color(color, modifier, theme, used)? {
            return Ok((color, "color"));
        }
    }
    if modifier.is_some() {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    if is_number_text(value) {
        let spacing = theme
            .resolve_text(&"--spacing".into(), used)
            .filter(|value| !value.is_empty())
            .ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))?;
        if canonical_nonnegative_text(value, false) {
            return Ok((
                concat_text(&[&"calc(".into(), &spacing, &" * ".into(), value, &")".into()]),
                "position",
            ));
        }
    } else if percentage(value) && percentage_integer(value) {
        return Ok((value.clone(), "position"));
    }
    Err(DiagnosticKind::InvalidValue.into())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<MaskOutput>, super::utilities::FamilyFailure> {
    let positive = root.strip_prefix('-').unwrap_or(root);
    let constructor = matches!(positive, "mask-linear" | "mask-conic" | "mask-radial");
    let stop = positive
        .strip_prefix("mask-")
        .and_then(|rest| rest.rsplit_once('-'))
        .filter(|(kind, stop)| {
            matches!(
                *kind,
                "x" | "y" | "t" | "r" | "b" | "l" | "linear" | "radial" | "conic"
            ) && matches!(*stop, "from" | "to")
        });
    if !constructor
        && stop.is_none()
        && !matches!(
            root,
            "mask" | "mask-size" | "mask-position" | "mask-radial-at"
        )
    {
        return Ok(None);
    }
    if root.starts_with('-') && !matches!(root, "-mask-linear" | "-mask-conic") {
        return Ok(None);
    }
    let value = value.ok_or(super::utilities::FamilyFailure::Candidate(
        DiagnosticKind::InvalidValue,
    ))?;
    if let Some((kind, stop)) = stop {
        let (value, data_type) = stop_value(value, modifier, theme, used)?;
        let mut globals = gradient_resources();
        let mut declarations = common();
        if matches!(kind, "x" | "y" | "t" | "r" | "b" | "l") {
            globals.extend(resources(&[
                ("--tw-mask-left", "linear-gradient(#fff, #fff)"),
                ("--tw-mask-right", "linear-gradient(#fff, #fff)"),
                ("--tw-mask-bottom", "linear-gradient(#fff, #fff)"),
                ("--tw-mask-top", "linear-gradient(#fff, #fff)"),
            ]));
            declarations.push(("--tw-mask-linear".into(), EDGE_IMAGE.into()));
            for (edge, enabled) in [
                ("top", matches!(kind, "y" | "t")),
                ("right", matches!(kind, "x" | "r")),
                ("bottom", matches!(kind, "y" | "b")),
                ("left", matches!(kind, "x" | "l")),
            ] {
                if !enabled {
                    continue;
                }
                declarations.push((format!("--tw-mask-{edge}").into(), format!("linear-gradient(to {edge}, var(--tw-mask-{edge}-from-color) var(--tw-mask-{edge}-from-position), var(--tw-mask-{edge}-to-color) var(--tw-mask-{edge}-to-position))").into()));
                for (suffix, initial) in [
                    ("from-position", "0%"),
                    ("to-position", "100%"),
                    ("from-color", "black"),
                    ("to-color", "transparent"),
                ] {
                    globals.push(property(
                        &format!("--tw-mask-{edge}-{suffix}"),
                        Some(initial),
                        "*",
                    ));
                }
                declarations.push((
                    format!("--tw-mask-{edge}-{stop}-{data_type}").into(),
                    value.clone(),
                ));
            }
        } else {
            globals.extend(kind_resources(kind));
            let position: String = match kind {
                "linear" => "var(--tw-mask-linear-position)".into(),
                "conic" => "from var(--tw-mask-conic-position)".into(),
                _ => "var(--tw-mask-radial-shape) var(--tw-mask-radial-size) at var(--tw-mask-radial-position)".into(),
            };
            declarations.push((format!("--tw-mask-{kind}-stops").into(), format!("{position}, var(--tw-mask-{kind}-from-color) var(--tw-mask-{kind}-from-position), var(--tw-mask-{kind}-to-color) var(--tw-mask-{kind}-to-position)").into()));
            declarations.push((
                format!("--tw-mask-{kind}").into(),
                format!("{kind}-gradient(var(--tw-mask-{kind}-stops))").into(),
            ));
            declarations.push((format!("--tw-mask-{kind}-{stop}-{data_type}").into(), value));
        }
        return Ok(Some(output(declarations, globals)));
    }
    if modifier.is_some() {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    let inferred_mask_type = if root == "mask" {
        if let CandidateValue::Arbitrary { value, data_type } = value {
            match data_type.as_ref().map(super::colors::known_type) {
                Some(hint) => Some(hint),
                None => mask_type(value)?,
            }
        } else {
            None
        }
    } else {
        None
    };
    let resolved: Text = match value {
        CandidateValue::Arbitrary { value, .. } => {
            if root.starts_with('-') {
                concat_text(&[&"calc(".into(), value, &" * -1)".into()])
            } else {
                value.clone()
            }
        }
        CandidateValue::Named { value }
            if matches!(positive, "mask-linear" | "mask-conic")
                && canonical_nonnegative_text(value, true) =>
        {
            concat_text(&[
                &if root.starts_with('-') {
                    "calc(1deg * -"
                } else {
                    "calc(1deg * "
                }
                .into(),
                value,
                &")".into(),
            ])
        }
        _ => return Err(DiagnosticKind::InvalidValue.into()),
    };
    if constructor {
        let kind = positive.strip_prefix("mask-").unwrap();
        let suffix = if kind == "radial" { "size" } else { "position" };
        let mut globals = gradient_resources();
        globals.extend(kind_resources(kind));
        let mut declarations = common();
        declarations.push((
            format!("--tw-mask-{kind}").into(),
            format!("{kind}-gradient(var(--tw-mask-{kind}-stops, var(--tw-mask-{kind}-{suffix})))")
                .into(),
        ));
        declarations.push((format!("--tw-mask-{kind}-{suffix}").into(), resolved));
        return Ok(Some(output(declarations, globals)));
    }
    let property = match root {
        "mask-size" => "mask-size",
        "mask-position" => "mask-position",
        "mask-radial-at" => "--tw-mask-radial-position",
        _ => match inferred_mask_type {
            Some("percentage" | "position") => "mask-position",
            Some("bg-size" | "length" | "size") => "mask-size",
            _ => "mask-image",
        },
    };
    Ok(Some(output(vec![(property.into(), resolved)], Vec::new())))
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
    #[cfg(rustify_repo_tests)]
    fn mask_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/masks-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = compiler(case["input"].as_str().unwrap());
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
    fn edge_alpha_fallback_keeps_nested_ast_important_and_source_resource_order() {
        let mut compiler = compiler("@theme { --color-brand: #123456; } @tailwind utilities;");
        let candidate = "mask-x-from-brand/[var(--opacity)]!";
        let prepared = compiler.prepare_add(&[candidate.into()]).unwrap();
        let properties = prepared
            .ast()
            .iter()
            .filter_map(|node| match node {
                AstNode::AtRule { name, params, .. } if name == "@property" => Some(params.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            &properties[..7],
            &[
                "--tw-mask-linear",
                "--tw-mask-radial",
                "--tw-mask-conic",
                "--tw-mask-left",
                "--tw-mask-right",
                "--tw-mask-bottom",
                "--tw-mask-top"
            ]
        );
        assert_eq!(
            &properties[7..11],
            &[
                "--tw-mask-right-from-position",
                "--tw-mask-right-to-position",
                "--tw-mask-right-from-color",
                "--tw-mask-right-to-color"
            ]
        );
        assert_eq!(
            &properties[11..15],
            &[
                "--tw-mask-left-from-position",
                "--tw-mask-left-to-position",
                "--tw-mask-left-from-color",
                "--tw-mask-left-to-color"
            ]
        );
        let nodes = prepared
            .ast()
            .iter()
            .find_map(|node| match node {
                AstNode::Rule {
                    selector, nodes, ..
                } if selector.starts_with(".mask-x-from-") => Some(nodes),
                _ => None,
            })
            .unwrap();
        assert!(nodes.iter().any(|node| matches!(node, AstNode::AtRule { name, nodes: Some(children), .. } if name == "@supports" && children.iter().any(|node| matches!(node, AstNode::Declaration { important: true, .. })))));
        assert!(nodes
            .iter()
            .filter_map(|node| match node {
                AstNode::Declaration { important, .. } => Some(important),
                _ => None,
            })
            .all(|important| *important));
        assert!(prepared
            .report()
            .gpu_programs
            .iter()
            .find(|program| program.candidate == candidate)
            .unwrap()
            .declarations
            .iter()
            .any(|declaration| declaration.property == "mask-image"));
    }
}
