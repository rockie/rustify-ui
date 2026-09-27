//! A number the application owns, typed as text.
//!
//! A strictly controlled field cannot hold a number on its way to being one:
//! `-`, `1e` and `0.` are not values, an application refuses them, and the
//! field would wipe them out from under the person typing. So this one is
//! built on the SDK's `Draft`, the one sanctioned exception to controlled
//! values: while it has focus it keeps what is typed, every number typed is a
//! preview request, and Enter or leaving the field is one commit request.
//! Whatever the application then holds is what it shows.
//!
//! The bounds are announced and not enforced. An arrow key asks for one step
//! from the value the application holds, and the application clamps it - or
//! does not; it owns the value, and a field that clamped for it would be a
//! second authority on what the value may be.

/// The number `text` spells, if it spells one.
///
/// A finite number in Rust's own syntax, blanks around it ignored: `12`,
/// `-3.5`, `.5`, `1e3`. The words Rust also reads as numbers - `inf`, `NaN` -
/// are not values a person means to type.
pub fn parse_number(text: &str) -> Option<f64> {
    text.trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

/// How the field writes a value: the shortest text that reads back as the
/// same number, and never `-0`.
pub fn format_number(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        value.to_string()
    }
}

/// How many decimals a number needs, capped where binary floating point
/// stops being able to tell.
fn decimals(value: f64) -> i32 {
    let text = format_number(value.abs());
    let digits = text
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    digits.min(12) as i32
}

/// One arrow key's worth: `from` moved by `delta`, rounded to the decimals
/// the two of them have between them, so that `0.1` and a step of `0.2` make
/// `0.3` and not `0.30000000000000004`.
pub fn stepped(from: f64, delta: f64) -> f64 {
    let scale = 10f64.powi(decimals(from).max(decimals(delta)));
    ((from + delta) * scale).round() / scale
}

#[cfg(target_arch = "wasm32")]
pub use dom::NumberField;

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{format_number, parse_number, stepped};
    use leptos::ev::KeyboardEvent;
    use leptos::prelude::*;
    use rustify_ui::Draft;

    const BASE: &str = "flex h-9 w-full min-w-0 rounded-md border border-border bg-input text-foreground px-3 py-1 text-sm tabular-nums transition-colors outline-none placeholder:text-muted-foreground focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 read-only:bg-muted aria-invalid:border-destructive aria-invalid:ring-destructive/40";

    /// A field for one number.
    ///
    /// Up and Down ask for one `step` more or less than the value the
    /// application holds, Page Up and Page Down for ten; a typed edit ends
    /// first, the way Enter would end it. A disabled or read-only field asks
    /// for nothing.
    #[component]
    pub fn NumberField(
        #[prop(into)] value: Signal<f64>,
        /// Asked for the number an edit ended on: Enter, leaving the field, or
        /// an arrow key.
        on_change: impl Fn(f64) + Send + Sync + 'static,
        /// Asked for every number typed while the edit goes on, for an
        /// application that shows the change as it is typed. Refusing one
        /// leaves the draft where it is.
        #[prop(optional, into)]
        on_preview: Option<Callback<f64>>,
        /// Asked to take back the previews of an edit Escape abandoned.
        #[prop(optional, into)]
        on_cancel: Option<Callback<()>>,
        /// Announced as the least the value may be. Not enforced: that is the
        /// application's to do with what it is asked for.
        #[prop(optional)]
        min: Option<f64>,
        #[prop(optional)] max: Option<f64>,
        /// What one arrow key asks to add. One, unless the caller says.
        #[prop(optional)]
        step: Option<f64>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        /// The application's own verdict on the value it holds. A draft that
        /// does not parse is marked invalid without being asked.
        #[prop(optional, into)]
        invalid: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let id = if id.is_empty() {
            crate::id::next("number")
        } else {
            id
        };
        let step = step.unwrap_or(1.0);
        let aria_label = (!aria_label.is_empty()).then_some(aria_label);
        let change = Callback::new(on_change);
        let draft = Draft::new(value, |value: &f64| format_number(*value), parse_number)
            .with_commit(move |value| change.run(value))
            .with_preview(move |value| {
                if let Some(preview) = on_preview {
                    preview.run(value);
                }
            })
            .with_cancel(move || {
                if let Some(cancel) = on_cancel {
                    cancel.run(());
                }
            });
        let still = move || disabled.get_untracked() || read_only.get_untracked();
        let keydown = draft.on_keydown();
        let on_keydown = move |event: KeyboardEvent| {
            if still() || event.is_composing() {
                return;
            }
            let delta = match event.key().as_str() {
                "ArrowUp" => step,
                "ArrowDown" => -step,
                "PageUp" => step * 10.0,
                "PageDown" => -step * 10.0,
                _ => return keydown(event),
            };
            event.prevent_default();
            draft.commit();
            // Read after the commit: the step is from whatever the
            // application made of what was typed.
            change.run(stepped(value.get_untracked(), delta));
        };
        let on_input = draft.on_input();
        let invalid_draft = draft.invalid();
        view! {
            <input
                type="text"
                inputmode="decimal"
                role="spinbutton"
                autocomplete="off"
                spellcheck="false"
                id=id
                class=crate::macros::merge(BASE, &class)
                data-name="NumberField"
                data-testid=test_id
                aria-label=aria_label
                aria-valuenow=move || format_number(value.get())
                aria-valuemin=min.map(format_number)
                aria-valuemax=max.map(format_number)
                aria-invalid=move || (invalid.get() || invalid_draft.get()).then_some("true")
                aria-readonly=move || read_only.get().then_some("true")
                aria-describedby=move || {
                    let described_by = described_by.get();
                    (!described_by.is_empty()).then_some(described_by)
                }
                prop:value=draft.text()
                prop:disabled=move || disabled.get()
                prop:readOnly=move || read_only.get()
                on:focus=draft.on_focus()
                on:blur=draft.on_blur()
                on:keydown=on_keydown
                on:input=move |event| {
                    if !still() {
                        on_input(event);
                    }
                }
            />
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{format_number, parse_number, stepped};

    #[test]
    fn a_number_is_what_rust_reads_as_a_finite_one() {
        assert_eq!(parse_number("12"), Some(12.0));
        assert_eq!(parse_number(" -3.5 "), Some(-3.5));
        assert_eq!(parse_number(".5"), Some(0.5));
        assert_eq!(parse_number("1e3"), Some(1000.0));
    }

    #[test]
    fn text_on_its_way_to_a_number_is_not_one_yet() {
        for text in ["", "-", "1e", "1e+", "abc", "1,5", "inf", "NaN", "1e999"] {
            assert_eq!(parse_number(text), None, "{text:?}");
        }
    }

    #[test]
    fn a_value_is_written_the_shortest_way_that_reads_back() {
        assert_eq!(format_number(40.0), "40");
        assert_eq!(format_number(12.5), "12.5");
        assert_eq!(format_number(-0.0), "0");
        for value in [0.1, 1.0 / 3.0, -250.75, 1e-7] {
            assert_eq!(parse_number(&format_number(value)), Some(value));
        }
    }

    #[test]
    fn a_step_lands_on_the_decimals_its_two_parts_have() {
        assert_eq!(stepped(40.0, 5.0), 45.0);
        assert_eq!(stepped(0.1, 0.2), 0.3);
        assert_eq!(stepped(1.25, -0.1), 1.15);
        assert_eq!(stepped(100.0, 50.0), 150.0);
        // Nothing is clamped here: that is the application's.
        assert_eq!(stepped(0.0, -1.0), -1.0);
    }
}
