#[cfg(target_arch = "wasm32")]
mod object_grid;
#[cfg(target_arch = "wasm32")]
mod object_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::object_grid::GridCell;
    use super::object_region::{ObjectRegion, SelectionAction, SelectionProps};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_ui::{mount, AppHandle, GpuRegion, MountConfig, RegionState};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
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

        let props = Signal::derive(move || {
            let (index, total) = position.get();
            let cells = cells.get();
            let selected = selected.get().map(|id| id.0);
            match current.get() {
                Some(object) => SelectionProps {
                    name: object.name,
                    position: format!("{} / {}", index.map(|i| i + 1).unwrap_or(0), total),
                    color: object.color,
                    cells,
                    selected,
                },
                None => SelectionProps {
                    name: "no selection".to_string(),
                    position: format!("0 / {total}"),
                    color: 0x101828,
                    cells,
                    selected,
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

        Effect::new(move || {
            let (index, total) = position.get();
            let current = current.get();
            let snapshot = format!(
                "{{\"count\":{},\"position\":{},\"selected\":{},\"name\":{},\"color\":\"{}\",\"first_colors\":\"{}\",\"first_ids\":\"{}\"}}",
                total,
                index.map(|i| i as i64 + 1).unwrap_or(0),
                current
                    .as_ref()
                    .map(|o| o.id.0 as i64)
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                current
                    .as_ref()
                    .map(|o| format!("\"{}\"", o.name))
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
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        let region_state = RwSignal::new(RegionState::Starting);
        let app = PhantomData::<ObjectRegion>;
        let rejected = RwSignal::new(None::<u32>);
        let on_action = move |action| match action {
            SelectionAction::SelectPrevious => step(-1),
            SelectionAction::SelectNext => step(1),
            SelectionAction::Pick(id) => selected.set(Some(ObjectId(id))),
            SelectionAction::RejectedDuplicate(id) => rejected.set(Some(id)),
        };

        view! {
            <div class="workbench">
                <div class="panel">
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
                    {move || rejected.get().map(|id| view! {
                        <p class="region-error" role="alert" data-testid="rejected-binding">
                            {format!("object {id} appears twice; the grid kept the last unambiguous list")}
                        </p>
                    })}
                    <p>"objects: " <span data-testid="object-count">{move || position.get().1}</span></p>
                </div>
                <div>
                    <GpuRegion
                        app=app
                        props=props
                        on_action=on_action
                        state=region_state
                        class="gpu-region"
                        test_id="workbench-gpu"
                    />
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
