//! A decorative or semantic division without an additional keyboard stop.
use crate::Orientation;
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(optional, into)] orientation: Signal<Orientation>,
    #[prop(default=Signal::stored(true),into)] decorative: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<span class=crate::macros::reactive_merge("",&class) data-rustify-part="separator-root" data-testid=test_id
    data-orientation=move ||orientation.get().attribute()
    role=move ||if decorative.get(){"presentation"}else{"separator"}
    aria-hidden=move ||decorative.get().then_some("true")
    aria-orientation=move ||(!decorative.get()).then(||orientation.get().attribute()) />}
}
