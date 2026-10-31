//! Native editing and submit/reset exercised through the shared parts.
use leptos::prelude::*;
use rustify_components::{primitives as p, Form};

#[component]
pub fn Fields() -> impl IntoView {
    let policy = RwSignal::new("accept".to_string());
    let disabled = RwSignal::new(false);
    let readonly = RwSignal::new(false);
    let text = RwSignal::new("accepted".to_string());
    let area = RwSignal::new("notes".to_string());
    let number = RwSignal::new(12.0);
    let color = RwSignal::new("#112233".to_string());
    let requests = RwSignal::new(Vec::<String>::new());
    let previews = RwSignal::new(Vec::<String>::new());
    let pending = RwSignal::new(Vec::<(String, String)>::new());
    let saves = RwSignal::new(0);
    let rules = RwSignal::new(0);
    let asked = RwSignal::new(Vec::<String>::new());
    let resets = RwSignal::new(0);
    let status = RwSignal::new("Not saved yet");
    let form = Form::new(&["text", "area", "number", "color"], move || {
        saves.update(|n| *n += 1);
        status.set("Saving");
    });
    let change = Callback::new(move |(field, value): (&'static str, String)| {
        requests.update(|requests| requests.push(format!("{field}:{value}")));
        match policy.get_untracked().as_str() {
            "refuse" => return,
            "delay" => {
                pending.update(|pending| pending.push((field.into(), value)));
                return;
            }
            _ => {}
        }
        match field {
            "text" => text.set(value),
            "area" => area.set(value),
            "number" => number.set(value.parse().unwrap()),
            "color" => color.set(value),
            _ => {}
        }
        form.changed(field);
        status.set("Unsaved changes");
    });
    view! {
        <section data-testid="native-fields">
            <h2>"Native fields"</h2>
            <label>"Acceptance"<select data-testid="field-policy" on:change=move |event| policy.set(event_target_value(&event))>
                <option value="accept">"Accept"</option><option value="refuse">"Refuse"</option><option value="delay">"Delay"</option>
            </select></label>
            <button type="button" data-testid="field-apply" on:click=move |_| {
                for (field, value) in pending.get_untracked() {
                    match field.as_str() { "text" => { text.set(value); form.changed("text"); }, "area" => { area.set(value); form.changed("area"); },
                        "number" => { number.set(value.parse().unwrap()); form.changed("number"); }, "color" => { color.set(value); form.changed("color"); }, _ => {} }
                }
                pending.set(Vec::new());
                status.set("Unsaved changes");
            }>"Apply delayed values"</button>
            <button type="button" data-testid="field-disabled" on:click=move |_| disabled.update(|value| *value = !*value)>"Toggle disabled"</button>
            <button type="button" data-testid="field-readonly" on:click=move |_| readonly.update(|value| *value = !*value)>"Toggle read only"</button>
            <button type="button" data-testid="field-saved" on:click=move |_| { if form.submitting() { form.submitted(Ok(())); status.set("Saved"); } }>"Complete save"</button>
            <button type="button" data-testid="field-failed" on:click=move |_| { if form.submitting() { form.submitted(Err("Try again".into())); status.set("Save failed"); } }>"Fail save"</button>
            <p role="status">"Save status: "<output data-testid="field-status">{move || if form.submitting() { "Saving" } else { status.get() }}</output></p>
            <dl class="sample-feedback">
                <div><dt>"Value requests:"</dt><dd><output data-testid="field-requests">{move || requests.get().join("|")}</output></dd></div>
                <div><dt>"Draft previews:"</dt><dd><output data-testid="field-previews">{move || previews.get().join("|")}</output></dd></div>
                <div><dt>"Submit history:"</dt><dd><output data-testid="field-asked">{move || asked.get().join("|")}</output></dd></div>
                <div><dt>"Save calls:"</dt><dd><output data-testid="field-saves">{move || saves.get()}</output></dd></div>
                <div><dt>"Rule checks:"</dt><dd><output data-testid="field-rules">{move || rules.get()}</output></dd></div>
                <div><dt>"Reset requests:"</dt><dd><output data-testid="field-resets">{move || resets.get()}</output></dd></div>
            </dl>
            <p data-testid="field-failure">{move || form.failure().unwrap_or_default()}</p>
            <p::form::Root form=form id="native-form" test_id="native-form"
                rules=move || { rules.update(|n| *n += 1); Vec::new() }
                on_asked=Callback::new(move |result| asked.update(|asked| asked.push(format!("{result:?}"))))
                on_reset=Callback::new(move |_| {
                    resets.update(|n| *n += 1);
                    if policy.get_untracked() == "accept" {
                        text.set("accepted".into()); area.set("notes".into()); number.set(12.0); color.set("#112233".into()); form.reset();
                        status.set("Not saved yet");
                    }
                })>
                <p::form::Field form=form field="text">
                    <p::form::Label>"Title"</p::form::Label><p::form::Description>"Application controlled"</p::form::Description>
                    <p::text_field::Root value=text on_change=move |value| change.run(("text", value))
                        name="title" required=true disabled=disabled read_only=readonly>
                        <p::text_field::Slot>"Prefix"</p::text_field::Slot>
                        {p::text_field::control_as(leptos::html::input().attr("data-testid", "field-text")
                            .attr("autocomplete", "nickname").attr("pattern", "[a-zA-Z\\p{Script=Han} !]+")
                            .on(leptos::ev::input, move |event| { if event_target_value(&event) == "cancel" { event.prevent_default(); } }))}
                    </p::text_field::Root><p::form::Message test_id="error-text" />
                </p::form::Field>
                <p::form::Field form=form field="area"><p::form::Label>"Notes"</p::form::Label><p::text_area::Root value=area
                    on_change=move |value| change.run(("area", value)) name="notes" test_id="field-area" disabled=disabled read_only=readonly /><p::form::Message /></p::form::Field>
                <p::form::Field form=form field="number"><p::form::Label>"Number"</p::form::Label><p::number_field::Root value=number
                    on_change=move |value: f64| change.run(("number", value.to_string()))
                    on_preview=Callback::new(move |value: f64| previews.update(|all| all.push(format!("number:{value}"))))
                    name="number" required=true disabled=disabled read_only=readonly min=0.0 max=20.0>
                    <p::number_field::Input test_id="field-number" />
                </p::number_field::Root><p::form::Message /></p::form::Field>
                <p::form::Field form=form field="color"><p::form::Label>"Color"</p::form::Label><p::color_field::Root value=color
                    on_change=move |value| change.run(("color", value)) name="color" required=true disabled=disabled read_only=readonly>
                    <p::color_field::Swatch test_id="field-swatch" /><p::color_field::Input test_id="field-color" />
                </p::color_field::Root><p::form::Message /></p::form::Field>
                <button type="submit" data-testid="field-submit">"Save"</button><button type="reset" data-testid="field-reset">"Reset"</button>
            </p::form::Root>
        </section>
    }
}
