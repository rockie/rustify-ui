//! Located, data-only declarations in Markdown component and demo fences.

use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, Visitor};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::ops::Range;

use crate::diagnostic::Issue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Component,
    Demo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub kind: Kind,
    pub name: String,
    pub props: serde_json::Value,
    pub children: String,
    pub span: Range<usize>,
    pub properties: BTreeMap<String, Range<usize>>,
}

pub fn parse(
    kind: Kind,
    payload: &str,
    offset: usize,
    span: Range<usize>,
) -> Result<Declaration, Issue> {
    // YAML's own Mapping deserializer rejects duplicate keys recursively.
    // Deserializing directly into JSON would silently keep the last value.
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(payload).map_err(|error| {
        let at = error
            .location()
            .map_or(0, |location| location_offset(payload, &location));
        Issue::error(offset + at..offset + at, "embed-payload", error.to_string())
    })?;
    let value =
        json_value(yaml).map_err(|message| Issue::error(span.clone(), "embed-payload", message))?;
    let fields = value.as_object().ok_or_else(|| {
        Issue::error(
            span.clone(),
            "embed-payload",
            "write a YAML mapping with a registered name",
        )
    })?;
    let locate = |key: &str| key_span(payload, &[key], offset).unwrap_or_else(|| span.clone());
    let error = |key: &str, message: String| Issue::error(locate(key), "embed-payload", message);
    for key in fields.keys() {
        if key != "name" && (kind == Kind::Demo || !matches!(key.as_str(), "props" | "children")) {
            return Err(error(key, format!("unknown field `{key}`; a demo takes only `name`, a component takes `name`, `props` and `children`")));
        }
    }
    let name = fields
        .get("name")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| error("name", "name must be a registered Rust identifier".into()))?;
    if name.is_empty()
        || !name.chars().enumerate().all(|(index, ch)| {
            ch == '_'
                || if index == 0 {
                    ch.is_alphabetic()
                } else {
                    ch.is_alphanumeric()
                }
        })
    {
        return Err(error(
            "name",
            "name must be a registered Rust identifier".into(),
        ));
    }
    let props = fields
        .get("props")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let properties = props
        .as_object()
        .ok_or_else(|| error("props", "props must be a mapping of declared fields".into()))?
        .keys()
        .map(|key| {
            (
                key.clone(),
                key_span(payload, &["props", key], offset).unwrap_or_else(|| span.clone()),
            )
        })
        .collect();
    let children = match fields.get("children") {
        None => String::new(),
        Some(serde_json::Value::String(children)) => children.clone(),
        Some(_) => {
            return Err(error(
                "children",
                "children must be a Markdown string; use a YAML block scalar (`|`)".into(),
            ))
        }
    };
    Ok(Declaration {
        kind,
        name: name.to_owned(),
        props,
        children,
        span,
        properties,
    })
}

fn location_offset(payload: &str, location: &serde_yaml_ng::Location) -> usize {
    let line = payload
        .split_inclusive('\n')
        .take(location.line().saturating_sub(1))
        .map(str::len)
        .sum::<usize>();
    let column = payload[line.min(payload.len())..]
        .chars()
        .take(location.column().saturating_sub(1))
        .map(char::len_utf8)
        .sum::<usize>();
    (line + column).min(payload.len())
}

fn key_span(payload: &str, path: &[&str], offset: usize) -> Option<Range<usize>> {
    let found = Cell::new(false);
    let error = KeyLocation {
        path,
        found: &found,
    }
    .deserialize(serde_yaml_ng::Deserializer::from_str(payload))
    .err()?;
    if !found.get() {
        return None;
    }
    let at = location_offset(payload, &error.location()?);
    let source = &payload[at..];
    let mut chars = source.char_indices().peekable();
    let (_, first) = chars.next()?;
    let key = *path.last()?;
    let mut length = if source.starts_with(key) {
        key.len()
    } else {
        0
    };
    if matches!(first, '\'' | '"') {
        while let Some((at, ch)) = chars.next() {
            if first == '"' && ch == '\\' {
                chars.next();
            } else if ch == first {
                if first == '\'' && chars.peek().is_some_and(|(_, next)| *next == first) {
                    chars.next();
                } else {
                    length = at + ch.len_utf8();
                    break;
                }
            }
        }
    }
    Some(offset + at..offset + at + length)
}

#[derive(Clone, Copy)]
struct KeyLocation<'a> {
    path: &'a [&'a str],
    found: &'a Cell<bool>,
}

impl<'de> DeserializeSeed<'de> for KeyLocation<'_> {
    type Value = bool;

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for KeyLocation<'_> {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a YAML mapping or string key")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut mapping: M) -> Result<bool, M::Error> {
        while let Some(matched) = mapping.next_key_seed(self)? {
            if matched {
                mapping.next_value_seed(Self {
                    path: &self.path[1..],
                    ..self
                })?;
            } else {
                mapping.next_value::<IgnoredAny>()?;
            }
        }
        Ok(false)
    }

    fn visit_str<E: de::Error>(self, key: &str) -> Result<bool, E> {
        if self.path[0] != key {
            return Ok(false);
        }
        if self.path.len() > 1 {
            return Ok(true);
        }
        // Returning inside the key visitor makes serde_yaml attach that scalar's
        // real mark. The complete payload was already validated above.
        self.found.set(true);
        Err(E::custom("located YAML key"))
    }
}

fn json_value(value: serde_yaml_ng::Value) -> Result<serde_json::Value, String> {
    use serde_yaml_ng::Value;
    Ok(match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(value) => value.into(),
        Value::String(value) => value.into(),
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                value.into()
            } else if let Some(value) = value.as_u64() {
                value.into()
            } else {
                serde_json::Number::from_f64(value.as_f64().ok_or("unsupported YAML number")?)
                    .ok_or("props numbers must be finite")?
                    .into()
            }
        }
        Value::Sequence(values) => serde_json::Value::Array(
            values
                .into_iter()
                .map(json_value)
                .collect::<Result<_, _>>()?,
        ),
        Value::Mapping(values) => {
            let mut object = serde_json::Map::new();
            for (key, value) in values {
                let Value::String(key) = key else {
                    return Err("mapping keys must be strings".into());
                };
                object.insert(key, json_value(value)?);
            }
            serde_json::Value::Object(object)
        }
        Value::Tagged(_) => {
            return Err("YAML tags are not supported; use plain props values".into())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_data_and_children_keep_source_locations() {
        let source = "name: Counter\nprops:\n  initial: 2\n  label: Café\nchildren: |\n  A **short** explanation.\n";
        let declaration = parse(Kind::Component, source, 30, 10..130).unwrap();
        assert_eq!(declaration.name, "Counter");
        assert_eq!(
            declaration.props,
            serde_json::json!({"initial":2,"label":"Café"})
        );
        assert_eq!(declaration.children, "A **short** explanation.\n");
        assert_eq!(
            &source[declaration.properties["initial"].start - 30
                ..declaration.properties["initial"].end - 30],
            "initial"
        );
    }

    #[test]
    fn property_locations_ignore_children_comments_strings_and_nested_names() {
        for source in [
            "children: |\n  initial: text\nname: Counter\nprops:\n  initial: 2\n",
            "# initial: text\nname: Counter\nprops:\n  initial: 2\n",
            "children: 'initial: text'\nname: Counter\nprops:\n  initial: 2\n",
            "name: Counter\nprops:\n  metadata: {initial: text}\n  initial: 2\n",
        ] {
            let declaration = parse(Kind::Component, source, 17, 0..source.len()).unwrap();
            let at = source.rfind("initial: 2").unwrap();
            assert_eq!(
                declaration.properties["initial"],
                17 + at..17 + at + "initial".len(),
                "{source}"
            );
        }
    }

    #[test]
    fn invalid_name_locations_ignore_children_comments_and_strings() {
        for source in [
            "children: |\n  name: false\nname: 3\n",
            "# name: false\nname: 3\n",
            "children: 'name: false'\nname: 3\n",
        ] {
            let issue = parse(Kind::Component, source, 23, 0..source.len()).unwrap_err();
            let at = source.rfind("name: 3").unwrap();
            assert_eq!(issue.span, 23 + at..23 + at + "name".len(), "{source}");
        }
    }

    #[test]
    fn quoted_and_flow_property_keys_use_the_actual_written_key_token() {
        for (source, token) in [
            ("name: Counter\nprops:\n  'initial': 2\n", "'initial'"),
            (
                "{children: 'initial: text', name: Counter, props: {\"initial\": 2}}",
                "\"initial\"",
            ),
            (
                "name: Counter\nprops: {\"\\u0069nitial\": 2}\n",
                "\"\\u0069nitial\"",
            ),
            ("name: Counter\nprops: {initial: 2}\n", "initial"),
        ] {
            let declaration = parse(Kind::Component, source, 31, 0..source.len()).unwrap();
            let at = source.rfind(token).unwrap();
            assert_eq!(
                declaration.properties["initial"],
                31 + at..31 + at + token.len(),
                "{source}"
            );
        }
    }

    #[test]
    fn unicode_and_crlf_locations_are_original_byte_offsets() {
        let source = "children: |\r\n  🦀 initial: text\r\n  Café\r\nname: Counter\r\nprops: {label: 資料, initial: 2}\r\n";
        let declaration = parse(Kind::Component, source, 41, 0..source.len()).unwrap();
        let at = source.rfind("initial: 2").unwrap();
        assert_eq!(
            declaration.properties["initial"],
            41 + at..41 + at + "initial".len()
        );
        assert_eq!(declaration.children, "🦀 initial: text\nCafé\n");
    }

    #[test]
    fn repeated_yaml_keys_are_errors_at_any_depth() {
        for source in [
            "name: Counter\nname: Other\n",
            "name: Counter\nprops: {initial: 1, initial: 2}\n",
            "name: Counter\nprops: {nested: {label: one, label: two}}\n",
        ] {
            let issue = parse(Kind::Component, source, 0, 0..source.len()).unwrap_err();
            assert!(issue.message.contains("duplicate"), "{issue:?}");
        }
    }

    #[test]
    fn demos_take_only_a_registered_name() {
        let source = "name: CounterDemo\n";
        assert_eq!(
            parse(Kind::Demo, source, 0, 0..source.len()).unwrap().name,
            "CounterDemo"
        );
        for source in ["name: Demo\nprops: {}\n", "name: Demo\nchildren: text\n"] {
            assert!(parse(Kind::Demo, source, 0, 0..source.len()).is_err());
        }
    }

    #[test]
    fn malformed_data_is_located_and_never_becomes_rust() {
        for source in [
            "name: Counter\nunknown: yes\n",
            "name: 3\n",
            "name: Counter\nprops: []\n",
            "name: Counter\nprops: {initial: .nan}\n",
            "name: Counter\nchildren: [text]\n",
        ] {
            assert!(parse(Kind::Component, source, 12, 0..source.len() + 12).is_err());
        }
        let issue = parse(Kind::Component, "name: Counter\nprops: [\n", 20, 0..50).unwrap_err();
        assert!(issue.span.start >= 20);
        assert_eq!(issue.rule, "embed-payload");
    }
}
