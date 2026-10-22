//! Candidate syntax is independent from utility implementation completeness.

use super::{registry, value, CompileError};
use crate::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub raw: Text,
    pub important: bool,
    pub variants: Vec<Variant>,
    pub kind: CandidateKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CandidateKind {
    Static {
        root: Text,
    },
    Functional {
        root: Text,
        value: Option<CandidateValue>,
        modifier: Option<Text>,
    },
    ArbitraryProperty {
        property: Text,
        value: Text,
        modifier: Option<Text>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CandidateValue {
    Named {
        value: Text,
    },
    Arbitrary {
        value: Text,
        data_type: Option<Text>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Variant {
    Registered {
        raw: Text,
        root: Text,
        value: Option<Text>,
        modifier: Option<Text>,
    },
    Arbitrary {
        selector: Text,
        relative: bool,
    },
}

impl<'de> Deserialize<'de> for CandidateKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Static,
            Functional,
            ArbitraryProperty,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            root: Option<Text>,
            value: Option<Box<serde_json::value::RawValue>>,
            modifier: Option<Text>,
            property: Option<Text>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        // Text fields decode directly, bypassing serde's scalar tagged Content buffer.
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Static => Self::Static {
                root: required(fields.root, "root")?,
            },
            Kind::Functional => Self::Functional {
                root: required(fields.root, "root")?,
                value: fields
                    .value
                    .map(|value| {
                        serde_json::from_str(value.get()).map_err(serde::de::Error::custom)
                    })
                    .transpose()?,
                modifier: fields.modifier,
            },
            Kind::ArbitraryProperty => Self::ArbitraryProperty {
                property: required(fields.property, "property")?,
                value: serde_json::from_str(required(fields.value, "value")?.get())
                    .map_err(serde::de::Error::custom)?,
                modifier: fields.modifier,
            },
        })
    }
}

impl<'de> Deserialize<'de> for CandidateValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Named,
            Arbitrary,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            value: Option<Text>,
            data_type: Option<Text>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        // Text fields decode directly, bypassing serde's scalar tagged Content buffer.
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Named => Self::Named {
                value: required(fields.value, "value")?,
            },
            Kind::Arbitrary => Self::Arbitrary {
                value: required(fields.value, "value")?,
                data_type: fields.data_type,
            },
        })
    }
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Registered,
            Arbitrary,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            raw: Option<Text>,
            root: Option<Text>,
            value: Option<Text>,
            modifier: Option<Text>,
            selector: Option<Text>,
            relative: Option<bool>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        // Text fields decode directly, bypassing serde's scalar tagged Content buffer.
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Registered => Self::Registered {
                raw: required(fields.raw, "raw")?,
                root: required(fields.root, "root")?,
                value: fields.value,
                modifier: fields.modifier,
            },
            Kind::Arbitrary => Self::Arbitrary {
                selector: required(fields.selector, "selector")?,
                relative: required(fields.relative, "relative")?,
            },
        })
    }
}

fn slice(value: &Text, start: usize, end: usize) -> Text {
    Text::from_units(value.units()[start..end].to_vec())
}

fn valid_modifier(raw: &Text, max_depth: usize) -> Result<bool, CompileError> {
    if raw.starts_with("[") && raw.ends_with("]") {
        let value = value::decode_arbitrary_text(&slice(raw, 1, raw.units().len() - 1), max_depth)?;
        Ok(!value.trim_js().is_empty() && value::is_valid_arbitrary_text(&value))
    } else if raw.starts_with("(") && raw.ends_with(")") {
        let variable = slice(raw, 1, raw.units().len() - 1);
        Ok(variable.starts_with("--") && value::is_valid_arbitrary_text(&variable))
    } else {
        Ok(true)
    }
}

pub fn parse_variant(raw: &str, max_depth: usize) -> Result<Option<Variant>, CompileError> {
    parse_variant_text(&raw.into(), max_depth)
}

pub fn parse_variant_text(raw: &Text, max_depth: usize) -> Result<Option<Variant>, CompileError> {
    parse_variant_with_theme(raw, max_depth, &super::theme::Theme::default())
}

/// What candidate parsing needs to know about registered utilities and variants.
pub(super) trait Registry {
    fn prefix(&self) -> Option<&str>;
    fn has_static_utility(&self, root: &Text) -> bool;
    /// Registered functional utility roots in descending order, so a longer
    /// root is tried before any root it extends.
    fn functional_roots(&self) -> impl Iterator<Item = &str> + '_;
    fn variant_kind(&self, root: &Text) -> Option<super::variants::VariantKind>;
    /// The rules a variant generates, as upstream `Compounds` flags.
    fn variant_compounds(&self, variant: &Variant) -> u8;
    /// The rules a compound variant accepts from its child.
    fn compounds_with(&self, root: &Text) -> u8;
}

/// The fixed upstream registries plus the stylesheet's own variants and utilities.
pub(super) struct Builtin<'a> {
    pub theme: &'a super::theme::Theme,
    pub custom: &'a super::custom_utilities::CustomUtilities,
}

impl Registry for Builtin<'_> {
    fn prefix(&self) -> Option<&str> {
        self.theme.prefix.as_deref()
    }
    fn has_static_utility(&self, root: &Text) -> bool {
        use super::custom_utilities::UtilityKind;
        // Static roots come only from fixed ASCII or validated custom registries.
        root.to_utf8().ok().is_some_and(|root| {
            registry::STATIC_ROOTS.binary_search(&root.as_str()).is_ok()
                || self.custom.contains_root(UtilityKind::Static, &root)
        })
    }
    fn functional_roots(&self) -> impl Iterator<Item = &str> + '_ {
        use super::custom_utilities::UtilityKind;
        let mut builtin = registry::FUNCTIONAL_ROOTS.iter().rev().copied().peekable();
        let mut registered = self.custom.roots(UtilityKind::Functional).rev().peekable();
        std::iter::from_fn(move || match (builtin.peek(), registered.peek()) {
            (Some(a), Some(b)) => match (*a).cmp(*b) {
                std::cmp::Ordering::Greater => builtin.next(),
                std::cmp::Ordering::Less => registered.next(),
                std::cmp::Ordering::Equal => {
                    registered.next();
                    builtin.next()
                }
            },
            (Some(_), None) => builtin.next(),
            (None, Some(_)) => registered.next(),
            (None, None) => None,
        })
    }
    fn variant_kind(&self, root: &Text) -> Option<super::variants::VariantKind> {
        super::variants::root_kind(root, self.theme)
    }
    fn variant_compounds(&self, variant: &Variant) -> u8 {
        super::variants::compounds(variant, self.theme)
    }
    fn compounds_with(&self, root: &Text) -> u8 {
        // `not` also negates conditional at-rules; group, peer, in and has
        // accept style rules only.
        if root == "not" {
            3
        } else {
            2
        }
    }
}

fn builtin(theme: &super::theme::Theme) -> Builtin<'_> {
    Builtin {
        theme,
        custom: super::custom_utilities::CustomUtilities::empty(),
    }
}

// Scalar convenience metadata API. The lossless public object codec is separate.
pub(super) fn parse_variant_json_with_theme(
    raw: &str,
    max_depth: usize,
    theme: &super::theme::Theme,
) -> Result<serde_json::Value, CompileError> {
    let parsed = parse_variant_object(&raw.into(), max_depth, theme)?;
    let json = serde_json::to_string(&parsed).map_err(|_| CompileError::Syntax {
        offset: 0,
        message: "Unable to encode variant.".into(),
    })?;
    serde_json::from_str(&json).map_err(|_| CompileError::Syntax {
        offset: 0,
        message: "Non-scalar variant requires the Text object API.".into(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParsedVariant {
    Static {
        root: Text,
    },
    Functional {
        root: Text,
        modifier: Option<ParsedValue>,
        value: Option<ParsedValue>,
    },
    Compound {
        root: Text,
        modifier: Option<ParsedValue>,
        variant: Box<ParsedVariant>,
    },
    Arbitrary {
        selector: Text,
        relative: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParsedValue {
    Named { value: Text },
    Arbitrary { value: Text },
}

pub(super) fn parse_variant_object(
    raw: &Text,
    max_depth: usize,
    theme: &super::theme::Theme,
) -> Result<Option<ParsedVariant>, CompileError> {
    parse_variant_object_with(raw, max_depth, &builtin(theme))
}

pub(super) fn parse_variant_object_with<R: Registry>(
    raw: &Text,
    max_depth: usize,
    registry: &R,
) -> Result<Option<ParsedVariant>, CompileError> {
    use super::variants::VariantKind;
    let Some(variant) = parse_variant_with(raw, max_depth, registry)? else {
        return Ok(None);
    };
    let (root, child_value, modifier) = match variant {
        Variant::Registered {
            root,
            value,
            modifier,
            ..
        } => (root, value, modifier),
        Variant::Arbitrary { selector, relative } => {
            return Ok(Some(ParsedVariant::Arbitrary { selector, relative }))
        }
    };
    let parts = value::segment_text(raw, 47, max_depth)?;
    let modifier = parts.get(1).zip(modifier).map(|(raw, value)| {
        if raw.starts_with("[") && raw.ends_with("]") || raw.starts_with("(") && raw.ends_with(")")
        {
            ParsedValue::Arbitrary { value }
        } else {
            ParsedValue::Named { value }
        }
    });
    Ok(match registry.variant_kind(&root) {
        Some(VariantKind::Static) => Some(ParsedVariant::Static { root }),
        Some(VariantKind::Functional) => {
            let value = child_value
                .as_ref()
                .map(|raw| {
                    super::variants::decoded(raw, max_depth).map(|value| {
                        value.map(|(value, arbitrary)| {
                            if arbitrary {
                                ParsedValue::Arbitrary { value }
                            } else {
                                ParsedValue::Named { value }
                            }
                        })
                    })
                })
                .transpose()?
                .flatten();
            Some(ParsedVariant::Functional {
                root,
                modifier,
                value,
            })
        }
        Some(VariantKind::Compound) => {
            let Some(child) = child_value else {
                return Ok(None);
            };
            parse_variant_object_with(&child, max_depth, registry)?.map(|variant| {
                ParsedVariant::Compound {
                    root,
                    modifier,
                    variant: Box::new(variant),
                }
            })
        }
        None => None,
    })
}

pub(super) fn parse_variant_with_theme(
    raw: &Text,
    max_depth: usize,
    theme: &super::theme::Theme,
) -> Result<Option<Variant>, CompileError> {
    parse_variant_with(raw, max_depth, &builtin(theme))
}

pub(super) fn parse_variant_with<R: Registry>(
    raw: &Text,
    max_depth: usize,
    registry: &R,
) -> Result<Option<Variant>, CompileError> {
    let mut remaining = raw.clone();
    let mut compound_depth = 0;
    while let Some(child) = ["not-", "group-", "peer-", "in-", "has-"]
        .iter()
        .find_map(|prefix| remaining.strip_prefix(prefix))
    {
        compound_depth += 1;
        super::check_limit(compound_depth, max_depth, "ast_depth")?;
        remaining = child;
    }
    value::segment_text(raw, 58, max_depth)?;
    if raw.starts_with("[") && raw.ends_with("]") {
        let selector =
            value::decode_arbitrary_text(&slice(raw, 1, raw.units().len() - 1), max_depth)?;
        if selector.trim_js().is_empty()
            || !value::is_valid_arbitrary_text(&selector)
            || selector.starts_with("@") && selector.contains("&")
        {
            return Ok(None);
        }
        let relative = [">", "+", "~"]
            .iter()
            .any(|prefix| selector.starts_with(prefix));
        let selector = if !relative && !selector.starts_with("@") && !selector.contains("&") {
            value::concat_text(&[&"&:is(".into(), &selector, &")".into()])
        } else {
            selector
        };
        return Ok(Some(Variant::Arbitrary { selector, relative }));
    }
    let modifiers = value::segment_text(raw, 47, max_depth)?;
    if modifiers.get(2).is_some_and(|part| !part.is_empty()) {
        return Ok(None);
    }
    let base = &modifiers[0];
    let mut modifier = modifiers.get(1).cloned();
    if let Some(modifier) = &modifier {
        if !valid_modifier(modifier, max_depth)? {
            return Ok(None);
        }
    }
    modifier = modifier
        .map(|raw| {
            if raw.starts_with("[") && raw.ends_with("]") {
                value::decode_arbitrary_text(&slice(&raw, 1, raw.units().len() - 1), max_depth)
            } else if raw.starts_with("(") && raw.ends_with(")") {
                value::decode_arbitrary_text(
                    &value::concat_text(&[
                        &"var(".into(),
                        &slice(&raw, 1, raw.units().len() - 1),
                        &")".into(),
                    ]),
                    max_depth,
                )
            } else {
                Ok(raw)
            }
        })
        .transpose()?;
    let mut roots = vec![(base.clone(), None)];
    let mut end = base.units().len();
    while let Some(index) = base.units()[..end]
        .iter()
        .rposition(|unit| *unit == 45)
        .filter(|index| *index > 0)
    {
        let root = slice(base, 0, index);
        if index + 1 == base.units().len() && registry.variant_kind(&root).is_some() {
            break;
        }
        if index == 1 && base.starts_with("@") {
            break;
        }
        roots.push((root, Some(slice(base, index + 1, base.units().len()))));
        end = index;
    }
    if base.starts_with("@") && base != "@" {
        roots.push(("@".into(), Some(slice(base, 1, base.units().len()))));
    }
    for (root, child_value) in roots {
        if let Some(kind) = registry.variant_kind(&root) {
            if kind == super::variants::VariantKind::Static {
                if child_value.is_some() || modifier.is_some() {
                    return Ok(None);
                }
                return Ok(Some(Variant::Registered {
                    raw: raw.clone(),
                    root,
                    value: None,
                    modifier: None,
                }));
            }
            if kind == super::variants::VariantKind::Functional {
                if let Some(child_value) = &child_value {
                    if child_value.ends_with("]") && !child_value.starts_with("[")
                        || child_value.ends_with(")") && !child_value.starts_with("(")
                    {
                        continue;
                    }
                    if (child_value.ends_with("]") || child_value.ends_with(")"))
                        && super::variants::decoded(child_value, max_depth)?.is_none()
                    {
                        return Ok(None);
                    }
                }
            } else {
                let Some(child_value) = &child_value else {
                    return Ok(None);
                };
                let Some(child) = parse_variant_with(child_value, max_depth, registry)? else {
                    return Ok(None);
                };
                if registry.variant_compounds(&child) & registry.compounds_with(&root) == 0 {
                    return Ok(None);
                }
            }
            return Ok(Some(Variant::Registered {
                raw: raw.clone(),
                root,
                value: child_value,
                modifier,
            }));
        }
    }
    Ok(None)
}

pub fn parse(raw: &str, max_depth: usize) -> Result<Vec<Candidate>, CompileError> {
    parse_text(&raw.into(), max_depth)
}

pub fn parse_text(raw: &Text, max_depth: usize) -> Result<Vec<Candidate>, CompileError> {
    parse_with_theme(raw, max_depth, &super::theme::Theme::default())
}

pub(super) fn parse_with_theme(
    raw: &Text,
    max_depth: usize,
    theme: &super::theme::Theme,
) -> Result<Vec<Candidate>, CompileError> {
    parse_with_utilities(
        raw,
        max_depth,
        theme,
        &super::custom_utilities::CustomUtilities::default(),
    )
}

fn arbitrary_value(raw: &Text, max_depth: usize) -> Result<Option<CandidateValue>, CompileError> {
    let mut decoded = value::decode_arbitrary_text(raw, max_depth)?;
    if !value::is_valid_arbitrary_text(&decoded) {
        return Ok(None);
    }
    let mut data_type = None;
    for (index, &unit) in decoded.units().iter().enumerate() {
        if unit == 58 {
            if index > 0 {
                data_type = Some(slice(&decoded, 0, index))
            }
            decoded = slice(&decoded, index + 1, decoded.units().len());
            break;
        }
        if unit != 45 && !(97..=122).contains(&unit) {
            break;
        }
    }
    if decoded.trim_js().is_empty() {
        return Ok(None);
    }
    Ok(Some(CandidateValue::Arbitrary {
        value: decoded,
        data_type,
    }))
}

pub(super) fn parse_with_utilities(
    raw: &Text,
    max_depth: usize,
    theme: &super::theme::Theme,
    custom: &super::custom_utilities::CustomUtilities,
) -> Result<Vec<Candidate>, CompileError> {
    parse_with(raw, max_depth, &Builtin { theme, custom })
}

pub(super) fn parse_with<R: Registry>(
    raw: &Text,
    max_depth: usize,
    registry: &R,
) -> Result<Vec<Candidate>, CompileError> {
    let mut parts = value::segment_text(raw, 58, max_depth)?;
    if let Some(prefix) = registry.prefix() {
        if parts.len() == 1 || parts.first().is_none_or(|part| part != prefix) {
            return Ok(Vec::new());
        }
        parts.remove(0);
    }
    super::check_limit(parts.len().saturating_sub(1), max_depth, "ast_depth")?;
    let Some(base) = parts.last() else {
        return Ok(Vec::new());
    };
    let mut base = base.clone();
    let mut variants = Vec::new();
    for part in parts[..parts.len() - 1].iter().rev() {
        let Some(variant) = parse_variant_with(part, max_depth, registry)? else {
            return Ok(Vec::new());
        };
        variants.push(variant);
    }
    let important = base.starts_with("!") || base.ends_with("!");
    if base.ends_with("!") {
        base = slice(&base, 0, base.units().len() - 1)
    } else if base.starts_with("!") {
        base = slice(&base, 1, base.units().len())
    }
    let mut candidates = Vec::new();
    let make = |kind| Candidate {
        raw: raw.clone(),
        important,
        variants: variants.clone(),
        kind,
    };
    if registry.has_static_utility(&base) && !base.contains("[") {
        candidates.push(make(CandidateKind::Static { root: base.clone() }))
    }
    let segments = value::segment_text(&base, 47, max_depth)?;
    if segments.len() > 2 || segments.iter().any(Text::is_empty) {
        return Ok(candidates);
    }
    let base = &segments[0];
    let modifier = segments.get(1).cloned();
    if let Some(modifier) = &modifier {
        if !valid_modifier(modifier, max_depth)? {
            return Ok(candidates);
        }
    }
    if base.starts_with("[") && base.ends_with("]") {
        if let Some((property, raw_value)) = slice(base, 1, base.units().len() - 1).split_once(":")
        {
            if property
                .units()
                .first()
                .is_some_and(|unit| *unit == 45 || (97..=122).contains(unit))
                && !raw_value.is_empty()
            {
                let value = value::decode_arbitrary_text(&raw_value, max_depth)?;
                if !value::is_valid_arbitrary_text(&value) {
                    return Ok(candidates);
                }
                candidates.push(make(CandidateKind::ArbitraryProperty {
                    property,
                    value,
                    modifier,
                }));
            }
        }
        return Ok(candidates);
    }
    let exact_root = if base.ends_with("]") {
        let Some(index) = base.find("-[") else {
            return Ok(candidates);
        };
        Some(slice(base, 0, index))
    } else if base.ends_with(")") {
        let Some(index) = base.find("-(") else {
            return Ok(candidates);
        };
        Some(slice(base, 0, index))
    } else {
        None
    };
    let roots = registry.functional_roots();
    for root in roots {
        if exact_root.as_ref().is_some_and(|expected| expected != root) {
            continue;
        }
        let raw_value = if base == root {
            None
        } else if let Some(rest) = base.strip_prefix(&format!("{root}-")) {
            Some(rest)
        } else {
            continue;
        };
        let parsed = match raw_value {
            Some(value) if value.starts_with("[") && value.ends_with("]") => {
                let Some(parsed) =
                    arbitrary_value(&slice(&value, 1, value.units().len() - 1), max_depth)?
                else {
                    continue;
                };
                Some(parsed)
            }
            Some(value) if value.starts_with("(") && value.ends_with(")") => {
                let variable = slice(&value, 1, value.units().len() - 1);
                let parts = value::segment_text(&variable, 58, max_depth)?;
                let (data_type, variable) = if parts.len() == 2 {
                    (Some(&parts[0]), &parts[1])
                } else {
                    (None, &variable)
                };
                if !variable.starts_with("--") || !value::is_valid_arbitrary_text(variable) {
                    continue;
                }
                let raw = if let Some(data_type) = data_type {
                    value::concat_text(&[data_type, &":var(".into(), variable, &")".into()])
                } else {
                    value::concat_text(&[&"var(".into(), variable, &")".into()])
                };
                let Some(parsed) = arbitrary_value(&raw, max_depth)? else {
                    continue;
                };
                Some(parsed)
            }
            Some(value) if !value.is_empty() && !value.contains("[") && !value.contains("]") => {
                Some(CandidateValue::Named { value })
            }
            Some(_) => continue,
            None => None,
        };
        candidates.push(make(CandidateKind::Functional {
            root: root.into(),
            value: parsed,
            modifier: modifier.clone(),
        }));
    }
    Ok(candidates)
}

pub fn escape(raw: &str) -> String {
    let mut escaped = String::new();
    let first = raw.chars().next();
    for (index, ch) in raw.chars().enumerate() {
        if ch == '\0' {
            escaped.push('\u{fffd}');
        } else if ch.is_ascii_control()
            || ch.is_ascii_digit() && (index == 0 || index == 1 && first == Some('-'))
        {
            escaped.push_str(&format!("\\{:x} ", ch as u32));
        } else if !ch.is_ascii() || ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            escaped.push(ch);
        } else {
            escaped.push('\\');
            escaped.push(ch);
        }
    }
    if raw == "-" {
        "\\-".into()
    } else {
        escaped
    }
}

/// CSS identifier escaping over ECMAScript units, including isolated surrogates.
pub fn escape_text(raw: &crate::text::Text) -> crate::text::Text {
    let mut output = Vec::new();
    for (index, &unit) in raw.units().iter().enumerate() {
        if unit == 0 {
            output.push(0xfffd);
        } else if unit <= 0x1f
            || unit == 0x7f
            || (48..=57).contains(&unit)
                && (index == 0 || index == 1 && raw.units().first() == Some(&45))
        {
            output.extend(format!("\\{unit:x} ").encode_utf16());
        } else if unit >= 128
            || (48..=57).contains(&unit)
            || (65..=90).contains(&unit)
            || (97..=122).contains(&unit)
            || matches!(unit, 45 | 95)
        {
            output.push(unit);
        } else {
            output.extend([92, unit]);
        }
    }
    if raw.units() == [45] {
        crate::text::Text::from("\\-")
    } else {
        crate::text::Text::from_units(output)
    }
}

#[cfg(test)]
mod variant_json_tests {
    use super::*;

    #[test]
    fn text_escape_keeps_surrogates_distinct_from_replacement_and_literal_escapes() {
        use crate::text::Text;
        let input = Text::from_units(vec![45, 45, 120, 45, 0xd800, 33, 0xdc00, 0, 32]);
        assert_eq!(
            escape_text(&input).units(),
            [45, 45, 120, 45, 0xd800, 92, 33, 0xdc00, 0xfffd, 92, 32]
        );
        assert_eq!(escape_text(&Text::from("�")), Text::from("�"));
        assert_eq!(escape_text(&Text::from("\\ud800")), Text::from("\\\\ud800"));
        for scalar in [
            "",
            "-",
            "0x",
            "-1",
            "--a.b",
            "😀",
            "�",
            "\\ud800",
            "\0\t\u{7f}",
        ] {
            assert_eq!(escape_text(&Text::from(scalar)), Text::from(escape(scalar)));
        }
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn variant_parameter_parser_matches_official_json_shapes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/variant-parameters-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        let mut checked = 0;
        for (mode, input) in fixture["inputModes"].as_object().unwrap() {
            let compiler = super::super::Compiler::new(
                super::super::CompilerInput {
                    css: input.as_str().unwrap().into(),
                    entry: "input.css".into(),
                },
                vec![],
                super::super::Limits {
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
            for item in fixture["parseCases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|item| item["mode"].as_str() == Some(mode.as_str()))
            {
                let actual = parse_variant_json_with_theme(
                    item["raw"].as_str().unwrap(),
                    32,
                    &compiler.theme,
                )
                .unwrap();
                if actual != item["expectedVariant"] {
                    failures.push(format!(
                        "{}: got {actual}, expected {}",
                        item["id"], item["expectedVariant"]
                    ));
                }
                checked += 1;
            }
        }
        assert_eq!(checked, fixture["parseCases"].as_array().unwrap().len());
        assert!(
            failures.is_empty(),
            "{} parser mismatches; first: {:#?}",
            failures.len(),
            &failures[..failures.len().min(20)]
        );
    }
}

#[cfg(test)]
mod canonical_text_tests {
    use super::*;

    #[test]
    fn candidate_tagged_json_roundtrip_preserves_all_units_and_field_order() {
        let raw = Text::from_units((0..=u16::MAX).collect());
        for kind in [
            CandidateKind::Static { root: raw.clone() },
            CandidateKind::Functional {
                root: "bg".into(),
                value: Some(CandidateValue::Named { value: raw.clone() }),
                modifier: Some(raw.clone()),
            },
            CandidateKind::Functional {
                root: "bg".into(),
                value: Some(CandidateValue::Arbitrary {
                    value: raw.clone(),
                    data_type: Some(raw.clone()),
                }),
                modifier: None,
            },
            CandidateKind::Functional {
                root: "bg".into(),
                value: None,
                modifier: None,
            },
            CandidateKind::ArbitraryProperty {
                property: raw.clone(),
                value: raw.clone(),
                modifier: Some(raw.clone()),
            },
        ] {
            let expected = Candidate {
                raw: raw.clone(),
                important: false,
                variants: vec![
                    Variant::Registered {
                        raw: raw.clone(),
                        root: raw.clone(),
                        value: Some(raw.clone()),
                        modifier: Some(raw.clone()),
                    },
                    Variant::Arbitrary {
                        selector: raw.clone(),
                        relative: true,
                    },
                ],
                kind,
            };
            let json = serde_json::to_string(&expected).unwrap();
            assert_eq!(serde_json::from_str::<Candidate>(&json).unwrap(), expected);
        }
        let parsed: CandidateKind=serde_json::from_str(r#"{"value":"\ud800","modifier":"\udc00","property":"--\ud800","kind":"arbitrary_property"}"#).unwrap();
        assert!(
            matches!(parsed,CandidateKind::ArbitraryProperty {value,modifier:Some(modifier),property}
            if value.units()==[0xd800] && modifier.units()==[0xdc00] && property.units()==[45,45,0xd800])
        );
    }

    #[test]
    fn canonical_candidate_syntax_uses_original_units_and_js_trim() {
        let input = Text::from_units(
            "[.".encode_utf16()
                .chain([0xd800])
                .chain("]:[--".encode_utf16())
                .chain([0xdc00])
                .chain(":".encode_utf16())
                .chain([0xd800])
                .chain("]".encode_utf16())
                .collect(),
        );
        let parsed = parse_text(&input, 32).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].raw, input);
        assert!(
            matches!(&parsed[0].kind,CandidateKind::ArbitraryProperty {property,value,..}
            if property.units()==[45,45,0xdc00] && value.units()==[0xd800])
        );
        assert!(
            matches!(&parsed[0].variants[0],Variant::Arbitrary {selector,..}
            if selector.units()==Text::from_units("&:is(.".encode_utf16().chain([0xd800]).chain(")".encode_utf16()).collect()).units())
        );
        assert!(parse("p-[\u{feff}]", 32).unwrap().is_empty());
        assert_eq!(parse("p-[\u{85}]", 32).unwrap().len(), 1);
    }

    #[test]
    fn type_hints_are_extracted_after_decoding_and_parenthesis_rewriting() {
        assert!(matches!(&parse("bg-[:red]",32).unwrap()[0].kind,
            CandidateKind::Functional {value:Some(CandidateValue::Arbitrary {value,data_type:None}),..} if value=="red"));
        for input in ["bg-(_color:--x)", "bg-(color_:--x)"] {
            assert!(matches!(
                &parse(input, 32).unwrap()[0].kind,
                CandidateKind::Functional {
                    value: Some(CandidateValue::Arbitrary {
                        data_type: None,
                        ..
                    }),
                    ..
                }
            ));
        }
        assert!(matches!(&parse("bg-(color:--x)",32).unwrap()[0].kind,
            CandidateKind::Functional {value:Some(CandidateValue::Arbitrary {value,data_type:Some(hint)}),..}
                if value=="var(--x)" && hint=="color"));
    }
}
