//! A button that opens the browser's file picker.
//!
//! The input is real and it is the thing the user activates; the button beside
//! it is the label. That way the keyboard, the focus ring and the picker
//! itself are the platform's, and a person using a screen reader hears a file
//! control rather than a button that does something unexplained.

use leptos::prelude::*;

const FIELD: &str = "block w-full text-sm text-muted-foreground file:mr-3 file:rounded-md file:border file:border-border file:bg-secondary file:px-3 file:py-1.5 file:text-sm file:text-foreground file:cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:ring-ring/50 focus-visible:ring-[3px]";

/// A file input.
///
/// `accept` is what the picker offers, and it should be built from the same
/// limits the import checks against - `rustify_ui::files::Limits::accept_attribute`
/// exists so the two cannot drift. It is a courtesy either way: a person can
/// defeat any picker filter, so the check still runs on what arrives.
///
/// The input is cleared after every choice. Without that, choosing the same
/// file twice in a row fires no second event - the value has not changed - and
/// a person who fixed the file and picked it again would see nothing happen.
#[component]
pub fn FilePicker(
    on_files: impl Fn(Option<leptos::web_sys::FileList>) + 'static,
    #[prop(optional, into)] accept: Signal<String>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        crate::id::next("file")
    } else {
        id
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    view! {
        <input
            type="file"
            id=id
            class=crate::macros::merge(FIELD, &class)
            data-name="FilePicker"
            data-testid=test_id
            aria-label=aria_label
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            accept=move || {
                let accept = accept.get();
                (!accept.is_empty()).then_some(accept)
            }
            prop:disabled=move || disabled.get()
            on:change:target=move |event| {
                let input = event.target();
                on_files(input.files());
                // Cleared so the same file can be chosen twice.
                input.set_value("");
            }
        />
    }
}
