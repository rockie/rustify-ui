//! Lossless ECMAScript text and JSON string codecs, including lone UTF-16 surrogates.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;
use std::{cmp::Ordering, fmt, string::FromUtf16Error};

/// Owned UTF-16 code units. Conversion to a Rust string is explicitly fallible.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Text(Vec<u16>);

impl Text {
    /// Appends scalar text without altering existing UTF-16 units.
    pub fn push_str(&mut self, value: &str) {
        self.0.extend(value.encode_utf16());
    }

    /// Appends arbitrary ECMAScript text.
    pub fn push_text(&mut self, value: &Self) {
        self.0.extend_from_slice(&value.0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, value: &str) -> bool {
        let needle: Vec<_> = value.encode_utf16().collect();
        needle.is_empty() || self.0.windows(needle.len()).any(|window| window == needle)
    }

    pub fn starts_with(&self, value: &str) -> bool {
        self.0
            .iter()
            .copied()
            .zip(value.encode_utf16())
            .all(|(a, b)| a == b)
            && self.0.len() >= value.encode_utf16().count()
    }

    /// Matches a scalar suffix against the original code units.
    pub fn ends_with(&self, value: &str) -> bool {
        let length = value.encode_utf16().count();
        self.0.len() >= length
            && self.0[self.0.len() - length..]
                .iter()
                .copied()
                .eq(value.encode_utf16())
    }

    /// Removes a scalar prefix using ECMAScript unit indexes.
    pub fn strip_prefix(&self, prefix: &str) -> Option<Self> {
        self.starts_with(prefix)
            .then(|| Self::from_units(self.0[prefix.encode_utf16().count()..].to_vec()))
    }

    /// Removes a scalar suffix without normalizing isolated units.
    pub fn strip_suffix(&self, suffix: &str) -> Option<Self> {
        self.ends_with(suffix).then(|| {
            Self::from_units(self.0[..self.0.len() - suffix.encode_utf16().count()].to_vec())
        })
    }

    /// Finds the first occurrence and returns a UTF-16 unit index.
    pub fn find(&self, needle: &str) -> Option<usize> {
        let needle: Vec<_> = needle.encode_utf16().collect();
        if needle.is_empty() {
            return Some(0);
        }
        self.0
            .windows(needle.len())
            .position(|units| units == needle)
    }

    /// Finds the last occurrence and returns a UTF-16 unit index.
    pub fn rfind(&self, needle: &str) -> Option<usize> {
        let needle: Vec<_> = needle.encode_utf16().collect();
        if needle.is_empty() {
            return Some(self.0.len());
        }
        self.0
            .windows(needle.len())
            .rposition(|units| units == needle)
    }

    pub fn split_once(&self, separator: &str) -> Option<(Self, Self)> {
        self.find(separator).map(|index| {
            (
                Self::from_units(self.0[..index].to_vec()),
                Self::from_units(self.0[index + separator.encode_utf16().count()..].to_vec()),
            )
        })
    }

    pub fn is_js_whitespace(unit: u16) -> bool {
        matches!(unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279)
    }

    /// Matches JavaScript String.trim's whitespace set.
    pub fn trim_js(&self) -> Self {
        let whitespace = |unit: &u16| Self::is_js_whitespace(*unit);
        let start = self.0.iter().take_while(|unit| whitespace(unit)).count();
        let end = self.0.len()
            - self.0[start..]
                .iter()
                .rev()
                .take_while(|unit| whitespace(unit))
                .count();
        Self::from_units(self.0[start..end].to_vec())
    }

    /// Literal replace-all, preserving original units and leaving dollar markers literal.
    pub fn replace_text(&self, from: &Self, replacement: &Self) -> Self {
        let mut output = Vec::new();
        let mut index = 0;
        if from.is_empty() {
            output.extend_from_slice(replacement.units());
            for &unit in &self.0 {
                output.push(unit);
                output.extend_from_slice(replacement.units());
            }
            return Self::from_units(output);
        }
        while index < self.0.len() {
            if self.0[index..].starts_with(from.units()) {
                output.extend_from_slice(replacement.units());
                index += from.units().len();
            } else {
                output.push(self.0[index]);
                index += 1;
            }
        }
        Self::from_units(output)
    }

    pub fn replace_ascii_sequence(&self, from: &str, replacement: &str) -> Self {
        self.replace_text(&Self::from(from), &Self::from(replacement))
    }

    /// ASCII case folding leaves all other code units untouched.
    pub fn eq_ignore_ascii_case(&self, value: &str) -> bool {
        fn fold(unit: u16) -> u16 {
            if (65..=90).contains(&unit) {
                unit + 32
            } else {
                unit
            }
        }
        self.0
            .iter()
            .copied()
            .map(fold)
            .eq(value.encode_utf16().map(fold))
    }

    /// Trims ASCII separators without converting arbitrary ECMAScript text.
    pub fn trim_ascii(&self) -> Self {
        let whitespace = |unit: &u16| matches!(*unit, 9..=13 | 32);
        let start = self.0.iter().take_while(|unit| whitespace(unit)).count();
        let end = self.0.len()
            - self.0[start..]
                .iter()
                .rev()
                .take_while(|unit| whitespace(unit))
                .count();
        Self::from_units(self.0[start..end].to_vec())
    }

    /// Trims Rust Unicode whitespace while retaining non-scalar units.
    pub fn trim_whitespace(&self) -> Self {
        let whitespace =
            |unit: &u16| char::from_u32(u32::from(*unit)).is_some_and(char::is_whitespace);
        let start = self.0.iter().take_while(|unit| whitespace(unit)).count();
        let end = self.0.len()
            - self.0[start..]
                .iter()
                .rev()
                .take_while(|unit| whitespace(unit))
                .count();
        Self::from_units(self.0[start..end].to_vec())
    }

    /// Replaces one ASCII code unit, preserving every other original unit.
    pub fn replace_ascii(&self, from: u8, replacement: &str) -> Self {
        let mut result = Self::default();
        for &unit in &self.0 {
            if unit == u16::from(from) {
                result.push_str(replacement);
            } else {
                result.0.push(unit);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.byte_len()
    }
    /// Retains the supplied allocation and every code unit.
    pub fn from_units(units: Vec<u16>) -> Self {
        Self(units)
    }

    /// Borrows the original code units, for explicit ECMAScript indexing/comparison.
    pub fn units(&self) -> &[u16] {
        &self.0
    }

    /// Returns the original allocation without converting its contents.
    pub fn into_units(self) -> Vec<u16> {
        self.0
    }

    /// Converts scalar text to UTF-8; lone surrogates return an error.
    pub fn to_utf8(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(&self.0)
    }

    /// UTF-8 length for scalar text, extended to three bytes per lone surrogate.
    /// This measures logical text, independently of allocation or JSON escaping.
    pub fn byte_len(&self) -> usize {
        char::decode_utf16(self.0.iter().copied())
            .map(|point| point.map_or(3, char::len_utf8))
            .sum()
    }

    /// Owned allocation bytes; the enclosing structure accounts for its own header.
    pub fn owned_bytes(&self) -> usize {
        self.0.capacity() * size_of::<u16>()
    }

    pub(crate) fn write_json_string(&self, output: &mut String) {
        fn hex_escape(output: &mut String, unit: u16) {
            const HEX: &[u8; 16] = b"0123456789abcdef";
            output.push_str("\\u");
            for shift in [12, 8, 4, 0] {
                output.push(char::from(HEX[usize::from((unit >> shift) & 15)]));
            }
        }

        output.push('"');
        for point in char::decode_utf16(self.0.iter().copied()) {
            match point {
                Ok('"') => output.push_str("\\\""),
                Ok('\\') => output.push_str("\\\\"),
                Ok('\u{08}') => output.push_str("\\b"),
                Ok('\t') => output.push_str("\\t"),
                Ok('\n') => output.push_str("\\n"),
                Ok('\u{0c}') => output.push_str("\\f"),
                Ok('\r') => output.push_str("\\r"),
                Ok(ch) if ch <= '\u{1f}' => hex_escape(output, ch as u16),
                Ok(ch) => output.push(ch),
                Err(error) => hex_escape(output, error.unpaired_surrogate()),
            }
        }
        output.push('"');
    }

    // serde_json's byte-string visitor delivers WTF-8, including surrogate code points.
    // Validate its byte sequences without ever constructing an invalid Rust String.
    fn from_wtf8(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut units = Vec::with_capacity(bytes.len());
        let mut offset = 0;
        while offset < bytes.len() {
            let first = bytes[offset];
            let (length, mut point, minimum) = match first {
                0x00..=0x7f => (1, u32::from(first), 0),
                0xc2..=0xdf => (2, u32::from(first & 0x1f), 0x80),
                0xe0..=0xef => (3, u32::from(first & 0x0f), 0x800),
                0xf0..=0xf4 => (4, u32::from(first & 0x07), 0x10000),
                _ => return Err("invalid WTF-8 leading byte"),
            };
            let Some(sequence) = bytes.get(offset..offset + length) else {
                return Err("incomplete WTF-8 sequence");
            };
            for byte in &sequence[1..] {
                if byte & 0xc0 != 0x80 {
                    return Err("invalid WTF-8 continuation byte");
                }
                point = (point << 6) | u32::from(byte & 0x3f);
            }
            if point < minimum || point > 0x10ffff {
                return Err("invalid WTF-8 code point");
            }
            if point <= 0xffff {
                units.push(point as u16);
            } else {
                point -= 0x10000;
                units.push(0xd800 | (point >> 10) as u16);
                units.push(0xdc00 | (point & 0x3ff) as u16);
            }
            offset += length;
        }
        Ok(Self(units))
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(value.encode_utf16().collect())
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&Text> for Text {
    fn from(value: &Text) -> Self {
        value.clone()
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.0.iter().copied().eq(other.encode_utf16())
    }
}
impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}
impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        fn points(text: &Text) -> impl Iterator<Item = u32> + '_ {
            char::decode_utf16(text.0.iter().copied()).map(|point| {
                point.map_or_else(|error| u32::from(error.unpaired_surrogate()), u32::from)
            })
        }
        // Preserve Rust String's scalar/code-point order. Source algorithms that use
        // charCodeAt must compare units() explicitly rather than using this order.
        points(self).cmp(points(other))
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Serialize for Text {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Ok(scalar) = self.to_utf8() {
            return serializer.serialize_str(&scalar);
        }
        let mut token = String::with_capacity(self.byte_len() + 2);
        self.write_json_string(&mut token);
        RawValue::from_string(token)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StringVisitor;
        impl de::Visitor<'_> for StringVisitor {
            type Value = Text;

            fn expecting(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
                output.write_str("a string containing UTF-16 text")
            }

            fn visit_bytes<E: de::Error>(self, bytes: &[u8]) -> Result<Text, E> {
                Text::from_wtf8(bytes).map_err(E::custom)
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Text, E> {
                Ok(Text::from(value))
            }
        }
        // JSON's ordinary string visitor rejects lone surrogates. Its bytes visitor
        // decodes them losslessly; omitting visit_seq keeps the schema string-only.
        deserializer.deserialize_bytes(StringVisitor)
    }
}

/// JSON object codec for `BTreeMap<Text, Text>`, including non-scalar keys.
/// Use `#[serde(with = "crate::text::map")]` on the map field.
pub mod map {
    use super::{RawValue, Text};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    /// Writes a JSON object with lossless string keys and values.
    pub fn serialize<S: Serializer>(
        map: &BTreeMap<Text, Text>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        // MapKeySerializer cannot serialize RawValue. Serialize this one object as
        // real JSON syntax, preserving the map's existing code-point order.
        let mut token = String::from("{");
        for (index, (key, value)) in map.iter().enumerate() {
            if index > 0 {
                token.push(',');
            }
            key.write_json_string(&mut token);
            token.push(':');
            value.write_json_string(&mut token);
        }
        token.push('}');
        RawValue::from_string(token)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }

    /// Reads a JSON object directly, without a scalar-only intermediate value.
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<BTreeMap<Text, Text>, D::Error> {
        BTreeMap::deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{map, Text};
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Report {
        css: Text,
        #[serde(with = "map")]
        gpu_variables: BTreeMap<Text, Text>,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Reply {
        token: u32,
        report: Report,
    }
    #[test]
    fn all_code_units_survive_two_roundtrips_and_nested_report() {
        let units = (0..=u16::MAX).collect::<Vec<_>>();
        let report = Reply {
            token: 7,
            report: Report {
                css: Text::from_units(units.clone()),
                gpu_variables: BTreeMap::from([
                    (
                        Text::from_units(vec![0xd800]),
                        Text::from_units(vec![0xdc00]),
                    ),
                    (Text::from("�"), Text::from("😀")),
                    (Text::from("\u{e000}"), Text::from("literal \\ud800")),
                ]),
            },
        };
        let first = serde_json::to_string(&report).unwrap();
        let parsed: Reply = serde_json::from_str(&first).unwrap();
        assert_eq!(parsed, report);
        let second = serde_json::to_string(&parsed).unwrap();
        let parsed: Reply = serde_json::from_str(&second).unwrap();
        assert_eq!(parsed, report);
    }
    #[test]
    fn scalar_json_bytes_match_existing_string_and_maps() {
        for scalar in [
            "ASCII: \\ \" \n\t\0",
            "😀",
            "\u{e000}",
            "�",
            "literal \\ud800",
            "line\u{2028}separator",
        ] {
            assert_eq!(
                serde_json::to_string(&Text::from(scalar)).unwrap(),
                serde_json::to_string(scalar).unwrap()
            );
        }
        let native = BTreeMap::from([
            ("\u{e000}".to_owned(), "😀".to_owned()),
            ("😀".to_owned(), "\0\n\t\u{0001}\u{001f}".to_owned()),
        ]);
        let report = Report {
            css: Text::from(""),
            gpu_variables: native
                .iter()
                .map(|(k, v)| (Text::from(k.as_str()), Text::from(v.as_str())))
                .collect(),
        };
        let expected = format!(
            "{{\"css\":\"\",\"gpu_variables\":{}}}",
            serde_json::to_string(&native).unwrap()
        );
        assert_eq!(serde_json::to_string(&report).unwrap(), expected);
    }
    #[test]
    fn byte_length_preserves_scalar_utf8_and_counts_lone_units_as_three() {
        for scalar in ["", "ABC", "é", "字", "😀", "é😀\u{e000}"] {
            assert_eq!(Text::from(scalar).byte_len(), scalar.len());
        }
        assert_eq!(Text::from_units(vec![0xd800]).byte_len(), 3);
        assert_eq!(
            Text::from_units(vec![0xdc00, 0xd800, 0xd83d, 0xde00]).byte_len(),
            10
        );
    }
    #[test]
    fn ordering_preserves_scalar_codepoint_order_and_distinguishes_unit_order() {
        let astral = Text::from("😀");
        let private_use = Text::from("\u{e000}");
        assert!(astral > private_use);
        assert!(astral.units() < private_use.units());
        assert!(Text::from_units(vec![0xd800]) < private_use);
        assert_ne!(Text::from_units(vec![0xd800]), Text::from("�"));
    }
    #[test]
    fn strict_scalar_conversion_rejects_lone_units_and_consuming_api_preserves_capacity() {
        assert!(Text::from_units(vec![0xd800]).to_utf8().is_err());
        assert_eq!(Text::from("😀").to_utf8().unwrap(), "😀");
        let mut units = Vec::with_capacity(32);
        units.extend([0xd800, 0xdc00]);
        let capacity = units.capacity();
        let text = Text::from_units(units);
        assert_eq!(text.owned_bytes(), capacity * 2);
        assert_eq!(text.units(), [0xd800, 0xdc00]);
        assert_eq!(text.into_units().capacity(), capacity);
    }
    #[test]
    fn deserialization_accepts_real_json_lone_units_and_rejects_nonstring_and_invalid_wtf8() {
        assert_eq!(
            serde_json::from_str::<Text>(r#""\ud800\ud800\udc00\udc00""#)
                .unwrap()
                .units(),
            [0xd800, 0xd800, 0xdc00, 0xdc00]
        );
        for invalid in ["[55296]", "null", "0", "true", "{}"] {
            assert!(serde_json::from_str::<Text>(invalid).is_err());
        }
        for bytes in [
            b"\"\xc0\x80\"".as_slice(),
            b"\"\xf4\x90\x80\x80\"".as_slice(),
            b"\"\xe0\x80\x80\"".as_slice(),
        ] {
            assert!(serde_json::from_slice::<Text>(bytes).is_err());
        }
    }
    #[test]
    fn whole_object_codec_matches_every_scalar_bmp_json_escape() {
        let scalar = (0..=u16::MAX)
            .filter_map(|u| char::from_u32(u.into()))
            .collect::<String>();
        let native = BTreeMap::from([(scalar.clone(), scalar.clone())]);
        let report = Report {
            css: Text::from(""),
            gpu_variables: BTreeMap::from([(Text::from(scalar.as_str()), Text::from(scalar))]),
        };
        assert_eq!(
            serde_json::to_string(&report).unwrap(),
            format!(
                "{{\"css\":\"\",\"gpu_variables\":{}}}",
                serde_json::to_string(&native).unwrap()
            )
        );
    }
}
