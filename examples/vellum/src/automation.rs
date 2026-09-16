use leptos::prelude::*;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

use crate::app::{current, js_error};

#[wasm_bindgen]
pub fn vellum_mount(container_id: &str) -> Result<u32, JsValue> {
    crate::app::mount(container_id)
}
#[wasm_bindgen]
pub fn vellum_dispose(handle: u32) -> bool {
    crate::app::dispose(handle)
}
#[wasm_bindgen]
pub fn vellum_identify(runtime: u32, build: &str) {
    rustify_ui::identify_runtime(runtime, build);
}
#[wasm_bindgen]
pub fn vellum_diagnostics() -> String {
    rustify_ui::report_json()
}
#[wasm_bindgen]
pub fn vellum_live_regions() -> u32 {
    rustify_makepad::live_region_count() as u32
}
#[wasm_bindgen]
pub fn vellum_snapshot() -> Result<String, JsValue> {
    Ok(current()?.snapshot().to_string())
}
#[wasm_bindgen]
pub fn vellum_serialize() -> Result<String, JsValue> {
    current()?
        .doc
        .with_untracked(|doc| doc.serialize())
        .map_err(js_error)
}
#[wasm_bindgen]
pub fn vellum_world(id: &str) -> Result<String, JsValue> {
    let editor = current()?;
    let world = editor.doc.with_untracked(|doc| {
        let frame = crate::scene::compose(doc);
        frame.world(id).map(|item|json!({"node":item.node,"matrix":item.matrix.0,"inverse":item.inverse.0,"box":{"x":item.bounds.x,"y":item.bounds.y,"w":item.bounds.w,"h":item.bounds.h},"opacity":item.opacity,"hidden":item.hidden,"locked":item.locked}))
    });
    Ok(world.unwrap_or(Value::Null).to_string())
}

#[wasm_bindgen]
pub fn vellum_text_layout(id: &str) -> Result<String, JsValue> {
    use crate::text_layout::Measure;

    let editor = current()?;
    editor.doc.with_untracked(|doc| {
        let node = doc
            .get(id)
            .ok_or_else(|| JsValue::from_str("Text layer not found"))?;
        editor.measure.with_value(|measure| {
            let layout = crate::text_layout::layout_text(node, measure);
            let widths: Vec<_> = layout
                .lines
                .iter()
                .map(|line| measure.width(node, line))
                .collect();
            Ok(json!({
                "displayText":measure.display_text(node),
                "fontSpec":crate::text_layout::font_spec(node),
                "lines":layout.lines,
                "lineHeight":layout.line_height,
                "height":layout.height,
                "baseline":layout.baseline,
                "widths":widths
            })
            .to_string())
        })
    })
}

#[wasm_bindgen]
pub fn vellum_raster_stats() -> Result<String, JsValue> {
    Ok(current()?
        .rasters
        .with_value(|cache| cache.stats())
        .to_string())
}

/// Injects a controlled value change without finishing the active text transaction.
/// This diagnostic exercises TextEdit invalidation, rather than normal editor commands.
#[wasm_bindgen]
pub fn vellum_replace_text_externally(payload: &str) -> Result<(), JsValue> {
    let value: Value = serde_json::from_str(payload).map_err(js_error)?;
    let id = value["id"]
        .as_str()
        .ok_or_else(|| JsValue::from_str("Missing text layer id"))?;
    let text = value["text"]
        .as_str()
        .ok_or_else(|| JsValue::from_str("Missing external text value"))?;
    current()?
        .doc
        .try_update(|doc| {
            let node = doc
                .get_mut(id)
                .filter(|node| node.kind == "text")
                .ok_or_else(|| JsValue::from_str("Text layer not found"))?;
            node.text = text.into();
            doc.touch(Some(id));
            Ok(())
        })
        .ok_or_else(|| JsValue::from_str("Vellum is no longer mounted"))?
}

#[wasm_bindgen]
pub async fn vellum_async_command(name: String, payload: String) -> Result<String, JsValue> {
    let editor = current()?;
    let value: Value = serde_json::from_str(&payload).map_err(js_error)?;
    let required = |key: &str| {
        value[key]
            .as_str()
            .ok_or_else(|| JsValue::from_str(&format!("Missing {key}")))
    };
    let result = match name.as_str() {
        "save" => {
            crate::storage::save(editor).await?;
            Value::Null
        }
        "importFont" => {
            editor
                .import_font(required("name")?, required("dataUrl")?)
                .await?
        }
        "loadStoredFonts" => editor.load_stored_fonts().await?,
        "fontReady" => editor.font_ready().await?,
        "setAsset" => {
            editor
                .set_asset(required("id")?, required("source")?)
                .await?
        }
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unknown asynchronous Vellum command: {name}"
            )))
        }
    };
    Ok(result.to_string())
}

#[wasm_bindgen]
pub async fn vellum_import_document(file: web_sys::File) -> Result<(), JsValue> {
    let editor = current()?;
    let result = crate::fileio::import_document(editor, file).await;
    crate::fileio::report(editor, result.clone());
    result
}

#[wasm_bindgen]
pub async fn vellum_import_image(file: web_sys::File, location: String) -> Result<(), JsValue> {
    let editor = current()?;
    let location: Value = serde_json::from_str(&location).map_err(js_error)?;
    let point = if location.is_null() {
        None
    } else {
        Some(crate::affine::Point::new(
            location["x"]
                .as_f64()
                .ok_or_else(|| JsValue::from_str("Image location requires x"))?,
            location["y"]
                .as_f64()
                .ok_or_else(|| JsValue::from_str("Image location requires y"))?,
        ))
    };
    let result = crate::fileio::import_image(editor, file, point).await;
    crate::fileio::report(editor, result.clone());
    result
}

#[wasm_bindgen]
pub async fn vellum_import_font_file(file: web_sys::File) -> Result<(), JsValue> {
    let editor = current()?;
    let result = crate::fileio::import_font(editor, file).await;
    crate::fileio::report(editor, result.clone());
    result
}

#[wasm_bindgen]
pub async fn vellum_export_canvas(
    selection: String,
    scale: f64,
) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let ids: Vec<String> = serde_json::from_str(&selection).map_err(js_error)?;
    crate::fileio::export_canvas(current()?, &ids, scale).await
}

#[wasm_bindgen]
pub async fn vellum_do_export(
    format: String,
    scale: f64,
    selection: String,
) -> Result<(), JsValue> {
    let editor = current()?;
    let ids: Option<Vec<String>> = serde_json::from_str(&selection).map_err(js_error)?;
    let result = crate::fileio::do_export(editor, &format, scale, ids).await;
    crate::fileio::report(editor, result.clone());
    result
}

fn ids(value: &Value) -> Result<Vec<String>, JsValue> {
    serde_json::from_value(value.clone()).map_err(js_error)
}

#[wasm_bindgen]
pub fn vellum_command(name: &str, payload: &str) -> Result<String, JsValue> {
    let editor = current()?;
    let value: Value = serde_json::from_str(payload).map_err(js_error)?;
    let selected = || editor.selection.get_untracked();
    match name {
        "select" => editor.select(ids(&value)?),
        "fit" => {
            let selection = if value.is_null() {
                None
            } else {
                Some(ids(&value)?)
            };
            editor.fit(selection.as_deref());
        }
        "zoomAt" => editor.zoom_at(
            value["factor"].as_f64().unwrap_or(1.0),
            value["x"].as_f64(),
            value["y"].as_f64(),
        ),
        "setTool" => crate::pointer::set_tool(editor, value.as_str().unwrap_or("select")),
        "setProperty" => editor.set_property(
            &selected(),
            value["prop"].as_str().unwrap_or(""),
            value["value"].clone(),
        )?,
        "createAtCenter" => {
            return editor
                .create_at_center(
                    value["type"].as_str().unwrap_or("rect"),
                    value["props"].clone(),
                )
                .map(|node| node.to_string())
        }
        "transaction" => {
            editor.edit(value["label"].as_str().unwrap_or("Edit"), |doc| {
                for patch in value["patches"].as_array().into_iter().flatten() {
                    let Some(id) = patch["id"].as_str() else {
                        continue;
                    };
                    let Some(node) = doc.get_mut(id) else {
                        continue;
                    };
                    let mut properties = serde_json::to_value(&*node).map_err(js_error)?;
                    let prop = patch["prop"]
                        .as_str()
                        .ok_or_else(|| JsValue::from_str("Patch property is missing"))?;
                    properties[prop] = patch["value"].clone();
                    *node = serde_json::from_value(properties).map_err(js_error)?;
                    doc.touch(Some(id));
                }
                Ok(())
            })?;
        }
        "switchPage" => editor.switch_page(value.as_str().unwrap_or("")),
        "action" => crate::shell::dispatch(editor, value["name"].as_str().unwrap_or("")),
        "setTheme" => {
            editor.dark.set(value.as_str() != Some("light"));
            editor.shell.update(|shell| shell.canvas_color = None);
        }
        "insertAsset" => crate::shell::assets::insert(editor, value.as_str().unwrap_or("card")),
        "instantiate" => crate::shell::assets::instantiate(editor, value.as_str().unwrap_or("")),
        "resetStarter" | "stressTest" | "stress" | "theme" | "grid" | "snap" | "rulers"
        | "blankBadges" | "actualSize" | "fitSelection" | "undo" | "redo" | "delete"
        | "duplicate" | "group" | "frameSelection" | "ungroup" | "newFile" | "addPage" => {
            crate::shell::dispatch(editor, name)
        }
        "render" => editor.doc.update(|doc| doc.touch(None)),
        "parse" => {
            return crate::document::Document::parse(value.as_str().unwrap_or(""))
                .and_then(|doc| doc.serialize().map_err(Into::into))
                .map_err(js_error)
        }
        "exportSVG" => {
            let ids = if value.is_null() {
                None
            } else {
                Some(ids(&value)?)
            };
            return crate::fileio::export_svg(editor, ids).map(|svg| json!(svg).to_string());
        }
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unknown Vellum command: {name}"
            )))
        }
    }
    Ok("null".into())
}
