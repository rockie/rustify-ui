// The sample, the scene's arithmetic and the sliced sort are the application's
// rules rather than its browser half, so they are compiled for the host too
// and tested there. Nothing calls them on the host, which is the point.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod dataset;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod scene_layout;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod sort;

#[cfg(target_arch = "wasm32")]
mod scene_region;
#[cfg(target_arch = "wasm32")]
mod scene_view;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::dataset::{Dataset, COLUMNS, ROWS};
    use super::scene_layout;
    use super::scene_region::{SceneAction, SceneProps, SceneRegion};
    use super::sort::Sort;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{
        mount, navigate, provide_routes, use_location, AppHandle, GpuRegion, MountConfig, Routes,
    };
    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    /// How long one slice of a job may take. Long enough that the overhead of
    /// yielding is small, short enough that a frame still fits around it.
    const SLICE_MS: f64 = 8.0;

    thread_local! {
        static DATA: RefCell<Option<Dataset>> = const { RefCell::new(None) };
        /// How long generating the sample took, in milliseconds.
        static GENERATED_MS: Cell<f64> = const { Cell::new(0.0) };
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: Cell<u32> = const { Cell::new(1) };
        static SCENE: Cell<Option<SceneControls>> = const { Cell::new(None) };
        static SORT: RefCell<Option<SortProbe>> = const { RefCell::new(None) };
        /// Where the mounted scope thinks it is. Read from an export, which
        /// is outside every reactive owner and so cannot ask the router.
        static PATH: RefCell<String> = const { RefCell::new(String::new()) };
    }

    fn now() -> f64 {
        leptos::web_sys::window()
            .and_then(|window| window.performance())
            .map(|performance| performance.now())
            .unwrap_or(0.0)
    }

    /// The handles the page needs on the scene: what it is looking at, and
    /// what the region said it drew.
    #[derive(Clone, Copy)]
    struct SceneControls {
        camera: RwSignal<(f64, f64)>,
        highlight: RwSignal<Option<u32>>,
        frozen: RwSignal<bool>,
        /// Camera, pane and object count of the region's last draw.
        drawn: RwSignal<(f64, f64, f64, f64, usize)>,
        /// Camera positions the application asked for, and viewport reports
        /// that came back. A frame gate compares the two.
        asked: RwSignal<u64>,
        reported: RwSignal<u64>,
    }

    /// A sort running on its own, with nothing else to show for it than how
    /// long it took and in how many slices.
    ///
    /// It is the shape a sliced job has - start, step, yield, finish - without
    /// the table, the cancellation or the version check that the real one will
    /// carry. What it measures is whether a hundred thousand rows can be put
    /// in order inside a budget at all.
    struct SortProbe {
        sort: Option<Sort>,
        started: f64,
        slices: usize,
        elapsed: f64,
        /// The order once it is finished, so the result can be compared with
        /// one worked out somewhere else.
        order: Option<Vec<u32>>,
    }

    fn with_data<R>(f: impl FnOnce(&Dataset) -> R) -> Option<R> {
        DATA.with(|slot| slot.borrow().as_ref().map(f))
    }

    /// Generates the sample once for this instance. Later mounts share it: it
    /// is thirty-two megabytes, and the scope is not what owns it.
    fn ensure_data() {
        DATA.with(|slot| {
            if slot.borrow().is_some() {
                return;
            }
            let started = now();
            let data = Dataset::generate();
            GENERATED_MS.with(|ms| ms.set(now() - started));
            *slot.borrow_mut() = Some(data);
        });
    }

    #[component]
    fn Workbench() -> impl IntoView {
        provide_routes(Routes::new(&["/", "/table", "/scene"]));
        let location = use_location();
        // One address for two views, and the bare root is neither: arriving
        // there is arriving at the table, without an entry of its own.
        Effect::new(move || {
            let path = location.get().path;
            PATH.with(|slot| *slot.borrow_mut() = path.clone());
            if path == "/" {
                navigate("/table", true);
            }
        });
        let scene = SceneControls {
            camera: RwSignal::new((0.0, 0.0)),
            highlight: RwSignal::new(None),
            frozen: RwSignal::new(false),
            drawn: RwSignal::new((0.0, 0.0, 0.0, 0.0, 0)),
            asked: RwSignal::new(0),
            reported: RwSignal::new(0),
        };
        SCENE.with(|slot| slot.set(Some(scene)));
        on_cleanup(move || {
            SCENE.with(|slot| slot.set(None));
            PATH.with(|slot| slot.borrow_mut().clear());
        });

        let showing = move || {
            let path = location.get().path;
            if path == "/scene" {
                "scene"
            } else {
                "table"
            }
        };
        view! {
            <div class="workbench">
                <header class="bar">
                    <nav aria-label="views">
                        <rustify_ui::Link href="/table" test_id="go-table">"table"</rustify_ui::Link>
                        <rustify_ui::Link href="/scene" test_id="go-scene">"scene"</rustify_ui::Link>
                    </nav>
                    <p data-testid="dataset-status">
                        {move || format!("{ROWS} rows x {COLUMNS} columns")}
                    </p>
                </header>
                {move || match showing() {
                    "scene" => view! { <SceneItem scene=scene /> }.into_any(),
                    _ => view! { <TableItem /> }.into_any(),
                }}
            </div>
        }
    }

    /// The table view. Its contents - the windowed grid, the group tree and the
    /// details form - are the next milestone; what is here is the frame they
    /// go in and the counts a page can already check.
    #[component]
    fn TableItem() -> impl IntoView {
        let rows = with_data(|data| data.len()).unwrap_or(0);
        let version = with_data(|data| data.version()).unwrap_or(0);
        view! {
            <section data-testid="table-view" class="view">
                <h1>"table"</h1>
                <p data-testid="table-rows">{rows.to_string()}</p>
                <p data-testid="table-version">{version.to_string()}</p>
            </section>
        }
    }

    #[component]
    fn SceneItem(scene: SceneControls) -> impl IntoView {
        let props = Signal::derive(move || SceneProps {
            camera: scene.camera.get(),
            highlight: scene.highlight.get(),
            frozen: scene.frozen.get(),
        });
        let on_action = move |action: SceneAction| match action {
            SceneAction::Viewport {
                x,
                y,
                width,
                height,
                drawn,
            } => {
                scene.drawn.set((x, y, width, height, drawn));
                scene.reported.update(|count| *count += 1);
            }
        };
        let app = PhantomData::<SceneRegion>;
        view! {
            <section data-testid="scene-view" class="view">
                <h1>"scene"</h1>
                <GpuRegion
                    app=app
                    props=props
                    on_action=on_action
                    class="scene-region"
                    test_id="scene-gpu"
                />
            </section>
        }
    }

    #[wasm_bindgen]
    pub fn data_workbench_mount(container_id: &str) -> Result<u32, JsValue> {
        ensure_data();
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .unchecked_into::<leptos::web_sys::HtmlElement>();
        let config = MountConfig {
            scope: container_id.to_string(),
            url_owner: true,
            base: String::new(),
        };
        let handle = mount(container, config, || view! { <Workbench /> })
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let id = NEXT_HANDLE.with(|next| {
            let id = next.get();
            next.set(id + 1);
            id
        });
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    #[wasm_bindgen]
    pub fn data_workbench_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
    }

    #[wasm_bindgen]
    pub fn data_workbench_identify(runtime: u32, build: &str) {
        rustify_ui::identify_runtime(runtime, build);
    }

    #[wasm_bindgen]
    pub fn data_workbench_diagnostics() -> String {
        rustify_ui::report_json()
    }

    #[wasm_bindgen]
    pub fn data_workbench_live_regions() -> u32 {
        rustify_makepad::live_region_count() as u32
    }

    /// What the application says about itself right now.
    #[wasm_bindgen]
    pub fn data_workbench_snapshot() -> String {
        let rows = with_data(|data| data.len()).unwrap_or(0);
        let version = with_data(|data| data.version()).unwrap_or(0);
        let generated = GENERATED_MS.with(|ms| ms.get());
        let path = PATH.with(|slot| slot.borrow().clone());
        let scene = SCENE.with(|slot| slot.get());
        let (camera, drawn, asked, reported) = match scene {
            Some(scene) => (
                scene.camera.get_untracked(),
                scene.drawn.get_untracked(),
                scene.asked.get_untracked(),
                scene.reported.get_untracked(),
            ),
            None => ((0.0, 0.0), (0.0, 0.0, 0.0, 0.0, 0), 0, 0),
        };
        format!(
            concat!(
                r#"{{"path":"{}","rows":{},"version":{},"generated_ms":{:.1},"#,
                r#""scene":{{"camera":[{},{}],"pane":[{},{}],"drawn":{},"#,
                r#""asked":{},"reported":{}}}}}"#
            ),
            path,
            rows,
            version,
            generated,
            camera.0,
            camera.1,
            drawn.2,
            drawn.3,
            drawn.4,
            asked,
            reported,
        )
    }

    /// A checksum over every cell, as a decimal string: the one number that
    /// says whether a second implementation produced the same sample.
    #[wasm_bindgen]
    pub fn data_workbench_dataset_hash() -> String {
        with_data(|data| data.hash().to_string()).unwrap_or_default()
    }

    /// One row's identity, so the second implementation can be checked on the
    /// column that no sort or filter is allowed to change.
    #[wasm_bindgen]
    pub fn data_workbench_row_id(row: usize) -> u32 {
        with_data(|data| data.id(row).unwrap_or(0)).unwrap_or(0)
    }

    /// One cell, for comparing a sample of them against the same second
    /// implementation.
    #[wasm_bindgen]
    pub fn data_workbench_cell(row: usize, column: usize) -> String {
        with_data(|data| {
            data.cell(row, column)
                .map(|cell| String::from_utf8_lossy(cell).into_owned())
                .unwrap_or_default()
        })
        .unwrap_or_default()
    }

    /// Points the scene's camera at a position in scene coordinates, clamped
    /// to the scene. Absolute, like the stream it stands in for.
    #[wasm_bindgen]
    pub fn data_workbench_look_at(x: f64, y: f64) -> bool {
        let Some(scene) = SCENE.with(|slot| slot.get()) else {
            return false;
        };
        let (_, _, width, height, _) = scene.drawn.get_untracked();
        let camera = scene_layout::clamp(x, y, width, height);
        scene.asked.update(|count| *count += 1);
        scene.camera.set(camera);
        true
    }

    /// Stops the scene changing what it presents, for the frame gate's own
    /// negative check.
    #[wasm_bindgen]
    pub fn data_workbench_freeze_scene(on: bool) -> bool {
        let Some(scene) = SCENE.with(|slot| slot.get()) else {
            return false;
        };
        scene.frozen.set(on);
        true
    }

    /// How many objects the scene would draw from where it is looking. The
    /// same arithmetic the region uses, so a page can say whether the region
    /// is culling without reading pixels.
    #[wasm_bindgen]
    pub fn data_workbench_scene_visible() -> usize {
        let Some(scene) = SCENE.with(|slot| slot.get()) else {
            return 0;
        };
        let (x, y) = scene.camera.get_untracked();
        let (_, _, width, height, _) = scene.drawn.get_untracked();
        scene_layout::visible(x, y, width, height).count()
    }

    /// Starts a sliced sort of the whole sample on `column`.
    #[wasm_bindgen]
    pub fn data_workbench_sort_probe(column: usize, ascending: bool) -> bool {
        if column >= COLUMNS || DATA.with(|slot| slot.borrow().is_none()) {
            return false;
        }
        let order = (0..ROWS as u32).collect();
        SORT.with(|slot| {
            *slot.borrow_mut() = Some(SortProbe {
                sort: Some(Sort::new(order, column, ascending)),
                started: now(),
                slices: 0,
                elapsed: 0.0,
                order: None,
            });
        });
        step_sort();
        true
    }

    /// One slice, then a turn of the browser before the next.
    fn step_sort() {
        let finished = SORT.with(|slot| {
            let mut slot = slot.borrow_mut();
            let Some(probe) = slot.as_mut() else {
                return true;
            };
            let Some(sort) = probe.sort.as_mut() else {
                return true;
            };
            let deadline = now() + SLICE_MS;
            let mut exhausted = || now() >= deadline;
            let done = DATA
                .with(|data| {
                    data.borrow()
                        .as_ref()
                        .map(|data| sort.step(data.rows(), &mut exhausted))
                })
                .unwrap_or(true);
            probe.slices += 1;
            if done {
                probe.elapsed = now() - probe.started;
                probe.order = probe.sort.take().map(Sort::into_order);
            }
            done
        });
        if !finished {
            rustify_makepad::defer(step_sort);
        }
    }

    /// `{"running":bool,"slices":n,"ms":f,"rows":n,"first":"..","last":".."}`.
    #[wasm_bindgen]
    pub fn data_workbench_sort_probe_state() -> String {
        SORT.with(|slot| {
            let slot = slot.borrow();
            let Some(probe) = slot.as_ref() else {
                return r#"{"running":false,"slices":0,"ms":0,"rows":0}"#.to_string();
            };
            let running = probe.order.is_none();
            let rows = probe.order.as_ref().map(|order| order.len()).unwrap_or(0);
            format!(
                r#"{{"running":{},"slices":{},"ms":{:.3},"rows":{}}}"#,
                running, probe.slices, probe.elapsed, rows
            )
        })
    }

    /// A window of the finished order, as row indices. The whole of it is a
    /// hundred thousand numbers, and a comparison against a second
    /// implementation does not need them in one string.
    #[wasm_bindgen]
    pub fn data_workbench_sort_probe_order(from: usize, count: usize) -> Vec<u32> {
        SORT.with(|slot| {
            let slot = slot.borrow();
            slot.as_ref()
                .and_then(|probe| probe.order.as_ref())
                .map(|order| {
                    let end = from.saturating_add(count).min(order.len());
                    order.get(from..end).unwrap_or_default().to_vec()
                })
                .unwrap_or_default()
        })
    }
}

fn main() {}
