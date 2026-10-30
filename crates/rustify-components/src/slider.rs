//! Themed scalar and named multi-thumb projections of the same slider behavior.

use crate::{primitives, Orientation, Responsive};
use leptos::prelude::*;
pub use primitives::slider::{ThumbLabel, ThumbValue};
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};
crate::choice_style::visual_types!(SliderSize, SliderVariant);

fn geometry(
    size: Signal<Responsive<SliderSize>>,
    color: Signal<Option<AccentColor>>,
    radius: Signal<Option<ThemeRadius>>,
) -> String {
    let mode = crate::component_color::use_mode();
    crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let track = match size {
                SliderSize::One => "calc(var(--rustify-space-2, 8px) * 0.75)",
                SliderSize::Two => "var(--rustify-space-2, 8px)",
                SliderSize::Three => "calc(var(--rustify-space-2, 8px) * 1.25)",
            };
            styles.insert(width, "--rustify-slider-track", track)?;
        }
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
        if let Some(radius) = radius.get() {
            styles.insert(
                0,
                "--rustify-slider-radius",
                if radius == ThemeRadius::Full {
                    "9999px".into()
                } else {
                    format!(
                        "calc(var(--rustify-slider-track) * {})",
                        radius.factor() / 3.0
                    )
                },
            )?;
        }
        Ok(styles)
    })
}

#[component]
pub fn Root(
    #[prop(into)] values: Signal<Vec<ThumbValue>>,
    #[prop(into)] on_change: Callback<Vec<ThumbValue>>,
    #[prop(optional_no_strip, into)] on_commit: Option<Callback<Vec<ThumbValue>>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(default = Signal::stored(0.0), into)] min: Signal<f64>,
    #[prop(default = Signal::stored(100.0), into)] max: Signal<f64>,
    #[prop(default = Signal::stored(1.0), into)] step: Signal<f64>,
    #[prop(optional, into)] min_steps_between: Signal<u32>,
    #[prop(optional, into)] preserve_order: Signal<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] inverted: bool,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] size: Signal<Responsive<SliderSize>>,
    #[prop(optional, into)] variant: Signal<SliderVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = geometry(size, color, radius);
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-slider".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! { <primitives::slider::Root values=values on_change=on_change on_commit=on_commit on_cancel=on_cancel
        min=min max=max step=step min_steps_between=min_steps_between preserve_order=preserve_order
        orientation=orientation inverted=inverted id=id name=name form=form disabled=disabled read_only=read_only invalid=invalid
        described_by=described_by aria_label=aria_label class=Signal::derive(classes) test_id=test_id>
        {children()}
    </primitives::slider::Root> }.add_any_attr((
        leptos::attr::custom::custom_attribute("data-name", "Slider"),
        leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some(""))))
}
#[component]
pub fn Track(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <primitives::slider::Track class=format!("rustify-slider-track {class}")>{children()}</primitives::slider::Track> }
}
#[component]
pub fn Range(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitives::slider::Range class=format!("rustify-slider-range {class}") /> }
}
#[component]
pub fn Thumb(
    #[prop(into)] id: String,
    #[prop(optional, into)] control_id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] value_text: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <primitives::slider::Thumb id=id control_id=control_id aria_label=aria_label labelled_by=labelled_by
    value_text=value_text disabled=disabled class=format!("rustify-slider-thumb {class}") test_id=test_id /> }
}
/// Styles one caller-supplied native thumb without adding a second target.
pub fn thumb_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    attributes: primitives::slider::ThumbAttributes,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::slider::thumb_as(target, attributes).add_any_attr(
        leptos::tachys::html::class::class(("rustify-slider-thumb", true)),
    )
}
#[component]
fn ThumbSet(labels: Vec<ThumbLabel>, test_id: String) -> impl IntoView {
    let mut names = std::collections::HashMap::new();
    for label in labels {
        use std::collections::hash_map::Entry;
        match names.entry(label.id) {
            Entry::Vacant(entry) => {
                entry.insert(label.label);
            }
            Entry::Occupied(mut entry) => {
                entry.insert(String::new());
            }
        }
    }
    let names = StoredValue::new(names);
    let test_id = StoredValue::new(test_id);
    let order = primitives::slider::declaration_order();
    view! { <For each=move || order.get() key=|id| id.clone() let:id>
        <Thumb id=id.clone() aria_label=names.with_value(|names| names.get(&id).cloned().unwrap_or_default())
            test_id=format!("{}-{id}", test_id.get_value()) />
    </For> }
}
/// Named values are normalized for display, ARIA and submission without rewriting the application.
#[component]
pub fn RangeSlider(
    #[prop(into)] values: Signal<Vec<ThumbValue>>,
    thumbs: Vec<ThumbLabel>,
    #[prop(into)] on_change: Callback<Vec<ThumbValue>>,
    #[prop(optional_no_strip, into)] on_commit: Option<Callback<Vec<ThumbValue>>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(default = Signal::stored(0.0), into)] min: Signal<f64>,
    #[prop(default = Signal::stored(100.0), into)] max: Signal<f64>,
    #[prop(default = Signal::stored(1.0), into)] step: Signal<f64>,
    #[prop(optional, into)] min_steps_between: Signal<u32>,
    #[prop(optional, into)] preserve_order: Signal<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] inverted: bool,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] size: Signal<Responsive<SliderSize>>,
    #[prop(optional, into)] variant: Signal<SliderVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <Root values=values on_change=on_change on_commit=on_commit on_cancel=on_cancel min=min max=max step=step
        min_steps_between=min_steps_between preserve_order=preserve_order orientation=orientation inverted=inverted id=id name=name form=form
        disabled=disabled read_only=read_only invalid=invalid described_by=described_by aria_label=aria_label size=size variant=variant
        color=color high_contrast=high_contrast radius=radius class=class test_id=test_id.clone()>
        <Track><Range /><ThumbSet labels=thumbs test_id=test_id /></Track>
    </Root> }
}
/// The legacy scalar callback projected through a single stable `value` thumb.
#[component]
pub fn Slider(
    #[prop(into)] value: Signal<f64>,
    on_change: impl Fn(f64) + 'static,
    #[prop(optional_no_strip, into)] on_commit: Option<Callback<f64>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] inverted: bool,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] value_text: Signal<String>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] size: Signal<Responsive<SliderSize>>,
    #[prop(optional, into)] variant: Signal<SliderVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let values = Signal::derive(move || {
        vec![ThumbValue {
            id: "value".into(),
            value: value.get(),
        }]
    });
    let change = StoredValue::new_local(on_change);
    let change = Callback::new(move |values: Vec<ThumbValue>| {
        if let Some(value) = values.first() {
            change.with_value(|change| change(value.value));
        }
    });
    let commit = on_commit.map(|commit| {
        Callback::new(move |values: Vec<ThumbValue>| {
            if let Some(value) = values.first() {
                commit.run(value.value);
            }
        })
    });
    view! { <Root values=values on_change=change on_commit=commit on_cancel=on_cancel min=min.unwrap_or(0.0) max=max.unwrap_or(100.0) step=step.unwrap_or(1.0)
        orientation=orientation inverted=inverted name=name form=form disabled=disabled read_only=read_only invalid=invalid described_by=described_by
        size=size variant=variant color=color high_contrast=high_contrast radius=radius class=class
        test_id=if test_id.is_empty() { String::new() } else { format!("{test_id}-root") }>
        <Track><Range /><Thumb id="value" control_id=id aria_label=aria_label labelled_by=labelled_by value_text=value_text test_id=test_id /></Track>
    </Root> }
}
