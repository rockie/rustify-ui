use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use js_sys::{Function, Promise};
use serde_json::{json, Value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{AbortSignal, FontFace, FontFaceSet};

const MAX_FONT_BYTES: usize = 15 * 1024 * 1024;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = __vellumAbortResource)]
    fn abort_resource(signal: &AbortSignal, resource: &JsValue, kind: &str) -> Function;
}

fn runtime_signal() -> Result<AbortSignal, JsValue> {
    rustify_makepad::listener_options()
        .and_then(|options| options.get_signal())
        .filter(|signal| !signal.aborted())
        .ok_or_else(|| JsValue::from_str("Vellum is no longer mounted"))
}

struct LoadedFont {
    source: Rc<str>,
    face: FontFace,
    unregister: Function,
}

impl Drop for LoadedFont {
    fn drop(&mut self) {
        // Normal removal releases the native abort listener. Fatal removal is entirely JS.
        let _ = self.unregister.call0(&JsValue::UNDEFINED);
    }
}

struct PendingFont {
    source: Rc<str>,
    promise: Promise,
}

#[derive(Default)]
struct Fonts {
    disposed: bool,
    loaded: BTreeMap<String, LoadedFont>,
    pending: BTreeMap<String, PendingFont>,
    errors: BTreeMap<String, String>,
}

/// Font faces belong to the mounted editor, including loads still in flight.
#[derive(Clone, Default)]
pub struct FontCache(Rc<RefCell<Fonts>>);

impl FontCache {
    /// Invalidates faces and in-flight results that no longer belong to the document.
    pub fn retain_sources(&self, sources: &BTreeMap<String, Rc<str>>) {
        let mut fonts = self.0.borrow_mut();
        let set = font_set().ok();
        fonts.loaded.retain(|family, font| {
            let keep = sources.get(family) == Some(&font.source);
            if !keep {
                if let Some(set) = &set {
                    set.delete(&font.face);
                }
            }
            keep
        });
        fonts
            .pending
            .retain(|family, font| sources.get(family) == Some(&font.source));
        fonts
            .errors
            .retain(|family, _| sources.contains_key(family));
    }

    /// Registers a face only after the importing editor has checked its lifetime and revision.
    pub fn insert_decoded(
        &self,
        family: &str,
        source: Rc<str>,
        face: FontFace,
    ) -> Result<(), JsValue> {
        let signal = runtime_signal()?;
        let mut fonts = self.0.borrow_mut();
        if fonts.disposed {
            return Err(JsValue::from_str("Vellum is no longer mounted"));
        }
        let set = font_set()?;
        set.add(&face)?;
        let unregister = abort_resource(&signal, face.as_ref(), "font");
        fonts.pending.remove(family);
        fonts.errors.remove(family);
        if let Some(old) = fonts.loaded.insert(
            family.into(),
            LoadedFont {
                source,
                face,
                unregister,
            },
        ) {
            set.delete(&old.face);
        }
        Ok(())
    }

    pub fn load(&self, family: &str, source: Rc<str>) -> Result<Promise, JsValue> {
        let signal = runtime_signal()?;
        {
            let fonts = self.0.borrow();
            if fonts.disposed {
                return Err(JsValue::from_str("Vellum is no longer mounted"));
            }
            if let Some(pending) = fonts.pending.get(family) {
                if pending.source == source {
                    return Ok(pending.promise.clone());
                }
            }
            if fonts
                .loaded
                .get(family)
                .is_some_and(|font| font.source == source)
                && !fonts.pending.contains_key(family)
            {
                return Ok(Promise::resolve(&JsValue::UNDEFINED));
            }
        }
        let start = || {
            let bytes = decode_data_url(&source)?;
            // A byte source works under font-src 'self' without a data/blob URL exception.
            let face = FontFace::new_with_u8_array(family, &bytes)?;
            let pending = face.load()?;
            Ok::<_, JsValue>((face, pending))
        };
        let (face, loading) = match start() {
            Ok(started) => started,
            Err(error) => {
                self.0
                    .borrow_mut()
                    .errors
                    .insert(family.into(), error_text(&error));
                return Err(error);
            }
        };
        let cache = self.clone();
        let key = family.to_owned();
        let pending_source = Rc::clone(&source);
        let promise = future_to_promise(async move {
            let loaded = JsFuture::from(loading).await;
            let mut fonts = cache.0.borrow_mut();
            let current = fonts
                .pending
                .get(&key)
                .is_some_and(|pending| pending.source == pending_source);
            if signal.aborted() || fonts.disposed || !current {
                if current {
                    fonts.pending.remove(&key);
                }
                return Err(JsValue::from_str("Font load was superseded or disposed"));
            }
            fonts.pending.remove(&key);
            let result = loaded.and_then(|_| {
                let set = font_set()?;
                set.add(&face)?;
                let unregister = abort_resource(&signal, face.as_ref(), "font");
                if let Some(old) = fonts.loaded.insert(
                    key.clone(),
                    LoadedFont {
                        source: pending_source,
                        face,
                        unregister,
                    },
                ) {
                    set.delete(&old.face);
                }
                Ok(JsValue::UNDEFINED)
            });
            match &result {
                Ok(_) => {
                    fonts.errors.remove(&key);
                }
                Err(error) => {
                    fonts.errors.insert(key, error_text(error));
                }
            }
            result
        });
        self.0.borrow_mut().pending.insert(
            family.into(),
            PendingFont {
                source,
                promise: promise.clone(),
            },
        );
        Ok(promise)
    }

    pub async fn ready(&self) -> Result<Value, JsValue> {
        let signal = runtime_signal()?;
        loop {
            if signal.aborted() {
                return Err(JsValue::from_str("Vellum is no longer mounted"));
            }
            let pending: Vec<_> = self
                .0
                .borrow()
                .pending
                .values()
                .map(|font| font.promise.clone())
                .collect();
            if pending.is_empty() {
                break;
            }
            for promise in pending {
                // Failed fonts are reported in the status without blocking the document.
                let _ = JsFuture::from(promise).await;
            }
        }
        JsFuture::from(font_set()?.ready()?).await?;
        if signal.aborted() {
            return Err(JsValue::from_str("Vellum is no longer mounted"));
        }
        Ok(self.status())
    }

    pub fn status(&self) -> Value {
        let fonts = self.0.borrow();
        json!({
            "pending": fonts.pending.len(),
            "families": fonts.loaded.keys().collect::<Vec<_>>(),
            "errors": fonts.errors.iter().map(|(family,error)| json!({"family":family,"error":error})).collect::<Vec<_>>()
        })
    }

    pub fn dispose(&self) {
        let mut fonts = self.0.borrow_mut();
        fonts.disposed = true;
        if let Ok(set) = font_set() {
            for font in fonts.loaded.values() {
                set.delete(&font.face);
            }
        }
        fonts.loaded.clear();
        fonts.pending.clear();
    }
}

pub fn family_name(name: &str) -> String {
    let lowercase = name.to_ascii_lowercase();
    let stem = [".ttf", ".otf", ".woff", ".woff2"]
        .iter()
        .find(|suffix| lowercase.ends_with(**suffix))
        .map_or(name, |suffix| &name[..name.len() - suffix.len()]);
    stem.replace(['"', '\\'], "")
}

fn font_set() -> Result<FontFaceSet, JsValue> {
    web_sys::window()
        .and_then(|window| window.document())
        .map(|document| document.fonts())
        .ok_or_else(|| JsValue::from_str("Document font set is unavailable"))
}

pub(crate) fn error_text(error: &JsValue) -> String {
    error.as_string().unwrap_or_else(|| format!("{error:?}"))
}

fn decode_data_url(source: &str) -> Result<Vec<u8>, JsValue> {
    let malformed = || JsValue::from_str("Invalid font data URL");
    let (header, body) = source.split_once(',').ok_or_else(malformed)?;
    if !header.starts_with("data:") {
        return Err(malformed());
    }
    // Avoid allocating a decoded copy of an arbitrarily large import.
    if body.len() > MAX_FONT_BYTES * 3 {
        return Err(JsValue::from_str("Font file is too large"));
    }
    let bytes = if header.split(';').any(|part| part == "base64") {
        let window = web_sys::window().ok_or_else(malformed)?;
        window.atob(body)?.chars().map(|ch| ch as u8).collect()
    } else {
        let mut bytes = Vec::with_capacity(body.len());
        let mut encoded = body.bytes();
        while let Some(byte) = encoded.next() {
            if byte == b'%' {
                let high = encoded.next().and_then(|byte| (byte as char).to_digit(16));
                let low = encoded.next().and_then(|byte| (byte as char).to_digit(16));
                match (high, low) {
                    (Some(high), Some(low)) => bytes.push((high * 16 + low) as u8),
                    _ => return Err(malformed()),
                }
            } else {
                bytes.push(byte);
            }
        }
        bytes
    };
    if bytes.len() > MAX_FONT_BYTES {
        return Err(JsValue::from_str("Font file is too large"));
    }
    Ok(bytes)
}
