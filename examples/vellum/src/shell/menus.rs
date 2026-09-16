use crate::browser_frame::{request_animation_frame_with_handle, AnimationFrameRequestHandle};
use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, HtmlElement, KeyboardEvent};

use crate::{affine::Point, app::Editor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuKind {
    Main,
    Selection,
    Zoom,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenuState {
    pub kind: MenuKind,
    pub position: Point,
}

pub fn open_main(editor: Editor) {
    open(editor, MenuKind::Main, Point::new(9.0, 46.0));
}

pub fn open_selection(editor: Editor, position: Option<Point>) {
    let position = position.unwrap_or_else(|| {
        Point::new(
            window()
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0)
                - 295.0,
            113.0,
        )
    });
    open(editor, MenuKind::Selection, position);
}

pub fn open_zoom(editor: Editor, client: Point) {
    open(
        editor,
        MenuKind::Zoom,
        Point::new(client.x - 150.0, client.y - 150.0),
    );
}

fn open(editor: Editor, kind: MenuKind, position: Point) {
    editor
        .shell
        .update(|shell| shell.menu = Some(MenuState { kind, position }));
}

pub fn close(editor: Editor) {
    editor.shell.update(|shell| shell.menu = None);
}

enum Entry {
    Header(&'static str),
    Divider,
    Item {
        label: String,
        action: &'static str,
        key: &'static str,
        disabled: bool,
    },
}

fn item(label: impl Into<String>, action: &'static str, key: &'static str) -> Entry {
    Entry::Item {
        label: label.into(),
        action,
        key,
        disabled: false,
    }
}

fn entries(editor: Editor, kind: MenuKind) -> Vec<Entry> {
    use Entry::Divider as D;
    let grid = if editor.grid.get_untracked() {
        "Hide dot grid"
    } else {
        "Show dot grid"
    };
    match kind {
        MenuKind::Main => vec![
            Entry::Header("Vellum — make room for ideas"),
            item("New document", "newFile", "⌘N"),
            item("Open document…", "openFile", "⌘O"),
            item("Save portable document", "saveFile", "⌘S"),
            D,
            Entry::Item {
                label: "Undo".into(),
                action: "undo",
                key: "⌘Z",
                disabled: editor.history.with_value(|h| h.undo_len() == 0),
            },
            Entry::Item {
                label: "Redo".into(),
                action: "redo",
                key: "⇧⌘Z",
                disabled: editor.history.with_value(|h| h.redo_len() == 0),
            },
            D,
            item("Place image…", "placeImage", "⇧⌘K"),
            item("Design tokens", "tokens", ""),
            item("Add page", "addPage", ""),
            D,
            item("Toggle light / dark", "theme", ""),
            item(grid, "grid", ""),
            item(
                if editor.rulers.get_untracked() {
                    "Hide rulers"
                } else {
                    "Show rulers"
                },
                "rulers",
                "",
            ),
            item("Editor settings", "settings", ""),
            item("Keyboard shortcuts", "help", "?"),
        ],
        MenuKind::Selection => vec![
            item("Copy", "copy", "⌘C"),
            item("Paste", "paste", "⌘V"),
            item("Duplicate", "duplicate", "⌘D"),
            D,
            item("Group selection", "group", "⌘G"),
            item("Ungroup", "ungroup", "⇧⌘G"),
            item("Frame selection", "frameSelection", "⌥⌘G"),
            item("Create component", "component", "⌥⌘K"),
            D,
            item("Bring to front", "front", "]"),
            item("Send to back", "back", "["),
            item("Distribute horizontally", "distributeH", ""),
            item("Distribute vertically", "distributeV", ""),
            D,
            item("Rename", "rename", "⌘R"),
            item("Lock / unlock", "lock", "⇧⌘L"),
            item("Hide / show", "visibility", "⇧⌘H"),
            D,
            item("Export PNG", "exportPNG", ""),
            item("Export SVG", "exportSVG", ""),
            D,
            item("Delete", "delete", "⌫"),
        ],
        MenuKind::Zoom => vec![
            item("Zoom to fit", "fit", "⇧1"),
            item("Zoom to selection", "fitSelection", "⇧2"),
            item("Zoom to 100%", "actualSize", "⇧0"),
            item(grid, "grid", ""),
        ],
    }
}

struct OutsideClick(Closure<dyn FnMut(Event)>);
impl Drop for OutsideClick {
    fn drop(&mut self) {
        let _ = document()
            .remove_event_listener_with_callback("pointerdown", self.0.as_ref().unchecked_ref());
    }
}

#[component]
pub fn Menus(editor: Editor) -> impl IntoView {
    let listener = Closure::new(move |event: Event| {
        let inside = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .and_then(|element| element.closest("#context-menu,#main-menu,#zoom-value").ok())
            .flatten()
            .is_some();
        if !inside && editor.shell.with_untracked(|shell| shell.menu.is_some()) {
            close(editor);
        }
    });
    let options =
        rustify_makepad::listener_options().unwrap_or_else(web_sys::AddEventListenerOptions::new);
    let _ = document().add_event_listener_with_callback_and_add_event_listener_options(
        "pointerdown",
        listener.as_ref().unchecked_ref(),
        &options,
    );
    let listener = StoredValue::new_local(Some(OutsideClick(listener)));
    on_cleanup(move || {
        listener.update_value(|listener| {
            listener.take();
        })
    });
    let state = Memo::new(move |_| editor.shell.with(|shell| shell.menu.clone()));
    view! { {move || state.get().map(|state| view! { <Menu editor state/> })} }
}

#[component]
fn Menu(editor: Editor, state: MenuState) -> impl IntoView {
    let anchor_id = match state.kind {
        MenuKind::Main => "main-menu",
        MenuKind::Zoom => "zoom-value",
        MenuKind::Selection => "overlay",
    };
    let anchor = document()
        .get_element_by_id(anchor_id)
        .map(|element| Anchor::element(&element))
        .unwrap_or(Anchor::Centred);
    let anchor = Signal::derive(move || anchor.clone());
    let node = NodeRef::<leptos::html::Div>::new();
    let left = RwSignal::new(state.position.x);
    let top = RwSignal::new(state.position.y);
    let rows = StoredValue::new_local(entries(editor, state.kind));
    Effect::new(move || {
        editor.viewport.track();
        if let Some(node) = node.get() {
            let width = window()
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0);
            let height = window()
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(800.0);
            left.set(
                state
                    .position
                    .x
                    .min(width - f64::from(node.offset_width()) - 10.0),
            );
            top.set(
                state
                    .position
                    .y
                    .min(height - f64::from(node.offset_height()) - 10.0),
            );
        }
    });
    let focus = StoredValue::new_local(None::<AnimationFrameRequestHandle>);
    node.on_load(move |node| {
        // Layer records the trigger before this deferred menu focus.
        if let Ok(handle) = request_animation_frame_with_handle(move || {
            if let Ok(Some(first)) = node.query_selector("button:not([disabled])") {
                if let Some(first) = first.dyn_ref::<HtmlElement>() {
                    let _ = first.focus();
                }
            }
        }) {
            focus.set_value(Some(handle));
        }
    });
    on_cleanup(move || {
        focus.update_value(|focus| {
            if let Some(focus) = focus.take() {
                focus.cancel();
            }
        })
    });
    view! {
        <Layer anchor on_close=move || close(editor) class="vellum-menu-layer" test_id="vellum-menu-layer">
            <div id="context-menu" class="context-menu" role="menu" node_ref=node
                style:left=move || format!("{}px",left.get()) style:top=move || format!("{}px",top.get())
                on:keydown=move |event| menu_key(node,event)>
                {rows.with_value(|rows| rows.iter().map(|entry| match entry {
                    Entry::Header(label) => view! { <div class="menu-header">{*label}</div> }.into_any(),
                    Entry::Divider => view! { <div class="menu-divider" role="separator"></div> }.into_any(),
                    Entry::Item {label,action,key,disabled} => {
                        let action = *action;
                        view! { <button class="menu-item" role="menuitem" data-menu-action=action disabled=*disabled
                            on:click=move |_| { close(editor); super::dispatch(editor,action); }>
                            <span>{label.clone()}</span><span class="shortcut">{*key}</span>
                        </button> }.into_any()
                    }
                }).collect_view())}
            </div>
        </Layer>
    }
}

fn menu_key(node: NodeRef<leptos::html::Div>, event: KeyboardEvent) {
    if !matches!(
        event.key().as_str(),
        "ArrowDown" | "ArrowUp" | "Home" | "End"
    ) {
        return;
    }
    let Some(node) = node.get() else {
        return;
    };
    let Ok(nodes) = node.query_selector_all("button:not([disabled])") else {
        return;
    };
    let buttons: Vec<_> = (0..nodes.length())
        .filter_map(|index| nodes.item(index)?.dyn_into::<HtmlElement>().ok())
        .collect();
    if buttons.is_empty() {
        return;
    }
    event.prevent_default();
    event.stop_propagation();
    let current = document()
        .active_element()
        .and_then(|active| buttons.iter().position(|button| active == *button.as_ref()));
    let next = match event.key().as_str() {
        "Home" => 0,
        "End" => buttons.len() - 1,
        "ArrowUp" => current.map_or(buttons.len() - 1, |index| {
            (index + buttons.len() - 1) % buttons.len()
        }),
        _ => current.map_or(0, |index| (index + 1) % buttons.len()),
    };
    let _ = buttons[next].focus();
}
