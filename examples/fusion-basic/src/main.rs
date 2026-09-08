#[cfg(target_arch = "wasm32")]
mod counter_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::counter_region::{CounterAction, CounterProps, CounterRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{mount, AppHandle, GpuRegion, MountConfig};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    pub use rustify_ui::makepad_widgets;

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: RefCell<u32> = const { RefCell::new(1) };
    }

    /// One counter shared by the DOM and two GPU regions.
    #[component]
    fn App(scope: String) -> impl IntoView {
        let (count, set_count) = signal(0i64);
        let props = Signal::derive(move || CounterProps { count: count.get() });
        let on_action = move |action| match action {
            CounterAction::Increment => set_count.update(|c| *c += 1),
        };
        let app = PhantomData::<CounterRegion>;
        let first = format!("{scope}-gpu-1");
        let second = format!("{scope}-gpu-2");
        view! {
            <div class="fusion-basic">
                <p>"DOM count: " <span data-testid=format!("{scope}-dom-count")>{count}</span></p>
                <button data-testid=format!("{scope}-dom-increment") on:click=move |_| set_count.update(|c| *c += 1)>
                    "DOM +1"
                </button>
                <GpuRegion app=app props=props on_action=on_action class="gpu-region" test_id=first />
                <GpuRegion app=app props=props on_action=on_action class="gpu-region" test_id=second />
            </div>
        }
    }

    /// Mounts one application scope into the element with `container_id` and
    /// returns a handle for `fusion_basic_dispose`.
    #[wasm_bindgen]
    pub fn fusion_basic_mount(container_id: &str) -> Result<u32, JsValue> {
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .unchecked_into::<leptos::web_sys::HtmlElement>();
        let config = MountConfig {
            scope: container_id.to_string(),
        };
        let scope = container_id.to_string();
        let handle = mount(container, config, move || view! { <App scope=scope /> })
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let id = NEXT_HANDLE.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    #[wasm_bindgen]
    pub fn fusion_basic_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
    }

    #[wasm_bindgen]
    pub fn fusion_basic_live_regions() -> u32 {
        rustify_makepad::live_region_count() as u32
    }
}

fn main() {}
