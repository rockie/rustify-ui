//! What a form knows that its fields do not.
//!
//! Three things, and they are the three that go wrong when every form writes
//! them again: which validation result is still the current one, whether a
//! submit that arrived during a validation is still the submit the user asked
//! for, and how many times a save can happen when the button is clicked
//! twenty times.
//!
//! **This module holds no values.** The application already holds them - that
//! is what a controlled component means - and a second copy would be a second
//! answer to "what is in this field", which is the bug P1 spent a milestone
//! removing from its controls. What lives here is the bookkeeping around the
//! values: a generation and source-specific errors per field, what is pending, and
//! whether a submit is waiting on it. Rules and saving are the application's
//! functions, called from here at the moments the ordering matters.

use crate::diagnostics::{note, record, ErrorKind};
use std::collections::BTreeMap;

/// How many times a field's value has changed.
///
/// A validation carries the generation it was started for and is ignored if
/// the field has moved on, which is the whole of the out-of-order problem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(u64);

impl Generation {
    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// The source of a field error, in display priority order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldErrorSource {
    Constraint,
    Rule,
    Check,
    Application,
}
impl FieldErrorSource {
    pub const ALL: &'static [Self] =
        &[Self::Constraint, Self::Rule, Self::Check, Self::Application];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constraint => "constraint",
            Self::Rule => "rule",
            Self::Check => "check",
            Self::Application => "application",
        }
    }
    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug, Default)]
struct Field {
    generation: Generation,
    errors: [Option<String>; 4],
    /// The generation a validation is running for, if one is.
    pending: Option<Generation>,
}

/// What asking to submit turned into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Submit {
    /// A save is already in flight. Nothing was started; this is what makes
    /// twenty clicks one save.
    Busy,
    /// Something is wrong now. The named field is the first one in field
    /// order, which is the one to move the keyboard to.
    Blocked { first_error: &'static str },
    /// A validation has to finish first. The answer arrives from
    /// [`FormState::validated`] when the last one does.
    Waiting,
    /// Save. Exactly once per request; the application calls
    /// [`FormState::submitted`] with what happened.
    Save,
}

/// A field nobody declared.
///
/// Recorded rather than ignored: a misspelled name is a control that never
/// validates and a form that never becomes dirty, and neither of those looks
/// like a typo from the outside.
fn unknown(field: &'static str) {
    record(note(ErrorKind::UnknownField, "the form has no such field").about_field(field));
}

/// The bookkeeping of one form.
#[derive(Clone, Debug)]
pub struct FormState {
    /// In the order a person meets them: "the first error" means the first in
    /// this order, not the first one a rule happened to report.
    order: Vec<&'static str>,
    fields: BTreeMap<&'static str, Field>,
    submitting: bool,
    /// The generations a waiting submit was asked for. If any field moves on
    /// from these, the user changed something and this request is not the one
    /// they asked for any more.
    request: Option<BTreeMap<&'static str, Generation>>,
    dirty: bool,
    /// What the last save failed with. Cleared by asking again.
    failure: Option<String>,
}

impl FormState {
    pub fn new(fields: &[&'static str]) -> Self {
        Self {
            order: fields.to_vec(),
            fields: fields
                .iter()
                .map(|name| (*name, Field::default()))
                .collect(),
            submitting: false,
            request: None,
            dirty: false,
            failure: None,
        }
    }

    pub fn fields(&self) -> &[&'static str] {
        &self.order
    }

    /// The application took a new value for this field.
    ///
    /// The field moves on: its error and any validation running for it are
    /// about a value that no longer exists, and a submit waiting on that
    /// validation is waiting for a form the user has since changed.
    pub fn changed(&mut self, field: &'static str) {
        let Some(entry) = self.fields.get_mut(field) else {
            unknown(field);
            return;
        };
        entry.generation = entry.generation.next();
        entry.errors = Default::default();
        entry.pending = None;
        self.dirty = true;
        self.failure = None;
        // The request was for the form as it was. Asking again is the user's
        // to do, now that they have changed it.
        self.request = None;
    }

    /// The generation a validation started now would be answering for.
    pub fn generation(&self, field: &'static str) -> Generation {
        self.fields
            .get(field)
            .map(|entry| entry.generation)
            .unwrap_or_default()
    }

    /// A validation has been started for this field's current value.
    pub fn validating(&mut self, field: &'static str) -> Generation {
        let Some(entry) = self.fields.get_mut(field) else {
            unknown(field);
            return Generation::default();
        };
        entry.pending = Some(entry.generation);
        entry.generation
    }

    /// A validation finished.
    ///
    /// A result for a generation the field has moved on from is dropped: it
    /// describes a value that is not on screen. The return value is what a
    /// submit that was waiting has become, and is `None` when none was.
    pub fn validated(
        &mut self,
        field: &'static str,
        generation: Generation,
        error: Option<String>,
    ) -> Option<Submit> {
        let Some(entry) = self.fields.get_mut(field) else {
            unknown(field);
            return None;
        };
        if entry.generation != generation {
            return None;
        }
        if entry.pending == Some(generation) {
            entry.pending = None;
        }
        entry.errors[FieldErrorSource::Check.index()] = error;
        self.resume()
    }

    /// Replaces only the synchronous Rule error, preserving other sources.
    pub fn set_error(&mut self, field: &'static str, error: Option<String>) {
        self.set_source_error(field, FieldErrorSource::Rule, error);
    }

    /// Replaces only the native constraint result for the accepted control.
    pub fn set_constraint_error(&mut self, field: &'static str, error: Option<String>) {
        self.set_source_error(field, FieldErrorSource::Constraint, error);
    }

    /// An application error remains until replaced or the accepted value changes.
    pub fn set_application_error(&mut self, field: &'static str, error: Option<String>) {
        self.set_source_error(field, FieldErrorSource::Application, error);
    }

    fn set_source_error(
        &mut self,
        field: &'static str,
        source: FieldErrorSource,
        error: Option<String>,
    ) {
        match self.fields.get_mut(field) {
            Some(entry) => entry.errors[source.index()] = error,
            None => unknown(field),
        }
    }
    fn error_entry(&self, field: &str) -> Option<(FieldErrorSource, &str)> {
        let entry = self.fields.get(field)?;
        FieldErrorSource::ALL
            .iter()
            .copied()
            .zip(&entry.errors)
            .find_map(|(source, error)| error.as_deref().map(|error| (source, error)))
    }
    pub fn error(&self, field: &str) -> Option<&str> {
        self.error_entry(field).map(|(_, error)| error)
    }
    /// The source of the same error returned by [`Self::error`].
    pub fn error_source(&self, field: &str) -> Option<FieldErrorSource> {
        self.error_entry(field).map(|(source, _)| source)
    }

    /// Clears bookkeeping without changing application values. A save in flight
    /// refuses reset and returns false; otherwise all old check generations expire.
    pub fn reset(&mut self) -> bool {
        if self.submitting {
            return false;
        }
        for entry in self.fields.values_mut() {
            entry.generation = entry.generation.next();
            entry.errors = Default::default();
            entry.pending = None;
        }
        self.request = None;
        self.dirty = false;
        self.failure = None;
        true
    }

    /// The first field in field order with an error.
    pub fn first_error(&self) -> Option<&'static str> {
        self.order
            .iter()
            .copied()
            .find(|name| self.error(name).is_some())
    }

    /// Why the last save failed, until something changes or it is asked again.
    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }

    pub fn is_validating(&self) -> bool {
        self.fields.values().any(|entry| entry.pending.is_some())
    }

    pub fn submitting(&self) -> bool {
        self.submitting
    }

    /// Whether a submit is waiting on a validation.
    pub fn waiting(&self) -> bool {
        self.request.is_some()
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Whether asking now would get anywhere. One definition, in one place:
    /// the submit button's state and the decision inside `submit` are the same
    /// question asked by two callers.
    pub fn can_submit(&self) -> bool {
        !self.submitting && !self.is_validating() && self.first_error().is_none()
    }

    /// Ask to submit.
    ///
    /// `rules` is the application's: every synchronous check, including the
    /// ones across fields, run here rather than earlier, because a rule that
    /// compares two fields has no single field to run on.
    pub fn submit(&mut self, rules: impl FnOnce() -> Vec<(&'static str, String)>) -> Submit {
        if self.submitting {
            return Submit::Busy;
        }
        self.failure = None;
        // The rules replace what the last run of them said - a rule that no
        // longer fails must not leave its error behind - and nothing else.
        for entry in self.fields.values_mut() {
            entry.errors[FieldErrorSource::Rule.index()] = None;
        }
        for (field, error) in rules() {
            self.set_error(field, Some(error));
        }
        if let Some(first_error) = self.first_error() {
            self.request = None;
            return Submit::Blocked { first_error };
        }
        if self.is_validating() {
            // Bound to the form as it is now. A field that moves on voids it.
            self.request = Some(
                self.order
                    .iter()
                    .map(|name| (*name, self.generation(name)))
                    .collect(),
            );
            return Submit::Waiting;
        }
        self.submitting = true;
        Submit::Save
    }

    /// The save finished.
    pub fn submitted(&mut self, result: Result<(), String>) {
        if !self.submitting {
            return;
        }
        self.submitting = false;
        match result {
            Ok(()) => {
                self.dirty = false;
                self.failure = None;
            }
            // The values are the application's and are untouched; what is kept
            // here is why, so the form can offer to try again.
            Err(reason) => self.failure = Some(reason),
        }
    }

    /// What a waiting request has become, now that a validation has landed.
    fn resume(&mut self) -> Option<Submit> {
        let request = self.request.as_ref()?;
        // Every field still at the generation the request was made for. A
        // changed field clears the request outright (see `changed`), so this
        // is belt and braces rather than the main path.
        if request
            .iter()
            .any(|(name, generation)| self.generation(name) != *generation)
        {
            self.request = None;
            return None;
        }
        if self.is_validating() {
            return None;
        }
        self.request = None;
        match self.first_error() {
            Some(first_error) => Some(Submit::Blocked { first_error }),
            None => {
                self.submitting = true;
                Some(Submit::Save)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldErrorSource, FormState, Submit};

    /// A ten-field form, which is what V5 asks about.
    fn form() -> FormState {
        FormState::new(&[
            "name", "email", "phone", "street", "city", "postcode", "country", "notes", "start",
            "end",
        ])
    }

    fn required(field: &'static str) -> Vec<(&'static str, String)> {
        vec![(field, format!("{field} is required"))]
    }

    #[test]
    fn a_new_form_is_clean_and_ready() {
        let state = form();
        assert!(state.can_submit());
        assert!(!state.dirty());
        assert_eq!(state.first_error(), None);
    }

    #[test]
    fn a_change_clears_what_was_said_about_the_old_value() {
        let mut state = form();
        state.set_error("email", Some("not an address".to_string()));
        assert_eq!(state.error("email"), Some("not an address"));

        state.changed("email");
        assert_eq!(state.error("email"), None);
        assert!(state.dirty());
    }

    #[test]
    fn twenty_validations_arriving_out_of_order_leave_the_latest_one_showing() {
        let mut state = form();
        // Twenty edits, each starting a validation, and every result arriving
        // in exactly the wrong order.
        let mut started = Vec::new();
        for round in 0..20 {
            state.changed("email");
            let generation = state.validating("email");
            started.push((generation, format!("round {round}")));
        }
        for (generation, message) in started.iter().rev().skip(1) {
            state.validated("email", *generation, Some(message.clone()));
        }
        // Nineteen stale answers changed nothing.
        assert_eq!(state.error("email"), None);
        assert!(state.is_validating());

        let (generation, message) = started.last().unwrap();
        state.validated("email", *generation, Some(message.clone()));
        assert_eq!(state.error("email"), Some(message.as_str()));
        assert!(!state.is_validating());
    }

    #[test]
    fn a_failed_rule_names_the_first_field_in_field_order_not_the_first_rule_to_fail() {
        let mut state = form();
        // Reported last-to-first on purpose: the order a person meets the
        // fields in is the order the keyboard should go to.
        let asked = state.submit(|| {
            vec![
                ("notes", "too long".to_string()),
                ("email", "not an address".to_string()),
            ]
        });
        assert_eq!(
            asked,
            Submit::Blocked {
                first_error: "email"
            }
        );
        assert!(!state.can_submit());
    }

    #[test]
    fn a_rule_that_stops_failing_stops_showing() {
        let mut state = form();
        assert!(matches!(
            state.submit(|| required("email")),
            Submit::Blocked { .. }
        ));
        state.changed("email");
        assert_eq!(state.submit(Vec::new), Submit::Save);
        assert_eq!(state.error("email"), None);
    }

    #[test]
    fn submitting_while_a_validation_runs_waits_and_then_saves_once() {
        let mut state = form();
        state.changed("email");
        let generation = state.validating("email");

        assert_eq!(state.submit(Vec::new), Submit::Waiting);
        assert!(state.waiting());
        assert!(!state.submitting(), "nothing is saved while waiting");

        assert_eq!(
            state.validated("email", generation, None),
            Some(Submit::Save)
        );
        assert!(state.submitting());
        assert!(!state.waiting());
    }

    #[test]
    fn a_validation_that_fails_turns_a_waiting_submit_into_a_block() {
        let mut state = form();
        state.changed("email");
        let generation = state.validating("email");
        assert_eq!(state.submit(Vec::new), Submit::Waiting);

        assert_eq!(
            state.validated("email", generation, Some("taken".to_string())),
            Some(Submit::Blocked {
                first_error: "email"
            })
        );
        assert!(!state.submitting(), "a failed check saves nothing");
        assert!(!state.waiting());
    }

    #[test]
    fn changing_a_field_while_a_submit_waits_voids_the_submit() {
        let mut state = form();
        state.changed("email");
        let generation = state.validating("email");
        assert_eq!(state.submit(Vec::new), Submit::Waiting);

        // The user carried on typing. What they asked to submit is not what
        // is on screen any more.
        state.changed("phone");
        assert!(!state.waiting());
        assert_eq!(state.validated("email", generation, None), None);
        assert!(!state.submitting(), "the voided request saves nothing");
    }

    #[test]
    fn twenty_clicks_while_a_save_runs_are_one_save() {
        let mut state = form();
        assert_eq!(state.submit(Vec::new), Submit::Save);
        for _ in 0..20 {
            assert_eq!(state.submit(Vec::new), Submit::Busy);
        }
        state.submitted(Ok(()));
        assert!(!state.submitting());
        assert!(!state.dirty());
    }

    #[test]
    fn a_save_that_fails_keeps_the_form_dirty_and_says_why() {
        let mut state = form();
        state.changed("email");
        assert_eq!(state.submit(Vec::new), Submit::Save);
        state.submitted(Err("the server said no".to_string()));

        assert!(state.dirty(), "nothing was saved, so there is still work");
        assert_eq!(state.failure(), Some("the server said no"));
        assert!(state.can_submit(), "and it can be asked again");

        assert_eq!(state.submit(Vec::new), Submit::Save);
        assert_eq!(state.failure(), None, "asking again clears the last answer");
    }

    #[test]
    fn asking_twenty_times_after_an_async_failure_never_saves() {
        let mut state = form();
        state.changed("email");
        let generation = state.validating("email");
        state.validated("email", generation, Some("taken".to_string()));

        for _ in 0..20 {
            assert_eq!(
                state.submit(Vec::new),
                Submit::Blocked {
                    first_error: "email"
                }
            );
            assert!(!state.submitting());
        }
    }

    #[test]
    fn asking_right_after_a_change_waits_for_the_check_that_change_started() {
        let mut state = form();
        for _ in 0..20 {
            state.changed("email");
            let generation = state.validating("email");
            // Submitted before the check comes back, twenty times over.
            assert_eq!(state.submit(Vec::new), Submit::Waiting);
            assert_eq!(
                state.validated("email", generation, None),
                Some(Submit::Save)
            );
            state.submitted(Ok(()));
        }
    }

    #[test]
    fn can_submit_is_false_for_each_of_the_three_reasons() {
        let mut state = form();
        state.changed("email");
        state.validating("email");
        assert!(!state.can_submit(), "a check is running");

        let generation = state.generation("email");
        state.validated("email", generation, Some("taken".to_string()));
        assert!(!state.can_submit(), "a field is wrong");

        state.changed("email");
        assert!(state.can_submit());
        assert_eq!(state.submit(Vec::new), Submit::Save);
        assert!(!state.can_submit(), "a save is running");
    }

    #[test]
    fn a_field_the_form_does_not_have_is_recorded_rather_than_ignored() {
        let mut state = form();
        crate::diagnostics::set_recording(true);
        state.changed("nickname");
        state.set_error("nickname", Some("no such field".to_string()));

        assert_eq!(state.error("nickname"), None);
        assert_eq!(
            state.validated("nickname", state.generation("nickname"), None),
            None
        );
        assert!(state.can_submit(), "and the form is unharmed");

        let named = crate::diagnostics::with_log(|log| {
            log.entries()
                .filter(|entry| entry.kind == crate::diagnostics::ErrorKind::UnknownField)
                .filter(|entry| entry.field == Some("nickname"))
                .count()
        });
        // Once per call that named it - `changed`, `set_error`, `validated` -
        // so a misspelling shows up as often as it is made rather than once.
        // Reading an error is not one of them: a read asserts nothing.
        assert_eq!(named, 3);
    }

    #[test]
    fn an_error_from_a_check_survives_the_next_ask_and_a_rules_error_does_not() {
        // The distinction that matters: asking again re-runs the rules, so
        // their errors are theirs to withdraw. A check answered about the
        // value itself, and the value has not changed.
        let mut state = form();
        state.set_error("email", Some("a rule said no".to_string()));
        assert!(matches!(state.submit(Vec::new), Submit::Save));
        assert_eq!(state.error("email"), None);

        let mut state = form();
        let generation = state.validating("email");
        state.validated("email", generation, Some("a check said no".to_string()));
        assert_eq!(
            state.submit(Vec::new),
            Submit::Blocked {
                first_error: "email"
            }
        );
        assert_eq!(state.error("email"), Some("a check said no"));
    }
    #[test]
    fn sources_reveal_retained_lower_priority_errors_and_changed_clears_them() {
        let mut state = form();
        state.set_application_error("email", Some("application".into()));
        let generation = state.validating("email");
        state.validated("email", generation, Some("check".into()));
        state.set_error("email", Some("rule".into()));
        state.set_constraint_error("email", Some("constraint".into()));
        assert_eq!(
            (state.error("email"), state.error_source("email")),
            (Some("constraint"), Some(FieldErrorSource::Constraint))
        );
        state.set_constraint_error("email", None);
        assert_eq!(
            (state.error("email"), state.error_source("email")),
            (Some("rule"), Some(FieldErrorSource::Rule))
        );
        assert_eq!(
            state.submit(Vec::new),
            Submit::Blocked {
                first_error: "email"
            }
        );
        assert_eq!(
            (state.error("email"), state.error_source("email")),
            (Some("check"), Some(FieldErrorSource::Check))
        );
        state.validated("email", generation, None);
        assert_eq!(
            (state.error("email"), state.error_source("email")),
            (Some("application"), Some(FieldErrorSource::Application))
        );
        assert_eq!(
            state.submit(Vec::new),
            Submit::Blocked {
                first_error: "email"
            }
        );
        state.changed("email");
        assert_eq!(
            (state.error("email"), state.error_source("email")),
            (None, None)
        );
    }

    #[test]
    fn reset_invalidates_old_checks_waiting_requests_and_dirty_state() {
        let mut state = form();
        state.changed("email");
        let old = state.validating("email");
        assert_eq!(state.submit(Vec::new), Submit::Waiting);
        assert!(state.reset());
        assert!(state.generation("email") > old);
        assert!(!state.dirty() && !state.waiting() && !state.is_validating());
        assert_eq!(state.validated("email", old, Some("stale".into())), None);
        assert_eq!(state.error("email"), None);
        assert!(state.can_submit());
    }

    #[test]
    fn reset_is_refused_during_save_and_old_completion_after_reset_is_ignored() {
        let mut state = form();
        state.changed("email");
        let generation = state.generation("email");
        assert_eq!(state.submit(Vec::new), Submit::Save);
        assert!(!state.reset());
        assert!(state.submitting() && state.dirty());
        assert_eq!(state.generation("email"), generation);
        state.submitted(Err("failed".into()));
        assert_eq!(state.failure(), Some("failed"));
        assert!(state.reset());
        assert!(!state.dirty());
        assert_eq!(state.failure(), None);
        state.submitted(Err("old completion".into()));
        assert_eq!(state.failure(), None);
    }
}
