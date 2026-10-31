//! Themed confirmation parts with safe initial focus and explicit Action semantics.
pub use crate::dialog::{
    trigger_as, Align, Content, Description, DialogSize, Overlay, Portal, Title, Trigger,
};
pub use crate::primitives::alert_dialog::Root;
use leptos::prelude::*;
#[component]
pub fn Cancel(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::alert_dialog::Cancel class=crate::macros::merge("rustify-dialog-button",&class) test_id=test_id>{children()}</crate::primitives::alert_dialog::Cancel>}
}
#[component]
pub fn Action(
    #[prop(into)] on_action: Callback<rustify_ui::Cancelable<()>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::alert_dialog::Action on_action=on_action class=crate::macros::merge("rustify-dialog-button",&class) test_id=test_id>{children()}</crate::primitives::alert_dialog::Action>}
}
pub fn cancel_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::alert_dialog::cancel_as(target.add_any_attr(
        crate::primitives::slot::class_token("rustify-dialog-button".into()),
    ))
}
pub fn action_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    on_action: Callback<rustify_ui::Cancelable<()>>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::alert_dialog::action_as(
        target.add_any_attr(crate::primitives::slot::class_token(
            "rustify-dialog-button".into(),
        )),
        on_action,
    )
}
