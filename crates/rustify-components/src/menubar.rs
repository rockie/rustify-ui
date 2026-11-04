//! Compact application commands with themed menu surfaces.
use crate::dropdown_menu::{content, surface, Appearance, Surface};
pub use crate::dropdown_menu::{
    Arrow, CheckboxItem, Group, Item, ItemIndicator, Label, MenuSize, MenuVariant, Portal,
    RadioGroup, RadioItem, Separator, Sub, SubContent, SubTrigger,
};
pub use crate::primitives::menubar::Menu;
use leptos::prelude::*;
use rustify_ui::{AccentColor, Cancelable, FloatingPosition, LayerDismiss};
content!(
    Content,
    Menubar,
    Signal::default(),
    Signal::default(),
    Signal::default(),
    Signal::default()
);
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(default = true)] loop_focus: bool,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::menubar::Root value=value on_value_change=on_value_change loop_focus=loop_focus aria_label=aria_label class=crate::macros::merge("rustify-menubar",&class) test_id=test_id>{children()}</crate::primitives::menubar::Root>}
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::menubar::Trigger disabled=disabled class=crate::macros::merge("rustify-menubar-trigger",&class) test_id=test_id>{children()}</crate::primitives::menubar::Trigger>}
}
