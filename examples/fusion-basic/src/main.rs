#[cfg(target_arch = "wasm32")]
mod anchor_grid;
#[cfg(target_arch = "wasm32")]
mod anchor_region;
#[cfg(target_arch = "wasm32")]
mod counter_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::anchor_grid::Anchor;
    use super::anchor_region::{AnchorAction, AnchorRegion};
    use super::counter_region::{CounterAction, CounterProps, CounterRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{mount, AppHandle, GpuRegion, MountConfig, RegionState};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    pub use rustify_ui::makepad_widgets;

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: RefCell<u32> = const { RefCell::new(1) };
        /// Last state each live region published, so the page can report it
        /// without reading signals that belong to a disposed scope.
        static REGION_STATES: RefCell<BTreeMap<String, &'static str>> =
            const { RefCell::new(BTreeMap::new()) };
        /// What the geometry fixture reports about its region, derived from
        /// its signals by an effect.
        static GEOMETRY: RefCell<String> = const { RefCell::new(String::new()) };
    }

    fn state_name(state: RegionState) -> &'static str {
        match state {
            RegionState::Starting => "starting",
            RegionState::Ready => "ready",
            RegionState::Suspended => "suspended",
            RegionState::Failed(_) => "failed",
            RegionState::Disposed => "disposed",
        }
    }

    /// One GPU region plus the application's own failure notice for it.
    #[component]
    fn CounterSlot(
        test_id: String,
        props: Signal<CounterProps>,
        on_action: impl Fn(CounterAction) + Clone + Send + Sync + 'static,
    ) -> impl IntoView {
        let state = RwSignal::new(RegionState::Starting);
        let key = test_id.clone();
        Effect::new(move || {
            let name = state_name(state.get());
            REGION_STATES.with(|states| states.borrow_mut().insert(key.clone(), name));
        });
        let key = test_id.clone();
        on_cleanup(move || {
            REGION_STATES.with(|states| states.borrow_mut().remove(&key));
        });
        let error_id = format!("{test_id}-error");
        let app = PhantomData::<CounterRegion>;
        view! {
            <GpuRegion
                app=app
                props=props
                on_action=on_action
                state=state
                class="gpu-region"
                test_id=test_id
            />
            {move || match state.get() {
                RegionState::Failed(error) => Some(view! {
                    <p class="region-error" role="alert" data-testid=error_id.clone()>
                        {error.to_string()}
                    </p>
                }),
                _ => None,
            }}
        }
    }

    /// One counter shared by the DOM and two GPU regions.
    #[component]
    fn App(scope: String) -> impl IntoView {
        let (count, set_count) = signal(0i64);
        let props = Signal::derive(move || CounterProps { count: count.get() });
        let on_action = move |action| match action {
            CounterAction::Increment => set_count.update(|c| *c += 1),
        };
        view! {
            <div class="fusion-basic">
                <p>"DOM count: " <span data-testid=format!("{scope}-dom-count")>{count}</span></p>
                <button data-testid=format!("{scope}-dom-increment") on:click=move |_| set_count.update(|c| *c += 1)>
                    "DOM +1"
                </button>
                <CounterSlot test_id=format!("{scope}-gpu-1") props=props on_action=on_action />
                <CounterSlot test_id=format!("{scope}-gpu-2") props=props on_action=on_action />
            </div>
        }
    }

    fn anchors_json(anchors: &[Anchor]) -> String {
        let body: Vec<String> = anchors
            .iter()
            .map(|a| {
                format!(
                    "{{\"id\":{},\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
                    a.id, a.x, a.y, a.width, a.height
                )
            })
            .collect();
        format!("[{}]", body.join(","))
    }

    /// A GPU region inside two nested scroll containers, clipped by the inner
    /// one. The fixture owns the scrolling structure so the geometry under
    /// test lives with the code that has to survive it.
    #[component]
    fn GeometryFixture() -> impl IntoView {
        let anchors = RwSignal::new(Vec::<Anchor>::new());
        let hits = RwSignal::new(0u32);
        let last_hit = RwSignal::new(None::<(usize, f64, f64)>);
        let state = RwSignal::new(RegionState::Starting);

        Effect::new(move || {
            let report = format!(
                "{{\"anchors\":{},\"hits\":{},\"last_hit\":{},\"state\":\"{}\"}}",
                anchors.with(|anchors| anchors_json(anchors)),
                hits.get(),
                last_hit
                    .get()
                    .map(|(anchor, x, y)| format!(
                        "{{\"anchor\":{anchor},\"x\":{x:.3},\"y\":{y:.3}}}"
                    ))
                    .unwrap_or_else(|| "null".to_string()),
                state_name(state.get()),
            );
            GEOMETRY.with(|slot| *slot.borrow_mut() = report);
        });

        let on_action = move |action| match action {
            AnchorAction::Layout(reported) => anchors.set(reported),
            AnchorAction::Hit(hit) => {
                hits.update(|n| *n += 1);
                last_hit.set(Some((hit.anchor, hit.x, hit.y)));
            }
        };
        let app = PhantomData::<AnchorRegion>;
        let props = Signal::derive(|| ());
        view! {
            <div class="geometry-outer" data-testid="geometry-outer">
                <div class="geometry-tall">
                    <div class="geometry-inner" data-testid="geometry-inner">
                        <div class="geometry-wide">
                            <GpuRegion
                                app=app
                                props=props
                                on_action=on_action
                                state=state
                                class="anchor-region"
                                test_id="geometry-gpu"
                            />
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    /// Mounts one scope into the element with `container_id` and returns a
    /// handle for `fusion_basic_dispose`.
    fn mount_scope<F, N>(container_id: &str, view: F) -> Result<u32, JsValue>
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
        N::State: 'static,
    {
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .unchecked_into::<leptos::web_sys::HtmlElement>();
        let config = MountConfig {
            scope: container_id.to_string(),
        };
        let handle =
            mount(container, config, view).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let id = NEXT_HANDLE.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    #[wasm_bindgen]
    pub fn fusion_basic_mount(container_id: &str) -> Result<u32, JsValue> {
        let scope = container_id.to_string();
        mount_scope(container_id, move || view! { <App scope=scope /> })
    }

    /// `{"anchors":[...],"hits":n,"last_hit":{...}|null,"state":"..."}` for the
    /// geometry fixture's region.
    #[wasm_bindgen]
    pub fn fusion_basic_geometry() -> String {
        GEOMETRY.with(|slot| slot.borrow().clone())
    }

    /// Mounts the geometry fixture into `container_id`.
    #[wasm_bindgen]
    pub fn fusion_basic_geometry_mount(container_id: &str) -> Result<u32, JsValue> {
        mount_scope(container_id, || view! { <GeometryFixture /> })
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

    /// `{"<region test id>": "<state>"}` for every region currently in a view.
    #[wasm_bindgen]
    pub fn fusion_basic_region_states() -> String {
        REGION_STATES.with(|states| {
            let states = states.borrow();
            let body: Vec<String> = states
                .iter()
                .map(|(key, state)| format!("\"{key}\":\"{state}\""))
                .collect();
            format!("{{{}}}", body.join(","))
        })
    }
}

fn main() {}
