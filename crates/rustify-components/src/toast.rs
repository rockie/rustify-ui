//! A short status message, styled.
//!
//! The rules - one message at a time, a newer one replacing it, each hiding
//! itself on a timer the runtime owns - are the SDK's toast core. What is here
//! is how one looks: a card in the corner of the scope, a colour for its tone,
//! and a button that takes it down. The region it sits in is the core's too: a
//! polite live region in the overlay plane that is not a layer, so it takes no
//! focus, answers no Escape, and stays reachable while a modal is open.

use crate::icon::{Glyph, Icon};
use leptos::prelude::*;
use rustify_ui::{use_locale, use_toasts, Message, ToastRegion, ToastTone};

const REGION: &str = "fixed bottom-4 end-4 z-50 flex max-w-sm flex-col items-end gap-2";
const CARD: &str = "flex w-full items-start gap-3 rounded-md border border-border bg-popover px-4 py-3 text-sm text-popover-foreground shadow-lg";
const DISMISS: &str = "shrink-0 rounded-sm p-0.5 text-muted-foreground transition-colors cursor-pointer outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]";

/// The colour a tone adds. A tone is how a message looks and nothing else:
/// it does not change how long it stays or how it is announced.
fn tone(tone: ToastTone) -> &'static str {
    match tone {
        ToastTone::Neutral => "",
        ToastTone::Success => "border-s-4 border-s-success",
        ToastTone::Warning => "border-s-4 border-s-warning",
        ToastTone::Error => "border-s-4 border-s-destructive",
    }
}

/// One message, and the button that takes it down.
///
/// The button's name is the SDK's, in the scope's language: it is a word the
/// SDK puts on screen on its own account.
#[component]
pub fn Toast(
    toast: rustify_ui::Toast,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let toasts = use_toasts();
    let locale = use_locale();
    let roots = use_context::<rustify_ui::ScopeRoots>();
    let button = NodeRef::<leptos::html::Button>::new();
    let seq = toast.seq;
    let base = format!("{CARD} {}", tone(toast.tone));
    view! {
        <div
            class=crate::macros::merge(&base, &class)
            data-name="Toast"
            data-tone=toast.tone.key()
            data-testid=test_id.clone()
        >
            <p class="flex-1" data-name="ToastMessage">
                {toast.message}
            </p>
            <button
                type="button"
                class=DISMISS
                data-name="ToastDismiss"
                data-testid=format!("{test_id}-dismiss")
                aria-label=move || locale.text(Message::Dismiss)
                node_ref=button
                on:click=move |_| {
                    // The button takes itself away, and the keyboard on it
                    // would fall to the page. It stays in the scope instead,
                    // where the overlay stack puts it when a layer closes
                    // with nothing to go back to.
                    let holding = button.get_untracked().is_some_and(|button| {
                        let button: &leptos::web_sys::Element = button.as_ref();
                        document().active_element().as_ref() == Some(button)
                    });
                    if let (true, Some(roots)) = (holding, &roots) {
                        let _ = roots.container().focus();
                    }
                    toasts.dismiss(seq);
                }
            >
                <Icon glyph=Glyph::Close />
            </button>
        </div>
    }
}

/// Where the scope's messages are shown: the bottom corner at the end of the
/// line, above everything else in the scope.
///
/// Put one in the scope root, after `provide_toasts`, and show messages with
/// `use_toasts().show(..)`. The card is `test_id` followed by `-toast`.
#[component]
pub fn Toaster(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let card = format!("{test_id}-toast");
    let render = Callback::new(move |toast: rustify_ui::Toast| {
        view! { <Toast toast=toast test_id=card.clone() /> }.into_any()
    });
    view! {
        <ToastRegion
            render=render
            class=crate::macros::merge(REGION, &class)
            test_id=test_id
        />
    }
}
