#[cfg(target_arch = "wasm32")]
mod details;
#[cfg(target_arch = "wasm32")]
mod group_list;
#[cfg(target_arch = "wasm32")]
mod name_field;
#[cfg(target_arch = "wasm32")]
mod object_grid;
#[cfg(target_arch = "wasm32")]
mod object_region;
#[cfg(target_arch = "wasm32")]
mod reset;
#[cfg(target_arch = "wasm32")]
mod third_party;
// Not gated: the file formats are arithmetic on bytes, and the host is where
// a round trip can be compared without a browser. Nothing calls them there,
// which is the point.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod transfer;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::details::{
        rules as property_rules, Details, DetailsFields, FIELDS as PROPERTY_FIELDS,
    };
    use super::group_list::Group;
    use super::object_grid::GridCell;
    use super::object_region::{EditField, ObjectRegion, SelectionAction, SelectionProps};
    use super::reset::Resets;
    use super::third_party::ThirdPartySlider;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{Element, PointerEvent};
    use rustify_components::workspace::splitter::initial as splitter_initial;
    use rustify_components::{
        Command, CommandPalette, Form, FormStatus, PanelTab, PanelTabs, Splitter, SubmitButton,
    };
    use rustify_components::{Support, CATALOG};
    use rustify_ui::{
        mount, navigate, provide_drags, provide_routes, use_params, Anchor, AppHandle, Button,
        Checkbox, GpuRegion, HitQuery, Load, LoadView, LocalRect, MountConfig, Navigation,
        NavigationGuard, Outcome, RegionState, Requests, Routes, Slider, TextArea, TextEdit,
        TextField, Theme, ThemeOverride, ThemePatch, ThemedScope, UiError,
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
    pub struct ObjectId(pub u32);

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
        /// Which group the object has been put in, or none. Set by dropping it
        /// on a group, which is the only way to set it: the whole point of the
        /// drag is that this is the property with no field.
        pub group: Option<u32>,
        /// The fifteen a save applies. Together with the five above they are
        /// the twenty visible properties of an object.
        pub details: crate::details::Details,
    }

    /// The groups objects can be dropped into.
    ///
    /// Twelve because the list has to be taller than the space the region can
    /// give it: a list that always fits never reaches an edge, and reaching
    /// the edge is what decides whose wheel event it is.
    const GROUPS: [(u32, &str); 12] = [
        (1, "inbox"),
        (2, "review"),
        (3, "approved"),
        (4, "shipped"),
        (5, "archive"),
        (6, "blocked"),
        (7, "spare"),
        (8, "draft"),
        (9, "legal"),
        (10, "support"),
        (11, "billing"),
        (12, "retired"),
    ];

    const PALETTE: [u32; 6] = [0x2e90fa, 0x12b76a, 0xf79009, 0xf04438, 0x7a5af8, 0x475467];

    /// The ten views of the object list. Ten because B1 says ten; filters
    /// rather than folders because the objects are one set and a person moving
    /// between views is changing what they are looking at, not where things
    /// are.
    const TABS: [(&str, &str); 10] = [
        ("all", "all"),
        ("blue", "blue"),
        ("green", "green"),
        ("amber", "amber"),
        ("red", "red"),
        ("violet", "violet"),
        ("slate", "slate"),
        ("locked", "locked"),
        ("archived", "archived"),
        ("recent", "recent"),
    ];

    /// The smallest each of the three panels may be. The application declares
    /// them because only it knows what its own content needs: a list of names,
    /// a region that has to be worth drawing, and a form of twenty fields.
    const PANEL_MINS: [f64; 3] = [220.0, 320.0, 360.0];

    /// Whether an object belongs in a view.
    fn in_tab(tab: &str, object: &WorkbenchObject, newest: u32) -> bool {
        match tab {
            "all" => true,
            "locked" => object.locked,
            "archived" => object.details.archived,
            "recent" => object.id.0 + 20 > newest,
            colour => PALETTE
                .iter()
                .position(|value| *value == object.color)
                .and_then(|index| TABS.get(index + 1))
                .is_some_and(|(id, _)| *id == colour),
        }
    }
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
                group: None,
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
        /// The path this build is served under, read from the build's own
        /// manifest by the loader. The address bar cannot say: at a deep link
        /// it is the route, not the base.
        static BASE: RefCell<String> = const { RefCell::new(String::new()) };
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
        /// Registered by the live scope: puts its state back to the page's
        /// first load, at the path the page was loaded at.
        static RESET: Seam<dyn Fn(&str)> = const { RefCell::new(BTreeMap::new()) };
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

    /// One number for the whole object list. Equal lists give equal numbers,
    /// which is all a snapshot compared with an earlier one needs, and it
    /// saves printing a thousand objects to say so.
    fn digest(objects: &[WorkbenchObject]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for object in objects {
            object.id.0.hash(&mut hasher);
            object.name.hash(&mut hasher);
            object.notes.hash(&mut hasher);
            object.color.hash(&mut hasher);
            object.locked.hash(&mut hasher);
            object.size.to_bits().hash(&mut hasher);
            object.group.hash(&mut hasher);
            object.details.hash(&mut hasher);
        }
        hasher.finish()
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
        // Everything below that a reset puts back registers here, next to
        // where it is made.
        let resets = Resets::provide();
        let objects = resets.signal(initial_objects);
        let selected = resets.signal(|| Some(ObjectId(1)));

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
        let checks = resets.signal(Vec::<(&'static str, rustify_ui::Generation)>::new);
        let saving = resets.signal(|| false);
        let saves = resets.signal(|| 0u32);
        let form = Form::new(&PROPERTY_FIELDS, move || {
            // Starting a save, not finishing one: the page says how it went,
            // and until it does the form is busy - which is what makes twenty
            // clicks one save rather than twenty.
            saves.update(|count| *count += 1);
            saving.set(true);
        });
        // The form's bookkeeping has no reset of its own, and needs none: a
        // field that moves on drops its error and any check still running for
        // it, and a save that finished leaves nothing dirty, failed or in
        // flight. Together that is a form nobody has touched.
        resets.on_reset(move || {
            for field in PROPERTY_FIELDS {
                form.changed(field);
            }
            form.submitted(Ok(()));
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
        // Set by a reset itself rather than left to the effect above. Until
        // that runs the draft would differ from the object, and the guard
        // would refuse the navigation a reset ends with.
        resets.on_reset(move || {
            draft.set(
                current
                    .get_untracked()
                    .map(|object| object.details)
                    .unwrap_or_default(),
            );
        });

        let asked_result = resets.signal(String::new);
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

        // The workspace: three panels, ten views over the object list, and
        // every command in one place.
        let panel_sizes = resets.signal(|| splitter_initial(&PANEL_MINS, 1160.0));
        let tabs = Signal::derive(|| {
            TABS.iter()
                .enumerate()
                .map(|(index, (id, label))| {
                    let tab = PanelTab::new(*id, *label);
                    // One that cannot be closed, so there is always somewhere
                    // to be.
                    if index == 0 {
                        tab.permanent()
                    } else {
                        tab
                    }
                })
                .collect::<Vec<_>>()
        });
        let open_tabs = resets.signal(|| {
            TABS.iter()
                .map(|(id, _)| id.to_string())
                .collect::<Vec<String>>()
        });
        let active_tab = resets.signal(|| "all".to_string());
        let palette_open = resets.signal(|| false);
        // Where in the region a context menu was asked for, in the region's
        // own local pixels. `None` when there is no menu open.
        let region_menu = resets.signal(|| None::<LocalRect>);

        // Everything the workspace can do, in one list. Availability is a
        // signal, so a command that cannot run says so at the moment it is
        // looked at rather than at the moment it was declared.
        let commands = Signal::derive(move || {
            let has_selection = selected.get().is_some();
            let unsaved = match current.get() {
                Some(object) => object.details != draft.get(),
                None => false,
            };
            let closable = open_tabs.with(Vec::len) > 1;
            vec![
                Command::new("next", "select the next object").keywords("move forward"),
                Command::new("previous", "select the previous object").keywords("move back"),
                if has_selection {
                    Command::new("lock", "lock the selected object").keywords("read-only")
                } else {
                    Command::new("lock", "lock the selected object")
                        .keywords("read-only")
                        .unavailable("nothing is selected")
                },
                if unsaved {
                    Command::new("save", "save the details").keywords("submit form")
                } else {
                    Command::new("save", "save the details")
                        .keywords("submit form")
                        .unavailable("there is nothing to save")
                },
                if closable {
                    Command::new("close-view", "close this view").keywords("tab panel")
                } else {
                    Command::new("close-view", "close this view")
                        .keywords("tab panel")
                        .unavailable("the last view stays open")
                },
                Command::new("theme", "switch the theme").keywords("dark light"),
            ]
        });

        provide_routes(Routes::new(&["/", "/objects", "/objects/:id"]));
        // Opening `objects/42` picks that object; after that the URL follows
        // the selection, so back and forward step through what was looked at.
        // Both directions check first, so neither can chase the other.
        let params = use_params();
        Effect::new(move || {
            let named = params
                .get()
                .get("id")
                .and_then(|id| id.parse::<u32>().ok())
                .map(ObjectId);
            match named {
                // The address names one: that is the object to show, whether
                // it came from a deep link or from pressing back.
                Some(id) if Some(id) != selected.get_untracked() => selected.set(Some(id)),
                Some(_) => {}
                // The address names none, and the application has a selection
                // of its own. The address bar catches up without adding an
                // entry - arriving at a page is not a navigation within it.
                None => {
                    if let Some(id) = selected.get_untracked() {
                        navigate(&format!("/objects/{}", id.0), true);
                    }
                }
            }
        });
        // Unsaved work in the details form is what a guard refuses to leave.
        // The live four are applied as they are typed, so there is nothing
        // unsaved about them; what this compares is the draft against the
        // object it came from.
        NavigationGuard::register(Signal::derive(move || match current.get() {
            Some(object) => object.details != draft.get(),
            None => false,
        }));

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
        let hovered = resets.signal(|| None::<ObjectId>);
        // One drag for the scope. Both halves of the page take part in the
        // same one, which is the point: a pointer cannot be in two drags.
        let drags = provide_drags();
        resets.on_reset(move || {
            if drags.dragging() {
                drags.cancel();
            }
        });
        // The scope's language, so the SDK's own words - a retry, a dialog's
        // close button - are in the same language as the application's.
        rustify_ui::provide_locale(rustify_ui::Locale::English);
        // The question in flight, projected into the region. The region is not
        // running while the pointer moves over the page, so this is how it
        // hears the question at all.
        let hit = resets.signal(|| None::<HitQuery>);
        // The group a release would land in, as the region answered. Kept by
        // the application so that both the highlight the region draws and the
        // drop that follows come from one fact.
        let drop_target = resets.signal(|| None::<u32>);
        // The DOM target under the pointer, for the same reason.
        let dom_target = resets.signal(|| None::<String>);
        // What a wheel at the end of the group list is for. A policy with one
        // value is not a policy, so it is a control rather than a constant.
        let wheel_propagates = resets.signal(|| true);
        // Exactly one per delivered drop, so a hundred drags can be counted
        // rather than inspected.
        let drops = resets.signal(|| 0u32);
        let drag_cancels = resets.signal(|| 0u32);
        // How far down the region's group list is, as the region reported it.
        // A wheel that the region kept moves this; one it handed to the page
        // does not, which is the whole difference D14 is about.
        let group_scroll = RwSignal::new((0.0f64, 0.0f64));
        // Where a press started, and on which object. A press is not a drag:
        // a drag begins once the pointer has travelled far enough that it
        // cannot have been a click.
        let press = resets.signal(|| None::<(f64, f64, ObjectId)>);
        // Set when a drag delivered something, so the click that follows the
        // release does not also select what was just dropped.
        let dragged = StoredValue::new(false);
        resets.on_reset(move || dragged.set_value(false));

        // The groups, with what is in them. Rebuilt when the objects change,
        // like the cells: a drag moving does not change the groups.
        let groups = Memo::new(move |_| {
            Arc::new(objects.with(|objects| {
                GROUPS
                    .iter()
                    .map(|(id, name)| Group {
                        id: *id,
                        name: name.to_string(),
                        members: objects
                            .iter()
                            .filter(|object| object.group == Some(*id))
                            .count(),
                    })
                    .collect::<Vec<_>>()
            }))
        });
        let theme = resets.signal(Theme::light);
        // The rectangle the region drew the name into, while a native control
        // is editing it. The application decides when the session exists.
        let editing = resets.signal(|| None::<(EditField, LocalRect)>);
        // How many edit sessions ended because the value moved underneath them.
        let invalidated = resets.signal(|| 0u32);
        let canvas = NodeRef::<leptos::html::Canvas>::new();
        // How many times the application has asked the region to open a link.
        // A region is not the page, so the embedded contract refuses it; what
        // is being exercised is that the refusal is reported rather than
        // silently swallowed. Not reset: the region acts on a number higher
        // than the last it saw, so winding it back would silence the next ask.
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
                    groups: groups.get(),
                    drop_target: drop_target.get(),
                    hit: hit.get(),
                    wheel_propagates: wheel_propagates.get(),
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
                    groups: groups.get(),
                    drop_target: drop_target.get(),
                    hit: hit.get(),
                    wheel_propagates: wheel_propagates.get(),
                    theme,
                },
            }
        });

        // One place moves the selection; the DOM buttons and the GPU actions
        // both go through it, so there is a single rule for what "next" means.
        // The URL names the object showing, so choosing one is a navigation -
        // and a navigation can be refused. Every path that picks an object
        // goes through here, so "there is unsaved work" is answered once
        // rather than at five call sites that would drift apart.
        let select = move |id: Option<ObjectId>| -> bool {
            if selected.get_untracked() == id {
                return true;
            }
            let to = match id {
                Some(id) => format!("/objects/{}", id.0),
                None => "/objects".to_string(),
            };
            if navigate(&to, false) != Navigation::Done {
                return false;
            }
            selected.set(id);
            true
        };

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
            select(Some(id));
        };

        // Why a value the user asked for was not taken, or `None` when the
        // last one was. The control shows the application's value either way;
        // this says out loud why it is not what was typed.
        let refusal = resets.signal(|| None::<String>);
        let refusals = resets.signal(|| 0u32);

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
                        group: None,
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
                navigate("/objects", true);
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
                return select(Some(ObjectId(id)));
            }
            false
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
        let find_id = resets.signal(String::new);
        let find_result = resets.signal(String::new);
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
        let details = resets.signal(|| Load::<String>::Loading);
        let requests = Requests::new();
        // An answer still on its way was asked for by the state a reset threw
        // away, so it must not land in the state put back.
        resets.on_reset({
            let requests = requests.clone();
            move || requests.cancel()
        });
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

        let rejected = resets.signal(|| None::<u32>);
        // Actions the scope refused because its queue was full. They never ran
        // and changed nothing, so the panel says so instead of pretending.
        let refused = resets.signal(|| 0usize);
        // Counts what the application was actually handed, so a run of actions
        // can be checked for losses and duplicates rather than only for where
        // the selection ended up - the selection wraps, a count does not.
        let accepted = resets.signal(|| 0u32);
        // Counted apart from the rest: a pointer stream is collapsed on its way
        // here, so its count says how much of it survived, not how much of it
        // happened.
        let hovers = resets.signal(|| 0u32);

        // What the region says about itself, so the page can tell a region
        // that is rebuilding from one that failed.
        let region_state = RwSignal::new(RegionState::Starting);

        // Where the region drew the controls it owns, as it reported them.
        // Business state like any other: nobody keeps a second copy of the
        // region's layout.
        // What an import will take. The picker offers the same list, because
        // the two come from one place and so cannot drift apart.
        let limits = rustify_ui::Limits {
            // A thousand objects is about 30 KB of text; a quarter of a
            // megabyte is room for a file that grew, and a wall in front of
            // one somebody picked by mistake.
            max_bytes: 256 * 1024,
            kinds: &[".txt", ".bin"],
        };
        // What the last import or export did, in the application's own words.
        let transfer_status = resets.signal(String::new);
        let imports = resets.signal(|| 0u32);
        let exports = resets.signal(|| 0u32);
        // The bytes of the last export, so a test can compare what the browser
        // downloaded against what the application meant to write.
        let exported = StoredValue::new(Vec::<u8>::new());
        resets.on_reset(move || exported.set_value(Vec::new()));
        let clipboard_status = resets.signal(String::new);
        let copies = resets.signal(|| 0u32);
        let pastes = resets.signal(|| 0u32);
        // Set when the clipboard refuses, so the view can offer the path that
        // always works: the text, selected, for the user to copy themselves.
        let copy_by_hand = resets.signal(|| false);

        let controls =
            RwSignal::new(None::<(LocalRect, LocalRect, LocalRect, LocalRect, LocalRect, f64)>);

        // Whether the third-party component is in the view, and every callback
        // it has made. A rebuild that left a subscription behind would show up
        // as two callbacks for one change.
        let third_party_present = resets.signal(|| true);
        let third_party_updates = resets.signal(|| 0u32);
        publish(
            &PRESENT,
            registration,
            Rc::new(move |present| third_party_present.set(present)) as Rc<dyn Fn(bool)>,
        );
        on_cleanup(move || withdraw(&PRESENT, registration));

        // One area of the scope with part of the theme changed. It writes only
        // what it names, onto its own element, so the rest of the scope and
        // every other scope keep the values they had.
        let emphasis = resets.signal(|| false);
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

        // The whole list as one number for the snapshot, recomputed when the
        // objects change rather than whenever the selection moves.
        let listed = Memo::new(move |_| objects.with(|objects| digest(objects)));
        Effect::new(move || {
            let (index, total) = position.get();
            let current = current.get();
            let snapshot = format!(
                "{{\"count\":{},\"position\":{},\"selected\":{},\"name\":{},\"color\":\"{}\",\"first_colors\":\"{}\",\"first_ids\":\"{}\",\"accepted\":{},\"refused\":{},\"hovered\":{},\"hovers\":{},\"editing\":{},\"invalidated\":{},\"notes\":{},\"theme\":\"{}\",\"details\":\"{}\",\"details_value\":{},\"locked\":{},\"size\":{},\"refusals\":{},\"refusal\":{},\"third_party\":{},\"third_party_updates\":{},\"controls\":{},\"region\":\"{}\",\"form\":{},\"path\":{},\"guarded\":{},\"workspace\":{},\"drag\":{},\"transfer\":{},\"find\":{{\"id\":{},\"result\":{}}},\"emphasis\":{},\"rejected\":{},\"objects\":\"{:016x}\"}}",
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
                    .map(|(locked, size, name, notes, groups, row)| {
                        format!(
                            "{{\"locked\":{},\"size\":{},\"name\":{},\"notes\":{},\"groups\":{},\"row\":{:.1}}}",
                            rect_json(locked),
                            rect_json(size),
                            rect_json(name),
                            rect_json(notes),
                            rect_json(groups),
                            row,
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
                json_string(&rustify_ui::use_location().get().path),
                match current.as_ref() {
                    Some(object) => object.details != draft.get(),
                    None => false,
                },
                format!(
                    "{{\"panels\":[{}],\"tabs\":{},\"tab\":{},\"palette\":{},\"menu\":{}}}",
                    panel_sizes
                        .get()
                        .iter()
                        // Three decimals, not none: a drag moves room between
                        // two panels exactly, and rounding each one separately
                        // would make the row look a pixel wider than it is.
                        .map(|size| format!("{:.3}", size))
                        .collect::<Vec<_>>()
                        .join(","),
                    open_tabs.with(Vec::len),
                    json_string(&active_tab.get()),
                    palette_open.get(),
                    region_menu.get().is_some(),
                ),
                format!(
                    "{{\"drops\":{},\"cancels\":{},\"grouped\":{},\"dragging\":{},\"target\":{},\"selected_group\":{},\"propagates\":{},\"scroll\":{:.1},\"scroll_max\":{:.1}}}",
                    drops.get(),
                    drag_cancels.get(),
                    objects
                        .with(|objects| objects
                            .iter()
                            .filter(|object| object.group.is_some())
                            .count()),
                    drags.dragging(),
                    match (drop_target.get(), dom_target.get()) {
                        (Some(group), _) => format!("\"group-{group}\""),
                        (None, Some(target)) => json_string(&target),
                        (None, None) => "null".to_string(),
                    },
                    current
                        .as_ref()
                        .and_then(|object| object.group)
                        .map(|group| group.to_string())
                        .unwrap_or_else(|| "null".to_string()),
                    wheel_propagates.get(),
                    group_scroll.get().0,
                    group_scroll.get().1,
                ),
                format!(
                    "{{\"imports\":{},\"exports\":{},\"status\":{},\"bytes\":{},\"copies\":{},\"pastes\":{},\"clipboard\":{},\"by_hand\":{},\"can_copy\":{}}}",
                    imports.get(),
                    exports.get(),
                    json_string(&transfer_status.get()),
                    exported.with_value(Vec::len),
                    copies.get(),
                    pastes.get(),
                    json_string(&clipboard_status.get()),
                    copy_by_hand.get(),
                    rustify_ui::clipboard::available(),
                ),
                json_string(&find_id.get()),
                json_string(&find_result.get()),
                emphasis.get(),
                rejected
                    .get()
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                listed.get(),
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        // What a drop does. One place, because a drop can arrive from three
        // routes - a DOM target answering on the spot, a region answering
        // later, or a release that was already decided - and "what happens
        // when an object lands somewhere" must not have three answers.
        let deliver = move |outcome: Outcome| {
            if let Outcome::Drop { target, payload } = outcome {
                if let Ok(id) = payload.parse::<u32>() {
                    let group = target
                        .strip_prefix("group-")
                        .and_then(|group| group.parse::<u32>().ok());
                    if group.is_some() || target == "ungrouped" {
                        objects.update(|objects| {
                            if let Some(object) =
                                objects.iter_mut().find(|object| object.id.0 == id)
                            {
                                object.group = group;
                            }
                        });
                        drops.update(|n| *n += 1);
                    }
                }
            }
            drop_target.set(None);
            dom_target.set(None);
            hit.set(None);
        };

        // Whether a DOM target would take this object.
        //
        // The bin refuses a locked object: a lock is a business rule about the
        // object, not a property of the control, and a target that says no is
        // the other half of the drag contract.
        let accepts = move |target: &str, payload: &str| -> bool {
            if target != "ungrouped" {
                return false;
            }
            let Ok(id) = payload.parse::<u32>() else {
                return false;
            };
            !objects.with(|objects| {
                objects
                    .iter()
                    .any(|object| object.id.0 == id && object.locked)
            })
        };

        // Where the pointer is now, in terms a drag can use.
        //
        // The region is asked rather than told, because only it knows what it
        // drew; a DOM target answers on the spot, because it is running.
        let point_at = move |client_x: f64, client_y: f64| {
            let under = leptos::web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.element_from_point(client_x as f32, client_y as f32));
            if let (Some(under), Some(canvas)) = (under.as_ref(), canvas.get_untracked()) {
                let canvas: leptos::web_sys::Element = canvas.into();
                if under.is_same_node(Some(canvas.as_ref())) {
                    let box_ = canvas.get_bounding_client_rect();
                    if let Some(query) =
                        drags.over_region("view", client_x - box_.left(), client_y - box_.top())
                    {
                        dom_target.set(None);
                        hit.set(Some(query));
                    }
                    return;
                }
            }
            let name = under.as_ref().and_then(|element| {
                element
                    .closest("[data-drop-target]")
                    .ok()
                    .flatten()
                    .and_then(|zone| zone.get_attribute("data-drop-target"))
            });
            let Some(query) = drags.over(name.as_deref()) else {
                if name.is_none() {
                    // Left everything: nothing would take a release here, and
                    // both halves have to stop saying it would.
                    drop_target.set(None);
                    dom_target.set(None);
                    hit.set(None);
                }
                return;
            };
            hit.set(None);
            drop_target.set(None);
            let payload = drags.payload().unwrap_or_default();
            let accepted = accepts(&query.target, &payload);
            dom_target.set(accepted.then(|| query.target.clone()));
            if let Some(outcome) = drags.answer(query.session, query.seq, accepted) {
                deliver(outcome);
            }
        };

        let end_drag = move || {
            if !drags.dragging() {
                press.set(None);
                return;
            }
            dragged.set_value(true);
            press.set(None);
            match drags.release() {
                // The region has been asked and has not answered. Its answer
                // decides, and it arrives on the action path like every other
                // thing the region says.
                Outcome::Waiting => {}
                outcome => deliver(outcome),
            }
        };

        let cancel_drag = move || {
            if drags.dragging() {
                drags.cancel();
                drag_cancels.update(|n| *n += 1);
            }
            press.set(None);
            drop_target.set(None);
            dom_target.set(None);
            hit.set(None);
        };

        let apply_records = move |records: Vec<crate::transfer::Record>| {
            let mut applied = 0usize;
            objects.update(|objects| {
                for record in &records {
                    let Some(object) = objects.iter_mut().find(|o| o.id.0 == record.id) else {
                        continue;
                    };
                    if let Some(name) = &record.name {
                        object.name = name.clone();
                    }
                    object.group = record.group;
                    object.locked = record.locked;
                    object.size = record.size;
                    applied += 1;
                }
            });
            applied
        };

        let receive = move |files: Option<leptos::web_sys::FileList>| {
            rustify_ui::files::import_files(files, limits, move |import| match import {
                rustify_ui::Import::Loaded { name, bytes } => {
                    match crate::transfer::parse(&name, &bytes) {
                        Ok(records) => {
                            let applied = apply_records(records);
                            imports.update(|n| *n += 1);
                            transfer_status.set(format!("imported {applied} objects"));
                        }
                        // A file that parses wrong changes nothing: the
                        // records were all read before any of them was
                        // applied, so there is no half-imported state to be
                        // in.
                        Err(malformed) => {
                            transfer_status.set(format!("{name} is not readable: {malformed}"))
                        }
                    }
                }
                rustify_ui::Import::Refused { name, why } => {
                    transfer_status.set(format!("{name} was not read: {why}"))
                }
                // Dismissing a picker is not an error and not worth a message
                // that says nothing happened. It is recorded so a test can see
                // that nothing did.
                rustify_ui::Import::Aborted => transfer_status.set("nothing chosen".to_string()),
            });
        };

        let send = move |binary: bool| {
            let records = objects.with(|objects| {
                objects
                    .iter()
                    .map(|object| crate::transfer::Record {
                        id: object.id.0,
                        name: Some(object.name.clone()),
                        group: object.group,
                        locked: object.locked,
                        size: object.size,
                    })
                    .collect::<Vec<_>>()
            });
            let bytes = if binary {
                crate::transfer::to_binary(&records)
            } else {
                crate::transfer::to_text(&records)
            };
            let (name, mime) = if binary {
                ("objects.bin", "application/octet-stream")
            } else {
                ("objects.txt", "text/plain")
            };
            exported.set_value(bytes.clone());
            match rustify_ui::files::export(name, &bytes, mime) {
                Ok(()) => {
                    exports.update(|n| *n += 1);
                    transfer_status.set(format!("exported {} bytes to {name}", bytes.len()));
                }
                // The browser refused to start the download. Saying so is the
                // whole of what can be done about it, and it is more than
                // saying nothing.
                Err(_) => transfer_status.set(format!("{name} could not be downloaded")),
            }
        };

        // Copying the notes, and what to do when the browser says no.
        //
        // A refusal is never dressed up as a success (D9): the control says it
        // could not, and offers the path that needs no permission - the text,
        // selected, in a control the user can press the copy key in.
        let copy_notes = move || {
            let notes = current.get_untracked().map(|o| o.notes).unwrap_or_default();
            if !rustify_ui::clipboard::available() {
                copy_by_hand.set(true);
                clipboard_status.set("this build cannot reach the clipboard".to_string());
                return;
            }
            rustify_ui::clipboard::copy(&notes, move |result| match result {
                Ok(()) => {
                    copies.update(|n| *n += 1);
                    copy_by_hand.set(false);
                    clipboard_status.set("copied".to_string());
                }
                Err(error) => {
                    copy_by_hand.set(true);
                    clipboard_status.set(format!("{error}; the notes are selected, press copy"));
                }
            });
        };

        let paste_notes = move || {
            if !rustify_ui::clipboard::available() {
                clipboard_status.set("this build cannot reach the clipboard".to_string());
                return;
            }
            rustify_ui::clipboard::paste(move |result| match result {
                Ok(text) => {
                    pastes.update(|n| *n += 1);
                    clipboard_status.set(format!("pasted {} characters", text.chars().count()));
                    renote(text);
                }
                Err(error) => {
                    clipboard_status
                        .set(format!("{error}; paste into the notes with the keyboard"));
                }
            });
        };

        // Selects the notes in their own control, which is what "press copy
        // yourself" needs to be true rather than an instruction.
        Effect::new(move || {
            if !copy_by_hand.get() {
                return;
            }
            if let Some(field) = leptos::web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| {
                    document
                        .query_selector("[data-testid=\"notes-input\"]")
                        .ok()
                })
                .flatten()
                .and_then(|element| {
                    element
                        .dyn_into::<leptos::web_sys::HtmlTextAreaElement>()
                        .ok()
                })
            {
                let _ = field.focus();
                field.select();
            }
        });

        // Escape ends a drag wherever the pointer is. The key does not arrive
        // at the element holding the pointer, so the window is where it has to
        // be listened for.
        let escape = window_event_listener(leptos::ev::keydown, move |event| {
            if event.key() == "Escape" {
                cancel_drag();
            }
        });
        on_cleanup(move || escape.remove());

        let label_id = format!("third-party-label-{registration}");
        let app = PhantomData::<ObjectRegion>;
        let on_action = move |action| {
            match action {
                SelectionAction::Hover(id) => {
                    hovers.update(|n| *n += 1);
                    hovered.set(id.map(ObjectId));
                }
                SelectionAction::Scrolled { at, max } => {
                    group_scroll.set((at, max));
                }
                SelectionAction::Hit(answer) => {
                    // What the region found, in the two places it matters: the
                    // highlight it draws, and the drag that may already be
                    // waiting on this exact answer.
                    let target = answer
                        .target
                        .as_deref()
                        .and_then(|target| target.strip_prefix("group-"))
                        .and_then(|group| group.parse::<u32>().ok());
                    if let Some(outcome) = drags.hit(&answer) {
                        deliver(outcome);
                    } else {
                        drop_target.set(target);
                    }
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
                    select(Some(ObjectId(id)));
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
                SelectionAction::Controls {
                    locked,
                    size,
                    name,
                    notes,
                    groups,
                    row,
                } => {
                    controls.set(Some((locked, size, name, notes, groups, row)));
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

        let run_command = move |id: String| match id.as_str() {
            "next" => step(1),
            "previous" => step(-1),
            "lock" => {
                if let Some(object) = current.get_untracked() {
                    set_locked(!object.locked);
                }
            }
            "save" => {
                palette_open.set(false);
                let object = current.get_untracked();
                let asked = draft.with_untracked(|draft| {
                    property_rules(
                        object.as_ref().map(|o| o.name.as_str()).unwrap_or(""),
                        object.as_ref().is_some_and(|o| o.locked),
                        object
                            .as_ref()
                            .map(|o| o.details.owner.as_str())
                            .unwrap_or(""),
                        draft,
                    )
                });
                let _ = form.submit(|| asked);
            }
            "close-view" => {
                let closing = active_tab.get_untracked();
                let open = open_tabs.get_untracked();
                let strip: Vec<PanelTab> = tabs
                    .get_untracked()
                    .into_iter()
                    .filter(|tab| open.contains(&tab.id))
                    .collect();
                if let Some(next) = rustify_components::workspace::panel_tabs::after_close(
                    &strip, &closing, &closing,
                ) {
                    open_tabs.update(|open| open.retain(|id| *id != closing));
                    active_tab.set(next);
                }
            }
            "theme" => theme.update(|theme| {
                *theme = if theme.name == "light" {
                    Theme::dark()
                } else {
                    Theme::light()
                };
            }),
            _ => {}
        };

        // The shortcut a workspace is expected to have. Registered on the
        // scope's own container rather than the window: a page with two of
        // these on it should not have them fighting over one key. The closure
        // and the element go into a `SendWrapper` because a cleanup has to be
        // `Send` and a DOM handle is not - they never leave this thread.
        Effect::new(move || {
            let Some(roots) = use_context::<rustify_ui::ScopeRoots>() else {
                return;
            };
            let container = roots.container();
            let handler = leptos::wasm_bindgen::closure::Closure::<
                dyn FnMut(leptos::web_sys::KeyboardEvent),
            >::new(move |event: leptos::web_sys::KeyboardEvent| {
                if event.key() == "k" && (event.meta_key() || event.ctrl_key()) {
                    event.prevent_default();
                    palette_open.set(true);
                }
            });
            let _ = container.add_event_listener_with_callback(
                "keydown",
                leptos::wasm_bindgen::JsCast::unchecked_ref(handler.as_ref()),
            );
            let held = send_wrapper::SendWrapper::new((container, handler));
            on_cleanup(move || {
                let (container, handler) = &*held;
                let _ = container.remove_event_listener_with_callback(
                    "keydown",
                    leptos::wasm_bindgen::JsCast::unchecked_ref(handler.as_ref()),
                );
            });
        });

        // A reset: everything registered above goes back to its first value,
        // and the address to the one the page was loaded at. It runs under
        // this scope's owner because a navigation finds its router there.
        if let Some(owner) = Owner::current() {
            publish(
                &RESET,
                registration,
                Rc::new(move |path: &str| {
                    owner.with(|| {
                        resets.run();
                        navigate(path, true);
                    })
                }) as Rc<dyn Fn(&str)>,
            );
            on_cleanup(move || withdraw(&RESET, registration));
        }

        let resize_panels = move |next: Vec<f64>| panel_sizes.set(next);
        let run_from_palette = move |id: String| run_command(id);

        // The first of the three panels. The other two are the ones that were
        // already here.
        let objects_panel = move || {
            let open = open_tabs.get();
            let strip: Vec<PanelTab> = tabs
                .get()
                .into_iter()
                .filter(|tab| open.contains(&tab.id))
                .collect();
            let newest = objects.with(|objects| objects.iter().map(|o| o.id.0).max().unwrap_or(0));
            let showing = objects.with(|objects| {
                objects
                    .iter()
                    .filter(|object| in_tab(&active_tab.get(), object, newest))
                    .map(|object| (object.id, object.name.clone()))
                    .collect::<Vec<_>>()
            });
            view! {
                <section class="objects" aria-label="objects" data-testid="objects-panel">
                    <PanelTabs
                        tabs=Signal::derive(move || strip.clone())
                        active=active_tab
                        aria_label="views of the objects"
                        test_id="object-views"
                        on_activate=move |id: String| active_tab.set(id)
                        on_close=move |id: String| {
                            // The strip asks; the application decides, and it
                            // is the application that knows what the next view
                            // should be.
                            let open = open_tabs.get_untracked();
                            let strip: Vec<PanelTab> = tabs
                                .get_untracked()
                                .into_iter()
                                .filter(|tab| open.contains(&tab.id))
                                .collect();
                            if let Some(next) =
                                rustify_components::workspace::panel_tabs::after_close(
                                    &strip,
                                    &active_tab.get_untracked(),
                                    &id,
                                )
                            {
                                open_tabs.update(|open| open.retain(|open| *open != id));
                                active_tab.set(next);
                            }
                        }
                    />
                    <p class="objects-count" data-testid="objects-count">
                        {format!("{} objects", showing.len())}
                    </p>
                    <ul
                        class="objects-list"
                        data-testid="objects-list"
                        // The DOM says the same thing the region reports to
                        // the host: `contain` keeps a wheel at the end of this
                        // list, `auto` gives it to the page. One control drives
                        // both halves, because one rule cannot have two values.
                        style:overscroll-behavior=move || {
                            if wheel_propagates.get() { "auto" } else { "contain" }
                        }
                    >
                        {showing
                            .into_iter()
                            .map(|(id, name)| {
                                let chosen = Memo::new(move |_| selected.get() == Some(id));
                                view! {
                                    <li>
                                        <button
                                            type="button"
                                            class="objects-item"
                                            data-testid=format!("object-{}", id.0)
                                            aria-current=move || chosen.get().then_some("true")
                                            on:pointerdown=move |event: PointerEvent| {
                                                // A press, not yet a drag. The
                                                // row is still a button, and a
                                                // button that started a drag on
                                                // every press could not be
                                                // clicked.
                                                if let Some(element) = event
                                                    .target()
                                                    .and_then(|target| {
                                                        target.dyn_into::<Element>().ok()
                                                    })
                                                {
                                                    let _ = element
                                                        .set_pointer_capture(event.pointer_id());
                                                }
                                                dragged.set_value(false);
                                                press
                                                    .set(
                                                        Some((
                                                            event.client_x() as f64,
                                                            event.client_y() as f64,
                                                            id,
                                                        )),
                                                    );
                                            }
                                            on:pointermove=move |event: PointerEvent| {
                                                let (x, y) = (
                                                    event.client_x() as f64,
                                                    event.client_y() as f64,
                                                );
                                                if let Some((from_x, from_y, id)) = press
                                                    .get_untracked()
                                                {
                                                    if !drags.dragging()
                                                        && (x - from_x).abs().max((y - from_y).abs())
                                                            > 4.0
                                                    {
                                                        drags
                                                            .start(
                                                                format!("object-{}", id.0),
                                                                id.0.to_string(),
                                                            );
                                                    }
                                                }
                                                if drags.dragging() {
                                                    point_at(x, y);
                                                }
                                            }
                                            on:pointerup=move |_| end_drag()
                                            on:pointercancel=move |_| cancel_drag()
                                            on:click=move |_| {
                                                // A release that delivered
                                                // something is not also a
                                                // click: the object was moved,
                                                // not chosen.
                                                if dragged.get_value() {
                                                    dragged.set_value(false);
                                                    return;
                                                }
                                                select(Some(id));
                                            }
                                        >
                                            {name}
                                        </button>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                    <div
                        class="objects-bin"
                        data-drop-target="ungrouped"
                        data-testid="ungrouped-bin"
                        aria-label="take an object out of its group"
                        data-active=move || {
                            dom_target.get().is_some_and(|target| target == "ungrouped")
                        }
                    >
                        "no group"
                    </div>
                </section>
            }
        };

        view! {
            <div class="workbench">
                <ThemedScope theme=theme />
                <div
                    class="workspace"
                    data-testid="workspace"
                    style:grid-template-columns=move || {
                        rustify_components::workspace::columns(&panel_sizes.get())
                    }
                >
                {objects_panel()}
                <section class="panel" aria-label="object properties">
                    <h2>"properties"</h2>
                    <Show
                        when=move || {
                            params.get().contains_key("id") && current.get().is_none()
                        }
                        fallback=|| ()
                    >
                        <p class="region-error" role="alert" data-testid="not-found">
                            {move || {
                                format!(
                                    "no object {}; the address is kept so it can be corrected",
                                    params.get().get("id").cloned().unwrap_or_default(),
                                )
                            }}
                        </p>
                    </Show>
                    <Button
                        test_id="open-commands"
                        aria_label="open the command palette"
                        on_click=move || palette_open.set(true)
                    >
                        "commands"
                    </Button>
                    <Button
                        test_id="open-link-from-region"
                        aria_label="ask the region to open a link"
                        on_click=move || open_link_requests.update(|n| *n += 1)
                    >
                        "open a link from the region"
                    </Button>
                    <Checkbox
                        test_id="wheel-propagates"
                        label="a wheel past the end of the groups scrolls the page"
                        checked=Signal::derive(move || wheel_propagates.get())
                        on_change=move |value: bool| wheel_propagates.set(value)
                    />
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
                    <div class="transfer" aria-label="files and the clipboard">
                        <Button
                            test_id="export-text"
                            aria_label="write the objects out as text"
                            on_click=move || send(false)
                        >
                            "export text"
                        </Button>
                        <Button
                            test_id="export-binary"
                            aria_label="write the objects out as bytes"
                            on_click=move || send(true)
                        >
                            "export binary"
                        </Button>
                        <rustify_components::FilePicker
                            test_id="import-file"
                            aria_label="read objects from a file"
                            accept=Signal::derive(move || limits.accept_attribute())
                            on_files=move |files| receive(files)
                        />
                        <rustify_components::DropZone
                            test_id="import-drop"
                            label="or drop one here"
                            on_files=move |files| receive(files)
                        />
                        <Button
                            test_id="copy-notes"
                            aria_label="copy the notes"
                            on_click=move || copy_notes()
                        >
                            "copy notes"
                        </Button>
                        <Button
                            test_id="paste-notes"
                            aria_label="paste into the notes"
                            on_click=move || paste_notes()
                        >
                            "paste notes"
                        </Button>
                        <p role="status" aria-label="transfer" data-testid="transfer-status">
                            {move || transfer_status.get()}
                        </p>
                        <p role="status" aria-label="clipboard" data-testid="clipboard-status">
                            {move || clipboard_status.get()}
                        </p>
                    </div>
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
                <div
                    on:contextmenu=move |event: leptos::ev::MouseEvent| {
                        // A menu on a rectangle *inside* the canvas: the region
                        // cannot draw one - a popup that leaves the canvas is a
                        // DOM layer - so the application opens the DOM one
                        // against the point that was clicked.
                        let Some(canvas) = canvas.get_untracked() else {
                            return;
                        };
                        event.prevent_default();
                        let box_ = canvas.get_bounding_client_rect();
                        region_menu
                            .set(
                                Some(
                                    LocalRect::new(
                                        event.client_x() as f64 - box_.left(),
                                        event.client_y() as f64 - box_.top(),
                                        1.0,
                                        1.0,
                                    ),
                                ),
                            );
                    }
                >
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
                <rustify_components::Menu
                    test_id="region-menu"
                    aria_label="what can be done here"
                    open=Signal::derive(move || region_menu.get().is_some())
                    on_open_change=move |open: bool| {
                        if !open {
                            region_menu.set(None);
                        }
                    }
                    anchor=Signal::derive(move || {
                        match (canvas.get(), region_menu.get()) {
                            (Some(canvas), Some(at)) => Anchor::region(&canvas.into(), at),
                            _ => Anchor::Centred,
                        }
                    })
                    items=Signal::derive(move || {
                        let locked = current.get().is_some_and(|object| object.locked);
                        vec![
                            rustify_components::MenuItem::new("next", "select the next object"),
                            rustify_components::MenuItem::new(
                                "lock",
                                if locked { "unlock this object" } else { "lock this object" },
                            ),
                            match current.get() {
                                Some(_) => rustify_components::MenuItem::new(
                                    "theme",
                                    "switch the theme",
                                ),
                                None => rustify_components::MenuItem::new(
                                        "theme",
                                        "switch the theme",
                                    )
                                    .disabled("nothing is selected"),
                            },
                        ]
                    })
                    on_activate=move |id: String| run_command(id)
                />
                <Splitter
                    sizes=panel_sizes
                    mins=PANEL_MINS.to_vec()
                    on_resize=resize_panels
                    divider_label="resize panel"
                    test_id="dividers"
                />
                </div>
                <CommandPalette
                    open=palette_open
                    on_open_change=move |open| palette_open.set(open)
                    commands=commands
                    on_run=run_from_palette
                    placeholder="type a command"
                    aria_label="workspace commands"
                />
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
        // This application wants the address bar. If another instance on the
        // page already has it, it runs with a location of its own instead -
        // which is what `UrlOwnerConflict` is for, and the only sensible
        // answer to it.
        let owned = MountConfig {
            scope: container_id.to_string(),
            url_owner: true,
            base: BASE.with(|base| base.borrow().clone()),
        };
        let handle = match mount(container.clone(), owned, || view! { <Workbench /> }) {
            Err(UiError::UrlOwnerConflict) => mount(
                container,
                MountConfig {
                    scope: container_id.to_string(),
                    url_owner: false,
                    base: String::new(),
                },
                || view! { <Workbench /> },
            ),
            other => other,
        }
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

    /// The path this build is served under, from the loader.
    #[wasm_bindgen]
    pub fn workbench_set_base(base: &str) {
        BASE.with(|slot| *slot.borrow_mut() = base.to_string());
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

    /// The first half of a reset. Every scope but `main` closes, and what
    /// outlives a scope is forgotten. The third-party component comes out of
    /// `main` here so that the second half builds it again the way a first
    /// load does - created at the value in force, with the one callback that
    /// creating it makes - rather than moving the old one to a new value.
    /// Answers whether `main` is still mounted; if it is not, the page mounts
    /// it again instead of resetting it.
    #[wasm_bindgen]
    pub fn workbench_reset_page(main: u32) -> bool {
        let others = HANDLES.with(|handles| {
            let mut handles = handles.borrow_mut();
            let kept = handles.remove(&main);
            let others = std::mem::take(&mut *handles);
            if let Some(kept) = kept {
                handles.insert(main, kept);
            }
            others
        });
        // Dropped outside the borrow: closing a scope runs its cleanups.
        drop(others);
        CLOSE_ON_ACTION.with(|armed| armed.set(false));
        DELETED.with(|deleted| deleted.borrow_mut().clear());
        rustify_ui::set_recording(true);
        let mounted = HANDLES.with(|handles| handles.borrow().contains_key(&main));
        if let (true, Some(present)) = (mounted, newest(&PRESENT)) {
            present(false);
        }
        mounted
    }

    /// The second half: the page's scope goes back to the state it loaded in,
    /// at `address`, the path and query the page was loaded at.
    #[wasm_bindgen]
    pub fn workbench_reset_scope(address: &str) -> bool {
        let base = BASE.with(|base| base.borrow().clone());
        let path = rustify_ui::router::strip_base(&base, address).unwrap_or("/");
        match newest(&RESET) {
            Some(reset) => {
                reset(path);
                true
            }
            None => false,
        }
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
