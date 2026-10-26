use leptos::wasm_bindgen::{prelude::*, JsCast};
use serde::Deserialize;

#[wasm_bindgen(module = "/src/storage.js")]
extern "C" {
    #[wasm_bindgen(js_name = makeStorageGuard)]
    pub fn make_guard() -> JsValue;
    #[wasm_bindgen(js_name = storageAbortHandle)]
    pub fn abort_handle(guard: &JsValue) -> js_sys::Function;
    #[wasm_bindgen(js_name = storageAlive)]
    pub fn alive(guard: &JsValue) -> bool;
    #[wasm_bindgen(js_name = storageWritable)]
    pub fn writable() -> bool;
    #[wasm_bindgen(catch, js_name = storageBackups)]
    pub fn backups() -> Result<String, JsValue>;
    #[wasm_bindgen(js_name = commitStorage)]
    pub fn commit(raw: &str, guard: &JsValue) -> js_sys::Promise;
}

pub struct Lifetime {
    pub guard: JsValue,
    _release: rustify_ui::AbortRelease,
}
impl Lifetime {
    pub fn new() -> Self {
        let guard = make_guard();
        let release = rustify_ui::release_on_abort(abort_handle(&guard));
        Self {
            guard,
            _release: release,
        }
    }
    pub fn cancel(&self) {
        let _ = abort_handle(&self.guard).call0(&JsValue::NULL);
    }
}
impl Drop for Lifetime {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[derive(Deserialize)]
pub struct Answer {
    pub kind: String,
    pub raw: Option<String>,
}

pub fn read(key: &str) -> Result<Option<String>, String> {
    let storage = web_sys::window()
        .ok_or("Browser window unavailable")?
        .local_storage()
        .map_err(message)?
        .ok_or("localStorage unavailable")?;
    storage.get_item(key).map_err(message)
}
pub fn message(value: JsValue) -> String {
    value
        .as_string()
        .or_else(|| {
            value
                .dyn_ref::<js_sys::Error>()
                .map(|error| error.message().into())
        })
        .unwrap_or_else(|| format!("{value:?}"))
}
