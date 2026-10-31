//! Safe confirmation dialogs sharing the controlled floating lifecycle.
pub use super::floating::{
    action_as, cancel_as, trigger_as, Action, Cancel, Content, Description, Overlay, Portal, Title,
    Trigger,
};
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    children: Children,
) -> impl IntoView {
    super::floating::root(
        open,
        on_open_change,
        true,
        super::floating::Kind::AlertDialog,
        children,
    )
}
