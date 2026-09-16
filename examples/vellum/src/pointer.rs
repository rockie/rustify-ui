//! DOM pointer gestures share a plain snapshot with the overlay and keyboard bindings.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use leptos::prelude::*;
use serde_json::{json, Value};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Event, EventTarget, HtmlCanvasElement, MouseEvent, PointerEvent, WheelEvent};

use crate::affine::{identity, point, Matrix, Point, Rect};
use crate::app::Editor;
use crate::document::Node;
use crate::gesture::{self, Camera, Modifiers, PathPoint};
use crate::hit::{self, Axis, Guide, Handle, HitOptions, PathHandle, SelectionGeometry};
use crate::scene::{compose, Frame};

/// Immutable drag originals are shared when a signal snapshot is read.
#[derive(Clone, Debug)]
pub struct TransformGesture {
    pub start: Point,
    pub geometry: SelectionGeometry,
    pub handle: Handle,
    pub originals: Arc<HashMap<String, Node>>,
    pub roots: Vec<String>,
    pub center: Point,
}

#[derive(Clone, Debug)]
pub enum Gesture {
    Pan {
        start: Point,
        camera: Camera,
    },
    PenHandle {
        index: usize,
        start: Point,
    },
    PathEdit {
        id: String,
        handle: PathHandle,
        start: Point,
        original: Arc<Node>,
    },
    Marquee {
        start: Point,
        old: Vec<String>,
        deep: bool,
    },
    Create {
        id: String,
        start: Point,
        local_start: Point,
        parent_inverse: Matrix,
        node_type: String,
    },
    Move {
        start: Point,
        originals: Arc<HashMap<String, Node>>,
        roots: Vec<String>,
        bounds: Option<Rect>,
        parent_matrices: Arc<HashMap<String, Matrix>>,
        moved: bool,
    },
    Resize(TransformGesture),
    Rotate(TransformGesture),
}

impl Gesture {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Pan { .. } => "pan",
            Self::PenHandle { .. } => "pen-handle",
            Self::PathEdit { .. } => "path-edit",
            Self::Marquee { .. } => "marquee",
            Self::Create { .. } => "create",
            Self::Move { .. } => "move",
            Self::Resize(_) => "resize",
            Self::Rotate(_) => "rotate",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pinch {
    pub distance: f64,
    pub camera: Camera,
    pub center: Point,
}

/// Requests handled by the workspace menus and native text session.
#[derive(Clone, Debug, PartialEq)]
pub enum InputRequest {
    CloseMenu,
    FinishText,
    StartText { id: String, select_all: bool },
    ContextMenu { client: Point },
    RevealSelection,
    Toast(String),
}

#[derive(Clone, Debug)]
pub struct InputState {
    pub gesture: Option<Gesture>,
    pub hover: Option<String>,
    pub space: bool,
    /// A vector preserves the original Map's insertion order for the pinch pair.
    pub pointers: Vec<(i32, Point)>,
    pub pinch: Option<Pinch>,
    pub guides: Vec<Guide>,
    pub marquee: Option<Rect>,
    pub pen: Vec<PathPoint>,
    pub pen_hover: Option<Point>,
    pub path_edit: Option<String>,
    pub active_path_point: Option<usize>,
    pub editing: Option<String>,
    pub modal_open: bool,
    pub cursor: String,
    pub tool_changed: bool,
    pub requests: Vec<InputRequest>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            gesture: None,
            hover: None,
            space: false,
            pointers: Vec::new(),
            pinch: None,
            guides: Vec::new(),
            marquee: None,
            pen: Vec::new(),
            pen_hover: None,
            path_edit: None,
            active_path_point: None,
            editing: None,
            modal_open: false,
            cursor: "default".into(),
            tool_changed: false,
            requests: Vec::new(),
        }
    }
}

impl InputState {
    /// The automation contract uses the reference editor's camelCase names.
    pub fn snapshot(&self) -> Value {
        json!({
            "gesture": self.gesture.as_ref().map(|gesture| json!({"kind": gesture.kind()})),
            "hover": self.hover,
            "space": self.space,
            "pointers": self.pointers,
            "pinch": self.pinch.map(|pinch| json!({
                "distance": pinch.distance, "camera": pinch.camera, "center": pinch.center,
            })),
            "guides": self.guides.iter().map(|guide| json!({
                "axis": match guide.axis { Axis::X => "x", Axis::Y => "y" },
                "value": guide.value,
            })).collect::<Vec<_>>(),
            "marquee": self.marquee,
            "pen": self.pen,
            "penHover": self.pen_hover,
            "pathEdit": self.path_edit,
            "activePathPoint": self.active_path_point,
            "editing": self.editing,
            "cursor": self.cursor,
        })
    }
}

struct Listener {
    target: EventTarget,
    name: &'static str,
    callback: Closure<dyn FnMut(Event)>,
}

/// Owns every canvas, area and window listener installed for this editor mount.
#[derive(Default)]
pub struct Bindings {
    listeners: Vec<Listener>,
}

impl Bindings {
    fn listen(
        &mut self,
        target: &EventTarget,
        name: &'static str,
        handler: impl FnMut(Event) + 'static,
    ) -> Result<(), JsValue> {
        let callback = Closure::wrap(Box::new(handler) as Box<dyn FnMut(Event)>);
        let options = rustify_makepad::listener_options()
            .unwrap_or_else(web_sys::AddEventListenerOptions::new);
        options.set_passive(false);
        target.add_event_listener_with_callback_and_add_event_listener_options(
            name,
            callback.as_ref().unchecked_ref(),
            &options,
        )?;
        self.listeners.push(Listener {
            target: target.clone(),
            name,
            callback,
        });
        Ok(())
    }
}

impl Drop for Bindings {
    fn drop(&mut self) {
        for listener in self.listeners.drain(..) {
            let _ = listener.target.remove_event_listener_with_callback(
                listener.name,
                listener.callback.as_ref().unchecked_ref(),
            );
        }
    }
}

/// Installs the reference pointer lifecycle. Dropping the result removes all listeners.
pub fn install(editor: Editor, canvas: HtmlCanvasElement) -> Result<Bindings, JsValue> {
    let mut bindings = Bindings::default();
    let target: EventTarget = canvas.clone().into();
    for name in ["pointerdown", "pointermove", "pointerup", "pointercancel"] {
        let canvas = canvas.clone();
        bindings.listen(&target, name, move |event| {
            if editor.input.is_disposed() {
                return;
            }
            let Ok(event) = event.dyn_into::<PointerEvent>() else {
                return;
            };
            match name {
                "pointerdown" => pointer_down(editor, &canvas, &event),
                "pointermove" => pointer_move(editor, &canvas, &event),
                "pointerup" => pointer_up(editor, &event),
                _ => {
                    editor.input.update(|input| {
                        input.pointers.retain(|(id, _)| *id != event.pointer_id());
                    });
                    cancel(editor);
                }
            }
        })?;
    }
    bindings.listen(&target, "pointerleave", move |_| {
        if !editor.input.is_disposed() {
            editor.input.update(|input| input.hover = None);
        }
    })?;
    for name in ["dblclick", "contextmenu"] {
        let canvas = canvas.clone();
        bindings.listen(&target, name, move |event| {
            if editor.input.is_disposed() {
                return;
            }
            let Ok(event) = event.dyn_into::<MouseEvent>() else {
                return;
            };
            if name == "dblclick" {
                double_click(editor, &canvas, &event);
            } else {
                context_menu(editor, &canvas, &event);
            }
        })?;
    }
    let area: EventTarget = canvas
        .parent_element()
        .map_or_else(|| target.clone(), Into::into);
    bindings.listen(&area, "wheel", move |event| {
        if editor.input.is_disposed() || editor.input.with_untracked(|input| input.modal_open) {
            return;
        }
        let Ok(event) = event.dyn_into::<WheelEvent>() else {
            return;
        };
        event.prevent_default();
        let cursor = event_point(&canvas, event.client_x(), event.client_y());
        editor.camera.update(|camera| {
            *camera = gesture::wheel(
                *camera,
                Point::new(event.delta_x(), event.delta_y()),
                cursor,
                Modifiers {
                    shift: event.shift_key(),
                    alt: event.alt_key(),
                    command: event.ctrl_key() || event.meta_key(),
                },
            );
        });
    })?;
    bindings.listen(&window().into(), "blur", move |_| {
        if !editor.input.is_disposed() {
            editor.input.update(|input| input.space = false);
            if editor.input.with_untracked(|input| input.gesture.is_some()) {
                editor.input.update(|input| input.pointers.clear());
                cancel(editor);
            }
        }
    })?;
    Ok(bindings)
}

fn event_point(canvas: &HtmlCanvasElement, x: impl Into<f64>, y: impl Into<f64>) -> Point {
    let bounds = canvas.get_bounding_client_rect();
    Point::new(x.into() - bounds.left(), y.into() - bounds.top())
}

fn modifiers(event: &PointerEvent) -> Modifiers {
    Modifiers {
        shift: event.shift_key(),
        alt: event.alt_key(),
        command: event.ctrl_key() || event.meta_key(),
    }
}

fn current_frame(editor: Editor) -> Arc<Frame> {
    let frame = editor.frame.get_untracked();
    editor.doc.with_untracked(|doc| {
        if frame.revision == doc.revision {
            frame
        } else {
            Arc::new(compose(doc))
        }
    })
}

fn selected_roots(editor: Editor) -> Vec<String> {
    let selected = editor.selection.get_untracked();
    editor.doc.with_untracked(|doc| {
        doc.roots(&selected)
            .into_iter()
            .map(|node| node.id.clone())
            .collect()
    })
}

fn selection_geometry(editor: Editor, frame: &Frame) -> Option<SelectionGeometry> {
    hit::selection_geometry(
        frame,
        &selected_roots(editor),
        &editor.selection.get_untracked(),
    )
}

fn hit_node(
    editor: Editor,
    frame: &Frame,
    world: Point,
    deep: bool,
    frames_only: bool,
) -> Option<Node> {
    let editing = editor.input.with_untracked(|input| input.editing.clone());
    editor.measure.with_value(|tester| {
        hit::hit_test(
            frame,
            world,
            HitOptions {
                deep,
                frames_only,
                zoom: editor.camera.get_untracked().zoom,
                editing: editing.as_deref(),
            },
            tester,
        )
        .cloned()
    })
}

fn begin(editor: Editor, label: &str) {
    crate::shell::text_session::finish(editor);
    editor.doc.with_untracked(|doc| {
        editor
            .history
            .update_value(|history| history.begin(doc, label));
    });
}

fn cancel_history(editor: Editor) {
    if editor.history.with_value(|history| history.is_pending()) {
        editor.doc.update(|doc| {
            editor.history.update_value(|history| history.cancel(doc));
        });
        // A canceled branch may have reused a version from an earlier raster.
        editor.reset_raster_caches();
    }
}

fn commit(editor: Editor, originals: Option<&HashMap<String, Node>>) {
    let mut error = None;
    editor.doc.update(|doc| {
        if let Some(originals) = originals {
            gesture::record_overrides(doc, originals);
        }
        doc.touch(None);
        crate::layout::apply_all_layouts(doc);
        match crate::commands::sync_components(doc) {
            Ok(()) => editor.history.update_value(|history| {
                history.commit(doc);
            }),
            Err(failure) => {
                error = Some(failure.to_string());
                editor.history.update_value(|history| history.cancel(doc));
            }
        }
    });
    if let Some(error) = error {
        editor.error.set(Some(error));
        editor.reset_raster_caches();
    }
}

fn request(editor: Editor, request: InputRequest) {
    editor.input.update(|input| {
        // Coalesce repeated notifications before the view consumes them.
        input.requests.retain(|previous| {
            std::mem::discriminant(previous) != std::mem::discriminant(&request)
        });
        input.requests.push(request);
    });
}

/// Drains menu and text session requests without retaining DOM objects in InputState.
pub fn take_requests(editor: Editor) -> Vec<InputRequest> {
    editor
        .input
        .try_update(|input| std::mem::take(&mut input.requests))
        .unwrap_or_default()
}

fn pointer_down(editor: Editor, canvas: &HtmlCanvasElement, event: &PointerEvent) {
    if event.button() == 2 {
        return;
    }
    request(editor, InputRequest::CloseMenu);
    if event.target().as_ref() != Some(canvas.as_ref()) {
        return;
    }
    let focus = web_sys::FocusOptions::new();
    focus.set_prevent_scroll(true);
    let _ = canvas.focus_with_options(&focus);
    let camera = editor.camera.get_untracked();
    let screen = event_point(canvas, event.client_x(), event.client_y());
    let world = camera.screen_to_world(screen);
    editor.input.update(|input| {
        if let Some((_, p)) = input
            .pointers
            .iter_mut()
            .find(|(id, _)| *id == event.pointer_id())
        {
            *p = screen;
        } else {
            input.pointers.push((event.pointer_id(), screen));
        }
    });
    let _ = canvas.set_pointer_capture(event.pointer_id());
    let input = editor.input.get_untracked();
    if event.pointer_type() == "touch" && input.pointers.len() == 2 {
        let canceled = input.gesture.is_some() && editor.history.with_value(|h| h.is_pending());
        if canceled {
            cancel_history(editor);
        }
        let (a, b) = (input.pointers[0].1, input.pointers[1].1);
        editor.input.update(|input| {
            if canceled {
                input.gesture = None;
            }
            input.pinch = Some(Pinch {
                distance: a.distance(b),
                camera,
                center: Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0),
            });
        });
        return;
    }
    if input.editing.is_some() {
        crate::shell::text_session::finish(editor);
    }
    let tool = editor.tool.get_untracked();
    if event.button() == 1 || input.space || tool == "hand" {
        editor.input.update(|input| {
            input.gesture = Some(Gesture::Pan {
                start: screen,
                camera,
            });
            input.cursor = "grabbing".into();
        });
        return;
    }
    let frame = current_frame(editor);
    if let Some(id) = input.path_edit.as_deref() {
        let path = frame.world(id).and_then(|scene| {
            hit::path_handle_at(&scene.node, scene.matrix, camera, screen)
                .map(|handle| (handle, scene.node.clone()))
        });
        if let Some((handle, original)) = path {
            begin(editor, "Edit vector point");
            editor.input.update(|input| {
                input.gesture = Some(Gesture::PathEdit {
                    id: id.into(),
                    handle,
                    start: world,
                    original: Arc::new(original),
                });
            });
            return;
        }
        editor.input.update(|input| input.path_edit = None);
    }
    if tool == "pen" {
        if input.pen.first().is_some_and(|first| {
            gesture::pen_can_close(
                Point::new(first.x, first.y),
                world,
                camera.zoom,
                input.pen.len(),
            )
        }) {
            finish_pen_closed(editor, true);
            return;
        }
        let previous = input.pen.last().map(|p| Point::new(p.x, p.y));
        let anchor = gesture::pen_anchor(previous, world, event.shift_key());
        editor.input.update(|input| {
            input.pen.push(anchor);
            input.gesture = Some(Gesture::PenHandle {
                index: input.pen.len() - 1,
                start: screen,
            });
        });
        return;
    }
    if tool == "text" {
        // The native editor mounts after pointerdown. A compatibility mousedown
        // would otherwise return focus to the canvas and commit its fresh draft.
        event.prevent_default();
        if let Some(existing) =
            hit_node(editor, &frame, world, true, false).filter(|n| n.kind == "text")
        {
            editor.select(vec![existing.id.clone()]);
            request(
                editor,
                InputRequest::StartText {
                    id: existing.id,
                    select_all: false,
                },
            );
            return;
        }
        let parent = hit_node(editor, &frame, world, false, true);
        let local = parent
            .as_ref()
            .and_then(|n| frame.world(&n.id))
            .map_or(world, |scene| point(scene.inverse, world.x, world.y));
        begin(editor, "Create text");
        let mut node = Node::new("text");
        node.x = local.x;
        node.y = local.y;
        node.w = 240.0;
        node.h = 44.0;
        node.text = "Type something".into();
        node.fill = if parent.is_some() || !editor.dark.get_untracked() {
            "#302937"
        } else {
            "#efe8f8"
        }
        .into();
        node.parent_id = parent.map(|node| node.id);
        let id = node.id.clone();
        editor.doc.update(|doc| {
            doc.add(node);
        });
        editor.select(vec![id.clone()]);
        editor.tool.set("select".into());
        request(
            editor,
            InputRequest::StartText {
                id,
                select_all: true,
            },
        );
        return;
    }
    if ["rect", "ellipse", "frame", "line"].contains(&tool.as_str()) {
        begin(editor, &format!("Draw {tool}"));
        let parent = if tool == "frame" {
            None
        } else {
            hit_node(editor, &frame, world, false, true)
        };
        let parent_inverse = parent
            .as_ref()
            .and_then(|n| frame.world(&n.id))
            .map_or_else(identity, |scene| scene.inverse);
        let local_start = point(parent_inverse, world.x, world.y);
        let mut node = Node::new(&tool);
        node.x = local_start.x;
        node.y = local_start.y;
        node.w = 0.1;
        node.h = 0.1;
        node.parent_id = parent.map(|node| node.id);
        node.name = match tool.as_str() {
            "rect" => "Rect",
            "ellipse" => "Ellipse",
            "frame" => "Frame",
            _ => "Line",
        }
        .into();
        node.fill = match tool.as_str() {
            "frame" => "#ffffff",
            "line" => "none",
            _ => "#b8a2e2",
        }
        .into();
        node.stroke = if tool == "line" { "#a18ad0" } else { "#000000" }.into();
        node.stroke_width = if tool == "line" { 2.0 } else { 0.0 };
        node.radius = if tool == "rect" { 8.0 } else { 0.0 };
        let id = node.id.clone();
        editor.doc.update(|doc| {
            doc.add(node);
        });
        editor.selection.set(vec![id.clone()]);
        editor.input.update(|input| {
            input.gesture = Some(Gesture::Create {
                id,
                start: world,
                local_start,
                parent_inverse,
                node_type: tool,
            });
        });
        return;
    }
    if let Some(geometry) = selection_geometry(editor, &frame) {
        if let Some(handle) = hit::handle_at(&hit::handles_for(geometry, camera), screen) {
            let handle = handle.name;
            begin(
                editor,
                if handle == Handle::Rotate {
                    "Rotate layers"
                } else {
                    "Resize layers"
                },
            );
            let roots = selected_roots(editor);
            let originals = editor
                .doc
                .with_untracked(|doc| Arc::new(gesture::save_originals(doc, &roots)));
            let transform = TransformGesture {
                start: world,
                geometry,
                handle,
                originals,
                roots,
                center: point(geometry.matrix, geometry.w / 2.0, geometry.h / 2.0),
            };
            editor.input.update(|input| {
                input.gesture = Some(if handle == Handle::Rotate {
                    Gesture::Rotate(transform)
                } else {
                    Gesture::Resize(transform)
                });
            });
            return;
        }
    }
    if let Some(hit) = hit_node(
        editor,
        &frame,
        world,
        event.ctrl_key() || event.meta_key(),
        false,
    ) {
        if event.shift_key() {
            editor.selection.update(|selection| {
                if selection.contains(&hit.id) {
                    selection.retain(|id| id != &hit.id);
                } else {
                    selection.push(hit.id.clone());
                }
            });
            if !editor
                .selection
                .with_untracked(|selection| selection.contains(&hit.id))
            {
                return;
            }
        } else if !editor
            .selection
            .with_untracked(|selection| selection.contains(&hit.id))
        {
            editor.select(vec![hit.id]);
        }
        begin(
            editor,
            if event.alt_key() {
                "Duplicate and move"
            } else {
                "Move layers"
            },
        );
        if event.alt_key() {
            let roots = selected_roots(editor);
            let clones = editor
                .doc
                .try_update(|doc| crate::commands::clone_nodes(doc, &roots, 0.0, false))
                .unwrap_or_default();
            editor.select(clones);
        }
        let frame = current_frame(editor);
        let roots: Vec<_> = selected_roots(editor)
            .into_iter()
            .filter(|id| frame.world(id).is_some_and(|scene| !scene.locked))
            .collect();
        let originals = editor
            .doc
            .with_untracked(|doc| Arc::new(gesture::save_originals(doc, &roots)));
        let parent_matrices = Arc::new(
            roots
                .iter()
                .filter_map(|id| {
                    frame.world(id).map(|scene| {
                        let matrix = scene
                            .node
                            .parent_id
                            .as_deref()
                            .and_then(|id| frame.world(id))
                            .map_or_else(identity, |parent| parent.inverse);
                        (id.clone(), matrix)
                    })
                })
                .collect(),
        );
        let bounds = frame.bounds(Some(&roots));
        editor.input.update(|input| {
            input.gesture = Some(Gesture::Move {
                start: world,
                originals,
                roots,
                bounds,
                parent_matrices,
                moved: false,
            });
        });
    } else {
        let old = if event.shift_key() {
            editor.selection.get_untracked()
        } else {
            Vec::new()
        };
        if !event.shift_key() {
            editor.select(Vec::new());
        }
        editor.input.update(|input| {
            input.gesture = Some(Gesture::Marquee {
                start: screen,
                old,
                deep: event.ctrl_key() || event.meta_key(),
            });
            input.marquee = Some(Rect::new(screen.x, screen.y, 0.0, 0.0));
        });
    }
}

fn pointer_move(editor: Editor, canvas: &HtmlCanvasElement, event: &PointerEvent) {
    let screen = event_point(canvas, event.client_x(), event.client_y());
    let camera = editor.camera.get_untracked();
    let world = camera.screen_to_world(screen);
    editor.input.update(|input| {
        if let Some((_, point)) = input
            .pointers
            .iter_mut()
            .find(|(id, _)| *id == event.pointer_id())
        {
            *point = screen;
        }
    });
    let input = editor.input.get_untracked();
    if let Some(pinch) = input.pinch.filter(|_| input.pointers.len() >= 2) {
        editor.camera.set(gesture::pinch(
            pinch.camera,
            pinch.center,
            pinch.distance,
            input.pointers[0].1,
            input.pointers[1].1,
        ));
        return;
    }
    let Some(active) = input.gesture else {
        let tool = editor.tool.get_untracked();
        if tool == "pen" {
            editor.input.update(|input| input.pen_hover = Some(world));
        } else if tool == "select" {
            let frame = current_frame(editor);
            let hover = hit_node(
                editor,
                &frame,
                world,
                event.ctrl_key() || event.meta_key(),
                false,
            )
            .map(|node| node.id);
            let handle = selection_geometry(editor, &frame).and_then(|geometry| {
                hit::handle_at(&hit::handles_for(geometry, camera), screen)
                    .map(|handle| handle.name)
            });
            let cursor = match handle {
                Some(Handle::Rotate) => "crosshair",
                Some(Handle::Nw | Handle::Se) => "nwse-resize",
                Some(Handle::N | Handle::S) => "ns-resize",
                Some(Handle::Ne | Handle::Sw) => "nesw-resize",
                Some(Handle::E | Handle::W) => "ew-resize",
                None if input.path_edit.is_some() => "crosshair",
                None if input.space => "grab",
                None => "default",
            };
            editor.input.update(|input| {
                input.hover = hover;
                input.cursor = cursor.into();
            });
        }
        return;
    };
    match active {
        Gesture::Pan { start, camera } => editor.camera.set(gesture::pan(camera, start, screen)),
        Gesture::PenHandle { index, start } => {
            editor.input.update(|input| {
                if input
                    .pen
                    .get_mut(index)
                    .is_some_and(|anchor| gesture::pen_handle(anchor, start, screen, world))
                {
                    input.pen_hover = None;
                }
            });
        }
        Gesture::PathEdit {
            id,
            handle,
            original,
            ..
        } => {
            editor.doc.update(|doc| {
                if let Some(node) = doc.get_mut(&id) {
                    gesture::edit_path(node, &original, handle, world, event.alt_key());
                    doc.touch(Some(&id));
                    doc.touch(None);
                }
            });
        }
        Gesture::Marquee { start, old, deep } => {
            let bounds = gesture::marquee_bounds(start, screen);
            let top_left = camera.screen_to_world(Point::new(bounds.x, bounds.y));
            let world_bounds = Rect::new(
                top_left.x,
                top_left.y,
                bounds.w / camera.zoom,
                bounds.h / camera.zoom,
            );
            let selected = hit::marquee(&current_frame(editor), world_bounds, deep, &old);
            editor.input.update(|input| input.marquee = Some(bounds));
            editor.selection.set(selected);
        }
        Gesture::Create {
            id,
            local_start,
            parent_inverse,
            ..
        } => {
            editor.doc.update(|doc| {
                if let Some(node) = doc.get_mut(&id) {
                    gesture::create(node, local_start, world, parent_inverse, modifiers(event));
                    doc.touch(Some(&id));
                    doc.touch(None);
                }
            });
        }
        Gesture::Move {
            start,
            originals,
            roots,
            bounds,
            parent_matrices,
            ..
        } => {
            let frame = current_frame(editor);
            let moved: HashSet<_> = originals.keys().cloned().collect();
            let snap = hit::smart_snap(
                &frame,
                hit::SnapRequest {
                    delta: gesture::move_delta(start, world, event.shift_key()),
                    bounds,
                    moved: &moved,
                    roots: &roots,
                    zoom: camera.zoom,
                    disabled: event.ctrl_key() || event.meta_key() || !editor.snap.get_untracked(),
                },
            );
            editor.input.update(|input| {
                input.guides = snap.guides;
                if let Some(Gesture::Move { moved, .. }) = &mut input.gesture {
                    *moved |= gesture::moved(snap.delta, camera.zoom);
                }
            });
            editor.doc.update(|doc| {
                for id in roots {
                    if let (Some(node), Some(original), Some(parent_inverse)) = (
                        doc.get_mut(&id),
                        originals.get(&id),
                        parent_matrices.get(&id),
                    ) {
                        gesture::move_node(node, original, *parent_inverse, snap.delta);
                        doc.touch(Some(&id));
                    }
                }
                doc.touch(None);
            });
        }
        Gesture::Resize(transform) => {
            editor.doc.update(|doc| {
                gesture::resize(
                    doc,
                    &transform.roots,
                    &transform.originals,
                    transform.geometry,
                    transform.handle,
                    world,
                    modifiers(event),
                );
                doc.touch(None);
            });
        }
        Gesture::Rotate(transform) => {
            editor.doc.update(|doc| {
                gesture::rotate(
                    doc,
                    &transform.roots,
                    &transform.originals,
                    transform.center,
                    transform.start,
                    world,
                    event.shift_key(),
                );
                doc.touch(None);
            });
        }
    }
}

fn pointer_up(editor: Editor, event: &PointerEvent) {
    editor
        .input
        .update(|input| input.pointers.retain(|(id, _)| *id != event.pointer_id()));
    if editor.input.with_untracked(|input| input.pinch.is_some()) {
        editor.input.update(|input| {
            if input.pointers.len() < 2 {
                input.pinch = None;
            }
            input.gesture = None;
        });
        return;
    }
    let active = editor
        .input
        .try_update(|input| {
            input.guides.clear();
            input.marquee = None;
            input.gesture.take()
        })
        .flatten();
    let Some(active) = active else {
        return;
    };
    match active {
        Gesture::Create { id, .. } => {
            editor.doc.update(|doc| {
                if let Some(node) = doc.get_mut(&id) {
                    if node.w < 3.0 && node.h < 3.0 {
                        gesture::finish_create(node);
                        doc.touch(Some(&id));
                    }
                }
            });
            set_tool(editor, "select");
            commit(editor, None);
        }
        Gesture::PathEdit { id, .. } => {
            editor.doc.update(|doc| {
                if let Some(node) = doc.get_mut(&id) {
                    gesture::normalize_path(node);
                    doc.touch(Some(&id));
                }
            });
            commit(editor, None);
        }
        Gesture::Move { originals, .. } => commit(editor, Some(&originals)),
        Gesture::Resize(transform) | Gesture::Rotate(transform) => {
            commit(editor, Some(&transform.originals))
        }
        _ => {}
    }
    reset_cursor(editor);
}

fn reset_cursor(editor: Editor) {
    let hand = editor.tool.get_untracked() == "hand";
    editor.input.update(|input| {
        input.cursor = if hand || input.space {
            "grab"
        } else {
            "default"
        }
        .into()
    });
}

/// Cancels a captured gesture and restores its pending document transaction.
pub fn cancel(editor: Editor) {
    cancel_history(editor);
    editor.input.update(|input| {
        input.gesture = None;
        input.pinch = None;
        input.guides.clear();
        input.marquee = None;
    });
    reset_cursor(editor);
}

/// Applies the reference Escape priority: active edit, unfinished pen, vector mode, selection.
pub fn escape(editor: Editor) {
    if editor.text_session.with_untracked(Option::is_some) {
        crate::shell::text_session::finish(editor);
        return;
    }
    let input = editor.input.get_untracked();
    if input.gesture.is_some() && editor.history.with_value(|history| history.is_pending()) {
        cancel_history(editor);
        editor.input.update(|input| {
            input.gesture = None;
            input.marquee = None;
            input.guides.clear();
        });
    } else if !input.pen.is_empty() {
        editor.input.update(|input| {
            input.pen.clear();
            input.pen_hover = None;
        });
    } else if input.path_edit.is_some() {
        editor.input.update(|input| input.path_edit = None);
    } else {
        editor.select(Vec::new());
    }
    set_tool(editor, "select");
}

/// Switches tools after finishing the active native text session and pen path.
pub fn set_tool(editor: Editor, tool: &str) {
    editor.input.update(|input| input.tool_changed = true);
    crate::shell::text_session::finish(editor);
    if editor.input.with_untracked(|input| !input.pen.is_empty()) {
        finish_pen(editor);
    }
    editor.tool.set(tool.into());
    editor.input.update(|input| {
        input.path_edit = None;
        input.cursor = match tool {
            "hand" => "grab",
            "text" => "text",
            "rect" | "frame" | "ellipse" | "line" | "pen" => "crosshair",
            _ => "default",
        }
        .into();
    });
}

/// Commits the pending pen vertices as an open vector path.
pub fn finish_pen(editor: Editor) {
    finish_pen_closed(editor, false);
}

fn finish_pen_closed(editor: Editor, closed: bool) {
    let points = editor
        .input
        .try_update(|input| {
            input.pen_hover = None;
            std::mem::take(&mut input.pen)
        })
        .unwrap_or_default();
    if points.len() < 2 {
        return;
    }
    crate::shell::text_session::finish(editor);
    let mut selected = None;
    let result = editor.edit("Draw vector path", |doc| {
        let mut node = Node::new("path");
        node.name = "Vector".into();
        node.x = 0.0;
        node.y = 0.0;
        node.w = 1.0;
        node.h = 1.0;
        node.extra.insert("pathW".into(), json!(1));
        node.extra.insert("pathH".into(), json!(1));
        node.extra.insert("points".into(), json!(points));
        node.extra.insert("closed".into(), json!(closed));
        node.fill = if closed { "#b8a2e2" } else { "none" }.into();
        node.stroke = "#9779c9".into();
        node.stroke_width = 2.0;
        let id = doc.add(node);
        if let Some(node) = doc.get_mut(&id) {
            gesture::normalize_path(node);
            doc.touch(Some(&id));
        }
        selected = Some(id);
        Ok(())
    });
    if let Err(error) = result {
        editor
            .error
            .set(Some(format!("Cannot finish path: {error:?}")));
    } else if let Some(id) = selected {
        editor.select(vec![id]);
    }
    set_tool(editor, "select");
}

fn double_click(editor: Editor, canvas: &HtmlCanvasElement, event: &MouseEvent) {
    let world = editor.camera.get_untracked().screen_to_world(event_point(
        canvas,
        event.client_x(),
        event.client_y(),
    ));
    let hit = hit_node(editor, &current_frame(editor), world, true, false);
    if editor.tool.get_untracked() == "pen" {
        finish_pen(editor);
        return;
    }
    if let Some(hit) = hit {
        editor.select(vec![hit.id.clone()]);
        match hit.kind.as_str() {
            "text" => request(
                editor,
                InputRequest::StartText {
                    id: hit.id,
                    select_all: true,
                },
            ),
            "path" => {
                editor.input.update(|input| {
                    input.path_edit = Some(hit.id);
                    input.active_path_point = None;
                });
                request(
                    editor,
                    InputRequest::Toast(
                        "Vector edit mode. Drag anchors or Bézier handles. Escape to finish."
                            .into(),
                    ),
                );
            }
            _ => request(editor, InputRequest::RevealSelection),
        }
    }
}

fn context_menu(editor: Editor, canvas: &HtmlCanvasElement, event: &MouseEvent) {
    event.prevent_default();
    let world = editor.camera.get_untracked().screen_to_world(event_point(
        canvas,
        event.client_x(),
        event.client_y(),
    ));
    if let Some(hit) = hit_node(editor, &current_frame(editor), world, false, false) {
        if !editor
            .selection
            .with_untracked(|selected| selected.contains(&hit.id))
        {
            editor.select(vec![hit.id]);
        }
    }
    request(
        editor,
        InputRequest::ContextMenu {
            client: Point::new(f64::from(event.client_x()), f64::from(event.client_y())),
        },
    );
}
