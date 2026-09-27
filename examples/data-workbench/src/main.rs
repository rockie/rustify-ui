// The sample, the scene's arithmetic and the sliced sort are the application's
// rules rather than its browser half, so they are compiled for the host too
// and tested there. Nothing calls them on the host, which is the point.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod dataset;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod scan;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod scene_layout;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod sort;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod view;

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
    use super::dataset::{Dataset, Row, CELL, COLUMNS, ROWS, ROW_BYTES};
    use super::scan::{Rule, Scan};
    use super::scene_layout;
    use super::scene_region::{SceneAction, SceneProps, SceneRegion};
    use super::sort::Sort;
    use super::strip_region::{StripAction, StripProps, StripRegion};
    use super::strip_view::BUCKETS;
    use super::view::View;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_components::data_table::{Cell as GridCell, Column, DataTable};
    use rustify_components::{Field, FieldBinding, Form, SubmitButton, TextField, Tree, TreeNode};
    use rustify_ui::job::{self, Ended, Job, Step};
    use rustify_ui::{
        mount, navigate, provide_routes, use_location, AppHandle, Counts, GpuRegion, MountConfig,
        Requests, Routes, Selection,
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
        /// The table's jobs. One handle, so starting one ends the one before
        /// it: a view has one answer, and it is the newest question's.
        static JOBS: RefCell<Option<Requests>> = const { RefCell::new(None) };
        /// Which job the status line is about. An older one ending is not news
        /// about the one running now, and four jobs started in one turn end
        /// three times before the fourth has taken a slice.
        static JOB_SEQ: Cell<u64> = const { Cell::new(0) };
        /// Where the mounted scope thinks it is. Read from an export, which
        /// is outside every reactive owner and so cannot ask the router.
        static PATH: RefCell<String> = const { RefCell::new(String::new()) };
        /// What a reset needs of the mounted scope that the handles above do
        /// not carry: its router, and the counter that rebuilds its controls.
        static MOUNTED: RefCell<Option<Mounted>> = const { RefCell::new(None) };
    }

    /// The mounted workbench, as a reset reaches it from outside.
    #[derive(Clone)]
    struct Mounted {
        /// The workbench's reactive owner. Navigating asks the router, and
        /// the router is only found from inside the scope it was provided in.
        owner: Owner,
        /// Raised by a reset. Every part of a view that keeps something of its
        /// own - a typed value, an open group, a scroll position, a draft - is
        /// built again from nothing when it moves; the regions beside them are
        /// not, because starting one again is what a reset is there to avoid.
        epoch: RwSignal<u64>,
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
        /// Which rows the table is showing, in which order. Everything the
        /// table is given is a position in here; everything the sample is
        /// asked is a position in the sample.
        view: RwSignal<Arc<View>>,
        /// The sample's version, republished so that everything reading a cell
        /// is reading the current one.
        version: RwSignal<u64>,
        /// Which column the view is sorted by, and which way.
        sort: RwSignal<Option<(usize, bool)>>,
        /// The text the filter box holds. What the view was filtered by is in
        /// the view, not here: this is what a person has typed.
        filter: RwSignal<String>,
        /// The job in flight, or the last one to end.
        job: RwSignal<JobShown>,
        /// Raised by the table whenever it works out its visible range again.
        /// A frame budget reads this: an animation frame in which it did not
        /// move is a frame in which the table presented nothing.
        window_version: RwSignal<u64>,
        selection: RwSignal<Selection>,
        counts: RwSignal<Counts>,
        focus: RwSignal<GridCell>,
        goto: RwSignal<Option<usize>>,
        /// The stable identity the details form is editing, if any.
        editing: RwSignal<Option<u32>>,
        /// The group a person chose in the tree, and what a filter job for it
        /// is made from.
        group: RwSignal<Option<String>>,
        /// Jumps the strip asked for, so a jump that happened can be told from
        /// one that was swallowed.
        jumps: RwSignal<u32>,
        saves: RwSignal<u32>,
    }

    /// What the status line says about a job: how far it has got, and how the
    /// last one ended.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct JobShown {
        running: bool,
        done: usize,
        total: usize,
        /// Slices taken. How much of a job is overhead is a question about
        /// this and the time it took together.
        slices: usize,
        /// "done", "stale" or "cancelled", once one has ended.
        ended: Option<&'static str>,
    }

    impl TableState {
        fn new(rows: usize) -> Self {
            // Placeholders: `reset` writes the values a view starts from.
            let table = Self {
                view: RwSignal::new(Arc::default()),
                version: RwSignal::new(0),
                sort: RwSignal::new(None),
                filter: RwSignal::new(String::new()),
                job: RwSignal::new(JobShown::default()),
                window_version: RwSignal::new(0),
                selection: RwSignal::new(Selection::new()),
                counts: RwSignal::new(Counts::default()),
                focus: RwSignal::new(GridCell::default()),
                goto: RwSignal::new(None),
                editing: RwSignal::new(None),
                group: RwSignal::new(None),
                jumps: RwSignal::new(0),
                saves: RwSignal::new(0),
            };
            table.reset(rows);
            table
        }

        /// Puts every fact back where a first mount starts it. A mount goes
        /// through here as well, so the two cannot come to disagree.
        fn reset(self, rows: usize) {
            self.view.set(Arc::new(View::identity(rows, 0)));
            self.version.set(0);
            self.sort.set(None);
            self.filter.set(String::new());
            self.job.set(JobShown::default());
            self.window_version.set(0);
            self.selection.set(Selection::new());
            self.counts.set(Counts::default());
            self.focus.set(GridCell::default());
            self.goto.set(None);
            self.editing.set(None);
            self.group.set(None);
            self.jumps.set(0);
            self.saves.set(0);
        }
    }

    /// The handles the page needs on the scene: what it is looking at, what
    /// is chosen in it, and what the region said it drew.
    #[derive(Clone, Copy)]
    struct SceneControls {
        camera: RwSignal<(f64, f64)>,
        highlight: RwSignal<Option<u32>>,
        frozen: RwSignal<bool>,
        /// Behind an `Arc` because both of them are projected into the region
        /// on every camera change, and a marquee can choose thousands.
        chosen: RwSignal<Arc<Selection>>,
        /// The labels a person changed. Everything not in here is the name the
        /// layout gives an object, which is arithmetic rather than storage.
        labels: RwSignal<Arc<BTreeMap<u32, String>>>,
        /// The object the details form is showing, and the one under the
        /// pointer.
        editing: RwSignal<Option<u32>>,
        hover: RwSignal<Option<u32>>,
        /// Camera, pane and object count of the region's last draw.
        drawn: RwSignal<(f64, f64, f64, f64, usize)>,
        /// Camera positions the application asked for, and viewport reports
        /// that came back. A frame gate compares the two.
        asked: RwSignal<u64>,
        reported: RwSignal<u64>,
        /// Camera positions the region sent that the scope let through. A
        /// frame gate drives the pointer and compares its own count against
        /// this one: a region that answered half the drives has not kept up,
        /// whatever its frame interval says.
        accepted: RwSignal<u64>,
        /// Boxes drawn, objects picked and objects renamed, so a check can say
        /// a gesture arrived once rather than that something changed.
        picks: RwSignal<u64>,
        marquees: RwSignal<u64>,
        renames: RwSignal<u64>,
    }

    impl SceneControls {
        fn new() -> Self {
            // Placeholders, as for the table: `reset` has the real values.
            let scene = Self {
                camera: RwSignal::new((0.0, 0.0)),
                highlight: RwSignal::new(None),
                frozen: RwSignal::new(false),
                chosen: RwSignal::new(Arc::new(Selection::new())),
                labels: RwSignal::new(Arc::new(BTreeMap::new())),
                editing: RwSignal::new(None),
                hover: RwSignal::new(None),
                drawn: RwSignal::new((0.0, 0.0, 0.0, 0.0, 0)),
                asked: RwSignal::new(0),
                reported: RwSignal::new(0),
                accepted: RwSignal::new(0),
                picks: RwSignal::new(0),
                marquees: RwSignal::new(0),
                renames: RwSignal::new(0),
            };
            scene.reset(false);
            scene
        }

        /// Puts the scene back where a first mount starts it.
        ///
        /// `drawn` is the region's own report of its last draw. A region that
        /// stays has still drawn it, and one that has to start again has not:
        /// a report kept from a region that has gone would pass for the new
        /// one having drawn before it has, and clamp the camera against it.
        fn reset(self, region_stays: bool) {
            self.camera.set((0.0, 0.0));
            self.highlight.set(None);
            self.frozen.set(false);
            self.chosen.set(Arc::new(Selection::new()));
            self.labels.set(Arc::new(BTreeMap::new()));
            self.editing.set(None);
            self.hover.set(None);
            if !region_stays {
                self.drawn.set((0.0, 0.0, 0.0, 0.0, 0));
            }
            self.asked.set(0);
            self.reported.set(0);
            self.accepted.set(0);
            self.picks.set(0);
            self.marquees.set(0);
            self.renames.set(0);
        }
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
        let scene = SceneControls::new();
        SCENE.with(|slot| slot.set(Some(scene)));
        let table = TableState::new(with_data(|data| data.len()).unwrap_or(0));
        TABLE.with(|slot| slot.set(Some(table)));
        JOBS.with(|slot| *slot.borrow_mut() = Some(Requests::new()));
        let epoch = RwSignal::new(0u64);
        MOUNTED.with(|slot| {
            *slot.borrow_mut() = Owner::current().map(|owner| Mounted { owner, epoch });
        });
        on_cleanup(move || {
            // The owner is the scope's, and a handle on it kept past the scope
            // would keep everything it owns alive with it.
            MOUNTED.with(|slot| *slot.borrow_mut() = None);
            SCENE.with(|slot| slot.set(None));
            TABLE.with(|slot| slot.set(None));
            // Closing the handle ends every job it issued: a slice that was
            // about to run does not, and nothing delivers into a view that
            // has gone.
            JOBS.with(|slot| {
                if let Some(requests) = slot.borrow_mut().take() {
                    requests.close();
                }
            });
            PATH.with(|slot| slot.borrow_mut().clear());
        });

        // A memo rather than a closure: the redirect from the root to the
        // table is a change of address that is not a change of view, and a
        // closure would answer "table" twice while tearing the first one down
        // - taking a GPU region with it - to build the second.
        let showing = Memo::new(move |_| view_of(&location.get().path));
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
                {move || match showing.get() {
                    "scene" => view! { <SceneItem scene=scene epoch=epoch /> }.into_any(),
                    _ => view! { <TableItem table=table epoch=epoch /> }.into_any(),
                }}
            </div>
        }
    }

    /// Which view an address shows. One address for each of two views, and
    /// everything else - the bare root included - is the table.
    fn view_of(path: &str) -> &'static str {
        if path == "/scene" {
            "scene"
        } else {
            "table"
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

    /// What it holds, built again from nothing each time `epoch` moves: new
    /// nodes as well as new state. A closure that returned the same view
    /// again would not do - the renderer builds a view of the same shape into
    /// the nodes it already has, and they keep whatever was left on them.
    #[component]
    fn Rebuilt(epoch: RwSignal<u64>, children: ChildrenFn) -> impl IntoView {
        view! {
            <For each=move || [epoch.get()] key=|epoch| *epoch let:_epoch>
                {children()}
            </For>
        }
    }

    /// The table view: the grid, the groups over it, the row being edited, and
    /// the strip that says where the selection is.
    ///
    /// The application owns all four of those facts and hands each component
    /// the part it needs. None of them holds any of it, which is why an edit
    /// shows up in all four at once without anything being told twice.
    ///
    /// The controls, the tree, the grid and the details form each keep
    /// something of their own, so a reset builds them again (`epoch`); the
    /// strip is a region, and stays.
    #[component]
    fn TableItem(table: TableState, epoch: RwSignal<u64>) -> impl IntoView {
        let columns = RwSignal::new(
            (0..COLUMNS)
                .map(|index| Column::new(FIELDS[index], format!("column {}", index + 1)))
                .collect::<Vec<_>>(),
        );

        // Both readers go through the view, and both take the version as a
        // dependency, so a write redraws the cells that changed rather than
        // only the row being edited.
        let version = table.version;
        let view = table.view;
        let at = move |row: usize| {
            version.track();
            view.get().row(row).map(|position| position as usize)
        };
        let cell = Arc::new(move |row: usize, column: usize| {
            at(row)
                .map(|row| read_cell(row, column))
                .unwrap_or_default()
        });
        let row_id = Arc::new(move |row: usize| at(row).map(read_id).unwrap_or(0));
        let rows = Signal::derive(move || table.view.get().len());

        // How much of each bucket is selected. Worked out when the selection
        // or the sample changes, not when the table scrolls.
        let strip = Memo::new(move |_| {
            table.version.track();
            let view = table.view.get();
            let selection = table.selection.get();
            let mut buckets = vec![0u16; BUCKETS];
            DATA.with(|slot| {
                let slot = slot.borrow();
                let Some(data) = slot.as_ref() else {
                    return;
                };
                for (row, position) in view.order().iter().enumerate() {
                    if data
                        .id(*position as usize)
                        .is_some_and(|id| selection.contains(id))
                    {
                        let bucket = bucket_of(row, view.len());
                        buckets[bucket] = buckets[bucket].saturating_add(1);
                    }
                }
            });
            let peak = buckets.iter().copied().max().unwrap_or(0);
            (Arc::new(buckets), peak)
        });
        let strip_props = Signal::derive(move || {
            let rows = rows.get();
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
                let row = first_row_of(bucket, rows.get_untracked());
                table.goto.set(Some(row));
                table.focus.update(|at| at.row = row);
                table.jumps.update(|count| *count += 1);
            }
        };

        // Selected rows that are not in the view: filtered out, or no longer
        // in the sample at all. One pass over the view rather than a set built
        // from it, because the pass is what a hundred thousand rows cost once
        // and the set is what they cost twice.
        let counts = Memo::new(move |_| {
            table.version.track();
            let view = table.view.get();
            let selection = table.selection.get();
            let mut visible = 0usize;
            DATA.with(|slot| {
                let slot = slot.borrow();
                let Some(data) = slot.as_ref() else {
                    return;
                };
                for position in view.order() {
                    if data
                        .id(*position as usize)
                        .is_some_and(|id| selection.contains(id))
                    {
                        visible += 1;
                    }
                }
            });
            Counts {
                visible,
                hidden: selection.len() - visible,
            }
        });
        table.counts.set(counts.get_untracked());
        Effect::new(move || table.counts.set(counts.get()));

        let select = move |row: usize| {
            let Some(id) = at(row).map(read_id).filter(|id| *id != 0) else {
                return;
            };
            table.selection.update(|selection| {
                selection.toggle(id);
            });
        };
        let activate = move |row: usize| {
            table
                .editing
                .set(at(row).map(read_id).filter(|id| *id != 0));
        };
        let strip_app = PhantomData::<StripRegion>;
        // The tree follows the table until a person chooses a group of their
        // own: the keyboard is somewhere in the sample, and which part of it
        // that is is exactly what the tree is for.
        let showing_group = Signal::derive(move || {
            table.group.get().or_else(|| {
                let row = at(table.focus.get().row)?;
                let (group, child) = Dataset::group(row);
                Some(format!("g{group}-{child}"))
            })
        });

        view! {
            <section data-testid="table-view" class="view">
                <Rebuilt epoch=epoch>
                    <TableControls table=table rows=rows counts=counts />
                </Rebuilt>
                <div class="table-layout">
                    <Rebuilt epoch=epoch>
                        <Tree
                            test_id="table-tree"
                            aria_label="groups"
                            class="groups"
                            nodes=Signal::derive(groups)
                            expanded=RwSignal::new(BTreeSet::from(["g0".to_string()]))
                            selected=showing_group
                            on_select=move |key: String| {
                                table.group.set(Some(key));
                                table.filter.set(String::new());
                                start(table, Ask::Filter, 1);
                            }
                        />
                    </Rebuilt>
                    <div class="table-middle">
                        <Rebuilt epoch=epoch>
                            <DataTable
                                test_id="table"
                                aria_label="the sample"
                                class="flex-1"
                                rows=rows
                                columns=columns
                                cell=cell.clone()
                                row_id=row_id.clone()
                                selected=table.selection
                                goto=table.goto
                                version=table.window_version
                                focus=table.focus
                                sorted=table.sort
                                on_sort=Arc::new(move |column: usize| {
                                    let next = match table.sort.get_untracked() {
                                        Some((at, true)) if at == column => (column, false),
                                        _ => (column, true),
                                    };
                                    start(table, Ask::Sort(next.0, next.1), 1);
                                })
                                on_select=select
                                on_activate=activate
                            />
                        </Rebuilt>
                        <Show when=move || rows.get() == 0 fallback=|| ()>
                            <p data-testid="table-empty">"nothing matches"</p>
                        </Show>
                        <GpuRegion
                            app=strip_app
                            props=strip_props
                            on_action=on_strip
                            class="strip-region"
                            test_id="table-strip"
                        />
                    </div>
                    <Rebuilt epoch=epoch>
                        <Details table=table columns=columns />
                    </Rebuilt>
                </div>
            </section>
        }
    }

    /// Going to a row, inserting and deleting, filtering, finding, and what
    /// the table says about the selection and the job in flight.
    #[component]
    fn TableControls(
        table: TableState,
        rows: Signal<usize>,
        counts: Memo<Counts>,
    ) -> impl IntoView {
        let goto_input = RwSignal::new(String::new());
        let find_input = RwSignal::new(String::new());
        view! {
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
                                    .min(rows.get_untracked().saturating_sub(1));
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
                <label class="field">
                    "filter"
                    <input
                        type="search"
                        data-testid="table-filter"
                        prop:value=move || table.filter.get()
                        on:input=move |ev| table.filter.set(event_target_value(&ev))
                        on:change=move |_| {
                            // Typing does not start a job; asking does. A
                            // job for every keystroke is a hundred
                            // thousand rows scanned for a prefix nobody
                            // meant to search for.
                            table.group.set(None);
                            start(table, Ask::Filter, 1);
                        }
                    />
                </label>
                <label class="field">
                    "find"
                    <input
                        type="search"
                        data-testid="table-find"
                        prop:value=move || find_input.get()
                        on:input=move |ev| find_input.set(event_target_value(&ev))
                        on:change=move |_| {
                            let text = find_input.get_untracked();
                            if !text.trim().is_empty() {
                                start(table, Ask::Find(text), 1);
                            }
                        }
                    />
                </label>
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
                <p role="status" data-testid="table-job" aria-live="polite">
                    {move || {
                        let job = table.job.get();
                        if job.running {
                            let percent = (job.done * 100).checked_div(job.total).unwrap_or(0);
                            format!("working, {percent}%")
                        } else {
                            match job.ended {
                                Some("cancelled") => "cancelled".to_string(),
                                Some("stale") => "the sample changed".to_string(),
                                Some(_) => "done".to_string(),
                                None => "idle".to_string(),
                            }
                        }
                    }}
                </p>
                <button
                    type="button"
                    data-testid="table-cancel"
                    prop:disabled=move || !table.job.get().running
                    on:click=move |_| cancel(table)
                >
                    "cancel"
                </button>
                <Show when=move || table.view.get().is_stale(table.version.get()) fallback=|| ()>
                    <p data-testid="table-stale">"the sample has changed since this view"</p>
                </Show>
            </div>
        }
    }

    /// Where a save finds the form it has to report back to.
    ///
    /// A form is built from its save, so the save cannot be given the form: it
    /// is put here once the form exists. Reporting back is not optional - a
    /// form that is never told its save finished goes on thinking one is
    /// running, and refuses the next one.
    type FormHandle = StoredValue<Option<Form>>;

    fn saved(handle: FormHandle) {
        if let Some(form) = handle.get_value() {
            form.submitted(Ok(()));
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
        // Inserting or deleting other rows changes positions and versions,
        // but does not change this row or discard its unsaved draft.
        let edited = Memo::new(move |_| {
            table.version.track();
            let id = table.editing.get()?;
            with_data(|data| Some((id, *data.rows().get(data.position(id)?)?))).flatten()
        });
        Effect::new(move || {
            let Some((_, row)) = edited.get() else {
                if table.editing.get_untracked().is_some() {
                    table.editing.set(None);
                }
                return;
            };
            draft.set(
                row.chunks_exact(CELL)
                    .map(|cell| String::from_utf8_lossy(cell).trim_end().to_string())
                    .collect(),
            );
        });
        let handle = FormHandle::new(None);
        let save = move || {
            let Some(id) = table.editing.get_untracked() else {
                return;
            };
            let values = draft.get_untracked();
            let mut written = 0;
            DATA.with(|slot| {
                let mut slot = slot.borrow_mut();
                let Some(data) = slot.as_mut() else {
                    return;
                };
                let Some(row) = data.position(id) else {
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
            saved(handle);
        };
        let form = Form::new(&FIELDS, save);
        handle.set_value(Some(form));
        view! {
            <div class="details" data-testid="table-details">
                <h2 data-testid="table-detail-row">
                    {move || match table.editing.get() {
                        Some(id) => format!("row {id}"),
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

    /// Inserts `count` rows into the sample at `at`, and corrects the view
    /// rather than waiting for a job: a person who inserts a row expects to
    /// see it, and the next job will put it where it belongs.
    fn insert_rows(table: TableState, at: usize, count: usize) {
        let seed = with_data(Dataset::len).unwrap_or(0);
        let mut added = 0;
        DATA.with(|slot| {
            let mut slot = slot.borrow_mut();
            let Some(data) = slot.as_mut() else {
                return;
            };
            for index in 0..count {
                if data
                    .insert((at + index).min(data.len()), new_row(seed + index))
                    .is_some()
                {
                    added += 1;
                }
            }
        });
        table
            .view
            .update(|view| Arc::make_mut(view).inserted(at, added));
        bump_version(table);
    }

    /// Deletes a run of the current view, removing those identities from
    /// the sample and the selection. Resolve the whole run before any write.
    fn delete_rows(table: TableState, at: usize, count: usize) {
        let mut removed = table.view.with_untracked(|view| {
            with_data(|data| {
                view.order()
                    .iter()
                    .skip(at)
                    .take(count)
                    .filter_map(|position| {
                        data.id(*position as usize)
                            .map(|id| (*position as usize, id))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
        });
        // Work backwards so earlier sample positions remain valid.
        removed.sort_unstable_by_key(|(position, _)| std::cmp::Reverse(*position));
        DATA.with(|slot| {
            if let Some(data) = slot.borrow_mut().as_mut() {
                for (position, _) in &removed {
                    data.remove(*position);
                }
            }
        });
        let gone: Vec<_> = removed.iter().map(|(_, id)| *id).collect();
        table.selection.update(|selection| {
            selection.remove_deleted(&gone);
        });
        table.view.update(|view| {
            let view = Arc::make_mut(view);
            for (position, _) in &removed {
                view.removed(*position, 1);
            }
        });
        bump_version(table);
    }

    /// What a job was asked for. Kept so that a job made stale by a write can
    /// be asked again, once, with the version it now has.
    #[derive(Clone, Debug)]
    enum Ask {
        /// Order the view by a column.
        Sort(usize, bool),
        /// Rebuild the view from the sample, keeping the rows the filter box
        /// and the tree agree on.
        Filter,
        /// Where in the view the first row containing this is.
        Find(String),
    }

    /// What a job produced.
    enum Answer {
        /// A new order for the table.
        Order(Vec<u32>),
        /// Where in the current view the first match was, if anywhere.
        Found(Option<usize>),
    }

    /// The rule the filter box and the tree add up to.
    fn filter_rule(table: TableState) -> Rule {
        if let Some(key) = table.group.get_untracked() {
            if let Some((group, child)) = parse_group(&key) {
                return match child {
                    Some(child) => Rule::Group(group, child),
                    None => Rule::ParentGroup(group),
                };
            }
        }
        let text = table.filter.get_untracked();
        Rule::text(text.trim())
    }

    /// `g3` or `g3-7` as a parent group and optional child.
    fn parse_group(key: &str) -> Option<(usize, Option<usize>)> {
        let key = key.strip_prefix('g')?;
        match key.split_once('-') {
            Some((group, child)) => Some((group.parse().ok()?, Some(child.parse().ok()?))),
            None => Some((key.parse().ok()?, None)),
        }
    }

    /// Starts a job, ending whatever was running. `retries` is how many times
    /// it may be asked again if a write makes it stale before it answers.
    fn start(table: TableState, ask: Ask, retries: u32) {
        let Some(requests) = JOBS.with(|slot| slot.borrow().clone()) else {
            return;
        };
        let ticket = requests.issue();
        let view = table.view.get_untracked();
        let Some(job) = build(table, ask.clone(), &view, ticket) else {
            return;
        };
        let seq = JOB_SEQ.with(|slot| {
            slot.set(slot.get() + 1);
            slot.get()
        });
        table.job.set(JobShown {
            running: true,
            done: 0,
            total: job.progress().1,
            slices: 0,
            ended: None,
        });
        job::run(job, move |end| finish(table, ask, retries, seq, end));
    }

    /// Builds the state machine one ask needs, wrapped in a job.
    fn build(
        table: TableState,
        ask: Ask,
        view: &Arc<View>,
        ticket: rustify_ui::Ticket,
    ) -> Option<Job<Answer>> {
        let version = || with_data(|data| data.version()).unwrap_or(0);
        match ask {
            Ask::Sort(column, ascending) => {
                let mut sort = Some(Sort::new(view.order().to_vec(), column, ascending));
                let mut charged = 0usize;
                let total = Sort::work(view.len());
                Some(Job::new(ticket, version, job::now, total, move |budget| {
                    // Progress is written once a slice. A job of this size
                    // takes a handful of them, so this is a handful of writes
                    // rather than the hundred a timer would make.
                    let state = sort.as_mut().expect("a finished sort is not stepped again");
                    let done = with_data(|data| {
                        let mut exhausted = || budget.exhausted();
                        state.step(data.rows(), &mut exhausted)
                    })
                    .unwrap_or(true);
                    let processed = state.processed();
                    budget.did(processed - charged);
                    charged = processed;
                    shown(table, processed, total);
                    if done {
                        let order = sort
                            .take()
                            .expect("the sort that just finished")
                            .into_order();
                        Step::Done(Answer::Order(order))
                    } else {
                        Step::More
                    }
                }))
            }
            Ask::Filter => {
                // A filter is asked of the whole sample rather than of the
                // view: a row the last filter hid is a row this one may want.
                let order: Vec<u32> = (0..with_data(Dataset::len).unwrap_or(0) as u32).collect();
                scan_job(table, ticket, Scan::filter(rule_for(&ask)?), order, false)
            }
            Ask::Find(_) => {
                // A find is asked of the view, because where it lands is a
                // place in what the person is looking at.
                scan_job(
                    table,
                    ticket,
                    Scan::find(rule_for(&ask)?),
                    view.order().to_vec(),
                    true,
                )
            }
        }
    }

    /// Stops the job in flight. Nothing is half-applied, because nothing is
    /// applied until a job is finished.
    fn cancel(table: TableState) {
        JOBS.with(|slot| {
            if let Some(requests) = slot.borrow().as_ref() {
                requests.cancel();
            }
        });
        // Said now rather than when the slice in flight notices: this is the
        // feedback a person is waiting for, and the slice is already running.
        table.job.update(|job| {
            job.running = false;
            job.ended = Some("cancelled");
        });
    }

    /// How far a job has got, for the status line to say.
    fn shown(table: TableState, done: usize, total: usize) {
        table.job.update(|job| {
            job.done = done.min(total);
            job.total = total;
            job.slices += 1;
        });
    }

    /// The rule an ask carries, for the two asks that carry one.
    fn rule_for(ask: &Ask) -> Option<Rule> {
        match ask {
            Ask::Find(text) => Some(Rule::text(text.trim())),
            Ask::Filter => TABLE.with(|slot| slot.get()).map(filter_rule),
            Ask::Sort(..) => None,
        }
    }

    fn scan_job(
        table: TableState,
        ticket: rustify_ui::Ticket,
        scan: Scan,
        order: Vec<u32>,
        find: bool,
    ) -> Option<Job<Answer>> {
        let mut scan = Some(scan);
        let mut charged = 0usize;
        let total = order.len();
        let version = || with_data(|data| data.version()).unwrap_or(0);
        Some(Job::new(ticket, version, job::now, total, move |budget| {
            let state = scan.as_mut().expect("a finished scan is not stepped again");
            let done = with_data(|data| {
                let mut exhausted = || budget.exhausted();
                state.step(data.rows(), &order, &mut exhausted)
            })
            .unwrap_or(true);
            let processed = state.processed();
            budget.did(processed - charged);
            charged = processed;
            shown(table, processed, total);
            if !done {
                return Step::More;
            }
            let state = scan.take().expect("the scan that just finished");
            Step::Done(if find {
                Answer::Found(state.found())
            } else {
                Answer::Order(state.into_hits())
            })
        }))
    }

    /// Takes a job's answer, or says why there is not one.
    fn finish(table: TableState, ask: Ask, retries: u32, seq: u64, end: Ended<Answer>) {
        let state = end.state();
        // Whether this is the job the status line is about. An older one may
        // still be ending; it is recorded, but it is not the news.
        let newest = JOB_SEQ.with(|slot| slot.get()) == seq;
        match end {
            Ended::Done(Answer::Order(order)) => {
                let version = with_data(|data| data.version()).unwrap_or(0);
                table.view.set(Arc::new(View::built(order, version)));
                table.sort.set(match ask {
                    Ask::Sort(column, ascending) => Some((column, ascending)),
                    _ => None,
                });
                table.goto.set(Some(0));
                table.focus.update(|at| at.row = 0);
            }
            Ended::Done(Answer::Found(at)) => {
                if let Some(row) = at {
                    table.goto.set(Some(row));
                    table.focus.update(|cell| cell.row = row);
                }
            }
            Ended::Stale => {
                rustify_ui::diagnostics::record(rustify_ui::diagnostics::note(
                    rustify_ui::diagnostics::ErrorKind::JobCancelled,
                    "the sample was written to while a job was running; its answer was dropped",
                ));
                if newest && retries > 0 {
                    // Once. A second write during the re-run is a person
                    // typing, and a job started for every keystroke would
                    // never finish one.
                    start(table, ask, retries - 1);
                    return;
                }
            }
            Ended::Cancelled => rustify_ui::diagnostics::record(rustify_ui::diagnostics::note(
                rustify_ui::diagnostics::ErrorKind::JobCancelled,
                "a job was cancelled or replaced before it answered",
            )),
        }
        if newest {
            table.job.update(|shown| {
                shown.running = false;
                shown.ended = Some(state);
            });
        }
    }

    /// The one field an object has that a person can change. A form's field
    /// list has to be `'static`, and there is only ever this one.
    const SCENE_FIELDS: [&str; 1] = ["label"];

    /// The most of the selection the page puts in the document.
    ///
    /// A marquee can take thousands of objects and a list of thousands of
    /// nodes is neither readable nor free to build. The count beside it is of
    /// all of them, so what is not listed is still said.
    const SHOWN_OBJECTS: usize = 50;

    /// What an object is called: what it was renamed to, or what the layout
    /// names it.
    fn object_label(labels: &BTreeMap<u32, String>, id: u32) -> String {
        labels
            .get(&id)
            .cloned()
            .unwrap_or_else(|| scene_layout::label(id as usize))
    }

    /// The scene view. As with the table, what the controls and the details
    /// form keep of their own is built again on a reset, and the region is
    /// not.
    #[component]
    fn SceneItem(scene: SceneControls, epoch: RwSignal<u64>) -> impl IntoView {
        let props = Signal::derive(move || SceneProps {
            camera: scene.camera.get(),
            highlight: scene.highlight.get(),
            chosen: scene.chosen.get(),
            labels: scene.labels.get(),
            frozen: scene.frozen.get(),
        });
        let choose = move |ids: Vec<u32>, additive: bool| {
            scene.chosen.update(|chosen| {
                let chosen = Arc::make_mut(chosen);
                if additive {
                    chosen.add(ids);
                } else {
                    chosen.set(ids);
                }
            });
        };
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
            SceneAction::Camera { x, y } => {
                // The region moved itself and said where to; the application
                // is what decides the scene has edges.
                let (_, _, width, height, _) = scene.drawn.get_untracked();
                scene.accepted.update(|count| *count += 1);
                scene.camera.set(scene_layout::clamp(x, y, width, height));
            }
            SceneAction::Marquee {
                x,
                y,
                width,
                height,
                additive,
            } => {
                let taken = scene_layout::within(x, y, width, height);
                choose(taken.iter().map(|index| *index as u32).collect(), additive);
                scene.marquees.update(|count| *count += 1);
            }
            SceneAction::Pick { id, additive } => {
                if additive {
                    scene.chosen.update(|chosen| {
                        Arc::make_mut(chosen).toggle(id);
                    });
                } else {
                    choose(vec![id], false);
                }
                scene.editing.set(Some(id));
                scene.picks.update(|count| *count += 1);
            }
            SceneAction::Hover(over) => scene.hover.set(over),
        };
        let app = PhantomData::<SceneRegion>;
        view! {
            <section data-testid="scene-view" class="view">
                <Rebuilt epoch=epoch>
                    <SceneControlsBar scene=scene />
                </Rebuilt>
                <ul class="chosen" data-testid="scene-selected" aria-label="selected">
                    {move || {
                        let labels = scene.labels.get();
                        scene
                            .chosen
                            .get()
                            .ids()
                            .take(SHOWN_OBJECTS)
                            .map(|id| {
                                view! {
                                    <li data-object=id.to_string()>{object_label(&labels, id)}</li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <div class="scene-layout">
                    <GpuRegion
                        app=app
                        props=props
                        on_action=on_action
                        class="scene-region"
                        test_id="scene-gpu"
                    />
                    <Rebuilt epoch=epoch>
                        <SceneDetails scene=scene />
                    </Rebuilt>
                </div>
            </section>
        }
    }

    /// Finding an object by name, and what is chosen and pointed at.
    #[component]
    fn SceneControlsBar(scene: SceneControls) -> impl IntoView {
        let find_input = RwSignal::new(String::new());
        let find = move || {
            let asked = find_input.get_untracked();
            let Some(id) = find_object(&scene.labels.get_untracked(), asked.trim()) else {
                return;
            };
            look_at_object(scene, id);
            scene.editing.set(Some(id));
        };
        view! {
            <div class="bar" data-testid="scene-controls">
                <label class="field">
                    "find an object"
                    <input
                        type="search"
                        data-testid="scene-find"
                        prop:value=move || find_input.get()
                        on:input=move |ev| find_input.set(event_target_value(&ev))
                        on:change=move |_| find()
                    />
                </label>
                <p role="status" data-testid="scene-selected-count" aria-live="polite">
                    {move || format!("selected {}", scene.chosen.get().len())}
                </p>
                <button
                    type="button"
                    data-testid="scene-clear"
                    on:click=move |_| {
                        scene.chosen.update(|chosen| Arc::make_mut(chosen).clear());
                    }
                >
                    "clear the selection"
                </button>
                <p data-testid="scene-hover">
                    {move || match scene.hover.get() {
                        Some(id) => object_label(&scene.labels.get(), id),
                        None => "nothing".to_string(),
                    }}
                </p>
            </div>
        }
    }

    /// The object with this label, by whatever it is called now.
    ///
    /// The names the layout gives are worked out rather than stored, so the
    /// common case is arithmetic on the digits; only a renamed object has to
    /// be looked for, and only among the ones that were renamed.
    fn find_object(labels: &BTreeMap<u32, String>, asked: &str) -> Option<u32> {
        if let Some((id, _)) = labels.iter().find(|(_, label)| *label == asked) {
            return Some(*id);
        }
        let digits = asked.strip_prefix("OBJ")?;
        let id: u32 = digits.parse().ok()?;
        // An object that was renamed no longer answers to the name the layout
        // gives it: that name now belongs to nothing.
        if (id as usize) < scene_layout::COUNT && !labels.contains_key(&id) {
            Some(id)
        } else {
            None
        }
    }

    /// Puts an object in the middle of the pane and marks it.
    fn look_at_object(scene: SceneControls, id: u32) {
        let (_, _, width, height, _) = scene.drawn.get_untracked();
        let rect = scene_layout::rect(id as usize);
        let camera = scene_layout::clamp(
            rect.x + scene_layout::WIDTH / 2.0 - width / 2.0,
            rect.y + scene_layout::HEIGHT / 2.0 - height / 2.0,
            width,
            height,
        );
        scene.asked.update(|count| *count += 1);
        scene.highlight.set(Some(id));
        scene.camera.set(camera);
    }

    /// The object being edited, and the one thing about it that can change.
    #[component]
    fn SceneDetails(scene: SceneControls) -> impl IntoView {
        let draft = RwSignal::new(String::new());
        Effect::new(move || {
            let Some(id) = scene.editing.get() else {
                return;
            };
            draft.set(object_label(&scene.labels.get(), id));
        });
        let handle = FormHandle::new(None);
        let save = move || {
            let Some(id) = scene.editing.get_untracked() else {
                return;
            };
            let asked = draft.get_untracked();
            scene.labels.update(|labels| {
                Arc::make_mut(labels).insert(id, asked);
            });
            scene.renames.update(|count| *count += 1);
            saved(handle);
        };
        let form = Form::new(&SCENE_FIELDS, save);
        handle.set_value(Some(form));
        view! {
            <div class="details" data-testid="scene-details">
                <h2 data-testid="scene-detail-object">
                    {move || match scene.editing.get() {
                        Some(id) => object_label(&scene.labels.get(), id),
                        None => "no object open".to_string(),
                    }}
                </h2>
                <Show when=move || scene.editing.get().is_some() fallback=|| ()>
                    <form on:submit=move |ev| ev.prevent_default()>
                        <Field
                            form=form
                            field=SCENE_FIELDS[0]
                            label="label"
                            control=move |binding: FieldBinding| {
                                view! {
                                    <TextField
                                        id=binding.id()
                                        test_id="scene-detail-label"
                                        value=Signal::derive(move || draft.get())
                                        on_change=move |next: String| {
                                            draft.set(next);
                                            form.changed(SCENE_FIELDS[0]);
                                        }
                                    />
                                }
                                    .into_any()
                            }
                        />
                        <SubmitButton form=form rules=Vec::new test_id="scene-detail-submit">
                            "save"
                        </SubmitButton>
                    </form>
                </Show>
            </div>
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

    /// Puts the instance back where a fresh load of `path` would leave it,
    /// without loading anything: a load starts the GPU regions again, and on
    /// a software rasteriser each one is seconds of shader compilation.
    ///
    /// What belongs to the instance goes back whether or not a scope is
    /// mounted. A mounted workbench then puts its own state back, builds its
    /// controls again, and goes to `path` through its own router. A region
    /// the route keeps is kept; a change of view drops one region and starts
    /// the other, as following the link would.
    ///
    /// Returns whether there was a workbench to put back. When there was not,
    /// the page mounts one, and a mount starts from all of this anyway.
    #[wasm_bindgen]
    pub fn data_workbench_reset(path: &str) -> bool {
        SORT.with(|slot| *slot.borrow_mut() = None);
        // Only a sample that was written to is made again: it is the one
        // part of this that costs anything, and every write bumps the version.
        if with_data(|data| data.version() != 0).unwrap_or(false) {
            // Dropped first, so that the new one takes the old one's memory
            // rather than growing the instance by another sample.
            DATA.with(|slot| *slot.borrow_mut() = None);
            ensure_data();
        }
        let Some(mounted) = MOUNTED.with(|slot| slot.borrow().clone()) else {
            return false;
        };
        // Every job in flight ends, and none of them may deliver: what it
        // would deliver is an answer about a sample and a view that have both
        // gone. Nor may one say how it ended, so the reset counts as a newer
        // question than all of them. `JOB_SEQ` goes on rather than back: it
        // only has to tell the newest job from the others, and a count that
        // started again could take one of these for the newest while it is
        // still ending.
        JOBS.with(|slot| {
            if let Some(requests) = slot.borrow_mut().replace(Requests::new()) {
                requests.close();
            }
        });
        JOB_SEQ.with(|slot| slot.set(slot.get() + 1));
        let target = path.split(['?', '#']).next().unwrap_or_default();
        let from = PATH.with(|slot| view_of(&slot.borrow()));
        let to = view_of(target);
        let rows = with_data(Dataset::len).unwrap_or(0);
        if let Some(table) = TABLE.with(|slot| slot.get()) {
            table.reset(rows);
        }
        if let Some(scene) = SCENE.with(|slot| slot.get()) {
            scene.reset(from == "scene" && to == "scene");
        }
        // A change of view builds the other view from nothing anyway.
        if from == to {
            mounted.epoch.update(|epoch| *epoch += 1);
        }
        mounted.owner.with(|| navigate(path, true));
        true
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
        let (chosen, open_object, hover, accepted, picks, marquees, renames) = match scene {
            Some(scene) => (
                scene.chosen.get_untracked().len(),
                scene.editing.get_untracked(),
                scene.hover.get_untracked(),
                scene.accepted.get_untracked(),
                scene.picks.get_untracked(),
                scene.marquees.get_untracked(),
                scene.renames.get_untracked(),
            ),
            None => (0, None, None, 0, 0, 0, 0),
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
        let (shown, stale, sorted, job) = match table {
            Some(table) => {
                let view = table.view.get_untracked();
                (
                    view.len(),
                    view.is_stale(version),
                    table.sort.get_untracked(),
                    table.job.get_untracked(),
                )
            }
            None => (0, false, None, JobShown::default()),
        };
        format!(
            concat!(
                r#"{{"path":"{}","rows":{},"version":{},"generated_ms":{:.1},"#,
                r#""scene":{{"camera":[{},{}],"pane":[{},{}],"drawn":{},"#,
                r#""asked":{},"reported":{},"accepted":{},"chosen":{},"editing":{},"#,
                r#""hover":{},"picks":{},"marquees":{},"renames":{}}},"#,
                r#""table":{{"selected":{},"hidden":{},"editing":{},"group":{},"#,
                r#""jumps":{},"saves":{},"window_version":{},"shown":{},"stale":{},"#,
                r#""sorted":{},"job":{{"running":{},"done":{},"total":{},"slices":{},"#,
                r#""ended":{}}}}}}}"#
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
            accepted,
            chosen,
            open_object
                .map(|id| id.to_string())
                .unwrap_or_else(|| "null".to_string()),
            hover
                .map(|id| id.to_string())
                .unwrap_or_else(|| "null".to_string()),
            picks,
            marquees,
            renames,
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
            shown,
            stale,
            sorted
                .map(|(column, ascending)| {
                    format!("\"{column}:{}\"", if ascending { "asc" } else { "desc" })
                })
                .unwrap_or_else(|| "null".to_string()),
            job.running,
            job.done,
            job.total,
            job.slices,
            job.ended
                .map(|state| format!("\"{state}\""))
                .unwrap_or_else(|| "null".to_string()),
        )
    }

    /// A window of the view, as sample positions: what the table is showing,
    /// in the order it is showing it, for comparing against an order worked
    /// out somewhere else.
    #[wasm_bindgen]
    pub fn data_workbench_view(from: usize, count: usize) -> Vec<u32> {
        TABLE
            .with(|slot| slot.get())
            .map(|table| {
                let view = table.view.get_untracked();
                let end = from.saturating_add(count).min(view.len());
                view.order().get(from..end).unwrap_or_default().to_vec()
            })
            .unwrap_or_default()
    }

    /// Starts a sort of a column, as clicking its heading does.
    #[wasm_bindgen]
    pub fn data_workbench_sort(column: usize, ascending: bool) -> bool {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return false;
        };
        if column >= COLUMNS {
            return false;
        }
        start(table, Ask::Sort(column, ascending), 1);
        true
    }

    /// Cancels the job in flight, as the cancel button does.
    #[wasm_bindgen]
    pub fn data_workbench_cancel_job() -> bool {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return false;
        };
        cancel(table);
        true
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

    /// Opens a row of the sample in the details form, as Enter does with the
    /// row the keyboard is on.
    #[wasm_bindgen]
    pub fn data_workbench_open_row(row: usize) -> bool {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return false;
        };
        if row >= with_data(Dataset::len).unwrap_or(0) {
            return false;
        }
        table.editing.set(with_data(|data| data.id(row)).flatten());
        true
    }

    /// Selects a run of the table's rows, for the checks that need a selection
    /// before they can say what happens to one. Positions in the view, which
    /// is what a person would be clicking.
    #[wasm_bindgen]
    pub fn data_workbench_select_rows(from: usize, count: usize) -> Vec<u32> {
        let Some(table) = TABLE.with(|slot| slot.get()) else {
            return Vec::new();
        };
        let view = table.view.get_untracked();
        let ids: Vec<u32> = (from..(from + count).min(view.len()))
            .filter_map(|row| view.row(row))
            .map(|position| read_id(position as usize))
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

    /// The objects currently chosen, in order. The page compares these against
    /// what a second implementation says the marquee took, rather than against
    /// a count.
    #[wasm_bindgen]
    pub fn data_workbench_scene_chosen() -> Vec<u32> {
        SCENE
            .with(|slot| slot.get())
            .map(|scene| scene.chosen.get_untracked().ids().collect())
            .unwrap_or_default()
    }

    /// What an object is called now: the name it was given, or the one the
    /// layout works out.
    #[wasm_bindgen]
    pub fn data_workbench_scene_label(id: u32) -> String {
        SCENE
            .with(|slot| slot.get())
            .map(|scene| object_label(&scene.labels.get_untracked(), id))
            .unwrap_or_default()
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
            rustify_ui::defer(step_sort);
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
