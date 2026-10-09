//! Conditional class joining and Tailwind class conflict resolution.

pub mod build;
pub mod class_variants;
pub mod compiler;
pub mod config;
#[path = "../../upstream/cn/config.rs"]
mod default_config;
#[path = "../../upstream/cn/tables.rs"]
mod default_tables;
mod emit;
mod engine;
mod instance;
mod join;
pub mod validators;

use std::sync::OnceLock;

pub use crate::class_variants;
pub use class_variants::IntoTailwindClass;
pub use config::{
    from_theme, merge_configs, ClassGroupDef, Config, ConfigExtension, ConfigGroups, OrderedMap,
    ThemeGetter, Utf16ClassGroupDef, Utf16Config, Utf16Map, Utf16ThemeGetter, Utf16Validator,
    Validator,
};
pub use emit::{EmitLanguage, EmitOptions};
pub use engine::{Tables, Utf16Tables};
pub use instance::{Cn, EngineOptions};
pub use join::{clsx, lite, tw_join, ClassValue};

/// A configuration or callback error, carrying the original observable message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CnError {
    pub message: String,
}

impl CnError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
        Self::new(format!("cn: invalid config: {}", message.into()))
    }
}

impl std::fmt::Display for CnError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for CnError {}

/// Merge already-joined class strings with the fixed default configuration.
pub fn merge(inputs: &[&str]) -> String {
    static ENGINE: OnceLock<engine::TableEngine> = OnceLock::new();
    let engine = ENGINE.get_or_init(|| engine::TableEngine::new(default_tables::tables()));
    match engine.merge(&join_text(inputs), None, |opcode, value| {
        Ok::<bool, CnError>(validators::test(opcode, value))
    }) {
        Ok(output) => output,
        Err(error) => panic!("{error}"),
    }
}

/// Join nonempty class fragments without resolving conflicts.
pub fn join_text(inputs: &[&str]) -> String {
    inputs
        .iter()
        .copied()
        .filter(|input| !input.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests;
