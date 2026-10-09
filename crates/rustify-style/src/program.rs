//! Renderer-independent declarations and conditions emitted during compilation.

use crate::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GpuCondition {
    Hover,
    Active,
    Focus,
    FocusVisible,
    Disabled,
    Dark,
    Selector { selector: Text },
    Media { query: Text },
    Unsupported { feature: String },
}

impl<'de> Deserialize<'de> for GpuCondition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Hover,
            Active,
            Focus,
            FocusVisible,
            Disabled,
            Dark,
            Selector,
            Media,
            Unsupported,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            selector: Option<Text>,
            query: Option<Text>,
            feature: Option<String>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Hover => Self::Hover,
            Kind::Active => Self::Active,
            Kind::Focus => Self::Focus,
            Kind::FocusVisible => Self::FocusVisible,
            Kind::Disabled => Self::Disabled,
            Kind::Dark => Self::Dark,
            Kind::Selector => Self::Selector {
                selector: required(fields.selector, "selector")?,
            },
            Kind::Media => Self::Media {
                query: required(fields.query, "query")?,
            },
            Kind::Unsupported => Self::Unsupported {
                feature: required(fields.feature, "feature")?,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GpuValue {
    Pixels {
        value: String,
    },
    CssLength {
        value: String,
    },
    Color {
        value: String,
    },
    Number {
        value: String,
    },
    Keyword {
        value: String,
    },
    ThemeVariable {
        name: String,
        fallback: Option<String>,
    },
    CssExpression {
        value: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpuDeclaration {
    pub property: String,
    pub value: GpuValue,
    pub important: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpuProgram {
    pub candidate: Text,
    pub conditions: Vec<GpuCondition>,
    pub declarations: Vec<GpuDeclaration>,
    pub cascade_order: usize,
}
