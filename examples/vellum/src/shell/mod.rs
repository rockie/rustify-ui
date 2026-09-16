use std::{collections::HashSet, rc::Rc};

use leptos::prelude::*;
use serde_json::{json, Value};
use wasm_bindgen::JsValue;

use crate::{
    affine::Point,
    app::{js_error, Editor},
    commands,
    document::Node,
    pointer::InputRequest,
};

pub mod assets;
pub mod clipboard;
pub mod dialogs;
pub mod fields;
pub mod inspector;
pub mod layer_tree;
pub mod left_panel;
pub mod menus;
pub mod pages;
pub mod palette;
pub mod presentation;
pub mod prototype;
pub mod text_session;
pub mod toast;
pub mod toolbar;
pub mod topbar;
pub mod zoom;

#[derive(Clone, Debug)]
pub struct ShellState {
    pub expanded: HashSet<String>,
    pub search: String,
    pub search_open: bool,
    pub left_tab: String,
    pub inspector_tab: String,
    pub left_hidden: bool,
    pub panels_hidden: bool,
    pub welcome: bool,
    pub canvas_color: Option<String>,
    pub export_scale: f64,
    pub export_format: String,
    pub toast: Option<String>,
    pub toast_serial: u64,
    pub menu: Option<menus::MenuState>,
    pub dialog: Option<dialogs::DialogState>,
    pub preview: Option<presentation::PreviewState>,
    pub clipboard: Option<Rc<clipboard::Payload>>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            expanded: HashSet::new(),
            search: String::new(),
            search_open: false,
            left_tab: "layers".into(),
            inspector_tab: "design".into(),
            left_hidden: false,
            panels_hidden: false,
            welcome: true,
            canvas_color: None,
            export_scale: 2.0,
            export_format: "PNG".into(),
            toast: None,
            toast_serial: 0,
            menu: None,
            dialog: None,
            preview: None,
            clipboard: None,
        }
    }
}

pub fn toast(editor: Editor, message: impl Into<String>) {
    toast::show(editor, &message.into());
}

pub fn report(editor: Editor, result: Result<(), JsValue>) {
    if let Err(error) = result {
        toast(
            editor,
            error.as_string().unwrap_or_else(|| format!("{error:?}")),
        );
    }
}

pub fn set_property(editor: Editor, prop: &str, value: Value, live: bool) {
    text_session::finish(editor);
    let ids = editor.selection.get_untracked();
    if ids.is_empty() {
        return;
    }
    let mut result = Ok(());
    editor.doc.update(|doc| {
        editor
            .history
            .update_value(|history| history.begin(doc, &format!("Change {prop}")));
        result = editor.measure.with_value(|measure| {
            commands::set_property(doc, &ids, prop, value, measure).map_err(js_error)
        });
        if result.is_ok() && !live {
            crate::layout::apply_all_layouts(doc);
            result = commands::sync_components(doc).map_err(js_error);
            if result.is_ok() {
                editor.history.update_value(|history| {
                    history.commit(doc);
                });
            }
        }
        if result.is_err() {
            editor.history.update_value(|history| history.cancel(doc));
        }
    });
    report(editor, result);
}

pub fn toggle_property(editor: Editor, prop: &str) {
    let first = active(editor);
    if let Some(node) = first {
        let value = match prop {
            "visible" => node.visible,
            "locked" => node.locked,
            "shadow" => node.shadow,
            _ => node.flag(prop),
        };
        set_property(editor, prop, json!(!value), false);
    }
}

pub fn active(editor: Editor) -> Option<Node> {
    let id = editor
        .selection
        .with_untracked(|ids| ids.first().cloned())?;
    editor.doc.with_untracked(|doc| doc.get(&id).cloned())
}

pub fn align(editor: Editor, mode: &str) {
    let alignment = match mode {
        "left" => commands::Align::Left,
        "center" => commands::Align::Center,
        "right" => commands::Align::Right,
        "top" => commands::Align::Top,
        "middle" => commands::Align::Middle,
        "bottom" => commands::Align::Bottom,
        _ => return,
    };
    let ids = editor.selection.get_untracked();
    if ids.is_empty() {
        return;
    }
    report(
        editor,
        editor.edit(&format!("Align {mode}"), |doc| {
            commands::align_selection(doc, &ids, alignment);
            Ok(())
        }),
    );
}

pub fn preset(editor: Editor, name: &str) {
    let (w, h) = match name {
        "desktop" => (1440, 900),
        "phone" => (390, 844),
        "tablet" => (834, 1194),
        "square" => (1080, 1080),
        _ => return,
    };
    let mut label = name.to_owned();
    label[0..1].make_ascii_uppercase();
    match editor.create_at_center("frame", json!({"name":label,"w":w,"h":h,"fill":"#ffffff"})) {
        Ok(node) => {
            if let Some(id) = node["id"].as_str() {
                editor.fit(Some(&[id.into()]));
            }
        }
        Err(error) => report(editor, Err(error)),
    }
}

pub fn reveal_selection(editor: Editor) {
    let ids = editor.selection.get_untracked();
    let ancestors = editor.doc.with_untracked(|doc| {
        ids.iter()
            .filter_map(|id| doc.get(id))
            .flat_map(|node| doc.ancestors(node))
            .map(|node| node.id.clone())
            .collect::<Vec<_>>()
    });
    editor
        .shell
        .update(|shell| shell.expanded.extend(ancestors));
}

/// Drain queued input requests before dispatching them outside the input signal borrow.
pub fn consume_pointer_requests(editor: Editor) {
    if editor
        .input
        .with_untracked(|input| input.requests.is_empty())
    {
        return;
    }
    let mut requests = Vec::new();
    editor.input.update(|input| {
        requests = std::mem::take(&mut input.requests);
    });
    for request in requests {
        match request {
            InputRequest::CloseMenu => menus::close(editor),
            InputRequest::ContextMenu { client } => menus::open_selection(editor, Some(client)),
            InputRequest::RevealSelection => reveal_selection(editor),
            InputRequest::Toast(message) => toast(editor, message),
            InputRequest::StartText { id, select_all } => {
                text_session::start(editor, &id, select_all)
            }
            InputRequest::FinishText => text_session::finish(editor),
        }
    }
}

pub fn dispatch(editor: Editor, action: &str) {
    report(editor, try_dispatch(editor, action));
}

fn try_dispatch(editor: Editor, action: &str) -> Result<(), JsValue> {
    if action != "editText" {
        text_session::finish(editor);
    }
    let selected = || editor.selection.get_untracked();
    match action {
        "undo" | "redo" => editor.restore_history(action == "redo"),
        "delete" => editor.delete_selection()?,
        "duplicate" => {
            if selected().is_empty() {
                return Ok(());
            }
            let mut ids = Vec::new();
            editor.edit("Duplicate layers", |doc| {
                ids = commands::clone_nodes(doc, &selected(), 20., false);
                Ok(())
            })?;
            editor.select(ids);
            toast(editor, "Selection duplicated");
        }
        "group" | "frameSelection" => {
            if selected().is_empty() {
                return Ok(());
            }
            let as_frame = action == "frameSelection";
            let mut id = None;
            editor.edit(
                if as_frame {
                    "Frame selection"
                } else {
                    "Group layers"
                },
                |doc| {
                    id = commands::group_selection(doc, &selected(), as_frame);
                    Ok(())
                },
            )?;
            if let Some(id) = &id {
                editor.shell.update(|shell| {
                    shell.expanded.insert(id.clone());
                });
            }
            editor.select(id.into_iter().collect());
        }
        "ungroup" => {
            let mut ids = Vec::new();
            editor.edit("Ungroup", |doc| {
                ids = commands::ungroup_selection(doc, &selected());
                Ok(())
            })?;
            editor.select(ids);
        }
        "component" => {
            if selected().is_empty() {
                return Ok(());
            }
            if selected().len() > 1 {
                try_dispatch(editor, "group")?;
            }
            let mut id = None;
            editor.edit("Create component", |doc| {
                id = commands::make_component(doc, &selected());
                Ok(())
            })?;
            editor.select(id.into_iter().collect());
            toast(editor, "Main component created. Find it in Assets.");
        }
        "front" | "back" | "forward" | "backward" => editor.reorder(match action {
            "front" => commands::Order::Front,
            "back" => commands::Order::Back,
            "forward" => commands::Order::Forward,
            _ => commands::Order::Backward,
        })?,
        "distributeH" | "distributeV" => {
            if editor
                .doc
                .with_untracked(|doc| doc.roots(&selected()).len())
                < 3
            {
                toast(editor, "Select at least three layers to distribute.");
                return Ok(());
            }
            editor.edit("Distribute spacing", |doc| {
                commands::distribute(doc, &selected(), action == "distributeH");
                Ok(())
            })?;
        }
        "copy" => clipboard::copy(editor)?,
        "paste" => clipboard::paste(editor),
        "cut" => {
            try_dispatch(editor, "copy")?;
            editor.delete_selection()?;
        }
        "selectAll" => editor.select(editor.doc.with_untracked(|doc| {
            doc.nodes()
                .iter()
                .filter(|n| {
                    n.parent_id.as_deref().is_none_or(str::is_empty) && n.visible && !n.locked
                })
                .map(|n| n.id.clone())
                .collect()
        })),
        "lock" => toggle_property(editor, "locked"),
        "visibility" => toggle_property(editor, "visible"),
        "fit" => editor.fit(None),
        "fitSelection" => editor.fit(Some(&selected())),
        "actualSize" => editor.zoom_at(1. / editor.camera.get_untracked().zoom, None, None),
        "theme" => {
            editor.dark.update(|dark| *dark = !*dark);
            editor.shell.update(|shell| shell.canvas_color = None);
        }
        "grid" => editor.grid.update(|value| *value = !*value),
        "rulers" => editor.rulers.update(|value| *value = !*value),
        "snap" => editor.snap.update(|value| *value = !*value),
        "blankBadges" => editor.blank_badges.set(true),
        "rename" => {
            if let Some(node) = active(editor) {
                dialogs::prompt(editor, dialogs::PromptKind::RenameLayer(node.id));
            }
        }
        "renameFile" => dialogs::prompt(editor, dialogs::PromptKind::RenameFile),
        "addPage" => dialogs::prompt(editor, dialogs::PromptKind::AddPage),
        "help" => dialogs::open(editor, dialogs::DialogState::Help),
        "settings" => dialogs::open(editor, dialogs::DialogState::Settings),
        "tokens" => dialogs::open(editor, dialogs::DialogState::Tokens),
        "export" => dialogs::open(editor, dialogs::DialogState::Export),
        "newFile" => dialogs::open(editor, dialogs::DialogState::NewFile),
        "confirmNew" => {
            editor.edit("New document", |doc| {
                let page = crate::document::Page::new("Page 1");
                doc.data.name = "Untitled design".into();
                doc.data.page_id = page.id.clone();
                doc.data.pages = vec![page];
                doc.refresh();
                Ok(())
            })?;
            editor.select(Vec::new());
            editor.page_views.update_value(|views| views.clear());
            editor.reset_raster_caches();
            editor.fit(None);
        }
        "inspectCSS" => {
            if let Some(node) = active(editor) {
                dialogs::open(editor, dialogs::DialogState::Css { id: node.id });
            }
        }
        "commands" | "commandPalette" => palette::open(editor),
        "selectionMenu" => menus::open_selection(editor, None),
        "toggleFill" => {
            if let Some(node) = active(editor) {
                set_property(
                    editor,
                    "fill",
                    json!(if node.fill == "none" {
                        "#b8a2e2"
                    } else {
                        "none"
                    }),
                    false,
                );
            }
        }
        "toggleStroke" => {
            if let Some(node) = active(editor) {
                set_property(
                    editor,
                    "strokeWidth",
                    json!(if node.stroke_width != 0. { 0 } else { 1 }),
                    false,
                );
            }
        }
        "toggleShadow" => toggle_property(editor, "shadow"),
        "toggleLayout" => {
            if let Some(node) = active(editor) {
                if matches!(node.kind.as_str(), "frame" | "group") {
                    set_property(
                        editor,
                        "layout",
                        json!(if node.string("layout").is_some_and(|v| v != "none") {
                            "none"
                        } else {
                            "horizontal"
                        }),
                        false,
                    );
                } else {
                    toast(editor, "Select a frame or group for auto layout.");
                }
            }
        }
        "editText" => {
            if let Some(node) = active(editor) {
                text_session::start(editor, &node.id, true);
            }
        }
        "saveFile" | "openFile" | "placeImage" | "loadFont" | "exportPNG" | "exportSVG"
        | "exportSelection" | "exportTokens" => crate::fileio::dispatch(editor, action),
        "present" => presentation::present(editor),
        "stressTest" | "stress" => {
            editor.stress_test()?;
            toast(
                editor,
                "5,000 editable GPU primitives. See Settings for measured render statistics.",
            );
        }
        "resetStarter" => editor.reset_starter()?,
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unknown editor action: {action}"
            )))
        }
    }
    Ok(())
}

pub fn center(editor: Editor) -> Point {
    let (w, h, _) = editor.viewport.get_untracked();
    editor
        .camera
        .get_untracked()
        .screen_to_world(Point::new(w / 2., h / 2.))
}
