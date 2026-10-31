//! Themed supplemental reading surfaces on native links.
pub use crate::popover::Arrow;
pub use crate::primitives::hover_card::{trigger_as, Portal, Root, Trigger};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardSize {
    One,
    #[default]
    Two,
    Three,
}
impl HoverCardSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<HoverCardSize> for Signal<crate::Responsive<HoverCardSize>> {
    fn from(value: HoverCardSize) -> Self {
        Signal::stored(value.into())
    }
}

#[component]
pub fn Content(
    #[prop(optional, into)] size: Signal<crate::Responsive<HoverCardSize>>,
    #[prop(optional, into)] layout: Signal<crate::Layout>,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] position: Signal<rustify_ui::FloatingPosition>,
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
    let size = Signal::derive(move || {
        size.get().map(|size| match size {
            HoverCardSize::One => crate::popover::PopoverSize::One,
            HoverCardSize::Two => crate::popover::PopoverSize::Two,
            HoverCardSize::Three => crate::popover::PopoverSize::Three,
        })
    });
    view! {<crate::popover::Content size=size layout=layout margin=margin position=position
        on_escape_key_down=on_escape_key_down on_pointer_down_outside=on_pointer_down_outside
        on_focus_outside=on_focus_outside on_interact_outside=on_interact_outside
        on_open_auto_focus=on_open_auto_focus on_close_auto_focus=on_close_auto_focus
        class=crate::macros::merge("rustify-hover-card-content",&class) test_id=test_id>
        {children()}
    </crate::popover::Content>}
}
