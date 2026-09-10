//! One line of text the application owns.

use leptos::prelude::*;

const BASE: &str = "rui:flex rui:h-9 rui:w-full rui:min-w-0 rui:rounded-md rui:border rui:border-border rui:bg-input rui:text-foreground rui:px-3 rui:py-1 rui:text-sm rui:transition-colors rui:outline-none rui:placeholder:text-muted-foreground rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px] rui:disabled:cursor-not-allowed rui:disabled:opacity-50 rui:read-only:bg-muted rui:aria-invalid:border-destructive rui:aria-invalid:ring-destructive/40";

/// Which keyboard the browser offers and which value it will accept.
///
/// Rust/UI's list had fifteen; these are the ones with a keyboard or a
/// validation rule of their own. `File` and `Color` are their own controls and
/// are not text fields, and `Hidden` is not a control at all.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextKind {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Search,
    Tel,
    Url,
}

impl TextKind {
    fn attribute(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// A text field whose value is the application's.
///
/// After every keystroke the control is put back in step with what the
/// application holds: a value it refused is not left on screen pretending to
/// have been taken. That is the same rule the SDK's own `TextField` follows,
/// and the reason both can be bound to one value without disagreeing.
#[component]
pub fn TextField(
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional)] kind: TextKind,
    /// The id other elements point at - a `Label`'s `control`, a form's error
    /// message. Generated when the caller has no opinion.
    #[prop(optional, into)]
    id: String,
    /// The accessible name, when no `Label` provides one.
    #[prop(optional, into)]
    aria_label: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    /// The id of the text that explains the value - an error, a hint.
    #[prop(optional, into)]
    described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        crate::id::next("field")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let placeholder = (!placeholder.is_empty()).then_some(placeholder);
    let node = NodeRef::<leptos::html::Input>::new();
    // Whether an input method is composing in this field right now.
    let composing = StoredValue::new(false);
    // Two handlers report now - an ordinary keystroke and the end of a
    // composition - and the application gave us one callback.
    let on_change = std::rc::Rc::new(on_change);
    let on_composed = std::rc::Rc::clone(&on_change);
    let settle = move || {
        if let Some(input) = node.get_untracked() {
            let held = value.get_untracked();
            if input.value() != held {
                input.set_value(&held);
            }
        }
    };
    view! {
        <input
            node_ref=node
            type=kind.attribute()
            id=id
            class=crate::macros::merge(BASE, &class)
            data-name="TextField"
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
