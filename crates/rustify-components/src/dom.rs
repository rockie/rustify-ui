//! The two things a component sometimes has to do to an element that is not
//! its own: move the keyboard to it, and read where it is.

use leptos::wasm_bindgen::JsCast;
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
