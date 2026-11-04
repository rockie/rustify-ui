//! Compact themed commands sharing one keyboard focus sequence.
pub use crate::primitives::toolbar::{button_as, item, link_as};
pub use crate::primitives::SelectionMode;
use crate::{macros::merge, Orientation};
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] loop_focus: bool,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::Root orientation=orientation loop_focus=loop_focus aria_label=aria_label class=merge("rustify-toolbar",&class) test_id=test_id>{children()}</crate::primitives::toolbar::Root>}
}
#[component]
pub fn Button(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional_no_strip, into)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::Button disabled=disabled class=merge("rustify-toolbar-button",&class) test_id=test_id on_click=on_click>{children()}</crate::primitives::toolbar::Button>}
}
#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional, into)] target: String,
    #[prop(optional, into)] rel: String,
    #[prop(optional_no_strip, into)] download: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::Link href=href target=target rel=rel download=download class=merge("rustify-toolbar-link",&class) test_id=test_id>{children()}</crate::primitives::toolbar::Link>}
}
#[component]
pub fn Separator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::Separator class=merge("rustify-toolbar-separator",&class) test_id=test_id />}
}
#[component]
pub fn ToggleGroup(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(optional)] mode: SelectionMode,
    #[prop(default = true)] allow_empty: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::ToggleGroup value=value on_change=on_change mode=mode allow_empty=allow_empty disabled=disabled class=merge("rustify-toolbar-group",&class) test_id=test_id>{children()}</crate::primitives::toolbar::ToggleGroup>}
}
#[component]
pub fn ToggleItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::toolbar::ToggleItem value=value disabled=disabled class=merge("rustify-toolbar-toggle",&class) test_id=test_id>{children()}</crate::primitives::toolbar::ToggleItem>}
}
