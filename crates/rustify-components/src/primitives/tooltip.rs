//! Controlled descriptions on a single native target.
pub use super::floating::{Arrow, Portal};
pub use super::popover::Content;
use leptos::prelude::*;
#[component]
pub fn Provider(
    #[prop(default = 700)] delay_ms: u32,
    #[prop(default = 300)] skip_delay_ms: u32,
    #[prop(optional)] disable_hoverable_content: bool,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        provide_context(super::hover::ProviderContext::new(
            delay_ms,
            skip_delay_ms,
            disable_hoverable_content,
        ));
        children()
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(optional)] delay_ms: Option<u32>,
    children: Children,
) -> impl IntoView {
    let delay = delay_ms.unwrap_or_else(|| {
        use_context::<super::hover::ProviderContext>().map_or(700, |provider| provider.delay)
    });
    super::hover::root(
        open,
        on_open_change,
        super::floating::Kind::Tooltip,
        super::hover::Options {
            open_delay: delay,
            close_delay: 0,
        },
        children,
    )
}
pub fn trigger_as<E, At, Ch>(target: leptos::html::HtmlElement<E, At, Ch>) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    super::hover::trigger_as(target)
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    trigger_as(
        leptos::html::button()
            .r#type("button")
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
    )
}
