//! The same scalar control workload is used before and after the refactor.

use leptos::prelude::*;
use rustify_components::{Checkbox, Select, SelectOption, Slider, TextField};
use rustify_ui::{mount, AppHandle, MountConfig};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
struct State {
    checked: RwSignal<bool>,
    text: RwSignal<String>,
    slider: RwSignal<f64>,
    selected: RwSignal<String>,
    open: RwSignal<bool>,
    accept: RwSignal<bool>,
    read_only: RwSignal<bool>,
    disabled: RwSignal<bool>,
    requests: RwSignal<u32>,
    accepted: RwSignal<u32>,
}

impl State {
    fn request(self) -> bool {
        self.requests.update(|n| *n += 1);
        if self.accept.get_untracked() {
            self.accepted.update(|n| *n += 1);
            true
        } else {
            false
        }
    }

    fn reset(self) {
        self.checked.set(false);
        self.text.set("initial".into());
        self.slider.set(50.0);
        self.selected.set("alpha".into());
        self.open.set(false);
        self.accept.set(true);
        self.read_only.set(false);
        self.disabled.set(false);
        self.requests.set(0);
        self.accepted.set(0);
    }
}

thread_local! {
    static BENCH: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

#[component]
fn Controls() -> impl IntoView {
    let state = State {
        checked: RwSignal::new(false),
        text: RwSignal::new("initial".into()),
        slider: RwSignal::new(50.0),
        selected: RwSignal::new("alpha".into()),
        open: RwSignal::new(false),
        accept: RwSignal::new(true),
        read_only: RwSignal::new(false),
        disabled: RwSignal::new(false),
        requests: RwSignal::new(0),
        accepted: RwSignal::new(0),
    };
    STATE.with(|slot| *slot.borrow_mut() = Some(state));
    view! {
        <section aria-label="Scalar form controls" class="flex flex-col gap-4 p-4">
            <h1>"Form control benchmark"</h1>
            <label for="bench-check">"Enabled"</label>
            <Checkbox id="bench-check" test_id="bench-checkbox" checked=state.checked
                disabled=state.disabled read_only=state.read_only
                on_change=move |value| { if state.request() { state.checked.set(value); } } />
            <label for="bench-text">"Name"</label>
            <TextField id="bench-text" test_id="bench-text" value=state.text
                disabled=state.disabled read_only=state.read_only
                on_change=move |value| { if state.request() { state.text.set(value); } } />
            <label for="bench-slider">"Amount"</label>
            <Slider id="bench-slider" test_id="bench-slider" value=state.slider
                disabled=state.disabled read_only=state.read_only
                on_change=move |value| { if state.request() { state.slider.set(value); } } />
            <label for="bench-select">"Choice"</label>
            <Select id="bench-select" test_id="bench-select" value=state.selected
                options=Signal::derive(|| vec![SelectOption::new("alpha", "Alpha"), SelectOption::new("beta", "Beta")])
                open=state.open on_open_change=move |value| state.open.set(value)
                disabled=state.disabled read_only=state.read_only
                on_change=move |value| { if state.request() { state.selected.set(value); } } />
            <output data-testid="accepted-check">{move || state.checked.get().to_string()}</output>
            <output data-testid="accepted-text">{move || state.text.get()}</output>
            <output data-testid="accepted-slider">{move || state.slider.get().to_string()}</output>
            <output data-testid="accepted-select">{move || state.selected.get()}</output>
            <output data-testid="requests">{move || state.requests.get()}</output>
            <output data-testid="accepts">{move || state.accepted.get()}</output>
        </section>
    }
}

#[wasm_bindgen]
pub fn form_benchmark_mount() -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id("form-benchmark")
        .ok_or_else(|| JsValue::from_str("benchmark container missing"))?
        .unchecked_into::<web_sys::HtmlElement>();
    let handle = mount(
        container,
        MountConfig {
            scope: "form-benchmark".into(),
            ..MountConfig::default()
        },
        move || view! { <Controls /> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    BENCH.with(|bench| *bench.borrow_mut() = Some(handle));
    Ok(())
}

#[wasm_bindgen]
pub fn form_benchmark_reset() {
    STATE.with(|bench| {
        if let Some(state) = bench.borrow().as_ref() {
            state.reset();
        }
    });
}

#[wasm_bindgen]
pub fn form_benchmark_policy(accept: bool, read_only: bool, disabled: bool) {
    STATE.with(|bench| {
        if let Some(state) = bench.borrow().as_ref() {
            state.accept.set(accept);
            state.read_only.set(read_only);
            state.disabled.set(disabled);
        }
    });
}
