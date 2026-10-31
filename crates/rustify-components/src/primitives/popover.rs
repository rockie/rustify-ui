//! Controlled anchored task panels, sharing Dialog's lifecycle and stack.
pub use super::floating::{close_as, trigger_as, Anchor, Arrow, Close, Portal, Trigger};
use leptos::prelude::*;
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(optional)] modal: bool,
    children: Children,
) -> impl IntoView {
    super::floating::root(
        open,
        on_open_change,
        modal,
        super::floating::Kind::Popover,
        children,
    )
}
#[component]
pub fn Content(
    #[prop(optional, into)] position: Signal<rustify_ui::FloatingPosition>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    super::floating::content(
        super::floating::ContentConfig {
            align: Signal::default(),
            position: Some(position),
            aria_label: Some(aria_label),
        },
        super::floating::Hooks {
            escape: on_escape_key_down,
            pointer: on_pointer_down_outside,
            focus: on_focus_outside,
            interact: on_interact_outside,
            open: on_open_auto_focus,
            close: on_close_auto_focus,
        },
        class,
        test_id,
        children,
    )
}
