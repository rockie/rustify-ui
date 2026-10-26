use super::{Theme, ThemeConfig, ThemeConfigGroup};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::collections::BTreeMap;

pub const COLOR_TOKENS: &[&str] = &[
    "background",
    "foreground",
    "card",
    "card-foreground",
    "popover",
    "popover-foreground",
    "primary",
    "primary-foreground",
    "secondary",
    "secondary-foreground",
    "muted",
    "muted-foreground",
    "accent",
    "accent-foreground",
    "destructive",
    "destructive-foreground",
    "border",
    "input",
    "ring",
    "chart-1",
    "chart-2",
    "chart-3",
    "chart-4",
    "chart-5",
    "sidebar",
    "sidebar-foreground",
    "sidebar-primary",
    "sidebar-primary-foreground",
    "sidebar-accent",
    "sidebar-accent-foreground",
    "sidebar-border",
    "sidebar-ring",
    "success",
    "warning",
    "shadow-color",
];

pub const VALUE_TOKENS: &[&str] = &[
    "font-sans",
    "font-serif",
    "font-mono",
    "letter-spacing",
    "font-size",
    "radius",
    "spacing",
    "layout-gap",
    "shadow-opacity",
    "shadow-blur",
    "shadow-spread",
    "shadow-offset-x",
    "shadow-offset-y",
    "reduce-motion",
];

pub const COMMON_TOKENS: &[&str] = &[
    "font-sans",
    "font-serif",
    "font-mono",
    "letter-spacing",
    "font-size",
    "radius",
    "spacing",
    "layout-gap",
    "shadow-opacity",
    "shadow-blur",
    "shadow-spread",
    "shadow-offset-x",
    "shadow-offset-y",
    "reduce-motion",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Author values stay intact when preview values are resolved or clipped.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ThemeValues(pub BTreeMap<String, String>);

impl<'de> Deserialize<'de> for ThemeValues {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Values;
        impl<'de> Visitor<'de> for Values {
            type Value = ThemeValues;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an object of unique token names and string values")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut values = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    if values.insert(key.clone(), value).is_some() {
                        return Err(serde::de::Error::custom(format!("duplicate token {key}")));
                    }
                }
                Ok(ThemeValues(values))
            }
        }
        deserializer.deserialize_map(Values)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeStyles {
    pub light: ThemeValues,
    pub dark: ThemeValues,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeDocument {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub base: ThemeStyles,
    pub config: ThemeConfig,
    pub overrides: ThemeOverrides,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeOverrides {
    pub light: ThemeValuePatch,
    pub dark: ThemeValuePatch,
}

impl ThemeOverrides {
    pub fn values(&self, mode: ThemeMode) -> &ThemeValuePatch {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }

    pub fn values_mut(&mut self, mode: ThemeMode) -> &mut ThemeValuePatch {
        match mode {
            ThemeMode::Light => &mut self.light,
            ThemeMode::Dark => &mut self.dark,
        }
    }

    fn migrated(base: &ThemeStyles) -> Self {
        let patch = |values: &ThemeValues| {
            ThemeValuePatch(
                ThemeConfigGroup::ALL
                    .iter()
                    .flat_map(|group| group.tokens().iter().copied())
                    .filter_map(|token| {
                        values
                            .0
                            .get(token)
                            .map(|value| (token.to_owned(), value.clone()))
                    })
                    .collect(),
            )
        };
        Self {
            light: patch(&base.light),
            dark: patch(&base.dark),
        }
    }
}

// Missing and explicitly null structural fields are different. A null field
// must fail before it could be mistaken for the compact wire form's absence.
macro_rules! structural_field {
    ($function:ident, $ty:ty, $field:literal) => {
        fn $function<'de, D: Deserializer<'de>>(de: D) -> Result<Option<$ty>, D::Error> {
            <$ty>::deserialize(de)
                .map(Some)
                .map_err(|error| serde::de::Error::custom(format!("{}: {error}", $field)))
        }
    };
}
structural_field!(styles_field, ThemeStyles, "styles");
structural_field!(base_field, ThemeStyles, "base");
structural_field!(config_field, ThemeConfig, "config");
structural_field!(overrides_field, ThemeOverrides, "overrides");

impl<'de> Deserialize<'de> for ThemeDocument {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            schema_version: u32,
            id: String,
            name: String,
            #[serde(default, deserialize_with = "styles_field")]
            styles: Option<ThemeStyles>,
            #[serde(default, deserialize_with = "base_field")]
            base: Option<ThemeStyles>,
            #[serde(default, deserialize_with = "config_field")]
            config: Option<ThemeConfig>,
            #[serde(default, deserialize_with = "overrides_field")]
            overrides: Option<ThemeOverrides>,
        }
        let wire = Wire::deserialize(de)?;
        if !matches!(wire.schema_version, 1 | 2) {
            return Err(serde::de::Error::custom(
                "schema_version: unsupported theme schema version",
            ));
        }
        match (wire.styles, wire.base, wire.config, wire.overrides) {
            (Some(styles), None, None, None) => Ok(Self::custom(wire.id, wire.name, styles)),
            (None, Some(base), Some(config), Some(overrides)) if wire.schema_version == 2 => Ok(Self {
                schema_version: 2, id: wire.id, name: wire.name, base, config, overrides,
            }),
            _ => Err(serde::de::Error::custom("use styles alone for v1/compact Custom v2, or base + config + overrides for full v2")),
        }
    }
}

impl Serialize for ThemeDocument {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.config == ThemeConfig::custom()
            && self.overrides == ThemeOverrides::migrated(&self.base)
        {
            #[derive(Serialize)]
            struct Compact<'a> {
                schema_version: u32,
                id: &'a str,
                name: &'a str,
                styles: &'a ThemeStyles,
            }
            Compact {
                schema_version: self.schema_version,
                id: &self.id,
                name: &self.name,
                styles: &self.base,
            }
            .serialize(serializer)
        } else {
            #[derive(Serialize)]
            struct Full<'a> {
                schema_version: u32,
                id: &'a str,
                name: &'a str,
                base: &'a ThemeStyles,
                config: ThemeConfig,
                overrides: &'a ThemeOverrides,
            }
            Full {
                schema_version: self.schema_version,
                id: &self.id,
                name: &self.name,
                base: &self.base,
                config: self.config,
                overrides: &self.overrides,
            }
            .serialize(serializer)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeError {
    pub field: String,
    pub message: String,
}

impl ThemeError {
    pub(crate) fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ThemeError {}

/// A sparse set of explicit values. Removing a key reveals configuration/base.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ThemeValuePatch(pub BTreeMap<String, String>);

impl<'de> Deserialize<'de> for ThemeValuePatch {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let values = ThemeValues::deserialize(de)?;
        for token in values.0.keys() {
            if !COLOR_TOKENS.contains(&token.as_str()) && !VALUE_TOKENS.contains(&token.as_str()) {
                return Err(serde::de::Error::custom(format!(
                    "unknown override token {token}"
                )));
            }
        }
        Ok(Self(values.0))
    }
}

impl ThemeValues {
    fn from_legacy(theme: Theme) -> Self {
        let mut values: BTreeMap<String, String> = theme
            .properties()
            .into_iter()
            .take(19)
            .map(|(key, value)| (key[2..].to_owned(), value))
            .collect();
        for (target, source) in [
            ("card-foreground", "foreground"),
            ("popover-foreground", "foreground"),
            ("sidebar", "background"),
            ("sidebar-foreground", "foreground"),
            ("sidebar-primary", "primary"),
            ("sidebar-primary-foreground", "primary-foreground"),
            ("sidebar-accent", "secondary"),
            ("sidebar-accent-foreground", "secondary-foreground"),
            ("sidebar-border", "border"),
            ("sidebar-ring", "ring"),
            ("chart-1", "primary"),
            ("chart-2", "success"),
            ("chart-3", "warning"),
            ("chart-4", "destructive"),
            ("chart-5", "muted-foreground"),
        ] {
            values.insert(target.into(), values[source].clone());
        }
        for (key, value) in [
            ("font-sans", "IBM Plex Sans, sans-serif"),
            ("font-serif", "ui-serif, serif"),
            ("font-mono", "Liberation Mono, monospace"),
            ("letter-spacing", "0em"),
            ("spacing", "0.25rem"),
            ("shadow-color", "#000000"),
            ("shadow-opacity", "0"),
            ("shadow-blur", "0px"),
            ("shadow-spread", "0px"),
            ("shadow-offset-x", "0px"),
            ("shadow-offset-y", "0px"),
        ] {
            values.insert(key.into(), value.into());
        }
        values.insert("radius".into(), format!("{}px", theme.radius));
        values.insert("font-size".into(), format!("{}px", theme.font_size));
        values.insert("layout-gap".into(), format!("{}px", theme.spacing));
        values.insert("reduce-motion".into(), theme.reduce_motion.to_string());
        Self(values)
    }

    pub fn get(&self, token: &str) -> Result<&str, ThemeError> {
        self.0
            .get(token)
            .map(String::as_str)
            .ok_or_else(|| ThemeError::new(token, "required token is missing"))
    }

    pub fn patched(&self, patch: &ThemeValuePatch) -> Result<Self, ThemeError> {
        let mut values = self.clone();
        for (token, value) in &patch.0 {
            if !COLOR_TOKENS.contains(&token.as_str()) && !VALUE_TOKENS.contains(&token.as_str()) {
                return Err(ThemeError::new(token, "unknown patch token"));
            }
            values.0.insert(token.clone(), value.clone());
        }
        Ok(values)
    }
}

impl ThemeDocument {
    pub fn from_legacy(light: Theme, dark: Theme) -> Self {
        Self::custom(
            "rustify-default",
            "Rustify Default",
            ThemeStyles {
                light: ThemeValues::from_legacy(light),
                dark: ThemeValues::from_legacy(dark),
            },
        )
    }

    /// Imports complete authored tables without assigning a brand or erasing them later.
    pub fn custom(id: impl Into<String>, name: impl Into<String>, base: ThemeStyles) -> Self {
        let overrides = ThemeOverrides::migrated(&base);
        Self {
            schema_version: 2,
            id: id.into(),
            name: name.into(),
            base,
            config: ThemeConfig::custom(),
            overrides,
        }
    }

    /// Creates an explicitly configured theme with no semantic overrides.
    pub fn standard(id: impl Into<String>, name: impl Into<String>) -> Self {
        let mut document = Self::from_legacy(Theme::light(), Theme::dark());
        document.id = id.into();
        document.name = name.into();
        document.config = ThemeConfig::standard();
        document.overrides = ThemeOverrides::default();
        document
    }

    /// The complete, unscaled author base, before configuration and overrides.
    pub fn base_values(&self, mode: ThemeMode) -> &ThemeValues {
        match mode {
            ThemeMode::Light => &self.base.light,
            ThemeMode::Dark => &self.base.dark,
        }
    }

    /// Mutating a base does not clear or silently rewrite explicit overrides.
    pub fn base_values_mut(&mut self, mode: ThemeMode) -> &mut ThemeValues {
        match mode {
            ThemeMode::Light => &mut self.base.light,
            ThemeMode::Dark => &mut self.base.dark,
        }
    }

    /// Config/base/overrides composed as unscaled author strings. `resolve`
    /// additionally validates values and derives CSS/GPU geometry and colors.
    pub fn effective_values(&self, mode: ThemeMode) -> Result<ThemeValues, ThemeError> {
        super::ThemeSource::document_values(self, mode)
    }

    pub fn validate_identity(&self) -> Result<(), ThemeError> {
        if self.schema_version != 2 {
            return Err(ThemeError::new(
                "schema_version",
                "unsupported theme schema version",
            ));
        }
        for (field, value) in [("id", &self.id), ("name", &self.name)] {
            if value.trim().is_empty()
                || value.chars().count() > 80
                || value.chars().any(char::is_control)
            {
                return Err(ThemeError::new(
                    field,
                    "use 1–80 characters without control characters",
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_documents_upgrade_to_the_new_author_schema() {
        let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        assert_eq!(document.schema_version, 2);
    }

    #[test]
    fn legacy_conversion_keeps_colors_and_separates_unit_from_layout_gap() {
        let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for (mode, legacy) in [
            (ThemeMode::Light, Theme::light()),
            (ThemeMode::Dark, Theme::dark()),
        ] {
            let values = document.base_values(mode);
            for (property, value) in legacy.properties().into_iter().take(19) {
                assert_eq!(values.get(&property[2..]).unwrap(), value);
            }
            assert_eq!(values.get("radius").unwrap(), "6px");
            assert_eq!(values.get("spacing").unwrap(), "0.25rem");
            assert_eq!(values.get("layout-gap").unwrap(), "8px");
            assert_eq!(values.get("shadow-opacity").unwrap(), "0");
            assert_eq!(
                values.get("card-foreground").unwrap(),
                values.get("foreground").unwrap()
            );
            for token in COLOR_TOKENS.iter().chain(VALUE_TOKENS) {
                assert!(values.get(token).is_ok(), "{token}");
            }
        }
    }

    #[test]
    fn identity_rejects_unknown_version_and_invalid_names() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        assert!(document.validate_identity().is_ok());
        document.schema_version = 3;
        assert_eq!(
            document.validate_identity().unwrap_err().field,
            "schema_version"
        );
        document.schema_version = 2;
        for name in ["", "   ", "bad\nname", &"中".repeat(81)] {
            document.name = name.into();
            assert_eq!(document.validate_identity().unwrap_err().field, "name");
        }
        document.name = "中".repeat(80);
        assert!(document.validate_identity().is_ok());
    }

    #[test]
    fn json_preserves_author_values_and_requires_both_modes_and_string_tokens() {
        let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        let json = serde_json::to_string(&document).unwrap();
        assert_eq!(
            serde_json::from_str::<ThemeDocument>(&json).unwrap(),
            document
        );
        let mut value = serde_json::to_value(&document).unwrap();
        value["styles"]["light"]["primary"] = serde_json::json!(42);
        assert!(serde_json::from_value::<ThemeDocument>(value).is_err());
        let mut value = serde_json::to_value(&document).unwrap();
        value["styles"].as_object_mut().unwrap().remove("dark");
        assert!(serde_json::from_value::<ThemeDocument>(value).is_err());
    }

    #[test]
    fn patch_rejects_unknown_tokens_without_mutating_the_source() {
        let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        let patch = ThemeValuePatch(BTreeMap::from([("unexpected".into(), "#fff".into())]));
        assert!(document.base.light.patched(&patch).is_err());
        assert!(!document.base.light.0.contains_key("unexpected"));
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn published_schema_requires_the_same_tokens_as_the_rust_model() {
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../../../../docs/theme-document.schema.json"))
                .unwrap();
        let required: Vec<_> = schema["$defs"]["values"]["required"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert_eq!(
            required,
            COLOR_TOKENS
                .iter()
                .chain(VALUE_TOKENS)
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(schema["properties"]["schema_version"]["const"], 2);
        for token in required {
            assert_eq!(
                schema["$defs"]["values"]["properties"][token]["type"],
                "string"
            );
        }
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn published_schema_configuration_and_sparse_overrides_match_the_typed_model() {
        fn encoded<T: Serialize>(values: &[T]) -> Vec<serde_json::Value> {
            values
                .iter()
                .map(|value| serde_json::to_value(value).unwrap())
                .collect()
        }
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../../../../docs/theme-document.schema.json"))
                .unwrap();
        let config = &schema["$defs"]["config"];
        for (field, mut values) in [
            ("accent_color", encoded(super::super::AccentColor::ALL)),
            ("gray_color", encoded(super::super::GrayColor::ALL)),
            ("radius", encoded(super::super::ThemeRadius::ALL)),
            (
                "panel_background",
                encoded(&[
                    super::super::PanelBackground::Solid,
                    super::super::PanelBackground::Translucent,
                ]),
            ),
        ] {
            values.insert(0, serde_json::Value::Null);
            assert_eq!(
                config["properties"][field]["enum"],
                serde_json::json!(values),
                "{field}"
            );
        }
        assert_eq!(
            config["properties"]["scaling"]["enum"],
            serde_json::json!(encoded(super::super::ThemeScaling::ALL))
        );
        assert_eq!(config["additionalProperties"], false);
        let patch = &schema["$defs"]["patch"];
        assert_eq!(patch["additionalProperties"], false);
        let mut actual: Vec<_> = patch["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let mut expected: Vec<_> = COLOR_TOKENS.iter().chain(VALUE_TOKENS).copied().collect();
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
