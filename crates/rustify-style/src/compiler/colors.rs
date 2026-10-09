//! Native color branches, separate from image, font-size, and line-width work.

use super::{candidate::CandidateValue, theme::Theme, value::concat_text, DiagnosticKind};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct ColorOutput {
    pub declarations: Vec<(Text, Text)>,
    pub selector: Option<&'static str>,
    pub sort_property: Option<&'static str>,
    pub declaration_count: usize,
}

// Fixed upstream utils/is-color.ts data; Tailwind CSS 4.1.13, MIT.
const NAMED_COLORS: &[&str] = &[
    "accentcolor",
    "accentcolortext",
    "activetext",
    "aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "buttonborder",
    "buttonface",
    "buttontext",
    "cadetblue",
    "canvas",
    "canvastext",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "currentcolor",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkgrey",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "field",
    "fieldtext",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "graytext",
    "green",
    "greenyellow",
    "grey",
    "highlight",
    "highlighttext",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "linktext",
    "magenta",
    "mark",
    "marktext",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "selecteditem",
    "selecteditemtext",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "transparent",
    "turquoise",
    "violet",
    "visitedtext",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];

struct Family {
    property: &'static str,
    namespaces: &'static [&'static str],
    selector: Option<&'static str>,
    sort_property: Option<&'static str>,
}

fn family(root: &str) -> Option<Family> {
    let (property, namespaces): (&'static str, &'static [&'static str]) = match root {
        "bg" => ("background-color", &["background-color", "color"]),
        "text" => ("color", &["text-color", "color"]),
        "border" => ("border-color", &["border-color", "color"]),
        "border-x" => ("border-inline-color", &["border-color", "color"]),
        "border-y" => ("border-block-color", &["border-color", "color"]),
        "border-s" => ("border-inline-start-color", &["border-color", "color"]),
        "border-e" => ("border-inline-end-color", &["border-color", "color"]),
        "border-t" => ("border-top-color", &["border-color", "color"]),
        "border-r" => ("border-right-color", &["border-color", "color"]),
        "border-b" => ("border-bottom-color", &["border-color", "color"]),
        "border-l" => ("border-left-color", &["border-color", "color"]),
        "outline" => ("outline-color", &["outline-color", "color"]),
        "decoration" => ("text-decoration-color", &["text-decoration-color", "color"]),
        "accent" => ("accent-color", &["accent-color", "color"]),
        "caret" => ("caret-color", &["caret-color", "color"]),
        "fill" => ("fill", &["fill", "color"]),
        "stroke" => ("stroke", &["stroke", "color"]),
        "divide" => ("border-color", &["divide-color", "border-color", "color"]),
        "placeholder" => ("color", &["background-color", "color"]),
        _ => return None,
    };
    let (selector, sort_property) = match root {
        "divide" => (Some(":where(& > :not(:last-child))"), Some("divide-color")),
        "placeholder" => (Some("&::placeholder"), Some("placeholder-color")),
        _ => (None, None),
    };
    Some(Family {
        property,
        namespaces,
        selector,
        sort_property,
    })
}

pub(super) fn resolve_named(
    value: &Text,
    namespaces: &[&str],
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    for namespace in namespaces {
        for key in [
            concat_text(&[&format!("--{namespace}-").into(), value]),
            concat_text(&[
                &format!("--{namespace}-").into(),
                &value.replace_ascii(b'.', "_"),
            ]),
        ] {
            if let Some(css) = theme.resolve_text(&key, used) {
                return Ok(Some(css));
            }
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

pub(super) fn known_type(value: &Text) -> &'static str {
    [
        "color",
        "length",
        "percentage",
        "ratio",
        "number",
        "integer",
        "url",
        "position",
        "bg-size",
        "line-width",
        "image",
        "family-name",
        "generic-name",
        "absolute-size",
        "relative-size",
        "angle",
        "vector",
        "size",
        "shadow",
    ]
    .into_iter()
    .find(|kind| value == *kind)
    .unwrap_or("")
}

pub(super) fn js_number(input: &str) -> Option<f64> {
    js_number_text(&Text::from(input))
}

pub(super) fn js_number_text(input: &Text) -> Option<f64> {
    let whitespace = |unit: &u16| matches!(*unit, 9..=13 | 32 | 160 | 0x1680 | 0x2000..=0x200a | 0x2028 | 0x2029 | 0x202f | 0x205f | 0x3000 | 0xfeff);
    let units = input.units();
    let start = units.iter().take_while(|unit| whitespace(unit)).count();
    let end = units.len()
        - units[start..]
            .iter()
            .rev()
            .take_while(|unit| whitespace(unit))
            .count();
    let units = &units[start..end];
    if units.is_empty() {
        return Some(0.0);
    }
    // Number syntax is ASCII; a non-ASCII unit is NaN, while callers retain
    // the original opaque CSS when this classification returns None.
    if !units.iter().all(|unit| *unit <= 127) {
        return None;
    }
    let input: String = units.iter().map(|unit| char::from(*unit as u8)).collect();
    match input.as_str() {
        "Infinity" | "+Infinity" => return Some(f64::INFINITY),
        "-Infinity" => return Some(f64::NEG_INFINITY),
        _ => {}
    }
    for (prefixes, radix) in [(["0x", "0X"], 16), (["0o", "0O"], 8), (["0b", "0B"], 2)] {
        if let Some(digits) = prefixes
            .iter()
            .find_map(|prefix| input.strip_prefix(prefix))
        {
            if digits.is_empty() {
                return None;
            }
            return digits.chars().try_fold(0.0, |number, digit| {
                digit
                    .to_digit(radix)
                    .map(|digit| number * f64::from(radix) + f64::from(digit))
            });
        }
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || b".+-eE".contains(&byte))
    {
        return None;
    }
    input.parse::<f64>().ok().filter(|number| !number.is_nan())
}

pub(super) fn js_string(number: f64) -> String {
    if number.is_infinite() {
        return if number.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .into();
    }
    if number == 0.0 {
        return "0".into();
    }
    let sign = if number.is_sign_negative() { "-" } else { "" };
    let number = number.abs();
    let ordinary = number.to_string();
    if (1e-6..1e21).contains(&number) {
        return format!("{sign}{ordinary}");
    }
    let (significant, exponent) = if number >= 1e21 {
        (ordinary.trim_end_matches('0'), ordinary.len() as i32 - 1)
    } else {
        let decimals = ordinary.strip_prefix("0.").unwrap_or(&ordinary);
        let significant = decimals.trim_start_matches('0');
        (
            significant.trim_end_matches('0'),
            -(decimals.len() as i32 - significant.len() as i32) - 1,
        )
    };
    let mut chars = significant.chars();
    let Some(first) = chars.next() else {
        return "0".into();
    };
    let rest = chars.as_str();
    let mantissa = if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first}.{rest}")
    };
    format!(
        "{sign}{mantissa}e{}{exponent}",
        if exponent >= 0 { "+" } else { "" }
    )
}

pub(super) fn canonical_nonnegative(input: &str, integer: bool) -> bool {
    canonical_nonnegative_text(&Text::from(input), integer)
}

pub(super) fn canonical_nonnegative_text(input: &Text, integer: bool) -> bool {
    js_number_text(input).is_some_and(|number| {
        number.is_finite()
            && number >= 0.0
            && if integer {
                number.fract() == 0.0
            } else {
                number % 0.25 == 0.0
            }
            && input == &Text::from(js_string(number))
    })
}

pub(super) fn as_color(
    color: Text,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    let Some(modifier) = modifier else {
        return Ok(Some(color));
    };
    let alpha = if modifier.starts_with("[") && modifier.ends_with("]") {
        match super::value::decode_arbitrary_text(
            &Text::from_units(modifier.units()[1..modifier.units().len() - 1].to_vec()),
            usize::MAX,
        ) {
            Ok(value) => value,
            Err(super::CompileError::Syntax { .. }) => return Ok(None),
            Err(error) => return Err(error.into()),
        }
    } else if modifier.starts_with("(") && modifier.ends_with(")") {
        concat_text(&[
            &"var(".into(),
            &Text::from_units(modifier.units()[1..modifier.units().len() - 1].to_vec()),
            &")".into(),
        ])
    } else if let Some(alpha) =
        resolve_named(modifier, &["opacity"], theme, used)?.filter(|alpha| !alpha.is_empty())
    {
        alpha
    } else if canonical_nonnegative_text(modifier, false) {
        concat_text(&[modifier, &"%".into()])
    } else {
        return Ok(None);
    };
    let alpha = match js_number_text(&alpha) {
        Some(number) => Text::from(format!("{}%", js_string(number * 100.0))),
        None => alpha,
    };
    if alpha == "100%" {
        return Ok(Some(color));
    }
    let mut output = Text::from("color-mix(in oklab, ");
    output.push_text(&color);
    output.push_str(" ");
    output.push_text(&alpha);
    output.push_str(", transparent)");
    Ok(Some(output))
}

fn css_number(bytes: &[u16]) -> bool {
    let mut i = usize::from(bytes.first().is_some_and(|byte| matches!(byte, 43 | 45)));
    let start = i;
    while bytes.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
        i += 1
    }
    let integral = i - start;
    if bytes.get(i) == Some(&46) {
        i += 1;
        let start = i;
        while bytes.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
            i += 1
        }
        if i == start {
            return false;
        }
    } else if integral == 0 {
        return false;
    }
    if bytes.get(i).is_some_and(|byte| matches!(byte, 101 | 69)) {
        i += 1;
        if bytes.get(i).is_some_and(|byte| matches!(byte, 43 | 45)) {
            i += 1
        }
        let start = i;
        while bytes.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
            i += 1
        }
        if i == start {
            return false;
        }
    }
    i == bytes.len()
}

fn math(value: &Text) -> bool {
    [
        "calc", "min", "max", "clamp", "mod", "rem", "sin", "cos", "tan", "asin", "acos", "atan",
        "atan2", "pow", "sqrt", "hypot", "log", "exp", "round",
    ]
    .iter()
    .any(|name| value.contains(&format!("{name}(")))
}
pub(super) fn is_number_text(value: &Text) -> bool {
    css_number(value.units()) || math(value)
}
fn percentage(value: &Text) -> bool {
    value.ends_with("%") && css_number(&value.units()[..value.units().len() - 1]) || math(value)
}
fn length(value: &Text) -> bool {
    [
        "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
        "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi",
        "cqb", "cqmin", "cqmax",
    ]
    .iter()
    .any(|unit| {
        value.ends_with(unit) && css_number(&value.units()[..value.units().len() - unit.len()])
    }) || math(value)
}
pub(super) fn is_color_text(value: &Text) -> bool {
    value.starts_with("#")
        || NAMED_COLORS
            .binary_search_by(|color| {
                color
                    .bytes()
                    .map(u16::from)
                    .cmp(value.units().iter().copied().map(|unit| {
                        match unit {
                            65..=90 => unit + 32,
                            // Named colors use JS toLowerCase, unlike the non-Unicode
                            // case-insensitive function regex: Kelvin sign lowers to k.
                            0x212a => 107,
                            _ => unit,
                        }
                    }))
            })
            .is_ok()
        || [
            "rgb(",
            "rgba(",
            "hsl(",
            "hsla(",
            "hwb(",
            "color(",
            "lab(",
            "lch(",
            "oklab(",
            "oklch(",
            "light-dark(",
            "color-mix(",
        ]
        .iter()
        .any(|prefix| {
            value.units().get(..prefix.len()).is_some_and(|units| {
                units
                    .iter()
                    .copied()
                    .zip(prefix.bytes())
                    .all(|(unit, byte)| {
                        let folded = if (65..=90).contains(&unit) {
                            unit + 32
                        } else {
                            unit
                        };
                        folded == u16::from(byte)
                    })
            })
        })
}
fn segments(value: &Text, separator: char) -> Vec<Text> {
    super::value::segment_text(value, separator as u16, usize::MAX).unwrap_or_default()
}
fn url(value: &Text) -> bool {
    // Upstream /^url\(.*\)$/ has no dotAll flag.
    value.starts_with("url(")
        && value.ends_with(")")
        && !value
            .units()
            .iter()
            .any(|unit| matches!(unit, 10 | 13 | 0x2028 | 0x2029))
}
fn image(value: &Text) -> bool {
    let mut count = 0;
    for part in segments(value, ',') {
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
            count += 1
        } else {
            return false;
        }
    }
    count > 0
}
fn position(value: &Text) -> bool {
    let mut count = 0;
    for part in segments(value, ' ') {
        if part.starts_with("var(") {
            continue;
        }
        if ["center", "top", "right", "bottom", "left"]
            .iter()
            .any(|keyword| part == *keyword)
            || length(&part)
            || percentage(&part)
        {
            count += 1
        } else {
            return false;
        }
    }
    count > 0
}
fn background_size(value: &Text) -> bool {
    let mut count = 0;
    for size in segments(value, ',') {
        if size == "cover" || size == "contain" {
            count += 1;
            continue;
        }
        let parts = segments(&size, ' ');
        if !matches!(parts.len(), 1 | 2) {
            return false;
        }
        if parts
            .iter()
            .all(|value| value == "auto" || length(value) || percentage(value))
        {
            count += 1
        }
    }
    count > 0
}
pub(super) fn inferred_type_text(root: &str, value: &Text) -> Option<&'static str> {
    if value.starts_with("var(") {
        return None;
    }
    if root == "bg" && image(value) {
        return Some("image");
    }
    if is_color_text(value) {
        return None;
    }
    match root {
        "bg" => {
            if percentage(value) {
                Some("percentage")
            } else if position(value) {
                Some("position")
            } else if background_size(value) {
                Some("bg-size")
            } else if length(value) {
                Some("length")
            } else if url(value) {
                Some("url")
            } else {
                None
            }
        }
        root if root == "border" || root.starts_with("border-") => {
            if segments(value, ' ').iter().all(|part| {
                length(part)
                    || is_number_text(part)
                    || ["thin", "medium", "thick"]
                        .iter()
                        .any(|keyword| part == *keyword)
            }) {
                Some("line-width")
            } else if length(value) {
                Some("length")
            } else {
                None
            }
        }
        "outline" | "stroke" => {
            if root == "outline" && length(value) {
                Some("length")
            } else if is_number_text(value) {
                Some("number")
            } else if length(value) {
                Some("length")
            } else if percentage(value) {
                Some("percentage")
            } else {
                None
            }
        }
        "decoration" | "text" => {
            if length(value) {
                Some("length")
            } else if percentage(value) {
                Some("percentage")
            } else if root == "text"
                && [
                    "xx-small",
                    "x-small",
                    "small",
                    "medium",
                    "large",
                    "x-large",
                    "xx-large",
                    "xxx-large",
                ]
                .iter()
                .any(|keyword| value == *keyword)
            {
                Some("absolute-size")
            } else if root == "text" && (value == "larger" || value == "smaller") {
                Some("relative-size")
            } else {
                None
            }
        }
        _ => None,
    }
}
fn other_type(root: &str, kind: &str) -> bool {
    match root {
        "bg" => matches!(
            kind,
            "percentage" | "position" | "bg-size" | "length" | "size" | "image" | "url"
        ),
        "text" => matches!(
            kind,
            "size" | "length" | "percentage" | "absolute-size" | "relative-size"
        ),
        "outline" | "stroke" => matches!(kind, "length" | "number" | "percentage"),
        "decoration" => matches!(kind, "length" | "percentage"),
        root if root == "border" || root.starts_with("border-") => {
            matches!(kind, "line-width" | "length")
        }
        _ => false,
    }
}
fn other_interpretation(
    root: &str,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<DiagnosticKind, super::utilities::FamilyFailure> {
    if root == "stroke" {
        return Ok(DiagnosticKind::PendingFamily);
    }
    Ok(if root == "text" {
        let recognized = match modifier {
            None => true,
            Some(modifier) => {
                (modifier.starts_with("[") || modifier.starts_with("("))
                    || modifier == "none"
                    || resolve_named(modifier, &["leading"], theme, used)?.is_some()
                    || canonical_nonnegative_text(modifier, false)
                        && theme.resolve_text(&Text::from("--spacing"), used).is_some()
            }
        };
        if recognized {
            DiagnosticKind::PendingFamily
        } else {
            DiagnosticKind::InvalidValue
        }
    } else if modifier.is_some() {
        DiagnosticKind::InvalidValue
    } else {
        DiagnosticKind::PendingFamily
    })
}
fn mark_variables(
    color: &Text,
    used: &mut BTreeSet<Text>,
) -> Result<(), super::utilities::FamilyFailure> {
    fn walk(nodes: &[super::value::ValueNode], used: &mut BTreeSet<Text>) {
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
    // Candidate syntax already bounds input function depth; the generated
    // color-mix wrapper adds only one level before the session quota check.
    let nodes = super::value::parse_text(color, usize::MAX)?;
    walk(&nodes, used);
    Ok(())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<ColorOutput>, super::utilities::FamilyFailure> {
    let Some(family) = family(root) else {
        return Ok(None);
    };
    let Some(value) = value else {
        return Err(
            if root == "outline" || root == "border" || root.starts_with("border-") {
                DiagnosticKind::PendingFamily
            } else {
                DiagnosticKind::InvalidValue
            }
            .into(),
        );
    };
    let color = match value {
        CandidateValue::Arbitrary { value, data_type } => {
            if data_type
                .as_ref()
                .map(super::colors::known_type)
                .or_else(|| inferred_type_text(root, value))
                .is_some_and(|kind| other_type(root, kind))
            {
                if root == "stroke" && modifier.is_some() {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                return Err(other_interpretation(root, modifier, theme, used)?.into());
            }
            as_color(value.clone(), modifier, theme, used)?
        }
        CandidateValue::Named { value } => {
            // Thickness precedes the color branch for decoration only.
            if root == "decoration"
                && (resolve_named(value, &["text-decoration-thickness"], theme, used)?.is_some()
                    || canonical_nonnegative_text(value, true))
            {
                return Err(other_interpretation(root, modifier, theme, used)?.into());
            }
            let color = match value {
                value if value == "current" => Some("currentcolor".into()),
                value if value == "inherit" || value == "transparent" => Some(value.clone()),
                _ => resolve_named(value, family.namespaces, theme, used)?,
            }
            .filter(|value| !value.is_empty());
            let color = match color {
                Some(color) => as_color(color, modifier, theme, used)?,
                None => None,
            };
            if color.is_none() {
                let namespace = match root {
                    "bg" => Some("background-image"),
                    "text" => Some("text"),
                    "outline" => Some("outline-width"),
                    "stroke" => Some("stroke-width"),
                    root if root == "border" || root.starts_with("border-") => Some("border-width"),
                    _ => None,
                };
                let resolved = match namespace {
                    Some(namespace) => resolve_named(value, &[namespace], theme, used)?.is_some(),
                    None => false,
                };
                if resolved
                    || matches!(
                        root,
                        "outline"
                            | "stroke"
                            | "border"
                            | "border-x"
                            | "border-y"
                            | "border-s"
                            | "border-e"
                            | "border-t"
                            | "border-r"
                            | "border-b"
                            | "border-l"
                    ) && canonical_nonnegative_text(value, true)
                {
                    return Err(other_interpretation(root, modifier, theme, used)?.into());
                }
            }
            color
        }
    }
    .ok_or(DiagnosticKind::InvalidValue)?;
    mark_variables(&color, used)?;
    Ok(Some(ColorOutput {
        declarations: vec![(family.property.into(), color)],
        selector: family.selector,
        sort_property: family.sort_property,
        declaration_count: 1 + usize::from(family.sort_property.is_some()),
    }))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, DiagnosticKind, Limits};

    #[test]
    fn canonical_theme_names_and_modifiers_keep_original_units() {
        use super::{as_color, resolve_named, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let name = Text::from_units(vec![0xd800]);
        let modifier = Text::from_units(vec![0xdc00]);
        let mut css = Text::from("@theme inline { --color-");
        css.push_text(&name);
        css.push_str(": var(--probe-");
        css.push_text(&name);
        css.push_str("); --opacity-");
        css.push_text(&modifier);
        css.push_str(": .5; }");
        let mut nodes = crate::compiler::parser::parse_text(&css, 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let mut used = BTreeSet::new();
        let color = resolve_named(&name, &["color"], &theme, &mut used)
            .unwrap()
            .unwrap();
        let output = as_color(color.clone(), Some(&modifier), &theme, &mut used)
            .unwrap()
            .unwrap();
        let mut expected = Text::from("color-mix(in oklab, ");
        expected.push_text(&color);
        expected.push_str(" 50%, transparent)");
        assert_eq!(output, expected);
        assert_ne!(
            resolve_named(&Text::from("�"), &["color"], &theme, &mut used).unwrap(),
            Some(color)
        );
    }

    #[test]
    fn arbitrary_modifier_variable_keeps_original_units() {
        use super::{as_color, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let mut modifier = Text::from("(--alpha-");
        modifier.push_text(&Text::from_units(vec![0xd800]));
        modifier.push_str(")");
        let result = as_color(
            Text::from("red"),
            Some(&modifier),
            &Theme::default(),
            &mut BTreeSet::new(),
        )
        .unwrap()
        .unwrap();
        let mut expected = Text::from("color-mix(in oklab, red var(--alpha-");
        expected.push_text(&Text::from_units(vec![0xd800]));
        expected.push_str("), transparent)");
        assert_eq!(result, expected);
    }

    #[test]
    fn classifications_match_upstream_case_and_dot_rules() {
        use super::{inferred_type_text, is_color_text, is_number_text};
        use crate::text::Text;

        assert!(is_color_text(&Text::from("blac\u{212a}")));
        assert!(!is_color_text(&Text::from("o\u{212a}lab(0 0 0)")));
        assert!(is_color_text(&Text::from("OKLAB(0 0 0)")));
        assert!(!is_number_text(&Text::from("CALC(1 + 1)")));
        assert!(!is_number_text(&Text::from("1\n")));
        for unit in [10, 13, 0x2028, 0x2029] {
            let mut image = Text::from("url(a");
            image.push_text(&Text::from_units(vec![unit]));
            image.push_str(")");
            assert_eq!(inferred_type_text("bg", &image), None);
        }
    }

    #[test]
    fn classifications_accept_opaque_function_bodies() {
        use super::{inferred_type_text, is_color_text, is_number_text};
        use crate::text::Text;

        let mut color = Text::from("RGB(");
        color.push_text(&Text::from_units(vec![0xd800]));
        color.push_str(")");
        assert!(is_color_text(&color));
        let mut math = Text::from("calc(");
        math.push_text(&Text::from_units(vec![0xdc00]));
        math.push_str(")");
        assert!(is_number_text(&math));
        assert_eq!(inferred_type_text("outline", &math), Some("length"));
        let mut image = Text::from("url('");
        image.push_text(&Text::from_units(vec![0xd800]));
        image.push_str("')");
        assert_eq!(inferred_type_text("bg", &image), Some("image"));
        assert!(!is_number_text(&Text::from_units(vec![0xd800])));
    }

    #[test]
    fn inline_color_and_alpha_preserve_non_scalar_units() {
        use super::{as_color, js_number_text, resolve_named, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let mut css = Text::from("@theme inline { --color-brand: rgb(");
        css.push_text(&Text::from_units(vec![0xd800]));
        css.push_str("); --opacity-soft: var(--alpha-");
        css.push_text(&Text::from_units(vec![0xdc00]));
        css.push_str("); }");
        let mut nodes = crate::compiler::parser::parse_text(&css, 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let mut used = BTreeSet::new();
        let color = resolve_named(&Text::from("brand"), &["color"], &theme, &mut used)
            .unwrap()
            .unwrap();
        let output = as_color(color.clone(), Some(&Text::from("soft")), &theme, &mut used)
            .unwrap()
            .unwrap();
        let mut expected = Text::from("color-mix(in oklab, ");
        expected.push_text(&color);
        expected.push_str(" var(--alpha-");
        expected.push_text(&Text::from_units(vec![0xdc00]));
        expected.push_str("), transparent)");
        assert_eq!(output.units(), expected.units());
        let mut variable = Text::from("--alpha-");
        variable.push_text(&Text::from_units(vec![0xdc00]));
        assert!(used.contains(&variable));
        assert_eq!(js_number_text(&Text::from("\u{feff}0.5\u{a0}")), Some(0.5));
        assert_eq!(js_number_text(&Text::from("\u{85}0.5")), None);
        assert_eq!(js_number_text(&Text::from_units(vec![0xd800])), None);
    }

    #[test]
    fn color_branches_match_official_fixtures_and_preserve_pending_interpretations() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/colors-fixtures.json"
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
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            let report = prepared.report();
            let implemented = case["expectedProperties"]
                .as_array()
                .is_some_and(|properties| {
                    !properties.is_empty()
                        && properties.iter().all(|property| {
                            matches!(
                                property.as_str(),
                                Some(
                                    "background-image"
                                        | "background-position"
                                        | "background-size"
                                        | "font-size"
                                        | "line-height"
                                        | "letter-spacing"
                                        | "font-weight"
                                        | "outline-style"
                                        | "outline-width"
                                        | "stroke-width"
                                        | "text-decoration-thickness"
                                        | "border-style"
                                        | "border-width"
                                        | "border-inline-style"
                                        | "border-inline-width"
                                        | "border-block-style"
                                        | "border-block-width"
                                        | "border-inline-start-style"
                                        | "border-inline-start-width"
                                        | "border-inline-end-style"
                                        | "border-inline-end-width"
                                        | "border-top-style"
                                        | "border-top-width"
                                        | "border-right-style"
                                        | "border-right-width"
                                        | "border-bottom-style"
                                        | "border-bottom-width"
                                        | "border-left-style"
                                        | "border-left-width"
                                )
                            )
                        })
                });
            if case["branch"] == "other_interpretation_pending"
                || case["branch"] == "other_interpretation" && !implemented
            {
                if report.unsupported.is_empty()
                    || report
                        .unsupported
                        .iter()
                        .any(|diagnostic| diagnostic.kind != DiagnosticKind::PendingFamily)
                    || !report.admitted.is_empty()
                    || !report.css.is_empty()
                {
                    failures.push(format!(
                        "{}: other branch must remain pending: {:?}",
                        case["id"].as_str().unwrap(),
                        report
                    ));
                }
            } else if report.css != case["expectedCss"].as_str().unwrap()
                || !report.unsupported.is_empty()
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
}
