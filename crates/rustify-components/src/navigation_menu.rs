//! Native site navigation with compact triggers and a measured content surface.
pub use crate::primitives::navigation_menu::link_as;
use crate::{macros::merge, Orientation};
use leptos::prelude::*;
use rustify_ui::Cancelable;
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(default = 200)] delay_ms: u32,
    #[prop(default = 300)] skip_delay_ms: u32,
    #[prop(optional)] orientation: Orientation,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Root value=value on_value_change=on_value_change delay_ms=delay_ms skip_delay_ms=skip_delay_ms orientation=orientation aria_label=aria_label class=merge("rustify-navigation-menu",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::Root>}
}
#[component]
pub fn Sub(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Sub value=value on_value_change=on_value_change orientation=orientation class=merge("rustify-navigation-menu",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::Sub>}
}
#[component]
pub fn List(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::List class=merge("rustify-navigation-menu-list",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::List>}
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Item value=value class=class test_id=test_id>{children()}</crate::primitives::navigation_menu::Item>}
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Trigger disabled=disabled class=merge("rustify-navigation-menu-trigger",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::Trigger>}
}
#[component]
pub fn Content(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Content force_mount=force_mount class=merge("rustify-navigation-menu-content",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::Content>}
}
#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] target: String,
    #[prop(optional, into)] rel: String,
    #[prop(optional_no_strip, into)] download: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Link href=href active=active on_select=on_select target=target rel=rel download=download class=merge("rustify-navigation-menu-link",&class) test_id=test_id>{children()}</crate::primitives::navigation_menu::Link>}
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Indicator class=merge("rustify-navigation-menu-indicator",&class) test_id=test_id />}
}
#[component]
pub fn Viewport(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::navigation_menu::Viewport class=merge("rustify-navigation-menu-viewport",&class) test_id=test_id />}
}
