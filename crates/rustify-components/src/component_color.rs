//! Component-local palette roles. Unset colors leave semantic tokens intact.

use crate::geometry::{PropError, StyleSet};
use leptos::prelude::*;
use rustify_ui::theme::{format_color, ColorFormat};
use rustify_ui::{AccentColor, ResolvedPalette, ThemeMode};

pub(crate) fn primary_is_generated(theme: &rustify_ui::ResolvedTheme) -> bool {
    use rustify_ui::{ThemeConfigGroup, ThemeValueOrigin};
    matches!(
        theme.origins.get("primary"),
        Some(ThemeValueOrigin::Configuration(ThemeConfigGroup::Accent))
            | Some(ThemeValueOrigin::BoundaryConfiguration {
                group: ThemeConfigGroup::Accent,
                ..
            })
    )
}

/// Reads the current provider without creating another theme owner.
pub(crate) fn use_mode() -> Signal<ThemeMode> {
    if let Some(theme) = rustify_ui::use_resolved_theme() {
        return Signal::derive(move || theme.get().mode);
    }
    #[cfg(target_arch = "wasm32")]
    if let Some(theme) = rustify_ui::use_theme_values() {
        return Signal::derive(move || {
            if theme.get().name == "dark" {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            }
        });
    }
    Signal::stored(ThemeMode::Light)
}

/// Adds explicit local color choices. The component supplies its semantic
/// fallback and uses `rustify-color` to reset inherited component roles.
pub(crate) fn set_color(
    styles: &mut StyleSet,
    color: Option<AccentColor>,
    mode: ThemeMode,
) -> Result<(), PropError> {
    let Some(color) = color else { return Ok(()) };
    let palette = ResolvedPalette::for_family(color.family(), mode).map_err(|error| PropError {
        field: "color",
        message: error.to_string(),
    })?;
    for (names, values) in [(&SOLID, &palette.solid), (&ALPHA, &palette.alpha)] {
        for (name, value) in names.iter().zip(values) {
            styles.insert(0, name, format_color(*value, ColorFormat::Rgb))?;
        }
    }
    for (name, value) in [
        ("--rustify-component-color-contrast", palette.contrast),
        ("--rustify-component-color-surface", palette.surface),
        ("--rustify-component-color-indicator", palette.indicator),
        ("--rustify-component-color-track", palette.track),
    ] {
        styles.insert(0, name, format_color(value, ColorFormat::Rgb))?;
    }
    Ok(())
}

const SOLID: [&str; 12] = [
    "--rustify-component-color-1",
    "--rustify-component-color-2",
    "--rustify-component-color-3",
    "--rustify-component-color-4",
    "--rustify-component-color-5",
    "--rustify-component-color-6",
    "--rustify-component-color-7",
    "--rustify-component-color-8",
    "--rustify-component-color-9",
    "--rustify-component-color-10",
    "--rustify-component-color-11",
    "--rustify-component-color-12",
];
const ALPHA: [&str; 12] = [
    "--rustify-component-color-a1",
    "--rustify-component-color-a2",
    "--rustify-component-color-a3",
    "--rustify-component-color-a4",
    "--rustify-component-color-a5",
    "--rustify-component-color-a6",
    "--rustify-component-color-a7",
    "--rustify-component-color-a8",
    "--rustify-component-color-a9",
    "--rustify-component-color-a10",
    "--rustify-component-color-a11",
    "--rustify-component-color-a12",
];
