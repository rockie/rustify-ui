use crate::binding::ActionSink;
use crate::diagnostics::UiError;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlElement;
use std::any::Any;

const MOUNTED_ATTRIBUTE: &str = "data-rustify-scope";

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
    let owner = Owner::new();
    let target = container.clone().unchecked_into::<HtmlElement>();
    let unmount = owner.with(|| {
        // Every region in this scope shares one acceptance order.
        provide_context(ActionSink::new());
        leptos::mount::mount_to(target, view)
    });
    Ok(AppHandle {
        container,
        owner: Some(owner),
        unmount: Some(Box::new(unmount)),
    })
}

impl AppHandle {
    /// Idempotent: the first call unmounts, later calls are no-ops.
    pub fn dispose(&mut self) {
        if let Some(owner) = self.owner.take() {
            owner.cleanup();
        }
        if let Some(unmount) = self.unmount.take() {
            drop(unmount);
            let _ = self.container.remove_attribute(MOUNTED_ATTRIBUTE);
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
