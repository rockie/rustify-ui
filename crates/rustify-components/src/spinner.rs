//! Something is happening and there is no percentage to show.

use leptos::prelude::*;

/// Rust/UI's spinner came from the `icons` crate, whose manifest turns on
/// `leptos/nightly`. This one draws its own ring, and turns on the scope's own
/// motion duration: a scope that asks for less movement gets a still glyph
/// rather than a spinning one (`css/controls.css`).
///
/// It is a status rather than an image: a reader hears "loading", not "circle".
#[component]
pub fn Spinner(
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let label = if label.is_empty() {
        "loading".to_string()
    } else {
        label
    };
    view! {
        <span
            class=crate::macros::merge("inline-flex text-muted-foreground", &class)
            data-name="Spinner"
            data-testid=test_id
            role="status"
            aria-label=label
        >
            <svg
                class="size-4"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                aria-hidden="true"
                focusable="false"
            >
                <circle cx="12" cy="12" r="9" class="opacity-25" />
                <path d="M21 12a9 9 0 0 0-9-9" />
            </svg>
        </span>
    }
}
