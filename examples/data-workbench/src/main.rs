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
mod strip_region;
#[cfg(target_arch = "wasm32")]
mod strip_view;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::dataset::{Dataset, Row, COLUMNS, ROWS, ROW_BYTES};
    use super::scene_layout;
    use super::scene_region::{SceneAction, SceneProps, SceneRegion};
    use super::sort::Sort;
    use super::strip_region::{StripAction, StripProps, StripRegion};
    use super::strip_view::BUCKETS;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_components::data_table::{Cell as GridCell, Column, DataTable};
    use rustify_components::{Field, FieldBinding, Form, SubmitButton, TextField, Tree, TreeNode};
    use rustify_ui::{
        mount, navigate, provide_routes, use_location, AppHandle, Counts, GpuRegion, MountConfig,
        Routes, Selection,
    };
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeMap, BTreeSet};
    use std::marker::PhantomData;
    use std::sync::Arc;

    /// How long one slice of a job may take. Long enough that the overhead of
    /// yielding is small, short enough that a frame still fits around it.
    const SLICE_MS: f64 = 8.0;

    thread_local! {
        static DATA: RefCell<Option<Dataset>> = const { RefCell::new(None) };
        static TABLE: Cell<Option<TableState>> = const { Cell::new(None) };
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

    /// The handles the page needs on the table. Every one of them is a fact
    /// the application owns and the components are shown.
    #[derive(Clone, Copy)]
    struct TableState {
        rows: RwSignal<usize>,
        /// The sample's version, republished so that everything reading a cell
        /// is reading the current one.
        version: RwSignal<u64>,
        /// Raised by the table whenever it works out its visible range again.
        /// A frame budget reads this: an animation frame in which it did not
        /// move is a frame in which the table presented nothing.
        window_version: RwSignal<u64>,
        selection: RwSignal<Selection>,
        counts: RwSignal<Counts>,
        focus: RwSignal<GridCell>,
        goto: RwSignal<Option<usize>>,
        /// The row the details form is showing, if any.
        editing: RwSignal<Option<usize>>,
        /// The group a person chose in the tree. What it filters is the next
        /// milestone's; what it is here is a choice the application heard.
        group: RwSignal<Option<String>>,
        /// Jumps the strip asked for, so a jump that happened can be told from
        /// one that was swallowed.
        jumps: RwSignal<u32>,
        saves: RwSignal<u32>,
    }

    impl TableState {
        fn new(rows: usize) -> Self {
            Self {
                rows: RwSignal::new(rows),
                version: RwSignal::new(0),
                window_version: RwSignal::new(0),
                selection: RwSignal::new(Selection::new()),
                counts: RwSignal::new(Counts::default()),
                focus: RwSignal::new(GridCell::default()),
                goto: RwSignal::new(None),
                editing: RwSignal::new(None),
                group: RwSignal::new(None),
                jumps: RwSignal::new(0),
                saves: RwSignal::new(0),
            }
        }
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
        let table = TableState::new(with_data(|data| data.len()).unwrap_or(0));
        TABLE.with(|slot| slot.set(Some(table)));
        on_cleanup(move || {
            SCENE.with(|slot| slot.set(None));
            TABLE.with(|slot| slot.set(None));
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
                    _ => view! { <TableItem table=table /> }.into_any(),
                }}
            </div>
        }
    }

    /// The twenty columns, as the names a form knows its fields by. A form's
    /// field list has to be `'static`, and the columns are fixed.
    const FIELDS: [&str; COLUMNS] = [
        "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10", "c11", "c12", "c13",
        "c14", "c15", "c16", "c17", "c18", "c19",
    ];

    /// Rows added or removed at a time by the controls that exercise identity.
    const BATCH: usize = 10;

    /// One cell of the sample, trimmed of the padding a fixed width leaves.
    fn read_cell(row: usize, column: usize) -> String {
        with_data(|data| {
            data.cell(row, column)
                .map(|cell| String::from_utf8_lossy(cell).trim_end().to_string())
                .unwrap_or_default()
        })
        .unwrap_or_default()
    }

    fn read_id(row: usize) -> u32 {
        with_data(|data| data.id(row).unwrap_or(0)).unwrap_or(0)
    }

    /// Ten groups of ten, over positions rather than values: the tree is for
    /// getting about, and the values are what the jobs are for.
    fn groups() -> Vec<TreeNode> {
        (0..10)
            .map(|group| {
                TreeNode::new(format!("g{group}"), format!("group {}", group + 1)).with(
                    (0..10)
                        .map(|child| {
                            TreeNode::new(
                                format!("g{group}-{child}"),
                                format!("group {}.{}", group + 1, child + 1),
                            )
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn bucket_of(row: usize, rows: usize) -> usize {
        if rows == 0 {
            return 0;
        }
        (row * BUCKETS / rows).min(BUCKETS - 1)
    }

    fn first_row_of(bucket: usize, rows: usize) -> usize {
        (bucket * rows / BUCKETS).min(rows.saturating_sub(1))
    }

    /// A row to insert: the sample's own alphabet, marked so a person can see
    /// which rows are new.
    fn new_row(seed: usize) -> Row {
        let mut row = [b'A'; ROW_BYTES];
        for (index, byte) in format!("NEW{seed:05}").bytes().enumerate() {
            row[index] = byte;
        }
        row
    }

    /// The table view: the grid, the groups over it, the row being edited, and
    /// the strip that says where the selection is.
    ///
    /// The application owns all four of those facts and hands each component
    /// the part it needs. None of them holds any of it, which is why an edit
    /// shows up in all four at once without anything being told twice.
    #[component]
    fn TableItem(table: TableState) -> impl IntoView {
        let columns = RwSignal::new(
            (0..COLUMNS)
                .map(|index| Column::new(FIELDS[index], format!("column {}", index + 1)))
                .collect::<Vec<_>>(),
        );
        let expanded = RwSignal::new(BTreeSet::from(["g0".to_string()]));
        let goto_input = RwSignal::new(String::new());

        // Both readers take the version as a dependency, so a write redraws
        // the cells that changed rather than only the row being edited.
        let version = table.version;
        let cell = Arc::new(move |row: usize, column: usize| {
            version.track();
            read_cell(row, column)
        });
        let row_id = Arc::new(move |row: usize| {
            version.track();
            read_id(row)
        });

        // How much of each bucket is selected. Worked out when the selection
        // or the sample changes, not when the table scrolls.
        let strip = Memo::new(move |_| {
            table.version.track();
            let rows = table.rows.get();
            let selection = table.selection.get();
            let mut buckets = vec![0u16; BUCKETS];
            DATA.with(|slot| {
                let slot = slot.borrow();
                let Some(data) = slot.as_ref() else {
                    return;
                };
                for row in 0..rows {
                    if data.id(row).is_some_and(|id| selection.contains(id)) {
                        let bucket = bucket_of(row, rows);
                        buckets[bucket] = buckets[bucket].saturating_add(1);
                    }
                }
            });
            let peak = buckets.iter().copied().max().unwrap_or(0);
            (Arc::new(buckets), peak)
        });
        let strip_props = Signal::derive(move || {
            let rows = table.rows.get();
            let at = table.focus.get().row;
            let (buckets, peak) = strip.get();
            let bucket = bucket_of(at, rows);
            StripProps {
                buckets,
                peak,
                viewport: (bucket, bucket + 1),
            }
        });
        let on_strip = move |action: StripAction| match action {
            StripAction::Jump(bucket) => {
                let row = first_row_of(bucket, table.rows.get_untracked());
                table.goto.set(Some(row));
                table.focus.update(|at| at.row = row);
                table.jumps.update(|count| *count += 1);
            }
        };

        // Selected rows that are not in the view. Nothing filters yet, so the
        // hidden count is the identities the sample no longer has - which is
        // exactly what deleting a selected row has to make true.
        let counts = Memo::new(move |_| {
            table.version.track();
            let rows = table.rows.get();
            DATA.with(|slot| {
                let slot = slot.borrow();
                let live: BTreeSet<u32> = slot
                    .as_ref()
                    .map(|data| (0..rows).filter_map(|row| data.id(row)).collect())
                    .unwrap_or_default();
                table.selection.get().counts(|id| live.contains(&id))
            })
        });
        table.counts.set(counts.get_untracked());
        Effect::new(move || table.counts.set(counts.get()));

        let select = move |row: usize| {
            let id = read_id(row);
            if id != 0 {
                table.selection.update(|selection| {
                    selection.toggle(id);
                });
            }
        };
        let activate = move |row: usize| table.editing.set(Some(row));
        let strip_app = PhantomData::<StripRegion>;
        // The tree follows the table until a person chooses a group of their
        // own: the keyboard is somewhere in the sample, and which part of it
        // that is is exactly what the tree is for.
        let showing_group = Signal::derive(move || {
            table.group.get().or_else(|| {
                let (group, child) = Dataset::group(table.focus.get().row);
                Some(format!("g{group}-{child}"))
            })
        });

        view! {
            <section data-testid="table-view" class="view">
                <div class="bar" data-testid="table-controls">
                    <label class="field">
                        "go to row"
                        <input
                            type="number"
                            data-testid="table-goto"
                            min="1"
                            prop:value=move || goto_input.get()
                            on:input=move |ev| goto_input.set(event_target_value(&ev))
                            on:change=move |ev| {
                                let asked = event_target_value(&ev);
                                goto_input.set(asked.clone());
                                if let Ok(row) = asked.trim().parse::<usize>() {
                                    let row = row
                                        .saturating_sub(1)
                                        .min(table.rows.get_untracked().saturating_sub(1));
                                    table.goto.set(Some(row));
                                    table.focus.update(|at| at.row = row);
                                }
                            }
                        />
                    </label>
                    <button
                        type="button"
                        data-testid="table-insert"
                        on:click=move |_| insert_rows(table, table.focus.get_untracked().row, BATCH)
                    >
                        {format!("insert {BATCH}")}
                    </button>
                    <button
                        type="button"
                        data-testid="table-delete"
                        on:click=move |_| {
                            delete_rows(table, table.focus.get_untracked().row, BATCH)
                        }
                    >
                        {format!("delete {BATCH}")}
                    </button>
                    <p role="status" data-testid="table-status" aria-live="polite">
                        {move || {
                            let counts = counts.get();
                            format!(
                                "selected {} (of which {} not in view)",
                                counts.total(),
                                counts.hidden,
                            )
                        }}
                    </p>
                </div>
                <div class="table-layout">
                    <Tree
                        test_id="table-tree"
                        aria_label="groups"
                        class="groups"
                        nodes=Signal::derive(groups)
                        expanded=expanded
                        selected=showing_group
                        on_select=move |key: String| table.group.set(Some(key))
                    />
                    <div class="table-middle">
                        <DataTable
                            test_id="table"
                            aria_label="the sample"
                            class="rui:flex-1"
                            rows=table.rows
                            columns=columns
                            cell=cell
                            row_id=row_id
                            selected=table.selection
                            goto=table.goto
                            version=table.window_version
                            focus=table.focus
                            on_select=select
                            on_activate=activate
                        />
                        <GpuRegion
                            app=strip_app
                            props=strip_props
                            on_action=on_strip
                            class="strip-region"
                            test_id="table-strip"
                        />
                    </div>
                    <Details table=table columns=columns />
                </div>
            </section>
        }
    }

    /// The twenty values of one row, editable.
    ///
    /// A save writes every field whose value changed, and every write bumps the
    /// sample's version: sorting, filtering and finding all read cell values,
    /// so any write at all makes a job that is still running stale.
    #[component]
    fn Details(table: TableState, columns: RwSignal<Vec<Column>>) -> impl IntoView {
        let draft = RwSignal::new(vec![String::new(); COLUMNS]);
        Effect::new(move || {
            let Some(row) = table.editing.get() else {
                return;
            };
            table.version.track();
            draft.set((0..COLUMNS).map(|column| read_cell(row, column)).collect());
        });
        let save = move || {
            let Some(row) = table.editing.get_untracked() else {
                return;
            };
            let values = draft.get_untracked();
            let mut written = 0;
            DATA.with(|slot| {
                let mut slot = slot.borrow_mut();
                let Some(data) = slot.as_mut() else {
                    return;
                };
                for (column, value) in values.iter().enumerate() {
                    let current = data
                        .cell(row, column)
                        .map(|cell| String::from_utf8_lossy(cell).trim_end().to_string())
                        .unwrap_or_default();
                    if current != *value && data.write(row, column, value) {
                        written += 1;
                    }
                }
            });
            if written > 0 {
                bump_version(table);
            }
            table.saves.update(|count| *count += 1);
        };
        let form = Form::new(&FIELDS, save);
        view! {
            <div class="details" data-testid="table-details">
                <h2 data-testid="table-detail-row">
                    {move || match table.editing.get() {
                        Some(row) => format!("row {}", row + 1),
                        None => "no row open".to_string(),
                    }}
                </h2>
                <Show when=move || table.editing.get().is_some() fallback=|| ()>
                    <form on:submit=move |ev| ev.prevent_default()>
                        {(0..COLUMNS)
                            .map(|column| {
                                let label = columns
                                    .get_untracked()
                                    .get(column)
                                    .map(|entry| entry.label.clone())
                                    .unwrap_or_default();
                                view! {
                                    <Field
                                        form=form
                                        field=FIELDS[column]
                                        label=label
                                        control=move |binding: FieldBinding| {
                                            view! {
                                                <TextField
                                                    id=binding.id()
                                                    test_id=format!("table-detail-{column}")
                                                    value=Signal::derive(move || {
                                                        draft
                                                            .get()
                                                            .get(column)
                                                            .cloned()
                                                            .unwrap_or_default()
                                                    })
                                                    on_change=move |next: String| {
                                                        draft
                                                            .update(|values| {
                                                                if let Some(slot) = values.get_mut(column) {
                                                                    *slot = next.clone();
                                                                }
                                                            });
                                                        form.changed(FIELDS[column]);
                                                    }
                                                />
                                            }
                                                .into_any()
                                        }
                                    />
                                }
                            })
                            .collect_view()}
                        <SubmitButton form=form rules=Vec::new test_id="table-detail-submit">
                            "save"
                        </SubmitButton>
                    </form>
                </Show>
            </div>
        }
    }

    /// Publishes the sample's version, and with it every reader of a cell.
    fn bump_version(table: TableState) {
        let version = with_data(|data| data.version()).unwrap_or(0);
        table.version.set(version);
    }

    /// Inserts `count` rows after `at`. The view takes them straight away: a
    /// job would rebuild the order, and there is no job until the next
    /// milestone.
    fn insert_rows(table: TableState, at: usize, count: usize) {
        let seed = table.rows.get_untracked();
        DATA.with(|slot| {
            let mut slot = slot.borrow_mut();
            let Some(data) = slot.as_mut() else {
                return;
            };
            for index in 0..count {
                data.insert((at + index).min(data.len()), new_row(seed + index));
            }
        });
        table.rows.set(with_data(|data| data.len()).unwrap_or(0));
        bump_version(table);
    }

    /// Deletes `count` rows from `at`, and takes the identities that have gone
    /// out of the selection. A selected row that no longer exists is not
    /// selected: it is a leak.
    fn delete_rows(table: TableState, at: usize, count: usize) {
        let mut gone = Vec::new();
        DATA.with(|slot| {
            let mut slot = slot.borrow_mut();
            let Some(data) = slot.as_mut() else {
                return;
            };
            for _ in 0..count {
                if at >= data.len() {
                    break;
                }
                if let Some(id) = data.remove(at) {
                    gone.push(id);
                }
            }
        });
        table.selection.update(|selection| {
            selection.remove_deleted(&gone);
        });
        table.rows.set(with_data(|data| data.len()).unwrap_or(0));
        bump_version(table);
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
        let table = TABLE.with(|slot| slot.get());
        let (selected, hidden, editing, group, jumps, saves, window) = match table {
            Some(table) => {
                let counts = table.counts.get_untracked();
                (
                    counts.total(),
                    counts.hidden,
                    table.editing.get_untracked(),
                    table.group.get_untracked(),
                    table.jumps.get_untracked(),
                    table.saves.get_untracked(),
                    table.window_version.get_untracked(),
                )
            }
            None => (0, 0, None, None, 0, 0, 0),
        };
        format!(
            concat!(
                r#"{{"path":"{}","rows":{},"version":{},"generated_ms":{:.1},"#,
                r#""scene":{{"camera":[{},{}],"pane":[{},{}],"drawn":{},"#,
                r#""asked":{},"reported":{}}},"#,
                r#""table":{{"selected":{},"hidden":{},"editing":{},"group":{},"#,
                r#""jumps":{},"saves":{},"window_version":{}}}}}"#
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
            selected,
            hidden,
            editing
                .map(|row| row.to_string())
                .unwrap_or_else(|| "null".to_string()),
            group
                .map(|key| format!("\"{key}\""))
                .unwrap_or_else(|| "null".to_string()),
            jumps,
            saves,
            window,
        )
    }

    /// How many times the table has worked out its visible range.
    ///
    /// This is the table's content version: an animation frame in which it did
    /// not move is a frame in which the table presented nothing, and a frame
    /// budget that counted those frames would pass a table that had stopped.
    #[wasm_bindgen]
    pub fn data_workbench_window_version() -> u64 {
        TABLE
            .with(|slot| slot.get())
            .map(|table| table.window_version.get_untracked())
            .unwrap_or(0)
    }

    /// The identities currently selected, in order. The page compares these
    /// against what it selected rather than against a count.
    #[wasm_bindgen]
    pub fn data_workbench_selected() -> Vec<u32> {
        TABLE
            .with(|slot| slot.get())
            .map(|table| table.selection.get_untracked().ids().collect())
            .unwrap_or_default()
    }

    /// Opens a row in the details form, as Enter does.
    #[wasm_bindgen]
    pub fn data_workbench_open_row(row: usize) -> bool {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return false;
        };
        if row >= table.rows.get_untracked() {
            return false;
        }
        table.editing.set(Some(row));
        true
    }

    /// Selects a run of rows by position, for the checks that need a selection
    /// before they can say what happens to one.
    #[wasm_bindgen]
    pub fn data_workbench_select_rows(from: usize, count: usize) -> Vec<u32> {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return Vec::new();
        };
        let rows = table.rows.get_untracked();
        let ids: Vec<u32> = (from..(from + count).min(rows))
            .map(read_id)
            .filter(|id| *id != 0)
            .collect();
        table.selection.update(|selection| {
            selection.add(ids.iter().copied());
        });
        ids
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
