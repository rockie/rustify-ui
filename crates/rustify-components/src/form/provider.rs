//! The handle an application holds, and the one the components read.
//!
//! The save lives in here rather than at the call site on purpose. "At most
//! one save per request" is the property the whole state machine exists for,
//! and a request can be granted at two different moments - straight away, or
//! when the last validation lands - so an application that had to run the save
//! itself would have to remember both. It does not: it says what saving means
//! once, and the two moments are this module's problem.

use leptos::prelude::*;
use rustify_ui::{FormState, Generation, Submit};
use std::sync::Arc;

/// One form, as an application and its components both see it.
#[derive(Clone, Copy)]
pub struct Form {
    state: RwSignal<FormState>,
    /// Distinguishes two forms on one page whose fields are named the same.
    group: StoredValue<String>,
    save: StoredValue<Arc<dyn Fn() + Send + Sync>>,
}

impl Form {
    /// `fields` in the order a person meets them: "the first error" means the
    /// first of these, which is where the keyboard goes.
    pub fn new(fields: &'static [&'static str], save: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            state: RwSignal::new(FormState::new(fields)),
            group: StoredValue::new(crate::id::next("form")),
            save: StoredValue::new(Arc::new(save)),
        }
    }

    /// The id of a field's control. A `Label` points at it and the keyboard is
    /// moved to it; both need the same answer, so there is one.
    pub fn control_id(&self, field: &'static str) -> String {
        format!("{}-{field}", self.group.get_value())
    }

    /// The id of the element holding a field's error text.
    pub fn error_id(&self, field: &'static str) -> String {
        format!("{}-{field}-error", self.group.get_value())
    }

    /// The application took a new value for this field.
    pub fn changed(&self, field: &'static str) {
        self.state.update(|state| state.changed(field));
    }

    /// A validation is starting; the generation it answers for.
    pub fn validating(&self, field: &'static str) -> Generation {
        let mut generation = Generation::default();
        self.state
            .update(|state| generation = state.validating(field));
        generation
    }

    /// A validation finished. A waiting submit is carried out here if this was
    /// the answer it was waiting for.
    pub fn validated(&self, field: &'static str, generation: Generation, error: Option<String>) {
        let resolved = self
            .state
            .try_update(|state| state.validated(field, generation, error))
            .flatten();
        if let Some(resolved) = resolved {
            self.carry_out(resolved);
        }
    }

    /// An error from a rule the application ran itself.
    pub fn set_error(&self, field: &'static str, error: Option<String>) {
        self.state.update(|state| state.set_error(field, error));
    }

    /// Ask to submit. `rules` is every synchronous check, including the ones
    /// across fields.
    pub fn submit(&self, rules: impl FnOnce() -> Vec<(&'static str, String)>) -> Submit {
        let mut asked = Submit::Busy;
        self.state.update(|state| asked = state.submit(rules));
        self.carry_out(asked.clone());
        asked
    }

    /// The save finished.
    pub fn submitted(&self, result: Result<(), String>) {
        self.state.update(|state| state.submitted(result));
    }

    pub fn error(&self, field: &'static str) -> Option<String> {
        self.state
            .with(|state| state.error(field).map(str::to_string))
    }

    pub fn can_submit(&self) -> bool {
        self.state.with(FormState::can_submit)
    }

    pub fn submitting(&self) -> bool {
        self.state.with(FormState::submitting)
    }

    pub fn dirty(&self) -> bool {
        self.state.with(FormState::dirty)
    }

    /// Why the last save failed, until something changes or it is asked again.
    pub fn failure(&self) -> Option<String> {
        self.state.with(|state| state.failure().map(str::to_string))
    }

    pub fn fields(&self) -> Vec<&'static str> {
        self.state.with(|state| state.fields().to_vec())
    }

    /// Saving, or moving the keyboard to the first thing that is wrong.
    /// `Busy` and `Waiting` are answers, not actions.
    fn carry_out(&self, asked: Submit) {
        match asked {
            Submit::Save => self.save.with_value(|save| save()),
            Submit::Blocked { first_error } => crate::dom::focus_id(&self.control_id(first_error)),
            Submit::Busy | Submit::Waiting => {}
        }
    }
}

/// Publishes a form to the components below it.
pub fn provide_form(form: Form) {
    provide_context(form);
}

pub fn use_form() -> Option<Form> {
    use_context::<Form>()
}
