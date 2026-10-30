//! All logical choice/slider controls share native form ordering and field focus.
use leptos::prelude::*;
use rustify_components::{primitives as p, Form};

#[component]
pub fn FormChoices() -> impl IntoView {
    let consent = RwSignal::new(true);
    let tags = RwSignal::new(vec!["b".to_string()]);
    let radio = RwSignal::new(Some("a".to_string()));
    let toggle = RwSignal::new(true);
    let formats = RwSignal::new(vec!["a".to_string()]);
    let amount = RwSignal::new(30.0);
    let saves = RwSignal::new(0);
    let form = Form::new(
        &["consent", "tags", "radio", "toggle", "formats", "amount"],
        move || saves.update(|n| *n += 1),
    );
    view! {
        <h2>"Native form choice registration"</h2>
        <button type="button" data-testid="form-choices-clear" on:click=move |_| {
            consent.set(false); tags.set(Vec::new()); radio.set(None); toggle.set(false); formats.set(Vec::new()); amount.set(10.0);
            for field in form.fields() { form.changed(field); }
        }>"Clear choices"</button>
        <output data-testid="form-choices-saves">{move || saves.get()}</output>
        <p::form::Root form=form id="choice-registered-form" test_id="choice-registered-form"
            rules=move || if amount.get_untracked() < 30.0 { vec![("amount", "Choose at least 30".into())] } else { Vec::new() }>
            <p::form::Field form=form field="consent"><p::form::Label>"Consent"</p::form::Label>
                <p::checkbox::Root checked=Signal::derive(move || p::CheckedState::from(consent.get())) required=true name="consent"
                    on_change=move |next| { consent.set(next == p::CheckedState::Checked); form.changed("consent"); }>
                    <p::checkbox::Control test_id="registered-consent" />
                </p::checkbox::Root><p::form::Message test_id="registered-error-consent" />
            </p::form::Field>
            <p::form::Field form=form field="tags"><p::form::Label>"Tags"</p::form::Label>
                <p::checkbox_group::Root value=tags required=true name="tags"
                    on_change=move |next| { tags.set(next); form.changed("tags"); }>
                    <p::checkbox_group::Item value="a" test_id="registered-tags-a">"Tag A"</p::checkbox_group::Item>
                    <p::checkbox_group::Item value="b" test_id="registered-tags-b">"Tag B"</p::checkbox_group::Item>
                </p::checkbox_group::Root><p::form::Message test_id="registered-error-tags" />
            </p::form::Field>
            <p::form::Field form=form field="radio"><p::form::Label>"Radio choice"</p::form::Label>
                <p::radio_group::Root value=radio required=true name="radio"
                    on_change=move |next| { radio.set(next); form.changed("radio"); }>
                    <p::radio_group::Item value="a" test_id="registered-radio-a">"Radio A"</p::radio_group::Item>
                    <p::radio_group::Item value="b" test_id="registered-radio-b">"Radio B"</p::radio_group::Item>
                </p::radio_group::Root><p::form::Message test_id="registered-error-radio" />
            </p::form::Field>
            <p::form::Field form=form field="toggle"><p::form::Label>"Standalone toggle"</p::form::Label>
                <p::toggle::Root pressed=toggle required=true name="toggle" on_change=move |next| { toggle.set(next); form.changed("toggle"); } test_id="registered-toggle">"Toggle"</p::toggle::Root>
                <p::form::Message test_id="registered-error-toggle" />
            </p::form::Field>
            <p::form::Field form=form field="formats"><p::form::Label>"Formats"</p::form::Label>
                <p::toggle_group::Root value=formats required=true name="formats" on_change=move |next| { formats.set(next); form.changed("formats"); }>
                    <p::toggle_group::Item value="a" test_id="registered-formats-a">"Format A"</p::toggle_group::Item>
                    <p::toggle_group::Item value="b" test_id="registered-formats-b">"Format B"</p::toggle_group::Item>
                </p::toggle_group::Root><p::form::Message test_id="registered-error-formats" />
            </p::form::Field>
            <p::form::Field form=form field="amount"><p::form::Label>"Amount"</p::form::Label>
                <p::slider::Root values=Signal::derive(move || vec![p::slider::ThumbValue { id: "amount".into(), value: amount.get() }])
                    on_change={move |values: Vec<p::slider::ThumbValue>| { amount.set(values[0].value); form.changed("amount"); }} name="amount">
                    <p::slider::Track><p::slider::Thumb id="amount" aria_label="Amount" test_id="registered-slider" /></p::slider::Track>
                </p::slider::Root><p::form::Message test_id="registered-error-amount" />
            </p::form::Field>
            <button type="submit" data-testid="form-choices-submit">"Save choices"</button>
        </p::form::Root>
    }
}
