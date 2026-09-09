#[cfg(target_arch = "wasm32")]
mod name_field;
#[cfg(target_arch = "wasm32")]
mod object_grid;
#[cfg(target_arch = "wasm32")]
mod object_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::object_grid::GridCell;
    use super::object_region::{EditField, ObjectRegion, SelectionAction, SelectionProps};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{
        mount, Anchor, AppHandle, GpuRegion, LocalRect, MountConfig, RegionState, TextEdit,
    };
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeMap, BTreeSet};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use std::sync::Arc;

    /// Stable business identity. Ids are assigned once and never reused, so a
    /// selection survives renames, reordering and deletions of other objects.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ObjectId(u32);

    #[derive(Clone, Debug, PartialEq)]
    pub struct WorkbenchObject {
        pub id: ObjectId,
        pub name: String,
        /// Several lines of it, so the region has something a single-line
        /// control could not hold.
        pub notes: String,
        /// `0xRRGGBB`.
        pub color: u32,
    }

    const PALETTE: [u32; 6] = [0x2e90fa, 0x12b76a, 0xf79009, 0xf04438, 0x7a5af8, 0x475467];
    const OBJECT_COUNT: u32 = 1_000;

    fn initial_objects() -> Vec<WorkbenchObject> {
        (1..=OBJECT_COUNT)
            .map(|n| WorkbenchObject {
                id: ObjectId(n),
                name: format!("object-{n:04}"),
                notes: format!("note {n}\nsecond line"),
                color: PALETTE[(n as usize - 1) % PALETTE.len()],
            })
            .collect()
    }

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: RefCell<u32> = const { RefCell::new(1) };
        /// What the page reports about the live scope. Derived from the
        /// application's signals by an effect; never written by anything else.
        static SNAPSHOT: RefCell<String> = const { RefCell::new(String::new()) };
        /// Registered by the live scope so the page's test seam can reach it.
        static INJECT_DUPLICATE: RefCell<Option<Rc<dyn Fn()>>> = const { RefCell::new(None) };
        /// Armed by the page for exactly one action, then spent.
        static CLOSE_ON_ACTION: Cell<bool> = const { Cell::new(false) };
        /// Ids the application has deleted, so a lookup can tell a name that
        /// never existed from one that is gone.
        static DELETED: RefCell<BTreeSet<u32>> = const { RefCell::new(BTreeSet::new()) };
        /// Registered by the live scope: selects an object by id and reports
        /// whether it could.
        static SELECT_BY_ID: RefCell<Option<Rc<dyn Fn(u32) -> bool>>> =
            const { RefCell::new(None) };
        /// Reads whether an id is in the application's current state, without
        /// selecting it.
        static EXISTS: RefCell<Option<Rc<dyn Fn(u32) -> bool>>> = const { RefCell::new(None) };
    }

    /// Drops every scope this page mounted. Called from inside an action
    /// callback by the test seam below, which is where it is worth proving:
    /// the region's own pump is still running at that point.
    fn close_scopes() {
        let live = HANDLES.with(|handles| std::mem::take(&mut *handles.borrow_mut()));
        drop(live);
    }

    /// A JSON string literal. Names come from a text input, so quotes,
    /// backslashes and control characters have to survive the snapshot.
    fn json_string(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for c in value.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }

    #[component]
    fn Workbench() -> impl IntoView {
        let objects = RwSignal::new(initial_objects());
        let selected = RwSignal::new(Some(ObjectId(1)));

        let current = Signal::derive(move || {
            let selected = selected.get()?;
            objects.with(|objects| objects.iter().find(|o| o.id == selected).cloned())
        });
        let position = Signal::derive(move || {
            let selected = selected.get();
            objects.with(|objects| {
                let index = selected.and_then(|id| objects.iter().position(|o| o.id == id));
                (index, objects.len())
            })
        });

        // Rebuilt only when the objects change, so moving the selection does
        // not rebuild a thousand cells.
        let cells = Memo::new(move |_| {
            Arc::new(objects.with(|objects| {
                objects
                    .iter()
                    .map(|o| GridCell {
                        id: o.id.0,
                        color: o.color,
                    })
                    .collect::<Vec<_>>()
            }))
        });

        // Where the pointer is, as the region reported it. Business state like
        // any other: the region draws what the application projects back, not
        // what its own pointer did.
        let hovered = RwSignal::new(None::<ObjectId>);
        // The rectangle the region drew the name into, while a native control
        // is editing it. The application decides when the session exists.
        let editing = RwSignal::new(None::<(EditField, LocalRect)>);
        // How many edit sessions ended because the value moved underneath them.
        let invalidated = RwSignal::new(0u32);
        let canvas = NodeRef::<leptos::html::Canvas>::new();

        let props = Signal::derive(move || {
            let (index, total) = position.get();
            let cells = cells.get();
            let selected = selected.get().map(|id| id.0);
            let hovered = hovered.get().map(|id| id.0);
            let editing = editing.get();
            let editing_name = matches!(editing, Some((EditField::Name, _)));
            let editing_notes = matches!(editing, Some((EditField::Notes, _)));
            match current.get() {
                Some(object) => SelectionProps {
                    name: object.name,
                    notes: object.notes,
                    position: format!("{} / {}", index.map(|i| i + 1).unwrap_or(0), total),
                    color: object.color,
                    cells,
                    selected,
                    hovered,
                    editing_name,
                    editing_notes,
                },
                None => SelectionProps {
                    name: "no selection".to_string(),
                    notes: String::new(),
                    position: format!("0 / {total}"),
                    color: 0x101828,
                    cells,
                    selected,
                    hovered,
                    editing_name,
                    editing_notes,
                },
            }
        });

        // One place moves the selection; the DOM buttons and the GPU actions
        // both go through it, so there is a single rule for what "next" means.
        let step = move |delta: i32| {
            let (index, total) = position.get();
            if total == 0 {
                return;
            }
            let next = match index {
                Some(index) => (index as i32 + delta).rem_euclid(total as i32) as usize,
                None => 0,
            };
            let id = objects.with(|objects| objects[next].id);
            selected.set(Some(id));
        };

        let rename = move |name: String| {
            let Some(id) = selected.get() else { return };
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.name = name;
                }
            });
        };
        let renote = move |notes: String| {
            let Some(id) = selected.get() else { return };
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.notes = notes;
                }
            });
        };
        let recolor = move |color: u32| {
            let Some(id) = selected.get() else { return };
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.color = color;
                }
            });
        };
        let delete_selected = move || {
            let Some(id) = selected.get() else { return };
            objects.update(|objects| objects.retain(|o| o.id != id));
            DELETED.with(|deleted| deleted.borrow_mut().insert(id.0));
            selected.set(None);
        };

        // One controlled model update for a hundred objects, not a hundred
        // updates: the projection and the GPU see one new value.
        let recolour_batch = move || {
            objects.update(|objects| {
                for (index, object) in objects.iter_mut().take(100).enumerate() {
                    object.color = PALETTE[(index + 1) % PALETTE.len()];
                }
            });
        };
        let reverse_batch = move || {
            objects.update(|objects| {
                let end = objects.len().min(100);
                objects[..end].reverse();
            });
        };
        let add_objects = move || {
            objects.update(|objects| {
                let mut next = objects.iter().map(|o| o.id.0).max().unwrap_or(0) + 1;
                for _ in 0..10 {
                    objects.push(WorkbenchObject {
                        id: ObjectId(next),
                        name: format!("object-{next:04}"),
                        notes: format!("note {next}\nsecond line"),
                        color: PALETTE[(next as usize) % PALETTE.len()],
                    });
                    next += 1;
                }
            });
        };
        let remove_objects = move || {
            let dropped: Vec<ObjectId> = objects.with(|objects| {
                objects
                    .iter()
                    .rev()
                    .take(10)
                    .map(|object| object.id)
                    .collect()
            });
            DELETED.with(|deleted| {
                let mut deleted = deleted.borrow_mut();
                for id in &dropped {
                    deleted.insert(id.0);
                }
            });
            objects.update(|objects| objects.retain(|o| !dropped.contains(&o.id)));
            if selected.get().is_some_and(|id| dropped.contains(&id)) {
                selected.set(None);
            }
        };
        // A test seam, not a feature: it breaks the invariant the application
        // otherwise keeps, so the refusal path can be exercised for real.
        let inject_duplicate = move || {
            objects.update(|objects| {
                if let Some(first) = objects.first().map(|o| o.id) {
                    if let Some(second) = objects.get_mut(1) {
                        second.id = first;
                    }
                }
            });
        };
        INJECT_DUPLICATE.with(|slot| *slot.borrow_mut() = Some(Rc::new(inject_duplicate)));
        on_cleanup(|| INJECT_DUPLICATE.with(|slot| *slot.borrow_mut() = None));

        // The same rule a click on the grid takes, reachable without a
        // pointer: the DOM path and the GPU path end in one place.
        let select_by_id = move |id: u32| {
            let known = objects.with(|objects| objects.iter().any(|o| o.id.0 == id));
            if known {
                selected.set(Some(ObjectId(id)));
            }
            known
        };
        SELECT_BY_ID.with(|slot| *slot.borrow_mut() = Some(Rc::new(select_by_id)));
        let exists = move |id: u32| objects.with(|objects| objects.iter().any(|o| o.id.0 == id));
        EXISTS.with(|slot| *slot.borrow_mut() = Some(Rc::new(exists)));
        on_cleanup(|| {
            SELECT_BY_ID.with(|slot| *slot.borrow_mut() = None);
            EXISTS.with(|slot| *slot.borrow_mut() = None);
        });
        let find_id = RwSignal::new(String::new());
        let find_result = RwSignal::new(String::new());
        let find = move || {
            let Ok(id) = find_id.get().trim().parse::<u32>() else {
                find_result.set("enter an object number".to_string());
                return;
            };
            if select_by_id(id) {
                find_result.set(format!("selected object {id}"));
            } else if DELETED.with(|deleted| deleted.borrow().contains(&id)) {
                find_result.set(format!("object {id} has been deleted"));
            } else {
                find_result.set(format!("no object {id}"));
            }
        };

        let rejected = RwSignal::new(None::<u32>);
        // Actions the scope refused because its queue was full. They never ran
        // and changed nothing, so the panel says so instead of pretending.
        let refused = RwSignal::new(0usize);
        // Counts what the application was actually handed, so a run of actions
        // can be checked for losses and duplicates rather than only for where
        // the selection ended up - the selection wraps, a count does not.
        let accepted = RwSignal::new(0u32);
        // Counted apart from the rest: a pointer stream is collapsed on its way
        // here, so its count says how much of it survived, not how much of it
        // happened.
        let hovers = RwSignal::new(0u32);

        Effect::new(move || {
            let (index, total) = position.get();
            let current = current.get();
            let snapshot = format!(
                "{{\"count\":{},\"position\":{},\"selected\":{},\"name\":{},\"color\":\"{}\",\"first_colors\":\"{}\",\"first_ids\":\"{}\",\"accepted\":{},\"refused\":{},\"hovered\":{},\"hovers\":{},\"editing\":{},\"invalidated\":{},\"notes\":{}}}",
                total,
                index.map(|i| i as i64 + 1).unwrap_or(0),
                current
                    .as_ref()
                    .map(|o| o.id.0 as i64)
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                current
                    .as_ref()
                    .map(|o| json_string(&o.name))
                    .unwrap_or_else(|| "null".to_string()),
                current
                    .as_ref()
                    .map(|o| format!("{:06x}", o.color))
                    .unwrap_or_else(|| "none".to_string()),
                objects.with(|objects| {
                    objects
                        .iter()
                        .take(4)
                        .map(|o| format!("{:06x}", o.color))
                        .collect::<Vec<_>>()
                        .join(",")
                }),
                objects.with(|objects| {
                    objects
                        .iter()
                        .take(4)
                        .map(|o| o.id.0.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }),
                accepted.get(),
                refused.get(),
                hovered
                    .get()
                    .map(|id| id.0.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                hovers.get(),
                editing.get().is_some(),
                invalidated.get(),
                current
                    .as_ref()
                    .map(|o| json_string(&o.notes))
                    .unwrap_or_else(|| "null".to_string()),
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        let region_state = RwSignal::new(RegionState::Starting);
        let app = PhantomData::<ObjectRegion>;
        let on_action = move |action| {
            match action {
                SelectionAction::Hover(id) => {
                    hovers.update(|n| *n += 1);
                    hovered.set(id.map(ObjectId));
                }
                SelectionAction::SelectPrevious => {
                    accepted.update(|n| *n += 1);
                    step(-1);
                }
                SelectionAction::SelectNext => {
                    accepted.update(|n| *n += 1);
                    step(1);
                }
                SelectionAction::Pick(id) => {
                    accepted.update(|n| *n += 1);
                    selected.set(Some(ObjectId(id)));
                }
                SelectionAction::Edit {
                    field,
                    x,
                    y,
                    width,
                    height,
                } => {
                    accepted.update(|n| *n += 1);
                    if current.get().is_some() {
                        editing.set(Some((field, LocalRect::new(x, y, width, height))));
                    }
                }
                SelectionAction::RejectedDuplicate(id) => {
                    accepted.update(|n| *n += 1);
                    rejected.set(Some(id));
                }
            }
            if CLOSE_ON_ACTION.with(|armed| armed.replace(false)) {
                close_scopes();
            }
        };

        view! {
            <div class="workbench">
                <section class="panel" aria-label="object properties">
                    <h2>"properties"</h2>
                    <p>
                        "selected: "
                        <span data-testid="selected-id">
                            {move || current.get().map(|o| o.id.0.to_string()).unwrap_or_else(|| "none".to_string())}
                        </span>
                    </p>
                    <label>
                        "name"
                        <input
                            type="text"
                            data-testid="name-input"
                            prop:value=move || current.get().map(|o| o.name).unwrap_or_default()
                            prop:disabled=move || current.get().is_none()
                            on:input:target=move |ev| rename(ev.target().value())
                        />
                    </label>
                    <label>
                        "notes"
                        <textarea
                            data-testid="notes-input"
                            rows="2"
                            prop:value=move || current.get().map(|o| o.notes).unwrap_or_default()
                            prop:disabled=move || current.get().is_none()
                            on:input:target=move |ev| renote(ev.target().value())
                        ></textarea>
                    </label>
                    <div class="swatches">
                        {PALETTE
                            .iter()
                            .map(|&color| {
                                view! {
                                    <button
                                        type="button"
                                        data-testid=format!("swatch-{color:06x}")
                                        class=format!("swatch swatch-{color:06x}")
                                        aria-label=format!("colour {color:06x}")
                                        aria-pressed=move || (current.get().map(|o| o.color) == Some(color)).to_string()
                                        on:click=move |_| recolor(color)
                                    />
                                }
                            })
                            .collect_view()}
                    </div>
                    <div>
                        <button type="button" data-testid="select-previous" on:click=move |_| step(-1)>
                            "previous"
                        </button>
                        <button type="button" data-testid="select-next" on:click=move |_| step(1)>
                            "next"
                        </button>
                    </div>
                    <button type="button" data-testid="delete-selected" on:click=move |_| delete_selected()>
                        "delete selected"
                    </button>
                    <div>
                        <button type="button" data-testid="recolour-batch" on:click=move |_| recolour_batch()>
                            "recolour 100"
                        </button>
                        <button type="button" data-testid="reverse-batch" on:click=move |_| reverse_batch()>
                            "reverse 100"
                        </button>
                        <button type="button" data-testid="add-objects" on:click=move |_| add_objects()>
                            "add 10"
                        </button>
                        <button type="button" data-testid="remove-objects" on:click=move |_| remove_objects()>
                            "remove 10"
                        </button>
                    </div>
                    {move || (refused.get() > 0).then(|| view! {
                        <p class="region-error" role="alert" data-testid="refused-actions">
                            {move || format!("{} actions were not executed; try them again", refused.get())}
                        </p>
                    })}
                    {move || rejected.get().map(|id| view! {
                        <p class="region-error" role="alert" data-testid="rejected-binding">
                            {format!("object {id} appears twice; the grid kept the last unambiguous list")}
                        </p>
                    })}
                    <p role="status" aria-label="object count">
                        "objects: "
                        <span data-testid="object-count">{move || position.get().1}</span>
                    </p>
                    <div class="find">
                        <label>
                            "go to object"
                            <input
                                type="text"
                                inputmode="numeric"
                                data-testid="find-object-id"
                                prop:value=move || find_id.get()
                                on:input:target=move |ev| find_id.set(ev.target().value())
                            />
                        </label>
                        <button type="button" data-testid="find-object" on:click=move |_| find()>
                            "go"
                        </button>
                        <p role="status" aria-label="find result" data-testid="find-result">
                            {move || find_result.get()}
                        </p>
                    </div>
                </section>
                <div>
                    <GpuRegion
                        app=app
                        props=props
                        on_action=on_action
                        state=region_state
                        refused=refused
                        node_ref=canvas
                        class="gpu-region"
                        test_id="workbench-gpu"
                    />
                    <Show
                        when=move || matches!(editing.get(), Some((EditField::Name, _)))
                        fallback=|| ()
                    >
                        <TextEdit
                            anchor=Signal::derive(move || match (canvas.get(), editing.get()) {
                                (Some(canvas), Some((_, rect))) => {
                                    Anchor::region(&canvas.into(), rect)
                                }
                                _ => Anchor::Centred,
                            })
                            value=Signal::derive(move || {
                                current.get().map(|o| o.name).unwrap_or_default()
                            })
                            on_commit=move |value| {
                                rename(value);
                                editing.set(None);
                            }
                            on_cancel=move || editing.set(None)
                            on_invalidated=move || {
                                invalidated.update(|n| *n += 1);
                                editing.set(None);
                            }
                            test_id="gpu-name-edit"
                        />
                    </Show>
                    <Show
                        when=move || matches!(editing.get(), Some((EditField::Notes, _)))
                        fallback=|| ()
                    >
                        <TextEdit
                            multiline=true
                            anchor=Signal::derive(move || match (canvas.get(), editing.get()) {
                                (Some(canvas), Some((_, rect))) => {
                                    // A few lines need more room than the one
                                    // line the region showed.
                                    Anchor::region(
                                        &canvas.into(),
                                        LocalRect::new(rect.x, rect.y, rect.width.max(200.0), 96.0),
                                    )
                                }
                                _ => Anchor::Centred,
                            })
                            value=Signal::derive(move || {
                                current.get().map(|o| o.notes).unwrap_or_default()
                            })
                            on_commit=move |value| {
                                renote(value);
                                editing.set(None);
                            }
                            on_cancel=move || editing.set(None)
                            on_invalidated=move || {
                                invalidated.update(|n| *n += 1);
                                editing.set(None);
                            }
                            test_id="gpu-notes-edit"
                        />
                    </Show>
                    {move || match region_state.get() {
                        RegionState::Failed(error) => Some(view! {
                            <p class="region-error" role="alert" data-testid="workbench-gpu-error">
                                {error.to_string()}
                            </p>
                        }),
                        _ => None,
                    }}
                </div>
            </div>
        }
    }

    #[wasm_bindgen]
    pub fn workbench_mount(container_id: &str) -> Result<u32, JsValue> {
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .unchecked_into::<leptos::web_sys::HtmlElement>();
        let config = MountConfig {
            scope: container_id.to_string(),
        };
        let handle = mount(container, config, || view! { <Workbench /> })
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
    pub fn workbench_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
    }

    #[wasm_bindgen]
    pub fn workbench_live_regions() -> u32 {
        rustify_makepad::live_region_count() as u32
    }

    /// The current selection as the application sees it, for tests to compare
    /// against what the DOM and the GPU region show.
    #[wasm_bindgen]
    pub fn workbench_snapshot() -> String {
        SNAPSHOT.with(|slot| slot.borrow().clone())
    }

    /// Arms the scope to close itself from inside its next action callback, so
    /// a teardown that starts while the region's pump is still running can be
    /// exercised instead of assumed.
    #[wasm_bindgen]
    pub fn workbench_close_on_next_action() {
        CLOSE_ON_ACTION.with(|armed| armed.set(true));
    }

    /// What the application can say about one object identity right now:
    /// `found`, `disposed` for one it has deleted, `not_found` otherwise. The
    /// first two are final answers; only `not_found` can still change.
    #[wasm_bindgen]
    pub fn workbench_lookup_object(id: u32) -> String {
        let known = SELECT_BY_ID.with(|slot| slot.borrow().is_some());
        if !known {
            return "not_found".to_string();
        }
        if EXISTS.with(|exists| exists.borrow().as_ref().is_some_and(|exists| exists(id))) {
            return "found".to_string();
        }
        if DELETED.with(|deleted| deleted.borrow().contains(&id)) {
            return "disposed".to_string();
        }
        "not_found".to_string()
    }

    /// Breaks the application's own id invariant on purpose, so the refusal of
    /// an ambiguous binding can be exercised instead of assumed.
    #[wasm_bindgen]
    pub fn workbench_inject_duplicate_id() -> bool {
        let inject = INJECT_DUPLICATE.with(|slot| slot.borrow().clone());
        match inject {
            Some(inject) => {
                inject();
                true
            }
            None => false,
        }
    }
}

fn main() {}
