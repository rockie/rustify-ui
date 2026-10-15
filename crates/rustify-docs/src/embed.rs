//! Located, data-only declarations in Markdown component and demo fences.

use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};
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

/// Reads one fence's payload. `origin` maps a byte offset in `payload` to
/// its offset in the file, which is not a constant shift when the Markdown
/// parser dropped a CR before LF or a container's prefix between lines.
pub fn parse(
    kind: Kind,
    payload: &str,
    origin: impl Fn(usize) -> usize,
    span: Range<usize>,
) -> Result<Declaration, Issue> {
    let located = |range: Range<usize>| origin(range.start)..origin(range.end);
    // YAML's own Mapping deserializer rejects duplicate keys recursively.
    // Deserializing directly into JSON would silently keep the last value.
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(payload).map_err(|error| {
        let at = error
            .location()
            .map_or(0, |location| location_offset(payload, &location));
        let repair = if repeats_key(payload) {
            DUPLICATE_KEY
        } else {
            PAYLOAD
        };
        Issue::error(located(at..at), "embed-payload", error.to_string()).suggest(repair)
    })?;
    let value = json_value(yaml)
        .map_err(|message| Issue::error(span.clone(), "embed-payload", message).suggest(PAYLOAD))?;
    let fields = value.as_object().ok_or_else(|| {
        Issue::error(
            span.clone(),
            "embed-payload",
            "write a YAML mapping with a registered name",
        )
        .suggest(PAYLOAD)
    })?;
    let locate = |key: &str| {
        key_span(payload, &[key])
            .map(located)
            .unwrap_or_else(|| span.clone())
    };
    let error = |key: &str, message: String| {
        Issue::error(locate(key), "embed-payload", message).suggest(PAYLOAD)
    };
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
                key_span(payload, &["props", key])
                    .map(located)
                    .unwrap_or_else(|| span.clone()),
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

/// Where a builtin's object fields are documented for authors.
pub const BUILTIN_REFERENCE: &str =
    "https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#static-builtins";

/// The repair for a payload whose YAML repeats a mapping key.
pub const DUPLICATE_KEY: &str =
    "Keep each YAML mapping key once; remove or rename the duplicate key.";

/// The repair for any other payload that is not a declaration.
pub const PAYLOAD: &str = "Write valid YAML for `rustify-component` (`name`, optional `props` mapping and `children` string) or `rustify-demo` (`name` only); correct the reported payload error.";

/// One declared prop as a `site!` manifest lists it, in normalized tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
}

/// Where a component's props are declared, which decides the next step of
/// a repair that the declared type alone cannot describe.
#[derive(Clone, Copy, Debug)]
pub enum Owner<'a> {
    Builtin,
    Registered { registry: &'a str },
}

pub fn unregistered_component(names: &[&str]) -> String {
    format!(
        "Use a component from {}; for an author component, enable `[rust]` and register its name in `site!`.",
        names_list(names)
    )
}

pub fn unregistered_demo(names: &[&str]) -> String {
    if names.is_empty() {
        return "Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block."
            .into();
    }
    format!(
        "Use a demo from {}; register demos in `site!`.",
        names_list(names)
    )
}

pub fn unknown_property(component: &str, fields: &[Field], owner: Owner, field: &str) -> String {
    if fields.is_empty() {
        return format!("Remove `{field}`; `{component}` declares no props.");
    }
    format!(
        "Use one of {} for `{component}`; rename or remove `{field}`.",
        fields_list(fields, owner)
    )
}

pub fn reserved_component(name: &str) -> String {
    format!(
        "Rename `{name}` in `site!` and its Markdown references; builtin component names are reserved."
    )
}

/// The repair for props the declared types refused. `path` is the serde
/// path of the refused value; only its first segment names a declared field.
pub fn invalid_props(fields: &[Field], owner: Owner, path: Option<&str>) -> String {
    let path = path.filter(|path| !path.is_empty() && *path != ".");
    let top = path.and_then(|path| path.split(['.', '[']).next());
    let mut repair = match top.and_then(|top| fields.iter().find(|field| field.name == top)) {
        Some(field) => field_repair(field, owner),
        None => format!(
            "Add or correct `props` using the declared fields {}; {}.",
            fields_list(fields, owner),
            reference_step(owner)
        ),
    };
    if let Some(path) = path {
        repair.push_str(&format!(" Reported props path: `{path}`."));
    }
    repair
}

fn field_repair(field: &Field, owner: Owner) -> String {
    let ty = display(&field.ty);
    let boundary = boundary(field, owner);
    let name = &field.name;
    match (shape(&ty), owner) {
        (Some(shape), _) => format!("Write `{name}: {shape}` for declared type `{ty}` ({boundary})."),
        (None, Owner::Builtin) => format!(
            "Write `{name}` as `{ty}` ({boundary}); see {BUILTIN_REFERENCE} for its object fields."
        ),
        (None, Owner::Registered { registry }) => format!(
            "Check `{name}` of declared type `{ty}` ({boundary}) in `{registry}`; follow the reported serde requirement."
        ),
    }
}

fn reference_step(owner: Owner) -> String {
    match owner {
        Owner::Builtin => format!("see {BUILTIN_REFERENCE} for its object fields"),
        Owner::Registered { registry } => format!("check the registration in `{registry}`"),
    }
}

/// Whether the prop may be left out, from what the declaration states. A
/// type that is neither a direct `Option` nor a recognized form says nothing
/// about omission, because an alias or a custom type decides that itself.
fn boundary(field: &Field, owner: Owner) -> String {
    let ty = display(&field.ty);
    if let Some(default) = &field.default {
        format!("default expression `{}`", display(default))
    } else if generic(
        &ty,
        &["Option", "std::option::Option", "core::option::Option"],
    )
    .is_some()
    {
        "optional; omission is null".into()
    } else if shape(&ty).is_some() || matches!(owner, Owner::Builtin) {
        "required".into()
    } else {
        "no declared default; confirm omission semantics in the registration".into()
    }
}

/// A YAML value of the declared form, for the types whose form is fixed.
/// It shows the shape only; a value range is the refused message's to state.
fn shape(ty: &str) -> Option<String> {
    if let Some(inner) = generic(
        ty,
        &["Option", "std::option::Option", "core::option::Option"],
    ) {
        return scalar(inner).map(|_| "null".to_owned());
    }
    if let Some(inner) = generic(ty, &["Vec", "std::vec::Vec", "alloc::vec::Vec"]) {
        return scalar(inner).map(|shape| format!("[{shape}]"));
    }
    scalar(ty).map(str::to_owned)
}

fn scalar(ty: &str) -> Option<&'static str> {
    Some(match ty {
        "String" | "std::string::String" | "alloc::string::String" => "\"text\"",
        "bool" => "false",
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => "0",
        "f32" | "f64" => "0.0",
        "char" => "\"a\"",
        _ => return None,
    })
}

fn generic<'a>(ty: &'a str, names: &[&str]) -> Option<&'a str> {
    names
        .iter()
        .find_map(|name| ty.strip_prefix(name)?.strip_prefix('<')?.strip_suffix('>'))
}

fn names_list(names: &[&str]) -> String {
    let mut names = names.to_vec();
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        return "none".into();
    }
    names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn fields_list(fields: &[Field], owner: Owner) -> String {
    let mut sorted: Vec<&Field> = fields.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted.dedup_by(|a, b| a.name == b.name);
    if sorted.is_empty() {
        return "none".into();
    }
    sorted
        .iter()
        .map(|field| {
            format!(
                "`{}` (`{}`; {})",
                field.name,
                display(&field.ty),
                boundary(field, owner)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A manifest's normalized tokens as an author writes them: `Vec<StepSpec>`
/// and `"Counter".to_string()` rather than `Vec < StepSpec >` and
/// `"Counter" . to_string ()`. String literals, raw ones included, and
/// character literals are kept whole.
pub fn display(tokens: &str) -> String {
    const SPACED: [&str; 10] = ["->", "=>", "=", "==", "!=", "<=", ">=", "+", "&&", "||"];
    let word = |ch: char| ch.is_alphanumeric() || matches!(ch, '_' | '"' | '\'');
    let mut out = String::new();
    let mut previous: Option<String> = None;
    for piece in pieces(tokens) {
        if let Some(previous) = &previous {
            let spaced = previous.ends_with([',', ';'])
                || SPACED.contains(&previous.as_str())
                || SPACED.contains(&piece.as_str())
                || (previous.ends_with(word) && piece.starts_with(word));
            if spaced {
                out.push(' ');
            }
        }
        out.push_str(&piece);
        previous = Some(piece);
    }
    out
}

fn pieces(tokens: &str) -> Vec<String> {
    let chars: Vec<char> = tokens.chars().collect();
    let mut pieces = Vec::new();
    let mut current = String::new();
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == ' ' {
            if !current.is_empty() {
                pieces.push(std::mem::take(&mut current));
            }
            index += 1;
            continue;
        }
        if let Some(end) = raw_literal_end(&chars, index) {
            current.extend(&chars[index..end]);
            index = end;
            continue;
        }
        let closing = match ch {
            '"' => Some('"'),
            // A character literal, not a lifetime such as `'static`.
            '\'' if chars.get(index + 1) == Some(&'\\') || chars.get(index + 2) == Some(&'\'') => {
                Some('\'')
            }
            _ => None,
        };
        current.push(ch);
        index += 1;
        if let Some(closing) = closing {
            while index < chars.len() {
                let ch = chars[index];
                current.push(ch);
                index += 1;
                if ch == '\\' {
                    if let Some(&escaped) = chars.get(index) {
                        current.push(escaped);
                        index += 1;
                    }
                } else if ch == closing {
                    break;
                }
            }
        }
    }
    if !current.is_empty() {
        pieces.push(current);
    }
    pieces
}

/// The end of a raw string literal (`r"…"`, `br#"…"#`, `cr##"…"##`) that
/// starts at `start`. Its content has no escapes and may hold quotes and
/// spaces; only a quote followed by as many hashes as opened it closes it.
fn raw_literal_end(chars: &[char], start: usize) -> Option<usize> {
    // `r` at the end of an identifier such as `bar` opens nothing.
    if start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        return None;
    }
    let mut index = start;
    if matches!(chars.get(index), Some('b' | 'c')) {
        index += 1;
    }
    if chars.get(index) != Some(&'r') {
        return None;
    }
    index += 1;
    let hashes = chars[index..].iter().take_while(|&&ch| ch == '#').count();
    index += hashes;
    // A raw identifier such as `r#type` has no quote here.
    if chars.get(index) != Some(&'"') {
        return None;
    }
    index += 1;
    while index < chars.len() {
        let closed = chars[index] == '"'
            && chars[index + 1..]
                .iter()
                .take(hashes)
                .filter(|&&ch| ch == '#')
                .count()
                == hashes;
        index += 1;
        if closed {
            return Some(index + hashes);
        }
    }
    Some(chars.len())
}

/// Whether some YAML mapping in the payload repeats a key, which is why the
/// YAML value refused it. Walking the payload again needs no error text.
fn repeats_key(payload: &str) -> bool {
    Repeats
        .deserialize(serde_yaml_ng::Deserializer::from_str(payload))
        .unwrap_or(false)
}

#[derive(Clone, Copy)]
struct Repeats;

impl<'de> DeserializeSeed<'de> for Repeats {
    type Value = bool;

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Repeats {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a YAML value")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut mapping: M) -> Result<bool, M::Error> {
        let mut keys = Vec::new();
        let mut repeated = false;
        while let Some(key) = mapping.next_key::<serde_yaml_ng::Value>()? {
            repeated |= keys.contains(&key);
            keys.push(key);
            repeated |= mapping.next_value_seed(self)?;
        }
        Ok(repeated)
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut items: S) -> Result<bool, S::Error> {
        let mut repeated = false;
        while let Some(item) = items.next_element_seed(self)? {
            repeated |= item;
        }
        Ok(repeated)
    }

    fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_i64<E: de::Error>(self, _: i64) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_u64<E: de::Error>(self, _: u64) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_f64<E: de::Error>(self, _: f64) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_str<E: de::Error>(self, _: &str) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_unit<E: de::Error>(self) -> Result<bool, E> {
        Ok(false)
    }

    fn visit_none<E: de::Error>(self) -> Result<bool, E> {
        Ok(false)
    }
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

/// The written key token at `path`, as a byte range of `payload`.
fn key_span(payload: &str, path: &[&str]) -> Option<Range<usize>> {
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
    Some(at..at + length)
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
        let declaration = parse(Kind::Component, source, |at| 30 + at, 10..130).unwrap();
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
            let declaration =
                parse(Kind::Component, source, |at| 17 + at, 0..source.len()).unwrap();
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
            let issue = parse(Kind::Component, source, |at| 23 + at, 0..source.len()).unwrap_err();
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
            let declaration =
                parse(Kind::Component, source, |at| 31 + at, 0..source.len()).unwrap();
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
        let declaration = parse(Kind::Component, source, |at| 41 + at, 0..source.len()).unwrap();
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
            let issue = parse(Kind::Component, source, |at| at, 0..source.len()).unwrap_err();
            assert!(issue.message.contains("duplicate"), "{issue:?}");
        }
    }

    #[test]
    fn demos_take_only_a_registered_name() {
        let source = "name: CounterDemo\n";
        assert_eq!(
            parse(Kind::Demo, source, |at| at, 0..source.len())
                .unwrap()
                .name,
            "CounterDemo"
        );
        for source in ["name: Demo\nprops: {}\n", "name: Demo\nchildren: text\n"] {
            assert!(parse(Kind::Demo, source, |at| at, 0..source.len()).is_err());
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
            assert!(parse(Kind::Component, source, |at| 12 + at, 0..source.len() + 12).is_err());
        }
        let issue = parse(
            Kind::Component,
            "name: Counter\nprops: [\n",
            |at| 20 + at,
            0..50,
        )
        .unwrap_err();
        assert!(issue.span.start >= 20);
        assert_eq!(issue.rule, "embed-payload");
    }

    fn field(name: &str, ty: &str, default: Option<&str>) -> Field {
        Field {
            name: name.into(),
            ty: ty.into(),
            default: default.map(Into::into),
        }
    }

    #[test]
    fn normalized_tokens_display_as_written() {
        for (tokens, shown) in [
            ("i32", "i32"),
            ("Vec < Option < String > >", "Vec<Option<String>>"),
            ("- 2", "-2"),
            ("\"Counter\" . to_string ()", "\"Counter\".to_string()"),
            (
                "std :: collections :: BTreeMap < String , u64 >",
                "std::collections::BTreeMap<String, u64>",
            ),
            ("& 'static str", "&'static str"),
            ("vec ! [1 , 2]", "vec![1, 2]"),
            ("\"a < b , c\" . into ()", "\"a < b , c\".into()"),
            ("' ' . to_string ()", "' '.to_string()"),
            ("impl Fn () -> bool + Send", "impl Fn() -> bool + Send"),
            ("r\"a  b\" . into ()", "r\"a  b\".into()"),
            (
                "r#\"a\"   b   \"c\"# . to_string ()",
                "r#\"a\"   b   \"c\"#.to_string()",
            ),
            ("r##\"x \"# , y\"## . into ()", "r##\"x \"# , y\"##.into()"),
            ("r\"C:\\  dir\\\" . into ()", "r\"C:\\  dir\\\".into()"),
            ("br#\"a  \"b\"# . to_vec ()", "br#\"a  \"b\"#.to_vec()"),
            ("cr\"a  b\" . into ()", "cr\"a  b\".into()"),
            ("Some (r#\"a  b\"#)", "Some(r#\"a  b\"#)"),
            ("r#type :: new ()", "r#type::new()"),
        ] {
            assert_eq!(display(tokens), shown, "{tokens}");
        }
    }

    #[test]
    fn declared_defaults_keep_their_literals_in_every_repair() {
        let source = r####"fn site() { rustify_docs::site! { components {
            Labels {
                plain: String = r"a   b".to_string(),
                quoted: String = r#"a"   b   "c"#.to_string(),
                hashed: String = r##"x "#   y"##.to_string(),
                path: String = r"C:\   dir\".to_string(),
                bytes: Vec<u8> = br#"a   "b"#.to_vec(),
                escaped: String = "a \"  b".to_string(),
                letter: char = ' ',
            } => |props, children| props.plain,
        } } }"####;
        let manifest = rustify_docs_syntax::parse_registry(source)
            .unwrap()
            .manifest();
        let fields: Vec<Field> = manifest.components[0]
            .fields
            .iter()
            .map(|field| Field {
                name: field.name.clone(),
                ty: field.ty.clone(),
                default: field.default.clone(),
            })
            .collect();
        let registered = Owner::Registered {
            registry: "src/lib.rs",
        };
        let listed = unknown_property("Labels", &fields, registered, "unexpected");
        let missing = invalid_props(&fields, registered, None);
        for (field, ty, literal) in [
            ("plain", "String", r#"r"a   b".to_string()"#),
            ("quoted", "String", r##"r#"a"   b   "c"#.to_string()"##),
            ("hashed", "String", r###"r##"x "#   y"##.to_string()"###),
            ("path", "String", r#"r"C:\   dir\".to_string()"#),
            ("bytes", "Vec<u8>", r##"br#"a   "b"#.to_vec()"##),
            ("escaped", "String", r#""a \"  b".to_string()"#),
            ("letter", "char", "' '"),
        ] {
            let boundary = format!("`{field}` (`{ty}`; default expression `{literal}`)");
            assert!(listed.contains(&boundary), "{field} in {listed}");
            assert!(missing.contains(&boundary), "{field} in {missing}");
            let refused = invalid_props(&fields, registered, Some(field));
            assert!(
                refused.contains(&format!("(default expression `{literal}`)")),
                "{field} in {refused}"
            );
        }
    }

    #[test]
    fn a_refused_prop_names_its_declared_form_and_boundary() {
        let registered = Owner::Registered {
            registry: "src/lib.rs",
        };
        let fields = [
            field("initial", "i32", Some("2")),
            field("label", "String", Some("\"Counter\" . to_string ()")),
            field("enabled", "bool", None),
            field("ratio", "f64", None),
            field("mark", "char", None),
            field("limit", "Option < u32 >", None),
            field("sizes", "Vec < u16 >", None),
            field("theme", "Palette", None),
            field("alias", "Labels", Some("Labels :: default ()")),
            field("maybe", "Option < Palette >", None),
        ];
        for (path, repair) in [
            ("initial", "Write `initial: 0` for declared type `i32` (default expression `2`). Reported props path: `initial`."),
            ("label", "Write `label: \"text\"` for declared type `String` (default expression `\"Counter\".to_string()`). Reported props path: `label`."),
            ("enabled", "Write `enabled: false` for declared type `bool` (required). Reported props path: `enabled`."),
            ("ratio", "Write `ratio: 0.0` for declared type `f64` (required). Reported props path: `ratio`."),
            ("mark", "Write `mark: \"a\"` for declared type `char` (required). Reported props path: `mark`."),
            ("limit", "Write `limit: null` for declared type `Option<u32>` (optional; omission is null). Reported props path: `limit`."),
            ("sizes[1]", "Write `sizes: [0]` for declared type `Vec<u16>` (required). Reported props path: `sizes[1]`."),
            ("theme.accent", "Check `theme` of declared type `Palette` (no declared default; confirm omission semantics in the registration) in `src/lib.rs`; follow the reported serde requirement. Reported props path: `theme.accent`."),
            ("alias", "Check `alias` of declared type `Labels` (default expression `Labels::default()`) in `src/lib.rs`; follow the reported serde requirement. Reported props path: `alias`."),
            ("maybe", "Check `maybe` of declared type `Option<Palette>` (optional; omission is null) in `src/lib.rs`; follow the reported serde requirement. Reported props path: `maybe`."),
        ] {
            assert_eq!(invalid_props(&fields, registered, Some(path)), repair);
        }
        // Without a declared field to point at, the whole declared list,
        // sorted, and the registration; `.` is serde's empty path.
        let missing = "Add or correct `props` using the declared fields `alias` (`Labels`; default expression `Labels::default()`), `enabled` (`bool`; required), `initial` (`i32`; default expression `2`), `label` (`String`; default expression `\"Counter\".to_string()`), `limit` (`Option<u32>`; optional; omission is null), `mark` (`char`; required), `maybe` (`Option<Palette>`; optional; omission is null), `ratio` (`f64`; required), `sizes` (`Vec<u16>`; required), `theme` (`Palette`; no declared default; confirm omission semantics in the registration); check the registration in `src/lib.rs`.";
        assert_eq!(invalid_props(&fields, registered, Some(".")), missing);
        assert_eq!(invalid_props(&fields, registered, None), missing);
        assert_eq!(
            invalid_props(&fields, registered, Some("unexpected")),
            format!("{missing} Reported props path: `unexpected`.")
        );
        assert_eq!(
            invalid_props(&[], registered, None),
            "Add or correct `props` using the declared fields none; check the registration in `src/lib.rs`."
        );
    }

    #[test]
    fn a_builtin_object_list_points_to_its_reference() {
        let fields = [
            field("cards", "Vec < CardSpec >", None),
            field("columns", "u8", Some("2")),
        ];
        assert_eq!(
            invalid_props(&fields, Owner::Builtin, Some("cards[0].link")),
            format!("Write `cards` as `Vec<CardSpec>` (required); see {BUILTIN_REFERENCE} for its object fields. Reported props path: `cards[0].link`.")
        );
        assert_eq!(
            invalid_props(&fields, Owner::Builtin, Some(".")),
            format!("Add or correct `props` using the declared fields `cards` (`Vec<CardSpec>`; required), `columns` (`u8`; default expression `2`); see {BUILTIN_REFERENCE} for its object fields.")
        );
    }

    #[test]
    fn names_fields_and_reserved_names_have_complete_sorted_repairs() {
        assert_eq!(
            unregistered_component(&["Steps", "Counter", "BrowserCounter", "Tabs", "CardGrid", "Counter"]),
            "Use a component from `BrowserCounter`, `CardGrid`, `Counter`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`."
        );
        assert_eq!(
            unregistered_demo(&["FailureDemo", "CounterDemo"]),
            "Use a demo from `CounterDemo`, `FailureDemo`; register demos in `site!`."
        );
        assert_eq!(
            unregistered_demo(&[]),
            "Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block."
        );
        let registered = Owner::Registered {
            registry: "src/lib.rs",
        };
        assert_eq!(
            unknown_property(
                "Counter",
                &[field("label", "String", Some("\"Counter\" . to_string ()")), field("initial", "i32", Some("2"))],
                registered,
                "unexpected"
            ),
            "Use one of `initial` (`i32`; default expression `2`), `label` (`String`; default expression `\"Counter\".to_string()`) for `Counter`; rename or remove `unexpected`."
        );
        assert_eq!(
            unknown_property("Badge", &[], registered, "tone"),
            "Remove `tone`; `Badge` declares no props."
        );
        assert_eq!(
            reserved_component("Tabs"),
            "Rename `Tabs` in `site!` and its Markdown references; builtin component names are reserved."
        );
    }

    #[test]
    fn payload_refusals_carry_the_duplicate_or_the_shape_repair() {
        for source in [
            "name: Counter\nname: Other\n",
            "name: Counter\nprops: {initial: 1, initial: 2}\n",
            "name: Counter\nprops: {nested: {label: one, label: two}}\n",
            "name: Counter\nprops:\n  items:\n    - {a: 1, a: 2}\n",
        ] {
            let issue = parse(Kind::Component, source, |at| at, 0..source.len()).unwrap_err();
            assert_eq!(issue.suggestion.as_deref(), Some(DUPLICATE_KEY), "{source}");
        }
        for source in [
            "name: Counter\nunknown: yes\n",
            "name: 3\n",
            "name: Counter\nprops: []\n",
            "name: Counter\nprops: {initial: .nan}\n",
            "name: Counter\nchildren: [text]\n",
            "name: Counter\nprops: [\n",
            "- just a list\n",
        ] {
            let issue = parse(Kind::Component, source, |at| at, 0..source.len()).unwrap_err();
            assert_eq!(issue.rule, "embed-payload");
            assert_eq!(issue.suggestion.as_deref(), Some(PAYLOAD), "{source}");
        }
    }

    #[test]
    fn a_payload_offset_maps_through_the_origin() {
        // The parser drops each CR before LF; the origin restores the file
        // offsets, so a key on the third line is where the file has it.
        let file = "name: Counter\r\nprops:\r\n  initial: 2\r\n";
        let payload = file.replace("\r\n", "\n");
        let origin = |at: usize| at + payload[..at].matches('\n').count();
        let declaration = parse(Kind::Component, &payload, origin, 0..file.len()).unwrap();
        let span = declaration.properties["initial"].clone();
        assert_eq!(&file[span], "initial");
    }
}
