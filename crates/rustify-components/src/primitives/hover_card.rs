//! Native links with controlled supplemental reading surfaces.
pub use super::floating::{Arrow, Portal};
pub use super::popover::Content;
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(default = 700)] open_delay_ms: u32,
    #[prop(default = 300)] close_delay_ms: u32,
    children: Children,
) -> impl IntoView {
    super::hover::root(
        open,
        on_open_change,
        super::floating::Kind::HoverCard,
        super::hover::Options {
            open_delay: open_delay_ms,
            close_delay: close_delay_ms,
        },
        children,
    )
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::A, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    super::hover::trigger_as(target)
}
#[component]
pub fn Trigger(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    trigger_as(
        leptos::html::a()
            .href(href)
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
    )
}
