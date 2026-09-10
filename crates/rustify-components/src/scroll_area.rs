//! A box with more in it than fits.
//!
//! Rust/UI's scroll area drew its own scrollbar next to a `overflow-auto`
//! viewport - a bar that never moved, because nothing connected it to the
//! scroll position. This one scrolls with the browser's own bar, painted in
//! the scope's colours (`css/controls.css`), and is reachable from the
//! keyboard, which a plain overflow container is not.

use leptos::prelude::*;

const BASE: &str = "rui:relative rui:overflow-auto rui:rounded-md rui:outline-none rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px]";

/// What happens to a wheel that arrives after this box has stopped scrolling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Boundary {
    /// The page behind it scrolls next. The browser's default, and what a
    /// panel inside a document wants.
    #[default]
    Propagate,
    /// Nothing else scrolls. What a pane that fills its own area wants, and
    /// the same rule a GPU region declares for itself (P2 §5.4).
    Stop,
}

impl Boundary {
    fn class(self) -> &'static str {
        match self {
            Self::Propagate => "rui:overscroll-auto",
            Self::Stop => "rui:overscroll-contain",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Propagate => "propagate",
            Self::Stop => "stop",
        }
    }
}

/// A scrolling box.
///
/// It takes a tab stop, because a region a mouse can scroll and a keyboard
/// cannot is unreachable for anyone not using a mouse; the name is what tells
/// a reader which one they have landed in.
#[component]
pub fn ScrollArea(
    #[prop(optional)] boundary: Boundary,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    view! {
        <div
            class=crate::macros::merge(&format!("{BASE} {}", boundary.class()), &class)
            data-name="ScrollArea"
            data-testid=test_id
            data-boundary=boundary.name()
            role="group"
            aria-label=aria_label
            tabindex="0"
        >
            {children()}
        </div>
    }
}
