//! A text field that keeps what is being typed while it has focus.
//!
//! This is the one exception to controlled values. A strictly controlled field
//! puts the application's value back after every keystroke, which is right for
//! free text and wrong for a number or a colour: `-`, `1e` and `#1f` are on
//! the way to a value without being one, and an application that refuses them
//! would wipe them out from under the person typing.
//!
//! The exception is bounded. Only a focused field shows a draft, and only
//! once something has been typed; the application's value is what everything
//! else shows, and what the field goes back to when it loses focus. The
//! application is still asked for every change and still decides:
//!
//! - every input that parses is a preview request, which the application may
//!   refuse without the draft being lost;
//! - Enter or leaving the field is one commit request; text that does not
//!   parse commits the last preview if there was one, and otherwise nothing;
//! - Escape drops the draft and asks for any preview to be taken back.
//!
//! While nothing has been typed, a change to the application's value shows at
//! once. Once something has, the draft stays until the edit ends and the field
//! settles on whatever the application then holds.

/// One field's edit, apart from the DOM.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, PartialEq)]
struct Session<T> {
    focused: bool,
    /// What has been typed since focus, a commit or a revert. `None` while
    /// untouched, and then the field shows the application's value.
    edit: Option<Edit<T>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Edit<T> {
    text: String,
    /// What `text` parses to.
    value: Option<T>,
    /// The last value asked for as a preview during this edit.
    previewed: Option<T>,
}

/// What the field asks of the application.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, PartialEq)]
enum Request<T> {
    /// Show this value for now; the edit is not over.
    Preview(T),
    /// The edit is over, and this is the value it settled on.
    Commit(T),
    /// The edit was abandoned: take back what the previews did.
    Cancel,
}

impl<T> Default for Session<T> {
    fn default() -> Self {
        Self {
            focused: false,
            edit: None,
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl<T: Clone> Session<T> {
    fn focus(&mut self) {
        self.focused = true;
        self.edit = None;
    }

    /// The field now holds `text`, which parses to `value`.
    fn input(&mut self, text: String, value: Option<T>) -> Option<Request<T>> {
        // An input can arrive without a focus event before it - a value set by
        // an extension or a test - and it is an edit all the same.
        self.focused = true;
        let previewed = value
            .clone()
            .or_else(|| self.edit.take().and_then(|edit| edit.previewed));
        self.edit = Some(Edit {
            text,
            value: value.clone(),
            previewed,
        });
        value.map(Request::Preview)
    }

    /// Enter: the edit so far becomes one commit, and the field stays focused
    /// on the application's value.
    fn commit(&mut self) -> Option<Request<T>> {
        let edit = self.edit.take()?;
        edit.value.or(edit.previewed).map(Request::Commit)
    }

    /// Leaving the field commits, and the field shows the application's
    /// value again.
    fn blur(&mut self) -> Option<Request<T>> {
        let request = self.commit();
        self.focused = false;
        request
    }

    /// Escape: the draft goes, and a preview is asked to be taken back.
    fn escape(&mut self) -> Option<Request<T>> {
        let edit = self.edit.take()?;
        edit.previewed.map(|_| Request::Cancel)
    }

    /// Whether there is a draft for Escape to drop.
    fn touched(&self) -> bool {
        self.edit.is_some()
    }

    /// Whether the draft is text that does not parse.
    fn invalid(&self) -> bool {
        self.edit.as_ref().is_some_and(|edit| edit.value.is_none())
    }

    /// What the field shows: the draft, or the application's value.
    fn text(&self, application: impl FnOnce() -> String) -> String {
        match &self.edit {
            Some(edit) if self.focused => edit.text.clone(),
            _ => application(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use dom::Draft;

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{Request, Session};
    use crate::overlay::{use_overlay, EditSession, OverlayStack};
    use leptos::ev::{Event, FocusEvent, KeyboardEvent};
    use leptos::prelude::*;
    use std::sync::Arc;

    type Format<T> = Arc<dyn Fn(&T) -> String + Send + Sync>;
    type Parse<T> = Arc<dyn Fn(&str) -> Option<T> + Send + Sync>;
    type Asked<T> = Option<Arc<dyn Fn(T) + Send + Sync>>;

    struct Hooks<T> {
        format: Format<T>,
        parse: Parse<T>,
        preview: Asked<T>,
        commit: Asked<T>,
        cancel: Option<Arc<dyn Fn() + Send + Sync>>,
        /// The scope's layer stack, which a touched draft registers with so
        /// that Escape reverts the draft before it closes a layer around it.
        stack: Option<OverlayStack>,
    }

    /// The draft behind one text field whose value is a `T`.
    ///
    /// Bind it to a text input - not `type=number`, which reports an empty
    /// value for exactly the intermediate text a draft exists to keep:
    ///
    /// ```ignore
    /// let draft = Draft::new(value, |v: &f64| v.to_string(), |text| text.trim().parse().ok())
    ///     .with_preview(move |v| preview.run(v))
    ///     .with_commit(move |v| commit.run(v))
    ///     .with_cancel(move || cancel.run(()));
    /// view! {
    ///     <input type="text" inputmode="decimal"
    ///         prop:value=draft.text()
    ///         aria-invalid=move || draft.invalid().get().then_some("true")
    ///         on:focus=draft.on_focus() on:input=draft.on_input()
    ///         on:keydown=draft.on_keydown() on:blur=draft.on_blur() />
    /// }
    /// ```
    pub struct Draft<T: Send + Sync + 'static> {
        value: Signal<T>,
        /// Notified only when an edit ends, which is when what the field shows
        /// stops being what is already in it. Writing the draft back into the
        /// field on every keystroke would take an input method's text out of
        /// it mid-composition.
        session: RwSignal<Session<T>>,
        invalid: RwSignal<bool>,
        hooks: StoredValue<Hooks<T>>,
    }

    impl<T: Send + Sync + 'static> Clone for Draft<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Send + Sync + 'static> Copy for Draft<T> {}

    impl<T: Clone + Send + Sync + 'static> Draft<T> {
        /// A draft over the application's `value`, shown with `format` and read
        /// back with `parse`, which returns `None` for text that is not a
        /// value yet.
        pub fn new(
            value: Signal<T>,
            format: impl Fn(&T) -> String + Send + Sync + 'static,
            parse: impl Fn(&str) -> Option<T> + Send + Sync + 'static,
        ) -> Self {
            let draft = Draft {
                value,
                session: RwSignal::new(Session::default()),
                invalid: RwSignal::new(false),
                hooks: StoredValue::new(Hooks {
                    format: Arc::new(format),
                    parse: Arc::new(parse),
                    preview: None,
                    commit: None,
                    cancel: None,
                    stack: use_overlay(),
                }),
            };
            // A field that goes away mid-edit must not leave its Escape handler
            // registered with the scope.
            on_cleanup(move || {
                if draft.touched() {
                    draft.leave_stack();
                }
            });
            draft
        }

        /// Asked to show a value for now, while the edit goes on.
        pub fn with_preview(self, f: impl Fn(T) + Send + Sync + 'static) -> Self {
            self.hooks
                .update_value(|hooks| hooks.preview = Some(Arc::new(f)));
            self
        }

        /// Asked to take a value: the edit is over.
        pub fn with_commit(self, f: impl Fn(T) + Send + Sync + 'static) -> Self {
            self.hooks
                .update_value(|hooks| hooks.commit = Some(Arc::new(f)));
            self
        }

        /// Asked to take back the previews of an edit that was abandoned.
        pub fn with_cancel(self, f: impl Fn() + Send + Sync + 'static) -> Self {
            self.hooks
                .update_value(|hooks| hooks.cancel = Some(Arc::new(f)));
            self
        }

        /// The text the field shows.
        pub fn text(&self) -> Signal<String> {
            let this = *self;
            Signal::derive(move || {
                let format = this.hooks.with_value(|hooks| hooks.format.clone());
                this.session
                    .with(|session| session.text(|| this.value.with(|value| format(value))))
            })
        }

        /// Whether the draft is text that does not parse, for `aria-invalid`.
        pub fn invalid(&self) -> Signal<bool> {
            self.invalid.into()
        }

        /// For `on:focus`: an edit starts from the application's value.
        pub fn on_focus(&self) -> impl Fn(FocusEvent) + Copy + 'static {
            let this = *self;
            move |_| this.change(Session::focus)
        }

        /// For `on:input`: the field's text becomes the draft, and a preview
        /// request when it parses.
        pub fn on_input(&self) -> impl Fn(Event) + Copy + 'static {
            let this = *self;
            move |event| {
                let text = event_target_value(&event);
                let Some(value) = this.hooks.try_with_value(|hooks| (hooks.parse)(&text)) else {
                    return;
                };
                let request = this.change(|session| session.input(text, value));
                this.ask(request);
            }
        }

        /// For `on:keydown`: Enter commits, Escape reverts.
        pub fn on_keydown(&self) -> impl Fn(KeyboardEvent) + Copy + 'static {
            let this = *self;
            move |event| {
                // Keys pressed while an input method is composing belong to
                // the composition.
                if event.is_composing() {
                    return;
                }
                match event.key().as_str() {
                    "Enter" => {
                        let request = this.change(Session::commit);
                        this.ask(request);
                    }
                    "Escape" if this.touched() => {
                        event.prevent_default();
                        event.stop_propagation();
                        this.revert();
                    }
                    _ => {}
                }
            }
        }

        /// For `on:blur`: commits, and the field shows the application's value
        /// again.
        pub fn on_blur(&self) -> impl Fn(FocusEvent) + Copy + 'static {
            let this = *self;
            move |_| {
                let request = this.change(Session::blur);
                this.ask(request);
            }
        }

        fn touched(&self) -> bool {
            self.session
                .try_with_untracked(Session::touched)
                .unwrap_or(false)
        }

        fn revert(&self) {
            let request = self.change(Session::escape);
            self.ask(request);
        }

        /// Applies one transition, and keeps the field, the invalid flag and
        /// the scope's Escape handler in step with it.
        fn change<R: Default>(&self, f: impl FnOnce(&mut Session<T>) -> R) -> R {
            let before = self.touched();
            let changed = self.session.try_maybe_update(|session| {
                let result = f(session);
                let touched = session.touched();
                // Only the end of an edit changes what the field shows: until
                // then it shows the draft, which is what is already in it.
                (before && !touched, (result, touched, session.invalid()))
            });
            let Some((result, touched, invalid)) = changed else {
                return R::default();
            };
            if self.invalid.get_untracked() != invalid {
                self.invalid.set(invalid);
            }
            match (before, touched) {
                (false, true) => self.join_stack(),
                (true, false) => self.leave_stack(),
                _ => {}
            }
            result
        }

        /// While there is a draft, Escape is the draft's before it is the
        /// scope's: the scope hears it first and would close the layer the
        /// field sits in, leaving the edit neither committed nor taken back.
        fn join_stack(&self) {
            let this = *self;
            let stack = self.hooks.with_value(|hooks| hooks.stack.clone());
            if let Some(stack) = stack {
                stack.begin_edit(EditSession {
                    // Not followed: the values a draft is for - numbers,
                    // colours - are not typed through an input method.
                    composing: Arc::new(|| false),
                    cancel: Arc::new(move || this.revert()),
                });
            }
        }

        fn leave_stack(&self) {
            let stack = self
                .hooks
                .try_with_value(|hooks| hooks.stack.clone())
                .flatten();
            if let Some(stack) = stack {
                stack.end_edit();
            }
        }

        /// Hands a request to the application, outside every borrow: what it
        /// does about one may well change the value this draft reads.
        fn ask(&self, request: Option<Request<T>>) {
            let Some(request) = request else {
                return;
            };
            let asked = self.hooks.try_with_value(|hooks| {
                (
                    hooks.preview.clone(),
                    hooks.commit.clone(),
                    hooks.cancel.clone(),
                )
            });
            let Some((preview, commit, cancel)) = asked else {
                return;
            };
            match request {
                Request::Preview(value) => {
                    if let Some(preview) = preview {
                        preview(value);
                    }
                }
                Request::Commit(value) => {
                    if let Some(commit) = commit {
                        commit(value);
                    }
                }
                Request::Cancel => {
                    if let Some(cancel) = cancel {
                        cancel();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Session};
    use Request::{Cancel, Commit, Preview};
    use Step::{Blur, Enter, Escape, External, Focus, Type};

    fn parse(text: &str) -> Option<f64> {
        text.trim()
            .parse()
            .ok()
            .filter(|value: &f64| value.is_finite())
    }

    fn format(value: f64) -> String {
        value.to_string()
    }

    /// A field driven by the same steps a browser would take, beside an
    /// application that holds `value` and answers requests with `accept`.
    struct Field {
        session: Session<f64>,
        value: f64,
        accept: fn(f64) -> bool,
        asked: Vec<Request<f64>>,
        /// The value before the first preview of the current edit, for a
        /// cancel to restore.
        before: Option<f64>,
    }

    enum Step {
        Focus,
        Type(&'static str),
        Enter,
        Escape,
        Blur,
        /// The application changes its value on its own.
        External(f64),
    }

    impl Field {
        fn new(value: f64, accept: fn(f64) -> bool) -> Self {
            Field {
                session: Session::default(),
                value,
                accept,
                asked: Vec::new(),
                before: None,
            }
        }

        fn shows(&self) -> String {
            self.session.text(|| format(self.value))
        }

        fn run(&mut self, step: Step) {
            let request = match step {
                Step::Focus => {
                    self.session.focus();
                    None
                }
                Step::Type(text) => self.session.input(text.into(), parse(text)),
                Step::Enter => self.session.commit(),
                Step::Escape => self.session.escape(),
                Step::Blur => self.session.blur(),
                Step::External(value) => {
                    self.value = value;
                    None
                }
            };
            if let Some(request) = request {
                self.answer(request);
            }
        }

        fn answer(&mut self, request: Request<f64>) {
            match request {
                Request::Preview(value) if (self.accept)(value) => {
                    self.before.get_or_insert(self.value);
                    self.value = value;
                }
                Request::Commit(value) => {
                    if (self.accept)(value) {
                        self.value = value;
                    }
                    self.before = None;
                }
                Request::Cancel => {
                    if let Some(before) = self.before.take() {
                        self.value = before;
                    }
                }
                Request::Preview(_) => {}
            }
            self.asked.push(request);
        }
    }

    fn positive(value: f64) -> bool {
        value > 0.0
    }

    fn anything(_: f64) -> bool {
        true
    }

    /// Runs `steps`, checking after each what the field shows and what it has
    /// asked for so far.
    fn check(start: f64, accept: fn(f64) -> bool, steps: Vec<(Step, &str, Vec<Request<f64>>)>) {
        let mut field = Field::new(start, accept);
        for (index, (step, shows, asked)) in steps.into_iter().enumerate() {
            field.run(step);
            assert_eq!(field.shows(), shows, "step {index}: what the field shows");
            assert_eq!(field.asked, asked, "step {index}: what it asked for");
        }
    }

    #[test]
    fn unfocused_and_untouched_the_field_shows_the_application_value() {
        check(
            10.0,
            anything,
            vec![
                (External(12.0), "12", vec![]),
                (Focus, "12", vec![]),
                // Focused but untouched: a change still shows at once.
                (External(14.0), "14", vec![]),
                (Blur, "14", vec![]),
            ],
        );
    }

    #[test]
    fn every_input_that_parses_is_a_preview_and_a_refusal_keeps_the_draft() {
        check(
            10.0,
            positive,
            vec![
                (Focus, "10", vec![]),
                (Type("-"), "-", vec![]),
                // Refused: the application keeps 10, and the field keeps "-5".
                (Type("-5"), "-5", vec![Preview(-5.0)]),
                (Type("5"), "5", vec![Preview(-5.0), Preview(5.0)]),
            ],
        );
    }

    #[test]
    fn enter_commits_once_and_the_field_settles_on_the_application_value() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("12.50"), "12.50", vec![Preview(12.5)]),
                (Enter, "12.5", vec![Preview(12.5), Commit(12.5)]),
                (Enter, "12.5", vec![Preview(12.5), Commit(12.5)]),
                // Blur after Enter has nothing left to commit.
                (Blur, "12.5", vec![Preview(12.5), Commit(12.5)]),
            ],
        );
    }

    #[test]
    fn blur_commits_once() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("3"), "3", vec![Preview(3.0)]),
                (Blur, "3", vec![Preview(3.0), Commit(3.0)]),
                (Blur, "3", vec![Preview(3.0), Commit(3.0)]),
            ],
        );
    }

    #[test]
    fn a_refused_commit_goes_back_to_the_application_value() {
        check(
            10.0,
            positive,
            vec![
                (Focus, "10", vec![]),
                (Type("-3"), "-3", vec![Preview(-3.0)]),
                (Blur, "10", vec![Preview(-3.0), Commit(-3.0)]),
            ],
        );
    }

    #[test]
    fn text_that_never_parsed_commits_nothing_and_shows_the_application_value() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("1e"), "1e", vec![]),
                (Enter, "10", vec![]),
                (Type("abc"), "abc", vec![]),
                (Blur, "10", vec![]),
            ],
        );
    }

    #[test]
    fn text_that_stopped_parsing_commits_the_last_preview() {
        // "12" was shown live; "12x" is not a value. Ending the edit finishes
        // the preview rather than leaving the application halfway through one.
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("12"), "12", vec![Preview(12.0)]),
                (Type("12x"), "12x", vec![Preview(12.0)]),
                (Blur, "12", vec![Preview(12.0), Commit(12.0)]),
            ],
        );
    }

    #[test]
    fn escape_drops_the_draft_and_asks_for_the_preview_back() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("7"), "7", vec![Preview(7.0)]),
                (Escape, "10", vec![Preview(7.0), Cancel]),
                // Nothing typed since: blur commits nothing.
                (Blur, "10", vec![Preview(7.0), Cancel]),
            ],
        );
        // With no preview to take back, Escape asks for nothing.
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("x"), "x", vec![]),
                (Escape, "10", vec![]),
            ],
        );
    }

    #[test]
    fn escape_on_an_untouched_field_is_not_the_field_s_key() {
        let mut session = Session::<f64>::default();
        session.focus();
        assert!(!session.touched());
        assert_eq!(session.escape(), None);
        session.input("4".into(), Some(4.0));
        assert!(session.touched());
        assert_eq!(session.escape(), Some(Cancel));
        assert!(!session.touched());
    }

    #[test]
    fn a_touched_draft_outlasts_an_outside_change_and_settles_on_blur() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("8"), "8", vec![Preview(8.0)]),
                // Someone else changes the value mid-edit: the draft stays.
                (External(20.0), "8", vec![Preview(8.0)]),
                (Blur, "8", vec![Preview(8.0), Commit(8.0)]),
            ],
        );
        // And when the application refuses the commit, the field settles on
        // whatever the application holds - here, the outside change.
        check(
            10.0,
            positive,
            vec![
                (Focus, "10", vec![]),
                (Type("0"), "0", vec![Preview(0.0)]),
                (External(20.0), "0", vec![Preview(0.0)]),
                (Blur, "20", vec![Preview(0.0), Commit(0.0)]),
            ],
        );
    }

    #[test]
    fn after_a_commit_an_untouched_field_follows_the_application_again() {
        check(
            10.0,
            anything,
            vec![
                (Focus, "10", vec![]),
                (Type("11"), "11", vec![Preview(11.0)]),
                (Enter, "11", vec![Preview(11.0), Commit(11.0)]),
                (External(30.0), "30", vec![Preview(11.0), Commit(11.0)]),
            ],
        );
    }

    #[test]
    fn a_draft_is_invalid_only_while_its_text_does_not_parse() {
        let mut session = Session::<f64>::default();
        session.focus();
        assert!(!session.invalid());
        session.input("-".into(), parse("-"));
        assert!(session.invalid());
        session.input("-2".into(), parse("-2"));
        assert!(!session.invalid());
        session.input("-2e".into(), parse("-2e"));
        assert!(session.invalid());
        session.blur();
        assert!(!session.invalid(), "the field shows the application value");
    }

    #[test]
    fn an_input_without_a_focus_event_is_still_an_edit() {
        let mut session = Session::<f64>::default();
        assert_eq!(session.input("5".into(), Some(5.0)), Some(Preview(5.0)));
        assert_eq!(session.text(|| "10".into()), "5");
        assert_eq!(session.blur(), Some(Commit(5.0)));
        assert_eq!(session.text(|| "10".into()), "10");
    }
}
