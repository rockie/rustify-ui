use leptos::prelude::window;
use leptos::wasm_bindgen::{closure::Closure, JsCast};

pub(crate) struct Timeout {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: rustify_ui::AbortRelease,
}
impl Timeout {
    pub(crate) fn new(delay: u32, run: impl Fn() + 'static) -> Option<Self> {
        let callback = Closure::<dyn FnMut()>::new(run);
        let clear = js_sys::Reflect::get(&window(), &"clearTimeout".into())
            .ok()?
            .dyn_into::<js_sys::Function>()
            .ok()?;
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                i32::try_from(delay).ok()?,
            )
            .ok()?;
        let release =
            rustify_ui::release_on_abort(clear.bind1(&window(), &id.into()).unchecked_into());
        Some(Self {
            id,
            _callback: callback,
            _release: release,
        })
    }
}
impl Drop for Timeout {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}
