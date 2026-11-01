//! Skeletons retain the original DOM and derive layout through external rules.
use crate::{Layout, Margin};
use leptos::prelude::*;
fn geometry(layout: Signal<Layout>, margin: Signal<Margin>) -> String {
    crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        layout.get().write(&mut styles)?;
        margin.get().write(&mut styles)?;
        Ok(styles)
    })
}
/// Keeps the native tag/listeners and projects loading without replacing it.
/// Replaced img/video targets become an empty box; wrap them in a legal native
/// container when a visible placeholder background is needed.
pub fn skeleton_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: impl Into<Signal<bool>>,
    layout: impl Into<Signal<Layout>>,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let geometry = geometry(layout.into(), Signal::default());
    crate::primitives::skeleton::skeleton_as(
        target.add_any_attr((
            crate::primitives::slot::class_token("rustify-skeleton".into()),
            crate::primitives::slot::class_token(geometry),
        )),
        loading,
    )
}
#[component]
pub fn Skeleton(
    #[prop(default=Signal::stored(true),into)] loading: Signal<bool>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let geometry = geometry(layout, margin);
    crate::primitives::skeleton::Root(crate::primitives::skeleton::RootProps {
        loading,
        class: crate::macros::merge(format!("rustify-skeleton {geometry}"), &class),
        test_id,
        children,
    })
}
pub use Skeleton as Root;
