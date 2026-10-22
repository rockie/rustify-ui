//! Structural static-variant templates preserve sibling and nested branches.

use super::{
    ast::AstNode, candidate::Variant, dependencies, program::GpuCondition, registry, theme::Theme,
    value, DiagnosticKind,
};
use crate::text::Text;
use std::cmp::Ordering;

#[path = "../../upstream/tailwind/variant-static-data.rs"]
mod data;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VariantKind {
    Static,
    Functional,
    Compound,
}

fn root_label(root: &Text) -> &'static str {
    registry::VARIANT_ROOTS
        .binary_search_by(|name| name.encode_utf16().cmp(root.units().iter().copied()))
        .map(|index| registry::VARIANT_ROOTS[index])
        .unwrap_or("")
}

fn static_index(root: &Text) -> Result<usize, usize> {
    data::STATIC_VARIANTS
        .binary_search_by(|(name, _)| name.encode_utf16().cmp(root.units().iter().copied()))
}

fn builtin_rank(root: &Text) -> Option<usize> {
    registry::VARIANT_ORDER
        .iter()
        .find_map(|(name, rank)| (root == *name).then_some(*rank))
}

fn breakpoint_override(root: &Text, theme: &Theme) -> bool {
    root != "min"
        && builtin_rank(root).is_none_or(|rank| rank <= 64)
        && theme
            .tokens
            .contains_key(&value::concat_text(&[&"--breakpoint-".into(), root]))
}

pub(super) fn root_kind(root: &Text, theme: &Theme) -> Option<VariantKind> {
    if theme.custom_variants.contains_root_text(root)
        || breakpoint_override(root, theme)
        || is_static_root(root)
    {
        Some(VariantKind::Static)
    } else if matches!(root_label(root), "not" | "group" | "peer" | "in" | "has") {
        Some(VariantKind::Compound)
    } else if matches!(
        root_label(root),
        "aria"
            | "data"
            | "nth"
            | "nth-last"
            | "nth-of-type"
            | "nth-last-of-type"
            | "supports"
            | "min"
            | "max"
            | "@"
            | "@min"
            | "@max"
    ) {
        Some(VariantKind::Functional)
    } else {
        None
    }
}

pub(super) fn selector_compounds(selector: &Text) -> u8 {
    if selector.starts_with("@") {
        if selector.starts_with("@media")
            || selector.starts_with("@supports")
            || selector.starts_with("@container")
        {
            1
        } else {
            0
        }
    } else if selector.contains("::") {
        0
    } else {
        2
    }
}

pub(super) fn compounds(variant: &Variant, theme: &Theme) -> u8 {
    let Variant::Registered { root, .. } = variant else {
        let Variant::Arbitrary { selector, .. } = variant else {
            unreachable!()
        };
        return selector_compounds(selector);
    };
    if let Some(compounds) = theme.custom_variants.compounds_text(root) {
        return compounds;
    }
    if breakpoint_override(root, theme) {
        return 1;
    }
    if matches!(root_label(root), "*" | "**" | "before" | "after") {
        return 0;
    }
    if let Ok(index) = static_index(root) {
        fn visit(nodes: &[VariantTemplate], flags: &mut u8) -> bool {
            for node in nodes {
                let (selector, children) = match node {
                    VariantTemplate::Slot => continue,
                    VariantTemplate::Rule(selector, children) => {
                        ((*selector).to_string(), *children)
                    }
                    VariantTemplate::AtRule(name, params, children) => {
                        (format!("{name} {params}"), *children)
                    }
                };
                let compounds = selector_compounds(&Text::from(selector));
                if compounds == 0 || !visit(children, flags) {
                    return false;
                }
                *flags |= compounds;
            }
            true
        }
        let mut flags = 0;
        return if visit(data::STATIC_VARIANTS[index].1, &mut flags) {
            flags
        } else {
            0
        };
    }
    if matches!(
        root_label(root),
        "supports" | "min" | "max" | "@" | "@min" | "@max"
    ) {
        1
    } else {
        2
    }
}

pub enum VariantTemplate {
    Slot,
    Rule(&'static str, &'static [VariantTemplate]),
    AtRule(&'static str, &'static str, &'static [VariantTemplate]),
}

fn materialize(template: &[VariantTemplate], slot: &[AstNode]) -> Vec<AstNode> {
    let mut nodes = Vec::new();
    for node in template {
        match node {
            VariantTemplate::Slot => nodes.extend_from_slice(slot),
            VariantTemplate::Rule(selector, children) => nodes.push(AstNode::Rule {
                origin: None,
                source: None,
                destination: None,
                selector: (*selector).into(),
                nodes: materialize(children, slot),
            }),
            VariantTemplate::AtRule(name, params, children) => nodes.push(AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: (*name).into(),
                params: (*params).into(),
                nodes: Some(materialize(children, slot)),
            }),
        }
    }
    nodes
}

pub fn apply(
    root: &str,
    mut nodes: Vec<AstNode>,
    _globals: &mut Vec<AstNode>,
) -> Option<(Vec<AstNode>, GpuCondition)> {
    if matches!(root, "before" | "after") {
        nodes.insert(
            0,
            AstNode::AtRoot {
                origin: None,
                nodes: vec![dependencies::content_property(true)],
            },
        );
        nodes.insert(1, AstNode::declaration("content", "var(--tw-content)"));
        let selector = format!("&::{root}");
        return Some((
            vec![AstNode::Rule {
                origin: None,
                source: None,
                destination: None,
                selector: selector.clone().into(),
                nodes,
            }],
            GpuCondition::Selector {
                selector: selector.into(),
            },
        ));
    }
    let index = data::STATIC_VARIANTS
        .binary_search_by_key(&root, |(name, _)| *name)
        .ok()?;
    let template = data::STATIC_VARIANTS[index].1;
    let condition = match root {
        "hover" => GpuCondition::Hover,
        "active" => GpuCondition::Active,
        "focus" => GpuCondition::Focus,
        "focus-visible" => GpuCondition::FocusVisible,
        "disabled" => GpuCondition::Disabled,
        "dark" => GpuCondition::Dark,
        _ => match template {
            [VariantTemplate::Rule(selector, _)] => GpuCondition::Selector {
                selector: (*selector).into(),
            },
            [VariantTemplate::AtRule("@media", query, _)] => GpuCondition::Media {
                query: (*query).into(),
            },
            _ => GpuCondition::Unsupported {
                feature: format!("{root} variant branches/at-rule"),
            },
        },
    };
    Some((materialize(template, &nodes), condition))
}

#[derive(Clone, Debug)]
pub struct SortKey {
    rank: usize,
    detail: SortDetail,
}
#[derive(Clone, Debug)]
enum SortDetail {
    Static,
    Value {
        root: Text,
        arbitrary: bool,
        value: Option<Text>,
    },
    Compound {
        child: Box<SortKey>,
        modifier: Option<Text>,
    },
    Breakpoint {
        value: Text,
        descending: bool,
    },
    Selector(Text),
}
impl PartialEq for SortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for SortKey {}
impl PartialOrd for SortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank
            .cmp(&other.rank)
            .then_with(|| match (&self.detail, &other.detail) {
                (SortDetail::Static, SortDetail::Static) => Ordering::Equal,
                (
                    SortDetail::Value {
                        root: a,
                        arbitrary: aa,
                        value: av,
                    },
                    SortDetail::Value {
                        root: b,
                        arbitrary: ba,
                        value: bv,
                    },
                ) => text_compare(a, b)
                    .then_with(|| aa.cmp(ba))
                    .then_with(|| optional_text_compare(av, bv)),
                (
                    SortDetail::Compound {
                        child: a,
                        modifier: am,
                    },
                    SortDetail::Compound {
                        child: b,
                        modifier: bm,
                    },
                ) => a.cmp(b).then_with(|| optional_text_compare(am, bm)),
                (
                    SortDetail::Breakpoint {
                        value: a,
                        descending,
                    },
                    SortDetail::Breakpoint { value: b, .. },
                ) => compare_breakpoints(a, b, *descending),
                (SortDetail::Selector(a), SortDetail::Selector(b)) => a.units().cmp(b.units()),
                _ => Ordering::Equal,
            })
    }
}
fn text_compare(a: &Text, b: &Text) -> Ordering {
    a.units().cmp(b.units())
}
fn optional_text_compare(a: &Option<Text>, b: &Option<Text>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => text_compare(a, b),
        _ => a.is_some().cmp(&b.is_some()),
    }
}
pub(super) fn compare_breakpoints(left: &Text, right: &Text, descending: bool) -> Ordering {
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
        (Some(left_number), Some(right_number)) => (if descending {
            right_number - left_number
        } else {
            left_number - right_number
        })
        .partial_cmp(&0.0)
        .unwrap_or_else(|| left.units().cmp(right.units())),
        _ => left.units().cmp(right.units()),
    }
}

pub struct VariantOutput {
    pub nodes: Vec<AstNode>,
    pub condition: GpuCondition,
    pub order: SortKey,
}
pub(super) enum Failure {
    Candidate(DiagnosticKind, Text),
    Resource(super::CompileError),
}
fn rule(selector: Text, nodes: Vec<AstNode>) -> Vec<AstNode> {
    vec![AstNode::Rule {
        origin: None,
        source: None,
        destination: None,
        selector,
        nodes,
    }]
}
fn at_rule(name: &str, params: Text, nodes: Vec<AstNode>) -> Vec<AstNode> {
    vec![AstNode::AtRule {
        origin: None,
        source: None,
        destination: None,
        name: name.into(),
        params,
        nodes: Some(nodes),
    }]
}
pub(super) fn decoded(
    value: &Text,
    depth: usize,
) -> Result<Option<(Text, bool)>, super::CompileError> {
    if let Some(inner) = value
        .strip_prefix("[")
        .and_then(|value| value.strip_suffix("]"))
    {
        let value = value::decode_arbitrary_text(&inner, depth)?;
        Ok(
            (!value.trim_js().is_empty() && value::is_valid_arbitrary_text(&value))
                .then_some((value, true)),
        )
    } else if let Some(variable) = value
        .strip_prefix("(")
        .and_then(|value| value.strip_suffix(")"))
    {
        if !variable.starts_with("--") || !value::is_valid_arbitrary_text(&variable) {
            return Ok(None);
        }
        Ok(Some((
            value::decode_arbitrary_text(
                &value::concat_text(&[&"var(".into(), &variable, &")".into()]),
                depth,
            )?,
            true,
        )))
    } else if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some((value.clone(), false)))
    }
}

fn join_text(parts: &[Text], separator: &str) -> Text {
    let mut output = Text::default();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            output.push_str(separator);
        }
        output.push_text(part);
    }
    output
}

fn quote_attribute(input: &Text, depth: usize) -> Result<Text, super::CompileError> {
    if !input.contains("=") {
        return Ok(input.clone());
    }
    let parts = value::segment_text(input, 61, depth)?;
    if parts.len() < 2 {
        return Ok(input.clone());
    }
    let raw = join_text(&parts[1..], "=");
    let value = raw.trim_js();
    let units = value.units();
    if matches!(units.first(), Some(39 | 34)) {
        return Ok(input.clone());
    }
    if units.len() > 1
        && units[units.len() - 2] == 32
        && matches!(units[units.len() - 1], 105 | 73 | 115 | 83)
    {
        Ok(value::concat_text(&[
            &parts[0],
            &"=\"".into(),
            &Text::from_units(units[..units.len() - 2].to_vec()),
            &"\" ".into(),
            &Text::from_units(units[units.len() - 1..].to_vec()),
        ]))
    } else {
        Ok(value::concat_text(&[
            &parts[0],
            &"=\"".into(),
            &value,
            &"\"".into(),
        ]))
    }
}

fn canonical_integer(value: &Text) -> bool {
    if !value.units().iter().all(|unit| *unit <= 0x7f) {
        return false;
    }
    let scalar: String = value
        .units()
        .iter()
        .map(|unit| char::from(*unit as u8))
        .collect();
    super::colors::canonical_nonnegative(&scalar, true)
}

fn authored_variant(
    params: &Text,
    theme: &Theme,
    limits: &super::Limits,
) -> Result<Variant, super::CompileError> {
    super::candidate::parse_variant_with_theme(params, limits.ast_depth, theme)?.ok_or_else(|| {
        super::CompileError::Syntax {
            offset: 0,
            message: value::concat_text(&[
                &"Cannot use `@variant` with unknown variant: ".into(),
                params,
            ]),
        }
    })
}

pub(super) fn apply_authored_variant(
    params: &Text,
    nodes: Vec<AstNode>,
    theme: &Theme,
    limits: &super::Limits,
    raw_slot: Option<&mut super::custom_variants::RawSlot<'_>>,
) -> Result<Vec<AstNode>, super::CompileError> {
    let variant = authored_variant(params, theme, limits)?;
    let mut globals = Vec::new();
    let mut output = compile_inner(
        &variant,
        nodes,
        &mut globals,
        theme,
        limits,
        ApplyContext {
            depth: 0,
            allow_relative: false,
            raw_slot,
        },
    )
    .map_err(|failure| match failure {
        Failure::Resource(error) => error,
        Failure::Candidate(_, _) => super::CompileError::Syntax {
            offset: 0,
            message: value::concat_text(&[&"Cannot use `@variant` with variant: ".into(), params]),
        },
    })?
    .nodes;
    if !globals.is_empty() {
        output.push(AstNode::AtRoot {
            origin: None,
            nodes: globals,
        });
    }
    Ok(output)
}

pub(super) fn is_static_root(root: &Text) -> bool {
    matches!(root_label(root), "before" | "after") || static_index(root).is_ok()
}

pub fn compile_variant(
    variant: &Variant,
    nodes: Vec<AstNode>,
    globals: &mut Vec<AstNode>,
    theme: &Theme,
    limits: &super::Limits,
) -> Result<VariantOutput, Failure> {
    compile_inner(
        variant,
        nodes,
        globals,
        theme,
        limits,
        ApplyContext {
            depth: 0,
            allow_relative: false,
            raw_slot: None,
        },
    )
}
struct ApplyContext<'a, 'b> {
    depth: usize,
    allow_relative: bool,
    raw_slot: Option<&'a mut super::custom_variants::RawSlot<'b>>,
}

fn compile_inner(
    variant: &Variant,
    nodes: Vec<AstNode>,
    globals: &mut Vec<AstNode>,
    theme: &Theme,
    limits: &super::Limits,
    context: ApplyContext<'_, '_>,
) -> Result<VariantOutput, Failure> {
    let ApplyContext {
        depth,
        allow_relative,
        mut raw_slot,
    } = context;
    let max_depth = limits.ast_depth;
    if depth > max_depth {
        return Err(Failure::Resource(super::CompileError::Limit {
            resource: "ast_depth".into(),
            limit: max_depth,
        }));
    }
    let Variant::Registered {
        root,
        value: raw_value,
        modifier,
        ..
    } = variant
    else {
        let Variant::Arbitrary { selector, relative } = variant else {
            unreachable!()
        };
        if *relative && !allow_relative {
            return Err(Failure::Candidate(
                DiagnosticKind::InvalidValue,
                "arbitrary variant".into(),
            ));
        }
        let (output, condition) = if selector.starts_with("@") {
            let node = super::parser::at_rule_text(selector, Some(nodes));
            let condition = match &node {
                AstNode::AtRule { name, params, .. } if name == "@media" => GpuCondition::Media {
                    query: params.clone(),
                },
                _ => GpuCondition::Selector {
                    selector: selector.clone(),
                },
            };
            (vec![node], condition)
        } else {
            (
                rule(selector.clone(), nodes),
                GpuCondition::Selector {
                    selector: selector.clone(),
                },
            )
        };
        return Ok(VariantOutput {
            nodes: output,
            condition,
            order: SortKey {
                rank: usize::MAX,
                detail: SortDetail::Selector(selector.clone()),
            },
        });
    };
    let invalid = || Failure::Candidate(DiagnosticKind::InvalidValue, root.clone());
    let rank = registry::VARIANT_ORDER
        .iter()
        .find(|(name, _)| root == *name)
        .map_or(64, |(_, rank)| *rank);
    let mut order = SortKey {
        rank,
        detail: SortDetail::Static,
    };
    if theme.custom_variants.contains_root_text(root) {
        if raw_value.is_some() || modifier.is_some() {
            return Err(invalid());
        }
        if !registry::VARIANT_ORDER
            .iter()
            .any(|(name, _)| root == *name)
        {
            let registered = theme
                .custom_variants
                .root_names()
                .filter(|name| {
                    !registry::VARIANT_ORDER
                        .iter()
                        .any(|(builtin, _)| builtin == name)
                })
                .position(|name| root == name)
                .unwrap();
            order.rank = registry::VARIANT_ORDER
                .iter()
                .map(|(_, rank)| *rank)
                .max()
                .unwrap_or(0)
                + 1
                + registered;
        }
        let output = theme
            .custom_variants
            .apply_text_current(root, &nodes, limits, raw_slot.as_deref_mut())
            .map_err(Failure::Resource)?
            .unwrap();
        globals.extend(output.globals);
        return Ok(VariantOutput {
            nodes: output.nodes,
            condition: GpuCondition::Unsupported {
                feature: format!(
                    "{} custom variant",
                    theme
                        .custom_variants
                        .root_names()
                        .find(|name| root == *name)
                        .unwrap()
                ),
            },
            order,
        });
    }
    let static_breakpoint = breakpoint_override(root, theme);
    let breakpoint = if static_breakpoint && raw_value.is_none() && modifier.is_none() {
        theme
            .tokens
            .get(&value::concat_text(&[&"--breakpoint-".into(), root]))
            .map(|token| token.value.clone())
    } else if matches!(root_label(root), "min" | "max" | "@" | "@min" | "@max")
        && !static_breakpoint
    {
        let (value, arbitrary) = decoded(raw_value.as_ref().ok_or_else(invalid)?, max_depth)
            .map_err(Failure::Resource)?
            .ok_or_else(invalid)?;
        let namespace = if root.starts_with("@") {
            "container"
        } else {
            "breakpoint"
        };
        let result = if arbitrary {
            Some(value)
        } else {
            theme
                .tokens
                .get(&value::concat_text(&[
                    &format!("--{namespace}-").into(),
                    &value,
                ]))
                .map(|token| token.value.clone())
        };
        let result = result
            .filter(|value| !value.is_empty() && !value.contains("var("))
            .ok_or_else(invalid)?;
        if modifier.is_some() && !root.starts_with("@") {
            return Err(invalid());
        }
        Some(result)
    } else {
        None
    };
    if let Some(width) = breakpoint {
        let descending = !static_breakpoint && matches!(root_label(root), "max" | "@max");
        order.rank = if static_breakpoint {
            builtin_rank(root).unwrap_or(64)
        } else if root.starts_with("@") {
            if descending {
                65
            } else {
                66
            }
        } else if descending {
            63
        } else {
            64
        };
        if order.rank >= 63 {
            order.detail = SortDetail::Breakpoint {
                value: width.clone(),
                descending: order.rank == 63 || order.rank == 65,
            };
        }
        let query = value::concat_text(&[
            &format!("(width {} ", if descending { "<" } else { ">=" }).into(),
            &width,
            &")".into(),
        ]);
        let (name, params, condition) = if root.starts_with("@") {
            let params = modifier.as_ref().map_or(query.clone(), |name| {
                value::concat_text(&[name, &" ".into(), &query])
            });
            (
                "@container",
                params,
                GpuCondition::Unsupported {
                    feature: "container query".into(),
                },
            )
        } else {
            ("@media", query.clone(), GpuCondition::Media { query })
        };
        return Ok(VariantOutput {
            nodes: at_rule(name, params, nodes),
            condition,
            order,
        });
    }
    if matches!(root_label(root), "not" | "group" | "peer" | "in" | "has") {
        if modifier.is_some() && !matches!(root_label(root), "group" | "peer") {
            return Err(invalid());
        }
        let child = super::candidate::parse_variant_with_theme(
            raw_value.as_ref().ok_or_else(invalid)?,
            max_depth,
            theme,
        )
        .map_err(Failure::Resource)?
        .ok_or_else(invalid)?;
        if matches!(&child,Variant::Registered {root,..} if matches!(root_label(root),"*"|"**")) {
            return Err(invalid());
        }
        if let Variant::Registered {
            root: child_root, ..
        } = &child
        {
            if let Some(compounds) = theme.custom_variants.compounds_text(child_root) {
                let allowed = if root == "not" { 3 } else { 2 };
                if compounds & allowed == 0 {
                    return Err(invalid());
                }
            }
        }
        let child_output = compile_inner(
            &child,
            Vec::new(),
            globals,
            theme,
            limits,
            ApplyContext {
                depth: depth + 1,
                allow_relative: matches!(root_label(root), "in" | "has"),
                raw_slot,
            },
        )?;
        order.detail = SortDetail::Compound {
            child: Box::new(child_output.order),
            modifier: modifier.clone(),
        };
        let output = compound(
            root,
            modifier.as_ref(),
            child_output.nodes,
            nodes,
            max_depth,
            theme.prefix.as_deref(),
        )
        .map_err(Failure::Resource)?
        .ok_or_else(invalid)?;
        return Ok(VariantOutput {
            nodes: output,
            condition: GpuCondition::Unsupported {
                feature: format!("{} relation variant", root_label(root)),
            },
            order,
        });
    }
    if raw_value.is_none() && modifier.is_none() {
        if let Some((nodes, condition)) = apply(root_label(root), nodes.clone(), globals) {
            if builtin_rank(root).is_some_and(|rank| rank > 64) {
                if let Some(token) = theme
                    .tokens
                    .get(&value::concat_text(&[&"--breakpoint-".into(), root]))
                {
                    order.rank = 64;
                    order.detail = SortDetail::Breakpoint {
                        value: token.value.clone(),
                        descending: false,
                    };
                }
            }
            return Ok(VariantOutput {
                nodes,
                condition,
                order,
            });
        }
    }
    if matches!(
        root_label(root),
        "aria" | "data" | "nth" | "nth-last" | "nth-of-type" | "nth-last-of-type" | "supports"
    ) {
        if modifier.is_some() {
            return Err(invalid());
        }
        let (value, arbitrary) = decoded(raw_value.as_ref().ok_or_else(invalid)?, max_depth)
            .map_err(Failure::Resource)?
            .ok_or_else(invalid)?;
        order.detail = SortDetail::Value {
            root: root.clone(),
            arbitrary,
            value: Some(value.clone()),
        };
        let (output, condition) = if matches!(root_label(root), "aria" | "data") {
            let value = if root == "aria" && !arbitrary {
                value::concat_text(&[&value, &"=\"true\"".into()])
            } else {
                quote_attribute(&value, max_depth).map_err(Failure::Resource)?
            };
            let selector =
                value::concat_text(&[&"&[".into(), root, &"-".into(), &value, &"]".into()]);
            (
                rule(selector.clone(), nodes),
                GpuCondition::Selector { selector },
            )
        } else if root.starts_with("nth") {
            if !arbitrary && !canonical_integer(&value) {
                return Err(invalid());
            }
            let suffix = match root_label(root) {
                "nth" => "child",
                "nth-last" => "last-child",
                "nth-of-type" => "of-type",
                _ => "last-of-type",
            };
            let selector =
                value::concat_text(&[&format!("&:nth-{suffix}(").into(), &value, &")".into()]);
            (
                rule(selector.clone(), nodes),
                GpuCondition::Selector { selector },
            )
        } else {
            let function = value
                .units()
                .iter()
                .position(|unit| *unit == 40)
                .is_some_and(|index| {
                    let prefix = &value.units()[..index];
                    let end = prefix
                        .iter()
                        .take_while(|unit| matches!(**unit, 65..=90 | 97..=122 | 48..=57 | 95 | 45))
                        .count();
                    Text::from_units(prefix[end..].to_vec())
                        .trim_js()
                        .is_empty()
                });
            let query = if function {
                let mut query = Text::default();
                let mut start = 0;
                for (index, unit) in value.units().iter().enumerate() {
                    if !matches!(*unit, 65..=90 | 97..=122 | 48..=57 | 95) {
                        let word = Text::from_units(value.units()[start..index].to_vec());
                        if ["and", "or", "not"].iter().any(|label| word == *label) {
                            query.push_str(" ");
                            query.push_text(&word);
                            query.push_str(" ");
                        } else {
                            query.push_text(&word);
                        }
                        query.push_text(&Text::from_units(vec![*unit]));
                        start = index + 1;
                    }
                }
                let word = Text::from_units(value.units()[start..].to_vec());
                if ["and", "or", "not"].iter().any(|label| word == *label) {
                    query.push_str(" ");
                    query.push_text(&word);
                    query.push_str(" ");
                } else {
                    query.push_text(&word);
                }
                query
            } else {
                let value = if value.contains(":") {
                    value
                } else {
                    value::concat_text(&[&value, &": var(--tw)".into()])
                };
                if value.starts_with("(") && value.ends_with(")") {
                    value
                } else {
                    value::concat_text(&[&"(".into(), &value, &")".into()])
                }
            };
            (
                at_rule("@supports", query, nodes),
                GpuCondition::Unsupported {
                    feature: "supports condition".into(),
                },
            )
        };
        return Ok(VariantOutput {
            nodes: output,
            condition,
            order,
        });
    }
    if !root_label(root).is_empty() {
        Err(invalid())
    } else {
        Err(Failure::Candidate(
            DiagnosticKind::PendingVariant,
            root.clone(),
        ))
    }
}

fn replace_ampersands(input: &Text, replacement: &Text) -> Text {
    let mut units = Vec::new();
    for unit in input.units() {
        if *unit == 38 {
            units.extend_from_slice(replacement.units());
        } else {
            units.push(*unit);
        }
    }
    Text::from_units(units)
}

fn compound(
    root: &Text,
    modifier: Option<&Text>,
    mut skeleton: Vec<AstNode>,
    slot: Vec<AstNode>,
    depth: usize,
    prefix: Option<&str>,
) -> Result<Option<Vec<AstNode>>, super::CompileError> {
    fn fill(nodes: &mut Vec<AstNode>, slot: &[AstNode]) {
        if nodes.is_empty() {
            *nodes = slot.to_vec();
            return;
        }
        for node in nodes {
            if let Some(children) = node.children_mut() {
                fill(children, slot)
            }
        }
    }
    if root == "not" {
        if skeleton.len() > 1 {
            return Ok(None);
        }
        fn negate(
            nodes: &[AstNode],
            rules: &mut Vec<crate::text::Text>,
            at_rules: &mut Vec<(crate::text::Text, crate::text::Text)>,
            out: &mut Vec<AstNode>,
            depth: usize,
        ) -> Result<bool, super::CompileError> {
            if nodes.is_empty() {
                if rules.len() > 1 || at_rules.len() > 1 {
                    return Ok(false);
                }
                for selector in rules.iter() {
                    if selector.contains("::") {
                        return Ok(false);
                    }
                    let parts = value::segment_text(selector, u16::from(b','), depth)?;
                    let mut joined = crate::text::Text::from("&:not(");
                    for (index, part) in parts.iter().enumerate() {
                        if index > 0 {
                            joined.push_str(", ");
                        }
                        joined.push_text(&part.replace_ascii(b'&', "*"));
                    }
                    joined.push_str(")");
                    out.push(AstNode::Rule {
                        origin: None,
                        source: None,
                        destination: None,
                        selector: joined,
                        nodes: Vec::new(),
                    });
                }
                for (name, params) in at_rules.iter() {
                    if !["@media", "@supports", "@container"]
                        .iter()
                        .any(|label| name == label)
                        || value::segment_text(params, u16::from(b','), depth)?.len() > 1
                    {
                        return Ok(false);
                    }
                    let params = params.trim_js();
                    let negated = if params.starts_with("not ") {
                        crate::text::Text::from_units(params.units()[4..].to_vec())
                    } else if name == "@container" && !params.starts_with("(") {
                        let Some(space) = params.units().iter().position(|unit| *unit == 32) else {
                            return Ok(false);
                        };
                        let query =
                            crate::text::Text::from_units(params.units()[space + 1..].to_vec());
                        let mut negated =
                            crate::text::Text::from_units(params.units()[..space + 1].to_vec());
                        if query.starts_with("not ") {
                            negated.push_text(&crate::text::Text::from_units(
                                query.units()[4..].to_vec(),
                            ));
                        } else {
                            negated.push_str("not ");
                            negated.push_text(&query);
                        }
                        negated
                    } else {
                        let mut negated = crate::text::Text::from("not ");
                        negated.push_text(&params);
                        negated
                    };
                    out.push(AstNode::AtRule {
                        origin: None,
                        source: None,
                        destination: None,
                        name: name.clone(),
                        params: negated,
                        nodes: Some(Vec::new()),
                    });
                }
                return Ok(true);
            }
            for node in nodes {
                match node {
                    AstNode::Rule {
                        selector, nodes, ..
                    } => {
                        rules.push(selector.clone());
                        if !negate(nodes, rules, at_rules, out, depth)? {
                            return Ok(false);
                        }
                        rules.pop();
                    }
                    AstNode::AtRule {
                        name,
                        params,
                        nodes: Some(nodes),
                        ..
                    } => {
                        at_rules.push((name.clone(), params.clone()));
                        if !negate(nodes, rules, at_rules, out, depth)? {
                            return Ok(false);
                        }
                        at_rules.pop();
                    }
                    _ => return Ok(false),
                }
            }
            Ok(true)
        }
        let mut output = Vec::new();
        let applied = negate(
            &skeleton,
            &mut Vec::new(),
            &mut Vec::new(),
            &mut output,
            depth,
        )?;
        if !applied || output.is_empty() {
            return Ok(None);
        }
        fill(&mut output, &slot);
        return Ok(Some(output));
    }
    fn transform(
        nodes: &mut [AstNode],
        root: &Text,
        modifier: Option<&Text>,
        style_depth: usize,
        applied: &mut bool,
        depth: usize,
        prefix: Option<&str>,
    ) -> Result<bool, super::CompileError> {
        for node in nodes {
            match node {
                AstNode::Rule {
                    selector, nodes, ..
                } => {
                    if style_depth > 0 || selector.contains("::") {
                        return Ok(false);
                    }
                    *selector = match root_label(root) {
                        "group" | "peer" => {
                            let mut replacement = Text::from(":where(.");
                            if let Some(prefix) = prefix {
                                replacement.push_str(prefix);
                                replacement.push_str("\\:");
                            }
                            replacement.push_text(root);
                            if let Some(name) = modifier {
                                replacement.push_str("\\/");
                                replacement.push_text(name);
                            }
                            replacement.push_str(")");
                            let selector = replace_ampersands(selector, &replacement);
                            let selector = if value::segment_text(
                                &selector,
                                u16::from(b','),
                                depth,
                            )?
                            .len()
                                > 1
                            {
                                let mut wrapped = crate::text::Text::from(":is(");
                                wrapped.push_text(&selector);
                                wrapped.push_str(")");
                                wrapped
                            } else {
                                selector
                            };
                            let mut wrapped = crate::text::Text::from("&:is(");
                            wrapped.push_text(&selector);
                            wrapped.push_str(if root == "peer" { " ~ *)" } else { " *)" });
                            wrapped
                        }
                        "in" | "has" => {
                            let mut wrapped = crate::text::Text::from(if root == "in" {
                                ":where("
                            } else {
                                "&:has("
                            });
                            wrapped.push_text(&selector.replace_ascii(b'&', "*"));
                            wrapped.push_str(if root == "in" { ") &" } else { ")" });
                            wrapped
                        }
                        _ => return Ok(false),
                    };
                    *applied = true;
                    if !transform(
                        nodes,
                        root,
                        modifier,
                        style_depth + 1,
                        applied,
                        depth,
                        prefix,
                    )? {
                        return Ok(false);
                    }
                }
                AstNode::AtRule {
                    nodes: Some(nodes), ..
                } => {
                    if !transform(nodes, root, modifier, style_depth, applied, depth, prefix)? {
                        return Ok(false);
                    }
                }
                AstNode::Context { nodes, .. } => {
                    if !transform(nodes, root, modifier, style_depth, applied, depth, prefix)? {
                        return Ok(false);
                    }
                }
                _ => {}
            }
        }
        Ok(true)
    }
    for node in &mut skeleton {
        if !matches!(node, AstNode::Rule { .. } | AstNode::AtRule { .. }) {
            return Ok(None);
        }
        let mut applied = false;
        let transformed = transform(
            std::slice::from_mut(node),
            root,
            modifier,
            0,
            &mut applied,
            depth,
            prefix,
        )?;
        if !transformed || !applied {
            return Ok(None);
        }
    }
    fill(&mut skeleton, &slot);
    Ok(Some(skeleton))
}

#[cfg(test)]
mod text_tests {
    use super::super::CompileError;
    use super::*;

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
    fn decoded_and_attribute_quotes_preserve_units_and_propagate_depth_limits() {
        for raw in [
            &[0xd800][..],
            &[0xdc00],
            &[0xd83d, 0xde00],
            &[0xfffd],
            &[0xe000],
        ] {
            assert_eq!(
                decoded(&units("[x_", raw, "]"), 32)
                    .unwrap()
                    .unwrap()
                    .0
                    .units(),
                units("x ", raw, "").units()
            );
            assert_eq!(
                quote_attribute(&units("state=", raw, " i"), 32)
                    .unwrap()
                    .units(),
                units("state=\"", raw, "\" i").units()
            );
            assert_eq!(
                quote_attribute(&units("state=\u{feff}", raw, "\u{feff}"), 32)
                    .unwrap()
                    .units(),
                units("state=\"", raw, "\"").units()
            );
            assert_eq!(
                quote_attribute(&units("state=\u{85}", raw, "\u{85}"), 32)
                    .unwrap()
                    .units(),
                units("state=\"\u{85}", raw, "\u{85}\"").units()
            );
        }
        assert!(matches!(
            decoded(&"[calc(1)]".into(), 0),
            Err(CompileError::Limit { .. })
        ));
        assert_eq!(quote_attribute(&"fn(x)".into(), 0).unwrap(), "fn(x)");
        assert!(matches!(
            quote_attribute(&"state=fn(x)".into(), 0),
            Err(CompileError::Limit { .. })
        ));
    }

    #[test]
    fn breakpoint_sort_matches_js_buckets_numbers_direction_and_nan_fallback() {
        for (left, right, ascending) in [
            ("1.2px", "1.9px", Ordering::Equal),
            ("9007199254740992px", "9007199254740993px", Ordering::Equal),
            ("0x10", "0x20", Ordering::Less),
            ("\u{feff}20px", "\u{feff}3px", Ordering::Greater),
            ("\u{85}20px", "\u{85}3px", Ordering::Less),
            ("calc(1rem)", "calc(2rem)", Ordering::Less),
            ("1em", "2px", Ordering::Less),
        ] {
            let left = Text::from(left);
            let right = Text::from(right);
            assert_eq!(compare_breakpoints(&left, &right, false), ascending);
            let numeric_same_bucket =
                !left.contains("calc(") && left != "\u{85}20px" && left != "1em";
            assert_eq!(
                compare_breakpoints(&left, &right, true),
                if numeric_same_bucket {
                    ascending.reverse()
                } else {
                    ascending
                }
            );
        }
        assert_eq!(
            compare_breakpoints(
                &units("20", &[0xd800], ""),
                &units("3", &[0xd800], ""),
                false
            ),
            Ordering::Greater
        );
        // UTF-16 sorts the high unit before BMP private-use; UTF-8 scalar ordering differs.
        assert_eq!(
            compare_breakpoints(
                &units("1", &[0xd83d, 0xde00], ""),
                &units("1", &[0xe000], ""),
                true
            ),
            Ordering::Less
        );
        let huge_a = Text::from(format!("{}px", "9".repeat(400)));
        let huge_b = Text::from(format!("{}px", "8".repeat(400)));
        assert_eq!(
            compare_breakpoints(&huge_a, &huge_b, false),
            Ordering::Greater
        );
        assert_eq!(
            compare_breakpoints(&huge_a, &huge_b, true),
            Ordering::Greater
        );
    }

    #[test]
    fn compound_units_escape_marker_and_propagate_segment_resource_errors() {
        let selector = units("&:", &[0xd800], "");
        let skeleton = vec![AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector,
            nodes: Vec::new(),
        }];
        let output = compound(
            &"group".into(),
            Some(&Text::from_units(vec![0xdc00])),
            skeleton,
            Vec::new(),
            32,
            Some("tw"),
        )
        .unwrap()
        .unwrap();
        let AstNode::Rule { selector, .. } = &output[0] else {
            panic!("expected rule");
        };
        let mut expected = units("&:is(:where(.tw\\:group\\/", &[0xdc00], "):");
        expected.push_text(&Text::from_units(vec![0xd800]));
        expected.push_str(" *)");
        assert_eq!(selector.units(), expected.units());
        let nested = vec![AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: "&:is(:is(x))".into(),
            nodes: Vec::new(),
        }];
        for root in ["group", "not"] {
            assert!(matches!(
                compound(&root.into(), None, nested.clone(), Vec::new(), 0, None),
                Err(CompileError::Limit { .. })
            ));
        }
    }

    #[test]
    fn authored_unknown_variant_error_retains_original_units() {
        let params = units("unknown-", &[0xd800], "");
        let limits = super::super::Limits {
            input_bytes: 65536,
            asset_bytes: 65536,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 65536,
            cache_bytes: 1048576,
        };
        let error = authored_variant(&params, &Theme::default(), &limits).unwrap_err();
        let CompileError::Syntax { message, .. } = error else {
            panic!("expected syntax error");
        };
        assert_eq!(
            message.units(),
            units(
                "Cannot use `@variant` with unknown variant: unknown-",
                &[0xd800],
                ""
            )
            .units()
        );
    }
}

#[cfg(test)]
mod tests {
    #[cfg(rustify_repo_tests)]
    use crate::compiler::{Compiler, CompilerInput, Limits};

    #[test]
    #[cfg(rustify_repo_tests)]
    fn dynamic_variants_match_official_css_and_union_order() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/dynamic-variant-fixtures.json"
        ))
        .unwrap();
        let mut differences = Vec::new();
        for item in fixture["cases"].as_array().unwrap() {
            let mut compiler = Compiler::new(
                CompilerInput {
                    css: item["input"].as_str().unwrap().into(),
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
            let candidates = item["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            if prepared.report().css != item["expectedCss"].as_str().unwrap()
                || !prepared.report().unsupported.is_empty()
            {
                differences.push(format!(
                    "{}\nactual: {:?}\nexpected: {:?}\npending: {:?}",
                    item["id"],
                    prepared.report().css,
                    item["expectedCss"],
                    prepared.report().unsupported
                ));
            }
        }
        assert!(
            differences.is_empty(),
            "{} differences:\n{}",
            differences.len(),
            differences
                .iter()
                .take(10)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
