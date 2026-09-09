//! The one fixed-version third-party DOM component this release verifies.
//!
//! noUiSlider 15.8.1 is vendored into the checkout and recorded in
//! `sources.lock.json`; the page's own module hands it to the application
//! through a small shim. Nothing here is SDK code. The point is that a
//! component the SDK did not write, with its own initialization and teardown,
//! can live beside a GPU region and be rebuilt without leaving a second copy
//! or a second subscription behind.

use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__rustify_third_party"], js_name = create)]
    fn third_party_create(
        element: &leptos::web_sys::HtmlElement,
        start: f64,
        min: f64,
        max: f64,
        step: f64,
        on_update: &JsValue,
    ) -> bool;

    #[wasm_bindgen(js_namespace = ["window", "__rustify_third_party"], js_name = destroy)]
    fn third_party_destroy(element: &leptos::web_sys::HtmlElement) -> bool;

    #[wasm_bindgen(js_namespace = ["window", "__rustify_third_party"], js_name = set)]
    fn third_party_set(element: &leptos::web_sys::HtmlElement, value: f64) -> bool;
}

/// A third-party slider bound to the same application value as the SDK's own.
///
/// `on_update` hears every callback the component makes, before the value is
/// compared with the application's: a rebuild that left an old subscription
/// behind would show up as two callbacks for one change.
#[component]
pub fn ThirdPartySlider(
    #[prop(into)] value: Signal<f64>,
    on_change: impl Fn(f64) + 'static,
    on_update: impl Fn() + 'static,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let node = NodeRef::<leptos::html::Div>::new();
    // The subscription is this closure. It is held for exactly as long as the
    // component, so dropping it is what makes a rebuild leave nothing behind.
    // Held locally: a JS closure belongs to the thread that made it.
    let subscription = StoredValue::new_local(None::<Closure<dyn FnMut(f64)>>);
    let on_change = Rc::new(on_change);
    let on_update = Rc::new(on_update);

    Effect::new({
        move || {
            let Some(element) = node.get() else {
                return;
            };
            if subscription.with_value(|slot| slot.is_some()) {
                return;
            }
            let element: leptos::web_sys::HtmlElement = element.into();
            let on_change = on_change.clone();
            let on_update = on_update.clone();
            let callback = Closure::wrap(Box::new(move |reported: f64| {
                on_update();
                // The component reports its own `set` back to us; only a value
                // the application does not already hold is a change.
                if reported != value.get_untracked() {
                    on_change(reported);
                }
            }) as Box<dyn FnMut(f64)>);
            third_party_create(
                &element,
                value.get_untracked(),
                0.0,
                100.0,
                5.0,
                callback.as_ref(),
            );
            subscription.set_value(Some(callback));
        }
    });

    // The application's value, pushed into a component that keeps its own.
    Effect::new(move || {
        let held = value.get();
        if let Some(element) = node.get_untracked() {
            let element: leptos::web_sys::HtmlElement = element.into();
            third_party_set(&element, held);
        }
    });

    on_cleanup(move || {
        if let Some(element) = node.get_untracked() {
            let element: leptos::web_sys::HtmlElement = element.into();
            third_party_destroy(&element);
        }
        // Dropped after the component has gone, so nothing can call into a
        // closure whose captured signals belong to a scope that is closing.
        subscription.set_value(None);
    });

    view! { <div class="third-party" node_ref=node data-testid=test_id></div> }
}
