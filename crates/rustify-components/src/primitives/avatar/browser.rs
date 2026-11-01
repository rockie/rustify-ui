use super::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlImageElement;

fn update(
    request: RwSignal<model::Request>,
    context: Option<Context>,
    callback: Option<Callback<ImageLoadingStatus>>,
    change: impl FnOnce(&mut model::Request),
) {
    let next = request.try_maybe_update(|request| {
        let previous = (request.generation, request.status);
        change(request);
        let current = (request.generation, request.status);
        (
            previous != current,
            (current.0, (previous.1 != current.1).then_some(current.1)),
        )
    });
    let Some((generation, next)) = next else {
        return;
    };
    if let Some(context) = context {
        if context.generation.try_get_untracked() != Some(generation) {
            let _ = context.generation.try_set(generation);
        }
        if let Some(status) = next {
            let _ = context.status.try_set(status);
        }
    }
    if let (Some(status), Some(callback)) = (next, callback) {
        untrack(|| callback.try_run(status));
    }
}
pub(super) fn source(
    src: Signal<String>,
    request: RwSignal<model::Request>,
    context: Option<Context>,
    callback: Option<Callback<ImageLoadingStatus>>,
) {
    Effect::new(move || {
        let src = src.get();
        update(request, context, callback, move |request| {
            request.set_source(src)
        });
    });
}
pub(super) fn image(
    node: NodeRef<leptos::html::Img>,
    request: RwSignal<model::Request>,
    context: Option<Context>,
    callback: Option<Callback<ImageLoadingStatus>>,
    generation: u64,
) {
    let held = StoredValue::new_local(Vec::<rustify_ui::Listener>::new());
    Effect::new(move || {
        let Some(node) = node.get() else {
            return;
        };
        let element: HtmlImageElement = node.into();
        let loaded = element.clone();
        let load = rustify_ui::listen(element.as_ref(), "load", Default::default(), move |_| {
            update(request, context, callback, |request| {
                request.inspect(generation, true, loaded.natural_width())
            });
        });
        let error = rustify_ui::listen(element.as_ref(), "error", Default::default(), move |_| {
            update(request, context, callback, |request| {
                request.fail(generation)
            });
        });
        held.set_value(vec![load, error]);
        update(request, context, callback, |request| {
            request.inspect(generation, element.complete(), element.natural_width())
        });
    });
    on_cleanup(move || {
        held.try_update_value(|held| held.clear());
    });
}
struct Timer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: rustify_ui::AbortRelease,
}
impl Timer {
    fn new(delay: u32, ready: RwSignal<bool>) -> Option<Self> {
        let callback = Closure::<dyn FnMut()>::new(move || {
            let _ = ready.try_set(true);
        });
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
impl Drop for Timer {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}
pub(super) fn fallback(context: Context, ready: RwSignal<bool>, delay: u32) {
    let timer = StoredValue::new_local(None::<Timer>);
    let generation = StoredValue::new(None);
    Effect::new(move || {
        let next = context.generation.get();
        if context.status.get() == ImageLoadingStatus::Loaded {
            timer.set_value(None);
            generation.set_value(None);
            if !ready.get_untracked() {
                ready.set(true);
            }
            return;
        }
        if generation.get_value() == Some(next) {
            return;
        }
        generation.set_value(Some(next));
        timer.set_value(None);
        if delay == 0 || delay > i32::MAX as u32 {
            if !ready.get_untracked() {
                ready.set(true);
            }
        } else {
            if ready.get_untracked() {
                ready.set(false);
            }
            timer.set_value(Timer::new(delay, ready));
            if timer.with_value(Option::is_none) {
                ready.set(true);
            }
        }
    });
    on_cleanup(move || {
        timer.try_set_value(None);
    });
}
