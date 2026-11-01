//! Themed parts and the convenience entry share one validated progress projection.
pub use crate::primitives::progress::ProgressValue;
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressSize {
    One,
    #[default]
    Two,
    Three,
}
impl ProgressSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<ProgressSize> for Signal<Responsive<ProgressSize>> {
    fn from(value: ProgressSize) -> Self {
        Signal::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}
impl ProgressVariant {
    pub const ALL: &'static [Self] = &[Self::Classic, Self::Surface, Self::Soft];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Surface => "surface",
            Self::Soft => "soft",
        }
    }
}
#[component]
pub fn Root(
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(default=Signal::stored(100.),into)] max: Signal<f64>,
    #[prop(optional_no_strip, into)] value_label: Option<Callback<ProgressValue, String>>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] size: Signal<Responsive<ProgressSize>>,
    #[prop(optional, into)] variant: Signal<ProgressVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(default=Signal::stored(5000),into)] duration_ms: Signal<u32>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let height = match size {
                ProgressSize::One => "var(--rustify-space-1, 4px)",
                ProgressSize::Two => "calc(var(--rustify-space-2, 8px) * 0.75)",
                ProgressSize::Three => "var(--rustify-space-2, 8px)",
            };
            styles.insert(width, "--rustify-progress-height", height)?;
        }
        if let Some(radius) = radius.get() {
            let scaling = theme.map_or(1., |theme| theme.get().config.selected.scaling.factor());
            styles.insert(
                0,
                "--rustify-progress-radius-factor",
                radius.factor().to_string(),
            )?;
            styles.insert(
                0,
                "--rustify-progress-radius-thumb",
                if matches!(radius, ThemeRadius::None | ThemeRadius::Small) {
                    format!("{}px", 0.5 * scaling)
                } else {
                    "9999px".into()
                },
            )?;
        }
        styles.insert(
            0,
            "--rustify-progress-duration",
            format!("{}ms", duration_ms.get()),
        )?;
        margin.get().write(&mut styles)?;
        let color = color.get();
        crate::component_color::set_color(
            &mut styles,
            color,
            if color.is_some() {
                mode.get()
            } else {
                ThemeMode::Light
            },
        )?;
        Ok(styles)
    });
    view! {<crate::primitives::progress::Root value=value max=max value_label=value_label aria_label=aria_label labelled_by=labelled_by
        class=crate::macros::merge(format!("rustify-color rustify-progress {geometry}"),&class) test_id=test_id
        attr:data-variant=move ||variant.get().as_str() attr:data-high-contrast=move ||high_contrast.get().then_some("")>
        {children()}
    </crate::primitives::progress::Root>}
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::progress::Indicator class=crate::macros::merge("rustify-progress-indicator",&class) test_id=test_id />}
}
#[component]
pub fn Progress(
    #[prop(optional, into)] value: Signal<f64>,
    #[prop(default=Signal::stored(100.),into)] max: Signal<f64>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional_no_strip, into)] value_label: Option<Callback<ProgressValue, String>>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] size: Signal<Responsive<ProgressSize>>,
    #[prop(optional, into)] variant: Signal<ProgressVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(default=Signal::stored(5000),into)] duration_ms: Signal<u32>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let value = Signal::derive(move || (!indeterminate.get()).then(|| value.get()));
    view! {<Root value=value max=max value_label=value_label aria_label=aria_label labelled_by=labelled_by size=size variant=variant
        color=color high_contrast=high_contrast radius=radius duration_ms=duration_ms margin=margin class=class test_id=test_id>
        <Indicator />
    </Root>}
}
