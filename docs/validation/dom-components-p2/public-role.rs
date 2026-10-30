#[cfg(target_arch = "wasm32")]
mod browser {
use leptos::prelude::*;
use rustify_components::Form;
use rustify_components::primitives::CheckedState;
#[cfg(not(feature="themed"))]
use rustify_components::primitives::{form::{Root as FormRoot,Field,Label,Description,Message},text_field,checkbox};
#[cfg(feature="themed")]
use rustify_components::{FormRoot,form::field::{Root as Field,Label,Description,Message},text_field,checkbox};
use wasm_bindgen::{prelude::*,JsCast};
use std::cell::RefCell;
thread_local! { static APP: RefCell<Option<rustify_ui::AppHandle>> = const {RefCell::new(None)}; }
#[component]
fn LocalForm() -> impl IntoView {
    let title=RwSignal::new(String::new()); let agree=RwSignal::new(CheckedState::Indeterminate);
    let refuse=RwSignal::new(false); let calls=RwSignal::new(0); let saved=RwSignal::new(0);
    let done=StoredValue::new(None::<Form>);
    let form=Form::new(&["title","agree"],move || { saved.update(|n| *n+=1); if let Some(f)=done.get_value() { f.submitted(Ok(())); } });
    done.set_value(Some(form));
    view! {
        <h1>"Public form recipe"</h1>
        <button type="button" data-testid="refuse" aria-pressed=move || refuse.get().to_string() on:click=move |_| refuse.update(|v| *v=!*v)>"Refuse requests"</button>
        <FormRoot form=form id="role-form" rules=Vec::new on_reset=Callback::new(move |_| {title.set(String::new()); agree.set(CheckedState::Indeterminate); form.reset();})>
            <Field form=form field="title"><Label>"Title"</Label><Description>"A required public field"</Description>
                <text_field::Root value=title on_change=move |value| {calls.update(|n| *n+=1); if !refuse.get_untracked() {title.set(value);form.changed("title");}} name="title" required=true>
                    <text_field::Input test_id="title" />
                </text_field::Root><Message test_id="title-error" />
            </Field>
            <Field form=form field="agree"><Label>"Agreement"</Label>
                <checkbox::Root checked=agree on_change=move |value| {calls.update(|n| *n+=1);if !refuse.get_untracked() {agree.set(value);form.changed("agree");}} name="agree" required=true>
                    <checkbox::Control test_id="agree" /><checkbox::Indicator>"✓"</checkbox::Indicator>
                </checkbox::Root><Message test_id="agree-error" />
            </Field>
            <button type="submit" data-testid="save">"Save locally"</button><button type="reset" data-testid="reset">"Reset"</button>
            <output data-testid="calls">{move || calls.get()}</output><output data-testid="saved">{move || saved.get()}</output>
        </FormRoot>
    }
}
#[wasm_bindgen]
pub fn role_mount() -> Result<(), JsValue> {
    let container=web_sys::window().unwrap().document().unwrap().get_element_by_id("app").unwrap().unchecked_into();
    let handle=rustify_ui::mount(container,rustify_ui::MountConfig{scope:"public-role".into(),..Default::default()},|| view!{<LocalForm/>}).map_err(|e| JsValue::from_str(&e.to_string()))?;
    APP.with(|app| *app.borrow_mut()=Some(handle)); Ok(())
}
}
