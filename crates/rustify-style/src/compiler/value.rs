//! CSS value functions and top-level segmentation retain their original text.

use super::CompileError;
use serde::{Deserialize, Serialize};

use crate::text::Text;

/// Concatenates CSS fragments without changing their UTF-16 code units.
pub(super) fn concat_text(parts: &[&Text]) -> Text {
    let mut units = Vec::with_capacity(parts.iter().map(|part| part.units().len()).sum());
    for part in parts {
        units.extend_from_slice(part.units());
    }
    Text::from_units(units)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValueNode {
    Word { value: Text },
    Separator { value: Text },
    Function { name: Text, nodes: Vec<ValueNode> },
}

impl<'de> Deserialize<'de> for ValueNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Word,
            Separator,
            Function,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            value: Option<Text>,
            name: Option<Text>,
            nodes: Option<Vec<ValueNode>>,
        }
        // Direct typed fields bypass tagged enum's scalar-string Content buffer.
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Word => Self::Word {
                value: fields
                    .value
                    .ok_or_else(|| serde::de::Error::missing_field("value"))?,
            },
            Kind::Separator => Self::Separator {
                value: fields
                    .value
                    .ok_or_else(|| serde::de::Error::missing_field("value"))?,
            },
            Kind::Function => Self::Function {
                name: fields
                    .name
                    .ok_or_else(|| serde::de::Error::missing_field("name"))?,
                nodes: fields
                    .nodes
                    .ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
            },
        })
    }
}

pub fn parse_text(
    input: &crate::text::Text,
    max_depth: usize,
) -> Result<Vec<ValueNode>, CompileError> {
    use crate::text::Text;
    fn flush(nodes: &mut Vec<ValueNode>, word: &mut Vec<u16>) {
        if !word.is_empty() {
            nodes.push(ValueNode::Word {
                value: Text::from_units(std::mem::take(word)),
            });
        }
    }
    fn separator(unit: u16) -> bool {
        matches!(unit, 58 | 44 | 61 | 62 | 60 | 10 | 47 | 32 | 9)
    }
    let mut normalized = Vec::new();
    let input = if input.units().windows(2).any(|pair| pair == [13, 10]) {
        let mut units = input.units().iter().copied().peekable();
        while let Some(unit) = units.next() {
            if unit == 13 && units.peek() == Some(&10) {
                continue;
            }
            normalized.push(unit);
        }
        normalized.as_slice()
    } else {
        input.units()
    };
    let mut nodes = Vec::new();
    let mut parents = Vec::new();
    let mut word = Vec::new();
    let mut offset = 0;
    while let Some(&unit) = input.get(offset) {
        offset += 1;
        match unit {
            92 => {
                word.push(unit);
                if let Some(&next) = input.get(offset) {
                    word.push(next);
                    offset += 1;
                } else {
                    word.extend("undefined".encode_utf16());
                }
            }
            39 | 34 => {
                let mut scan = offset;
                let mut end = None;
                while let Some(&next) = input.get(scan) {
                    scan += 1;
                    if next == 92 {
                        scan += usize::from(scan < input.len());
                    } else if next == unit {
                        end = Some(scan);
                        break;
                    }
                }
                if let Some(end) = end {
                    word.extend_from_slice(&input[offset - 1..end]);
                    offset = end;
                } else {
                    word.push(unit);
                }
            }
            40 => {
                if parents.len() >= max_depth {
                    return Err(CompileError::Limit {
                        resource: "ast_depth".into(),
                        limit: max_depth,
                    });
                }
                parents.push((
                    Text::from_units(std::mem::take(&mut word)),
                    std::mem::take(&mut nodes),
                ));
            }
            41 => {
                if let Some((name, mut parent)) = parents.pop() {
                    flush(&mut nodes, &mut word);
                    parent.push(ValueNode::Function { name, nodes });
                    nodes = parent;
                } else {
                    word.clear();
                }
            }
            unit if separator(unit) => {
                flush(&mut nodes, &mut word);
                let start = offset - 1;
                while input.get(offset).is_some_and(|unit| separator(*unit)) {
                    offset += 1;
                }
                nodes.push(ValueNode::Separator {
                    value: Text::from_units(input[start..offset].to_vec()),
                });
            }
            _ => word.push(unit),
        }
    }
    while let Some((name, mut parent)) = parents.pop() {
        parent.push(ValueNode::Function { name, nodes });
        nodes = parent;
    }
    flush(&mut nodes, &mut word);
    Ok(nodes)
}

pub fn to_css_text(nodes: &[ValueNode]) -> crate::text::Text {
    use crate::text::Text;
    let mut output = Text::default();
    for node in nodes {
        match node {
            ValueNode::Word { value } | ValueNode::Separator { value } => output.push_text(value),
            ValueNode::Function { name, nodes } => {
                output.push_text(name);
                output.push_str("(");
                output.push_text(&to_css_text(nodes));
                output.push_str(")");
            }
        }
    }
    output
}

pub fn parse(input: &str, max_depth: usize) -> Result<Vec<ValueNode>, CompileError> {
    parse_text(&Text::from(input), max_depth)
}

pub fn to_css(nodes: &[ValueNode]) -> Text {
    to_css_text(nodes)
}

pub(super) fn scalar_text(text: &Text) -> Result<String, CompileError> {
    text.to_utf8().map_err(|_| CompileError::PendingFeature {
        feature: "non-scalar value passed to scalar compiler consumer".into(),
    })
}

/// Splits ECMAScript text at an ASCII delimiter outside strings and brackets.
/// JavaScript split(/\s+/g), retaining empty edge fields and isolated units.
pub(super) fn split_whitespace_text(input: &Text) -> Vec<Text> {
    let units = input.units();
    let mut fields = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < units.len() {
        if Text::is_js_whitespace(units[index]) {
            fields.push(Text::from_units(units[start..index].to_vec()));
            while index < units.len() && Text::is_js_whitespace(units[index]) {
                index += 1;
            }
            start = index;
        } else {
            index += 1;
        }
    }
    fields.push(Text::from_units(units[start..].to_vec()));
    fields
}

pub fn segment_text(
    input: &crate::text::Text,
    delimiter: u16,
    max_depth: usize,
) -> Result<Vec<crate::text::Text>, CompileError> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut brackets = Vec::new();
    let mut offset = 0;
    let units = input.units();
    while let Some(&unit) = units.get(offset) {
        if brackets.is_empty() && unit == delimiter {
            parts.push(crate::text::Text::from_units(units[start..offset].to_vec()));
            start = offset + 1;
            offset += 1;
            continue;
        }
        match unit {
            92 => offset += 1,
            39 | 34 => {
                offset += 1;
                while let Some(&next) = units.get(offset) {
                    if next == 92 {
                        offset += 2;
                    } else if next == unit {
                        break;
                    } else {
                        offset += 1;
                    }
                }
            }
            40 | 91 | 123 => {
                brackets.push(match unit {
                    40 => 41,
                    91 => 93,
                    _ => 125,
                });
                if brackets.len() > max_depth {
                    return Err(CompileError::Limit {
                        resource: "ast_depth".into(),
                        limit: max_depth,
                    });
                }
            }
            41 | 93 | 125 if brackets.last() == Some(&unit) => {
                brackets.pop();
            }
            _ => {}
        }
        offset += 1;
    }
    parts.push(crate::text::Text::from_units(units[start..].to_vec()));
    Ok(parts)
}

pub fn segment(input: &str, delimiter: char, max_depth: usize) -> Result<Vec<&str>, CompileError> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut brackets = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    for (offset, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
            continue;
        }
        if matches!(ch, '"' | '\'') {
            quote = Some(ch);
            continue;
        }
        match ch {
            '(' | '[' | '{' => {
                brackets.push(match ch {
                    '(' => ')',
                    '[' => ']',
                    _ => '}',
                });
                if brackets.len() > max_depth {
                    return Err(CompileError::Limit {
                        resource: "ast_depth".into(),
                        limit: max_depth,
                    });
                }
            }
            ')' | ']' | '}' => {
                if brackets.last() == Some(&ch) {
                    brackets.pop();
                }
            }
            _ if ch == delimiter && brackets.is_empty() => {
                parts.push(&input[start..offset]);
                start = offset + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&input[start..]);
    Ok(parts)
}

pub fn decode_arbitrary(input: &str, max_depth: usize) -> Result<String, CompileError> {
    scalar_text(&decode_arbitrary_text(&Text::from(input), max_depth)?)
}

pub fn decode_arbitrary_text(input: &Text, max_depth: usize) -> Result<Text, CompileError> {
    fn underscores(text: &Text, preserve: bool) -> Text {
        let mut units = text.units().iter().copied().peekable();
        let mut output = Vec::with_capacity(text.units().len());
        while let Some(unit) = units.next() {
            if unit == 92 && units.peek() == Some(&95) {
                units.next();
                output.push(95);
            } else {
                output.push(if unit == 95 && !preserve { 32 } else { unit });
            }
        }
        Text::from_units(output)
    }
    fn decode(nodes: &mut [ValueNode]) {
        for node in nodes {
            match node {
                ValueNode::Word { value } | ValueNode::Separator { value } => {
                    *value = underscores(value, false)
                }
                ValueNode::Function { name, nodes } => {
                    let url = name == "url" || name.ends_with("_url");
                    let variable = name == "var"
                        || name.ends_with("_var")
                        || name == "theme"
                        || name.ends_with("_theme");
                    *name = underscores(name, false);
                    if url {
                        continue;
                    }
                    if variable {
                        if let Some(ValueNode::Word { value }) = nodes.first_mut() {
                            *value = underscores(value, true);
                            decode(&mut nodes[1..]);
                            continue;
                        }
                    }
                    decode(nodes);
                }
            }
        }
    }
    if !input.contains("(") {
        return Ok(underscores(input, false));
    }
    let mut ast = parse_text(input, max_depth)?;
    decode(&mut ast);
    Ok(format_math(&to_css_text(&ast)))
}

// Port of utils/math-operators.ts at the fixed upstream commit. Formatting
// follows the enclosing function, including unary signs and scientific numbers.
fn format_math(input: &Text) -> Text {
    const FUNCTIONS: &[&str] = &[
        "calc", "min", "max", "clamp", "mod", "rem", "sin", "cos", "tan", "asin", "acos", "atan",
        "atan2", "pow", "sqrt", "hypot", "log", "exp", "round",
    ];
    if !FUNCTIONS.iter().any(|name| input.contains(name)) {
        return input.clone();
    }
    let chars = input.units();
    let mut result: Vec<u16> = Vec::with_capacity(chars.len());
    let mut formattable = Vec::new();
    let mut value_position = None;
    let mut last_value_position = None;
    let operator = |ch: u16| matches!(ch, 43 | 45 | 42 | 47);
    for (index, &ch) in chars.iter().enumerate() {
        if (48..=57).contains(&ch)
            || (value_position.is_some()
                && (ch == 37 || ((65..=90).contains(&ch) || (97..=122).contains(&ch))))
        {
            value_position = Some(index);
        } else {
            last_value_position = value_position;
            value_position = None;
        }
        let formatting = formattable.last().copied().unwrap_or(false);
        if ch == 40 {
            result.push(ch);
            let start = (0..index)
                .rev()
                .take_while(|&i| (48..=57).contains(&chars[i]) || (97..=122).contains(&chars[i]))
                .last()
                .unwrap_or(index);
            let name = Text::from_units(chars[start..index].to_vec());
            formattable.push(
                FUNCTIONS.iter().any(|known| name == *known) || (formatting && name.is_empty()),
            );
        } else if ch == 41 {
            result.push(ch);
            formattable.pop();
        } else if ch == 44 && formatting {
            result.extend([44, 32]);
        } else if ch == 32 && formatting && result.ends_with(&[32]) {
            continue;
        } else if operator(ch) && formatting {
            let mut previous = result.iter().copied().rev().skip_while(|unit| {
                matches!(*unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279)
            });
            let prev = previous.next();
            let prev_prev = previous.next();
            let next = chars.get(index + 1).copied();
            if (matches!(prev, Some(101 | 69)) && prev_prev.is_some_and(|c| (48..=57).contains(&c)))
                || prev.is_some_and(operator)
                || matches!(prev, Some(40 | 44))
            {
                result.push(ch);
            } else if index > 0 && chars[index - 1] == 32 {
                result.push(ch);
                result.push(32);
            } else if prev.is_some_and(|c| (48..=57).contains(&c) || c == 41)
                || next.is_some_and(|c| (48..=57).contains(&c) || c == 40 || operator(c))
                || (index > 0 && last_value_position == Some(index - 1))
            {
                result.push(32);
                result.push(ch);
                result.push(32);
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    Text::from_units(result)
}

pub fn is_valid_arbitrary(input: &str) -> bool {
    is_valid_arbitrary_text(&input.into())
}

/// Checks arbitrary-value syntax without requiring scalar Unicode text.
pub fn is_valid_arbitrary_text(input: &Text) -> bool {
    is_valid_arbitrary_units(input.units())
}

pub(super) fn is_valid_arbitrary_units(input: &[u16]) -> bool {
    // Match the fixed upstream Uint8Array(256), including ignored overflow
    // writes and unmatched reads. An overflowing stack cannot unwind.
    let mut stack = [0u8; 256];
    let mut depth = 0usize;
    let mut index = 0usize;
    while let Some(&unit) = input.get(index) {
        index += 1;
        match unit {
            92 => index += 1,
            34 | 39 => {
                while let Some(&next) = input.get(index) {
                    index += 1;
                    if next == 92 {
                        index += 1;
                    } else if next == unit {
                        break;
                    }
                }
            }
            40 | 91 => {
                if let Some(closer) = stack.get_mut(depth) {
                    *closer = if unit == 40 { 41 } else { 93 };
                }
                depth += 1;
            }
            41 | 93 | 125 => {
                if depth == 0 {
                    return false;
                }
                if stack.get(depth - 1).copied().map(u16::from) == Some(unit) {
                    depth -= 1;
                }
            }
            59 if depth == 0 => return false,
            _ => {}
        }
    }
    true
}
pub(super) fn js_whitespace(c: char) -> bool {
    matches!(c, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

#[cfg(test)]
mod arbitrary_validity_tests {
    use super::*;

    #[test]
    fn scalar_convenience_uses_the_text_validity_contract() {
        for input in [
            "",
            "plain",
            "😀",
            "\u{feff} \t\n",
            "()",
            "[a]",
            "{a}",
            "([)]",
            "(]",
            "(",
            "'",
            "\\",
            "\\;",
            "\";)]}\"",
            "a;b",
            "f(a;b)",
        ] {
            assert_eq!(
                is_valid_arbitrary(input),
                is_valid_arbitrary_text(&input.into()),
                "{input:?}"
            );
        }
        for unit in 0..=u16::MAX {
            if let Some(character) = char::from_u32(u32::from(unit)) {
                let input = character.to_string();
                assert_eq!(
                    is_valid_arbitrary(&input),
                    is_valid_arbitrary_units(&[unit]),
                    "unit {unit:04x}"
                );
            }
        }
    }

    #[test]
    fn opaque_units_follow_the_upstream_bracket_quote_and_escape_rules() {
        let cases: &[(&[u16], bool)] = &[
            (&[0xd800], true),
            (&[0xdc00], true),
            (&[40, 0xd800, 41], true),
            (&[91, 0xdc00, 93], true),
            (&[0xd800, 41], false),
            (&[0xdc00, 93], false),
            (&[123, 0xd800, 125], false),
            (&[40, 125, 41], true),
            (&[40, 93], true),
            (&[40, 91, 41, 93], true),
            (&[40, 0xd800], true),
            (&[34, 0xd800], true),
            (&[0xd800, 92], true),
            (&[92, 0xd800, 41], false),
            (&[92, 41, 0xd800], true),
            (&[92, 59, 0xdc00], true),
            (&[34, 0xd800, 59, 41, 93, 125, 34], true),
            (&[39, 0xdc00, 92, 39, 59, 39], true),
            (&[40, 0xd800, 59, 41], true),
            (&[40, 0xd800, 41, 59], false),
            (&[0xd800, 59, 0xdc00], false),
        ];
        for &(units, expected) in cases {
            assert_eq!(
                is_valid_arbitrary_text(&Text::from_units(units.to_vec())),
                expected,
                "{units:x?}"
            );
            assert_eq!(is_valid_arbitrary_units(units), expected, "{units:x?}");
        }
    }

    #[test]
    fn whitespace_is_opaque_in_arbitrary_validity() {
        for unit in [
            9, 10, 11, 12, 13, 32, 0x85, 0xa0, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004,
            0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000,
            0xfeff,
        ] {
            assert!(
                is_valid_arbitrary_units(&[0xd800, unit, 0xdc00]),
                "unit {unit:04x}"
            );
        }
    }

    #[test]
    fn every_utf16_unit_retains_its_exact_syntax_role() {
        for unit in 0..=u16::MAX {
            let expected = !matches!(unit, 41 | 59 | 93 | 125);
            assert_eq!(
                is_valid_arbitrary_units(&[unit]),
                expected,
                "unit {unit:04x}"
            );
            assert_eq!(
                is_valid_arbitrary_text(&Text::from_units(vec![unit])),
                expected,
                "unit {unit:04x}"
            );
            assert!(
                is_valid_arbitrary_units(&[92, unit]),
                "escaped unit {unit:04x}"
            );
            assert!(
                is_valid_arbitrary_units(&[34, unit, 34]),
                "quoted unit {unit:04x}"
            );
        }
    }

    #[test]
    fn fixed_upstream_bracket_buffer_overflow_does_not_unwind() {
        // The fixed Uint8Array(256) cannot store a closer at depth 257.
        for (depth, expected) in [(255, false), (256, false), (257, true), (258, true)] {
            let input = format!("{}{};", "(".repeat(depth), ")".repeat(depth));
            assert_eq!(is_valid_arbitrary(&input), expected, "depth {depth}");
            assert_eq!(
                is_valid_arbitrary_text(&input.into()),
                expected,
                "depth {depth}"
            );
        }
    }
}
