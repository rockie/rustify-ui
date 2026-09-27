//! A value on a range, snapped to the steps the application declared.

use leptos::prelude::*;
use rustify_ui::snap;

/// Rust/UI's slider was an unbound `<input type="range">` with a stylesheet
/// that reached the whole page. The control is the browser's own here too, but
/// the value is the application's, snapped by the same rule the GPU half uses
/// so the two halves cannot offer values from different sets.
#[component]
pub fn Slider(
    #[prop(into)] value: Signal<f64>,
    on_change: impl Fn(f64) + 'static,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(100.0);
    let step = step.unwrap_or(1.0);
    let id = if id.is_empty() {
        crate::id::next("slider")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let node = NodeRef::<leptos::html::Input>::new();
    let settle = move || {
        if let Some(input) = node.get_untracked() {
            let held = snap(value.get_untracked(), min, max, step).to_string();
            if input.value() != held {
                input.set_value(&held);
            }
        }
    };
    view! {
        <input
            node_ref=node
            type="range"
            id=id
            class=crate::macros::merge("w-full", &class)
            data-name="Slider"
            data-testid=test_id
            min=min.to_string()
            max=max.to_string()
            step=step.to_string()
            aria-label=aria_label
            aria-readonly=move || read_only.get().then_some("true")
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            prop:value=move || snap(value.get(), min, max, step).to_string()
            prop:disabled=move || disabled.get()
            on:input:target=move |ev| {
                let asked = ev.target().value().parse::<f64>().unwrap_or(min);
                if !disabled.get_untracked() && !read_only.get_untracked() {
                    on_change(snap(asked, min, max, step));
                }
                settle();
            }
        />
    }
}
