//! A retained loading target, hidden from assistive technology and interaction.
use leptos::prelude::*;

/// Decorates one native target without changing its tag or recreating children.
///
/// Replaced targets such as img/video keep an empty layout box while loading.
/// Use a legal container around them when a visible skeleton surface is needed.
/// The loading projection owns aria-hidden and inert, as Slot owns reserved attributes.
pub fn skeleton_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: impl Into<Signal<bool>>,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    super::loading::target(target, loading.into(), "skeleton-target")
}
#[component]
pub fn Root(
    #[prop(default=Signal::stored(true),into)] loading: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if let Some(children) = children {
        skeleton_as(
            leptos::html::div()
                .class(class)
                .attr("data-testid", test_id)
                .child(children()),
            loading,
        )
        .into_any()
    } else {
        skeleton_as(
            leptos::html::span()
                .class(class)
                .attr("data-testid", test_id),
            loading,
        )
        .into_any()
    }
}
