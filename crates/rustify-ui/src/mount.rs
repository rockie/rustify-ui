use crate::binding::ActionSink;
use crate::diagnostics::{note, record, ErrorKind, UiError};
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
    /// Whether this scope owns the page's address bar.
    ///
    /// One page has one address bar, so at most one scope may say yes. A
    /// scope that says no still routes - it gets a location of its own, in
    /// memory - which is what lets an application be embedded in a page that
    /// is navigating for its own reasons, and what lets a second instance
    /// exist on the same page at all.
    pub url_owner: bool,
    /// The path this application is deployed under, for an owner. `/tools/demo/`
    /// means a route of `/objects` is at `/tools/demo/objects`.
    pub base: String,
}

/// The scope's own elements, for the parts of the SDK that need to reach them
/// from inside the view: the layer stack and the theme.
#[derive(Clone)]
pub struct ScopeRoots(
    #[cfg(target_arch = "wasm32")] send_wrapper::SendWrapper<leptos::web_sys::HtmlElement>,
    #[cfg(not(target_arch = "wasm32"))] std::marker::PhantomData<()>,
);

#[cfg(target_arch = "wasm32")]
impl ScopeRoots {
    pub fn container(&self) -> leptos::web_sys::HtmlElement {
        (*self.0).clone()
    }
}

/// Owns one mounted application scope. Disposing it runs the scope's reactive
/// cleanups first (which destroys every GPU region the view created) and
/// then unmounts the DOM.
pub struct AppHandle {
    container: HtmlElement,
    /// Held for as long as the scope is mounted; dropping it lets another
    /// scope own the URL.
    #[cfg(target_arch = "wasm32")]
    url_owner: Option<crate::router::UrlClaim>,
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
        record(
            note(
                ErrorKind::InvalidContainer,
                "the container is not in the document",
            )
            .in_scope(config.scope.clone()),
        );
        return Err(UiError::InvalidContainer);
    }
    if container.has_attribute(MOUNTED_ATTRIBUTE) {
        record(
            note(
                ErrorKind::OccupiedContainer,
                "another scope already owns the container",
            )
            .in_scope(config.scope.clone()),
        );
        return Err(UiError::OccupiedContainer);
    }
    let scope = if config.scope.is_empty() {
        "default".to_string()
    } else {
        config.scope.clone()
    };
    // After the container gates and before anything is written: a scope that
    // cannot have what it asked for must leave the page as it found it, and
    // the scope that does own the URL must be untouched by the attempt.
    #[cfg(target_arch = "wasm32")]
    let url_owner = if config.url_owner {
        match crate::router::claim_url() {
            Some(claim) => Some(claim),
            None => {
                record(
                    note(
                        ErrorKind::UrlOwnerConflict,
                        "another scope on this page already owns the URL",
                    )
                    .in_scope(scope.clone()),
                );
                return Err(UiError::UrlOwnerConflict);
            }
        }
    } else {
        None
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
        {
            provide_context(crate::overlay::OverlayStack::new(
                &container, &content, &overlay,
            ));
            provide_context(ScopeRoots(send_wrapper::SendWrapper::new(
                container.clone(),
            )));
            crate::router::provide_router(&container, url_owner.clone(), &config.base);
        }
        leptos::mount::mount_to(target, view)
    });
    Ok(AppHandle {
        container,
        #[cfg(target_arch = "wasm32")]
        url_owner,
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
        #[cfg(target_arch = "wasm32")]
        {
            self.url_owner = None;
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
