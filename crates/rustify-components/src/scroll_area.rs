//! Themed ScrollArea parts and a convenience composition over one native scrollport.

pub use crate::primitives::scroll_area::{
    Boundary, Corner, Scrollbar, Scrollbars, Thumb, Viewport, Visibility,
};
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::{Direction, ThemeRadius};

/// The thickness of each custom scrollbar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollAreaSize {
    #[default]
    One,
    Two,
    Three,
}
impl ScrollAreaSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<ScrollAreaSize> for Signal<Responsive<ScrollAreaSize>> {
    fn from(value: ScrollAreaSize) -> Self {
        Signal::stored(value.into())
    }
}

/// A themed provider for Viewport, Scrollbar, Thumb and Corner.
#[component]
pub fn Root(
    #[prop(optional, into)] scrollbars: Signal<Scrollbars>,
    #[prop(optional, into)] visibility: Signal<Visibility>,
    #[prop(default = Signal::stored(600), into)] hide_delay_ms: Signal<u32>,
    #[prop(optional, into)] boundary: Signal<Boundary>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional, into)] labelled_by: Signal<String>,
    #[prop(optional_no_strip, into)] direction: Option<Signal<Direction>>,
    #[prop(optional, into)] size: Signal<Responsive<ScrollAreaSize>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        styles.set("--rustify-scrollbar-size", &size.get(), |size| {
            format!(
                "var(--rustify-space-{}, {}px)",
                size.number(),
                match size {
                    ScrollAreaSize::One => 4,
                    ScrollAreaSize::Two => 8,
                    ScrollAreaSize::Three => 12,
                }
            )
        })?;
        if let Some(radius) = radius.get() {
            let value = match radius {
                ThemeRadius::None => "0px",
                ThemeRadius::Small => "calc(var(--rustify-space-1, 4px) * 0.5625)",
                ThemeRadius::Medium => "calc(var(--rustify-space-1, 4px) * 0.75)",
                ThemeRadius::Large => "calc(var(--rustify-space-1, 4px) * 1.125)",
                ThemeRadius::Full => "9999px",
            };
            styles.insert(0, "--rustify-scrollbar-radius", value)?;
        }
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! { <crate::primitives::scroll_area::Root scrollbars=scrollbars visibility=visibility hide_delay_ms=hide_delay_ms boundary=boundary
        aria_label=aria_label labelled_by=labelled_by direction=direction
        class=crate::macros::merge(format!("rustify-scroll-area {geometry}"),&class) test_id=test_id attr:data-name="ScrollArea">
        {children()}
    </crate::primitives::scroll_area::Root> }
}

/// A complete scrolling area. `test_id` identifies its native scrollport; class sizes the outer Root.
#[component]
pub fn ScrollArea(
    #[prop(optional, into)] scrollbars: Signal<Scrollbars>,
    #[prop(optional, into)] visibility: Signal<Visibility>,
    #[prop(default = Signal::stored(600), into)] hide_delay_ms: Signal<u32>,
    #[prop(optional, into)] boundary: Signal<Boundary>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional, into)] labelled_by: Signal<String>,
    #[prop(optional_no_strip, into)] direction: Option<Signal<Direction>>,
    #[prop(optional, into)] size: Signal<Responsive<ScrollAreaSize>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let root_id = if test_id.is_empty() {
        String::new()
    } else {
        format!("{test_id}-root")
    };
    view! { <Root scrollbars=scrollbars visibility=visibility hide_delay_ms=hide_delay_ms boundary=boundary aria_label=aria_label labelled_by=labelled_by
        direction=direction size=size radius=radius margin=margin class=class test_id=root_id>
        <Viewport test_id=test_id>{children()}</Viewport>
        <Scrollbar orientation=crate::Orientation::Horizontal><Thumb /></Scrollbar>
        <Scrollbar orientation=crate::Orientation::Vertical><Thumb /></Scrollbar>
        <Corner />
    </Root> }
}
