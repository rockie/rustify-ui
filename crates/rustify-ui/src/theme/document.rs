use super::Theme;
use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThemeValues(pub BTreeMap<String, String>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeStyles {
    pub light: ThemeValues,
    pub dark: ThemeValues,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeDocument {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub styles: ThemeStyles,
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

/// A sparse set of author values. A boundary keeps the enclosing mode.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThemeValuePatch(pub BTreeMap<String, String>);

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
        Self {
            schema_version: 1,
            id: "rustify-default".into(),
            name: "Rustify Default".into(),
            styles: ThemeStyles {
                light: ThemeValues::from_legacy(light),
                dark: ThemeValues::from_legacy(dark),
            },
        }
    }

    pub fn values(&self, mode: ThemeMode) -> &ThemeValues {
        match mode {
            ThemeMode::Light => &self.styles.light,
            ThemeMode::Dark => &self.styles.dark,
        }
    }

    pub fn values_mut(&mut self, mode: ThemeMode) -> &mut ThemeValues {
        match mode {
            ThemeMode::Light => &mut self.styles.light,
            ThemeMode::Dark => &mut self.styles.dark,
        }
    }

    pub fn validate_identity(&self) -> Result<(), ThemeError> {
        if self.schema_version != 1 {
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
    fn legacy_conversion_keeps_colors_and_separates_unit_from_layout_gap() {
        let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for (mode, legacy) in [
            (ThemeMode::Light, Theme::light()),
            (ThemeMode::Dark, Theme::dark()),
        ] {
            let values = document.values(mode);
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
        document.schema_version = 2;
        assert_eq!(
            document.validate_identity().unwrap_err().field,
            "schema_version"
        );
        document.schema_version = 1;
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
        assert!(document.styles.light.patched(&patch).is_err());
        assert!(!document.styles.light.0.contains_key("unexpected"));
    }

    #[test]
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
        assert_eq!(schema["properties"]["schema_version"]["const"], 1);
        for token in required {
            assert_eq!(
                schema["$defs"]["values"]["properties"][token]["type"],
                "string"
            );
        }
    }
}
