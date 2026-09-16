//! Canvas shortcuts are scoped listeners; form controls keep their native editing keys.

use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{HtmlElement, KeyboardEvent};

use crate::{app::Editor, commands::Order, pointer, shell};

pub struct Bindings {
    document: web_sys::Document,
    down: Closure<dyn FnMut(KeyboardEvent)>,
    up: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Drop for Bindings {
    fn drop(&mut self) {
        let _ = self
            .document
            .remove_event_listener_with_callback("keydown", self.down.as_ref().unchecked_ref());
        let _ = self
            .document
            .remove_event_listener_with_callback("keyup", self.up.as_ref().unchecked_ref());
    }
}

pub fn install(editor: Editor) -> Result<Bindings, JsValue> {
    let down = Closure::new(move |event: KeyboardEvent| {
        if let Err(error) = key_down(editor, &event) {
            editor
                .error
                .set(Some(format!("Shortcut failed: {error:?}")));
        }
    });
    let up = Closure::new(move |event: KeyboardEvent| {
        if event.code() == "Space" {
            let hand = editor.tool.get_untracked() == "hand";
            editor.input.update(|input| {
                input.space = false;
                input.cursor = if hand { "grab" } else { "default" }.into();
            });
        }
    });
    let bindings = Bindings {
        document: document(),
        down,
        up,
    };
    let options =
        rustify_makepad::listener_options().unwrap_or_else(web_sys::AddEventListenerOptions::new);
    bindings
        .document
        .add_event_listener_with_callback_and_add_event_listener_options(
            "keydown",
            bindings.down.as_ref().unchecked_ref(),
            &options,
        )?;
    bindings
        .document
        .add_event_listener_with_callback_and_add_event_listener_options(
            "keyup",
            bindings.up.as_ref().unchecked_ref(),
            &options,
        )?;
    Ok(bindings)
}

fn key_down(editor: Editor, event: &KeyboardEvent) -> Result<(), JsValue> {
    if event.is_composing() || event.default_prevented() {
        return Ok(());
    }
    if shell::presentation::is_open(editor) {
        match event.key().as_str() {
            "Escape" => shell::presentation::close(editor),
            "ArrowRight" => shell::presentation::step(editor, 1),
            "ArrowLeft" => shell::presentation::step(editor, -1),
            _ => return Ok(()),
        }
        event.prevent_default();
        return Ok(());
    }
    if editor.input.with_untracked(|input| input.modal_open) {
        return Ok(());
    }
    let command = event.meta_key() || event.ctrl_key();
    let key = event.key();
    let lowercase = key.to_lowercase();
    let is_input = event
        .target()
        .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        .is_some_and(|element| {
            element.is_content_editable()
                || matches!(element.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
        });
    if is_input {
        if command
            && key == "Enter"
            && event
                .target()
                .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                .is_some_and(|element| element.class_list().contains("vellum-text-session"))
        {
            event.prevent_default();
            event.stop_propagation();
            shell::text_session::finish(editor);
            return Ok(());
        }
        if command && lowercase == "s" {
            event.prevent_default();
            shell::dispatch(editor, "saveFile");
        }
        return Ok(());
    }
    if key == "Escape" {
        event.prevent_default();
        shell::menus::close(editor);
        pointer::escape(editor);
        return Ok(());
    }
    if command && matches!(lowercase.as_str(), "z" | "y") {
        event.prevent_default();
        editor.restore_history(lowercase == "y" || event.shift_key());
        return Ok(());
    }
    if command {
        let action = match lowercase.as_str() {
            "c" => Some("copy"),
            "v" => Some("paste"),
            "x" => Some("cut"),
            "d" => Some("duplicate"),
            "a" => Some("selectAll"),
            "g" => Some(if event.shift_key() {
                "ungroup"
            } else if event.alt_key() {
                "frameSelection"
            } else {
                "group"
            }),
            "k" => Some(if event.alt_key() {
                "component"
            } else if event.shift_key() {
                "placeImage"
            } else {
                "commands"
            }),
            "s" => Some("saveFile"),
            "o" => Some("openFile"),
            "n" => Some("newFile"),
            "r" => Some(if editor.selection.with_untracked(Vec::is_empty) {
                "renameFile"
            } else {
                "rename"
            }),
            "l" if event.shift_key() => Some("lock"),
            "h" if event.shift_key() => Some("visibility"),
            _ => None,
        };
        if let Some(action) = action {
            event.prevent_default();
            shell::dispatch(editor, action);
            return Ok(());
        }
        if matches!(lowercase.as_str(), "b" | "i" | "u") {
            event.prevent_default();
            if let Some(node) = shell::active(editor).filter(|node| node.kind == "text") {
                let (prop, value) = match lowercase.as_str() {
                    "b" => (
                        "fontWeight",
                        serde_json::json!(if node.font_weight >= 700. { 400 } else { 700 }),
                    ),
                    "i" => (
                        "fontStyle",
                        serde_json::json!(if node.font_style == "italic" {
                            "normal"
                        } else {
                            "italic"
                        }),
                    ),
                    _ => (
                        "textDecoration",
                        serde_json::json!(if node.text_decoration == "underline" {
                            "none"
                        } else {
                            "underline"
                        }),
                    ),
                };
                shell::set_property(editor, prop, value, false);
            }
            return Ok(());
        }
        if matches!(lowercase.as_str(), "l" | "h") {
            event.prevent_default();
        }
    }
    if event.code() == "Space" {
        event.prevent_default();
        editor.input.update(|input| {
            input.space = true;
            input.cursor = "grab".into();
        });
        return Ok(());
    }
    if event.shift_key() {
        match event.code().as_str() {
            "Digit1" => {
                event.prevent_default();
                editor.fit(None);
                return Ok(());
            }
            "Digit2" => {
                event.prevent_default();
                editor.fit(Some(&editor.selection.get_untracked()));
                return Ok(());
            }
            "Digit0" => {
                event.prevent_default();
                editor.zoom_at(1.0 / editor.camera.get_untracked().zoom, None, None);
                return Ok(());
            }
            _ => {}
        }
    }
    match key.as_str() {
        "Tab" => {
            event.prevent_default();
            editor
                .shell
                .update(|shell| shell.panels_hidden = !shell.panels_hidden);
        }
        "F2" => {
            event.prevent_default();
            shell::dispatch(editor, "rename");
        }
        "?" | "/" => {
            event.prevent_default();
            shell::dispatch(editor, "help");
        }
        "Delete" | "Backspace" => {
            event.prevent_default();
            editor.delete_selection()?;
        }
        "Enter" => {
            if editor.input.with_untracked(|input| !input.pen.is_empty()) {
                event.prevent_default();
                pointer::finish_pen(editor);
            } else if shell::active(editor).is_some_and(|node| node.kind == "text") {
                // Opening focuses/selects the textarea synchronously; consume the opening key.
                event.prevent_default();
                shell::dispatch(editor, "editText");
            }
        }
        "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => {
            event.prevent_default();
            let distance = if event.shift_key() { 10.0 } else { 1.0 };
            let (dx, dy) = match key.as_str() {
                "ArrowLeft" => (-distance, 0.0),
                "ArrowRight" => (distance, 0.0),
                "ArrowUp" => (0.0, -distance),
                _ => (0.0, distance),
            };
            editor.nudge(dx, dy)?;
        }
        "[" | "]" => {
            event.prevent_default();
            let order = match (key.as_str(), event.shift_key()) {
                ("[", true) => Order::Back,
                ("[", false) => Order::Backward,
                (_, true) => Order::Front,
                (_, false) => Order::Forward,
            };
            editor.reorder(order)?;
        }
        "+" | "=" if !command => editor.zoom_at(1.25, None, None),
        "-" if !command => editor.zoom_at(0.8, None, None),
        _ if !command && !event.alt_key() => {
            let tool = match lowercase.as_str() {
                "v" => Some("select"),
                "f" => Some("frame"),
                "r" => Some("rect"),
                "o" => Some("ellipse"),
                "l" => Some("line"),
                "p" => Some("pen"),
                "t" => Some("text"),
                "h" => Some("hand"),
                _ => None,
            };
            if let Some(tool) = tool {
                event.prevent_default();
                pointer::set_tool(editor, tool);
            }
        }
        _ => {}
    }
    Ok(())
}
