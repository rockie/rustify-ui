use leptos::prelude::*;
use p::form::FormRoot;
use rustify_components as p;
use rustify_components::primitives::{CheckedState, SelectionMode};
use rustify_components::Form;
use rustify_ui::{mount, AppHandle, MountConfig};
use std::{cell::RefCell, collections::BTreeMap, fmt::Debug};
use wasm_bindgen::{prelude::*, JsCast};

thread_local! {
    static HANDLE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
    static POLICIES: RefCell<BTreeMap<String, RwSignal<bool>>> = const { RefCell::new(BTreeMap::new()) };
}

struct Controlled<T: Clone + Send + Sync + 'static> {
    value: RwSignal<T>,
    accept: RwSignal<bool>,
    requests: RwSignal<Vec<String>>,
}
impl<T: Clone + Send + Sync + 'static> Copy for Controlled<T> {}
impl<T: Clone + Send + Sync + 'static> Clone for Controlled<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Clone + Debug + Send + Sync + 'static> Controlled<T> {
    fn new(name: &'static str, value: T) -> Self {
        let accept = RwSignal::new(true);
        POLICIES.with(|policies| policies.borrow_mut().insert(name.into(), accept));
        Self {
            value: RwSignal::new(value),
            accept,
            requests: RwSignal::new(Vec::new()),
        }
    }
    fn request(self, value: T) {
        self.requests
            .update(|requests| requests.push(format!("{value:?}")));
        if self.accept.get_untracked() {
            self.value.set(value);
        }
    }
    fn output(self, name: &'static str) -> impl IntoView {
        view! { <details><summary>"Application state"</summary><dl data-parity=name>
            <dt>"Application value"</dt><dd data-value>{move || format!("{:?}", self.value.get())}</dd>
            <dt>"Requests"</dt><dd data-requests>{move || self.requests.get().join("\n")}</dd>
        </dl></details> }
    }
}

#[component]
fn Parity() -> impl IntoView {
    let checkbox = Controlled::new("checkbox", CheckedState::Unchecked);
    let mixed = Controlled::new("checkbox-mixed", CheckedState::Indeterminate);
    let checks = Controlled::new("checkbox-group", vec!["a".to_string()]);
    let radio = Controlled::new("radio-group", Some("a".to_string()));
    let switch = Controlled::new("switch", false);
    let toggle = Controlled::new("toggle", false);
    let toggles = Controlled::new("toggle-group", vec!["a".to_string()]);
    let slider = Controlled::new(
        "slider",
        vec![p::slider::ThumbValue {
            id: "single".into(),
            value: 20.,
        }],
    );
    let range = Controlled::new(
        "range",
        vec![
            p::slider::ThumbValue {
                id: "start".into(),
                value: 20.,
            },
            p::slider::ThumbValue {
                id: "end".into(),
                value: 70.,
            },
        ],
    );
    let select = Controlled::new("select", Some("a".to_string()));
    let open = Controlled::new("select-open", false);
    let text = Controlled::new("text-field", "Accepted".to_string());
    let area = Controlled::new("textarea", "Notes".to_string());
    let number = Controlled::new("number-field", 12.0);
    let color = Controlled::new("color-field", "#112233".to_string());
    let resets = Controlled::new("form", false);
    let hover = Controlled::new("hover-card", false);
    let submits = RwSignal::new(0_u32);
    let form = Form::new(&["text", "area", "number", "color"], move || {
        submits.update(|n| *n += 1)
    });
    view! {
        <h1>"Controlled component parity"</h1>
        <section aria-label="Choices">
            <h2>"Choices"</h2>
            <p::checkbox::Root checked=checkbox.value on_change=move |value|checkbox.request(value) aria_label="Checkbox">
                <p::checkbox::Control test_id="checkbox"/><p::checkbox::Indicator>"✓"</p::checkbox::Indicator>
            </p::checkbox::Root>{checkbox.output("checkbox")}
            <p::checkbox::Root checked=mixed.value on_change=move |value|mixed.request(value) aria_label="Mixed checkbox">
                <p::checkbox::Control test_id="checkbox-mixed"/><p::checkbox::Indicator>"−"</p::checkbox::Indicator>
            </p::checkbox::Root>{mixed.output("checkbox-mixed")}
            <p::checkbox_group::Root value=checks.value on_change=move |value|checks.request(value) aria_label="Features">
                <p::checkbox_group::Item value="a" test_id="checkbox-group-a">"Alpha"</p::checkbox_group::Item>
                <p::checkbox_group::Item value="b" test_id="checkbox-group-b">"Beta"</p::checkbox_group::Item>
            </p::checkbox_group::Root>{checks.output("checkbox-group")}
            <p::radio_group::Root value=radio.value on_change=move |value|radio.request(value) aria_label="Choice">
                <p::radio_group::Item value="a" test_id="radio-group-a">"Alpha"</p::radio_group::Item>
                <p::radio_group::Item value="b" test_id="radio-group-b">"Beta"</p::radio_group::Item>
            </p::radio_group::Root>{radio.output("radio-group")}
            <p::switch::Root checked=switch.value on_change=move |value|switch.request(value) aria_label="Notifications">
                <p::switch::Control test_id="switch"/><p::switch::Thumb/>
            </p::switch::Root>{switch.output("switch")}
            <p::toggle::Root pressed=toggle.value on_change=move |value|toggle.request(value) test_id="toggle">"Bold"</p::toggle::Root>{toggle.output("toggle")}
            <p::toggle_group::Root value=toggles.value on_change=move |value|toggles.request(value) mode=SelectionMode::Multiple aria_label="Formatting">
                <p::toggle_group::Item value="a" test_id="toggle-group-a">"Bold"</p::toggle_group::Item>
                <p::toggle_group::Item value="b" test_id="toggle-group-b">"Italic"</p::toggle_group::Item>
            </p::toggle_group::Root>{toggles.output("toggle-group")}
        </section>
        <section aria-label="Sliders"><h2>"Sliders"</h2>
            <p::slider::Root values=slider.value on_change=move |value|slider.request(value)>
                <p::slider::Track><p::slider::Range/><p::slider::Thumb id="single" aria_label="Amount" test_id="slider"/></p::slider::Track>
            </p::slider::Root>{slider.output("slider")}
            <p::slider::Root values=range.value on_change=move |value|range.request(value)>
                <p::slider::Track><p::slider::Range/>
                    <p::slider::Thumb id="start" aria_label="Range start" test_id="range-start"/>
                    <p::slider::Thumb id="end" aria_label="Range end" test_id="range-end"/>
                </p::slider::Track>
            </p::slider::Root>{range.output("range")}
        </section>
        <section aria-label="Select"><h2>"Select"</h2>
            <p::select::Root value=select.value on_change=move |value|select.request(value)
                open=open.value on_open_change=move |value|open.request(value) aria_label="Fruit"
                entries=vec![p::select::SelectEntry::new("a","Apricot"),p::select::SelectEntry::new("b","Banana")]>
                <p::select::Trigger test_id="select"><p::select::Value/></p::select::Trigger>
                <p::select::Content test_id="select-content"><p::select::Viewport>
                    <p::select::Item value="a" test_id="select-a">"Apricot"</p::select::Item>
                    <p::select::Item value="b" test_id="select-b">"Banana"</p::select::Item>
                </p::select::Viewport></p::select::Content>
            </p::select::Root>{select.output("select")}{open.output("select-open")}
        </section>
        <section aria-label="Form"><h2>"Form fields"</h2>
            <FormRoot form=form rules=Vec::new id="parity-form" on_reset=Callback::new(move |_| {
                resets.request(true);
                if resets.accept.get_untracked() { text.value.set("Accepted".into()); form.reset(); }
            })>
                <p::Field form=form field="text" label="Title" description="Application controlled"
                    control=move |_| view! {
                        <p::text_field::Root value=text.value on_change=move |value| {text.request(value);if text.accept.get_untracked() {form.changed("text");}} name="title">
                            <p::text_field::Input test_id="text-field"/>
                        </p::text_field::Root>
                    }.into_any()/>
                <p::Field form=form field="area" label="Notes" control=move |_| view! {
                    <p::text_area::Root value=area.value on_change=move |value| {area.request(value);if area.accept.get_untracked() {form.changed("area");}} name="notes" test_id="textarea"/>
                }.into_any()/>
                <p::Field form=form field="number" label="Quantity" control=move |_| view! {
                    <p::number_field::Root value=number.value on_change=move |value| {number.request(value);if number.accept.get_untracked() {form.changed("number");}} name="quantity">
                        <p::number_field::Input test_id="number-field"/>
                    </p::number_field::Root>
                }.into_any()/>
                <p::Field form=form field="color" label="Color" control=move |_| view! {
                    <p::color_field::Root value=color.value on_change=move |value| {color.request(value);if color.accept.get_untracked() {form.changed("color");}} name="color">
                        <p::color_field::Input test_id="color-field"/>
                    </p::color_field::Root>
                }.into_any()/>
                <button type="reset" data-testid="form-reset">"Restore title"</button>
                <button type="submit" data-testid="form-submit">"Submit"</button>
            </FormRoot>
            {text.output("text-field")}{area.output("textarea")}{number.output("number-field")}{color.output("color-field")}{resets.output("form")}
            <output data-testid="form-submits">{move || submits.get()}</output>
        </section>
        <section aria-label="Reading card"><h2>"Reading card"</h2>
            <p::hover_card::Root open=hover.value on_open_change=move |value|hover.request(value)>
                <p::hover_card::Trigger href="#parity" test_id="hover-card">"Read owner profile"</p::hover_card::Trigger>
                <p::hover_card::Portal><p::hover_card::Content test_id="hover-card-content">"Morgan reviews task changes."</p::hover_card::Content></p::hover_card::Portal>
            </p::hover_card::Root>{hover.output("hover-card")}
        </section>
        <button type="button" data-testid="outside">"Outside the form"</button>
    }
}

#[wasm_bindgen]
pub fn parity_mount() -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id("parity")
        .ok_or_else(|| JsValue::from_str("parity container missing"))?
        .unchecked_into::<web_sys::HtmlElement>();
    let handle = mount(
        container,
        MountConfig {
            scope: "parity".into(),
            ..Default::default()
        },
        || view! {<Parity/>},
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    Ok(())
}
#[wasm_bindgen]
pub fn parity_policy(component: &str, policy: &str) -> Result<(), JsValue> {
    let accept = match policy {
        "accept" => true,
        "reject" => false,
        _ => return Err(JsValue::from_str("policy must be accept or reject")),
    };
    POLICIES.with(|policies| {
        let policies = policies.borrow();
        let signal = policies
            .get(component)
            .ok_or_else(|| JsValue::from_str("unknown parity component"))?;
        signal.set(accept);
        Ok(())
    })
}
#[wasm_bindgen]
pub fn parity_dispose() {
    HANDLE.with(|slot| slot.borrow_mut().take());
    POLICIES.with(|policies| policies.borrow_mut().clear());
}
