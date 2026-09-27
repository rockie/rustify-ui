//! Two states, presented as a thing that moves rather than a box that fills.

use leptos::prelude::*;

const TRACK: &str = "inline-flex h-6 w-11 shrink-0 items-center rounded-full border-2 border-transparent transition-colors cursor-pointer outline-none bg-border aria-checked:bg-primary focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50";
const KNOB: &str =
    "pointer-events-none block size-5 rounded-full bg-background transition-transform";

/// Rust/UI's switch held its own state in an `RwSignal` and had no way to
/// report a change, so two of them bound to one value could disagree. This one
/// shows what the application holds and asks for the rest.
#[component]
pub fn Switch(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        crate::id::next("switch")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    view! {
        <button
            type="button"
            role="switch"
            id=id
            class=crate::macros::merge(TRACK, &class)
            data-name="Switch"
            data-testid=test_id
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            aria-label=aria_label
            aria-checked=move || checked.get().to_string()
            aria-readonly=move || read_only.get().then_some("true")
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            prop:disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get_untracked() && !read_only.get_untracked() {
                    on_change(!checked.get_untracked());
                }
            }
        >
            <span
                class=KNOB
                class=("translate-x-5", move || checked.get())
                aria-hidden="true"
            />
        </button>
    }
}
