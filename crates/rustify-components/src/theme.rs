//! The shared author theme and its controlled configuration panel.
//!
//! Browser roots use the SDK's one scope owner. Static renderers bind their
//! externally generated theme CSS with [`provide_static_theme`].

pub use crate::theme_panel::{ThemeOverrideCounts, ThemePanel};
pub use rustify_ui::{
    provide_static_theme, use_resolved_theme, AccentColor, Appearance, ConfigChange,
    EffectiveThemeConfig, GrayColor, PanelBackground, ThemeBoundaryPatch, ThemeConfig,
    ThemeConfigGroup, ThemeConfigPatch, ThemeDocument, ThemeMode, ThemeRadius, ThemeScaling,
    ThemeSource,
};
#[cfg(target_arch = "wasm32")]
pub use rustify_ui::{ThemeBoundary, ThemeScope as Theme};
