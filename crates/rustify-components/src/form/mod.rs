//! Forms: the bookkeeping is the SDK's (`rustify_ui::FormState`), the values
//! are the application's, and what is here is the wiring between them and the
//! accessibility tree.

pub mod field;
pub mod provider;
#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
mod ssr_field;
pub mod submit;

pub use field::{Field, FieldBinding};
pub use provider::{provide_form, use_form, Form};
pub use submit::{FormStatus, SubmitButton};

pub use rustify_ui::FieldErrorSource;

/// The themed native form boundary; submit/reset behavior is shared with primitives.
#[leptos::component]
pub fn FormRoot(
    form: Form,
    rules: impl Fn() -> Vec<(&'static str, String)> + Send + Sync + 'static,
    #[prop(optional_no_strip, into)] on_reset: Option<leptos::prelude::Callback<()>>,
    #[prop(optional_no_strip, into)] on_asked: Option<
        leptos::prelude::Callback<rustify_ui::Submit>,
    >,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <crate::primitives::form::Root form=form rules=rules on_reset=on_reset on_asked=on_asked
        id=id aria_label=aria_label class=format!("flex flex-col gap-4 {class}") test_id=test_id>
        {children()}
    </crate::primitives::form::Root> }
}
