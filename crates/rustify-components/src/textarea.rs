//! Several lines of text the application owns. Enter is a newline here;
//! nothing commits on it.

use leptos::prelude::*;

const BASE: &str = "flex min-h-16 w-full rounded-md border border-border bg-input text-foreground px-3 py-2 text-sm transition-colors outline-none placeholder:text-muted-foreground focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 read-only:bg-muted aria-invalid:border-destructive aria-invalid:ring-destructive/40";

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
    // Whether an input method is composing in this field right now.
    let composing = StoredValue::new(false);
    // Two handlers report now - an ordinary keystroke and the end of a
    // composition - and the application gave us one callback.
    let on_change = std::rc::Rc::new(on_change);
    let on_composed = std::rc::Rc::clone(&on_change);
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
            on:compositionstart=move |_| composing.set_value(true)
            on:compositionend:target=move |ev| {
                // The composition is over and the user has chosen. This is the
                // first moment the field holds a value rather than the keys
                // that were being used to look one up.
                composing.set_value(false);
                let chosen = ev.target().value();
                if !disabled.get_untracked() && !read_only.get_untracked() {
                    on_composed(chosen);
                }
                settle();
            }
            on:input:target=move |ev| {
                // A composition in flight is not a value. An input method puts
                // the keys being used to *look up* a character into the field -
                // pinyin, a bopomofo string, a partial Hangul syllable - and
                // reporting those to the application would name an object
                // "gongzuo" and leave it named that if the user changed their
                // mind. Settling is skipped for the same reason: writing the
                // held value back into the element mid-composition takes the
                // input method's own text away from it.
                if composing.get_value() {
                    return;
                }
                let typed = ev.target().value();
                if !disabled.get_untracked() && !read_only.get_untracked() {
                    on_change(typed);
                }
                settle();
            }
        />
    }
}
