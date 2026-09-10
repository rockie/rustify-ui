//! How far along something is, when anybody knows.

use leptos::prelude::*;

const TRACK: &str =
    "rui:relative rui:h-2 rui:w-full rui:overflow-hidden rui:rounded-full rui:bg-secondary";
const FILL: &str = "rui:h-full rui:bg-primary rui:transition-[width]";

/// Rust/UI's version set the bar's width through a `style` attribute, which a
/// strict policy refuses (P1 M3's lesson: a static `style` attribute is written
/// with `setAttribute` and blocked, while the `style:` directive goes through
/// the CSSOM and is not). The width here is a directive.
///
/// `indeterminate` is a separate prop rather than an absent value because the
/// two mean different things to a reader: a bar with no `aria-valuenow` says
/// "in progress, no idea how far", and one at zero says "not started".
#[component]
pub fn Progress(
    /// Optional, because a bar whose quantity is not known has no value to
    /// give: `indeterminate` with no `value` is the honest pair.
    #[prop(optional, into)]
    value: Signal<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let max = max.unwrap_or(100.0).max(f64::MIN_POSITIVE);
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let percent = move || (value.get() / max * 100.0).clamp(0.0, 100.0);
    view! {
        <div
            class=crate::macros::merge(TRACK, &class)
            data-name="Progress"
            data-testid=test_id
            data-state=move || if indeterminate.get() { "indeterminate" } else { "determinate" }
            role="progressbar"
            aria-label=aria_label
            aria-valuemin="0"
            aria-valuemax=max.to_string()
            aria-valuenow=move || (!indeterminate.get()).then(|| percent().to_string())
        >
            <div
                class=FILL
                aria-hidden="true"
                style:width=move || {
                    if indeterminate.get() { "100%".to_string() } else { format!("{}%", percent()) }
                }
                class=("rui:opacity-40", move || indeterminate.get())
            />
        </div>
    }
}
