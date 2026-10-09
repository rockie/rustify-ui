//! Fixed-version public design-system operations share the Rust utility emitters.

use super::{CompileOptions, SourceLocation, Stylesheet};
use crate::compiler::{
    self,
    ast::AstNode,
    candidate::{Candidate, CandidateKind, CandidateValue},
    custom_utilities::UtilityKind,
    theme::Theme,
    utilities::{CompileFailure, CompiledCandidate, FamilyFailure},
    value, variants, CompileError, Compiler, Limits,
};
use crate::text::Text;
use indexmap::IndexMap;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;
#[cfg(test)]
use serde_json::{json, Value};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/// Public object fields retain insertion order and original text.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PublicObject {
    identity: Arc<()>,
    fields: IndexMap<Text, PublicValue>,
}

impl PublicObject {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn identity(&self) -> &Arc<()> {
        &self.identity
    }
}
impl std::ops::Deref for PublicObject {
    type Target = IndexMap<Text, PublicValue>;
    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}
impl std::ops::DerefMut for PublicObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}
impl FromIterator<(Text, PublicValue)> for PublicObject {
    fn from_iter<T: IntoIterator<Item = (Text, PublicValue)>>(values: T) -> Self {
        Self {
            identity: Arc::new(()),
            fields: values.into_iter().collect(),
        }
    }
}
impl<const N: usize> From<[(Text, PublicValue); N]> for PublicObject {
    fn from(values: [(Text, PublicValue); N]) -> Self {
        values.into_iter().collect()
    }
}
impl<'de> Deserialize<'de> for PublicObject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            identity: Arc::new(()),
            fields: IndexMap::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(Text),
    Array(Arc<Vec<PublicValue>>),
    Object(Arc<PublicObject>),
}

impl PublicValue {
    pub fn as_text(&self) -> Option<&Text> {
        if let Self::String(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_array(&self) -> Option<&Vec<Self>> {
        if let Self::Array(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Self>> {
        if let Self::Array(value) = self {
            Some(Arc::make_mut(value))
        } else {
            None
        }
    }
    pub fn as_object(&self) -> Option<&PublicObject> {
        if let Self::Object(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut PublicObject> {
        if let Self::Object(value) = self {
            Some(Arc::make_mut(value))
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        if let Self::Number(value) = self {
            value.as_u64()
        } else {
            None
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn get(&self, key: &str) -> Option<&Self> {
        self.as_object()?.get(&Text::from(key))
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Self> {
        self.as_object_mut()?.get_mut(&Text::from(key))
    }
    pub fn get_text(&self, key: &Text) -> Option<&Self> {
        self.as_object()?.get(key)
    }
    pub fn owned_bytes(&self) -> usize {
        json_bytes(self)
    }
    fn text_or_empty(&self) -> &Text {
        static EMPTY: std::sync::OnceLock<Text> = std::sync::OnceLock::new();
        self.as_text()
            .unwrap_or_else(|| EMPTY.get_or_init(Text::default))
    }
    fn write_json(&self, token: &mut String) {
        match self {
            Self::Null => token.push_str("null"),
            Self::Bool(value) => token.push_str(if *value { "true" } else { "false" }),
            Self::Number(value) => token.push_str(&value.to_string()),
            Self::String(value) => value.write_json_string(token),
            Self::Array(values) => {
                token.push('[');
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        token.push(',');
                    }
                    value.write_json(token);
                }
                token.push(']');
            }
            Self::Object(values) => {
                token.push('{');
                for (index, (key, value)) in values.iter().enumerate() {
                    if index > 0 {
                        token.push(',');
                    }
                    key.write_json_string(token);
                    token.push(':');
                    value.write_json(token);
                }
                token.push('}');
            }
        }
    }
    /// Adapts the scalar convenience facade after the canonical operation.
    pub(super) fn scalar(&self) -> Result<serde_json::Value, CompileError> {
        let mut token = String::new();
        self.write_json(&mut token);
        serde_json::from_str(&token)
            .map_err(|_| syntax("Non-scalar public value requires the Text facade"))
    }
    pub(super) fn from_scalar(value: &serde_json::Value) -> Result<Self, CompileError> {
        let token = serde_json::to_string(value).map_err(|_| syntax("Invalid public value"))?;
        serde_json::from_str(&token).map_err(|_| syntax("Invalid public value"))
    }
}
impl Serialize for PublicValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut token = String::new();
        self.write_json(&mut token);
        RawValue::from_string(token)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for PublicValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        let json = raw.get();
        let decoded = match json.as_bytes().first().copied() {
            Some(b'"') => serde_json::from_str::<Text>(json).map(Self::String),
            Some(b'[') => {
                serde_json::from_str::<Vec<Self>>(json).map(|values| Self::Array(Arc::new(values)))
            }
            Some(b'{') => serde_json::from_str::<PublicObject>(json)
                .map(|values| Self::Object(Arc::new(values))),
            Some(b'n') => serde_json::from_str::<()>(json).map(|_| Self::Null),
            Some(b't' | b'f') => serde_json::from_str::<bool>(json).map(Self::Bool),
            _ => serde_json::from_str::<serde_json::Number>(json).map(Self::Number),
        };
        decoded.map_err(serde::de::Error::custom)
    }
}
impl From<Text> for PublicValue {
    fn from(value: Text) -> Self {
        Self::String(value)
    }
}
impl From<String> for PublicValue {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}
impl From<&str> for PublicValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<bool> for PublicValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<usize> for PublicValue {
    fn from(value: usize) -> Self {
        Self::Number(value.into())
    }
}
impl From<u32> for PublicValue {
    fn from(value: u32) -> Self {
        Self::Number(value.into())
    }
}
impl From<u64> for PublicValue {
    fn from(value: u64) -> Self {
        Self::Number(value.into())
    }
}
impl<T: Clone + Into<PublicValue>> From<&T> for PublicValue {
    fn from(value: &T) -> Self {
        value.clone().into()
    }
}
impl<T: Into<PublicValue>> From<Vec<T>> for PublicValue {
    fn from(value: Vec<T>) -> Self {
        Self::Array(Arc::new(value.into_iter().map(Into::into).collect()))
    }
}
impl<T: Into<PublicValue>> From<Option<T>> for PublicValue {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Null, Into::into)
    }
}
impl PartialEq<str> for PublicValue {
    fn eq(&self, value: &str) -> bool {
        self.as_text().is_some_and(|text| text == value)
    }
}
impl PartialEq<&str> for PublicValue {
    fn eq(&self, value: &&str) -> bool {
        self == *value
    }
}
impl PartialEq<Text> for PublicValue {
    fn eq(&self, value: &Text) -> bool {
        self.as_text() == Some(value)
    }
}
impl PartialEq<&Text> for PublicValue {
    fn eq(&self, value: &&Text) -> bool {
        self == *value
    }
}
impl PartialEq<bool> for PublicValue {
    fn eq(&self, value: &bool) -> bool {
        self.as_bool() == Some(*value)
    }
}
impl std::ops::Index<&str> for PublicValue {
    type Output = Self;
    fn index(&self, key: &str) -> &Self {
        self.get(key).unwrap_or(&Self::Null)
    }
}
impl std::ops::IndexMut<&str> for PublicValue {
    fn index_mut(&mut self, key: &str) -> &mut Self {
        if !matches!(self, Self::Object(_)) {
            *self = Self::Object(Arc::default());
        }
        match self {
            Self::Object(map) => Arc::make_mut(map).entry(key.into()).or_insert(Self::Null),
            _ => unreachable!(),
        }
    }
}
impl std::ops::Index<usize> for PublicValue {
    type Output = Self;
    fn index(&self, index: usize) -> &Self {
        self.as_array()
            .and_then(|array| array.get(index))
            .unwrap_or(&Self::Null)
    }
}
macro_rules! public_json {
    ({$($key:literal : $value:expr),* $(,)?}) => { PublicValue::Object(Arc::new([$((Text::from($key),PublicValue::from(&$value))),*].into_iter().collect())) };
    ([$($value:expr),* $(,)?]) => { PublicValue::Array(Arc::new(vec![$(PublicValue::from(&$value)),*])) };
    ($value:expr) => { PublicValue::from(&$value) };
}
pub(super) fn literal(text: &Text) -> &'static str {
    for name in [
        "static",
        "functional",
        "arbitrary",
        "named",
        "compound",
        "not",
        "group",
        "peer",
        "in",
        "has",
        "aria",
        "min",
        "max",
        "@",
        "@min",
        "@max",
        "--font",
        "--inset",
        "--text",
        "rule",
        "at-rule",
        "declaration",
        "comment",
        "context",
        "at-root",
    ] {
        if text == name {
            return name;
        }
    }
    ""
}
pub(in crate::compiler) use public_json;
fn concat(parts: &[&Text]) -> Text {
    value::concat_text(parts)
}
pub(in crate::compiler) fn typed<T: Serialize>(value: &T) -> Result<PublicValue, CompileError> {
    let json = serde_json::to_string(value).map_err(|_| syntax("Invalid public object"))?;
    serde_json::from_str(&json).map_err(|_| syntax("Invalid public object"))
}
/// A lightweight mark for rolling back newly published public metadata.
#[derive(Clone, Copy)]
pub struct CacheCheckpoint {
    candidates: usize,
    variants: usize,
    retained_bytes: usize,
    candidate_capacity: usize,
    variant_capacity: usize,
}

pub struct DesignSystem {
    compiler: Compiler,
    parsed_candidates: BTreeMap<Text, Vec<PublicValue>>,
    parsed_variants: Vec<(Text, PublicValue)>,
    variant_indices: BTreeMap<Text, usize>,
    used_variables: BTreeSet<crate::text::Text>,
    retained_bytes: usize,
    candidate_order: Vec<Text>,
    reserved_cache: usize,
    pub important: bool,
    /// Candidates the caller marked invalid, in insertion order; class
    /// compilation skips them as upstream `compileCandidates` does.
    invalid_candidates: Vec<Text>,
}

/// A public theme token retains its original ECMAScript value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeEntry {
    pub value: Text,
    pub options: u32,
    pub src: Option<SourceLocation>,
}

impl Serialize for ThemeEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut entry = serializer.serialize_struct("ThemeEntry", 3)?;
        entry.serialize_field("value", &self.value)?;
        entry.serialize_field("options", &self.options)?;
        if let Some(src) = &self.src {
            entry.serialize_field("src", src)?;
        } else {
            entry.serialize_field("src", &undefined())?;
        }
        entry.end()
    }
}

pub fn load_design_system(
    css: &str,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<DesignSystem, CompileError> {
    load_design_system_text(&css.into(), options, assets, limits)
}

/// See [`super::compile_text_with_loader`] for `loader`.
pub fn load_design_system_with_loader(
    css: &crate::text::Text,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
    loader: bool,
) -> Result<DesignSystem, CompileError> {
    design_system(super::compile_text_impl(
        css, options, assets, limits, false, loader,
    )?)
}

pub fn load_design_system_text(
    css: &crate::text::Text,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<DesignSystem, CompileError> {
    design_system(super::compile_text_impl(
        css, options, assets, limits, false, false,
    )?)
}

fn design_system(api: super::CssCompiler) -> Result<DesignSystem, CompileError> {
    // `@import … important` and `@tailwind utilities important` mark the system.
    let important = api.compiler.important;
    Ok(DesignSystem {
        retained_bytes: api.compiler.report().metrics.cache_bytes,
        candidate_order: Vec::new(),
        reserved_cache: 0,
        compiler: api.compiler,
        parsed_candidates: BTreeMap::new(),
        parsed_variants: Vec::new(),
        variant_indices: BTreeMap::new(),
        used_variables: BTreeSet::new(),
        important,
        invalid_candidates: Vec::new(),
    })
}

fn syntax(message: &str) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}
fn string<'a>(value: &'a PublicValue, name: &str) -> Result<&'a Text, CompileError> {
    value
        .get(name)
        .and_then(PublicValue::as_text)
        .ok_or_else(|| syntax("Invalid public design-system string field"))
}
fn text_cmp(a: &Text, b: &Text) -> Ordering {
    a.units().cmp(b.units())
}
fn undefined() -> PublicValue {
    public_json!({"undefined":true})
}
fn json_bytes(value: &PublicValue) -> usize {
    json_bytes_seen(value, &mut BTreeSet::new())
}
pub(in crate::compiler) fn json_bytes_seen(
    value: &PublicValue,
    seen: &mut BTreeSet<(u8, usize)>,
) -> usize {
    match value {
        PublicValue::String(value) => value.owned_bytes().saturating_add(32),
        PublicValue::Array(values) => {
            if !seen.insert((0, Arc::as_ptr(values) as usize)) {
                return 0;
            }
            values.iter().fold(
                std::mem::size_of::<Vec<PublicValue>>()
                    .saturating_add(2 * std::mem::size_of::<usize>())
                    .saturating_add(
                        values
                            .capacity()
                            .saturating_mul(std::mem::size_of::<PublicValue>()),
                    ),
                |total, value| total.saturating_add(json_bytes_seen(value, seen)),
            )
        }
        PublicValue::Object(values) => {
            if !seen.insert((1, Arc::as_ptr(values) as usize)) {
                return 0;
            }
            let identity = if seen.insert((2, Arc::as_ptr(values.identity()) as usize)) {
                2 * std::mem::size_of::<usize>()
            } else {
                0
            };
            values.iter().fold(
                std::mem::size_of::<PublicObject>()
                    .saturating_add(2 * std::mem::size_of::<usize>())
                    .saturating_add(identity)
                    .saturating_add(values.capacity().saturating_mul(128)),
                |total, (key, value)| {
                    total
                        .saturating_add(key.owned_bytes())
                        .saturating_add(json_bytes_seen(value, seen))
                },
            )
        }
        _ => 32,
    }
}

impl DesignSystem {
    pub fn with_reserved_cache<T>(
        &mut self,
        reserved: usize,
        operation: impl FnOnce(&mut Self) -> Result<T, CompileError>,
    ) -> Result<T, CompileError> {
        let previous = self.reserved_cache;
        let checkpoint = self.cache_checkpoint();
        self.reserved_cache = self.reserved_cache.saturating_add(reserved);
        let result = self.budget(0).and_then(|_| operation(self));
        self.reserved_cache = previous;
        if result.is_err() {
            self.rollback_cache(checkpoint);
        }
        result
    }
    pub fn cache_checkpoint(&self) -> CacheCheckpoint {
        CacheCheckpoint {
            candidates: self.candidate_order.len(),
            variants: self.parsed_variants.len(),
            retained_bytes: self.retained_bytes,
            candidate_capacity: self.candidate_order.capacity(),
            variant_capacity: self.parsed_variants.capacity(),
        }
    }
    pub fn rollback_cache(&mut self, checkpoint: CacheCheckpoint) {
        while self.candidate_order.len() > checkpoint.candidates {
            if let Some(raw) = self.candidate_order.pop() {
                self.parsed_candidates.remove(&raw);
            }
        }
        while self.parsed_variants.len() > checkpoint.variants {
            if let Some((raw, _)) = self.parsed_variants.pop() {
                self.variant_indices.remove(&raw);
            }
        }
        self.candidate_order
            .shrink_to(checkpoint.candidate_capacity);
        self.parsed_variants.shrink_to(checkpoint.variant_capacity);
        self.retained_bytes = checkpoint.retained_bytes;
    }
    pub fn parse_candidate_text(&mut self, raw: &Text) -> Result<Vec<PublicValue>, CompileError> {
        self.parse_candidate_text_with_reserved_cache(raw, 0)
    }
    pub fn parse_candidate_text_with_reserved_cache(
        &mut self,
        raw: &Text,
        reserved: usize,
    ) -> Result<Vec<PublicValue>, CompileError> {
        let checkpoint = self.cache_checkpoint();
        let previous = self.reserved_cache;
        self.reserved_cache = self.reserved_cache.saturating_add(reserved);
        let result = self.parse_candidate_impl(raw).and_then(|parsed| {
            let bytes = parsed
                .iter()
                .map(PublicValue::owned_bytes)
                .sum::<usize>()
                .saturating_add(
                    parsed
                        .capacity()
                        .saturating_mul(std::mem::size_of::<PublicValue>()),
                );
            self.budget(if reserved == 0 {
                bytes
            } else {
                bytes
                    .saturating_mul(4)
                    .saturating_add(raw.owned_bytes())
                    .saturating_add(128)
            })?;
            Ok(parsed)
        });
        self.reserved_cache = previous;
        if result.is_err() {
            self.rollback_cache(checkpoint);
        }
        result
    }
    pub fn parse_variant_text(&mut self, raw: &Text) -> Result<PublicValue, CompileError> {
        self.parse_variant_text_with_reserved_cache(raw, 0)
    }
    pub fn parse_variant_text_with_reserved_cache(
        &mut self,
        raw: &Text,
        reserved: usize,
    ) -> Result<PublicValue, CompileError> {
        let checkpoint = self.cache_checkpoint();
        let previous = self.reserved_cache;
        self.reserved_cache = self.reserved_cache.saturating_add(reserved);
        let result = self.parse_variant_impl(raw).and_then(|parsed| {
            self.budget(if reserved == 0 {
                parsed.owned_bytes()
            } else {
                parsed
                    .owned_bytes()
                    .saturating_mul(4)
                    .saturating_add(raw.owned_bytes())
                    .saturating_add(128)
            })?;
            Ok(parsed)
        });
        self.reserved_cache = previous;
        if result.is_err() {
            self.rollback_cache(checkpoint);
        }
        result
    }
    pub fn candidate_variant_inputs_text(&self, raw: &Text) -> Result<Vec<Text>, CompileError> {
        self.input(raw)?;
        let parts = value::segment_text(raw, 58, self.compiler.limits.ast_depth)?;
        let start = usize::from(self.compiler.theme.prefix.is_some());
        let end = parts.len().saturating_sub(1);
        let result = parts
            .get(start..end)
            .unwrap_or_default()
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        self.budget(
            result
                .iter()
                .map(Text::owned_bytes)
                .sum::<usize>()
                .saturating_add(
                    result
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Text>()),
                ),
        )?;
        Ok(result)
    }
    pub fn variant_child_input_text(&self, raw: &Text) -> Result<Option<Text>, CompileError> {
        self.input(raw)?;
        let child = match compiler::candidate::parse_variant_with_theme(
            raw,
            self.compiler.limits.ast_depth,
            &self.compiler.theme,
        )? {
            Some(compiler::candidate::Variant::Registered {
                root,
                value: Some(child),
                ..
            }) if variants::root_kind(&root, &self.compiler.theme)
                == Some(variants::VariantKind::Compound) =>
            {
                Some(child)
            }
            _ => None,
        };
        self.budget(child.as_ref().map_or(0, Text::owned_bytes))?;
        Ok(child)
    }
    pub fn parse_candidate(&mut self, raw: &str) -> Result<Vec<serde_json::Value>, CompileError> {
        self.parse_candidate_text(&raw.into())?
            .iter()
            .map(PublicValue::scalar)
            .collect()
    }
    pub fn parse_variant(&mut self, raw: &str) -> Result<serde_json::Value, CompileError> {
        self.parse_variant_text(&raw.into())?.scalar()
    }
    pub fn print_candidate(&self, value: &serde_json::Value) -> Result<String, CompileError> {
        self.print_candidate_text(&PublicValue::from_scalar(value)?)?
            .to_utf8()
            .map_err(|_| syntax("Non-scalar output requires the Text facade"))
    }
    pub fn print_variant(&self, value: &serde_json::Value) -> Result<String, CompileError> {
        self.print_variant_text(&PublicValue::from_scalar(value)?)?
            .to_utf8()
            .map_err(|_| syntax("Non-scalar output requires the Text facade"))
    }
    pub fn compile_ast_nodes(
        &mut self,
        value: &serde_json::Value,
        flags: u32,
    ) -> Result<Vec<serde_json::Value>, CompileError> {
        self.compile_ast_nodes_text(&PublicValue::from_scalar(value)?, flags)?
            .iter()
            .map(PublicValue::scalar)
            .collect()
    }
    pub fn get_class_order(
        &mut self,
        classes: &[String],
    ) -> Result<serde_json::Value, CompileError> {
        self.get_class_order_text(&classes.iter().map(Text::from).collect::<Vec<_>>())?
            .scalar()
    }
    pub fn candidates_to_css(
        &mut self,
        classes: &[String],
    ) -> Result<serde_json::Value, CompileError> {
        self.candidates_to_css_text(&classes.iter().map(Text::from).collect::<Vec<_>>())?
            .scalar()
    }
    pub fn resolve_theme_value(
        &self,
        path: &str,
        inline: bool,
    ) -> Result<serde_json::Value, CompileError> {
        self.resolve_theme_value_text(&path.into(), inline)?
            .scalar()
    }
    pub fn invalid_candidates(&self) -> Result<serde_json::Value, CompileError> {
        self.invalid_candidates_text().scalar()
    }
    pub fn get_class_list(&self) -> Result<serde_json::Value, CompileError> {
        self.get_class_list_text()?.scalar()
    }
    pub fn get_variants(&self) -> Result<Vec<serde_json::Value>, CompileError> {
        self.get_variants_text()?
            .iter()
            .map(PublicValue::scalar)
            .collect()
    }
    pub fn variant_selectors(
        &mut self,
        root: &str,
        options: &serde_json::Value,
    ) -> Result<serde_json::Value, CompileError> {
        self.variant_selectors_text(&root.into(), &PublicValue::from_scalar(options)?)?
            .scalar()
    }
    pub fn variant_order_entries(
        &self,
    ) -> Result<Vec<(String, serde_json::Value, usize)>, CompileError> {
        self.variant_order_entries_text()?
            .into_iter()
            .map(|(raw, value, index)| {
                Ok((
                    raw.to_utf8()
                        .map_err(|_| syntax("Non-scalar variant requires the Text facade"))?,
                    value.scalar()?,
                    index,
                ))
            })
            .collect()
    }
    pub fn get_variant_order(&self) -> Result<serde_json::Value, CompileError> {
        self.get_variant_order_text()?.scalar()
    }
    pub fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    fn public_input(&self, value: &PublicValue) -> Result<(), CompileError> {
        let mut stack = vec![(value, 0usize)];
        let mut bytes = 0usize;
        while let Some((value, depth)) = stack.pop() {
            compiler::check_limit(depth, self.compiler.limits.ast_depth, "ast_depth")?;
            match value {
                PublicValue::String(value) => bytes = bytes.saturating_add(value.len()),
                PublicValue::Array(values) => {
                    stack.extend(values.iter().map(|value| (value, depth + 1)))
                }
                PublicValue::Object(values) => {
                    bytes = bytes.saturating_add(values.keys().map(Text::len).sum::<usize>());
                    stack.extend(values.values().map(|value| (value, depth + 1)));
                }
                _ => {}
            }
            compiler::check_limit(bytes, self.compiler.limits.input_bytes, "input_bytes")?;
            self.budget(stack.len().saturating_mul(32).saturating_add(bytes))?;
        }
        Ok(())
    }
    fn input(&self, raw: &Text) -> Result<(), CompileError> {
        compiler::check_limit(raw.len(), self.compiler.limits.input_bytes, "input_bytes")
    }
    fn budget(&self, extra: usize) -> Result<(), CompileError> {
        compiler::check_limit(
            self.retained_bytes
                .saturating_add(self.reserved_cache)
                .saturating_add(extra),
            self.compiler.limits.cache_bytes,
            "cache_bytes",
        )
    }
    fn result(&self, value: PublicValue) -> Result<PublicValue, CompileError> {
        self.budget(json_bytes(&value))?;
        Ok(value)
    }

    fn parse_candidate_impl(&mut self, raw: &Text) -> Result<Vec<PublicValue>, CompileError> {
        self.input(raw)?;
        if let Some(parsed) = self.parsed_candidates.get(raw) {
            self.budget(parsed.iter().map(json_bytes).sum())?;
            return Ok(parsed.clone());
        }
        compiler::check_limit(
            self.parsed_candidates.len().saturating_add(1),
            self.compiler.limits.candidate_count,
            "candidate_count",
        )?;
        let candidates = compiler::candidate::parse_with_utilities(
            raw,
            self.compiler.limits.ast_depth,
            &self.compiler.theme,
            &self.compiler.custom,
        )?;
        let mut result = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            let mut variants = Vec::with_capacity(candidate.variants.len());
            let parts = value::segment_text(raw, 58, self.compiler.limits.ast_depth)?;
            let start = usize::from(self.compiler.theme.prefix.is_some());
            for part in parts[start..parts.len() - 1].iter().rev() {
                variants.push(self.parse_variant_text(part)?);
            }
            result.push(public_candidate(
                candidate,
                variants,
                self.compiler.limits.ast_depth,
            )?);
        }
        let bytes = result
            .iter()
            .map(json_bytes)
            .sum::<usize>()
            .saturating_add(raw.owned_bytes().saturating_mul(2))
            .saturating_add(std::mem::size_of::<Text>().saturating_mul(4))
            .saturating_add(128);
        self.budget(bytes)?;
        self.retained_bytes = self.retained_bytes.saturating_add(bytes);
        self.candidate_order.push(raw.clone());
        self.parsed_candidates.insert(raw.into(), result.clone());
        Ok(result)
    }

    fn parse_variant_impl(&mut self, raw: &Text) -> Result<PublicValue, CompileError> {
        self.input(raw)?;
        if let Some(index) = self.variant_indices.get(raw) {
            self.budget(json_bytes(&self.parsed_variants[*index].1))?;
            return Ok(self.parsed_variants[*index].1.clone());
        }
        compiler::check_limit(
            self.parsed_variants.len().saturating_add(1),
            self.compiler.limits.candidate_count,
            "candidate_count",
        )?;
        // Compound children are inserted first, as in the upstream DefaultMap.
        if let Some(compiler::candidate::Variant::Registered {
            root,
            value: Some(child),
            ..
        }) = compiler::candidate::parse_variant_with_theme(
            raw,
            self.compiler.limits.ast_depth,
            &self.compiler.theme,
        )? {
            if variants::root_kind(&root, &self.compiler.theme)
                == Some(variants::VariantKind::Compound)
            {
                self.parse_variant_text(&child)?;
            }
        }
        let parsed = typed(&compiler::candidate::parse_variant_object(
            raw,
            self.compiler.limits.ast_depth,
            &self.compiler.theme,
        )?)?;
        let bytes = json_bytes(&parsed)
            .saturating_add(raw.len().saturating_mul(2))
            .saturating_add(192);
        self.budget(bytes)?;
        self.retained_bytes = self.retained_bytes.saturating_add(bytes);
        self.variant_indices
            .insert(raw.into(), self.parsed_variants.len());
        self.parsed_variants.push((raw.into(), parsed.clone()));
        Ok(parsed)
    }

    pub fn print_candidate_text(&self, candidate: &PublicValue) -> Result<Text, CompileError> {
        self.public_input(candidate)?;
        let mut parts = candidate["variants"]
            .as_array()
            .ok_or_else(|| syntax("Invalid candidate variants"))?
            .iter()
            .rev()
            .map(|variant| self.print_variant_text(variant))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(prefix) = &self.compiler.theme.prefix {
            parts.insert(0, Text::from(prefix));
        }
        let mut base = match literal(string(candidate, "kind")?) {
            "static" => string(candidate, "root")?.into(),
            "functional" => {
                let mut base = string(candidate, "root")?.clone();
                if !candidate["value"].is_null() {
                    base.push_str("-");
                    base.push_text(&print_value(
                        &candidate["value"],
                        true,
                        self.compiler.limits.ast_depth,
                    )?);
                }
                base.push_text(&print_modifier(
                    &candidate["modifier"],
                    self.compiler.limits.ast_depth,
                )?);
                base
            }
            "arbitrary" => concat(&[
                &Text::from("["),
                string(candidate, "property")?,
                &Text::from(":"),
                &print_arbitrary(string(candidate, "value")?, self.compiler.limits.ast_depth)?,
                &Text::from("]"),
                &print_modifier(&candidate["modifier"], self.compiler.limits.ast_depth)?,
            ]),
            _ => return Err(syntax("Invalid candidate kind")),
        };
        if candidate["important"].as_bool().unwrap_or(false) {
            base.push_str("!");
        }
        parts.push(base);
        let mut result = Text::default();
        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                result.push_str(":");
            }
            result.push_text(part);
        }
        self.input(&result)?;
        Ok(result)
    }
    pub fn print_variant_text(&self, variant: &PublicValue) -> Result<Text, CompileError> {
        self.public_input(variant)?;
        print_variant(variant, self.compiler.limits.ast_depth)
    }

    fn internal_candidate(&self, candidate: &PublicValue) -> Result<Candidate, CompileError> {
        self.public_input(candidate)?;
        let raw = string(candidate, "raw")?.clone();
        self.input(&raw)?;
        let kind = match literal(string(candidate, "kind")?) {
            "static" => CandidateKind::Static {
                root: string(candidate, "root")?.into(),
            },
            "functional" => CandidateKind::Functional {
                root: string(candidate, "root")?.into(),
                value: if candidate["value"].is_null() {
                    None
                } else {
                    Some(match literal(string(&candidate["value"], "kind")?) {
                        "named" => CandidateValue::Named {
                            value: string(&candidate["value"], "value")?.into(),
                        },
                        "arbitrary" => CandidateValue::Arbitrary {
                            value: string(&candidate["value"], "value")?.into(),
                            data_type: candidate["value"]["dataType"].as_text().cloned(),
                        },
                        _ => return Err(syntax("Invalid candidate value")),
                    })
                },
                modifier: internal_modifier(
                    &candidate["modifier"],
                    self.compiler.limits.ast_depth,
                )?,
            },
            "arbitrary" => CandidateKind::ArbitraryProperty {
                property: string(candidate, "property")?.into(),
                value: string(candidate, "value")?.into(),
                modifier: internal_modifier(
                    &candidate["modifier"],
                    self.compiler.limits.ast_depth,
                )?,
            },
            _ => return Err(syntax("Invalid candidate kind")),
        };
        let variants = candidate["variants"]
            .as_array()
            .ok_or_else(|| syntax("Invalid candidate variants"))?
            .iter()
            .map(|variant| {
                let raw = self.print_variant_text(variant)?;
                compiler::candidate::parse_variant_with_theme(
                    &raw,
                    self.compiler.limits.ast_depth,
                    &self.compiler.theme,
                )?
                .ok_or_else(|| syntax("Invalid candidate variant"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Candidate {
            raw,
            kind,
            variants,
            important: candidate["important"].as_bool().unwrap_or(false),
        })
    }
    fn compiled(
        &self,
        candidate: &PublicValue,
        flags: u32,
    ) -> Result<Vec<CompiledCandidate>, CompileError> {
        let mut candidate = self.internal_candidate(candidate)?;
        candidate.important |= self.important && flags & 1 != 0;
        match compiler::utilities::compile_all(
            &candidate,
            &self.compiler.theme,
            &self.compiler.limits,
            &self.compiler.custom,
        ) {
            Ok(outputs) => Ok(outputs),
            Err(CompileFailure::Resource(error)) => Err(error),
            Err(CompileFailure::Candidate(_)) => Ok(vec![]),
        }
    }
    pub fn compile_ast_nodes_text(
        &mut self,
        candidate: &PublicValue,
        flags: u32,
    ) -> Result<Vec<PublicValue>, CompileError> {
        let values = self.compiled(candidate,flags)?.iter().map(|output| Ok(public_json!({"node":node_json(&output.node)?,"propertySort":public_json!({"order":output.property_order,"count":output.declaration_count})}))).collect::<Result<Vec<_>, CompileError>>()?;
        self.budget(values.iter().map(json_bytes).sum())?;
        Ok(values)
    }
    fn compile_classes(
        &mut self,
        classes: &[Text],
    ) -> Result<Vec<CompiledCandidate>, CompileError> {
        compiler::check_limit(
            classes.len(),
            self.compiler.limits.candidate_count,
            "candidate_count",
        )?;
        let mut rules = Vec::new();
        let unique: BTreeSet<_> = classes.iter().collect();
        for raw in unique {
            if self.invalid_candidates.contains(raw) {
                continue;
            }
            for candidate in self.parse_candidate_text(raw)? {
                rules.extend(self.compiled(&candidate, 1)?);
            }
        }
        rules.sort_by(compiler::compare_rules);
        Ok(rules)
    }
    pub fn get_class_order_text(&mut self, classes: &[Text]) -> Result<PublicValue, CompileError> {
        let rules = self.compile_classes(classes)?;
        let mut order = BTreeMap::new();
        for rule in rules {
            let index = order.len();
            order.entry(rule.program.candidate).or_insert(index);
        }
        self.result(public_json!(classes
            .iter()
            .map(|raw| public_json!([
                raw,
                order
                    .get(raw)
                    .map(|index| public_json!({"bigint":index.to_string()}))
            ]))
            .collect::<Vec<_>>()))
    }
    pub fn candidates_to_css_text(
        &mut self,
        classes: &[Text],
    ) -> Result<PublicValue, CompileError> {
        compiler::check_limit(
            classes.len(),
            self.compiler.limits.candidate_count,
            "candidate_count",
        )?;
        let mut values = Vec::with_capacity(classes.len());
        for raw in classes {
            let rules = self.compile_classes(std::slice::from_ref(raw))?;
            if rules.is_empty() {
                values.push(PublicValue::Null);
                continue;
            }
            let mut nodes = rules.into_iter().map(|rule| rule.node).collect::<Vec<_>>();
            let mut globals = Vec::new();
            compiler::ast::hoist_at_roots(&mut nodes, &mut globals);
            nodes.extend(globals);
            compiler::ast::optimize_rules(&mut nodes);
            let css = compiler::ast::to_css(&nodes);
            compiler::check_limit(css.len(), self.compiler.limits.css_bytes, "css_bytes")?;
            values.push(public_json!(css));
        }
        self.result(public_json!(values))
    }

    pub fn resolve_theme_value_text(
        &self,
        path: &Text,
        force_inline: bool,
    ) -> Result<PublicValue, CompileError> {
        self.input(path)?;
        let (path, alpha) = if let Some(index) = path.rfind("/") {
            (
                Text::from_units(path.units()[..index].to_vec()).trim_js(),
                Some(Text::from_units(path.units()[index + 1..].to_vec()).trim_js()),
            )
        } else {
            (path.clone(), None)
        };
        let Some(token) = self.compiler.theme.tokens.get(&path) else {
            return Ok(undefined());
        };
        let mut used = BTreeSet::new();
        let resolved = if force_inline {
            Some(token.value.clone())
        } else {
            self.compiler.theme.resolve_text(&path, &mut used)
        };
        let value = match (resolved, alpha.filter(|alpha| !alpha.is_empty())) {
            (Some(value), Some(alpha)) => {
                let modifier = concat(&[&Text::from("["), &alpha, &Text::from("]")]);
                match compiler::colors::as_color(
                    value,
                    Some(&modifier),
                    &self.compiler.theme,
                    &mut used,
                ) {
                    Ok(value) => value,
                    Err(FamilyFailure::Candidate(_)) => None,
                    Err(FamilyFailure::Resource(error)) => return Err(error),
                }
            }
            (value, _) => value,
        };
        self.result(value.map_or_else(undefined, |value| public_json!(value)))
    }
    /// Number of entries and their owned snapshot bytes, without cloning text.
    pub fn theme_entries_storage_bytes(&self) -> (usize, usize) {
        let prefix_units = self
            .compiler
            .theme
            .prefix
            .as_ref()
            .map(|prefix| prefix.encode_utf16().count().saturating_add(3));
        self.compiler.theme.ordered_tokens().fold(
            (0usize, 0usize),
            |(count, total), (name, token)| {
                let key_bytes = prefix_units.map_or(name.owned_bytes(), |prefix| {
                    prefix
                        .saturating_add(name.units().get(2..).unwrap_or(name.units()).len())
                        .saturating_mul(2)
                });
                (
                    count.saturating_add(1),
                    total
                        .saturating_add(std::mem::size_of::<(Text, ThemeEntry)>())
                        .saturating_add(key_bytes)
                        .saturating_add(token.value.owned_bytes()),
                )
            },
        )
    }
    /// Borrowed token locations let crossings budget shared source graphs first.
    pub fn theme_entry_sources(&self) -> impl Iterator<Item = &SourceLocation> {
        self.compiler
            .theme
            .ordered_tokens()
            .filter_map(|(_, token)| token.source.as_ref())
    }
    pub fn theme_entries(&self) -> Result<Vec<(Text, ThemeEntry)>, CompileError> {
        let (count, bytes) = self.theme_entries_storage_bytes();
        self.budget(bytes)?;
        let mut entries = Vec::with_capacity(count);
        entries.extend(self.compiler.theme.ordered_tokens().map(|(name, token)| {
            let options = u32::from(token.inline)
                | (u32::from(token.reference) << 1)
                | (u32::from(token.default) << 2)
                | (u32::from(token.always_emit) << 3)
                | (u32::from(self.used_variables.contains(name)) << 4);
            (
                self.compiler.theme.prefix_key_text(name),
                ThemeEntry {
                    value: token.value.clone(),
                    options,
                    src: token.source.clone(),
                },
            )
        }));
        Ok(entries)
    }
    pub fn track_used_variables(&mut self, raw: &str) -> Result<(), CompileError> {
        self.input(&Text::from(raw))?;
        self.budget(raw.encode_utf16().count().saturating_mul(8))?;
        let raw = Text::from(raw);
        self.track_used_variables_text_with_reserved_cache(&raw, raw.owned_bytes())
    }
    pub fn track_used_variables_text(&mut self, raw: &Text) -> Result<(), CompileError> {
        self.track_used_variables_text_with_reserved_cache(raw, 0)
    }
    /// Includes retained crossing values in the same tracking transaction budget.
    pub fn track_used_variables_text_with_reserved_cache(
        &mut self,
        raw: &Text,
        reserved: usize,
    ) -> Result<(), CompileError> {
        use std::mem::size_of;
        fn set_nodes_bytes(count: usize) -> usize {
            count
                .div_ceil(5)
                .saturating_mul(11)
                .saturating_mul(size_of::<Text>() + 2 * size_of::<usize>())
        }
        compiler::check_limit(raw.len(), self.compiler.limits.input_bytes, "input_bytes")?;
        if raw.is_empty() {
            return self.budget(reserved);
        }
        // Allocation bounds, not another value parser: every separator or paren
        // can split a word; a terminal backslash adds the literal "undefined".
        let mut boundaries = 0usize;
        let mut opens = 0usize;
        for &unit in raw.units() {
            if matches!(unit, 58 | 44 | 61 | 62 | 60 | 10 | 47 | 32 | 9 | 40 | 41) {
                boundaries = boundaries.saturating_add(1);
            }
            opens = opens.saturating_add(usize::from(unit == 40));
        }
        let nodes = boundaries.saturating_mul(2).saturating_add(1);
        let units = raw
            .units()
            .len()
            .saturating_add(usize::from(raw.units().last() == Some(&92)) * 9);
        // Nonempty node/variable/parent Vecs have at least four slots, and
        // doubling stays below four slots per item across all nested vectors.
        // Text: parser buffers (8/unit), variable clones (2), prefix scratch (8),
        // decoded pending keys (8); CRLF normalization has its own buffer.
        let temporary = nodes
            .saturating_mul(4)
            .saturating_mul(size_of::<value::ValueNode>() + size_of::<Text>())
            .saturating_add(
                opens
                    .min(self.compiler.limits.ast_depth)
                    .saturating_mul(4)
                    .saturating_mul(size_of::<(Text, Vec<value::ValueNode>)>()),
            )
            .saturating_add(units.saturating_mul(26))
            .saturating_add(
                usize::from(raw.units().windows(2).any(|pair| pair == [13, 10]))
                    .saturating_mul(raw.units().len())
                    .saturating_mul(8),
            )
            // Both the pending tree and growth of the committed tree can be live.
            .saturating_add(set_nodes_bytes(nodes).saturating_mul(2));
        self.budget(reserved.saturating_add(temporary))?;
        let parsed = value::parse_text(raw, self.compiler.limits.ast_depth)?;
        let used = compiler::theme::used_variables(&parsed);
        drop(parsed);
        let prefix = self
            .compiler
            .theme
            .prefix
            .as_ref()
            .map(|prefix| prefix.encode_utf16().count().saturating_add(3));
        let mut pending = BTreeSet::new();
        let mut failure = None;
        for variable in used {
            let name = if let Some(prefix) = &prefix {
                let rest = variable.units().get(*prefix..).unwrap_or_default();
                let mut units = Vec::with_capacity(rest.len().saturating_add(2));
                units.extend([45, 45]);
                units.extend_from_slice(rest);
                Text::from_units(units)
            } else {
                variable
            };
            let name = match compiler::legacy_theme::unescape_key_units(&name) {
                Ok(name) => name,
                Err(error @ CompileError::RangeError { .. }) => {
                    failure = Some(error);
                    break;
                }
                Err(error) => return Err(error),
            };
            if self.compiler.theme.tokens.contains_key(&name)
                && !self.used_variables.contains(&name)
            {
                pending.insert(name);
            }
        }
        let bytes = pending
            .iter()
            .fold(0usize, |bytes, name| {
                bytes.saturating_add(name.owned_bytes())
            })
            .saturating_add(
                set_nodes_bytes(self.used_variables.len().saturating_add(pending.len()))
                    .saturating_sub(set_nodes_bytes(self.used_variables.len())),
            );
        self.budget(reserved.saturating_add(bytes))?;
        for name in pending {
            self.used_variables.insert(name);
        }
        self.retained_bytes = self.retained_bytes.saturating_add(bytes);
        failure.map_or(Ok(()), Err)
    }
    pub fn invalid_candidates_text(&self) -> PublicValue {
        public_json!(self.invalid_candidates)
    }
    /// Replaces the caller's invalid-candidate set; duplicates keep their first position.
    pub fn set_invalid_candidates_text(&mut self, values: Vec<Text>) -> Result<(), CompileError> {
        compiler::check_limit(
            values.len(),
            self.compiler.limits.candidate_count,
            "candidate_count",
        )?;
        let mut seen = BTreeSet::new();
        let unique: Vec<Text> = values
            .into_iter()
            .filter(|value| seen.insert(value.clone()))
            .collect();
        let bytes = unique.iter().map(Text::owned_bytes).sum::<usize>();
        let previous = self
            .invalid_candidates
            .iter()
            .map(Text::owned_bytes)
            .sum::<usize>();
        self.budget(bytes)?;
        self.retained_bytes = self
            .retained_bytes
            .saturating_sub(previous)
            .saturating_add(bytes);
        self.invalid_candidates = unique;
        Ok(())
    }

    pub fn get_class_list_text(&self) -> Result<PublicValue, CompileError> {
        let mut limits = self.compiler.limits.clone();
        limits.cache_bytes = limits
            .cache_bytes
            .saturating_sub(self.retained_bytes)
            .saturating_sub(self.reserved_cache);
        class_list(&self.compiler.theme, &self.compiler.custom, &limits)
    }
    fn variant_roots(&self) -> Result<Vec<Text>, CompileError> {
        let mut roots = Vec::new();
        for (root, rank) in compiler::registry::VARIANT_ORDER {
            if *rank == 64 && *root != "min" {
                continue;
            }
            if *root == "min" {
                for (name, _) in self.compiler.theme.ordered_tokens() {
                    if let Some(root) = name.strip_prefix("--breakpoint-") {
                        if !roots.contains(&root) {
                            roots.push(root);
                        }
                    }
                }
            }
            roots.push((*root).into());
        }
        for root in self.compiler.theme.custom_variants.root_names() {
            if !roots.iter().any(|name| name == root) {
                roots.push(root.into());
            }
        }
        Ok(roots)
    }
    pub fn get_variants_text(&self) -> Result<Vec<PublicValue>, CompileError> {
        let roots = self.variant_roots()?;
        let mut values = Vec::new();
        for root in &roots {
            let kind = variants::root_kind(root, &self.compiler.theme);
            let suggestions = match literal(root) {
                "not" | "group" | "peer" | "in" | "has" => roots
                    .iter()
                    .filter(|name| {
                        let variant = compiler::candidate::Variant::Registered {
                            raw: (*name).clone(),
                            root: (*name).clone(),
                            value: None,
                            modifier: None,
                        };
                        variants::compounds(&variant, &self.compiler.theme)
                            & if root == "not" { 3 } else { 2 }
                            != 0
                    })
                    .cloned()
                    .collect::<Vec<_>>(),
                "aria" => [
                    "busy", "checked", "disabled", "expanded", "hidden", "pressed", "readonly",
                    "required", "selected",
                ]
                .map(Text::from)
                .into(),
                "min" | "max" => {
                    namespace_keys(&self.compiler.theme, &[&Text::from("--breakpoint")])?
                }
                "@" | "@min" | "@max" => {
                    namespace_keys(&self.compiler.theme, &[&Text::from("--container")])?
                }
                _ => vec![],
            };
            values.push(public_json!({"name":root,"values":suggestions,"isArbitrary":kind!=Some(variants::VariantKind::Static),"hasDash":root!="@"}));
        }
        self.budget(values.iter().map(json_bytes).sum())?;
        Ok(values)
    }
    pub fn variant_selectors_text(
        &mut self,
        root: &Text,
        options: &PublicValue,
    ) -> Result<PublicValue, CompileError> {
        let mut raw = root.clone();
        if let Some(value) = options["value"].as_text().filter(|value| !value.is_empty()) {
            if root != "@" {
                raw.push_str("-");
            }
            raw.push_text(value);
        }
        if let Some(modifier) = options["modifier"]
            .as_text()
            .filter(|value| !value.is_empty())
        {
            raw.push_str("/");
            raw.push_text(modifier);
        }
        let parsed = self.parse_variant_text(&raw)?;
        if parsed.is_null() {
            return Ok(public_json!([]));
        }
        let Some(variant) = compiler::candidate::parse_variant_with_theme(
            &raw,
            self.compiler.limits.ast_depth,
            &self.compiler.theme,
        )?
        else {
            return Ok(public_json!([]));
        };
        let output = match variants::compile_variant(
            &variant,
            vec![],
            &mut vec![],
            &self.compiler.theme,
            &self.compiler.limits,
        ) {
            Ok(output) => output,
            Err(variants::Failure::Resource(error)) => return Err(error),
            Err(variants::Failure::Candidate(..)) => return Ok(public_json!([])),
        };
        let mut selectors = Vec::new();
        selector_paths(&output.nodes, &mut vec![], &mut selectors)?;
        self.result(public_json!(selectors))
    }
    pub fn variant_order_entries_text(
        &self,
    ) -> Result<Vec<(Text, PublicValue, usize)>, CompileError> {
        let mut variants = self
            .parsed_variants
            .iter()
            .filter(|(_, variant)| !variant.is_null())
            .collect::<Vec<_>>();
        let mut breakpoint_values = BTreeMap::new();
        for (raw, variant) in &variants {
            collect_breakpoint_values(raw, variant, &self.compiler.theme, &mut breakpoint_values)?;
        }
        let breakpoint_bytes = breakpoint_values
            .iter()
            .fold(0usize, |total, (raw, value)| {
                total
                    .saturating_add(raw.owned_bytes())
                    .saturating_add(value.as_ref().map_or(0, Text::owned_bytes))
                    .saturating_add(128)
            });
        self.budget(breakpoint_bytes)?;
        variants.sort_by(|(a, av), (b, bv)| {
            cached_variant_cmp(a, av, b, bv, &self.compiler.theme, &breakpoint_values)
        });
        let mut index = 0;
        let mut previous: Option<(&Text, &PublicValue)> = None;
        let mut ordered = Vec::new();
        for (raw, variant) in variants {
            if previous.is_some_and(|(previous_raw, previous)| {
                cached_variant_cmp(
                    previous_raw,
                    previous,
                    raw,
                    variant,
                    &self.compiler.theme,
                    &breakpoint_values,
                ) != Ordering::Equal
            }) {
                index += 1;
            }
            ordered.push((raw.clone(), variant.clone(), index));
            previous = Some((raw, variant));
        }
        self.budget(
            ordered
                .iter()
                .map(|(raw, variant, _)| raw.len().saturating_add(json_bytes(variant)))
                .sum::<usize>()
                .saturating_add(breakpoint_bytes),
        )?;
        Ok(ordered)
    }
    pub fn get_variant_order_text(&self) -> Result<PublicValue, CompileError> {
        self.result(public_json!(self
            .variant_order_entries_text()?
            .into_iter()
            .map(|(_, variant, order)| public_json!({"variant":variant,"order":order}))
            .collect::<Vec<_>>()))
    }
}

/// Upstream's plain candidate object; `variants` are already public, innermost last.
pub(in crate::compiler) fn public_candidate(
    candidate: Candidate,
    variants: Vec<PublicValue>,
    depth: usize,
) -> Result<PublicValue, CompileError> {
    let mut node = match candidate.kind {
        CandidateKind::Static { root } => public_json!({"kind":"static","root":root}),
        CandidateKind::Functional {
            root,
            value,
            modifier,
        } => {
            let modifier = modifier_json(modifier.as_ref(), depth)?;
            let value = match value {
                None => PublicValue::Null,
                Some(CandidateValue::Named { value }) => {
                    public_json!({"kind":"named","value":value,"fraction":modifier["value"].as_text().filter(|_| modifier["kind"] == "named").map(|m|concat(&[&value,&Text::from("/"),m]))})
                }
                Some(CandidateValue::Arbitrary { value, data_type }) => {
                    public_json!({"kind":"arbitrary","dataType":data_type,"value":value})
                }
            };
            public_json!({"kind":"functional","root":root,"modifier":modifier,"value":value})
        }
        CandidateKind::ArbitraryProperty {
            property,
            value,
            modifier,
        } => {
            public_json!({"kind":"arbitrary","property":property,"value":value,"modifier":modifier_json(modifier.as_ref(),depth)?})
        }
    };
    node["variants"] = public_json!(variants);
    node["important"] = public_json!(candidate.important);
    node["raw"] = public_json!(candidate.raw);
    Ok(node)
}
fn modifier_json(raw: Option<&Text>, depth: usize) -> Result<PublicValue, CompileError> {
    Ok(match raw {
        None => PublicValue::Null,
        Some(raw) if raw.starts_with("[") && raw.ends_with("]") => {
            let inner = Text::from_units(raw.units()[1..raw.units().len() - 1].to_vec());
            public_json!({"kind":"arbitrary","value":value::decode_arbitrary_text(&inner,depth)?})
        }
        Some(raw) if raw.starts_with("(") && raw.ends_with(")") => {
            // Upstream wraps `var(…)` before decoding, so the name keeps `_`.
            let inner = Text::from_units(raw.units()[1..raw.units().len() - 1].to_vec());
            let variable = concat(&[&Text::from("var("), &inner, &Text::from(")")]);
            public_json!({"kind":"arbitrary","value":value::decode_arbitrary_text(&variable,depth)?})
        }
        Some(raw) => public_json!({"kind":"named","value":raw}),
    })
}
fn internal_modifier(modifier: &PublicValue, depth: usize) -> Result<Option<Text>, CompileError> {
    if modifier.is_null() {
        Ok(None)
    } else {
        let printed = print_modifier(modifier, depth)?;
        let index = printed
            .units()
            .iter()
            .take_while(|unit| **unit == 47)
            .count();
        Ok(Some(Text::from_units(printed.units()[index..].to_vec())))
    }
}
fn print_modifier(modifier: &PublicValue, depth: usize) -> Result<Text, CompileError> {
    if modifier.is_null() {
        Ok(Text::default())
    } else {
        Ok(concat(&[
            &Text::from("/"),
            &print_value(modifier, false, depth)?,
        ]))
    }
}
fn is_var(raw: &Text, depth: usize) -> Result<bool, CompileError> {
    let nodes = value::parse_text(raw, depth)?;
    Ok(matches!(nodes.as_slice(),[value::ValueNode::Function {name,..}] if name=="var"))
}
fn print_value(value: &PublicValue, data_type: bool, depth: usize) -> Result<Text, CompileError> {
    let raw = string(value, "value")?;
    if value["kind"] == "named" {
        return Ok(raw.clone());
    }
    let is_var = is_var(raw, depth)?;
    let inner = if is_var {
        Text::from_units(raw.units()[4..raw.units().len() - 1].to_vec())
    } else {
        raw.clone()
    };
    let mut output = Text::from(if is_var { "(" } else { "[" });
    if data_type {
        if let Some(kind) = value["dataType"].as_text() {
            output.push_text(kind);
            output.push_str(":");
        }
    }
    output.push_text(&print_arbitrary(&inner, depth)?);
    output.push_str(if is_var { ")" } else { "]" });
    Ok(output)
}
fn print_variant(variant: &PublicValue, depth: usize) -> Result<Text, CompileError> {
    if depth == 0 {
        return Err(CompileError::Limit {
            resource: "ast_depth".into(),
            limit: 0,
        });
    }
    let mut base = match literal(string(variant, "kind")?) {
        "static" => string(variant, "root")?.clone(),
        "arbitrary" => {
            let selector = string(variant, "selector")?;
            let inner = selector
                .strip_prefix("&:is(")
                .and_then(|raw| raw.strip_suffix(")"))
                .unwrap_or_else(|| selector.clone());
            return Ok(concat(&[
                &Text::from("["),
                &print_arbitrary(&inner, depth)?,
                &Text::from("]"),
            ]));
        }
        "functional" => {
            let root = string(variant, "root")?;
            if variant["value"].is_null() {
                root.clone()
            } else {
                concat(&[
                    root,
                    &Text::from(if root == "@" { "" } else { "-" }),
                    &print_value(&variant["value"], false, depth)?,
                ])
            }
        }
        "compound" => concat(&[
            string(variant, "root")?,
            &Text::from("-"),
            &print_variant(&variant["variant"], depth - 1)?,
        ]),
        _ => return Err(syntax("Invalid variant kind")),
    };
    if variant["kind"] == "functional" || variant["kind"] == "compound" {
        base.push_text(&print_modifier(&variant["modifier"], depth)?);
    }
    Ok(base)
}
fn print_arbitrary(raw: &Text, depth: usize) -> Result<Text, CompileError> {
    fn normalize(nodes: &mut Vec<value::ValueNode>) {
        let mut drop = BTreeSet::new();
        for (index, node) in nodes.iter_mut().enumerate() {
            match node {
                value::ValueNode::Function { nodes, .. } => normalize(nodes),
                value::ValueNode::Separator { value }
                    if value.trim_ascii() == "," || value.trim_ascii() == "/" =>
                {
                    *value = value.trim_ascii()
                }
                _ => {}
            }
            if index == 0
                && matches!(node,value::ValueNode::Separator {value} if value.trim_ascii().is_empty())
            {
                drop.insert(index);
            }
        }
        if let Some(value::ValueNode::Separator { value }) = nodes.last() {
            if value.trim_ascii().is_empty() {
                drop.insert(nodes.len() - 1);
            }
        }
        for index in 1..nodes.len().saturating_sub(1) {
            if matches!(&nodes[index],value::ValueNode::Word {value} if ["+","-","*","/"].iter().any(|operator| value == *operator))
                && matches!(&nodes[index-1],value::ValueNode::Separator {value} if value==" ")
                && matches!(&nodes[index+1],value::ValueNode::Separator {value} if value==" ")
            {
                drop.insert(index - 1);
                drop.insert(index + 1);
            }
        }
        let mut index = 0;
        nodes.retain(|_| {
            let keep = !drop.contains(&index);
            index += 1;
            keep
        });
    }
    fn escape(nodes: &mut [value::ValueNode]) {
        for node in nodes {
            match node {
                value::ValueNode::Word { value } if value.starts_with("--") => {}
                value::ValueNode::Word { value } | value::ValueNode::Separator { value } => {
                    *value = value.replace_ascii(b'_', "\\_").replace_ascii(b' ', "_")
                }
                value::ValueNode::Function { name, nodes } => {
                    let url = name == "url" || name.ends_with("_url");
                    *name = name.replace_ascii(b'_', "\\_").replace_ascii(b' ', "_");
                    if !url {
                        escape(nodes);
                    }
                }
            }
        }
    }
    let mut nodes = value::parse_text(raw, depth)?;
    normalize(&mut nodes);
    escape(&mut nodes);
    Ok(value::to_css_text(&nodes))
}
fn node_json(node: &AstNode) -> Result<PublicValue, CompileError> {
    let mut value = typed(node)?;
    if let Some(kind) = value["kind"].as_text() {
        value["kind"] = public_json!(kind.replace_ascii(b'_', "-"));
    }
    if let Some(children) = node.children() {
        value["nodes"] = public_json!(children
            .iter()
            .map(node_json)
            .collect::<Result<Vec<_>, _>>()?);
    }
    Ok(value)
}
fn selector_paths(
    nodes: &[AstNode],
    path: &mut Vec<(bool, Text)>,
    selectors: &mut Vec<Text>,
) -> Result<(), CompileError> {
    for node in nodes {
        let part = match node {
            AstNode::Rule { selector, .. } => Some((false, selector.clone())),
            AstNode::AtRule { name, params, .. } => {
                Some((true, concat(&[name, &Text::from(" "), params])))
            }
            _ => None,
        };
        let Some(part) = part else {
            continue;
        };
        path.push(part);
        let children = node.children().unwrap_or_default();
        if children.is_empty() {
            let mut parts = path
                .iter()
                .filter(|(_, value)| value != "&")
                .collect::<Vec<_>>();
            parts.sort_by_key(|(is_at, _)| !is_at);
            let mut selector = Text::default();
            for (_, part) in parts.into_iter().rev() {
                selector = if selector.is_empty() {
                    part.clone()
                } else {
                    concat(&[part, &Text::from(" { "), &selector, &Text::from(" }")])
                };
            }
            selectors.push(selector);
        } else {
            selector_paths(children, path, selectors)?;
        }
        path.pop();
    }
    Ok(())
}
fn namespace_keys(theme: &Theme, namespaces: &[&Text]) -> Result<Vec<Text>, CompileError> {
    let mut keys = Vec::new();
    for namespace in namespaces {
        let prefix = concat(&[namespace, &Text::from("-")]);
        for (name, _) in theme.ordered_tokens() {
            let ignored = match literal(namespace) {
                "--font" => name.starts_with("--font-weight") || name.starts_with("--font-size"),
                "--inset" => name.starts_with("--inset-shadow") || name.starts_with("--inset-ring"),
                "--text" => [
                    "--text-color",
                    "--text-decoration-color",
                    "--text-decoration-thickness",
                    "--text-indent",
                    "--text-shadow",
                    "--text-underline-offset",
                ]
                .iter()
                .any(|ignored| name == *ignored || name.starts_with(&format!("{ignored}-"))),
                _ => false,
            };
            if !ignored
                && !name
                    .units()
                    .get(2..)
                    .unwrap_or_default()
                    .windows(2)
                    .any(|units| units == [45, 45])
                && name.units().starts_with(prefix.units())
            {
                keys.push(legacy_numeric(&Text::from_units(
                    name.units()[prefix.units().len()..].to_vec(),
                )));
            }
        }
    }
    Ok(keys)
}
fn legacy_numeric(raw: &Text) -> Text {
    let mut units = raw.units().to_vec();
    for index in 1..units.len().saturating_sub(1) {
        if units[index] == 95
            && matches!(units[index - 1], 48..=57)
            && matches!(units[index + 1], 48..=57)
        {
            units[index] = 46;
        }
    }
    Text::from_units(units)
}
fn class_list(
    theme: &Theme,
    custom: &compiler::custom_utilities::CustomUtilities,
    limits: &Limits,
) -> Result<PublicValue, CompileError> {
    let metadata = include_str!("design_system/completions.json");
    compiler::check_limit(
        metadata.len().saturating_mul(16),
        limits.cache_bytes,
        "cache_bytes",
    )?;
    let mut definitions: PublicValue =
        serde_json::from_str(metadata).map_err(|_| syntax("Invalid fixed completion metadata"))?;
    let scratch = json_bytes(&definitions);
    compiler::check_limit(scratch, limits.cache_bytes, "cache_bytes")?;
    for (root, metadata) in custom.completion_definitions() {
        if !custom.contains_root(UtilityKind::Functional, root) {
            continue;
        }
        let group = custom_completion_group(metadata, theme)?;
        if let Some(entries) = definitions["definitions"].as_array_mut() {
            if let Some(entry) = entries.iter_mut().find(|entry| entry[0] == root) {
                *entry = public_json!([root, public_json!([group])]);
            } else {
                entries.push(public_json!([root, public_json!([group])]));
            }
        }
    }
    struct Item {
        utility: Text,
        fraction: bool,
        modifiers: Vec<Text>,
    }
    let mut items: BTreeMap<Text, Item> = BTreeMap::new();
    let mut item_bytes = 0usize;
    for root in compiler::registry::STATIC_ROOTS
        .iter()
        .copied()
        .chain(custom.roots(UtilityKind::Static))
    {
        item_bytes = item_bytes
            .saturating_add(root.len().saturating_mul(2))
            .saturating_add(160);
        compiler::check_limit(
            scratch.saturating_add(item_bytes),
            limits.cache_bytes,
            "cache_bytes",
        )?;
        items.insert(
            root.into(),
            Item {
                utility: root.into(),
                fraction: false,
                modifiers: vec![],
            },
        );
    }
    let fractions = [
        "1/2", "1/3", "2/3", "1/4", "2/4", "3/4", "1/5", "2/5", "3/5", "4/5", "1/6", "2/6", "3/6",
        "4/6", "5/6", "1/12", "2/12", "3/12", "4/12", "5/12", "6/12", "7/12", "8/12", "9/12",
        "10/12", "11/12",
    ];
    for definition in definitions["definitions"]
        .as_array()
        .ok_or_else(|| syntax("Invalid fixed completion definitions"))?
    {
        let root = definition[0]
            .as_text()
            .ok_or_else(|| syntax("Invalid completion root"))?;
        for group in definition[1]
            .as_array()
            .ok_or_else(|| syntax("Invalid completion groups"))?
        {
            let mut values = group["values"].as_array().cloned().unwrap_or_default();
            if let Some(key) = group["valuesWhen"]["key"].as_text() {
                if theme.tokens.contains_key(key) {
                    values.extend(
                        group["valuesWhen"]["values"]
                            .as_array()
                            .cloned()
                            .unwrap_or_default(),
                    );
                }
            }
            let keys = group["valueThemeKeys"]
                .as_array()
                .map(|keys| {
                    keys.iter()
                        .filter_map(PublicValue::as_text)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            values.extend(
                namespace_keys(theme, &keys)?
                    .into_iter()
                    .map(|value| public_json!(value)),
            );
            if group["supportsFractions"] == true {
                values.extend(fractions.map(|value| public_json!(value)));
            }
            if group["hasDefaultValue"] == true
                || group["defaultWhen"]
                    .as_text()
                    .is_some_and(|key| theme.tokens.contains_key(key))
            {
                values.insert(0, PublicValue::Null);
            }
            let mut modifiers = group["modifiers"]
                .as_array()
                .map(|values| {
                    values
                        .iter()
                        .filter_map(PublicValue::as_text)
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let keys = group["modifierThemeKeys"]
                .as_array()
                .map(|keys| {
                    keys.iter()
                        .filter_map(PublicValue::as_text)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            modifiers.extend(namespace_keys(theme, &keys)?);
            for value in values {
                let fraction = value.as_text().is_some_and(|value| {
                    value.split_once("/").is_some_and(|(a, b)| {
                        !a.is_empty()
                            && !b.is_empty()
                            && a.units().iter().all(|b| matches!(*b, 48..=57))
                            && b.units().iter().all(|b| matches!(*b, 48..=57))
                    })
                });
                let name = value.as_text().map_or_else(
                    || root.clone(),
                    |value| concat(&[root, &Text::from("-"), value]),
                );
                for negative in [false, true] {
                    if negative && group["supportsNegative"] != true {
                        continue;
                    }
                    let name = if negative {
                        concat(&[&Text::from("-"), &name])
                    } else {
                        name.clone()
                    };
                    let utility = if negative {
                        concat(&[&Text::from("-"), root])
                    } else {
                        root.into()
                    };
                    item_bytes = item_bytes.saturating_add(
                        modifiers
                            .iter()
                            .map(|value| value.len().saturating_add(32))
                            .sum::<usize>(),
                    );
                    if !items.contains_key(&name) {
                        item_bytes = item_bytes
                            .saturating_add(name.len())
                            .saturating_add(utility.len())
                            .saturating_add(160);
                    }
                    compiler::check_limit(
                        scratch.saturating_add(item_bytes),
                        limits.cache_bytes,
                        "cache_bytes",
                    )?;
                    let item = items.entry(name).or_insert(Item {
                        utility: utility.clone(),
                        fraction: false,
                        modifiers: vec![],
                    });
                    item.utility = utility;
                    item.fraction |= fraction;
                    item.modifiers.extend(modifiers.iter().cloned());
                }
            }
        }
    }
    let mut list = items.into_iter().collect::<Vec<_>>();
    list.sort_by(|(a, _), (b, _)| compiler::compare_classes(a, b));
    let mut buckets: Vec<(Text, Vec<PublicValue>)> = Vec::new();
    let mut last = BTreeMap::new();
    let mut deferred: BTreeMap<Text, Vec<PublicValue>> = BTreeMap::new();
    for (name, item) in list {
        if buckets
            .last()
            .is_none_or(|(utility, _)| utility != &item.utility)
        {
            buckets.push((item.utility.clone(), vec![]));
            last.insert(item.utility.clone(), buckets.len() - 1);
        }
        let options = PublicValue::Object(Arc::new(PublicObject::from([(
            Text::from("modifiers"),
            PublicValue::from(item.modifiers),
        )])));
        let value = PublicValue::Array(Arc::new(vec![PublicValue::from(name), options]));
        if item.fraction {
            deferred.entry(item.utility).or_default().push(value);
        } else {
            let index = buckets.len() - 1;
            buckets[index].1.push(value);
        }
    }
    for (utility, values) in deferred {
        if let Some(index) = last.get(&utility) {
            buckets[*index].1.extend(values);
        }
    }
    let result = PublicValue::Array(Arc::new(
        buckets.into_iter().flat_map(|(_, values)| values).collect(),
    ));
    compiler::check_limit(
        scratch.saturating_add(json_bytes(&result)),
        limits.cache_bytes,
        "cache_bytes",
    )?;
    Ok(result)
}

const SPACING_SUGGESTIONS: &[&str] = &[
    "0", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "5", "6", "7", "8", "9", "10", "11", "12",
    "14", "16", "20", "24", "28", "32", "36", "40", "44", "48", "52", "56", "60", "64", "72", "80",
    "96",
];
fn custom_completion_group(
    metadata: &compiler::custom_utilities::CompletionMetadata,
    theme: &Theme,
) -> Result<PublicValue, CompileError> {
    let mut result = [Vec::new(), Vec::new()];
    for (index, target) in [&metadata.values, &metadata.modifiers]
        .into_iter()
        .enumerate()
    {
        result[index].extend(target.literals.iter().cloned());
        if target.number {
            result[index].extend(SPACING_SUGGESTIONS.iter().map(|value| (*value).into()));
        } else if target.integer {
            result[index].extend(
                SPACING_SUGGESTIONS
                    .iter()
                    .filter(|value| !value.contains('.'))
                    .map(|value| (*value).into()),
            );
        }
        result[index].extend(namespace_keys(
            theme,
            &target.namespaces.iter().collect::<Vec<_>>(),
        )?);
    }
    Ok(public_json!({"values":result[0],"modifiers":result[1]}))
}

fn variant_rank(variant: &PublicValue, theme: &Theme) -> usize {
    let root = variant["root"].text_or_empty();
    if let Some((_, rank)) = compiler::registry::VARIANT_ORDER
        .iter()
        .find(|(name, _)| root == *name)
    {
        *rank
    } else if theme
        .tokens
        .contains_key(&concat(&[&Text::from("--breakpoint-"), root]))
    {
        64
    } else {
        compiler::registry::VARIANT_ORDER
            .iter()
            .map(|(_, rank)| *rank)
            .max()
            .unwrap_or(0)
            + 1
            + theme
                .custom_variants
                .root_names()
                .filter(|root| {
                    !compiler::registry::VARIANT_ORDER
                        .iter()
                        .any(|(name, _)| name == root)
                })
                .position(|name| root == name)
                .unwrap_or(0)
    }
}
fn breakpoint_value(variant: &PublicValue, theme: &Theme) -> Result<Option<Text>, CompileError> {
    let Some(root) = variant["root"].as_text() else {
        return Ok(None);
    };
    if variant["kind"] == "static" {
        return Ok(theme
            .tokens
            .get(&concat(&[&Text::from("--breakpoint-"), root]))
            .map(|token| token.value.clone()));
    }
    if variant["value"].is_null() || !root.starts_with("@") && !variant["modifier"].is_null() {
        return Ok(None);
    }
    let Some(value) = variant["value"]["value"].as_text() else {
        return Ok(None);
    };
    let value = if variant["value"]["kind"] == "arbitrary" {
        value.clone()
    } else {
        let key = concat(&[
            &Text::from(if root.starts_with("@") {
                "--container-"
            } else {
                "--breakpoint-"
            }),
            value,
        ]);
        let Some(token) = theme.tokens.get(&key) else {
            return Ok(None);
        };
        token.value.clone()
    };
    Ok((!value.is_empty() && !value.contains("var(")).then_some(value))
}
fn compound_child(raw: &Text) -> Text {
    // A cached compound is syntactically valid. Split its top-level modifier
    // directly, matching segment's escaped and nested UTF-16 unit semantics.
    let units = raw.units();
    let mut stack = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    let mut end = units.len();
    for (index, &unit) in units.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if unit == 92 {
            escaped = true;
            continue;
        }
        if let Some(current) = quote {
            if unit == current {
                quote = None;
            }
            continue;
        }
        match unit {
            34 | 39 => quote = Some(unit),
            40 | 91 | 123 => stack.push(match unit {
                40 => 41,
                91 => 93,
                _ => 125,
            }),
            41 | 93 | 125 if stack.last() == Some(&unit) => {
                stack.pop();
            }
            47 if stack.is_empty() => {
                end = index;
                break;
            }
            _ => {}
        }
    }
    let base = &units[..end];
    base.iter()
        .position(|unit| *unit == 45)
        .map_or_else(Text::default, |index| {
            Text::from_units(base[index + 1..].to_vec())
        })
}
fn collect_breakpoint_values(
    raw: &Text,
    variant: &PublicValue,
    theme: &Theme,
    values: &mut BTreeMap<Text, Option<Text>>,
) -> Result<(), CompileError> {
    if variant["kind"] == "compound" {
        collect_breakpoint_values(&compound_child(raw), &variant["variant"], theme, values)?;
    } else if (63..=66).contains(&variant_rank(variant, theme)) && !values.contains_key(raw) {
        values.insert(raw.clone(), breakpoint_value(variant, theme)?);
    }
    Ok(())
}
fn cached_variant_cmp(
    raw_a: &Text,
    a: &PublicValue,
    raw_b: &Text,
    b: &PublicValue,
    theme: &Theme,
    breakpoint_values: &BTreeMap<Text, Option<Text>>,
) -> Ordering {
    if raw_a == raw_b {
        return Ordering::Equal;
    }
    match (a["kind"] == "arbitrary", b["kind"] == "arbitrary") {
        (true, true) => {
            return text_cmp(a["selector"].text_or_empty(), b["selector"].text_or_empty())
        }
        (true, false) => return Ordering::Greater,
        (false, true) => return Ordering::Less,
        _ => {}
    }
    let rank = variant_rank(a, theme);
    let order = rank.cmp(&variant_rank(b, theme));
    if order != Ordering::Equal {
        return order;
    }
    if a["kind"] == "compound" && b["kind"] == "compound" {
        return cached_variant_cmp(
            &compound_child(raw_a),
            &a["variant"],
            &compound_child(raw_b),
            &b["variant"],
            theme,
            breakpoint_values,
        )
        .then_with(
            || match (a["modifier"].as_object(), b["modifier"].as_object()) {
                (Some(_), Some(_)) => text_cmp(
                    a["modifier"]["value"].text_or_empty(),
                    b["modifier"]["value"].text_or_empty(),
                ),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            },
        );
    }
    if (63..=66).contains(&rank) {
        let descending = rank == 63 || rank == 65;
        return match (
            breakpoint_values.get(raw_a).and_then(Option::as_ref),
            breakpoint_values.get(raw_b).and_then(Option::as_ref),
        ) {
            (Some(a), Some(b)) => variants::compare_breakpoints(a, b, descending),
            (None, _) => {
                if descending {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (_, None) => {
                if descending {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        };
    }
    let root = text_cmp(a["root"].text_or_empty(), b["root"].text_or_empty());
    if root != Ordering::Equal {
        return root;
    }
    match (a["value"].is_null(), b["value"].is_null()) {
        (true, _) => return Ordering::Less,
        (_, true) => return Ordering::Greater,
        _ => {}
    }
    (a["value"]["kind"] == "arbitrary")
        .cmp(&(b["value"]["kind"] == "arbitrary"))
        .then_with(|| {
            let order = text_cmp(
                a["value"]["value"].text_or_empty(),
                b["value"]["value"].text_or_empty(),
            );
            if order == Ordering::Equal {
                Ordering::Greater
            } else {
                order
            }
        })
}

#[cfg(test)]
#[path = "../../../../../tests/utility/compiler/design-system-native.rs"]
mod tests;

#[cfg(test)]
mod canonical_text_tests {
    use super::*;

    #[test]
    fn public_object_order_survives_decode_overwrite_and_roundtrip() {
        let input = r#"{"z":1,"\ud800":2,"a":3,"z":4}"#;
        let mut value: PublicValue = serde_json::from_str(input).unwrap();
        let expected = vec![
            Text::from("z"),
            Text::from_units(vec![0xd800]),
            Text::from("a"),
        ];
        assert_eq!(
            value
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<Vec<_>>(),
            expected
        );
        value["a"] = PublicValue::from(5usize);
        value["last"] = PublicValue::from(6usize);
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"{"z":4,"\ud800":2,"a":5,"last":6}"#
        );
        let decoded: PublicValue =
            serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
        assert_eq!(
            decoded.as_object().unwrap().keys().collect::<Vec<_>>(),
            value.as_object().unwrap().keys().collect::<Vec<_>>()
        );
        let original = value.owned_bytes();
        value.as_object_mut().unwrap().reserve(128);
        assert!(value.owned_bytes() > original);
    }

    #[test]
    fn public_json_preserves_lone_units_in_object_keys_and_values() {
        let json = r#"{"\ud800":"\udc00\ufffd😀","array":[null,true,7,"\ud800"]}"#;
        let object: PublicValue = serde_json::from_str(json).unwrap();
        let key = Text::from_units(vec![0xd800]);
        assert_eq!(
            object.get_text(&key).unwrap().as_text().unwrap().units(),
            &[0xdc00, 0xfffd, 0xe000, 0xd83d, 0xde00]
        );
        let encoded = serde_json::to_string(&object).unwrap();
        let decoded: PublicValue = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, object);
    }

    #[test]
    fn candidate_print_and_emission_preserve_arbitrary_value_units() {
        let mut api = load_design_system(
            "@tailwind utilities;",
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let mut raw = Text::from("[--mark:");
        raw.push_text(&Text::from_units(vec![0xd800, 0xfffd, 0xe000]));
        raw.push_str("]");
        let parsed = api.parse_candidate_text(&raw).unwrap();
        assert_eq!(api.print_candidate_text(&parsed[0]).unwrap(), raw);
        let output = api.compile_ast_nodes_text(&parsed[0], 0).unwrap();
        assert_eq!(
            output[0]["node"]["nodes"][0]["value"]
                .as_text()
                .unwrap()
                .units(),
            &[0xd800, 0xfffd, 0xe000]
        );
        let css = api.candidates_to_css_text(&[raw]).unwrap();
        assert!(css[0].as_text().unwrap().units().contains(&0xd800));
    }

    #[test]
    fn dynamic_theme_completion_names_preserve_original_units() {
        let mut css = Text::from("@theme {--color-");
        css.push_text(&Text::from_units(vec![0xd800]));
        css.push_str(":red;--breakpoint-");
        css.push_text(&Text::from_units(vec![0xdc00]));
        css.push_str(":40rem;}");
        let mut limits = crate::compiler::tests::limits();
        limits.cache_bytes = 8_000_000;
        let api = load_design_system_text(&css, CompileOptions::default(), vec![], limits).unwrap();
        let name = concat(&[&Text::from("bg-"), &Text::from_units(vec![0xd800])]);
        assert!(api
            .get_class_list_text()
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry[0].as_text() == Some(&name)));
        assert!(api
            .get_variants_text()
            .unwrap()
            .iter()
            .any(|variant| variant["name"]
                .as_text()
                .is_some_and(|name| name.units() == [0xdc00])));
    }

    #[test]
    fn reserved_cache_failure_rolls_back_new_metadata_and_vec_capacity() {
        let mut api = load_design_system(
            "",
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let initial = api.cache_checkpoint();
        let error = api
            .parse_variant_text_with_reserved_cache(&Text::from("group-hover"), usize::MAX)
            .unwrap_err();
        assert!(matches!(error,CompileError::Limit {resource,..} if resource=="cache_bytes"));
        assert_eq!(
            (
                api.candidate_order.len(),
                api.parsed_variants.len(),
                api.retained_bytes,
                api.candidate_order.capacity(),
                api.parsed_variants.capacity()
            ),
            (
                initial.candidates,
                initial.variants,
                initial.retained_bytes,
                initial.candidate_capacity,
                initial.variant_capacity
            )
        );
    }
}
