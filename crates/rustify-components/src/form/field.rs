//! A label, a control and the error that belongs to it, wired together.
//!
//! The wiring is the point. A label that names its control, an
//! `aria-invalid` that matches the error on screen, and an
//! `aria-describedby` that points at text which is actually there - three
//! things every form needs and every form gets slightly wrong. The control is
//! the caller's; what it is handed is the three answers.

use leptos::prelude::*;

use super::provider::Form;

/// What a control has to be told to take part in a form.
#[derive(Clone, Copy)]
pub struct FieldBinding {
    id: StoredValue<String>,
    described_by: Signal<String>,
    invalid: Signal<bool>,
}

impl FieldBinding {
    /// The id the label points at.
    pub fn id(&self) -> String {
        self.id.get_value()
    }

    /// The id of the error text, and empty when there is none: a control that
    /// points at an element which is not on the page describes nothing.
    pub fn described_by(&self) -> Signal<String> {
        self.described_by
    }

    pub fn invalid(&self) -> Signal<bool> {
        self.invalid
    }
}

#[component]
pub fn Field(
    form: Form,
    field: &'static str,
    #[prop(into)] label: String,
    /// The control this field is about, given what it needs to take part.
    control: impl Fn(FieldBinding) -> AnyView + Send + Sync + 'static,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let control_id = StoredValue::new(form.control_id(field));
    let error_id = StoredValue::new(form.error_id(field));
    let error = Memo::new(move |_| form.error(field));
    let binding = FieldBinding {
        id: control_id,
        described_by: Signal::derive(move || {
            if error.get().is_some() {
                error_id.get_value()
            } else {
                String::new()
            }
        }),
        invalid: Signal::derive(move || error.get().is_some()),
    };
    view! {
        <div
            class=crate::macros::merge("rui:flex rui:flex-col rui:gap-1.5", &class)
            data-name="Field"
            data-testid=format!("field-{field}")
            data-invalid=move || error.get().is_some().then_some("true")
        >
            <crate::label::Label control=control_id.get_value() test_id=format!("label-{field}")>
                {label}
            </crate::label::Label>
            {control(binding)}
            <Show when=move || error.get().is_some() fallback=|| ()>
                <p
                    id=move || error_id.get_value()
                    class="rui:text-sm rui:text-destructive"
                    data-name="FieldError"
                    data-testid=format!("error-{field}")
                >
                    {move || error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
