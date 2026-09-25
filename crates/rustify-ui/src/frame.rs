//! Animation frames that end with the instance that asked for them.
//!
//! A frame's callback is wasm code. A trap runs no destructor, so a frame
//! requested just before one would still be called afterwards, into a module
//! nothing can trust. Each request is therefore also handed to the instance's
//! abort signal, as a cancellation the browser performs on its own: what the
//! signal calls is `cancelAnimationFrame` itself, not anything in this module.

use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::web_sys::{AbortSignal, AddEventListenerOptions};
use send_wrapper::SendWrapper;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// A frame asked for with [`next_frame`]. Dropping it does not cancel it.
///
/// `Send` and `Sync` so that a cleanup can hold it; it never leaves the one
/// thread a browser page has.
#[derive(Clone)]
pub struct FrameHandle(SendWrapper<Rc<Pending>>);

#[derive(Default)]
struct Pending {
    /// The browser's id for the frame, while it is still to come.
    id: Cell<Option<i32>>,
    /// The callback, held here so that cancelling frees it: a callback handed
    /// over to JS for good would be freed only by being called.
    callback: RefCell<Option<Closure<dyn FnMut()>>>,
    /// The cancellation registered on the abort signal.
    on_abort: RefCell<Option<(AbortSignal, js_sys::Function)>>,
}

impl Pending {
    /// Leaves nothing of this request registered anywhere.
    fn release(&self) {
        self.id.set(None);
        if let Some((signal, cancel)) = self.on_abort.take() {
            let _ = signal.remove_event_listener_with_callback("abort", &cancel);
        }
        // May be the callback that is running: wasm-bindgen frees a closure
        // dropped during its own call once that call returns.
        drop(self.callback.take());
    }
}

impl FrameHandle {
    /// Cancels the frame if it has not run yet.
    pub fn cancel(&self) {
        if let (Some(id), Some(window)) = (self.0.id.get(), leptos::web_sys::window()) {
            let _ = window.cancel_animation_frame(id);
        }
        self.0.release();
    }
}

/// Runs `f` once, before the browser's next repaint.
///
/// It never runs after this instance has failed: the frame is cancelled when
/// the instance's abort signal fires, by the browser, with no wasm involved.
/// Without a runtime - before `boot()` - it is an ordinary animation frame.
pub fn next_frame(f: impl FnOnce() + 'static) -> FrameHandle {
    let handle = FrameHandle(SendWrapper::new(Rc::new(Pending::default())));
    let Some(window) = leptos::web_sys::window() else {
        return handle;
    };
    let signal = crate::listeners::instance_signal();
    if signal.as_ref().is_some_and(AbortSignal::aborted) {
        return handle;
    }

    // The callback holds its own request until it runs or is cancelled; both
    // paths release it, which is what breaks the cycle.
    let pending = Rc::clone(&handle.0);
    let mut f = Some(f);
    let callback = Closure::<dyn FnMut()>::new(move || {
        pending.release();
        if let Some(f) = f.take() {
            f();
        }
    });
    let Ok(id) = window.request_animation_frame(callback.as_ref().unchecked_ref()) else {
        return handle;
    };
    handle.0.id.set(Some(id));
    *handle.0.callback.borrow_mut() = Some(callback);

    if let Some(signal) = signal {
        let cancel = js_sys::Reflect::get(&window, &JsValue::from_str("cancelAnimationFrame"))
            .ok()
            .and_then(|cancel| cancel.dyn_into::<js_sys::Function>().ok())
            .map(|cancel| {
                cancel
                    .bind1(&window, &JsValue::from(id))
                    .unchecked_into::<js_sys::Function>()
            });
        if let Some(cancel) = cancel {
            let once = AddEventListenerOptions::new();
            once.set_once(true);
            let _ = signal.add_event_listener_with_callback_and_add_event_listener_options(
                "abort", &cancel, &once,
            );
            *handle.0.on_abort.borrow_mut() = Some((signal, cancel));
        }
    }
    handle
}
