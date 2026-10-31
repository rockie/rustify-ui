//! A segmented single choice using the native radio-group behavior.
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::ThemeRadius;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedControlSize {
    One,
    #[default]
    Two,
    Three,
}
impl SegmentedControlSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}
impl From<SegmentedControlSize> for Signal<Responsive<SegmentedControlSize>> {
    fn from(value: SegmentedControlSize) -> Self {
        Signal::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedControlVariant {
    #[default]
    Surface,
    Classic,
}
impl SegmentedControlVariant {
    pub const ALL: &'static [Self] = &[Self::Surface, Self::Classic];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Classic => "classic",
        }
    }
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] size: Signal<Responsive<SegmentedControlSize>>,
    #[prop(optional, into)] variant: Signal<SegmentedControlVariant>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let n = size.number();
            let (height, padding, font) = match n {
                1 => (24, 12, 12),
                3 => (40, 16, 16),
                _ => (32, 16, 14),
            };
            styles.insert(
                width,
                "--rustify-segment-height",
                format!("var(--rustify-space-{}, {height}px)", n + 4),
            )?;
            styles.insert(
                width,
                "--rustify-segment-padding",
                format!(
                    "var(--rustify-space-{}, {padding}px)",
                    if n == 1 { 3 } else { 4 }
                ),
            )?;
            styles.insert(
                width,
                "--rustify-segment-radius-base",
                if n == 3 {
                    "var(--rustify-radius-3, 6px)"
                } else {
                    "var(--rustify-radius-2, 4px)"
                },
            )?;
            styles.insert(
                width,
                "font-size",
                format!("var(--rustify-font-size-{n}, {font}px)"),
            )?;
        }
        if let Some(radius) = radius.get() {
            let scaling = theme.map_or(1., |t| t.get().config.selected.scaling.factor());
            styles.insert(
                0,
                "--rustify-segment-radius",
                if radius == ThemeRadius::Full {
                    "9999px".into()
                } else {
                    format!("{}px", 4. * radius.factor() * scaling)
                },
            )?;
        }
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-segmented-control".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! {<crate::primitives::radio_group::Root value=value on_change=on_change name=name form=form aria_label=aria_label
    disabled=disabled read_only=read_only required=required invalid=invalid class=Signal::derive(classes) test_id=test_id
    attr:data-variant=move ||variant.get().as_str()>{children()}</crate::primitives::radio_group::Root>}
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<crate::primitives::radio_group::Item value=value disabled=disabled class=crate::macros::merge("rustify-segmented-item",&class) control_class="rustify-segmented-input" test_id=test_id>
        <span class="rustify-segmented-label">{children()}</span>
    </crate::primitives::radio_group::Item>}
}
