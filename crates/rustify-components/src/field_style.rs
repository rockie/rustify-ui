//! Shared input skin geometry; editing remains in primitives::field.
use crate::{geometry::StyleSet, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};

pub(crate) fn classes(
    size: Signal<Responsive<u8>>,
    color: Signal<Option<AccentColor>>,
    radius: Signal<Option<ThemeRadius>>,
    class: Signal<String>,
    area: bool,
) -> Signal<String> {
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        for (width, size) in size.get().iter() {
            let (height, padding, gap, slot_padding, radius, area_height, area_y) = match size {
                1 => (
                    5,
                    "calc(var(--rustify-space-1, 4px) * 1.5 - 1px)",
                    2,
                    1,
                    2,
                    "var(--rustify-space-8, 64px)",
                    "calc(var(--rustify-space-1, 4px) - 1px)",
                ),
                3 => (
                    7,
                    "calc(var(--rustify-space-3, 12px) - 1px)",
                    3,
                    3,
                    3,
                    "80px",
                    "calc(var(--rustify-space-2, 8px) - 1px)",
                ),
                _ => (
                    6,
                    "calc(var(--rustify-space-2, 8px) - 1px)",
                    2,
                    2,
                    2,
                    "var(--rustify-space-9, 80px)",
                    "calc(var(--rustify-space-1, 4px) * 1.5 - 1px)",
                ),
            };
            styles.insert(
                width,
                "--rustify-field-height",
                format!(
                    "var(--rustify-space-{height}, {}px)",
                    match height {
                        5 => 24,
                        7 => 40,
                        _ => 32,
                    }
                ),
            )?;
            styles.insert(width, "--rustify-field-padding", padding)?;
            styles.insert(
                width,
                "--rustify-field-slot-gap",
                format!("var(--rustify-space-{gap}, {}px)", gap * 4),
            )?;
            styles.insert(
                width,
                "--rustify-field-slot-padding",
                format!(
                    "var(--rustify-space-{slot_padding}, {}px)",
                    slot_padding * 4
                ),
            )?;
            styles.insert(
                width,
                "--rustify-field-radius-base",
                format!(
                    "var(--rustify-radius-{radius}, {}px)",
                    if radius == 3 { 6 } else { 4 }
                ),
            )?;
            styles.insert(
                width,
                "--rustify-field-radius-unit",
                if radius == 3 {
                    "calc(var(--rustify-unit, 4px) * 1.5)"
                } else {
                    "var(--rustify-unit, 4px)"
                },
            )?;
            styles.insert(
                width,
                "--rustify-field-font",
                format!(
                    "var(--rustify-font-size-{size}, {}px)",
                    if *size == 1 {
                        12
                    } else if *size == 3 {
                        16
                    } else {
                        14
                    }
                ),
            )?;
            styles.insert(
                width,
                "--rustify-field-line",
                format!(
                    "var(--rustify-line-height-{size}, {}px)",
                    if *size == 1 {
                        16
                    } else if *size == 3 {
                        24
                    } else {
                        20
                    }
                ),
            )?;
            if area {
                styles.insert(width, "--rustify-area-height", area_height)?;
                styles.insert(width, "--rustify-area-padding-y", area_y)?;
            }
        }
        let color = color.get();
        crate::component_color::set_color(
            &mut styles,
            color,
            if color.is_some() {
                mode.get()
            } else {
                ThemeMode::Light
            },
        )?;
        if let Some(radius) = radius.get() {
            styles.insert(
                0,
                "--rustify-field-radius",
                if radius == ThemeRadius::Full {
                    "9999px".into()
                } else {
                    format!(
                        "calc(var(--rustify-field-radius-unit, 4px) * {})",
                        radius.factor()
                    )
                },
            )?;
        }
        Ok(styles)
    });
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-field".into(),
            if area {
                "rustify-text-area".into()
            } else {
                String::new()
            },
            geometry.clone(),
            class.get(),
        ]
    });
    Signal::derive(classes)
}
