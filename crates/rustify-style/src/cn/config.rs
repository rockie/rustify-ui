use std::rc::Rc;

use serde_json::Value;

use super::{validators, CnError};

/// Ordered string-keyed configuration map.
pub type OrderedMap<T> = Vec<(String, T)>;
/// A fallible validator callback. Its error propagates from the merge call.
pub type Validator = Rc<dyn Fn(&str) -> Result<bool, CnError>>;
/// A fallible theme getter, distinguished from validator callbacks.
pub type ThemeGetter =
    Rc<dyn Fn(&OrderedMap<Vec<ClassGroupDef>>) -> Result<Vec<ClassGroupDef>, CnError>>;

/// A declarative class-group member or a native callback.
#[derive(Clone)]
pub enum ClassGroupDef {
    Literal(String),
    Object(OrderedMap<Vec<ClassGroupDef>>),
    Builtin(usize),
    Theme(String),
    Validator(Validator),
    ThemeGetter(ThemeGetter),
}

impl ClassGroupDef {
    /// Parse the marker-form configuration used by the fixed upstream.
    pub fn from_json(value: &Value) -> Result<Self, CnError> {
        if let Some(text) = value.as_str() {
            return Ok(Self::Literal(text.to_owned()));
        }
        let object = value.as_object().ok_or_else(|| {
            CnError::invalid_config("class definition must be a string or object")
        })?;
        if object.len() == 1 {
            if let Some(name) = object.get("$v").and_then(Value::as_str) {
                let opcode = validators::NAMES
                    .iter()
                    .position(|known| *known == name)
                    .ok_or_else(|| CnError::new(format!("cn: unknown validator \"{name}\"")))?;
                return Ok(Self::Builtin(opcode));
            }
            if let Some(name) = object.get("$t").and_then(Value::as_str) {
                return Ok(Self::Theme(name.to_owned()));
            }
        }
        Ok(Self::Object(
            object
                .iter()
                .map(|(key, value)| Ok((key.clone(), definitions(value, false)?)))
                .collect::<Result<_, CnError>>()?,
        ))
    }
}

/// The complete cn configuration. Definition and validator order is significant.
#[derive(Clone)]
pub struct Config {
    pub theme: OrderedMap<Vec<ClassGroupDef>>,
    pub class_groups: OrderedMap<Vec<ClassGroupDef>>,
    pub conflicting_class_groups: OrderedMap<Vec<String>>,
    pub conflicting_class_group_modifiers: OrderedMap<Vec<String>>,
    pub order_sensitive_modifiers: Vec<String>,
    pub postfix_lookup_class_groups: Vec<String>,
    pub prefix: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        super::default_config::config()
    }
}

impl Config {
    /// Parse a complete JSON-safe configuration, preserving map order.
    pub fn from_json(value: &Value) -> Result<Self, CnError> {
        Ok(Self {
            theme: group_map(required(value, "theme")?, false)?,
            class_groups: group_map(required(value, "classGroups")?, false)?,
            conflicting_class_groups: string_map(required(value, "conflictingClassGroups")?)?,
            conflicting_class_group_modifiers: string_map(required(
                value,
                "conflictingClassGroupModifiers",
            )?)?,
            order_sensitive_modifiers: strings(required(value, "orderSensitiveModifiers")?)?,
            postfix_lookup_class_groups: value
                .get("postfixLookupClassGroups")
                .map(strings)
                .transpose()?
                .unwrap_or_default(),
            prefix: optional_string(value, "prefix")?,
        })
    }
}

/// Fields accepted by both the override and extend operations.
#[derive(Clone, Default)]
pub struct ConfigGroups {
    pub theme: OrderedMap<Vec<ClassGroupDef>>,
    pub class_groups: OrderedMap<Vec<ClassGroupDef>>,
    pub conflicting_class_groups: OrderedMap<Vec<String>>,
    pub conflicting_class_group_modifiers: OrderedMap<Vec<String>>,
    pub order_sensitive_modifiers: Option<Vec<String>>,
}

/// Tailwind-merge-shaped configuration extension.
#[derive(Clone, Default)]
pub struct ConfigExtension {
    pub prefix: Option<String>,
    pub cache_size: Option<usize>,
    pub override_groups: ConfigGroups,
    pub extend: ConfigGroups,
}

impl ConfigExtension {
    pub fn from_json(value: &Value) -> Result<Self, CnError> {
        if !value.is_object() {
            return Err(CnError::invalid_config("extension must be an object"));
        }
        let cache_size = value
            .get("cacheSize")
            .map(|value| {
                value
                    .as_u64()
                    .and_then(|size| usize::try_from(size).ok())
                    .ok_or_else(|| {
                        CnError::invalid_config("cacheSize must be a nonnegative integer")
                    })
            })
            .transpose()?;
        Ok(Self {
            prefix: optional_string(value, "prefix")?,
            cache_size,
            override_groups: value
                .get("override")
                .map(groups)
                .transpose()?
                .unwrap_or_default(),
            extend: value
                .get("extend")
                .map(groups)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}

fn groups(value: &Value) -> Result<ConfigGroups, CnError> {
    if !value.is_object() {
        return Err(CnError::invalid_config(
            "config group extension must be an object",
        ));
    }
    Ok(ConfigGroups {
        theme: value
            .get("theme")
            .map(|value| group_map(value, true))
            .transpose()?
            .unwrap_or_default(),
        class_groups: value
            .get("classGroups")
            .map(|value| group_map(value, true))
            .transpose()?
            .unwrap_or_default(),
        conflicting_class_groups: value
            .get("conflictingClassGroups")
            .map(string_map)
            .transpose()?
            .unwrap_or_default(),
        conflicting_class_group_modifiers: value
            .get("conflictingClassGroupModifiers")
            .map(string_map)
            .transpose()?
            .unwrap_or_default(),
        order_sensitive_modifiers: value
            .get("orderSensitiveModifiers")
            .map(strings)
            .transpose()?,
    })
}

fn required<'a>(value: &'a Value, key: &str) -> Result<&'a Value, CnError> {
    value
        .get(key)
        .ok_or_else(|| CnError::invalid_config(format!("missing {key}")))
}

fn optional_string(value: &Value, key: &str) -> Result<Option<String>, CnError> {
    value
        .get(key)
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| CnError::invalid_config(format!("{key} must be a string")))
        })
        .transpose()
}

fn strings(value: &Value) -> Result<Vec<String>, CnError> {
    value
        .as_array()
        .ok_or_else(|| CnError::invalid_config("expected string array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| CnError::invalid_config("expected string array member"))
        })
        .collect()
}

fn string_map(value: &Value) -> Result<OrderedMap<Vec<String>>, CnError> {
    value
        .as_object()
        .ok_or_else(|| CnError::invalid_config("expected string-array map"))?
        .iter()
        .map(|(key, value)| Ok((key.clone(), strings(value)?)))
        .collect()
}

fn definitions(value: &Value, single: bool) -> Result<Vec<ClassGroupDef>, CnError> {
    if let Some(values) = value.as_array() {
        values.iter().map(ClassGroupDef::from_json).collect()
    } else if single {
        Ok(vec![ClassGroupDef::from_json(value)?])
    } else {
        Err(CnError::invalid_config("class group must be an array"))
    }
}

fn group_map(value: &Value, single: bool) -> Result<OrderedMap<Vec<ClassGroupDef>>, CnError> {
    value
        .as_object()
        .ok_or_else(|| CnError::invalid_config("expected class-group map"))?
        .iter()
        .map(|(key, value)| Ok((key.clone(), definitions(value, single)?)))
        .collect()
}

fn apply_map<T: Clone>(target: &mut OrderedMap<Vec<T>>, source: &OrderedMap<Vec<T>>, append: bool) {
    for (key, values) in source {
        if let Some((_, existing)) = target.iter_mut().find(|(name, _)| name == key) {
            if append {
                existing.extend(values.iter().cloned());
            } else {
                *existing = values.clone();
            }
        } else {
            target.push((key.clone(), values.clone()));
        }
    }
}

/// Clone the base, apply overrides, and append extensions in upstream order.
pub fn merge_configs(base: &Config, extension: &ConfigExtension) -> Config {
    let mut config = base.clone();
    if extension.prefix.is_some() {
        config.prefix = extension.prefix.clone();
    }
    for (groups, append) in [
        (&extension.override_groups, false),
        (&extension.extend, true),
    ] {
        apply_map(&mut config.theme, &groups.theme, append);
        apply_map(&mut config.class_groups, &groups.class_groups, append);
        apply_map(
            &mut config.conflicting_class_groups,
            &groups.conflicting_class_groups,
            append,
        );
        apply_map(
            &mut config.conflicting_class_group_modifiers,
            &groups.conflicting_class_group_modifiers,
            append,
        );
        if let Some(modifiers) = &groups.order_sensitive_modifiers {
            if append {
                config
                    .order_sensitive_modifiers
                    .extend(modifiers.iter().cloned());
            } else {
                config.order_sensitive_modifiers = modifiers.clone();
            }
        }
    }
    config
}

/// Refer to a theme scale in a class definition.
pub fn from_theme(key: impl Into<String>) -> ClassGroupDef {
    ClassGroupDef::Theme(key.into())
}

/// Lossless validator used by ECMAScript facades.
pub type Utf16Validator = Rc<dyn Fn(&[u16]) -> Result<bool, CnError>>;
pub type Utf16ThemeGetter =
    Rc<dyn Fn(&Utf16Map<Vec<Utf16ClassGroupDef>>) -> Result<Vec<Utf16ClassGroupDef>, CnError>>;
pub type Utf16Map<T> = Vec<(Vec<u16>, T)>;
pub type Utf16Deferred = Rc<dyn Fn() -> Result<Utf16ClassGroupDef, CnError>>;
pub type Utf16DefinitionIterator = Box<dyn Iterator<Item = Result<Utf16ClassGroupDef, CnError>>>;
pub type Utf16Sequence = Rc<dyn Fn() -> Result<Utf16DefinitionIterator, CnError>>;

#[derive(Clone)]
pub enum Utf16ClassGroupDef {
    Literal(Vec<u16>),
    Object(Utf16Map<Vec<Utf16ClassGroupDef>>),
    Builtin(usize),
    Theme(Vec<u16>),
    Validator(Utf16Validator),
    ThemeGetter(Utf16ThemeGetter),
    Deferred(Utf16Deferred),
    Sequence(Utf16Sequence),
}

#[derive(Clone)]
pub struct Utf16Config {
    pub theme: Utf16Map<Vec<Utf16ClassGroupDef>>,
    pub class_groups: Utf16Map<Vec<Utf16ClassGroupDef>>,
    pub conflicting_class_groups: Utf16Map<Vec<Vec<u16>>>,
    pub conflicting_class_group_modifiers: Utf16Map<Vec<Vec<u16>>>,
    pub order_sensitive_modifiers: Vec<Vec<u16>>,
    pub postfix_lookup_class_groups: Vec<Vec<u16>>,
    pub prefix: Option<Vec<u16>>,
}

impl From<ClassGroupDef> for Utf16ClassGroupDef {
    fn from(definition: ClassGroupDef) -> Self {
        native_definition(definition, &mut Vec::new())
    }
}
fn native_definition(
    definition: ClassGroupDef,
    callbacks: &mut Vec<(Validator, Utf16Validator)>,
) -> Utf16ClassGroupDef {
    match definition {
        ClassGroupDef::Literal(value) => {
            Utf16ClassGroupDef::Literal(value.encode_utf16().collect())
        }
        ClassGroupDef::Object(entries) => {
            Utf16ClassGroupDef::Object(native_groups(entries, callbacks))
        }
        ClassGroupDef::Builtin(op) => Utf16ClassGroupDef::Builtin(op),
        ClassGroupDef::Theme(value) => Utf16ClassGroupDef::Theme(value.encode_utf16().collect()),
        ClassGroupDef::Validator(callback) => {
            if let Some((_, converted)) = callbacks
                .iter()
                .find(|(known, _)| Rc::ptr_eq(known, &callback))
            {
                return Utf16ClassGroupDef::Validator(converted.clone());
            }
            let original = callback.clone();
            let converted: Utf16Validator = Rc::new(move |units| {
                let value = String::from_utf16(units).map_err(|_| {
                    CnError::invalid_config("native callback requires valid Unicode")
                })?;
                callback(&value)
            });
            callbacks.push((original, converted.clone()));
            Utf16ClassGroupDef::Validator(converted)
        }
        ClassGroupDef::ThemeGetter(callback) => {
            Utf16ClassGroupDef::ThemeGetter(Rc::new(move |theme| {
                let theme = from_utf16_groups(theme)?;
                callback(&theme).map(|defs| defs.into_iter().map(Into::into).collect())
            }))
        }
    }
}
fn native_groups(
    entries: OrderedMap<Vec<ClassGroupDef>>,
    callbacks: &mut Vec<(Validator, Utf16Validator)>,
) -> Utf16Map<Vec<Utf16ClassGroupDef>> {
    entries
        .into_iter()
        .map(|(key, defs)| {
            (
                key.encode_utf16().collect(),
                defs.into_iter()
                    .map(|def| native_definition(def, callbacks))
                    .collect(),
            )
        })
        .collect()
}
fn to_utf16_groups(entries: OrderedMap<Vec<ClassGroupDef>>) -> Utf16Map<Vec<Utf16ClassGroupDef>> {
    native_groups(entries, &mut Vec::new())
}

fn from_utf16_groups(
    entries: &Utf16Map<Vec<Utf16ClassGroupDef>>,
) -> Result<OrderedMap<Vec<ClassGroupDef>>, CnError> {
    fn text(units: &[u16]) -> Result<String, CnError> {
        String::from_utf16(units)
            .map_err(|_| CnError::invalid_config("native theme getter requires valid Unicode"))
    }
    fn definition(value: &Utf16ClassGroupDef) -> Result<ClassGroupDef, CnError> {
        Ok(match value {
            Utf16ClassGroupDef::Literal(units) => ClassGroupDef::Literal(text(units)?),
            Utf16ClassGroupDef::Object(entries) => {
                ClassGroupDef::Object(from_utf16_groups(entries)?)
            }
            Utf16ClassGroupDef::Builtin(op) => ClassGroupDef::Builtin(*op),
            Utf16ClassGroupDef::Theme(units) => ClassGroupDef::Theme(text(units)?),
            Utf16ClassGroupDef::Validator(callback) => {
                let callback = callback.clone();
                ClassGroupDef::Validator(Rc::new(move |value| {
                    callback(&value.encode_utf16().collect::<Vec<_>>())
                }))
            }
            Utf16ClassGroupDef::Deferred(_) | Utf16ClassGroupDef::Sequence(_) => {
                return Err(CnError::new(
                    "cn: lazy JS definitions cannot cross the native config adapter",
                ))
            }
            Utf16ClassGroupDef::ThemeGetter(callback) => {
                let callback = callback.clone();
                ClassGroupDef::ThemeGetter(Rc::new(move |theme| {
                    let theme = to_utf16_groups(theme.clone());
                    callback(&theme)?.iter().map(definition).collect()
                }))
            }
        })
    }
    entries
        .iter()
        .map(|(key, defs)| {
            Ok((
                text(key)?,
                defs.iter().map(definition).collect::<Result<_, _>>()?,
            ))
        })
        .collect()
}

impl From<Config> for Utf16Config {
    fn from(config: Config) -> Self {
        let map = |entries: OrderedMap<Vec<String>>| {
            entries
                .into_iter()
                .map(|(key, values)| {
                    (
                        key.encode_utf16().collect(),
                        values
                            .into_iter()
                            .map(|s| s.encode_utf16().collect())
                            .collect(),
                    )
                })
                .collect()
        };
        let mut callbacks = Vec::new();
        Self {
            theme: native_groups(config.theme, &mut callbacks),
            class_groups: native_groups(config.class_groups, &mut callbacks),
            conflicting_class_groups: map(config.conflicting_class_groups),
            conflicting_class_group_modifiers: map(config.conflicting_class_group_modifiers),
            order_sensitive_modifiers: config
                .order_sensitive_modifiers
                .into_iter()
                .map(|s| s.encode_utf16().collect())
                .collect(),
            postfix_lookup_class_groups: config
                .postfix_lookup_class_groups
                .into_iter()
                .map(|s| s.encode_utf16().collect())
                .collect(),
            prefix: config.prefix.map(|s| s.encode_utf16().collect()),
        }
    }
}
