//! Composable controlled dialogs with scope-local modality.
pub use super::floating::{
    close_as, trigger_as, Close, Content, Description, Overlay, Portal, Title, Trigger,
};
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(default = true)] modal: bool,
    children: Children,
) -> impl IntoView {
    super::floating::root(
        open,
        on_open_change,
        modal,
        super::floating::Kind::Dialog,
        children,
    )
}
