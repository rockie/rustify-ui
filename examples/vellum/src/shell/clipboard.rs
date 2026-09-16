//! System clipboard access with an editor-local fallback for refused permissions.

use std::{collections::BTreeMap, collections::HashSet, rc::Rc};

use leptos::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use wasm_bindgen::JsValue;

use crate::{
    app::{js_error, Editor},
    commands,
    document::{Document, Node},
};

const MAX_CLIPBOARD_BYTES: usize = 80 * 1024 * 1024;

/// `fonts` extends the original format without changing its node/asset envelope.
#[derive(Clone, Debug, Serialize)]
pub struct Payload {
    #[serde(flatten)]
    pub layers: commands::Clipboard,
    pub fonts: BTreeMap<String, Rc<str>>,
}

pub fn copy(editor: Editor) -> Result<(), JsValue> {
    super::text_session::finish(editor);
    let ids = editor.selection.get_untracked();
    let payload = editor.doc.with_untracked(|doc| {
        commands::copy_selection(doc, &ids).map(|layers| Payload {
            layers,
            fonts: doc.data.fonts.clone(),
        })
    });
    let Some(payload) = payload else {
        return Ok(());
    };
    let count = payload.layers.root_ids.len();
    let text = serde_json::to_string(&payload).map_err(js_error)?;
    editor
        .shell
        .update(|shell| shell.clipboard = Some(Rc::new(payload)));
    rustify_ui::clipboard::copy(&text, move |result| {
        if editor.shell.is_disposed() {
            return;
        }
        match result {
            Ok(()) => super::toast(
                editor,
                format!("{count} layer{} copied", if count > 1 { "s" } else { "" }),
            ),
            Err(_) => super::toast(
                editor,
                "Clipboard blocked. Layers are available to paste inside Vellum.",
            ),
        }
    });
    Ok(())
}

pub fn paste(editor: Editor) {
    super::text_session::finish(editor);
    let fallback = editor.shell.with_untracked(|shell| shell.clipboard.clone());
    rustify_ui::clipboard::paste(move |result| {
        if editor.shell.is_disposed() {
            return;
        }
        let mut payload = fallback;
        if let Ok(text) = result {
            if text.len() > MAX_CLIPBOARD_BYTES {
                super::toast(editor, "Clipboard data is too large.");
                return;
            }
            if !text.is_empty() {
                let value = serde_json::from_str::<Value>(&text).ok();
                if let Some(value) = value.filter(|value| value["format"] == "vellum-clipboard") {
                    match parse_payload(&value) {
                        Ok(parsed) => payload = Some(Rc::new(parsed)),
                        Err(_) => {
                            super::toast(editor, "Clipboard data is not valid.");
                            return;
                        }
                    }
                } else if payload.is_none() {
                    super::report(editor, paste_text(editor, text));
                    return;
                }
            }
        }
        let Some(payload) = payload else {
            super::toast(editor, "Nothing copied yet. Copy a layer first.");
            return;
        };
        super::report(editor, paste_layers(editor, &payload));
    });
}

fn parse_payload(value: &Value) -> Result<Payload, JsValue> {
    let invalid = || JsValue::from_str("Clipboard data is not valid.");
    let root_ids: Vec<String> =
        serde_json::from_value(value["rootIds"].clone()).map_err(js_error)?;
    if value["format"] != "vellum-clipboard" || root_ids.is_empty() {
        return Err(invalid());
    }
    // Parse the raw nodes before deserializing their defaulted Rust fields, so missing
    // geometry, duplicate IDs, cycles, and unsupported assets fail the document rules.
    let temp = json!({
        "format":"vellum", "version":1, "name":"Clipboard", "pageId":"paste",
        "pages":[{"id":"paste","name":"Paste","nodes":value["nodes"]}],
        "assets":value.get("assets").cloned().unwrap_or_else(||json!({})),
        "fonts":value.get("fonts").cloned().unwrap_or_else(||json!({})),
    });
    let parsed = Document::parse(&temp.to_string()).map_err(js_error)?;
    let mut seen = HashSet::new();
    if root_ids.iter().any(|id| {
        !seen.insert(id)
            || parsed
                .get(id)
                .is_none_or(|node| node.parent_id.as_deref().is_some_and(|id| !id.is_empty()))
    }) || parsed.data.fonts.values().any(|source| {
        !source.starts_with("data:") || !source.contains(',') || source.len() > 45 * 1024 * 1024
    }) {
        return Err(invalid());
    }
    Ok(Payload {
        layers: commands::Clipboard {
            format: "vellum-clipboard".into(),
            root_ids,
            nodes: parsed.nodes().to_vec(),
            assets: parsed.data.assets.clone(),
        },
        fonts: parsed.data.fonts.clone(),
    })
}

fn paste_layers(editor: Editor, payload: &Payload) -> Result<(), JsValue> {
    let mut ids = Vec::new();
    editor.edit("Paste layers", |doc| {
        ids = commands::paste_selection(doc, &payload.layers).map_err(js_error)?;
        doc.data.fonts.extend(payload.fonts.clone());
        Ok(())
    })?;
    editor.select(ids);
    editor.reset_raster_caches();
    if !payload.fonts.is_empty() {
        leptos::task::spawn_local(async move {
            let result = editor.load_stored_fonts().await;
            if !editor.shell.is_disposed() {
                super::report(editor, result.map(|_| ()));
            }
        });
    }
    Ok(())
}

fn paste_text(editor: Editor, text: String) -> Result<(), JsValue> {
    if text.encode_utf16().count() > 100_000 {
        return Err(JsValue::from_str("Text layer is too large."));
    }
    let center = super::center(editor);
    let mut node = Node::new("text");
    node.name = String::from_utf16_lossy(&text.encode_utf16().take(28).collect::<Vec<_>>());
    node.text = text;
    node.w = 360.;
    node.h = 70.;
    node.font_size = 28.;
    node.fill = if editor.dark.get_untracked() {
        "#f0eaf8"
    } else {
        "#282431"
    }
    .into();
    node.x = center.x - node.w / 2.;
    node.y = center.y - node.h / 2.;
    node.h = editor
        .measure
        .with_value(|measure| crate::text_layout::layout_text(&node, measure).height);
    let id = node.id.clone();
    editor.edit("Insert text", |doc| {
        doc.add(node);
        doc.touch(Some(&id));
        Ok(())
    })?;
    editor.select(vec![id]);
    Ok(())
}
