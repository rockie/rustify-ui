//! A container for a drawing, and the handful of drawings this crate needs.
//!
//! There is no dependency on Rust/UI's `icons` crate: its manifest turns on
//! `leptos/nightly`, and the toolchain this repository pins is stable. What is
//! here instead is a `<svg>` wrapper an application can put any child into,
//! and the six glyphs the components in this crate draw for themselves.

use leptos::prelude::*;

const BASE: &str = "inline-block shrink-0 size-4";

/// The drawings this crate ships. An application that wants another one puts
/// its own children in [`Icon`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glyph {
    Check,
    ChevronDown,
    ChevronUp,
    ChevronRight,
    Close,
    Dot,
}

impl Glyph {
    pub fn name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::ChevronUp => "chevron-up",
            Self::ChevronRight => "chevron-right",
            Self::Close => "close",
            Self::Dot => "dot",
        }
    }

    /// The outline, on the 24×24 grid the wrapper's `viewBox` sets up.
    fn path(self) -> &'static str {
        match self {
            Self::Check => "M20 6 9 17l-5-5",
            Self::ChevronDown => "m6 9 6 6 6-6",
            Self::ChevronUp => "m18 15-6-6-6 6",
            Self::ChevronRight => "m9 18 6-6-6-6",
            Self::Close => "M18 6 6 18M6 6l12 12",
            Self::Dot => "M12 8a4 4 0 1 0 0 8 4 4 0 1 0 0-8",
        }
    }

    /// A dot is a shape rather than a stroke; everything else is a line.
    fn solid(self) -> bool {
        matches!(self, Self::Dot)
    }
}

/// One drawing, sized by the text around it.
///
/// A glyph with no `label` is decoration and stays out of the accessibility
/// tree: the control it sits in already has a name, and a second one read
/// after it is noise. Giving a label makes it an image with that name, which
/// is what an icon-only control needs.
#[component]
pub fn Icon(
    #[prop(optional)] glyph: Option<Glyph>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let named = !label.is_empty();
    let solid = glyph.is_some_and(Glyph::solid);
    view! {
        <svg
            class=crate::macros::merge(BASE, &class)
            data-name="Icon"
            data-testid=test_id
            data-glyph=glyph.map(Glyph::name)
            viewBox="0 0 24 24"
            fill=if solid { "currentColor" } else { "none" }
            stroke=if solid { "none" } else { "currentColor" }
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            role=named.then_some("img")
            aria-label=named.then(|| label.clone())
            aria-hidden=(!named).then_some("true")
            focusable="false"
        >
            {glyph.map(|glyph| view! { <path d=glyph.path() /> })}
            {children.map(|children| children())}
        </svg>
    }
}
