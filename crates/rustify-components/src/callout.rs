//! Static explanations, with announcements enabled only by an explicit role.

use crate::geometry::{PropError, StyleSet};
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutSize {
    One,
    #[default]
    Two,
    Three,
}
impl CalloutSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<CalloutSize> for Signal<Responsive<CalloutSize>> {
    fn from(value: CalloutSize) -> Self {
        Signal::stored(value.into())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutVariant {
    #[default]
    Soft,
    Surface,
    Outline,
}
impl CalloutVariant {
    pub const ALL: &'static [Self] = &[Self::Soft, Self::Surface, Self::Outline];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Surface => "surface",
            Self::Outline => "outline",
        }
    }
}

/// Optional live-region semantics for an author-selected change in status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalloutRole {
    Status,
    Alert,
}
impl CalloutRole {
    pub const ALL: &'static [Self] = &[Self::Status, Self::Alert];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Alert => "alert",
        }
    }
}

/// A static explanation by default; color supplements readable content.
#[component]
pub fn Root(
    #[prop(optional, into)] size: Signal<Responsive<CalloutSize>>,
    #[prop(optional, into)] variant: Signal<CalloutVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] role: Signal<Option<CalloutRole>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        set_size(&mut styles, &size.get())?;
        margin.get().write(&mut styles)?;
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-callout".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! {
        <div class=classes data-name="Callout" data-rustify-part="callout-root" data-testid=test_id
            role=move || role.get().map(CalloutRole::as_str)
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().map(AccentColor::as_str)
            data-high-contrast=move || high_contrast.get().then_some("true")
            data-authored-primary=move || theme.is_some_and(|theme| !crate::component_color::primary_is_generated(&theme.get())).then_some("true")>
            {children()}
        </div>
    }
}

/// An icon is decorative unless the caller explicitly supplies its semantics.
#[component]
pub fn Icon(
    #[prop(default = Signal::stored(true), into)] decorative: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=crate::macros::reactive_merge_with(move || vec!["rustify-callout-icon".into(), class.get()])
            data-rustify-part="callout-icon" data-testid=test_id aria-hidden=move || decorative.get().then_some("true")>{children()}</div>
    }
}

/// A paragraph that retains the author's text and inline markup.
#[component]
pub fn Text(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=crate::macros::reactive_merge_with(move || vec!["rustify-callout-text".into(), class.get()])
            data-rustify-part="callout-text" data-testid=test_id>{children()}</p>
    }
}

fn set_size(styles: &mut StyleSet, sizes: &Responsive<CalloutSize>) -> Result<(), PropError> {
    for (width, size) in sizes.iter() {
        let (
            row_gap,
            row_px,
            column_gap,
            column_px,
            padding,
            padding_px,
            radius,
            radius_px,
            text,
            text_px,
            line_px,
        ) = match size {
            CalloutSize::One => (2, 8, 2, 8, 3, 12, 3, 6, 2, 14, 20),
            CalloutSize::Two => (2, 8, 3, 12, 4, 16, 4, 8, 2, 14, 20),
            CalloutSize::Three => (3, 12, 4, 16, 5, 24, 5, 12, 3, 16, 24),
        };
        for (property, value) in [
            (
                "--rustify-callout-row-gap",
                format!("var(--rustify-space-{row_gap}, {row_px}px)"),
            ),
            (
                "--rustify-callout-column-gap",
                format!("var(--rustify-space-{column_gap}, {column_px}px)"),
            ),
            (
                "--rustify-callout-padding",
                format!("var(--rustify-space-{padding}, {padding_px}px)"),
            ),
            (
                "--rustify-callout-radius",
                format!("var(--rustify-radius-{radius}, {radius_px}px)"),
            ),
            (
                "--rustify-callout-font-size",
                format!("var(--rustify-font-size-{text}, {text_px}px)"),
            ),
            (
                "--rustify-callout-line-height",
                format!("var(--rustify-line-height-{text}, {line_px}px)"),
            ),
        ] {
            styles.insert(width, property, value)?;
        }
    }
    Ok(())
}
