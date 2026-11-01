use leptos::prelude::*;

pub(crate) fn target<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: Signal<bool>,
    part: &'static str,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    super::Slot(
        target,
        (
            data("data-rustify-part", part),
            data("data-loading", move || loading.get().to_string()),
            data("aria-hidden", move || loading.get().then_some("true")),
            data("inert", move || loading.get().then_some("")),
        ),
    )
}
