//! Themed anchored panels on the shared controlled floating lifecycle.
pub use crate::dialog::{close_as, trigger_as, Close, Trigger};
pub use crate::primitives::popover::{Anchor, Portal, Root};
use leptos::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverSize {
    One,
    #[default]
    Two,
    Three,
    Four,
}
impl PopoverSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three, Self::Four];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<PopoverSize> for Signal<crate::Responsive<PopoverSize>> {
    fn from(value: PopoverSize) -> Self {
        Signal::stored(value.into())
    }
}
#[component]
pub fn Content(
    #[prop(optional, into)] size: Signal<crate::Responsive<PopoverSize>>,
    #[prop(optional, into)] layout: Signal<crate::Layout>,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
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
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        if let Some(theme) = theme {
            styles.set_floating_surface(&theme.get())?;
        }
        for (width, size) in size.get().iter() {
            let (space, padding, radius, pixels) = match size {
                PopoverSize::One => (3, 12, 4, 8),
                PopoverSize::Two => (4, 16, 4, 8),
                PopoverSize::Three => (5, 24, 5, 12),
                PopoverSize::Four => (6, 32, 5, 12),
            };
            styles.insert(
                width,
                "--rustify-popover-padding",
                format!("var(--rustify-space-{space}, {padding}px)"),
            )?;
            styles.insert(
                width,
                "--rustify-popover-radius",
                format!("var(--rustify-radius-{radius}, {pixels}px)"),
            )?;
        }
        let layout = layout.get();
        for (property, variable, value) in [
            ("width", "--rustify-popover-width", &layout.width),
            (
                "min-width",
                "--rustify-popover-min-width",
                &layout.min_width,
            ),
            (
                "max-width",
                "--rustify-popover-max-width",
                &layout.max_width,
            ),
            ("height", "--rustify-popover-height", &layout.height),
            (
                "min-height",
                "--rustify-popover-min-height",
                &layout.min_height,
            ),
            (
                "max-height",
                "--rustify-popover-max-height",
                &layout.max_height,
            ),
        ] {
            styles.set_dimension_variable(property, variable, value)?;
        }
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! {<crate::primitives::popover::Content position=position aria_label=aria_label on_escape_key_down=on_escape_key_down on_pointer_down_outside=on_pointer_down_outside
    on_focus_outside=on_focus_outside on_interact_outside=on_interact_outside on_open_auto_focus=on_open_auto_focus on_close_auto_focus=on_close_auto_focus
    class=crate::macros::merge(format!("rustify-popover-content {geometry}"),&class) test_id=test_id>{children()}</crate::primitives::popover::Content>}
}

#[component]
pub fn Arrow(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::popover::Arrow class=crate::macros::merge("rustify-popover-arrow",&class) test_id=test_id />}
}
