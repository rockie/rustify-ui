//! The two things a component sometimes has to do to an element that is not
//! its own: move the keyboard to it, and read where it is.

use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Element;
use leptos::web_sys::HtmlElement;

/// Moves the keyboard to the element with this id, if it is on the page.
///
/// Silence when it is not is deliberate: a component that asks for focus
/// during the render that creates the element is asking a moment early, and
/// the answer arrives on the next one.
pub(crate) fn focus_id(id: &str) {
    if let Some(element) = leptos::prelude::document().get_element_by_id(id) {
        if let Ok(element) = element.dyn_into::<HtmlElement>() {
            let _ = element.focus();
        }
    }
}

/// Moves the keyboard to the first element inside `container` matching
/// `selector`.
///
/// A component with hundreds of interchangeable elements cannot give each of
/// them an id: an id is a reactive attribute, and a reactive attribute per
/// element is a JavaScript value per element that wasm has to keep hold of.
/// Asking the container for the one that matters costs nothing until it is
/// asked.
#[cfg(target_arch = "wasm32")]
pub(crate) fn focus_within(container: &Element, selector: &str) {
    if let Ok(Some(element)) = container.query_selector(selector) {
        if let Ok(element) = element.dyn_into::<HtmlElement>() {
            let _ = element.focus();
        }
    }
}
