//! Native box/text/inset shadows and rings with structural alpha fallbacks.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{
        as_color, canonical_nonnegative_text, inferred_type_text, is_color_text, js_number_text,
        js_string, resolve_named,
    },
    dependencies::{box_shadow_properties, property},
    theme::Theme,
    value::ValueNode,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct ShadowOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

const BOX_SHADOW: &str = "var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)";
const RING_OFFSET_SHADOW: &str =
    "var(--tw-ring-inset,) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)";

fn output(nodes: Vec<AstNode>, globals: Vec<AstNode>) -> ShadowOutput {
    fn count(nodes: &[AstNode]) -> usize {
        nodes
            .iter()
            .map(|node| {
                usize::from(matches!(node, AstNode::Declaration { .. }))
                    + node.children().map_or(0, count)
            })
            .sum()
    }
    ShadowOutput {
        declaration_count: count(&nodes),
        nodes,
        globals,
    }
}

fn properties(root: &str) -> Vec<AstNode> {
    if root == "text-shadow" {
        vec![
            property("--tw-text-shadow-color", None, "*"),
            property("--tw-text-shadow-alpha", Some("100%"), "<percentage>"),
        ]
    } else {
        box_shadow_properties()
    }
}

fn named_color(
    value: &Text,
    modifier: Option<&Text>,
    namespaces: &[&str],
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    let color = match value {
        value if value == "current" => Some("currentcolor".into()),
        value if value == "inherit" || value == "transparent" => Some(value.clone()),
        _ => resolve_named(value, namespaces, theme, used)?,
    }
    .filter(|value| !value.is_empty());
    match color {
        Some(color) => as_color(color, modifier, theme, used),
        None => Ok(None),
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

/// Upstream `replaceAlpha`: relative oklab color with the alpha replaced.
pub(super) fn replace_alpha(color: &Text, alpha: &Text) -> Text {
    concat_text(&[
        &"oklab(from ".into(),
        color,
        &" l a b / ".into(),
        &alpha_percentage(alpha),
        &")".into(),
    ])
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

// The source helper detects offsets by a number prefix, not full CSS length validation.
fn offset_prefix(value: &Text) -> bool {
    let units = value.units();
    // The fixed LENGTH regex has no dotAll flag: its suffix cannot cross a line terminator.
    if units
        .iter()
        .any(|unit| matches!(*unit, 0x0a | 0x0d | 0x2028 | 0x2029))
    {
        return false;
    }
    let index = usize::from(units.first() == Some(&45));
    let index = index + usize::from(units.get(index) == Some(&46));
    units
        .get(index)
        .is_some_and(|unit| (48..=57).contains(unit))
}

pub(super) fn segment(
    value: &Text,
    delimiter: char,
) -> Result<Vec<Text>, super::utilities::FamilyFailure> {
    super::value::segment_text(value, delimiter as u16, usize::MAX).map_err(|error| match error {
        super::CompileError::Syntax { .. } => DiagnosticKind::InvalidValue.into(),
        error => super::utilities::FamilyFailure::Resource(error),
    })
}

pub(super) fn join_text(values: &[Text], separator: &str) -> Text {
    let mut output = Text::default();
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(separator);
        }
        output.push_text(value);
    }
    output
}

fn trim_js(value: &Text) -> Text {
    fn whitespace(unit: &u16) -> bool {
        matches!(*unit, 0x09..=0x0d | 0x20 | 0xa0 | 0x1680 | 0x2000..=0x200a | 0x2028 | 0x2029 | 0x202f | 0x205f | 0x3000 | 0xfeff)
    }
    let units = value.units();
    let start = units
        .iter()
        .position(|unit| !whitespace(unit))
        .unwrap_or(units.len());
    let end = units
        .iter()
        .rposition(|unit| !whitespace(unit))
        .map_or(start, |index| index + 1);
    Text::from_units(units[start..end].to_vec())
}

// Measure the complete UTF-16 splice before allocating it. Lone surrogates cost
// three logical bytes; a pair costs four, including pairs formed across chunks.
fn bounded_text(
    max_bytes: usize,
    reserved_bytes: usize,
    chunks: impl Fn(
        &mut dyn FnMut(&[u16]) -> Result<(), super::utilities::FamilyFailure>,
    ) -> Result<(), super::utilities::FamilyFailure>,
) -> Result<Text, super::utilities::FamilyFailure> {
    super::check_limit(reserved_bytes, max_bytes, "css_bytes")?;
    let mut bytes = reserved_bytes;
    let mut length = 0;
    let mut high = false;
    chunks(&mut |units| {
        for &unit in units {
            let increment = if unit < 0x80 || high && (0xdc00..=0xdfff).contains(&unit) {
                1
            } else if unit < 0x800 {
                2
            } else {
                3
            };
            bytes = bytes.saturating_add(increment);
            super::check_limit(bytes, max_bytes, "css_bytes")?;
            high = (0xd800..=0xdbff).contains(&unit);
            length += 1;
        }
        Ok(())
    })?;
    let mut output = Vec::with_capacity(length);
    chunks(&mut |units| {
        output.extend_from_slice(units);
        Ok(())
    })?;
    Ok(Text::from_units(output))
}

// String.replace with a string search interprets these markers without captures.
fn replaced_text(
    replacement: &Text,
    input: &Text,
    start: usize,
    length: usize,
    max_bytes: usize,
    reserved_bytes: usize,
) -> Result<Text, super::utilities::FamilyFailure> {
    bounded_text(max_bytes, reserved_bytes, |visit| {
        visit(&input.units()[..start])?;
        let units = replacement.units();
        let mut index = 0;
        while index < units.len() {
            let chunk = if units[index] == 36 {
                match units.get(index + 1) {
                    Some(36) => Some(&units[index..index + 1]),
                    Some(38) => Some(&input.units()[start..start + length]),
                    Some(96) => Some(&input.units()[..start]),
                    Some(39) => Some(&input.units()[start + length..]),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(chunk) = chunk {
                visit(chunk)?;
                index += 2;
            } else {
                visit(&units[index..index + 1])?;
                index += 1;
            }
        }
        visit(&input.units()[start + length..])
    })
}

pub(super) fn replace_shadow_colors(
    value: &Text,
    max_bytes: usize,
    mut replacement: impl FnMut(&Text) -> Text,
) -> Result<Text, super::utilities::FamilyFailure> {
    let mut shadows = Vec::new();
    let mut bytes: usize = 0;
    for shadow in segment(value, ',')? {
        let shadow = trim_js(&shadow);
        let mut offsets = 0;
        let mut color = None;
        for part in segment(&shadow, ' ')? {
            if trim_js(&part).is_empty()
                || ["inset", "inherit", "initial", "revert", "unset"]
                    .iter()
                    .any(|keyword| part == *keyword)
            {
                continue;
            }
            if offset_prefix(&part) {
                offsets += 1;
            } else if color.is_none() {
                color = Some(part);
            }
        }
        let reserved = bytes.saturating_add(if shadows.is_empty() { 0 } else { 2 });
        if offsets < 2 {
            bytes = reserved.saturating_add(shadow.byte_len());
            super::check_limit(bytes, max_bytes, "css_bytes")?;
            shadows.push(shadow);
            continue;
        }
        let replacement = replacement(color.as_ref().unwrap_or(&"currentcolor".into()));
        let output = if let Some(color) = color {
            let index = shadow
                .units()
                .windows(color.units().len())
                .position(|part| part == color.units())
                .ok_or(DiagnosticKind::InvalidValue)?;
            replaced_text(
                &replacement,
                &shadow,
                index,
                color.units().len(),
                max_bytes,
                reserved,
            )?
        } else {
            // No search match: the upstream helper appends markers literally.
            bounded_text(max_bytes, reserved, |visit| {
                visit(shadow.units())?;
                visit(&[32])?;
                visit(replacement.units())
            })?
        };
        bytes = reserved.saturating_add(output.byte_len());
        shadows.push(output);
    }
    Ok(join_text(&shadows, ", "))
}

fn replaced_shadow_nodes(
    property: &str,
    value: &Text,
    alpha: Option<&Text>,
    color_property: &str,
    prefix: &str,
    max_bytes: usize,
) -> Result<Vec<AstNode>, super::utilities::FamilyFailure> {
    let mut fallback = false;
    let replaced = replace_shadow_colors(value, max_bytes, |color| {
        let color = if let Some(alpha) = alpha {
            if color.starts_with("current") {
                with_alpha(color, alpha)
            } else {
                fallback |= color.starts_with("var(") || alpha.starts_with("var(");
                replace_alpha(color, alpha)
            }
        } else {
            color.clone()
        };
        concat_text(&[
            &format!("var({color_property}, ").into(),
            &color,
            &")".into(),
        ])
    })?;
    let apply_prefix = |value: Text| -> Result<Text, super::utilities::FamilyFailure> {
        if prefix.is_empty() {
            Ok(value)
        } else {
            let segments = segment(&value, ',')?;
            let bytes = segments
                .iter()
                .fold(segments.len().saturating_sub(1), |bytes, value| {
                    bytes
                        .saturating_add(prefix.len())
                        .saturating_add(value.byte_len())
                });
            super::check_limit(bytes, max_bytes, "css_bytes")?;
            Ok(join_text(
                &segments
                    .iter()
                    .map(|value| concat_text(&[&prefix.into(), value]))
                    .collect::<Vec<_>>(),
                ",",
            ))
        }
    };
    if fallback {
        let plain = replace_shadow_colors(value, max_bytes, |color| {
            concat_text(&[
                &format!("var({color_property}, ").into(),
                color,
                &")".into(),
            ])
        })?;
        Ok(vec![
            AstNode::declaration(property, apply_prefix(plain)?),
            AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@supports".into(),
                params: "(color: lab(from red l a b))".into(),
                nodes: Some(vec![AstNode::declaration(
                    property,
                    apply_prefix(replaced)?,
                )]),
            },
        ])
    } else {
        Ok(vec![AstNode::declaration(
            property,
            apply_prefix(replaced)?,
        )])
    }
}

fn shadow(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
    max_bytes: usize,
) -> Result<ShadowOutput, super::utilities::FamilyFailure> {
    let alpha = alpha_modifier(modifier)?;
    let property = if root == "text-shadow" {
        "text-shadow".into()
    } else {
        format!("--tw-{root}")
    };
    let color_property = format!("--tw-{root}-color");
    let alpha_property = format!("--tw-{root}-alpha");
    let color_output = |color: Text| {
        output(
            vec![AstNode::declaration(
                &color_property,
                with_alpha(&color, &format!("var({alpha_property})").into()),
            )],
            properties(root),
        )
    };
    let (raw, prefix) = match value {
        None => (
            theme
                .raw_text(&Text::from(&format!("--{root}")), used)
                .ok_or(DiagnosticKind::InvalidValue)?,
            "",
        ),
        Some(CandidateValue::Arbitrary { value, data_type }) => {
            if data_type
                .as_ref()
                .map(super::colors::known_type)
                .map_or_else(|| is_color_text(value), |hint| hint == "color")
            {
                let color = as_color(value.clone(), modifier, theme, used)?
                    .ok_or(DiagnosticKind::InvalidValue)?;
                return Ok(color_output(color));
            }
            (
                value.clone(),
                if root == "inset-shadow" { "inset " } else { "" },
            )
        }
        Some(CandidateValue::Named { value }) => {
            if value == "none" || value == "inherit" {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                let mut nodes = if value == "inherit" {
                    vec![AstNode::declaration(&color_property, "inherit")]
                } else {
                    vec![AstNode::declaration(
                        &property,
                        if root == "text-shadow" {
                            "none"
                        } else {
                            "0 0 #0000"
                        },
                    )]
                };
                if value == "none" && root != "text-shadow" {
                    nodes.push(AstNode::declaration("box-shadow", BOX_SHADOW));
                }
                return Ok(output(nodes, properties(root)));
            }
            if let Some(raw) = theme
                .raw_text(&concat_text(&[&format!("--{root}-").into(), value]), used)
                .filter(|value| !value.is_empty())
            {
                (raw, "")
            } else {
                let color = named_color(
                    value,
                    modifier,
                    &[
                        if root == "text-shadow" {
                            "text-shadow-color"
                        } else {
                            "box-shadow-color"
                        },
                        "color",
                    ],
                    theme,
                    used,
                )?
                .ok_or(DiagnosticKind::InvalidValue)?;
                return Ok(color_output(color));
            }
        }
    };
    let mut nodes = Vec::new();
    if let Some(alpha) = &alpha {
        nodes.push(AstNode::declaration(alpha_property, alpha.clone()));
    }
    nodes.extend(replaced_shadow_nodes(
        &property,
        &raw,
        alpha.as_ref(),
        &color_property,
        prefix,
        max_bytes,
    )?);
    if root != "text-shadow" {
        nodes.push(AstNode::declaration("box-shadow", BOX_SHADOW));
    }
    Ok(output(nodes, properties(root)))
}

fn ring_width(
    root: &str,
    value: Text,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<ShadowOutput, super::utilities::FamilyFailure> {
    if root == "ring-offset" {
        return Ok(output(
            vec![
                AstNode::declaration("--tw-ring-offset-width", value),
                AstNode::declaration("--tw-ring-offset-shadow", RING_OFFSET_SHADOW),
            ],
            Vec::new(),
        ));
    }
    let shadow = if root == "inset-ring" {
        concat_text(&[
            &"inset 0 0 0 ".into(),
            &value,
            &" var(--tw-inset-ring-color, currentcolor)".into(),
        ])
    } else {
        let color = theme
            .raw_text(&Text::from("--default-ring-color"), used)
            .unwrap_or_else(|| "currentcolor".into());
        concat_text(&[
            &"var(--tw-ring-inset,) 0 0 0 calc(".into(),
            &value,
            &" + var(--tw-ring-offset-width)) var(--tw-ring-color, ".into(),
            &color,
            &")".into(),
        ])
    };
    Ok(output(
        vec![
            AstNode::declaration(format!("--tw-{root}-shadow"), shadow),
            AstNode::declaration("box-shadow", BOX_SHADOW),
        ],
        box_shadow_properties(),
    ))
}

fn ring(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<ShadowOutput, super::utilities::FamilyFailure> {
    let property = format!("--tw-{root}-color");
    let color_output =
        |color: Text| output(vec![AstNode::declaration(&property, color)], Vec::new());
    let width_namespaces: &[&str] = if root == "ring-offset" {
        &["ring-offset-width"]
    } else {
        &["ring-width"]
    };
    let color_namespaces: &[&str] = if root == "ring-offset" {
        &["ring-offset-color", "color"]
    } else {
        &["ring-color", "color"]
    };
    match value {
        None => {
            if root == "ring-offset" || modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            let width = if root == "ring" {
                theme
                    .raw_text(&Text::from("--default-ring-width"), used)
                    .unwrap_or_else(|| "1px".into())
            } else {
                "1px".into()
            };
            ring_width(root, width, theme, used)
        }
        Some(CandidateValue::Arbitrary { value, data_type }) => {
            if data_type
                .as_ref()
                .map(super::colors::known_type)
                .map_or_else(
                    || inferred_type_text("decoration", value) == Some("length"),
                    |hint| hint == "length",
                )
            {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                ring_width(root, value.clone(), theme, used)
            } else {
                Ok(color_output(
                    as_color(value.clone(), modifier, theme, used)?
                        .ok_or(DiagnosticKind::InvalidValue)?,
                ))
            }
        }
        Some(CandidateValue::Named { value }) => {
            if root != "ring-offset" {
                if let Some(color) = named_color(value, modifier, color_namespaces, theme, used)? {
                    return Ok(color_output(color));
                }
            }
            let width = resolve_named(value, width_namespaces, theme, used)?
                .filter(|value| !value.is_empty())
                .or_else(|| {
                    canonical_nonnegative_text(value, true)
                        .then(|| concat_text(&[value, &"px".into()]))
                });
            if let Some(width) = width {
                if modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                return ring_width(root, width, theme, used);
            }
            if root == "ring-offset" {
                if let Some(color) = named_color(value, modifier, color_namespaces, theme, used)? {
                    return Ok(color_output(color));
                }
            }
            Err(DiagnosticKind::InvalidValue.into())
        }
    }
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
) -> Result<Option<ShadowOutput>, super::utilities::FamilyFailure> {
    let result = match root {
        "text-shadow" | "shadow" | "inset-shadow" => {
            shadow(root, value, modifier, theme, used, max_bytes)?
        }
        "ring" | "inset-ring" | "ring-offset" => ring(root, value, modifier, theme, used)?,
        _ => return Ok(None),
    };
    mark_variables(&result.nodes, used)?;
    Ok(Some(result))
}

#[cfg(test)]
mod text_tests {
    use super::{offset_prefix, replace_shadow_colors, with_alpha};
    use crate::text::Text;

    fn units(prefix: &str, raw: &[u16], suffix: &str) -> Text {
        Text::from_units(
            prefix
                .encode_utf16()
                .chain(raw.iter().copied())
                .chain(suffix.encode_utf16())
                .collect(),
        )
    }

    #[test]
    fn shadow_replacement_preserves_raw_color_units_and_first_occurrence() {
        // The fixed helper replaces the first matching color substring, while retaining
        // the remainder of each shadow and joining comma segments with ", ".
        for raw in [
            &[0xd800][..],
            &[0xdc00],
            &[0xd83d, 0xde00],
            &[0xdc00, 0xd800],
            &[0xfffd],
            &[0xe000],
            &[92, 117, 100, 56, 48, 48],
        ] {
            let input = units("0 0 ", raw, " 1px");
            let replaced = replace_shadow_colors(&input, usize::MAX, |color| {
                units("var(--color, ", color.units(), ")")
            })
            .unwrap();
            assert_eq!(
                replaced.units(),
                units("0 0 var(--color, ", raw, ") 1px").units()
            );
            let twice = units("0 0 ", raw, " ");
            let twice = Text::from_units(
                twice
                    .into_units()
                    .into_iter()
                    .chain(raw.iter().copied())
                    .collect(),
            );
            let mut expected = units("0 0 var(--color, ", raw, ") ");
            expected.push_text(&Text::from_units(raw.to_vec()));
            assert_eq!(
                replace_shadow_colors(&twice, usize::MAX, |color| units(
                    "var(--color, ",
                    color.units(),
                    ")"
                ))
                .unwrap()
                .units(),
                expected.units()
            );
        }
    }

    #[test]
    fn shadow_trim_uses_js_bom_and_retains_nel_and_lone_units() {
        for raw in [&[0xd800][..], &[0xdc00]] {
            let input = units("\u{feff}0 0 ", raw, "\u{feff}");
            let output = replace_shadow_colors(&input, usize::MAX, |color| {
                units("var(--color, ", color.units(), ")")
            })
            .unwrap();
            assert_eq!(output.units(), units("0 0 var(--color, ", raw, ")").units());
            // NEL is not JS whitespace. It prevents the first offset from matching LENGTH.
            let input = units("\u{85}0 0 ", raw, "");
            assert_eq!(
                replace_shadow_colors(&input, usize::MAX, |_| "red".into())
                    .unwrap()
                    .units(),
                input.units()
            );
        }
    }

    #[test]
    fn shadow_offsets_reject_js_line_terminators_without_normalizing_other_units() {
        for raw in [&[0xd800][..], &[0xdc00], &[0xd83d, 0xde00]] {
            assert!(offset_prefix(&units("-.1", raw, "px")));
            for terminator in [0x0a, 0x0d, 0x2028, 0x2029] {
                let mut offset = units("1", raw, "").into_units();
                offset.push(terminator);
                assert!(!offset_prefix(&Text::from_units(offset)));
                let mut offset = vec![49, terminator];
                offset.extend_from_slice(raw);
                assert!(!offset_prefix(&Text::from_units(offset)));
                let mut input = Text::from("0 0").into_units();
                input.push(terminator);
                input.push(32);
                input.extend_from_slice(raw);
                let input = Text::from_units(input);
                // With only one matching offset, the fixed helper leaves the shadow intact.
                assert_eq!(
                    replace_shadow_colors(&input, usize::MAX, |_| "red".into())
                        .unwrap()
                        .units(),
                    input.units()
                );
            }
        }
    }

    #[test]
    fn shadow_color_replacement_interprets_js_dollar_markers_on_original_units() {
        for raw in [&[0xd800][..], &[0xdc00], &[0xd83d, 0xde00]] {
            let input = units("0 0 ", raw, " 1px");
            for marker in ["$$", "$&", "$`", "$'", "$1", "$01", "$<name>", "$"] {
                let replacement = units("L", raw, &format!("{marker}R"));
                let mut expected = units("0 0 L", raw, "");
                match marker {
                    "$$" => expected.push_str("$"),
                    "$&" => expected.push_text(&Text::from_units(raw.to_vec())),
                    "$`" => expected.push_str("0 0 "),
                    "$'" => expected.push_str(" 1px"),
                    _ => expected.push_str(marker),
                }
                expected.push_str("R 1px");
                assert_eq!(
                    replace_shadow_colors(&input, usize::MAX, |_| replacement.clone())
                        .unwrap()
                        .units(),
                    expected.units()
                );
            }
        }
        // The helper appends a replacement directly when no color segment was found.
        assert_eq!(
            replace_shadow_colors(&"0 0".into(), usize::MAX, |_| "$&".into()).unwrap(),
            "0 0 $&"
        );
    }

    #[test]
    fn shadow_budget_counts_exact_logical_bytes_and_comma_separators() {
        for raw in [
            &[65][..],
            &[0x7ff],
            &[0x800],
            &[0xd800],
            &[0xdc00],
            &[0xd83d, 0xde00],
            &[0xdc00, 0xd800],
        ] {
            for input in [Text::from("0 0 red"), Text::from("0 0"), Text::from("none")] {
                let replacement = Text::from_units(raw.to_vec());
                let expected =
                    replace_shadow_colors(&input, usize::MAX, |_| replacement.clone()).unwrap();
                let bytes = expected.byte_len();
                for limit in [bytes, bytes + 1] {
                    assert_eq!(
                        replace_shadow_colors(&input, limit, |_| replacement.clone()).unwrap(),
                        expected
                    );
                }
                assert!(
                    matches!(replace_shadow_colors(&input, bytes - 1, |_| replacement.clone()),
                    Err(super::super::utilities::FamilyFailure::Resource(
                        super::super::CompileError::Limit { resource, limit }
                    )) if resource == "css_bytes" && limit == bytes - 1)
                );
            }
            let input = Text::from("0 0 red, 0 0 blue");
            let expected = units("0 0 ", raw, ", 0 0 ");
            let expected = super::concat_text(&[&expected, &Text::from_units(raw.to_vec())]);
            let bytes = expected.byte_len();
            assert_eq!(
                replace_shadow_colors(&input, bytes, |_| Text::from_units(raw.to_vec())).unwrap(),
                expected
            );
            assert!(
                replace_shadow_colors(&input, bytes - 1, |_| Text::from_units(raw.to_vec()))
                    .is_err()
            );
        }
    }

    #[test]
    fn shadow_splice_budget_counts_surrogate_pairs_across_chunk_boundaries() {
        for (input, replacement, start, length, expected) in [
            (vec![0xd800, 65, 0xdc00], vec![], 1, 1, vec![0xd800, 0xdc00]),
            (vec![0xd800, 65], vec![0xdc00], 1, 1, vec![0xd800, 0xdc00]),
            (vec![65, 0xdc00], vec![0xd800], 0, 1, vec![0xd800, 0xdc00]),
            (vec![0xdc00, 65, 0xd800], vec![], 1, 1, vec![0xdc00, 0xd800]),
        ] {
            let expected = Text::from_units(expected);
            let bytes = expected.byte_len();
            let input = Text::from_units(input);
            let replacement = Text::from_units(replacement);
            assert_eq!(
                super::replaced_text(&replacement, &input, start, length, bytes, 0).unwrap(),
                expected
            );
            assert!(
                super::replaced_text(&replacement, &input, start, length, bytes - 1, 0).is_err()
            );
        }
    }

    #[test]
    fn relative_alpha_text_preserves_non_scalar_color_and_alpha_units() {
        let color = units("oklch(", &[0xd800], ")");
        let alpha = units("var(--", &[0xdc00], ")");
        let mut expected = Text::from("color-mix(in oklab, ");
        expected.push_text(&color);
        expected.push_str(" ");
        expected.push_text(&alpha);
        expected.push_str(", transparent)");
        assert_eq!(with_alpha(&color, &alpha).units(), expected.units());
        assert_eq!(with_alpha(&color, &"1".into()).units(), color.units());
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::{ast::AstNode, Compiler, CompilerInput, Limits};
    use crate::program::GpuCondition;

    #[test]
    #[cfg(rustify_repo_tests)]
    fn shadow_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/shadows-fixtures.json"
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
    fn dynamic_inset_alpha_keeps_nested_ast_and_offset_does_not_create_resources() {
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
            .prepare_add(&["ring-offset-[13px]".into()])
            .unwrap();
        assert!(!prepared.report().css.contains("@property"));
        compiler.discard(prepared).unwrap();
        let candidate = "inset-shadow-[0_1px_red,2px_3px_var(--color)]/[var(--opacity)]!";
        let prepared = compiler.prepare_add(&[candidate.into()]).unwrap();
        let nodes = prepared
            .ast()
            .iter()
            .find_map(|node| match node {
                AstNode::Rule {
                    selector, nodes, ..
                } if selector.starts_with(".inset-shadow-") => Some(nodes),
                _ => None,
            })
            .unwrap();
        assert!(
            matches!(&nodes[1], AstNode::Declaration { property, value, important: true, .. }
            if property == "--tw-inset-shadow" && value.starts_with("inset ") && value.contains(",inset "))
        );
        assert!(
            matches!(&nodes[2], AstNode::AtRule { name, params, nodes: Some(children) , .. }
            if name == "@supports" && params == "(color: lab(from red l a b))"
            && matches!(&children[0], AstNode::Declaration { property, value, important: true, .. }
                if property == "--tw-inset-shadow" && value.contains("oklab(from red l a b / var(--opacity))")))
        );
        let report = prepared.report();
        let program = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == candidate)
            .unwrap();
        assert_eq!(
            program
                .declarations
                .iter()
                .filter(|declaration| declaration.property == "--tw-inset-shadow")
                .count(),
            1
        );
        assert!(program
            .conditions
            .iter()
            .any(|condition| matches!(condition, GpuCondition::Unsupported { .. })));
        assert_eq!(
            report
                .gpu_variables
                .get(&crate::text::Text::from("--tw-inset-shadow-alpha"))
                .map(|value| value.to_utf8().unwrap())
                .as_deref(),
            Some("100%")
        );
    }
}
