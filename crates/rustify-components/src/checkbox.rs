//! A box with two states, and the application holding which one.

use crate::icon::{Glyph, Icon};
use leptos::prelude::*;

const BOX: &str = "pointer-events-none absolute inset-0 flex items-center justify-center rounded-[4px] border border-border bg-input text-primary-foreground transition-colors peer-checked:bg-primary peer-checked:border-primary peer-focus-visible:ring-ring/50 peer-focus-visible:ring-[3px] peer-disabled:opacity-50 peer-aria-invalid:border-destructive peer-aria-invalid:ring-destructive/40 peer-aria-invalid:ring-[3px]";
const INPUT: &str =
    "peer absolute inset-0 size-full m-0 opacity-0 cursor-pointer disabled:cursor-not-allowed";

/// Rust/UI drew this as a `<button role="checkbox">`. This one is the
/// browser's own control, kept transparent above the box that is drawn for it:
/// the semantics, the keyboard and the form participation are the platform's,
/// and only the appearance is ours.
///
/// There is no read-only checkbox in HTML, so a read-only one takes the click
/// and puts the box back the way the application has it - the same answer the
/// SDK's own checkbox gives.
#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        crate::id::next("checkbox")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let node = NodeRef::<leptos::html::Input>::new();
    let settle = move || {
        if let Some(input) = node.get_untracked() {
            let held = checked.get_untracked();
            if input.checked() != held {
                input.set_checked(held);
            }
        }
    };
    view! {
        <span
            class=crate::macros::merge("relative inline-block size-4 shrink-0", &class)
            data-name="Checkbox"
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
        >
            <input
                node_ref=node
                type="checkbox"
                id=id
                class=INPUT
                data-testid=test_id
                aria-label=aria_label
                aria-invalid=move || invalid.get().then_some("true")
                aria-readonly=move || read_only.get().then_some("true")
                aria-describedby=move || {
                    let described_by = described_by.get();
                    (!described_by.is_empty()).then_some(described_by)
                }
                prop:checked=move || checked.get()
                prop:disabled=move || disabled.get()
                on:change:target=move |ev| {
                    let asked = ev.target().checked();
                    if !disabled.get_untracked() && !read_only.get_untracked() {
                        on_change(asked);
                    }
                    settle();
                }
            />
            <span class=BOX aria-hidden="true">
                <Show when=move || checked.get()>
                    <Icon glyph=Glyph::Check class="size-3" />
                </Show>
            </span>
        </span>
    }
}
