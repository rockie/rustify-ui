//! Native-input workloads retained across the panel component migration.
use leptos::prelude::*;
use rustify_components::{
    Button, Dialog, Progress, ScrollArea, Tab, TabPanel, Tabs, Toaster, Tooltip,
};
use rustify_ui::{mount, provide_toasts, AppHandle, MountConfig, ToastOptions};
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Copy)]
struct State {
    accept: RwSignal<bool>,
    requests: RwSignal<u32>,
    accepts: RwSignal<u32>,
}
impl State {
    fn request(self) -> bool {
        self.requests.update(|n| *n += 1);
        if self.accept.get_untracked() {
            self.accepts.update(|n| *n += 1);
            true
        } else {
            false
        }
    }
}
thread_local! {
    static HANDLE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}
#[component]
fn Panels() -> impl IntoView {
    let state = State {
        accept: RwSignal::new(true),
        requests: RwSignal::new(0),
        accepts: RwSignal::new(0),
    };
    STATE.with(|slot| *slot.borrow_mut() = Some(state));
    let dialog = RwSignal::new(false);
    let tooltip = RwSignal::new(false);
    let active = RwSignal::new("summary".to_string());
    let progress = RwSignal::new(0.0);
    let toasts = provide_toasts();
    let serial = RwSignal::new(0);
    view! {
        <section class="flex flex-col gap-4 p-4" aria-label="Panel benchmark">
            <h1>"Application panel"</h1>
            <Button test_id="bench-dialog-open" on_click=move || { if state.request() { dialog.set(true); } }>"Edit entry"</Button>
            <Dialog open=dialog title="Edit entry" test_id="bench-dialog" on_open_change=move |value| { if state.request() { dialog.set(value); } }>
                <p>"Review this local entry, then close to return to the panel."</p>
            </Dialog>
            <Tabs active=active tabs=Signal::derive(|| vec![Tab::new("summary", "Summary"), Tab::new("details", "Details")]) aria_label="Entry views"
                on_activate=move |value| { if state.request() { active.set(value); } }>
                <TabPanel value="summary">"Entry summary"</TabPanel>
                <TabPanel value="details">"Entry details"</TabPanel>
            </Tabs>
            <Tooltip open=tooltip label="More information about this entry" test_id="bench-tooltip"
                on_open_change=move |value| { if state.request() { tooltip.set(value); } }
                trigger=leptos::html::button().r#type("button").attr("data-testid","bench-tooltip-trigger").child("Entry information") />
            <Button test_id="bench-toast-show" on_click=move || { if state.request() { serial.update(|n| *n += 1); toasts.show(format!("Saved {}", serial.get_untracked()), ToastOptions { duration_ms: 1000, ..Default::default() }); } }>"Save entry"</Button>
            <Button test_id="bench-progress-update" on_click=move || { if state.request() { progress.update(|n| *n = (*n + 1.0) % 100.0); } }>"Advance progress"</Button>
            <Progress value=progress aria_label="Entry progress" test_id="bench-progress" />
            <ScrollArea aria_label="Entry history" test_id="bench-scroll">
                {(0..100).map(|i| view! { <p>{format!("History entry {i}")}</p> }).collect_view()}
            </ScrollArea>
            <output data-testid="bench-requests">{move || state.requests.get()}</output>
            <output data-testid="bench-accepts">{move || state.accepts.get()}</output>
            <output data-testid="bench-active">{move || active.get()}</output>
            <output data-testid="bench-dialog-state">{move || dialog.get().to_string()}</output>
            <output data-testid="bench-tooltip-state">{move || tooltip.get().to_string()}</output>
            <output data-testid="bench-progress-state">{move || progress.get()}</output>
            <Toaster test_id="bench-toaster" />
            <crate::panels::Panels />
            <crate::content::Content />
            <crate::feedback::Feedback />
        </section>
    }
}
#[wasm_bindgen]
pub fn panel_benchmark_mount() -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id("panel-benchmark")
        .ok_or_else(|| JsValue::from_str("benchmark container missing"))?
        .unchecked_into::<web_sys::HtmlElement>();
    let handle = mount(
        container,
        MountConfig {
            scope: "panel-benchmark".into(),
            ..Default::default()
        },
        || view! { <Panels /> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    Ok(())
}
#[wasm_bindgen]
pub fn panel_benchmark_policy(accept: bool) {
    STATE.with(|slot| {
        if let Some(state) = *slot.borrow() {
            state.accept.set(accept);
        }
    });
}
