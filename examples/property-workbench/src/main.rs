#[cfg(target_arch = "wasm32")]
mod details;
#[cfg(target_arch = "wasm32")]
mod name_field;
#[cfg(target_arch = "wasm32")]
mod object_grid;
#[cfg(target_arch = "wasm32")]
mod object_region;
#[cfg(target_arch = "wasm32")]
mod third_party;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::details::{
        rules as property_rules, Details, DetailsFields, FIELDS as PROPERTY_FIELDS,
    };
    use super::object_grid::GridCell;
    use super::object_region::{EditField, ObjectRegion, SelectionAction, SelectionProps};
    use super::third_party::ThirdPartySlider;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_components::{Form, FormStatus, SubmitButton};
    use rustify_components::{Support, CATALOG};
    use rustify_ui::{
        mount, Anchor, AppHandle, Button, Checkbox, GpuRegion, Load, LoadView, LocalRect,
        MountConfig, RegionState, Requests, Slider, TextArea, TextEdit, TextField, Theme,
        ThemeOverride, ThemePatch, ThemedScope, UiError,
    };
    use std::cell::{Cell, RefCell};
    use std::collections::{BTreeMap, BTreeSet};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;

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
        /// A locked object refuses a new name. It is what makes read-only a
        /// business rule here rather than a control attribute.
        pub locked: bool,
        /// 0..=100 in steps of 5.
        pub size: f64,
        /// The fifteen a save applies. Together with the five above they are
        /// the twenty visible properties of an object.
        pub details: crate::details::Details,
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
                locked: false,
                size: 50.0,
                details: crate::details::Details::seeded(n),
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
        static INJECT_DUPLICATE: Seam<dyn Fn()> = const { RefCell::new(BTreeMap::new()) };
        /// Armed by the page for exactly one action, then spent.
        static CLOSE_ON_ACTION: Cell<bool> = const { Cell::new(false) };
        /// Ids the application has deleted, so a lookup can tell a name that
        /// never existed from one that is gone.
        static DELETED: RefCell<BTreeSet<u32>> = const { RefCell::new(BTreeSet::new()) };
        /// Registered by the live scope: selects an object by id and reports
        /// whether it could.
        static SELECT_BY_ID: Seam<dyn Fn(u32) -> bool> = const { RefCell::new(BTreeMap::new()) };
        /// The validations the form is waiting on, oldest first, so a test can
        /// answer them in any order it likes.
        static FORM_CHECKS: Seam<dyn Fn() -> String> = const { RefCell::new(BTreeMap::new()) };
        static RESOLVE_CHECK: Seam<dyn Fn(usize, bool) -> bool> = const { RefCell::new(BTreeMap::new()) };
        /// The save the form started, held open until the page says how it
        /// went - which is what makes twenty clicks measurable.
        static RESOLVE_SAVE: Seam<dyn Fn(bool) -> bool> = const { RefCell::new(BTreeMap::new()) };
        /// Reads whether an id is in the application's current state, without
        /// selecting it.
        static EXISTS: Seam<dyn Fn(u32) -> bool> = const { RefCell::new(BTreeMap::new()) };
        /// Starts one asynchronous load, so the page can order two of them.
        static START_LOAD: Seam<dyn Fn(i32, String)> = const { RefCell::new(BTreeMap::new()) };
        /// Registered by the live scope: puts the third-party component into
        /// the view or takes it out, so a rebuild can be driven from the page.
        static PRESENT: Seam<dyn Fn(bool)> = const { RefCell::new(BTreeMap::new()) };
    }

    /// Drops every scope this page mounted. Called from inside an action
    /// callback by the test seam below, which is where it is worth proving:
    /// the region's own pump is still running at that point.
    fn close_scopes() {
        let live = HANDLES.with(|handles| std::mem::take(&mut *handles.borrow_mut()));
        drop(live);
    }

    /// What a support level is called in the catalogue table. Never empty:
    /// a blank cell would be the one thing R18 asks the catalogue not to have.
    fn support(support: Support) -> &'static str {
        support.name()
    }

    /// One page-level seam, held by every scope that is currently mounted.
    ///
    /// A single slot cannot serve two scopes: whichever mounted last would
    /// take it, and its teardown would leave the page with nothing even though
    /// the first scope is still there. Each scope registers under its own
    /// identity and withdraws only that one.
    type Seam<T> = RefCell<BTreeMap<u32, Rc<T>>>;

    fn publish<T: ?Sized>(
        slot: &'static std::thread::LocalKey<Seam<T>>,
        registration: u32,
        value: Rc<T>,
    ) {
        slot.with(|slot| slot.borrow_mut().insert(registration, value));
    }

    fn withdraw<T: ?Sized>(slot: &'static std::thread::LocalKey<Seam<T>>, registration: u32) {
        slot.with(|slot| slot.borrow_mut().remove(&registration));
    }

    /// The newest scope still mounted, which is the one a page that just
    /// mounted means when it says "the application".
    fn newest<T: ?Sized>(slot: &'static std::thread::LocalKey<Seam<T>>) -> Option<Rc<T>> {
        slot.with(|slot| slot.borrow().values().next_back().cloned())
    }

    /// One reported rectangle, in the region's own local CSS pixels.
    fn rect_json(rect: LocalRect) -> String {
        format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            rect.x, rect.y, rect.width, rect.height
        )
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
        // This scope's identity among the page-level registrations. Taken
        // once, so every seam this scope registers can be withdrawn by it and
        // by nothing else.
        let registration = NEXT_HANDLE.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });
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

        // The fifteen drafted properties of the object showing, and the
        // bookkeeping over all twenty. The draft is the application's copy to
        // hold, which is why `FormState` does not keep one: two answers to
        // "what is in this field" is the bug a controlled component exists to
        // prevent, and a form is not exempt from it.
        let draft = RwSignal::new(Details::default());
        let checks = RwSignal::new(Vec::<(&'static str, rustify_ui::Generation)>::new());
        let saving = RwSignal::new(false);
        let saves = RwSignal::new(0u32);
        let form = Form::new(&PROPERTY_FIELDS, move || {
            // Starting a save, not finishing one: the page says how it went,
            // and until it does the form is busy - which is what makes twenty
            // clicks one save rather than twenty.
            saves.update(|count| *count += 1);
            saving.set(true);
        });

        // Selecting another object replaces the draft. Anything unsaved goes
        // with it, which is this application's answer rather than the SDK's;
        // a workspace that must not lose it is P2 M5's guard.
        Effect::new(move || {
            let held = current
                .get()
                .map(|object| object.details)
                .unwrap_or_default();
            draft.set(held);
            checks.set(Vec::new());
        });

        let asked_result = RwSignal::new(String::new());
        let changed = Callback::new(move |field: &'static str| {
            form.changed(field);
            // One property is checked against something only a server knows.
            // It is started here rather than inside the form because what a
            // check *is* belongs to the application.
            if field == "reference" {
                let generation = form.validating(field);
                checks.update(|queue| queue.push((field, generation)));
            }
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
        let theme = RwSignal::new(Theme::light());
        // The rectangle the region drew the name into, while a native control
        // is editing it. The application decides when the session exists.
        let editing = RwSignal::new(None::<(EditField, LocalRect)>);
        // How many edit sessions ended because the value moved underneath them.
        let invalidated = RwSignal::new(0u32);
        let canvas = NodeRef::<leptos::html::Canvas>::new();
        // How many times the application has asked the region to open a link.
        // A region is not the page, so the embedded contract refuses it; what
        // is being exercised is that the refusal is reported rather than
        // silently swallowed.
        let open_link_requests = RwSignal::new(0u32);

        let props = Signal::derive(move || {
            let (index, total) = position.get();
            let cells = cells.get();
            let selected = selected.get().map(|id| id.0);
            let hovered = hovered.get().map(|id| id.0);
            let theme = theme.get();
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
                    locked: object.locked,
                    size: object.size,
                    open_link_requests: open_link_requests.get(),
                    theme,
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
                    locked: false,
                    size: 0.0,
                    open_link_requests: open_link_requests.get(),
                    theme,
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

        // Why a value the user asked for was not taken, or `None` when the
        // last one was. The control shows the application's value either way;
        // this says out loud why it is not what was typed.
        let refusal = RwSignal::new(None::<String>);
        let refusals = RwSignal::new(0u32);

        /// The application's own rule for a name. It is deliberately not the
        /// control's: a control cannot know that two objects may not share a
        /// name, or that this one is locked.
        fn check_name(objects: &[WorkbenchObject], id: ObjectId, name: &str) -> Result<(), String> {
            let Some(object) = objects.iter().find(|o| o.id == id) else {
                return Err("nothing is selected".to_string());
            };
            if object.locked {
                return Err(format!("object {} is locked", id.0));
            }
            if name.trim().is_empty() {
                return Err("a name cannot be empty".to_string());
            }
            if name.chars().count() > 40 {
                return Err("a name is at most 40 characters".to_string());
            }
            if let Some(other) = objects
                .iter()
                .find(|o| o.id != id && o.name == name)
                .map(|o| o.id.0)
            {
                return Err(format!("object {other} already has that name"));
            }
            Ok(())
        }

        // Returns whether the value was taken, so a caller that has a control
        // to put back knows it has to.
        let rename = move |name: String| -> bool {
            let Some(id) = selected.get() else {
                refusal.set(Some("nothing is selected".to_string()));
                refusals.update(|n| *n += 1);
                return false;
            };
            if let Err(reason) = objects.with(|objects| check_name(objects, id, &name)) {
                refusal.set(Some(reason));
                refusals.update(|n| *n += 1);
                return false;
            }
            refusal.set(None);
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.name = name;
                }
            });
            true
        };
        let set_locked = move |locked: bool| {
            let Some(id) = selected.get() else { return };
            refusal.set(None);
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.locked = locked;
                }
            });
        };
        let set_size = move |size: f64| {
            let Some(id) = selected.get() else { return };
            objects.update(|objects| {
                if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                    object.size = size;
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
                        locked: false,
                        size: 50.0,
                        details: Details::seeded(next),
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
        publish(&INJECT_DUPLICATE, registration, Rc::new(inject_duplicate));
        on_cleanup(move || withdraw(&INJECT_DUPLICATE, registration));

        // The same rule a click on the grid takes, reachable without a
        // pointer: the DOM path and the GPU path end in one place.
        let select_by_id = move |id: u32| {
            let known = objects.with(|objects| objects.iter().any(|o| o.id.0 == id));
            if known {
                selected.set(Some(ObjectId(id)));
            }
            known
        };
        publish(&SELECT_BY_ID, registration, Rc::new(select_by_id));
        publish(
            &FORM_CHECKS,
            registration,
            Rc::new(move || {
                checks.with(|queue| {
                    let entries: Vec<String> = queue
                        .iter()
                        .map(|(field, _)| format!("\"{field}\""))
                        .collect();
                    format!("[{}]", entries.join(","))
                })
            }),
        );
        publish(
            &RESOLVE_CHECK,
            registration,
            Rc::new(move |index: usize, ok: bool| {
                let entry = checks
                    .try_update(|queue| (index < queue.len()).then(|| queue.remove(index)))
                    .flatten();
                let Some((field, generation)) = entry else {
                    return false;
                };
                form.validated(
                    field,
                    generation,
                    (!ok).then(|| "no such reference".to_string()),
                );
                true
            }),
        );
        publish(
            &RESOLVE_SAVE,
            registration,
            Rc::new(move |ok: bool| {
                if !saving.get_untracked() {
                    return false;
                }
                saving.set(false);
                if ok {
                    // Applying the draft is the application's business, and it
                    // happens once, here, when the save it started succeeds.
                    let held = draft.get_untracked();
                    if let Some(id) = selected.get_untracked() {
                        objects.update(|objects| {
                            if let Some(object) = objects.iter_mut().find(|o| o.id == id) {
                                object.details = held;
                            }
                        });
                    }
                }
                form.submitted(if ok {
                    Ok(())
                } else {
                    Err("the server refused the details".to_string())
                });
                true
            }),
        );
        let exists = move |id: u32| objects.with(|objects| objects.iter().any(|o| o.id.0 == id));
        publish(&EXISTS, registration, Rc::new(exists));
        on_cleanup(move || {
            withdraw(&SELECT_BY_ID, registration);
            withdraw(&FORM_CHECKS, registration);
            withdraw(&RESOLVE_CHECK, registration);
            withdraw(&RESOLVE_SAVE, registration);
            withdraw(&EXISTS, registration);
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

        // One field loaded asynchronously, in its four states. The requests
        // outlive nothing: a stale answer and an answer to a closed view both
        // change what is shown by exactly nothing.
        let details = RwSignal::new(Load::<String>::Loading);
        let requests = Requests::new();
        let start_load = {
            let requests = requests.clone();
            move |delay_ms: i32, outcome: String| {
                let ticket = requests.issue();
                details.set(Load::Loading);
                set_timeout(
                    move || {
                        let answer = match outcome.as_str() {
                            "empty" => Load::Empty,
                            "error" => Load::Error(UiError::Timeout),
                            value => Load::Ready(value.to_string()),
                        };
                        ticket.deliver(answer, |answer| details.set(answer));
                    },
                    Duration::from_millis(delay_ms.max(0) as u64),
                );
            }
        };
        // The retry the four-state view offers. It is the application's, not
        // the component's: only the application knows what was being asked.
        let retry_details = {
            let start_load = start_load.clone();
            move || start_load(60, "the details".to_string())
        };
        publish(&START_LOAD, registration, Rc::new(start_load));
        on_cleanup({
            let requests = requests.clone();
            move || {
                requests.close();
                withdraw(&START_LOAD, registration);
            }
        });

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

        // What the region says about itself, so the page can tell a region
        // that is rebuilding from one that failed.
        let region_state = RwSignal::new(RegionState::Starting);

        // Where the region drew the controls it owns, as it reported them.
        // Business state like any other: nobody keeps a second copy of the
        // region's layout.
        let controls = RwSignal::new(None::<(LocalRect, LocalRect)>);

        // Whether the third-party component is in the view, and every callback
        // it has made. A rebuild that left a subscription behind would show up
        // as two callbacks for one change.
        let third_party_present = RwSignal::new(true);
        let third_party_updates = RwSignal::new(0u32);
        publish(
            &PRESENT,
            registration,
            Rc::new(move |present| third_party_present.set(present)) as Rc<dyn Fn(bool)>,
        );
        on_cleanup(move || withdraw(&PRESENT, registration));

        // One area of the scope with part of the theme changed. It writes only
        // what it names, onto its own element, so the rest of the scope and
        // every other scope keep the values they had.
        let emphasis = RwSignal::new(false);
        let patch = Signal::derive(move || {
            if emphasis.get() {
                ThemePatch {
                    background: Some(0xfff4ed),
                    foreground: Some(0xb42318),
                    font_size: Some(12.0),
                    spacing: Some(2.0),
                    ..ThemePatch::default()
                }
            } else {
                ThemePatch::default()
            }
        });

        Effect::new(move || {
            let (index, total) = position.get();
            let current = current.get();
            let snapshot = format!(
                "{{\"count\":{},\"position\":{},\"selected\":{},\"name\":{},\"color\":\"{}\",\"first_colors\":\"{}\",\"first_ids\":\"{}\",\"accepted\":{},\"refused\":{},\"hovered\":{},\"hovers\":{},\"editing\":{},\"invalidated\":{},\"notes\":{},\"theme\":\"{}\",\"details\":\"{}\",\"details_value\":{},\"locked\":{},\"size\":{},\"refusals\":{},\"refusal\":{},\"third_party\":{},\"third_party_updates\":{},\"controls\":{},\"region\":\"{}\",\"form\":{}}}",
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
                theme.get().name,
                details.with(|details| details.state()),
                details.with(|details| {
                    details
                        .ready()
                        .map(|value| json_string(value))
                        .unwrap_or_else(|| "null".to_string())
                }),
                current.as_ref().map(|o| o.locked).unwrap_or(false),
                current.as_ref().map(|o| o.size as i64).unwrap_or(0),
                refusals.get(),
                refusal
                    .get()
                    .map(|reason| json_string(&reason))
                    .unwrap_or_else(|| "null".to_string()),
                third_party_present.get(),
                third_party_updates.get(),
                controls
                    .get()
                    .map(|(locked, size)| {
                        format!(
                            "{{\"locked\":{},\"size\":{}}}",
                            rect_json(locked),
                            rect_json(size)
                        )
                    })
                    .unwrap_or_else(|| "null".to_string()),
                match region_state.get() {
                    RegionState::Starting => "starting",
                    RegionState::Ready => "ready",
                    RegionState::Suspended => "suspended",
                    RegionState::Lost => "lost",
                    RegionState::Failed(_) => "failed",
                    RegionState::Disposed => "disposed",
                },
                format!(
                    "{{\"fields\":{},\"errors\":{},\"first_error\":{},\"can_submit\":{},\"submitting\":{},\"dirty\":{},\"checks\":{},\"saves\":{},\"asked\":{},\"failure\":{}}}",
                    PROPERTY_FIELDS.len(),
                    PROPERTY_FIELDS
                        .iter()
                        .filter(|field| form.error(field).is_some())
                        .count(),
                    PROPERTY_FIELDS
                        .iter()
                        .find(|field| form.error(field).is_some())
                        .map(|field| json_string(field))
                        .unwrap_or_else(|| "null".to_string()),
                    form.can_submit(),
                    form.submitting(),
                    form.dirty(),
                    checks.with(Vec::len),
                    saves.get(),
                    json_string(&asked_result.get()),
                    form.failure()
                        .map(|failure| json_string(&failure))
                        .unwrap_or_else(|| "null".to_string()),
                ),
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        let label_id = format!("third-party-label-{registration}");
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
                SelectionAction::Controls { locked, size } => {
                    controls.set(Some((locked, size)));
                }
                SelectionAction::SetLocked(locked) => {
                    accepted.update(|n| *n += 1);
                    set_locked(locked);
                }
                SelectionAction::SetSize(size) => {
                    // Counted with the hovers: a drag is a stream, and what
                    // reaches the application is what survived collapsing.
                    hovers.update(|n| *n += 1);
                    set_size(size);
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
                <ThemedScope theme=theme />
                <section class="panel" aria-label="object properties">
                    <h2>"properties"</h2>
                    <Button
                        test_id="open-link-from-region"
                        aria_label="ask the region to open a link"
                        on_click=move || open_link_requests.update(|n| *n += 1)
                    >
                        "open a link from the region"
                    </Button>
                    <Button
                        test_id="toggle-theme"
                        aria_label="switch theme"
                        on_click=move || {
                            theme
                                .update(|theme| {
                                    *theme = if theme.name == "light" {
                                        Theme::dark()
                                    } else {
                                        Theme::light()
                                    };
                                });
                        }
                    >
                        {move || format!("theme: {}", theme.get().name)}
                    </Button>
                    <p>
                        "selected: "
                        <span data-testid="selected-id">
                            {move || current.get().map(|o| o.id.0.to_string()).unwrap_or_else(|| "none".to_string())}
                        </span>
                    </p>
                    <TextField
                        label="name"
                        test_id="name-input"
                        value=Signal::derive(move || {
                            current.get().map(|o| o.name).unwrap_or_default()
                        })
                        disabled=Signal::derive(move || current.get().is_none())
                        read_only=Signal::derive(move || {
                            current.get().is_some_and(|o| o.locked)
                        })
                        on_input=move |value| {
                            rename(value);
                        }
                    />
                    // A control the user can reach and read but not change:
                    // the identity is the application's, not theirs.
                    <TextField
                        label="object id"
                        test_id="object-id"
                        read_only=true
                        value=Signal::derive(move || {
                            current
                                .get()
                                .map(|o| o.id.0.to_string())
                                .unwrap_or_else(|| "none".to_string())
                        })
                        on_input=move |_| {
                            refusal.set(Some("an object id cannot be edited".to_string()));
                            refusals.update(|n| *n += 1);
                        }
                    />
                    <TextArea
                        label="notes"
                        test_id="notes-input"
                        rows=2
                        value=Signal::derive(move || {
                            current.get().map(|o| o.notes).unwrap_or_default()
                        })
                        disabled=Signal::derive(move || current.get().is_none())
                        on_input=move |value| renote(value)
                    />
                    <Checkbox
                        label="locked"
                        test_id="locked-input"
                        checked=Signal::derive(move || current.get().is_some_and(|o| o.locked))
                        disabled=Signal::derive(move || current.get().is_none())
                        on_change=move |locked| set_locked(locked)
                    />
                    <Slider
                        label="size"
                        test_id="size-input"
                        min=0.0
                        max=100.0
                        step=5.0
                        value=Signal::derive(move || {
                            current.get().map(|o| o.size).unwrap_or(0.0)
                        })
                        disabled=Signal::derive(move || current.get().is_none())
                        on_change=move |size| set_size(size)
                    />
                    <section class="details" aria-label="object details" data-testid="object-details">
                        <h3>"details"</h3>
                        <DetailsFields
                            form=form
                            draft=draft
                            disabled=Signal::derive(move || current.get().is_none())
                            on_changed=changed
                        />
                        <div class="details-actions">
                            <SubmitButton
                                form=form
                                test_id="save-details"
                                rules=move || {
                                    let object = current.get_untracked();
                                    draft.with_untracked(|draft| {
                                        property_rules(
                                            object.as_ref().map(|o| o.name.as_str()).unwrap_or(""),
                                            object.as_ref().is_some_and(|o| o.locked),
                                            object
                                                .as_ref()
                                                .map(|o| o.details.owner.as_str())
                                                .unwrap_or(""),
                                            draft,
                                        )
                                    })
                                }
                                on_asked=Callback::new(move |asked: rustify_ui::Submit| {
                                    asked_result
                                        .set(
                                            match asked {
                                                rustify_ui::Submit::Busy => "busy",
                                                rustify_ui::Submit::Waiting => "waiting",
                                                rustify_ui::Submit::Blocked { .. } => "blocked",
                                                rustify_ui::Submit::Save => "save",
                                            }
                                                .to_string(),
                                        )
                                })
                            >
                                "save details"
                            </SubmitButton>
                            <FormStatus form=form test_id="details-status" />
                        </div>
                    </section>
                    {move || refusal.get().map(|reason| view! {
                        <p class="region-error" role="alert" data-testid="rejected-value">
                            {format!("{reason}; the field shows the value in force")}
                        </p>
                    })}
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
                    <LoadView
                        label="details"
                        test_id="details"
                        value=Signal::derive(move || details.get())
                        on_retry=move || retry_details()
                    />
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

                    // One area with part of the theme changed. Everything it
                    // does not name inherits, so the panel around it and the
                    // region beside it keep the scope's own values.
                    <ThemeOverride patch=patch class="emphasis" test_id="emphasis">
                        <Button
                            test_id="toggle-emphasis"
                            aria_label="emphasise the summary"
                            on_click=move || emphasis.update(|on| *on = !*on)
                        >
                            {move || {
                                if emphasis.get() { "plain summary" } else { "emphasise summary" }
                            }}
                        </Button>
                        <p role="status" aria-label="summary" data-testid="emphasis-summary">
                            {move || {
                                let (index, total) = position.get();
                                format!(
                                    "object {} of {total}",
                                    index.map(|i| i + 1).unwrap_or(0),
                                )
                            }}
                        </p>
                    </ThemeOverride>

                    // A component the SDK did not write, at a fixed version,
                    // with its own initialization and teardown.
                    // Two controls for one value on purpose: the SDK's slider
                    // above and this one. A real application would show one;
                    // what is being checked here is that a component the SDK
                    // did not write can be rebuilt beside a region without
                    // leaving anything behind. The group carries the name, so
                    // the component's own handle is not a nameless second
                    // slider in the tree.
                    <div
                        class="third-party-slot"
                        role="group"
                        aria-labelledby=label_id.clone()
                    >
                        // The id is per scope: two scopes on one page would
                        // otherwise put the same id on two elements, and an
                        // `aria-labelledby` resolves to whichever came first.
                        <p id=label_id.clone()>"size, by a third-party component"</p>
                        <Show when=move || third_party_present.get() fallback=|| ()>
                            <ThirdPartySlider
                                test_id="third-party-size"
                                value=Signal::derive(move || {
                                    current.get().map(|o| o.size).unwrap_or(0.0)
                                })
                                on_change=move |size| set_size(size)
                                on_update=move || third_party_updates.update(|n| *n += 1)
                            />
                        </Show>
                        <Button
                            test_id="toggle-third-party"
                            aria_label="rebuild the third-party component"
                            on_click=move || {
                                third_party_present.update(|present| *present = !*present)
                            }
                        >
                            "rebuild"
                        </Button>
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
                                // A refused commit still ends the session; the
                                // region goes back to drawing the name the
                                // application holds, and the panel says why.
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
            <section class="catalogue" aria-label="component catalogue">
                <h2>"components"</h2>
                <table data-testid="catalogue">
                    <thead>
                    <tr>
                        <th scope="col">"category"</th>
                        <th scope="col">"DOM"</th>
                        <th scope="col">"GPU"</th>
                        <th scope="col">"across regions"</th>
                        <th scope="col">"properties"</th>
                        <th scope="col">"actions"</th>
                        <th scope="col">"theme"</th>
                        <th scope="col">"input"</th>
                        <th scope="col">"accessibility"</th>
                        <th scope="col">"environment"</th>
                    </tr>
                    </thead>
                    <tbody>
                    {CATALOG
                        .iter()
                        .map(|entry| {
                            let cells = entry
                                .capabilities()
                                .into_iter()
                                .map(|(class, capability)| {
                                    view! {
                                        <td data-class=class data-support=capability
                                            .support
                                            .name()>
                                            {capability.note}
                                        </td>
                                    }
                                })
                                .collect_view();
                            view! {
                                <tr
                                    data-testid=format!("catalogue-{}", entry.category.name())
                                    data-region=entry.drawn_by_region().to_string()
                                >
                                    <th scope="row">{entry.category.name()}</th>
                                    <td data-support=entry
                                        .presentation
                                        .dom
                                        .name()>{support(entry.presentation.dom)}</td>
                                    <td data-support=entry
                                        .presentation
                                        .gpu
                                        .name()>{support(entry.presentation.gpu)}</td>
                                    <td data-support=entry
                                        .presentation
                                        .across_regions
                                        .name()>
                                        {support(entry.presentation.across_regions)}
                                    </td>
                                    {cells}
                                </tr>
                            }
                        })
                        .collect_view()}
                    </tbody>
                </table>
            </section>
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
        let Some(exists) = newest(&EXISTS) else {
            return "not_found".to_string();
        };
        if exists(id) {
            return "found".to_string();
        }
        if DELETED.with(|deleted| deleted.borrow().contains(&id)) {
            return "disposed".to_string();
        }
        "not_found".to_string()
    }

    /// Starts one asynchronous load that answers after `delay_ms` with
    /// `outcome`: `empty`, `error`, or any other text as the value.
    #[wasm_bindgen]
    pub fn workbench_start_load(delay_ms: i32, outcome: &str) -> bool {
        let start = newest(&START_LOAD);
        match start {
            Some(start) => {
                start(delay_ms, outcome.to_string());
                true
            }
            None => false,
        }
    }

    /// The validations the form is waiting on, oldest first.
    #[wasm_bindgen]
    pub fn workbench_form_checks() -> String {
        newest(&FORM_CHECKS)
            .map(|checks| checks())
            .unwrap_or_else(|| "[]".to_string())
    }

    /// Answers the validation at `index`, so a test can answer them in any
    /// order it likes - which is the whole of the out-of-order question.
    #[wasm_bindgen]
    pub fn workbench_resolve_check(index: usize, ok: bool) -> bool {
        newest(&RESOLVE_CHECK).is_some_and(|resolve| resolve(index, ok))
    }

    /// Finishes the save the form started. Until this is called the form is
    /// saving, which is what makes a run of clicks countable.
    #[wasm_bindgen]
    pub fn workbench_resolve_save(ok: bool) -> bool {
        newest(&RESOLVE_SAVE).is_some_and(|resolve| resolve(ok))
    }

    /// Names this runtime and the build it came from, so every diagnostic
    /// after it can be matched to a source tree.
    #[wasm_bindgen]
    pub fn workbench_identify(runtime: u32, build: &str) {
        rustify_ui::identify_runtime(runtime, build);
    }

    /// This runtime's bounded diagnostic record.
    #[wasm_bindgen]
    pub fn workbench_diagnostics() -> String {
        rustify_ui::report_json()
    }

    /// Turns the record on or off and answers what it was, so the cost of
    /// keeping it can be measured against the same path without it.
    #[wasm_bindgen]
    pub fn workbench_set_diagnostics(on: bool) -> bool {
        rustify_ui::set_recording(on)
    }

    /// Puts the third-party component into the view or takes it out, so a
    /// rebuild is driven from outside the application.
    #[wasm_bindgen]
    pub fn workbench_set_third_party(present: bool) -> bool {
        let set = newest(&PRESENT);
        match set {
            Some(set) => {
                set(present);
                true
            }
            None => false,
        }
    }

    /// Breaks the application's own id invariant on purpose, so the refusal of
    /// an ambiguous binding can be exercised instead of assumed.
    #[wasm_bindgen]
    pub fn workbench_inject_duplicate_id() -> bool {
        let inject = newest(&INJECT_DUPLICATE);
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
