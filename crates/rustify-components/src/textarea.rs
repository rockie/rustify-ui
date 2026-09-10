//! Several lines of text the application owns. Enter is a newline here;
//! nothing commits on it.

use leptos::prelude::*;

const BASE: &str = "rui:flex rui:min-h-16 rui:w-full rui:rounded-md rui:border rui:border-border rui:bg-input rui:text-foreground rui:px-3 rui:py-2 rui:text-sm rui:transition-colors rui:outline-none rui:placeholder:text-muted-foreground rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px] rui:disabled:cursor-not-allowed rui:disabled:opacity-50 rui:read-only:bg-muted rui:aria-invalid:border-destructive rui:aria-invalid:ring-destructive/40";

#[component]
pub fn TextArea(
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        crate::id::next("textarea")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let placeholder = (!placeholder.is_empty()).then_some(placeholder);
    let node = NodeRef::<leptos::html::Textarea>::new();
    let settle = move || {
        if let Some(area) = node.get_untracked() {
            let held = value.get_untracked();
            if area.value() != held {
                area.set_value(&held);
            }
        }
    };
    view! {
        <textarea
            node_ref=node
            id=id
            rows=rows
            class=crate::macros::merge(BASE, &class)
            data-name="TextArea"
            data-testid=test_id
            placeholder=placeholder
            aria-label=aria_label
            aria-invalid=move || invalid.get().then_some("true")
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            aria-readonly=move || read_only.get().then_some("true")
            prop:value=move || value.get()
            prop:disabled=move || disabled.get()
            prop:readOnly=move || read_only.get()
            on:input:target=move |ev| {
                let typed = ev.target().value();
                if !disabled.get_untracked() && !read_only.get_untracked() {
                    on_change(typed);
                }
                settle();
            }
        />
    }
}
