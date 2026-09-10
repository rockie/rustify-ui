#[cfg(target_arch = "wasm32")]
mod anchor_grid;
#[cfg(target_arch = "wasm32")]
mod anchor_region;
#[cfg(target_arch = "wasm32")]
mod counter_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::anchor_grid::DEFAULT_COLOR;
    use super::anchor_grid::{Anchor as GridAnchor, COLUMNS, ROWS};
    use super::anchor_region::{AnchorAction, AnchorProps, AnchorRegion};
    use super::counter_region::{CounterAction, CounterProps, CounterRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{
        mount, Anchor, AppHandle, GpuRegion, Layer, LocalRect, MountConfig, RegionState,
    };
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;
    use std::sync::Arc;

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
            RegionState::Lost => "lost",
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

    fn anchors_json(anchors: &[GridAnchor]) -> String {
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
        let anchors = RwSignal::new(Vec::<GridAnchor>::new());
        let hits = RwSignal::new(0u32);
        let last_hit = RwSignal::new(None::<(usize, f64, f64)>);
        let state = RwSignal::new(RegionState::Starting);

        // The menu opens on the next anchor picked, so the ordinary hit tests
        // are not fighting a menu that covers the next anchor.
        let armed = RwSignal::new(false);
        let menu = RwSignal::new(None::<usize>);
        let dialog = RwSignal::new(false);
        // The main path: an anchor is picked on the region, its colour is
        // edited in a modal, and the edit only reaches the application when it
        // is confirmed.
        let selected = RwSignal::new(None::<usize>);
        let colors = RwSignal::new(vec![DEFAULT_COLOR; COLUMNS * ROWS]);
        let draft = RwSignal::new(String::new());
        let refusal = RwSignal::new(None::<String>);
        let applied = RwSignal::new(0u32);
        // A layer anchored to an ordinary element, and the element it is
        // anchored to: removing the element has to end the layer.
        let host_present = RwSignal::new(true);
        let host_layer = RwSignal::new(false);
        let host = NodeRef::<leptos::html::Button>::new();
        let canvas = NodeRef::<leptos::html::Canvas>::new();
        // The application's own Escape command. It must not also run when a
        // layer is open: the layer owns the key while it is on the stack.
        let commands = RwSignal::new(0u32);

        Effect::new(move || {
            let report = format!(
                "{{\"anchors\":{},\"hits\":{},\"last_hit\":{},\"state\":\"{}\",\"menu\":{},\"dialog\":{},\"anchored\":{},\"commands\":{},\"selected\":{},\"colors\":\"{}\",\"applied\":{},\"refusal\":{}}}",
                anchors.with(|anchors| anchors_json(anchors)),
                hits.get(),
                last_hit
                    .get()
                    .map(|(anchor, x, y)| format!(
                        "{{\"anchor\":{anchor},\"x\":{x:.3},\"y\":{y:.3}}}"
                    ))
                    .unwrap_or_else(|| "null".to_string()),
                state_name(state.get()),
                menu.get()
                    .map(|index| index.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                dialog.get(),
                host_layer.get() && host_present.get(),
                commands.get(),
                selected
                    .get()
                    .map(|index| index.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                colors.with(|colors| {
                    colors
                        .iter()
                        .take(4)
                        .map(|color| format!("{color:06x}"))
                        .collect::<Vec<_>>()
                        .join(",")
                }),
                applied.get(),
                refusal
                    .get()
                    .map(|reason| format!("\"{reason}\""))
                    .unwrap_or_else(|| "null".to_string()),
            );
            GEOMETRY.with(|slot| *slot.borrow_mut() = report);
        });

        let on_action = move |action| match action {
            AnchorAction::Layout(reported) => anchors.set(reported),
            AnchorAction::Hit(hit) => {
                hits.update(|n| *n += 1);
                last_hit.set(Some((hit.anchor, hit.x, hit.y)));
                selected.set(Some(hit.anchor));
                if armed.get() {
                    armed.set(false);
                    menu.set(Some(hit.anchor));
                }
            }
        };

        // The menu belongs to a rectangle inside the region, not to the page:
        // it follows that rectangle when anything between them scrolls, and
        // closes when the region itself goes.
        let menu_anchor = Signal::derive(move || {
            let (Some(canvas), Some(index)) = (canvas.get(), menu.get()) else {
                return Anchor::Centred;
            };
            match anchors.with(|anchors| anchors.get(index).copied()) {
                Some(anchor) => Anchor::region(
                    &canvas.into(),
                    LocalRect::new(anchor.x, anchor.y, anchor.width, anchor.height),
                ),
                None => Anchor::Centred,
            }
        });

        // The application's rule for a colour. The control cannot know it, so
        // the control does not enforce it: it asks, and this answers.
        let apply_colour = move || {
            let Some(index) = selected.get() else {
                refusal.set(Some("nothing is selected".to_string()));
                return;
            };
            let text = draft.get();
            let text = text.trim().trim_start_matches('#');
            match u32::from_str_radix(text, 16) {
                Ok(color) if text.len() == 6 => {
                    refusal.set(None);
                    colors.update(|colors| colors[index] = color);
                    applied.update(|n| *n += 1);
                    dialog.set(false);
                }
                _ => refusal.set(Some("a colour is six hexadecimal digits".to_string())),
            }
        };

        let app = PhantomData::<AnchorRegion>;
        let props = Signal::derive(move || AnchorProps {
            colors: Arc::new(colors.get()),
            selected: selected.get(),
        });
        view! {
            <div
                class="geometry-controls"
                on:keydown=move |event| {
                    if event.key() == "Escape" {
                        commands.update(|n| *n += 1);
                    }
                }
            >
                <button type="button" data-testid="arm-menu" on:click=move |_| armed.set(true)>
                    "menu on next pick"
                </button>
                <Show when=move || host_present.get() fallback=|| ()>
                    <button
                        type="button"
                        node_ref=host
                        data-testid="anchor-host"
                        on:click=move |_| host_layer.set(true)
                    >
                        "anchored layer"
                    </button>
                </Show>
                <button
                    type="button"
                    data-testid="drop-anchor-host"
                    on:click=move |_| host_present.set(false)
                >
                    "remove the anchor"
                </button>
                {(1..=20)
                    .map(|n| {
                        let id = format!("focus-{n:02}");
                        if n == 12 {
                            view! {
                                <input type="text" data-testid=id readonly value="read only" />
                            }
                                .into_any()
                        } else if n == 7 {
                            view! { <button type="button" data-testid=id disabled>{n}</button> }
                                .into_any()
                        } else {
                            view! { <button type="button" data-testid=id>{n}</button> }.into_any()
                        }
                    })
                    .collect_view()}
            </div>
            <div class="geometry-outer" data-testid="geometry-outer">
                <div class="geometry-tall">
                    <div class="geometry-inner" data-testid="geometry-inner">
                        <div class="geometry-wide">
                            <GpuRegion
                                app=app
                                props=props
                                on_action=on_action
                                state=state
                                node_ref=canvas
                                class="anchor-region"
                                test_id="geometry-gpu"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <Show when=move || menu.get().is_some() fallback=|| ()>
                <Layer
                    anchor=menu_anchor
                    on_close=move || menu.set(None)
                    class="geometry-menu"
                    test_id="geometry-menu"
                >
                    <ul role="menu">
                        <li>
                            <button type="button" role="menuitem" data-testid="menu-rename">
                                "rename"
                            </button>
                        </li>
                        <li>
                            <button
                                type="button"
                                role="menuitem"
                                data-testid="menu-open-dialog"
                                on:click=move |_| {
                                    // The draft starts at the value in force,
                                    // so cancelling and confirming without
                                    // typing both leave it where it was.
                                    let current = selected
                                        .get()
                                        .and_then(|index| colors.with(|colors| colors.get(index).copied()))
                                        .unwrap_or(DEFAULT_COLOR);
                                    draft.set(format!("{current:06x}"));
                                    refusal.set(None);
                                    dialog.set(true);
                                }
                            >
                                "open dialog"
                            </button>
                        </li>
                    </ul>
                </Layer>
            </Show>
            <Show when=move || host_layer.get() && host_present.get() fallback=|| ()>
                <Layer
                    anchor=Signal::derive(move || match host.get() {
                        Some(element) => Anchor::element(&element.into()),
                        None => Anchor::Centred,
                    })
                    on_close=move || host_layer.set(false)
                    class="geometry-menu"
                    test_id="anchor-layer"
                >
                    <p>"anchored to a button"</p>
                    <button type="button" data-testid="anchor-layer-button">"inside"</button>
                </Layer>
            </Show>
            <Show when=move || dialog.get() fallback=|| ()>
                <Layer
                    modal=true
                    anchor=Signal::derive(|| Anchor::Centred)
                    on_close=move || dialog.set(false)
                    class="geometry-dialog"
                    test_id="geometry-dialog"
                >
                    <h3>"a modal dialog"</h3>
                    <label>
                        "colour"
                        <input
                            type="text"
                            data-testid="dialog-input"
                            prop:value=move || draft.get()
                            on:input:target=move |ev| draft.set(ev.target().value())
                        />
                    </label>
                    {move || refusal.get().map(|reason| view! {
                        <p class="region-error" role="alert" data-testid="dialog-error">
                            {reason}
                        </p>
                    })}
                    <button type="button" data-testid="dialog-apply" on:click=move |_| apply_colour()>
                        "apply"
                    </button>
                    <button type="button" data-testid="dialog-close" on:click=move |_| dialog.set(false)>
                        "close"
                    </button>
                </Layer>
            </Show>
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
        mount_scope_with(container_id, false, view)
    }

    /// The same, saying whether this scope owns the page's URL. Only one on a
    /// page may, and this example mounts several - which is exactly the case
    /// worth having a fixture for.
    fn mount_scope_with<F, N>(container_id: &str, url_owner: bool, view: F) -> Result<u32, JsValue>
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
            url_owner,
            base: String::new(),
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

    /// Names this runtime and the build it came from, and starts watching for
    /// assets that do not arrive.
    #[wasm_bindgen]
    pub fn fusion_basic_identify(runtime: u32, build: &str) {
        rustify_ui::identify_runtime(runtime, build);
    }

    /// This runtime's bounded diagnostic record.
    #[wasm_bindgen]
    pub fn fusion_basic_diagnostics() -> String {
        rustify_ui::report_json()
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
