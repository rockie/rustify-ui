//! Built-in validator predicates from the fixed cn configuration.

/// Opcode names in the upstream compiled table format.
pub const NAMES: [&str; 25] = [
    "isAny",
    "isAnyNonArbitrary",
    "isArbitraryValue",
    "isArbitraryVariable",
    "isFraction",
    "isNumber",
    "isInteger",
    "isPercent",
    "isTshirtSize",
    "isNamedContainerQuery",
    "isArbitraryLength",
    "isArbitraryNumber",
    "isArbitraryWeight",
    "isArbitraryFamilyName",
    "isArbitraryPosition",
    "isArbitrarySize",
    "isArbitraryImage",
    "isArbitraryShadow",
    "isArbitraryVariableLength",
    "isArbitraryVariableFamilyName",
    "isArbitraryVariablePosition",
    "isArbitraryVariableSize",
    "isArbitraryVariableImage",
    "isArbitraryVariableShadow",
    "isArbitraryVariableWeight",
];

/// ECMAScript whitespace, used by cn's tokenizer and numeric conversion.
pub fn is_whitespace(c: char) -> bool {
    matches!(c, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

fn number(value: &str) -> Option<f64> {
    if value.is_empty() {
        return None;
    }
    let text = value.trim_matches(is_whitespace);
    if text.is_empty() {
        return Some(0.0);
    }
    if matches!(text, "Infinity" | "+Infinity") {
        return Some(f64::INFINITY);
    }
    if text == "-Infinity" {
        return Some(f64::NEG_INFINITY);
    }
    for (prefixes, radix) in [(["0x", "0X"], 16), (["0o", "0O"], 8), (["0b", "0B"], 2)] {
        if let Some(digits) = prefixes.iter().find_map(|prefix| text.strip_prefix(prefix)) {
            if digits.is_empty() {
                return None;
            }
            return digits.chars().try_fold(0.0, |value, c| {
                c.to_digit(radix)
                    .map(|digit| value * radix as f64 + digit as f64)
            });
        }
    }
    let mut text = text;
    if text.starts_with(['+', '-']) {
        text = &text[1..];
    }
    let mut parts = text.split(['e', 'E']);
    let mantissa = parts.next()?;
    if let Some(exponent) = parts.next() {
        let exponent = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        if exponent.is_empty()
            || !exponent.bytes().all(|c| c.is_ascii_digit())
            || parts.next().is_some()
        {
            return None;
        }
    }
    let mut digits = 0;
    let mut dots = 0;
    for c in mantissa.bytes() {
        if c.is_ascii_digit() {
            digits += 1;
        } else if c == b'.' {
            dots += 1;
        } else {
            return None;
        }
    }
    if digits == 0 || dots > 1 {
        return None;
    }
    value.trim_matches(is_whitespace).parse().ok()
}

fn decimal(text: &str) -> bool {
    let mut parts = text.split('.');
    let whole = parts.next().unwrap_or("");
    if whole.is_empty() || !whole.bytes().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if let Some(fraction) = parts.next() {
        if fraction.is_empty()
            || !fraction.bytes().all(|c| c.is_ascii_digit())
            || parts.next().is_some()
        {
            return false;
        }
    }
    true
}

fn arbitrary(value: &str, open: char, close: char) -> Option<(Option<&str>, &str)> {
    let body = value.strip_prefix(open)?.strip_suffix(close)?;
    if body.is_empty() || body.contains(['\n', '\r', '\u{2028}', '\u{2029}']) {
        return None;
    }
    if let Some((label, tail)) = body.split_once(':') {
        let mut bytes = label.bytes();
        if !tail.is_empty()
            && bytes
                .next()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_')
            && bytes.all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
        {
            return Some((Some(label), tail));
        }
    }
    Some((None, body))
}

fn css_function(value: &str, names: &[&str]) -> bool {
    names.iter().any(|name| {
        value
            .strip_prefix(name)
            .is_some_and(|tail| tail.starts_with('(') && tail.ends_with(')') && tail.len() > 2)
    })
}

fn length(value: &str) -> bool {
    if css_function(
        value,
        &[
            "rgb",
            "rgba",
            "hsl",
            "hsla",
            "hwb",
            "lab",
            "lch",
            "oklab",
            "oklch",
            "color-mix",
        ],
    ) {
        return false;
    }
    if value == "0" {
        return true;
    }
    const UNITS: [&str; 37] = [
        "%", "px", "em", "rem", "vh", "vw", "vi", "vb", "vmin", "vmax", "svh", "svw", "svi", "svb",
        "svmin", "svmax", "dvh", "dvw", "dvi", "dvb", "dvmin", "dvmax", "lvh", "lvw", "lvi", "lvb",
        "lvmin", "lvmax", "pt", "pc", "in", "cm", "mm", "cap", "ch", "ex", "lh",
    ];
    let bytes = value.as_bytes();
    for (start, &c) in bytes.iter().enumerate() {
        if c.is_ascii_digit() {
            let end = start
                + bytes[start..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();
            if UNITS
                .iter()
                .chain(["rlh", "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax"].iter())
                .any(|unit| value[end..].starts_with(unit))
            {
                return true;
            }
        }
    }
    for name in ["calc", "min", "max", "clamp"] {
        for (start, _) in value.match_indices(name) {
            if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
                continue;
            }
            let tail = &value[start + name.len()..];
            if tail.starts_with('(') && tail.rfind(')').is_some_and(|end| end > 1) {
                return true;
            }
        }
    }
    false
}

fn shadow(value: &str) -> bool {
    let value = value.strip_prefix("inset_").unwrap_or(value);
    let component = |text: &str| {
        let text = text.strip_prefix('-').unwrap_or(text);
        if text == "0" {
            return true;
        }
        let unit_at = text.bytes().position(|c| !c.is_ascii_digit() && c != b'.');
        let Some(unit_at) = unit_at else {
            return false;
        };
        let (numeric, unit) = text.split_at(unit_at);
        !unit.is_empty()
            && unit.bytes().all(|c| c.is_ascii_lowercase())
            && numeric.bytes().any(|c| c.is_ascii_digit())
            && numeric.bytes().filter(|c| *c == b'.').count() <= 1
    };
    let Some((first, tail)) = value.split_once('_') else {
        return false;
    };
    if !component(first) {
        return false;
    }
    let end = tail
        .bytes()
        .take_while(|c| c.is_ascii_digit() || *c == b'.' || *c == b'-' || c.is_ascii_lowercase())
        .count();
    component(&tail[..end])
}

/// Evaluate a built-in marker by its fixed opcode.
pub fn test(opcode: usize, value: &str) -> bool {
    let bracket = arbitrary(value, '[', ']');
    let variable = arbitrary(value, '(', ')');
    match opcode {
        0 => true,
        1 => bracket.is_none() && variable.is_none(),
        2 => bracket.is_some(),
        3 => variable.is_some(),
        4 => value
            .split_once('/')
            .is_some_and(|(left, right)| decimal(left) && decimal(right)),
        5 => number(value).is_some(),
        6 => number(value).is_some_and(|v| v.is_finite() && v.fract() == 0.0),
        7 => value.strip_suffix('%').is_some_and(|v| number(v).is_some()),
        8 => ["xs", "sm", "md", "lg", "xl"].iter().any(|suffix| {
            value
                .strip_suffix(suffix)
                .is_some_and(|v| v.is_empty() || decimal(v))
        }),
        9 => {
            value
                .strip_prefix("@container/")
                .is_some_and(|v| !v.is_empty())
                || value
                    .strip_prefix("@container-size/")
                    .is_some_and(|v| !v.is_empty())
                || value
                    .strip_prefix("@container-normal/")
                    .is_some_and(|v| !v.is_empty())
        }
        10..=24 => {
            let kind = if opcode < 18 { bracket } else { variable };
            let Some((label, body)) = kind else {
                return false;
            };
            let labels: &[&str] = match opcode {
                10 | 18 => &["length"],
                11 => &["number"],
                12 | 24 => &["number", "weight"],
                13 | 19 => &["family-name"],
                14 | 20 => &["position", "percentage"],
                15 | 21 => &["length", "size", "bg-size"],
                16 | 22 => &["image", "url"],
                17 | 23 => &["shadow"],
                _ => &[],
            };
            if let Some(label) = label {
                return labels.contains(&label);
            }
            match opcode {
                10 => length(body),
                11 => number(body).is_some(),
                12 | 23 | 24 => true,
                16 => css_function(
                    body,
                    &[
                        "url",
                        "image",
                        "image-set",
                        "cross-fade",
                        "element",
                        "linear-gradient",
                        "radial-gradient",
                        "conic-gradient",
                        "repeating-linear-gradient",
                        "repeating-radial-gradient",
                        "repeating-conic-gradient",
                    ],
                ),
                17 => shadow(body),
                _ => false,
            }
        }
        _ => false,
    }
}
