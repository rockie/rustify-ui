//! Browser file adapters. Validation and document changes stay in Rust; the SDK owns I/O.

use std::{cell::RefCell, rc::Rc};

use js_sys::{Promise, Uint8Array};
use leptos::prelude::*;
use rustify_ui::files::{self, Import, Limits, Refusal};
use serde_json::json;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, DataTransfer, DragEvent, EventTarget, File, FontFace, HtmlCanvasElement, HtmlImageElement,
};

use crate::{
    affine::Point,
    app::{js_error, Editor},
    assets::ImageCache,
    document::{uid, Document, Node},
    painter::Painter,
    shell::{self, text_session},
};

const DOCUMENT_BYTES: u64 = 80 * 1024 * 1024;
const IMAGE_BYTES: u64 = 25 * 1024 * 1024;
const FONT_BYTES: u64 = 15 * 1024 * 1024;

#[derive(Clone, Copy)]
enum Kind {
    Document,
    Image,
    Font,
}

impl Kind {
    fn limits(self, picker: bool) -> Limits {
        Limits {
            max_bytes: match self {
                Self::Document => DOCUMENT_BYTES,
                Self::Image => IMAGE_BYTES,
                Self::Font => FONT_BYTES,
            },
            kinds: if picker {
                match self {
                    Self::Document => &[".vellum", ".json"],
                    Self::Image => &[
                        ".png", ".jpg", ".jpeg", ".webp", ".gif", ".svg", ".bmp", ".avif", ".ico",
                    ],
                    Self::Font => &[".ttf", ".otf", ".woff", ".woff2"],
                }
            } else {
                &[]
            },
        }
    }

    fn refusal(self, why: Refusal) -> JsValue {
        JsValue::from_str(match why {
            Refusal::TooLarge { .. } => match self {
                Self::Document => "File exceeds the 80 MB import limit.",
                Self::Image => "Please use an image under 25 MB.",
                Self::Font => "Font file exceeds 15 MB.",
            },
            Refusal::WrongKind { .. } => match self {
                Self::Document => "Choose a Vellum document or JSON file.",
                Self::Image => "Choose an image file.",
                Self::Font => "Choose a TTF, OTF, WOFF, or WOFF2 font.",
            },
            Refusal::Unreadable => "Could not read the file.",
        })
    }
}

/// A delayed picker/read/decode must not replace a newer document or page.
#[derive(Clone)]
struct ImportGuard {
    editor: Editor,
    revision: u64,
    page: String,
}

impl ImportGuard {
    fn new(editor: Editor) -> Result<Self, JsValue> {
        mounted(editor)?;
        let (revision, page) = editor
            .doc
            .with_untracked(|doc| (doc.revision, doc.data.page_id.clone()));
        Ok(Self {
            editor,
            revision,
            page,
        })
    }

    fn check(&self) -> Result<(), JsValue> {
        mounted(self.editor)?;
        if self
            .editor
            .doc
            .with_untracked(|doc| doc.revision == self.revision && doc.data.page_id == self.page)
        {
            Ok(())
        } else {
            Err(JsValue::from_str(
                "The document changed while the file was loading. Please try again.",
            ))
        }
    }
}

fn mounted(editor: Editor) -> Result<(), JsValue> {
    let failed = rustify_makepad::listener_options()
        .and_then(|options| options.get_signal())
        .is_some_and(|signal| signal.aborted());
    if editor.doc.is_disposed() || failed {
        Err(JsValue::from_str("Vellum is no longer mounted"))
    } else {
        Ok(())
    }
}

pub fn report(editor: Editor, result: Result<(), JsValue>) {
    if mounted(editor).is_ok() {
        shell::report(editor, result);
    }
}

async fn read_file(file: &File, kind: Kind) -> Result<Vec<u8>, JsValue> {
    kind.limits(false)
        .check(&file.name(), file.size() as u64)
        .map_err(|why| kind.refusal(why))?;
    let transfer = DataTransfer::new()?;
    transfer.items().add_with_file(file)?;
    let files = transfer.files();
    let answer = Rc::new(RefCell::new(None));
    let result = Rc::clone(&answer);
    let mut files = Some(files);
    let ready = Promise::new(&mut |resolve, _reject| {
        let result = Rc::clone(&result);
        files::import_files(files.take().flatten(), kind.limits(false), move |import| {
            *result.borrow_mut() = Some(import);
            let _ = resolve.call0(&JsValue::UNDEFINED);
        });
    });
    JsFuture::from(ready).await?;
    let result = answer.borrow_mut().take();
    match result {
        Some(Import::Loaded { bytes, .. }) => Ok(bytes),
        Some(Import::Refused { why, .. }) => Err(kind.refusal(why)),
        _ => Err(JsValue::from_str("No file was selected.")),
    }
}

pub async fn import_document(editor: Editor, file: File) -> Result<(), JsValue> {
    let guard = ImportGuard::new(editor)?;
    let bytes = read_file(&file, Kind::Document).await?;
    open_document(guard, &bytes).await
}

async fn open_document(guard: ImportGuard, bytes: &[u8]) -> Result<(), JsValue> {
    // File.text uses UTF-8 replacement; a leading UTF-8 BOM is stripped by the browser.
    let text = String::from_utf8_lossy(bytes);
    let parsed = Document::parse(text.trim_start_matches('\u{feff}')).map_err(js_error)?;
    guard.check()?;
    let editor = guard.editor;
    text_session::finish(editor);
    let name = parsed.data.name.clone();
    editor.doc.update(|doc| {
        editor
            .history
            .update_value(|history| history.begin(doc, "Open document"));
        doc.data = parsed.data;
        doc.refresh();
        editor.history.update_value(|history| {
            history.commit(doc);
        });
    });
    editor.selection.set(Vec::new());
    editor.shell.update(|state| state.expanded.clear());
    editor.page_views.update_value(|views| views.clear());
    editor.input.update(|input| {
        input.hover = None;
        input.path_edit = None;
        input.pen.clear();
    });
    editor.reset_raster_caches();
    let opened = ImportGuard::new(editor)?;
    editor.load_stored_fonts().await?;
    opened.check()?;
    editor.fit(None);
    shell::toast(editor, format!("Opened {name}"));
    Ok(())
}

pub async fn import_image(
    editor: Editor,
    file: File,
    location: Option<Point>,
) -> Result<(), JsValue> {
    let guard = ImportGuard::new(editor)?;
    Kind::Image
        .limits(false)
        .check(&file.name(), file.size() as u64)
        .map_err(|why| Kind::Image.refusal(why))?;
    let mime = file.type_();
    if !mime.starts_with("image/") {
        return Err(JsValue::from_str("Choose an image file."));
    }
    let bytes = read_file(&file, Kind::Image).await?;
    place_image(guard, &file.name(), &mime, &bytes, location).await
}

async fn place_image(
    guard: ImportGuard,
    name: &str,
    mime: &str,
    bytes: &[u8],
    location: Option<Point>,
) -> Result<(), JsValue> {
    let source: Rc<str> = data_url(mime, bytes).into();
    let image = HtmlImageElement::new()?;
    image.set_src(&source);
    JsFuture::from(image.decode())
        .await
        .map_err(|_| JsValue::from_str("Could not decode this image."))?;
    let width = f64::from(image.natural_width());
    let height = f64::from(image.natural_height());
    if width * height > 64e6 {
        return Err(JsValue::from_str("Image exceeds the 64 megapixel limit."));
    }
    if width == 0.0 || height == 0.0 {
        return Err(JsValue::from_str("Could not decode this image."));
    }
    let editor = guard.editor;
    mounted(editor)?;
    let center = location.unwrap_or_else(|| {
        let (w, h, _) = editor.viewport.get_untracked();
        editor
            .camera
            .get_untracked()
            .screen_to_world(Point::new(w / 2.0, h / 2.0))
    });
    let scale = 1.0_f64.min(800.0 / width).min(800.0 / height);
    let asset = uid();
    let mut node = Node::new("image");
    node.name = name.into();
    node.w = width * scale;
    node.h = height * scale;
    node.x = center.x - node.w / 2.0;
    node.y = center.y - node.h / 2.0;
    node.fill = "#ffffff".into();
    node.extra.insert("assetId".into(), json!(asset));
    let id = node.id.clone();
    editor.edit("Place image", |doc| {
        doc.data.assets.insert(asset.clone(), source);
        doc.add(node);
        Ok(())
    })?;
    editor.images.with_value(|images| {
        editor.doc.with_untracked(|doc| {
            images.sync(doc);
        });
        images.insert_decoded(asset, image);
    });
    editor.select(vec![id]);
    shell::toast(editor, "Image placed. Original resolution preserved.");
    Ok(())
}

pub async fn import_font(editor: Editor, file: File) -> Result<(), JsValue> {
    let guard = ImportGuard::new(editor)?;
    let bytes = read_file(&file, Kind::Font).await?;
    load_font(guard, &file.name(), &bytes).await
}

async fn load_font(guard: ImportGuard, name: &str, bytes: &[u8]) -> Result<(), JsValue> {
    let family = crate::fonts::family_name(name);
    if family.is_empty() {
        return Err(JsValue::from_str("Font family is empty"));
    }
    let face = FontFace::new_with_u8_array(&family, bytes)?;
    JsFuture::from(face.load()?).await.map_err(|error| {
        JsValue::from_str(&format!(
            "Could not load the font: {}",
            crate::fonts::error_text(&error)
        ))
    })?;
    let editor = guard.editor;
    mounted(editor)?;
    text_session::finish(editor);
    let source: Rc<str> = data_url("application/octet-stream", bytes).into();
    editor
        .fonts
        .with_value(|fonts| fonts.insert_decoded(&family, Rc::clone(&source), face))?;
    editor.doc.update(|doc| {
        doc.data.fonts.insert(family.clone(), source);
        doc.touch(None);
    });
    let selected = editor.selection.get_untracked();
    if editor.doc.with_untracked(|doc| {
        selected
            .iter()
            .any(|id| doc.get(id).is_some_and(|node| node.kind == "text"))
    }) {
        editor.set_property(&selected, "fontFamily", json!(family))?;
    }
    editor.reset_raster_caches();
    shell::toast(
        editor,
        format!("{family} loaded. Import only fonts you have permission to embed."),
    );
    Ok(())
}

fn pick(editor: Editor, kind: Kind) {
    let guard = match ImportGuard::new(editor) {
        Ok(guard) => guard,
        Err(error) => {
            report(editor, Err(error));
            return;
        }
    };
    files::pick(kind.limits(true), move |import| {
        leptos::task::spawn_local(async move {
            let result = match import {
                Import::Loaded { name, bytes } => match kind {
                    Kind::Document => open_document(guard, &bytes).await,
                    Kind::Image => place_image(guard, &name, image_mime(&name), &bytes, None).await,
                    Kind::Font => load_font(guard, &name, &bytes).await,
                },
                Import::Refused { why, .. } => Err(kind.refusal(why)),
                Import::Aborted => Ok(()),
            };
            report(editor, result);
        });
    });
}

fn image_mime(name: &str) -> &'static str {
    match name
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        _ => "image/png",
    }
}

fn data_url(mime: &str, bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut value = String::with_capacity(32 + bytes.len().div_ceil(3) * 4);
    value.push_str("data:");
    value.push_str(mime);
    value.push_str(";base64,");
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        value.push(ALPHABET[(a >> 2) as usize] as char);
        value.push(ALPHABET[((a & 3) << 4 | b >> 4) as usize] as char);
        value.push(if chunk.len() > 1 {
            ALPHABET[((b & 15) << 2 | c >> 6) as usize] as char
        } else {
            '='
        });
        value.push(if chunk.len() > 2 {
            ALPHABET[(c & 63) as usize] as char
        } else {
            '='
        });
    }
    value
}

fn safe_name(name: &str) -> String {
    let units: Vec<_> = name
        .chars()
        .map(|c| {
            if c <= '\u{1f}' || "<>:\"/\\|?*".contains(c) {
                '-'
            } else {
                c
            }
        })
        .collect::<String>()
        .encode_utf16()
        .take(120)
        .collect();
    if units.is_empty() {
        "Vellum".into()
    } else {
        String::from_utf16_lossy(&units)
    }
}

fn export_ids(editor: Editor, ids: Option<Vec<String>>) -> Vec<String> {
    let ids = ids.unwrap_or_else(|| editor.selection.get_untracked());
    if ids.is_empty() {
        editor.doc.with_untracked(|doc| {
            doc.nodes()
                .iter()
                .filter(|node| node.parent_id.as_deref().is_none_or(str::is_empty))
                .map(|node| node.id.clone())
                .collect()
        })
    } else {
        ids
    }
}

pub fn export_svg(editor: Editor, ids: Option<Vec<String>>) -> Result<String, JsValue> {
    mounted(editor)?;
    text_session::finish(editor);
    let ids = export_ids(editor, ids);
    editor
        .doc
        .with_untracked(|doc| {
            editor
                .measure
                .with_value(|measure| crate::svg_export::export_svg(doc, &ids, measure))
        })
        .map_err(js_error)
}

pub async fn export_canvas(
    editor: Editor,
    ids: &[String],
    scale: f64,
) -> Result<HtmlCanvasElement, JsValue> {
    mounted(editor)?;
    text_session::finish(editor);
    let document = editor.doc.get_untracked();
    let images = ImageCache::new(Rc::new(|| {}));
    let canvas = Painter::new()
        .export_canvas(&document, ids, scale, &images)
        .await?;
    mounted(editor)?;
    Ok(canvas)
}

pub fn save_file(editor: Editor) -> Result<(), JsValue> {
    mounted(editor)?;
    text_session::finish(editor);
    let (name, document) = editor
        .doc
        .with_untracked(|doc| (safe_name(&doc.data.name), doc.serialize()));
    files::export(
        &format!("{name}.vellum"),
        document.map_err(js_error)?.as_bytes(),
        "application/json",
    )?;
    shell::toast(
        editor,
        "Portable document exported. Includes all pages and images.",
    );
    Ok(())
}

async fn png_bytes(canvas: &HtmlCanvasElement) -> Result<Vec<u8>, JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let reject_callback = reject.clone();
        let callback = Closure::once_into_js(move |blob: Option<Blob>| {
            if let Some(blob) = blob {
                let _ = resolve.call1(&JsValue::UNDEFINED, &blob);
            } else {
                let _ = reject_callback.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str("Image encoding failed."),
                );
            }
        });
        if let Err(error) = canvas.to_blob_with_type(callback.unchecked_ref(), "image/png") {
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        }
    });
    let blob: Blob = JsFuture::from(promise).await?.dyn_into()?;
    Ok(Uint8Array::new(&JsFuture::from(blob.array_buffer()).await?).to_vec())
}

pub async fn do_export(
    editor: Editor,
    format: &str,
    scale: f64,
    ids: Option<Vec<String>>,
) -> Result<(), JsValue> {
    mounted(editor)?;
    text_session::finish(editor);
    let ids = export_ids(editor, ids);
    if ids.is_empty() {
        shell::toast(editor, "Add something to the canvas first.");
        return Ok(());
    }
    let name = editor.doc.with_untracked(|doc| {
        safe_name(if ids.len() == 1 {
            doc.get(&ids[0]).map_or(&doc.page().name, |node| &node.name)
        } else {
            &doc.page().name
        })
    });
    if format == "SVG" {
        let svg = export_svg(editor, Some(ids))?;
        files::export(&format!("{name}.svg"), svg.as_bytes(), "image/svg+xml")?;
    } else {
        let canvas = export_canvas(editor, &ids, scale).await?;
        let bytes = png_bytes(&canvas).await?;
        mounted(editor)?;
        files::export(&format!("{name}@{scale}x.png"), &bytes, "image/png")?;
    }
    shell::toast(editor, format!("{format} exported"));
    Ok(())
}

pub fn dispatch(editor: Editor, action: &str) {
    match action {
        "openFile" => pick(editor, Kind::Document),
        "placeImage" => pick(editor, Kind::Image),
        "loadFont" => pick(editor, Kind::Font),
        "saveFile" => report(editor, save_file(editor)),
        "exportTokens" => {
            text_session::finish(editor);
            let bytes = editor
                .doc
                .with_untracked(|doc| serde_json::to_vec_pretty(&doc.data.tokens));
            report(
                editor,
                bytes.map_err(js_error).and_then(|bytes| {
                    files::export("vellum-tokens.json", &bytes, "application/json")
                }),
            );
        }
        "exportPNG" | "exportSVG" | "exportSelection" => {
            let (format, scale) = editor.shell.with_untracked(|state| {
                (
                    if action == "exportSelection" {
                        state.export_format.clone()
                    } else if action == "exportSVG" {
                        "SVG".into()
                    } else {
                        "PNG".into()
                    },
                    if action == "exportSelection" {
                        state.export_scale
                    } else {
                        2.0
                    },
                )
            });
            leptos::task::spawn_local(async move {
                report(editor, do_export(editor, &format, scale, None).await);
            });
        }
        _ => {}
    }
}

pub struct DropBindings {
    target: EventTarget,
    over: Closure<dyn FnMut(DragEvent)>,
    drop: Closure<dyn FnMut(DragEvent)>,
}

impl Drop for DropBindings {
    fn drop(&mut self) {
        let _ = self
            .target
            .remove_event_listener_with_callback("dragover", self.over.as_ref().unchecked_ref());
        let _ = self
            .target
            .remove_event_listener_with_callback("drop", self.drop.as_ref().unchecked_ref());
    }
}

pub fn install_drop(editor: Editor, overlay: HtmlCanvasElement) -> Result<DropBindings, JsValue> {
    let target: EventTarget = overlay.parent_element().map_or_else(
        || overlay.clone().unchecked_into(),
        |element| element.unchecked_into(),
    );
    let over = Closure::new(move |event: DragEvent| {
        if event.data_transfer().is_some_and(|transfer| {
            transfer
                .types()
                .iter()
                .any(|kind| kind.as_string().as_deref() == Some("Files"))
        }) {
            event.prevent_default();
            if let Some(transfer) = event.data_transfer() {
                transfer.set_drop_effect("copy");
            }
        }
    });
    let drop = Closure::new(move |event: DragEvent| {
        let Some(file) = event
            .data_transfer()
            .and_then(|transfer| transfer.files())
            .and_then(|files| files.get(0))
        else {
            return;
        };
        event.prevent_default();
        if editor.doc.is_disposed() {
            return;
        }
        let bounds = overlay.get_bounding_client_rect();
        let location = editor.camera.get_untracked().screen_to_world(Point::new(
            f64::from(event.client_x()) - bounds.x(),
            f64::from(event.client_y()) - bounds.y(),
        ));
        leptos::task::spawn_local(async move {
            let name = file.name().to_ascii_lowercase();
            let result = if name.ends_with(".vellum") || name.ends_with(".json") {
                import_document(editor, file).await
            } else {
                import_image(editor, file, Some(location)).await
            };
            report(editor, result);
        });
    });
    let options = rustify_makepad::listener_options()
        .ok_or_else(|| JsValue::from_str("Runtime lifecycle is unavailable"))?;
    target.add_event_listener_with_callback_and_add_event_listener_options(
        "dragover",
        over.as_ref().unchecked_ref(),
        &options,
    )?;
    if let Err(error) = target.add_event_listener_with_callback_and_add_event_listener_options(
        "drop",
        drop.as_ref().unchecked_ref(),
        &options,
    ) {
        let _ =
            target.remove_event_listener_with_callback("dragover", over.as_ref().unchecked_ref());
        return Err(error);
    }
    Ok(DropBindings { target, over, drop })
}
