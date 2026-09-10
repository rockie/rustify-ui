//! The button that asks, and the line that says what happened.

use leptos::prelude::*;
use rustify_ui::Submit;

use super::provider::Form;

/// Asks the form to submit.
///
/// The button says whether asking would get anywhere, and `submit()` decides
/// what asking does - the same question asked twice, deliberately. It is
/// announced as unavailable rather than natively disabled, for two reasons
/// that are the same reason: a button the browser has switched off never sees
/// the click, so `submit()` could not be the authority the form needs it to
/// be, and a person who cannot reach it cannot be told why it is off.
#[component]
pub fn SubmitButton(
    form: Form,
    /// Every synchronous check, including the ones across fields. Run at the
    /// moment of asking rather than earlier, because a rule that compares two
    /// fields has no single field to run on.
    rules: impl Fn() -> Vec<(&'static str, String)> + Send + Sync + 'static,
    /// What the application is told when asking got somewhere it should hear
    /// about: `Busy`, `Waiting`, or a `Blocked` whose field already has the
    /// keyboard. `Save` never arrives here - the form carries it out.
    #[prop(optional)]
    on_asked: Option<Callback<Submit>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let test_id = if test_id.is_empty() {
        "submit".to_string()
    } else {
        test_id
    };
    view! {
        <crate::button::Button
            test_id=test_id
            class=class
            unavailable=Signal::derive(move || !form.can_submit())
            on_click=move || {
                let asked = form.submit(&rules);
                if let Some(on_asked) = on_asked {
                    on_asked.run(asked);
                }
            }
        >
            {children()}
        </crate::button::Button>
    }
}

/// What the form is doing, and what went wrong if something did.
///
/// It is a live region: a save that failed while the keyboard was elsewhere is
/// not something a person should have to go looking for.
#[component]
pub fn FormStatus(
    form: Form,
    /// The words, in the application's language. M7 replaces these with the
    /// SDK's own catalogue.
    #[prop(optional, into)]
    saving: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let saving = if saving.is_empty() {
        "saving…".to_string()
    } else {
        saving
    };
    let test_id = if test_id.is_empty() {
        "form-status".to_string()
    } else {
        test_id
    };
    view! {
        <p
            class=crate::macros::merge("rui:text-sm rui:text-muted-foreground", &class)
            data-name="FormStatus"
            data-testid=test_id
            data-state=move || {
                if form.submitting() {
                    "saving"
                } else if form.failure().is_some() {
                    "failed"
                } else if form.dirty() {
                    "unsaved"
                } else {
                    "saved"
                }
            }
            role="status"
            aria-live="polite"
        >
            {move || match (form.submitting(), form.failure()) {
                (true, _) => saving.clone(),
                (false, Some(failure)) => failure,
                (false, None) => String::new(),
            }}
        </p>
    }
}
