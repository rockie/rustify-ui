//! Fixed built-in static AST templates and theme-dependent static emitters.

use super::value::concat_text;
use super::{ast::AstNode, registry, theme::Theme};
use crate::text::Text;
use std::{cmp::Ordering, collections::BTreeSet};

#[path = "../../upstream/tailwind/static-data.rs"]
mod data;

pub enum StaticNode {
    Declaration(&'static str, &'static str),
    Rule(&'static str, &'static [usize]),
    AtRule(&'static str, &'static str, &'static [usize]),
}
pub struct StaticTemplate {
    pub nodes: &'static [usize],
    pub globals: &'static [usize],
    pub property_order: &'static [usize],
    pub declaration_count: usize,
}
pub struct StaticOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
    pub property_order: Vec<usize>,
    pub declaration_count: usize,
}

fn materialize(ids: &[usize], important: bool) -> Vec<AstNode> {
    ids.iter()
        .map(|&id| match &data::NODES[id] {
            StaticNode::Declaration(property, value) => AstNode::Declaration {
                origin: None,
                source: None,
                destination: None,
                property: (*property).into(),
                value: (*value).into(),
                important,
                identity: Default::default(),
            },
            StaticNode::Rule(selector, nodes) => AstNode::Rule {
                origin: None,
                source: None,
                destination: None,
                selector: (*selector).into(),
                nodes: materialize(nodes, important),
            },
            StaticNode::AtRule(name, params, nodes) => AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: (*name).into(),
                params: (*params).into(),
                nodes: Some(materialize(nodes, important)),
            },
        })
        .collect()
}

fn property_order(properties: &[&str]) -> Vec<usize> {
    let mut order: Vec<_> = properties
        .iter()
        .filter_map(|property| {
            registry::PROPERTY_ORDER
                .iter()
                .position(|item| item == property)
        })
        .collect();
    order.sort_unstable();
    order.dedup();
    order
}

pub fn compile(
    root: &str,
    important: bool,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<StaticOutput>, super::CompileError> {
    if let Ok(index) = data::TEMPLATES.binary_search_by_key(&root, |(name, _)| *name) {
        let template = &data::TEMPLATES[index].1;
        return Ok(Some(StaticOutput {
            nodes: materialize(template.nodes, important),
            globals: materialize(template.globals, false),
            property_order: template.property_order.to_vec(),
            declaration_count: template.declaration_count,
        }));
    }
    let declaration = |property: &str, value: Text| AstNode::Declaration {
        origin: None,
        source: None,
        destination: None,
        property: property.into(),
        value,
        important,
        identity: Default::default(),
    };
    if root == "container" {
        let mut breakpoints = theme.namespace_values_text(&Text::from("--breakpoint"));
        breakpoints.sort_by(compare_breakpoints);
        let mut nodes = vec![declaration("width", "100%".into())];
        let count = 2 + breakpoints.len();
        for breakpoint in breakpoints {
            nodes.push(AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@media".into(),
                params: concat_text(&[&"(width >= ".into(), &breakpoint, &")".into()]),
                nodes: Some(vec![declaration("max-width", breakpoint)]),
            });
        }
        return Ok(Some(StaticOutput {
            nodes,
            globals: Vec::new(),
            property_order: property_order(&["--tw-container-component"]),
            declaration_count: count,
        }));
    }
    let properties = match root {
        "transition-all" => "all",
        "transition-colors" => {
            "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to"
        }
        "transition-opacity" => "opacity",
        "transition-shadow" => "box-shadow",
        "transition-transform" => "transform, translate, scale, rotate",
        _ => return Ok(None),
    };
    let timing = theme
        .resolve_text(&Text::from("--default-transition-timing-function"), used)
        .unwrap_or_else(|| "ease".into());
    let duration = theme
        .resolve_text(&Text::from("--default-transition-duration"), used)
        .unwrap_or_else(|| "0s".into());
    Ok(Some(StaticOutput {
        nodes: vec![
            declaration("transition-property", properties.into()),
            declaration(
                "transition-timing-function",
                concat_text(&[&"var(--tw-ease, ".into(), &timing, &")".into()]),
            ),
            declaration(
                "transition-duration",
                concat_text(&[&"var(--tw-duration, ".into(), &duration, &")".into()]),
            ),
        ],
        globals: Vec::new(),
        property_order: property_order(&[
            "transition-property",
            "transition-timing-function",
            "transition-duration",
        ]),
        declaration_count: 3,
    }))
}

fn compare_breakpoints(left: &Text, right: &Text) -> Ordering {
    fn bucket(value: &Text) -> Text {
        let units = value.units();
        if let Some(index) = units.iter().position(|unit| *unit == 40) {
            Text::from_units(units[..index].to_vec())
        } else {
            Text::from_units(
                units
                    .iter()
                    .copied()
                    .filter(|unit| !matches!(unit, 48..=57 | 46))
                    .collect(),
            )
        }
    }
    let order = bucket(left).units().cmp(bucket(right).units());
    if order != Ordering::Equal {
        return order;
    }
    fn integer(value: &Text) -> Option<f64> {
        let units = value.units();
        let whitespace = |unit: &u16| matches!(*unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279);
        let mut start = units.iter().take_while(|unit| whitespace(unit)).count();
        let negative = units.get(start) == Some(&45);
        if matches!(units.get(start), Some(43 | 45)) {
            start += 1;
        }
        let hex = units.get(start) == Some(&48) && matches!(units.get(start + 1), Some(88 | 120));
        if hex {
            start += 2;
        }
        let digits = &units[start..];
        let digit = |unit: u16| match unit {
            48..=57 => Some(unit - 48),
            65..=70 if hex => Some(unit - 55),
            97..=102 if hex => Some(unit - 87),
            _ => None,
        };
        let count = digits
            .iter()
            .take_while(|unit| digit(**unit).is_some())
            .count();
        if count == 0 {
            return None;
        }
        let number = if hex {
            digits[..count].iter().fold(0.0, |number, unit| {
                number * 16.0 + f64::from(digit(*unit).unwrap_or(0))
            })
        } else {
            // Only proven decimal ASCII digits cross into Rust's number parser.
            digits[..count]
                .iter()
                .map(|unit| char::from(*unit as u8))
                .collect::<String>()
                .parse::<f64>()
                .ok()?
        };
        Some(if negative { -number } else { number })
    }
    match (integer(left), integer(right)) {
        (Some(left_number), Some(right_number)) => (left_number - right_number)
            .partial_cmp(&0.0)
            .unwrap_or_else(|| left.units().cmp(right.units())),
        _ => left.units().cmp(right.units()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn static_container_and_transition_keep_opaque_theme_units() {
        let opaque = Text::from_units(vec![0xd800, 0xfffd, 92, 117, 100, 56, 48, 48]);
        let mut nodes = vec![AstNode::AtRule {
            origin: None,
            name: "@theme".into(),
            params: "inline".into(),
            source: None,
            destination: None,
            nodes: Some(
                [
                    "--breakpoint-card",
                    "--default-transition-timing-function",
                    "--default-transition-duration",
                ]
                .into_iter()
                .map(|name| AstNode::declaration(name, opaque.clone()))
                .collect(),
            ),
        }];
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let mut used = BTreeSet::new();
        let container = compile("container", false, &theme, &mut used)
            .unwrap()
            .unwrap();
        let AstNode::AtRule {
            params,
            nodes: Some(children),
            ..
        } = &container.nodes[1]
        else {
            panic!("expected media rule")
        };
        assert_eq!(
            params,
            &concat_text(&[&"(width >= ".into(), &opaque, &")".into()])
        );
        let AstNode::Declaration { value, .. } = &children[0] else {
            panic!("expected max-width declaration")
        };
        assert_eq!(value, &opaque);
        let transition = compile("transition-all", false, &theme, &mut used)
            .unwrap()
            .unwrap();
        let AstNode::Declaration { value, .. } = &transition.nodes[1] else {
            panic!("expected timing declaration")
        };
        assert_eq!(
            value,
            &concat_text(&[&"var(--tw-ease, ".into(), &opaque, &")".into()])
        );
        let AstNode::Declaration { value, .. } = &transition.nodes[2] else {
            panic!("expected duration declaration")
        };
        assert_eq!(
            value,
            &concat_text(&[&"var(--tw-duration, ".into(), &opaque, &")".into()])
        );
    }

    #[test]
    fn breakpoint_order_uses_original_js_units_and_numeric_prefixes() {
        let cases = [
            (
                Text::from_units(vec![0xd83d, 0xde00]),
                Text::from_units(vec![0xe000]),
                Ordering::Less,
            ),
            (
                Text::from_units(vec![0xd800]),
                Text::from_units(vec![0xfffd]),
                Ordering::Less,
            ),
            ("10rem".into(), "2rem".into(), Ordering::Greater),
            (
                "\u{feff}10rem".into(),
                "\u{feff}2rem".into(),
                Ordering::Greater,
            ),
            ("\u{85}10rem".into(), "\u{85}2rem".into(), Ordering::Less),
            ("0x10rem".into(), "0x2rem".into(), Ordering::Greater),
            ("calc(10rem)".into(), "calc(2rem)".into(), Ordering::Less),
        ];
        for (left, right, expected) in cases {
            assert_eq!(compare_breakpoints(&left, &right), expected);
        }
    }
}
