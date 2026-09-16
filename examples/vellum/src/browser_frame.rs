//! Animation frames keep their normal timing and are canceled by the runtime's abort signal.

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = __vellumAbortResource)]
    fn abort_resource(signal: &web_sys::AbortSignal, resource: &JsValue, kind: &str) -> Function;
}

#[derive(Clone)]
pub struct AnimationFrameRequestHandle {
    id: i32,
    release: Rc<RefCell<Option<Function>>>,
}

fn release(listener: &RefCell<Option<Function>>) {
    if let Some(unregister) = listener.borrow_mut().take() {
        let _ = unregister.call0(&JsValue::UNDEFINED);
    }
}

impl AnimationFrameRequestHandle {
    pub fn cancel(&self) {
        if let Some(window) = web_sys::window() {
            let _ = window.cancel_animation_frame(self.id);
        }
        release(&self.release);
    }
}

pub fn request_animation_frame_with_handle(
    callback: impl FnOnce() + 'static,
) -> Result<AnimationFrameRequestHandle, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window is unavailable"))?;
    let release_listener = Rc::new(RefCell::new(None));
    let listener = Rc::clone(&release_listener);
    let mut callback = Some(callback);
    let closure = Closure::<dyn FnMut()>::new(move || {
        release(&listener);
        if let Some(callback) = callback.take() {
            callback();
        }
    });
    // Like Leptos's helper, a canceled callback can be reclaimed by the JS finalizer.
    let closure = closure.into_js_value();
    let id = window.request_animation_frame(closure.unchecked_ref())?;
    if let Some(signal) =
        rustify_makepad::listener_options().and_then(|options| options.get_signal())
    {
        *release_listener.borrow_mut() = Some(abort_resource(&signal, &JsValue::from(id), "raf"));
    }
    Ok(AnimationFrameRequestHandle {
        id,
        release: release_listener,
    })
}

pub fn request_animation_frame(callback: impl FnOnce() + 'static) {
    let _ = request_animation_frame_with_handle(callback);
}
