//! A single localized loading status with retained content geometry.
use leptos::prelude::*;
#[component]
fn Status(
    loading: Signal<bool>,
    label: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let locale = rustify_ui::use_locale();
    view! {<span class=class data-testid=test_id data-rustify-part="spinner-status" hidden=move ||!loading.get()
        role=move ||loading.get().then_some("status")
        aria-label=move ||loading.get().then(||{let label=label.get();if label.trim().is_empty(){locale.text(rustify_ui::Message::Loading).into()}else{label}})>
        <span data-rustify-part="spinner-graphic" aria-hidden="true">
            {(0..8).map(|_|view! {<span data-rustify-part="spinner-leaf" />}).collect_view()}
        </span>
    </span>}
}
/// Returns the retained native target followed by a loading status sibling.
///
/// Call within a flow-content parent: a complete table is a valid target, but
/// table rows/cells need skeleton_as or a Spinner in one of their legal containers.
/// Loading owns aria-hidden/inert; original target listeners remain installed.
pub fn spinner_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: impl Into<Signal<bool>>,
    label: impl Into<Signal<String>>,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    with_target(target, loading.into(), label.into(), String::new())
}
pub(crate) fn with_target<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: Signal<bool>,
    label: Signal<String>,
    class: String,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    (
        super::loading::target(target, loading, "spinner-target"),
        view! {<Status loading=loading label=label class=class />},
    )
}
#[component]
pub fn Root(
    #[prop(default=Signal::stored(true),into)] loading: Signal<bool>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if let Some(children) = children {
        view! {<div class=class data-rustify-part="spinner-root" data-testid=test_id>
            {super::loading::target(leptos::html::div().child(children()),loading,"spinner-target")}
            <Status loading=loading label=label />
        </div>}
        .into_any()
    } else {
        view! {<Status loading=loading label=label class=class test_id=test_id />}.into_any()
    }
}
