//! Shared geometry projection for the native choice skins.

use crate::{geometry::StyleSet, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};

macro_rules! visual_types {
    ($size:ident, $variant:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum $size {
            One,
            #[default]
            Two,
            Three,
        }
        impl $size {
            pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
            pub const fn number(self) -> u8 {
                match self {
                    Self::One => 1,
                    Self::Two => 2,
                    Self::Three => 3,
                }
            }
        }
        impl From<$size> for leptos::prelude::Signal<crate::Responsive<$size>> {
            fn from(value: $size) -> Self {
                Self::stored(value.into())
            }
        }
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum $variant {
            Classic,
            #[default]
            Surface,
            Soft,
        }
        impl $variant {
            pub const ALL: &'static [Self] = &[Self::Classic, Self::Surface, Self::Soft];
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Classic => "classic",
                    Self::Surface => "surface",
                    Self::Soft => "soft",
                }
            }
        }
    };
}
pub(crate) use visual_types;

pub(crate) fn geometry(
    size: Signal<Responsive<u8>>,
    color: Signal<Option<AccentColor>>,
    radius: Signal<Option<ThemeRadius>>,
) -> String {
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        for (width, size) in size.get().iter() {
            let (shape, mark, switch, gap, font, line) = match size {
                1 => (
                    "calc(var(--rustify-space-4, 16px) * 0.875)",
                    "calc(var(--rustify-unit, 4px) * 2.25)",
                    "var(--rustify-space-4, 16px)",
                    2,
                    2,
                    2,
                ),
                3 => (
                    "calc(var(--rustify-space-4, 16px) * 1.25)",
                    "calc(var(--rustify-unit, 4px) * 3)",
                    "var(--rustify-space-5, 24px)",
                    3,
                    3,
                    3,
                ),
                _ => (
                    "var(--rustify-space-4, 16px)",
                    "calc(var(--rustify-unit, 4px) * 2.5)",
                    "calc(var(--rustify-space-5, 24px) * 5 / 6)",
                    2,
                    2,
                    2,
                ),
            };
            for (name, value) in [
                ("--rustify-choice-size", shape),
                ("--rustify-choice-mark", mark),
                ("--rustify-switch-height", switch),
            ] {
                styles.insert(width, name, value)?;
            }
            styles.insert(
                width,
                "--rustify-choice-gap",
                format!("var(--rustify-space-{gap}, 8px)"),
            )?;
            styles.insert(
                width,
                "--rustify-choice-font",
                format!("var(--rustify-font-size-{font}, 14px)"),
            )?;
            styles.insert(
                width,
                "--rustify-choice-line",
                format!("var(--rustify-line-height-{line}, 20px)"),
            )?;
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
            let scaling = theme.map_or(1., |theme| theme.get().config.selected.scaling.factor());
            styles.insert(
                0,
                "--rustify-choice-radius",
                format!("{}px", 4. * radius.factor() * scaling),
            )?;
            styles.insert(
                0,
                "--rustify-switch-radius",
                if radius == ThemeRadius::Full {
                    "9999px".into()
                } else {
                    format!("{}px", 4. * radius.factor() * scaling)
                },
            )?;
        }
        Ok(styles)
    })
}

#[component]
pub(crate) fn Mark(#[prop(optional)] radio: bool) -> impl IntoView {
    let state = crate::primitives::choice::use_checked_state();
    view! {
        <svg class="rustify-choice-glyph" viewBox="0 0 16 16" aria-hidden="true">
            {move || if radio {
                view! { <circle cx="8" cy="8" r="4" fill="currentColor" /> }.into_any()
            } else if state.get() == crate::primitives::CheckedState::Indeterminate {
                view! { <path d="M3 8h10" stroke="currentColor" stroke-width="2" /> }.into_any()
            } else {
                view! { <path d="m3 8 3 3 7-7" fill="none" stroke="currentColor" stroke-width="2" /> }.into_any()
            }}
        </svg>
    }
}

pub(crate) fn cards_geometry(
    size: Signal<Responsive<u8>>,
    columns: Signal<Responsive<crate::GridTracks>>,
    gap: Signal<crate::Gap>,
) -> String {
    crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        for (width, tracks) in columns.get().iter() {
            styles.insert(width, "grid-template-columns", tracks.css("columns")?)?;
        }
        gap.get().write(&mut styles)?;
        for (width, size) in size.get().iter() {
            let (x, y, radius) = match size {
                1 => (
                    "var(--rustify-space-3, 12px)",
                    "calc(var(--rustify-space-3, 12px) / 1.2)",
                    "var(--rustify-radius-3, 6px)",
                ),
                3 => (
                    "var(--rustify-space-5, 24px)",
                    "calc(var(--rustify-space-5, 24px) / 1.2)",
                    "var(--rustify-radius-4, 8px)",
                ),
                _ => (
                    "var(--rustify-space-4, 16px)",
                    "calc(var(--rustify-space-4, 16px) * 0.875)",
                    "var(--rustify-radius-3, 6px)",
                ),
            };
            styles.insert(width, "--rustify-card-padding-x", x)?;
            styles.insert(width, "--rustify-card-padding-y", y)?;
            styles.insert(width, "--rustify-card-radius", radius)?;
        }
        Ok(styles)
    })
}
