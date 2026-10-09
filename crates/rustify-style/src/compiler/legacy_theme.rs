//! Built-in legacy theme paths, without JavaScript configuration execution.

use super::{theme::Theme, CompileError};
use crate::text::Text;
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Node {
    Text(&'static str),
    List(&'static [Node]),
    Map(&'static [(&'static str, Node)]),
}

#[path = "../../upstream/tailwind/legacy-theme-data.rs"]
mod data;

#[derive(Clone, Debug)]
struct Value {
    kind: Kind,
    default: bool,
    css_options: BTreeMap<Text, bool>,
}

#[derive(Clone, Debug)]
enum Kind {
    Text(Text),
    List(Vec<Value>),
    Map(BTreeMap<Text, Value>),
}

impl Value {
    fn text(text: impl Into<Text>, default: bool) -> Self {
        Self {
            kind: Kind::Text(text.into()),
            default,
            css_options: BTreeMap::new(),
        }
    }

    fn map(map: BTreeMap<Text, Self>) -> Self {
        Self {
            kind: Kind::Map(map),
            default: false,
            css_options: BTreeMap::new(),
        }
    }

    fn list(values: Vec<Self>) -> Self {
        Self {
            kind: Kind::List(values),
            default: false,
            css_options: BTreeMap::new(),
        }
    }

    fn from_node(node: &Node) -> Self {
        match node {
            Node::Text(text) => Self::text(*text, false),
            Node::List(values) => Self::list(values.iter().map(Self::from_node).collect()),
            Node::Map(values) => Self::map(
                values
                    .iter()
                    .map(|(key, value)| ((*key).into(), Self::from_node(value)))
                    .collect(),
            ),
        }
    }

    fn child_text(&self, key: &Text) -> Option<&Self> {
        match &self.kind {
            Kind::Map(map) => map.get(key),
            _ => key.to_utf8().ok().and_then(|key| self.child(&key)),
        }
    }

    fn child(&self, key: &str) -> Option<&Self> {
        match &self.kind {
            Kind::Map(map) => map.get(&Text::from(key)),
            Kind::List(list) => key
                .parse::<usize>()
                .ok()
                .filter(|index| index.to_string() == key)
                .and_then(|index| list.get(index)),
            Kind::Text(_) => None,
        }
    }
}

fn lookup<'a>(mut value: &'a Value, path: &[String]) -> Option<&'a Value> {
    let mut pending = None;
    for part in path {
        let key = pending
            .take()
            .map_or_else(|| part.clone(), |prefix| format!("{prefix}-{part}"));
        if let Some(child) = value.child(&key) {
            value = child;
        } else {
            pending = Some(key);
        }
    }
    pending.is_none().then_some(value)
}

fn lookup_map<'a>(map: &'a BTreeMap<Text, Value>, path: &[String]) -> Option<&'a Value> {
    let mut key = String::new();
    for (index, part) in path.iter().enumerate() {
        if index > 0 {
            key.push('-');
        }
        key.push_str(part);
        if let Some(value) = map.get(&Text::from(&key)) {
            return lookup(value, &path[index + 1..]);
        }
    }
    None
}

fn lookup_owned(value: &Value, path: &[String]) -> Result<Option<Value>, CompileError> {
    let path = path.iter().map(Text::from).collect::<Vec<_>>();
    lookup_owned_text(value, &path)
}

fn lookup_owned_text(value: &Value, path: &[Text]) -> Result<Option<Value>, CompileError> {
    if path.is_empty() {
        return Ok(Some(value.clone()));
    }
    let mut key = Text::default();
    for (index, part) in path.iter().enumerate() {
        if index > 0 {
            key.push_str("-");
        }
        key.push_text(part);
        if let Some(child) = value.child_text(&key) {
            return lookup_owned_text(child, &path[index + 1..]);
        }
        if let Kind::Text(text) = &value.kind {
            if let Ok(key) = key.to_utf8() {
                if let Ok(character) = key.parse::<usize>() {
                    if character.to_string() == key {
                        if let Some(&unit) = text.units().get(character) {
                            return lookup_owned_text(
                                &Value::text(Text::from_units(vec![unit]), false),
                                &path[index + 1..],
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

pub(super) fn unescape_key_text(value: &str) -> Result<Text, CompileError> {
    unescape_key_units(&Text::from(value))
}

pub(super) fn unescape_key_units(value: &Text) -> Result<Text, CompileError> {
    let input = value.units();
    let mut units = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let unit = input[index];
        index += 1;
        if unit != 92 || index == input.len() {
            units.push(unit);
            continue;
        }
        let next = input[index];
        index += 1;
        fn hex(unit: u16) -> Option<u32> {
            match unit {
                48..=57 => Some(u32::from(unit - 48)),
                65..=70 => Some(u32::from(unit - 65 + 10)),
                97..=102 => Some(u32::from(unit - 97 + 10)),
                _ => None,
            }
        }
        let Some(mut codepoint) = hex(next) else {
            units.push(next);
            continue;
        };
        let mut count = 1;
        while count < 6 {
            let Some(digit) = input.get(index).and_then(|unit| hex(*unit)) else {
                break;
            };
            codepoint = codepoint * 16 + digit;
            count += 1;
            index += 1;
        }
        let terminator = input
            .get(index)
            .is_some_and(|unit| matches!(unit, 9 | 10 | 12 | 13 | 32));
        if terminator {
            index += 1;
        }
        if count == 1 && !terminator {
            units.push(next);
        } else if codepoint > 0x10ffff {
            return Err(CompileError::RangeError {
                message: format!("Invalid code point {codepoint}").into(),
            });
        } else if codepoint <= 0xffff {
            units.push(codepoint as u16);
        } else {
            let point = codepoint - 0x10000;
            units.push(0xd800 | (point >> 10) as u16);
            units.push(0xdc00 | (point & 0x3ff) as u16);
        }
    }
    Ok(Text::from_units(units))
}

fn token_default(theme: &Theme, key: &Text) -> Result<bool, CompileError> {
    let key = if let Some(prefix) = &theme.prefix {
        let mut unprefixed = Text::from("--");
        unprefixed.push_text(&Text::from_units(
            key.units()
                .get(3 + prefix.encode_utf16().count()..)
                .unwrap_or(&[])
                .to_vec(),
        ));
        unprefixed
    } else {
        key.clone()
    };
    Ok(theme
        .tokens
        .get(&unescape_key_units(&key)?)
        .is_some_and(|token| token.default))
}

pub(super) fn key_path_text(path: &Text) -> Result<Vec<Text>, CompileError> {
    let mut output = Vec::new();
    // The caller bounds parser depth; this scan only decomposes an existing path.
    let parts = super::value::segment_text(path, 46, path.units().len())?;
    for part in parts {
        let units = part.units();
        if !units.contains(&91) {
            output.push(part);
            continue;
        }
        let mut offset = 0;
        while let Some(left) = units[offset..]
            .iter()
            .position(|unit| *unit == 91)
            .map(|index| offset + index)
        {
            let Some(right) = units[left..]
                .iter()
                .position(|unit| *unit == 93)
                .map(|index| left + index)
            else {
                break;
            };
            if left > offset {
                output.push(Text::from_units(units[offset..left].to_vec()));
            }
            output.push(Text::from_units(units[left + 1..right].to_vec()));
            offset = right + 1;
        }
        if offset < units.len() {
            output.push(Text::from_units(units[offset..].to_vec()));
        }
    }
    Ok(output)
}

fn kebab_case(value: &str) -> String {
    let mut output = String::new();
    let mut previous = None;
    for ch in value.chars() {
        if previous.is_some_and(|previous: char| previous.is_ascii_lowercase())
            && ch.is_ascii_uppercase()
        {
            output.push('-');
            output.push(ch.to_ascii_lowercase());
        } else {
            output.push(if ch == '.' { '_' } else { ch });
        }
        previous = Some(ch);
    }
    output
}

fn css_key(path: &[String]) -> Option<String> {
    let first = path.first()?;
    if first == "container" {
        return None;
    }
    let mapped = match first.as_str() {
        "animation" => "animate",
        "aspectRatio" => "aspect",
        "borderRadius" => "radius",
        "boxShadow" => "shadow",
        "colors" => "color",
        "containers" => "container",
        "fontFamily" => "font",
        "fontSize" => "text",
        "letterSpacing" => "tracking",
        "lineHeight" => "leading",
        "maxWidth" => "container",
        "screens" => "breakpoint",
        "transitionTimingFunction" => "ease",
        _ => first,
    };
    let mut parts = Vec::new();
    for (index, part) in path.iter().enumerate() {
        if part.is_empty()
            || !part
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || "-_%/.".contains(ch))
        {
            return None;
        }
        if index + 1 == path.len() && part == "DEFAULT" {
            continue;
        }
        let part = if index == 0 {
            mapped
        } else if index + 1 < path.len() && part == "1" {
            ""
        } else {
            part
        };
        parts.push(kebab_case(part));
    }
    Some(format!("--{}", parts.join("-")))
}

fn read_css(theme: &Theme, path: &[String]) -> Result<Option<Value>, CompileError> {
    let Some(namespace) = css_key(path) else {
        return Ok(None);
    };
    let prefix = format!("{namespace}-");
    let namespace_units: Vec<_> = namespace.encode_utf16().collect();
    let prefix_units: Vec<_> = prefix.encode_utf16().collect();
    let mut map: BTreeMap<Text, Value> = BTreeMap::new();
    let mut nested: BTreeMap<Text, BTreeMap<Text, Value>> = BTreeMap::new();
    for (key, token) in theme.tokens.range(Text::from(namespace.as_str())..) {
        if !key.starts_with(&namespace) {
            break;
        }
        let suffix = if key == &namespace {
            &[][..]
        } else if key.starts_with(&format!("{prefix}-")) {
            &key.units()[namespace_units.len()..]
        } else if key.starts_with(&prefix) {
            &key.units()[prefix_units.len()..]
        } else {
            continue;
        };
        let split = suffix.windows(2).position(|units| units == [45, 45]);
        if let Some(split) = split {
            let main = Text::from_units(suffix[..split].to_vec());
            let extra = Text::from_units(suffix[split + 2..].to_vec());
            // Camel-case only the ASCII dash before a lower-case letter.
            let mut camel = Vec::new();
            let mut index = 0;
            while index < extra.units().len() {
                let unit = extra.units()[index];
                if unit == 45
                    && extra
                        .units()
                        .get(index + 1)
                        .is_some_and(|unit| (97..=122).contains(unit))
                {
                    camel.push(extra.units()[index + 1] - 32);
                    index += 2;
                } else {
                    camel.push(unit);
                    index += 1;
                }
            }
            let mut options_key = Text::from(namespace.as_str());
            options_key.push_text(&Text::from_units(suffix.to_vec()));
            nested.entry(main).or_default().insert(
                Text::from_units(camel),
                Value::text(token.value.clone(), token_default(theme, &options_key)?),
            );
        } else {
            map.insert(
                if suffix.is_empty() {
                    Text::from("DEFAULT")
                } else {
                    Text::from_units(suffix.to_vec())
                },
                Value::text(token.value.clone(), token_default(theme, key)?),
            );
        }
    }
    for (main, extra) in nested {
        let main = if main.is_empty() {
            "DEFAULT".into()
        } else {
            main
        };
        if let Some(base) = map.get_mut(&main) {
            if matches!(&base.kind, Kind::Text(_)) {
                base.default = token_default(theme, &Text::from(namespace.as_str()))?;
                *base = Value::list(vec![base.clone(), Value::map(extra)]);
            }
        }
    }
    if path.last().is_some_and(|last| last == "DEFAULT") {
        return Ok(map.remove(&Text::from("DEFAULT")));
    }
    if map.len() == 1 && map.contains_key(&Text::from("DEFAULT")) {
        return Ok(map.remove(&Text::from("DEFAULT")));
    }
    Ok((!map.is_empty()).then(|| {
        let css_options = map
            .iter()
            .map(|(key, value)| (key.clone(), value.default))
            .collect();
        let mut value = Value::map(map);
        value.css_options = css_options;
        value
    }))
}

fn combine(css: Option<Value>, config: Option<Value>) -> Result<Option<Value>, CompileError> {
    let Some(css) = css else { return Ok(config) };
    let Some(mut config) = config else {
        return Ok(Some(css));
    };
    let css_options = &config.css_options;
    Ok(match (&css.kind, &mut config.kind) {
        (Kind::Text(_), _) => {
            if css.default {
                Some(config)
            } else {
                Some(css)
            }
        }
        (Kind::Map(css_map), Kind::Map(config_map)) => {
            for (key, value) in css_map {
                let path = super::value::segment_text(key, 45, 256)?;
                let scalar_path: Option<Vec<_>> =
                    path.iter().map(|part| part.to_utf8().ok()).collect();
                if css_options.get(key).copied().unwrap_or(false)
                    && scalar_path
                        .as_ref()
                        .is_some_and(|path| lookup_map(config_map, path).is_some())
                {
                    continue;
                }
                let key = match key.to_utf8() {
                    Ok(key) => unescape_key_text(&key)?,
                    Err(_) => key.clone(),
                };
                config_map.insert(key, value.clone());
            }
            Some(config)
        }
        (Kind::List(css_list), Kind::List(config_list))
            if css_list.len() == 2 && config_list.len() == 2 =>
        {
            let mut list = css_list.clone();
            if list[0].default {
                list[0] = config_list[0].clone();
            }
            if let Kind::Map(extra) = &mut list[1].kind {
                for (key, value) in extra {
                    if value.default {
                        if let Some(config) = config_list[1].child_text(key) {
                            *value = config.clone();
                        }
                    }
                }
            }
            Some(Value::list(list))
        }
        _ => Some(css),
    })
}

#[derive(Clone, Debug)]
pub(super) struct LegacyTheme {
    namespaces: BTreeMap<String, Value>,
    usage: CacheUsage,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct CacheUsage {
    pub nodes: usize,
    pub string_bytes: usize,
    pub owned_bytes: usize,
}

struct Builder<'a> {
    theme: &'a Theme,
    namespaces: BTreeMap<String, Value>,
}

// Rust alloc/collections/btree/node.rs: B=6, CAPACITY=11, MIN_LEN_AFTER_SPLIT=5.
// https://doc.rust-lang.org/src/alloc/collections/btree/node.rs.html
// The root may be smaller; ceil(entries/5) also covers that root.
// Charge full internal-node slots and pointers even for leaves, giving a conservative bound.
fn map_heap_bytes<T>(entries: usize) -> usize {
    entries
        .div_ceil(5)
        .saturating_mul(11 * std::mem::size_of::<(Text, T)>() + 16 * std::mem::size_of::<usize>())
}

impl CacheUsage {
    fn strings(&mut self, value: &str, capacity: usize) {
        self.string_bytes = self.string_bytes.saturating_add(value.len());
        self.owned_bytes = self.owned_bytes.saturating_add(capacity);
    }

    fn value(&mut self, value: &Value) {
        self.nodes = self.nodes.saturating_add(1);
        self.owned_bytes = self
            .owned_bytes
            .saturating_add(map_heap_bytes::<bool>(value.css_options.len()));
        for key in value.css_options.keys() {
            self.string_bytes = self.string_bytes.saturating_add(key.byte_len());
            self.owned_bytes = self.owned_bytes.saturating_add(key.owned_bytes());
        }
        match &value.kind {
            Kind::Text(text) => {
                self.string_bytes = self.string_bytes.saturating_add(text.byte_len());
                self.owned_bytes = self.owned_bytes.saturating_add(text.owned_bytes());
            }
            Kind::List(values) => {
                self.owned_bytes = self.owned_bytes.saturating_add(
                    values
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Value>()),
                );
                for value in values {
                    self.value(value);
                }
            }
            Kind::Map(map) => {
                self.owned_bytes = self
                    .owned_bytes
                    .saturating_add(map_heap_bytes::<Value>(map.len()));
                for (key, value) in map {
                    self.string_bytes = self.string_bytes.saturating_add(key.byte_len());
                    self.owned_bytes = self.owned_bytes.saturating_add(key.owned_bytes());
                    self.value(value);
                }
            }
        }
    }
}

pub(super) fn cache_usage(theme: &Theme) -> CacheUsage {
    match theme.legacy.get() {
        Some(Ok(cache)) => cache.usage,
        Some(Err(CompileError::Syntax { message, .. } | CompileError::RangeError { message })) => {
            CacheUsage {
                nodes: 0,
                string_bytes: message.len(),
                owned_bytes: std::mem::size_of::<CompileError>() + message.owned_bytes(),
            }
        }
        Some(Err(CompileError::PendingFeature { feature })) => CacheUsage {
            nodes: 0,
            string_bytes: feature.len(),
            owned_bytes: std::mem::size_of::<CompileError>() + feature.capacity(),
        },
        _ => CacheUsage::default(),
    }
}

impl LegacyTheme {
    fn new(theme: &Theme) -> Result<Self, CompileError> {
        let mut builder = Builder {
            theme,
            namespaces: BTreeMap::new(),
        };
        for (namespace, _, _) in data::DEFAULTS {
            builder.namespace_config(namespace)?;
        }
        let mut usage = CacheUsage {
            owned_bytes: std::mem::size_of::<Self>()
                + map_heap_bytes::<Value>(builder.namespaces.len()),
            ..CacheUsage::default()
        };
        for (key, value) in &builder.namespaces {
            usage.strings(key, key.capacity());
            usage.value(value);
        }
        if let Some(budget) = theme.legacy_budget {
            super::check_limit(usage.owned_bytes, budget, "cache_bytes")?;
        }
        Ok(Self {
            namespaces: builder.namespaces,
            usage,
        })
    }
}

impl Builder<'_> {
    fn namespace_config(&mut self, namespace: &str) -> Result<Option<Value>, CompileError> {
        if let Some(value) = self.namespaces.get(namespace) {
            return Ok(Some(value.clone()));
        }
        let theme = self.theme;
        let Some((_, node, from)) = data::DEFAULTS.iter().find(|(key, _, _)| *key == namespace)
        else {
            return Ok(None);
        };
        let mut config = Value::from_node(node);
        if let Some(from) = from {
            let inherited = self.resolve_value(&[(*from).into()])?;
            if let Some(inherited) = inherited {
                config.css_options = inherited.css_options;
                if let (Kind::Map(base), Kind::Map(inherited)) = (&mut config.kind, inherited.kind)
                {
                    base.extend(inherited);
                }
            }
        }
        let extend = match namespace {
            "fontSize" => Some("text"),
            "boxShadow" => Some("shadow"),
            "animation" => Some("animate"),
            "aspectRatio" => Some("aspect"),
            "borderRadius" => Some("radius"),
            "screens" => Some("breakpoint"),
            "letterSpacing" => Some("tracking"),
            "lineHeight" => Some("leading"),
            "maxWidth" => Some("container"),
            _ => None,
        };
        if let Some(extend) = extend {
            if let Some(value) = read_css(theme, &[extend.into()])? {
                config.css_options.extend(value.css_options);
                if let (Kind::Map(base), Kind::Map(extra)) = (&mut config.kind, value.kind) {
                    base.extend(extra);
                }
            }
        }
        if matches!(namespace, "transitionDuration" | "transitionTimingFunction") {
            let key = if namespace == "transitionDuration" {
                "--default-transition-duration"
            } else {
                "--default-transition-timing-function"
            };
            if let Kind::Map(map) = &mut config.kind {
                map.remove(&Text::from("DEFAULT"));
                if let Some(token) = theme.tokens.get(&Text::from(key)) {
                    map.insert("DEFAULT".into(), Value::text(token.value.clone(), false));
                }
            }
        }
        self.namespaces.insert(namespace.into(), config.clone());
        Ok(Some(config))
    }

    fn resolve_value(&mut self, path: &[String]) -> Result<Option<Value>, CompileError> {
        let css = read_css(self.theme, path)?;
        let config = if let Some(namespace) = path.first() {
            if let Some(config) = self.namespace_config(namespace)? {
                lookup_owned(&config, &path[1..])?
            } else {
                None
            }
        } else {
            None
        };
        combine(css, normalize_config(config))
    }
}

fn normalize_config(config: Option<Value>) -> Option<Value> {
    config.map(|mut value| {
        if let Kind::Text(text) = &mut value.kind {
            if let Ok(scalar) = text.to_utf8() {
                *text = scalar.replacen("<alpha-value>", "1", 1).into();
            }
        }
        value
    })
}

fn resolve_value(
    theme: &Theme,
    cache: &LegacyTheme,
    path: &[String],
) -> Result<Option<Value>, CompileError> {
    let css = read_css(theme, path)?;
    let config = if let Some(config) = path
        .first()
        .and_then(|namespace| cache.namespaces.get(namespace))
    {
        lookup_owned(config, &path[1..])?
    } else {
        None
    };
    combine(css, normalize_config(config))
}

pub(super) fn resolve_path_text(theme: &Theme, path: &Text) -> Result<Option<Text>, CompileError> {
    let cache = theme
        .legacy
        .get_or_init(|| LegacyTheme::new(theme))
        .as_ref()
        .map_err(Clone::clone)?;
    let path = key_path_text(path)?;
    let scalar_path: Option<Vec<_>> = path.iter().map(|part| part.to_utf8().ok()).collect();
    let value = if let Some(path) = scalar_path {
        resolve_value(theme, cache, &path)?
    } else {
        path.first()
            .and_then(|namespace| namespace.to_utf8().ok())
            .and_then(|namespace| cache.namespaces.get(&namespace))
            .map(|config| lookup_owned_text(config, &path[1..]))
            .transpose()?
            .flatten()
    };
    let Some(value) = value else {
        return Ok(None);
    };
    Ok(match value.kind {
        Kind::Text(value) => Some(value),
        Kind::List(values) if values.len() == 2 => match &values[0].kind {
            Kind::Text(value) => Some(value.clone()),
            _ => None,
        },
        Kind::List(values) => {
            let mut output = Text::default();
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push_str(", ");
                }
                match &value.kind {
                    Kind::Text(value) => output.push_text(value),
                    _ => output.push_str("[object Object]"),
                }
            }
            Some(output)
        }
        Kind::Map(_) => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::compiler::{CompileError, Compiler, CompilerInput, Limits};

    fn compiler(css: String, cache_bytes: usize) -> Result<Compiler, CompileError> {
        Compiler::new(
            CompilerInput {
                css: css.into(),
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
                cache_bytes,
            },
        )
    }

    #[test]
    fn authored_legacy_theme_cannot_publish_metadata_above_caller_budget() {
        let css = format!(
            "{} .p {{ width: theme(spacing.12); }} @tailwind utilities;",
            crate::compiler::DEFAULT_THEME_CSS
        );
        assert!(
            matches!(compiler(css, 1048576), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
    }

    #[test]
    fn failed_lazy_legacy_cache_preserves_commit_and_allows_plain_candidates() {
        let mut compiler = compiler(
            format!(
                "{} @tailwind utilities;",
                crate::compiler::DEFAULT_THEME_CSS
            ),
            1048576,
        )
        .unwrap();
        let first = compiler.prepare_add(&["underline".into()]).unwrap();
        compiler.commit(first).unwrap();
        let before = serde_json::to_value(compiler.report()).unwrap();
        assert!(compiler.theme.legacy.get().is_none());
        assert!(
            matches!(compiler.prepare_add(&["[width:theme(spacing.12)]".into()]), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
        assert_eq!(serde_json::to_value(compiler.report()).unwrap(), before);
        assert!(compiler.theme.legacy.get().is_none());
        let next = compiler.prepare_add(&["block".into()]).unwrap();
        assert!(next.report().css.contains("display: block"));
        compiler.commit(next).unwrap();
    }

    #[test]
    fn explicit_legacy_budget_charges_initial_and_candidate_metrics() {
        let css = format!(
            "{} .p {{ width: theme(spacing.12); }} @tailwind utilities;",
            crate::compiler::DEFAULT_THEME_CSS
        );
        let mut compiler = compiler(css, 4194304).unwrap();
        let usage = super::cache_usage(&compiler.theme);
        assert!(usage.owned_bytes > 1048576);
        assert!(compiler.report().metrics.cache_bytes >= usage.owned_bytes);
        let first = compiler
            .prepare_add(&["[height:theme(spacing.2)]".into()])
            .unwrap();
        assert!(first.report().metrics.cache_bytes >= usage.owned_bytes);
        compiler.commit(first).unwrap();
        let repeated = compiler
            .prepare_add(&["[height:theme(spacing.2)]".into()])
            .unwrap();
        assert_eq!(
            repeated.report().metrics.cache_bytes,
            compiler.report().metrics.cache_bytes
        );
        assert_eq!(
            super::cache_usage(&compiler.theme).owned_bytes,
            usage.owned_bytes
        );
    }

    #[test]
    fn legacy_cache_charges_owned_dynamic_namespace_data_once() {
        for (name, css) in [
            ("empty", "".to_owned()),
            ("custom", "@theme { --color-brand: #123456; --spacing-card: 13px; --text-card: 23px; --text-card--line-height: 29px; }".to_owned()),
            ("css-defaults", crate::compiler::DEFAULT_THEME_CSS.to_owned()),
        ] {
            let mut nodes = crate::compiler::parser::parse(&css, 32).unwrap();
            let theme = crate::compiler::theme::Theme::extract(&mut nodes, 32).unwrap();
            assert_eq!(super::cache_usage(&theme).owned_bytes, 0);
            super::resolve_path_text(&theme, &crate::text::Text::from("spacing.12")).unwrap();
            let first = super::cache_usage(&theme);
            super::resolve_path_text(&theme, &crate::text::Text::from("padding.4")).unwrap();
            let repeated = super::cache_usage(&theme);
            assert_eq!(first.owned_bytes, repeated.owned_bytes);
            assert!(first.owned_bytes >= first.string_bytes);
            eprintln!("legacy-cache:{name}: namespaces=119 nodes={} string_bytes={} owned_bytes={}", first.nodes, first.string_bytes, first.owned_bytes);
        }
    }

    #[test]
    fn legacy_theme_matches_all_default_namespaces_and_css_overrides() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/legacy-theme-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let result = Compiler::new(
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
                    cache_bytes: case["limits"]["cache_bytes"]
                        .as_u64()
                        .map(|bytes| usize::try_from(bytes).unwrap())
                        .unwrap_or(1048576),
                },
            );
            let actual = if let Some(expected) = case["expectedError"].as_str() {
                let result = result.and_then(|mut compiler| {
                    let candidates = case["candidates"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap().to_owned())
                        .collect::<Vec<_>>();
                    compiler.prepare_add(&candidates).map(|_| ())
                });
                match result {
                    Err(CompileError::Syntax { message, .. })
                        if message == expected && case["expectedErrorName"] != "RangeError" =>
                    {
                        continue
                    }
                    Err(CompileError::RangeError { message })
                        if message == expected && case["expectedErrorName"] == "RangeError" =>
                    {
                        continue
                    }
                    Err(error) => format!("{error:?}"),
                    Ok(_) => "unexpected success".into(),
                }
            } else {
                match result {
                    Ok(mut compiler) => {
                        let candidates = case["candidates"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|value| value.as_str().unwrap().to_owned())
                            .collect::<Vec<_>>();
                        match compiler.prepare_add(&candidates) {
                            Ok(prepared)
                                if prepared.report().css
                                    == case["expectedCss"].as_str().unwrap()
                                    && prepared.report().unsupported.is_empty() =>
                            {
                                continue;
                            }
                            Ok(prepared) => format!(
                                "CSS={:?}, pending={:?}",
                                prepared.report().css,
                                prepared.report().unsupported
                            ),
                            Err(error) => format!("{error:?}"),
                        }
                    }
                    Err(error) => format!("{error:?}"),
                }
            };
            failures.push(format!(
                "{}: {actual}; expected={}",
                case["id"],
                case["expectedError"]
                    .as_str()
                    .or(case["expectedCss"].as_str())
                    .unwrap()
            ));
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first: {:#?}",
            failures.len(),
            &failures[..failures.len().min(8)]
        );
    }
}
