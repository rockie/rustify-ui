//! Themed parts for a controlled expandable section.
pub use crate::primitives::collapsible::Root;
use leptos::prelude::*;
#[component]
pub fn Trigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::collapsible::Trigger class=crate::macros::merge("rustify-disclosure-trigger",&class) test_id=test_id>{children()}</crate::primitives::collapsible::Trigger>}
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::collapsible::trigger_as(target.add_any_attr(
        crate::primitives::slot::class_token("rustify-disclosure-trigger".into()),
    ))
}
#[component]
pub fn Content(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {<crate::primitives::collapsible::Content force_mount=force_mount class=crate::macros::merge("rustify-disclosure-content",&class) test_id=test_id>{children()}</crate::primitives::collapsible::Content>}
}
