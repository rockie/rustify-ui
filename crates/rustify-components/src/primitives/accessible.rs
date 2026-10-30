//! Small accessibility parts which carry no visual theme.

use leptos::prelude::*;

/// Clips content visually while preserving it for assistive technology.
#[component]
pub fn VisuallyHidden(children: Children) -> impl IntoView {
    view! { <span data-rustify-part="visually-hidden">{children()}</span> }
}

/// Gives an icon one accessible name, or makes an unlabeled icon decorative.
///
/// Pass a decorative glyph as children. An icon-only operation must still name
/// its actual button; this part does not turn a picture into an operation.
#[component]
pub fn AccessibleIcon(#[prop(optional, into)] label: String, children: Children) -> impl IntoView {
    let named = !label.is_empty();
    view! {
        <span data-rustify-part="accessible-icon" role=named.then_some("img")
            aria-label=named.then_some(label) aria-hidden=(!named).then_some("true")>
            <span aria-hidden="true">{children()}</span>
        </span>
    }
}
