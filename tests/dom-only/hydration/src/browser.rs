use crate::view::{CatalogView, State};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use rustify_components::geometry::{provide_geometry_styles, GeometryStyles};
use rustify_ui::AppHandle;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

thread_local! {
    static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
    static STATES: RefCell<BTreeMap<String, State>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(1) };
}

pub(crate) fn register(scope: String, state: State) {
    STATES.with(|states| states.borrow_mut().insert(scope.clone(), state));
    on_cleanup(move || {
        STATES.with(|states| states.borrow_mut().remove(&scope));
    });
}

#[wasm_bindgen]
pub fn harness_start(container_id: &str, scope: &str) -> Result<u32, JsValue> {
    let container = document()
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("container not found"))?
        .unchecked_into::<leptos::web_sys::HtmlElement>();
    attach(container, scope)
}

#[wasm_bindgen]
pub fn harness_attach(
    container: leptos::web_sys::HtmlElement,
    scope: &str,
) -> Result<u32, JsValue> {
    attach(container, scope)
}

fn attach(container: leptos::web_sys::HtmlElement, scope: &str) -> Result<u32, JsValue> {
    let scope = scope.to_string();
    let config = crate::config(scope.clone());
    let author_view = move || {
        #[cfg(feature = "csr")]
        rustify_ui::id::provide(&scope);
        provide_geometry_styles(GeometryStyles::default());
        view! { <CatalogView scope=scope /> }
    };
    #[cfg(feature = "hydrate")]
    let handle = rustify_ui::hydrate(container, config, author_view);
    #[cfg(feature = "csr")]
    let handle = rustify_ui::mount(container, config, author_view);
    let handle = handle.map_err(|error| JsValue::from_str(&format!("{error:?}: {error}")))?;
    let id = NEXT_HANDLE.with(|next| next.replace(next.get() + 1));
    HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
    Ok(id)
}

#[cfg(feature = "csr")]
#[wasm_bindgen]
pub fn harness_geometry_mount(
    container: leptos::web_sys::HtmlElement,
    seed: &str,
) -> Result<u32, JsValue> {
    let seed = seed.to_string();
    let handle = rustify_ui::mount(container, crate::config(seed.clone()), move || {
        rustify_ui::id::provide(&seed);
        // Deliberately no provide_geometry_styles: each Show creates its own fallback.
        view! { <crate::geometry_branches::GeometryBranches/> }
    })
    .map_err(|error| JsValue::from_str(&format!("{error:?}: {error}")))?;
    let id = NEXT_HANDLE.with(|next| next.replace(next.get() + 1));
    HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
    Ok(id)
}

#[wasm_bindgen]
pub fn harness_dispose(handle: u32) -> bool {
    HANDLES
        .with(|handles| handles.borrow_mut().remove(&handle))
        .is_some()
}

#[wasm_bindgen]
pub fn harness_state(scope: &str) -> Option<String> {
    STATES
        .with(|states| states.borrow().get(scope).copied())
        .map(State::snapshot)
}

#[wasm_bindgen]
pub fn harness_async(scope: &str) -> bool {
    let state = STATES.with(|states| states.borrow().get(scope).copied());
    if let Some(state) = state {
        state.asynchronous.set("pending".to_string());
        rustify_ui::defer_after(25, move || {
            state.asynchronous.try_set("complete".to_string());
        });
        true
    } else {
        false
    }
}

#[wasm_bindgen]
pub fn harness_identify(runtime: u32) {
    rustify_ui::identify_runtime(runtime, "hydration-harness");
}

#[wasm_bindgen]
pub fn harness_diagnostics() -> String {
    rustify_ui::report_json()
}

#[wasm_bindgen]
pub fn harness_trap_deferred() {
    rustify_ui::defer(|| unreachable!("deliberate hydration harness trap"));
}
