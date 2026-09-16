use std::{cell::RefCell, collections::HashMap, rc::Rc};

use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::HtmlImageElement;

use crate::document::{Document, SharedAssets};

#[derive(Default)]
struct Images {
    disposed: bool,
    generation: u64,
    sources: SharedAssets,
    decoded: HashMap<String, HtmlImageElement>,
    pending: HashMap<String, Promise>,
}

#[derive(Clone)]
pub struct ImageCache {
    images: Rc<RefCell<Images>>,
    invalidate: Rc<dyn Fn()>,
}

impl ImageCache {
    pub fn new(invalidate: Rc<dyn Fn()>) -> Self {
        Self {
            images: Rc::new(RefCell::new(Images::default())),
            invalidate,
        }
    }

    /// Updates sources and discards decoded assets whose contents changed.
    pub fn sync(&self, document: &Document) -> bool {
        let mut images = self.images.borrow_mut();
        if images.disposed {
            return false;
        }
        if images.sources == document.data.assets {
            return false;
        }
        let changed: Vec<_> = images
            .sources
            .keys()
            .filter(|id| images.sources.get(*id) != document.data.assets.get(*id))
            .cloned()
            .collect();
        for id in changed {
            images.decoded.remove(&id);
            images.pending.remove(&id);
        }
        images.sources = document.data.assets.clone();
        true
    }

    /// Starts decoding on the first lookup and returns no image until it is ready.
    pub fn get(&self, id: &str) -> Option<HtmlImageElement> {
        let (source, generation) = {
            let images = self.images.borrow();
            if images.disposed {
                return None;
            }
            if let Some(image) = images.decoded.get(id) {
                return Some(image.clone());
            }
            if images.pending.contains_key(id) {
                return None;
            }
            (images.sources.get(id)?.clone(), images.generation)
        };
        let image = HtmlImageElement::new().ok()?;
        image.set_src(&source);
        let decoded = image.decode();
        let cache = self.clone();
        let key = id.to_owned();
        let pending = future_to_promise(async move {
            if JsFuture::from(decoded).await.is_ok() {
                let still_current = {
                    let images = cache.images.borrow();
                    !images.disposed
                        && images.generation == generation
                        && images.sources.get(&key) == Some(&source)
                };
                if still_current {
                    cache.images.borrow_mut().decoded.insert(key, image);
                    (cache.invalidate)();
                }
            }
            // A failed asset must not prevent the remaining document from exporting.
            Ok(JsValue::UNDEFINED)
        });
        self.images
            .borrow_mut()
            .pending
            .insert(id.to_owned(), pending);
        None
    }

    pub async fn ready(&self) {
        let ids: Vec<_> = self.images.borrow().sources.keys().cloned().collect();
        for id in ids {
            self.get(&id);
        }
        let pending: Vec<_> = self.images.borrow().pending.values().cloned().collect();
        for promise in pending {
            let _ = JsFuture::from(promise).await;
        }
    }

    pub fn insert_decoded(&self, id: String, image: HtmlImageElement) {
        {
            let mut images = self.images.borrow_mut();
            if images.disposed {
                return;
            }
            images.decoded.insert(id, image);
        }
        (self.invalidate)();
    }

    pub fn clear(&self) {
        let mut images = self.images.borrow_mut();
        images.generation = images.generation.wrapping_add(1);
        images.decoded.clear();
        images.pending.clear();
    }

    pub fn dispose(&self) {
        self.clear();
        let mut images = self.images.borrow_mut();
        images.disposed = true;
        images.sources.clear();
    }
}
