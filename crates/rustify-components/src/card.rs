//! Themed containers; native interactivity belongs to the caller's target.

use crate::geometry::{PropError, StyleSet};
use crate::{Layout, Margin, Responsive};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}
impl CardSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three, Self::Four, Self::Five];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<CardSize> for Signal<Responsive<CardSize>> {
    fn from(value: CardSize) -> Self {
        Signal::stored(value.into())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Surface,
    Classic,
    Ghost,
}
impl CardVariant {
    pub const ALL: &'static [Self] = &[Self::Surface, Self::Classic, Self::Ghost];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Classic => "classic",
            Self::Ghost => "ghost",
        }
    }
}

/// Shared visual choices for [`Card`] and [`card_as`].
#[derive(Clone, Copy, Default)]
pub struct CardOptions {
    pub size: Signal<Responsive<CardSize>>,
    pub variant: Signal<CardVariant>,
    pub layout: Signal<Layout>,
    pub margin: Signal<Margin>,
}

/// A static div. Use [`card_as`] for a native link or other semantic target.
/// A whole-card target should not contain nested interactive elements.
#[component]
pub fn Card(
    #[prop(optional, into)] size: Signal<Responsive<CardSize>>,
    #[prop(optional, into)] variant: Signal<CardVariant>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    card_as(
        leptos::html::div()
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        CardOptions {
            size,
            variant,
            layout,
            margin,
        },
    )
}

/// Applies the skin without creating a wrapper, click handler or keyboard stop.
pub fn card_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    options: CardOptions,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        set_size(&mut styles, &options.size.get())?;
        options.layout.get().write(&mut styles)?;
        let mut margins = StyleSet::default();
        options.margin.get().write(&mut margins)?;
        let ghost = options.variant.get() == CardVariant::Ghost;
        for width in crate::responsive::BREAKPOINTS {
            for property in ["margin-top", "margin-right", "margin-bottom", "margin-left"] {
                let value = margins.value_at(width, property);
                if ghost {
                    let value = value.unwrap_or("0px");
                    styles.insert(
                        width,
                        property,
                        if matches!(
                            value,
                            "auto" | "inherit" | "initial" | "unset" | "revert" | "revert-layer"
                        ) {
                            value.into()
                        } else {
                            format!("calc({value} - var(--rustify-card-padding))")
                        },
                    )?;
                } else if let Some(value) = value {
                    styles.insert(width, property, value)?;
                }
            }
        }
        Ok(styles)
    });
    crate::primitives::Slot(
        target,
        (
            crate::primitives::slot::class_token("rustify-reset".into()),
            crate::primitives::slot::class_token("rustify-card".into()),
            crate::primitives::slot::class_token(geometry),
            data("data-name", "Card"),
            data("data-rustify-part", "card"),
            data("data-variant", move || options.variant.get().as_str()),
        ),
    )
}

fn set_size(styles: &mut StyleSet, sizes: &Responsive<CardSize>) -> Result<(), PropError> {
    for (width, size) in sizes.iter() {
        let (space, pixels, radius, radius_px) = match size {
            CardSize::One => (3, 12, 4, 8),
            CardSize::Two => (4, 16, 4, 8),
            CardSize::Three => (5, 24, 5, 12),
            CardSize::Four => (6, 32, 5, 12),
            CardSize::Five => (8, 48, 6, 16),
        };
        styles.insert(
            width,
            "--rustify-card-padding",
            format!("var(--rustify-space-{space}, {pixels}px)"),
        )?;
        styles.insert(
            width,
            "--rustify-card-radius",
            format!("var(--rustify-radius-{radius}, {radius_px}px)"),
        )?;
    }
    Ok(())
}
