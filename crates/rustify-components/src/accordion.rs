//! Themed accordion parts sharing the expandable-region implementation.
pub use crate::primitives::accordion::{AccordionValue, Item, Root};

#[component]
pub fn Header(
    #[prop(default=crate::HeadingLevel::H3)] level: crate::HeadingLevel,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::accordion::Header level=level class=format!("rustify-accordion-header {class}")>{children()}</crate::primitives::accordion::Header>}
}
use leptos::prelude::*;
#[component]
pub fn Trigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::accordion::Trigger class=crate::macros::merge("rustify-disclosure-trigger",&class) test_id=test_id>{children()}</crate::primitives::accordion::Trigger>}
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::accordion::trigger_as(target.add_any_attr(
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
    view! {<crate::primitives::accordion::Content force_mount=force_mount class=crate::macros::merge("rustify-disclosure-content",&class) test_id=test_id>{children()}</crate::primitives::accordion::Content>}
}
