//! One of several, and the application holding which.
//!
//! Rust/UI's version put the chosen value in a context signal the item wrote
//! to directly, which makes the group its own owner. Here the group takes the
//! value and reports the request, like every other control in this crate, and
//! the items are the browser's own radios: one tab stop for the group and the
//! arrow keys moving within it are the platform's, not a re-implementation.

use crate::icon::{Glyph, Icon};
use leptos::prelude::*;

const DOT: &str = "pointer-events-none absolute inset-0 flex items-center justify-center rounded-full border border-border bg-input text-primary transition-colors peer-checked:border-primary peer-focus-visible:ring-ring/50 peer-focus-visible:ring-[3px] peer-disabled:opacity-50";
const INPUT: &str =
    "peer absolute inset-0 size-full m-0 opacity-0 cursor-pointer disabled:cursor-not-allowed";

/// One choice in a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A group of choices, exactly one of which the application holds.
#[component]
pub fn RadioGroup(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] options: Signal<Vec<RadioOption>>,
    on_change: impl Fn(String) + Send + Sync + 'static,
    /// The name that groups the radios for the browser. Two groups on one page
    /// must not share it, so it is generated unless the caller says.
    #[prop(optional, into)]
    name: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let name = if name.is_empty() {
        crate::id::next("radio")
    } else {
        name
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let on_change = std::sync::Arc::new(on_change);
    let group_test_id = test_id.clone();
    view! {
        <div
            class=crate::macros::merge("flex flex-col gap-3", &class)
            data-name="RadioGroup"
            data-testid=group_test_id
            role="radiogroup"
            aria-label=aria_label
            aria-readonly=move || read_only.get().then_some("true")
        >
            <For each=move || options.get() key=|option| option.value.clone() let:option>
                {
                    let RadioOption { value: option_value, label, disabled: option_disabled } = option;
                    let name = name.clone();
                    let id = crate::id::next("radio-item");
                    let mine = option_value.clone();
                    let checked = Memo::new(move |_| value.get() == mine);
                    let node = NodeRef::<leptos::html::Input>::new();
                    // A read-only radio has taken the click by the time we see
                    // it: the browser moved the dot, so put it back where the
                    // application has it.
                    let settle = move || {
                        if let Some(input) = node.get_untracked() {
                            let held = checked.get_untracked();
                            if input.checked() != held {
                                input.set_checked(held);
                            }
                        }
                    };
                    let on_change = on_change.clone();
                    let asked = option_value.clone();
                    let test_id = format!("{test_id}-{option_value}");
                    view! {
                        <label class="flex items-center gap-2 text-sm text-foreground">
                            <span
                                class="relative inline-block size-4 shrink-0"
                                data-name="RadioItem"
                                data-value=option_value
                                data-state=move || if checked.get() { "checked" } else { "unchecked" }
                            >
                                <input
                                    node_ref=node
                                    type="radio"
                                    id=id
                                    name=name
                                    class=INPUT
                                    data-testid=test_id
                                    prop:checked=move || checked.get()
                                    prop:disabled=move || disabled.get() || option_disabled
                                    on:change=move |_| {
                                        if !disabled.get_untracked() && !read_only.get_untracked()
                                            && !option_disabled
                                        {
                                            on_change(asked.clone());
                                        }
                                        settle();
                                    }
                                />
                                <span class=DOT aria-hidden="true">
                                    <Show when=move || checked.get()>
                                        <Icon glyph=Glyph::Dot class="size-3.5" />
                                    </Show>
                                </span>
                            </span>
                            {label}
                        </label>
                    }
                }
            </For>
        </div>
    }
}
