//! Matched native-input workload for the data-driven menu.
use leptos::prelude::*;
use rustify_components::{Menu, MenuItem};
use rustify_ui::{mount, Anchor, AppHandle, MountConfig};
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
fn MenuBenchmark() -> impl IntoView {
    let state = State {
        accept: RwSignal::new(true),
        requests: RwSignal::new(0),
        accepts: RwSignal::new(0),
    };
    STATE.with(|slot| *slot.borrow_mut() = Some(state));
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(Anchor::Centred);
    let commands = RwSignal::new(0_u32);
    let last = RwSignal::new(String::new());
    let trigger = NodeRef::<leptos::html::Button>::new();
    view! {
        <section class="flex flex-col gap-4 p-4" aria-label="Menu benchmark">
            <h1>"Document commands"</h1>
            <button node_ref=trigger type="button" data-testid="bench-menu-open"
                aria-haspopup="menu" aria-expanded=move || open.get().to_string()
                on:click=move |_| {
                    if state.request() {
                        if let Some(element) = trigger.get() {
                            anchor.set(Anchor::element(&element.unchecked_into()));
                            open.set(true);
                        }
                    }
                }>"Document actions"</button>
            <p>"Choose a command for this document."</p>
            <Menu open=open anchor=anchor aria_label="Document actions" test_id="bench-menu"
                items=Signal::derive(|| vec![
                    MenuItem::heading("Document"),
                    MenuItem::new("bench-open", "Open"),
                    MenuItem::new("bench-unavailable", "Publish").disabled("Requires approval"),
                    MenuItem::new("bench-copy", "Duplicate"),
                    MenuItem::separator(),
                    MenuItem::new("bench-archive", "Archive"),
                ])
                on_open_change=move |value| { if state.request() { open.set(value); } }
                on_activate=move |value| {
                    commands.update(|n| *n += 1);
                    last.set(value);
                }
            />
            <p>"Last command: "<output data-testid="bench-last">{move || last.get()}</output></p>
            <details>
                <summary>"Benchmark diagnostics"</summary>
                <output data-testid="bench-requests">{move || state.requests.get()}</output>
                <output data-testid="bench-accepts">{move || state.accepts.get()}</output>
                <output data-testid="bench-commands">{move || commands.get()}</output>
                <output data-testid="bench-open">{move || open.get().to_string()}</output>
            </details>
        </section>
    }
}

#[wasm_bindgen]
pub fn menu_benchmark_mount() -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id("menu-benchmark")
        .ok_or_else(|| JsValue::from_str("benchmark container missing"))?
        .unchecked_into::<web_sys::HtmlElement>();
    let handle = mount(
        container,
        MountConfig {
            scope: "menu-benchmark".into(),
            ..Default::default()
        },
        || view! { <MenuBenchmark /> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    Ok(())
}

#[wasm_bindgen]
pub fn menu_benchmark_policy(accept: bool) {
    STATE.with(|slot| {
        if let Some(state) = *slot.borrow() {
            state.accept.set(accept);
        }
    });
}
