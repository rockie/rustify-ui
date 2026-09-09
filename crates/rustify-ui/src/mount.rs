use crate::binding::ActionSink;
use crate::diagnostics::UiError;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlElement;
use std::any::Any;

const MOUNTED_ATTRIBUTE: &str = "data-rustify-scope";
/// The application's own nodes. A modal layer makes this inert; it lays out as
/// if it were not there (`display: contents` in the runtime stylesheet).
const CONTENT_ATTRIBUTE: &str = "data-rustify-content";
/// Above the content and above every GPU region in the scope; the layers of
/// this scope and nothing else live here.
const OVERLAY_ATTRIBUTE: &str = "data-rustify-overlay";

#[derive(Clone, Debug, Default)]
pub struct MountConfig {
    /// Human readable scope name used in diagnostics and DOM attributes.
    pub scope: String,
}

/// Owns one mounted application scope. Disposing it runs the scope's reactive
/// cleanups first (which destroys every GPU region the view created) and
/// then unmounts the DOM.
pub struct AppHandle {
    container: HtmlElement,
    roots: Vec<leptos::web_sys::Element>,
    owner: Option<Owner>,
    unmount: Option<Box<dyn Any>>,
}

pub fn mount<F, N>(
    container: HtmlElement,
    config: MountConfig,
    view: F,
) -> Result<AppHandle, UiError>
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
    N::State: 'static,
{
    if !container.is_connected() {
        return Err(UiError::InvalidContainer);
    }
    if container.has_attribute(MOUNTED_ATTRIBUTE) {
        return Err(UiError::OccupiedContainer);
    }
    let scope = if config.scope.is_empty() {
        "default".to_string()
    } else {
        config.scope
    };
    container
        .set_attribute(MOUNTED_ATTRIBUTE, &scope)
        .map_err(|_| UiError::InvalidContainer)?;
    // A scope can always be given focus back, even when the control a layer
    // was opened from has gone.
    let _ = container.set_attribute("tabindex", "-1");
    let content = child(&container, CONTENT_ATTRIBUTE)?;
    let overlay = child(&container, OVERLAY_ATTRIBUTE)?;
    let owner = Owner::new();
    let target = content.clone().unchecked_into::<HtmlElement>();
    let unmount = owner.with(|| {
        // Every region in this scope shares one acceptance order.
        provide_context(ActionSink::new());
        #[cfg(target_arch = "wasm32")]
        provide_context(crate::overlay::OverlayStack::new(
            &container, &content, &overlay,
        ));
        leptos::mount::mount_to(target, view)
    });
    Ok(AppHandle {
        container,
        roots: vec![content, overlay],
        owner: Some(owner),
        unmount: Some(Box::new(unmount)),
    })
}

/// Adds one of the scope's own roots to the container.
fn child(container: &HtmlElement, attribute: &str) -> Result<leptos::web_sys::Element, UiError> {
    let element = leptos::prelude::document()
        .create_element("div")
        .map_err(|_| UiError::InvalidContainer)?;
    element
        .set_attribute(attribute, "")
        .map_err(|_| UiError::InvalidContainer)?;
    container
        .append_child(&element)
        .map_err(|_| UiError::InvalidContainer)?;
    Ok(element)
}

impl AppHandle {
    /// Idempotent: the first call unmounts, later calls are no-ops.
    pub fn dispose(&mut self) {
        if let Some(owner) = self.owner.take() {
            owner.cleanup();
        }
        if let Some(unmount) = self.unmount.take() {
            drop(unmount);
            for root in self.roots.drain(..) {
                let _ = self.container.remove_child(&root);
            }
            let _ = self.container.remove_attribute(MOUNTED_ATTRIBUTE);
            let _ = self.container.remove_attribute("tabindex");
        }
    }

    pub fn is_disposed(&self) -> bool {
        self.unmount.is_none()
    }
}

impl Drop for AppHandle {
    fn drop(&mut self) {
        self.dispose();
    }
}
