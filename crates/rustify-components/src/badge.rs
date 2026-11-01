//! Compact themed labels on a span or one caller-owned native element.

use crate::geometry::{PropError, StyleSet};
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    One,
    Two,
    Three,
}
impl BadgeSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<BadgeSize> for Signal<Responsive<BadgeSize>> {
    fn from(value: BadgeSize) -> Self {
        Signal::stored(value.into())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    Solid,
    #[default]
    Soft,
    Surface,
    Outline,
}
impl BadgeVariant {
    pub const ALL: &'static [Self] = &[Self::Solid, Self::Soft, Self::Surface, Self::Outline];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
            Self::Surface => "surface",
            Self::Outline => "outline",
        }
    }
}

/// Shared visual choices for [`Badge`] and [`badge_as`].
#[derive(Clone, Copy, Default)]
pub struct BadgeOptions {
    pub size: Signal<Responsive<BadgeSize>>,
    pub variant: Signal<BadgeVariant>,
    pub color: Signal<Option<AccentColor>>,
    pub high_contrast: Signal<bool>,
    pub radius: Signal<Option<ThemeRadius>>,
    pub margin: Signal<Margin>,
}

#[component]
pub fn Badge(
    #[prop(optional, into)] size: Signal<Responsive<BadgeSize>>,
    #[prop(optional, into)] variant: Signal<BadgeVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    badge_as(
        leptos::html::span()
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        BadgeOptions {
            size,
            variant,
            color,
            high_contrast,
            radius,
            margin,
        },
    )
}

/// Decorates one native target without replacing its attributes, children or listeners.
pub fn badge_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    options: BadgeOptions,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        set_size(&mut styles, &options.size.get())?;
        options.margin.get().write(&mut styles)?;
        crate::component_color::set_color(&mut styles, options.color.get(), mode.get())?;
        if let Some(radius) = options.radius.get() {
            let scaling = theme.map_or(1., |theme| theme.get().config.selected.scaling.factor());
            for (property, base) in [
                ("--rustify-badge-radius-1", 3.),
                ("--rustify-badge-radius-2", 4.),
            ] {
                styles.insert(
                    0,
                    property,
                    format!("{}px", base * radius.factor() * scaling),
                )?;
            }
            styles.insert(
                0,
                "--rustify-badge-radius-full",
                if radius == ThemeRadius::Full {
                    "9999px"
                } else {
                    "0px"
                },
            )?;
        }
        Ok(styles)
    });
    crate::primitives::Slot(
        target,
        (
            crate::primitives::slot::class_token("rustify-reset".into()),
            crate::primitives::slot::class_token("rustify-color".into()),
            crate::primitives::slot::class_token("rustify-badge".into()),
            crate::primitives::slot::class_token(geometry),
            data("data-name", "Badge"),
            data("data-rustify-part", "badge"),
            data("data-variant", move || options.variant.get().as_str()),
            data("data-color", move || {
                options.color.get().map(AccentColor::as_str)
            }),
            data("data-high-contrast", move || {
                options.high_contrast.get().then_some("true")
            }),
            data("data-radius", move || {
                options.radius.get().map(ThemeRadius::as_str)
            }),
            data("data-authored-primary", move || {
                theme
                    .is_some_and(|theme| {
                        !crate::component_color::primary_is_generated(&theme.get())
                    })
                    .then_some("true")
            }),
        ),
    )
}

fn set_size(styles: &mut StyleSet, sizes: &Responsive<BadgeSize>) -> Result<(), PropError> {
    for (width, size) in sizes.iter() {
        let (font, font_px, line_px, padding_y, padding_x, gap, radius) = match size {
            BadgeSize::One => (
                1,
                12,
                16,
                "calc(var(--rustify-space-1, 4px) * 0.5)",
                "calc(var(--rustify-space-1, 4px) * 1.5)",
                "calc(var(--rustify-space-1, 4px) * 1.5)",
                1,
            ),
            BadgeSize::Two => (
                1,
                12,
                16,
                "var(--rustify-space-1, 4px)",
                "var(--rustify-space-2, 8px)",
                "calc(var(--rustify-space-1, 4px) * 1.5)",
                2,
            ),
            BadgeSize::Three => (
                2,
                14,
                20,
                "var(--rustify-space-1, 4px)",
                "calc(var(--rustify-space-2, 8px) * 1.25)",
                "var(--rustify-space-2, 8px)",
                2,
            ),
        };
        for (property, value) in [
            (
                "--rustify-badge-font-size",
                format!("var(--rustify-font-size-{font}, {font_px}px)"),
            ),
            (
                "--rustify-badge-line-height",
                format!("var(--rustify-line-height-{font}, {line_px}px)"),
            ),
            (
                "--rustify-badge-tracking",
                if font == 1 { "0.0025em" } else { "0em" }.into(),
            ),
            ("--rustify-badge-padding-y", padding_y.into()),
            ("--rustify-badge-padding-x", padding_x.into()),
            ("--rustify-badge-gap", gap.into()),
            (
                "--rustify-badge-radius",
                format!(
                    "max(var(--rustify-badge-radius-{radius}), var(--rustify-badge-radius-full))"
                ),
            ),
        ] {
            styles.insert(width, property, value)?;
        }
    }
    Ok(())
}
