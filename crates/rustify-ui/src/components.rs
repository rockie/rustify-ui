//! The DOM half of the component subset, with the application owning the value.
//!
//! Every control here is controlled in the strict sense: what the user does is
//! a request, and what the control shows afterwards is the application's
//! answer. When the application refuses a value the control goes back to
//! showing the value the application holds, rather than keeping keystrokes
//! that changed nothing. Disabled and read-only controls make no request at
//! all.

/// A slider's value as the application will see it: inside the range, and on
/// one of the steps.
///
/// The browser already does this for a pointer drag, but not for a value an
/// application hands back, so the rule lives here where both can use it.
pub fn snap(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let (min, max) = if min <= max { (min, max) } else { (max, min) };
    let clamped = value.clamp(min, max);
    // A step of zero, a negative one or a NaN is not a set of values to land
    // on; the range still holds.
    if step.is_nan() || step <= 0.0 {
        return clamped;
    }
    let steps = ((clamped - min) / step).round();
    (min + steps * step).clamp(min, max)
}

#[cfg(target_arch = "wasm32")]
pub use dom::{Button, Checkbox, Label, LoadView, Slider, TextArea, TextField};

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::snap;
    use crate::task::Load;
    use leptos::prelude::*;

    /// An ordinary button. A disabled one is out of the tab order and calls
    /// nothing.
    #[component]
    pub fn Button(
        on_click: impl Fn() + 'static,
        #[prop(optional, into)] disabled: Signal<bool>,
        /// The accessible name, when the button's own text is not it - a text
        /// that changes with the state, say. Left out, the text names it.
        #[prop(optional, into)]
        aria_label: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        // An empty `aria-label` would take the name away rather than leave the
        // text to provide it.
        let aria_label = (!aria_label.is_empty()).then_some(aria_label);
        view! {
            <button
                type="button"
                class=class
                data-testid=test_id
                aria-label=aria_label
                prop:disabled=move || disabled.get()
                on:click=move |_| {
                    if !disabled.get_untracked() {
                        on_click();
                    }
                }
            >
                {children()}
            </button>
        }
    }

    /// Text with no action of its own. `control` names the control it labels,
    /// so clicking it moves focus there.
    #[component]
    pub fn Label(
        #[prop(into)] text: Signal<String>,
        #[prop(optional, into)] control: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let control = (!control.is_empty()).then_some(control);
        view! {
            <label class=class data-testid=test_id r#for=control>
                {move || text.get()}
            </label>
        }
    }

    /// One line of text the application owns.
    ///
    /// The label wraps the control, so the accessible name is the label's
    /// text. After every keystroke the control is put back in step with the
    /// application: a refused value is not left on screen.
    #[component]
    pub fn TextField(
        #[prop(into)] label: String,
        #[prop(into)] value: Signal<String>,
        on_input: impl Fn(String) + 'static,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let node = NodeRef::<leptos::html::Input>::new();
        // Whether an input method is composing in this field right now, and the
        // one callback shared by the two handlers that now report.
        let composing = StoredValue::new(false);
        let on_input = std::rc::Rc::new(on_input);
        let on_composed = std::rc::Rc::clone(&on_input);
        let settle = move || {
            if let Some(input) = node.get_untracked() {
                let held = value.get_untracked();
                if input.value() != held {
                    input.set_value(&held);
                }
            }
        };
        view! {
            <label>
                {label}
                <input
                    type="text"
                    node_ref=node
                    data-testid=test_id
                    prop:value=move || value.get()
                    prop:disabled=move || disabled.get()
                    prop:readOnly=move || read_only.get()
                    aria-readonly=move || read_only.get().then_some("true")
                    on:compositionstart=move |_| composing.set_value(true)
                    on:compositionend:target=move |ev| {
                        // The first moment the field holds a value rather than
                        // the keys being used to look one up.
                        composing.set_value(false);
                        let chosen = ev.target().value();
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_composed(chosen);
                        }
                        settle();
                    }
                    on:input:target=move |ev| {
                        // A composition in flight is not a value. An input
                        // method puts the keys used to *look up* a character
                        // into the field - pinyin, bopomofo, a partial Hangul
                        // syllable - and reporting those would name an object
                        // "gongzuo" and leave it named that if the user changed
                        // their mind. Settling is skipped for the same reason:
                        // writing the held value back mid-composition takes the
                        // input method's own text out of the field.
                        if composing.get_value() {
                            return;
                        }
                        let typed = ev.target().value();
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_input(typed);
                        }
                        settle();
                    }
                />
            </label>
        }
    }

    /// Several lines of text the application owns. Enter is a newline here;
    /// nothing commits on it.
    #[component]
    pub fn TextArea(
        #[prop(into)] label: String,
        #[prop(into)] value: Signal<String>,
        on_input: impl Fn(String) + 'static,
        #[prop(optional)] rows: Option<u32>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let node = NodeRef::<leptos::html::Textarea>::new();
        let composing = StoredValue::new(false);
        let on_input = std::rc::Rc::new(on_input);
        let on_composed = std::rc::Rc::clone(&on_input);
        let settle = move || {
            if let Some(area) = node.get_untracked() {
                let held = value.get_untracked();
                if area.value() != held {
                    area.set_value(&held);
                }
            }
        };
        view! {
            <label>
                {label}
                <textarea
                    node_ref=node
                    data-testid=test_id
                    rows=rows.unwrap_or(2).to_string()
                    prop:value=move || value.get()
                    prop:disabled=move || disabled.get()
                    prop:readOnly=move || read_only.get()
                    aria-readonly=move || read_only.get().then_some("true")
                    on:compositionstart=move |_| composing.set_value(true)
                    on:compositionend:target=move |ev| {
                        // The first moment the field holds a value rather than
                        // the keys being used to look one up.
                        composing.set_value(false);
                        let chosen = ev.target().value();
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_composed(chosen);
                        }
                        settle();
                    }
                    on:input:target=move |ev| {
                        // A composition in flight is not a value. An input
                        // method puts the keys used to *look up* a character
                        // into the field - pinyin, bopomofo, a partial Hangul
                        // syllable - and reporting those would name an object
                        // "gongzuo" and leave it named that if the user changed
                        // their mind. Settling is skipped for the same reason:
                        // writing the held value back mid-composition takes the
                        // input method's own text out of the field.
                        if composing.get_value() {
                            return;
                        }
                        let typed = ev.target().value();
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_input(typed);
                        }
                        settle();
                    }
                ></textarea>
            </label>
        }
    }

    /// A two-state control the application owns.
    ///
    /// The browser has no read-only checkbox, so a read-only one takes the
    /// user's click and puts the box back the way the application has it.
    #[component]
    pub fn Checkbox(
        #[prop(into)] label: String,
        #[prop(into)] checked: Signal<bool>,
        on_change: impl Fn(bool) + 'static,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let node = NodeRef::<leptos::html::Input>::new();
        let settle = move || {
            if let Some(input) = node.get_untracked() {
                let held = checked.get_untracked();
                if input.checked() != held {
                    input.set_checked(held);
                }
            }
        };
        view! {
            <label>
                <input
                    type="checkbox"
                    node_ref=node
                    data-testid=test_id
                    prop:checked=move || checked.get()
                    prop:disabled=move || disabled.get()
                    aria-readonly=move || read_only.get().then_some("true")
                    on:change:target=move |ev| {
                        let asked = ev.target().checked();
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_change(asked);
                        }
                        settle();
                    }
                />
                {label}
            </label>
        }
    }

    /// A value on a range the application owns. The value is clamped and
    /// snapped before the application is asked for it.
    #[component]
    pub fn Slider(
        #[prop(into)] label: String,
        #[prop(into)] value: Signal<f64>,
        on_change: impl Fn(f64) + 'static,
        #[prop(optional)] min: Option<f64>,
        #[prop(optional)] max: Option<f64>,
        #[prop(optional)] step: Option<f64>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let min = min.unwrap_or(0.0);
        let max = max.unwrap_or(100.0);
        let step = step.unwrap_or(1.0);
        let node = NodeRef::<leptos::html::Input>::new();
        let settle = move || {
            if let Some(input) = node.get_untracked() {
                let held = snap(value.get_untracked(), min, max, step).to_string();
                if input.value() != held {
                    input.set_value(&held);
                }
            }
        };
        view! {
            <label>
                {label}
                <input
                    type="range"
                    node_ref=node
                    data-testid=test_id
                    min=min.to_string()
                    max=max.to_string()
                    step=step.to_string()
                    prop:value=move || snap(value.get(), min, max, step).to_string()
                    prop:disabled=move || disabled.get()
                    aria-readonly=move || read_only.get().then_some("true")
                    on:input:target=move |ev| {
                        let asked = ev.target().value().parse::<f64>().unwrap_or(min);
                        if !disabled.get_untracked() && !read_only.get_untracked() {
                            on_change(snap(asked, min, max, step));
                        }
                        settle();
                    }
                />
            </label>
        }
    }

    /// The four states of an asynchronous value, and one retry the application
    /// defines.
    ///
    /// The retry sits beside the status rather than inside it, so a reader of
    /// the status hears the state and not a button's label.
    ///
    /// The words are the SDK's - a wait, an empty answer and a failure are
    /// states of the SDK's own machinery, not of the application's data - so
    /// they come from the scope's language and change with it. The value and
    /// the reason for a failure are the application's and are passed through
    /// untouched.
    #[component]
    pub fn LoadView(
        #[prop(into)] value: Signal<Load<String>>,
        #[prop(into)] label: String,
        on_retry: impl Fn() + Clone + Send + Sync + 'static,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let retry_id = format!("{test_id}-retry");
        let locale = crate::i18n::use_locale();
        view! {
            <p
                class="load-view"
                role="status"
                aria-label=label
                data-testid=test_id
                data-state=move || value.with(|value| value.state())
            >
                {move || {
                    value
                        .with(|value| {
                            let say = |message| locale.text(message).to_string();
                            match value {
                                Load::Loading => say(crate::i18n::Message::Loading),
                                Load::Empty => say(crate::i18n::Message::Empty),
                                Load::Ready(value) => value.clone(),
                                Load::Error(error) => format!(
                                    "{}: {error}; {}",
                                    say(crate::i18n::Message::LoadFailed),
                                    say(crate::i18n::Message::Retry),
                                ),
                            }
                        })
                }}
            </p>
            <Show when=move || value.with(|value| value.error().is_some()) fallback=|| ()>
                <button
                    type="button"
                    class="load-retry"
                    data-testid=retry_id.clone()
                    on:click={
                        let on_retry = on_retry.clone();
                        move |_| on_retry()
                    }
                >
                    {move || locale.text(crate::i18n::Message::Retry)}
                </button>
            </Show>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::snap;

    #[test]
    fn a_value_outside_the_range_comes_back_inside_it() {
        assert_eq!(snap(-5.0, 0.0, 10.0, 1.0), 0.0);
        assert_eq!(snap(15.0, 0.0, 10.0, 1.0), 10.0);
        assert_eq!(snap(4.0, 0.0, 10.0, 1.0), 4.0);
    }

    #[test]
    fn a_value_between_two_steps_lands_on_one_of_them() {
        assert_eq!(snap(2.4, 0.0, 10.0, 2.0), 2.0);
        assert_eq!(snap(2.6, 0.0, 10.0, 2.0), 2.0);
        assert_eq!(snap(3.1, 0.0, 10.0, 2.0), 4.0);
        // A range that is not a whole number of steps stops at the last
        // step, not at the maximum - the same values the browser's own
        // control offers.
        assert_eq!(snap(9.9, 0.0, 10.0, 3.0), 9.0);
        assert_eq!(snap(10.0, 0.0, 10.0, 3.0), 9.0);
    }

    #[test]
    fn a_range_with_no_step_only_clamps() {
        assert_eq!(snap(2.345, 0.0, 10.0, 0.0), 2.345);
        assert_eq!(snap(12.5, 0.0, 10.0, -1.0), 10.0);
    }

    #[test]
    fn a_range_given_backwards_is_still_a_range() {
        assert_eq!(snap(5.0, 10.0, 0.0, 1.0), 5.0);
        assert_eq!(snap(-1.0, 10.0, 0.0, 1.0), 0.0);
    }
}
