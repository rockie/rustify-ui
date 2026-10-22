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

#[cfg(feature = "hydrate")]
const HYDRATED_ATTRIBUTE: &str = "data-rustify-mounted";

/// Generates the roots and deterministic identity that `hydrate` takes over.
/// The caller supplies the same initial view and scope configuration on both
/// targets. Browser-only work begins after hydration.
#[cfg(feature = "ssr")]
pub fn ssr_scope<F, N>(config: MountConfig, view: F) -> impl IntoView
where
    F: FnOnce() -> N,
    N: IntoView,
{
    let owner = Owner::new();
    let nodes = owner.with(|| {
        let scope = scope_name(&config);
        crate::id::provide(&scope);
        crate::provide_locale(crate::Locale::English);
        provide_context(ActionSink::new());
        #[cfg(not(target_arch = "wasm32"))]
        provide_context(ScopeRoots(std::marker::PhantomData, config.base.clone()));
        view! {
            <div
                data-rustify-scope=scope.clone()
                data-rustify-seed=scope
                data-rustify-ssr="1"
                data-rustify-base=config.base
                tabindex="-1"
            >
                <div data-rustify-content="">{view()}</div>
                <div data-rustify-overlay=""></div>
            </div>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(nodes, owner)
}

#[cfg(any(feature = "ssr", all(feature = "hydrate", target_arch = "wasm32")))]
fn scope_name(config: &MountConfig) -> String {
    if config.scope.is_empty() {
        "default".to_string()
    } else {
        config.scope.clone()
    }
}

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
    String,
);

impl ScopeRoots {
    pub fn base(&self) -> &str {
        &self.1
    }
}

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
    // Before anything else: a scope whose listeners and tasks nothing could
    // end when the instance fails is worse than no scope.
    #[cfg(target_arch = "wasm32")]
    if !crate::runtime::booted() {
        record(
            note(
                ErrorKind::RuntimeNotStarted,
                "mount was called before the runtime was started",
            )
            .in_scope(config.scope.clone()),
        );
        return Err(UiError::RuntimeNotStarted);
    }
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
        match crate::router::claim_url(&scope) {
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
            provide_context(ScopeRoots(
                send_wrapper::SendWrapper::new(container.clone()),
                config.base.clone(),
            ));
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

/// Takes over a generated scope without replacing its content nodes.
/// Refusals leave its DOM untouched. The runtime must be booted first and the
/// external startup script must initialize Leptos' empty hydration globals.
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub fn hydrate<F, N>(
    container: HtmlElement,
    config: MountConfig,
    view: F,
) -> Result<AppHandle, UiError>
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
    N::State: 'static,
{
    let scope = scope_name(&config);
    let refuse = |error: UiError, detail| {
        if let Some(kind) = error.kind() {
            record(note(kind, detail).in_scope(scope.clone()));
        }
        error
    };
    if !crate::runtime::booted() {
        return Err(refuse(
            UiError::RuntimeNotStarted,
            "hydrate was called before the runtime was started",
        ));
    }
    if !container.is_connected() {
        return Err(refuse(
            UiError::InvalidContainer,
            "the hydration container is not in the document",
        ));
    }
    if container.has_attribute(HYDRATED_ATTRIBUTE) {
        return Err(refuse(
            UiError::OccupiedContainer,
            "another scope already owns the hydration container",
        ));
    }
    let content = container.first_element_child();
    let overlay = content
        .as_ref()
        .and_then(|element| element.next_element_sibling());
    let valid_roots = content.as_ref().is_some_and(|element| {
        element.tag_name() == "DIV" && element.has_attribute(CONTENT_ATTRIBUTE)
    }) && overlay.as_ref().is_some_and(|element| {
        element.tag_name() == "DIV"
            && element.has_attribute(OVERLAY_ATTRIBUTE)
            && element.next_element_sibling().is_none()
            && !element.has_child_nodes()
    });
    if container.tag_name() != "DIV"
        || container.get_attribute("data-rustify-ssr").as_deref() != Some("1")
        || !valid_roots
    {
        return Err(refuse(
            UiError::InvalidHydrationRoot,
            "the generated content and overlay roots are missing or invalid",
        ));
    }
    if container.get_attribute(MOUNTED_ATTRIBUTE).as_deref() != Some(scope.as_str())
        || container.get_attribute("data-rustify-seed").as_deref() != Some(scope.as_str())
        || container.get_attribute("data-rustify-base").as_deref() != Some(config.base.as_str())
    {
        return Err(refuse(
            UiError::HydrationIdentityMismatch,
            "the generated scope, seed or base does not match the hydration request",
        ));
    }
    let url_owner = if config.url_owner {
        Some(crate::router::claim_url(&scope).ok_or_else(|| {
            refuse(
                UiError::UrlOwnerConflict,
                "another scope on this page already owns the URL",
            )
        })?)
    } else {
        None
    };
    // The structure was checked without touching the DOM, so these options are
    // now known to contain the two roots.
    let content = content.ok_or(UiError::InvalidHydrationRoot)?;
    let overlay = overlay.ok_or(UiError::InvalidHydrationRoot)?;
    container
        .set_attribute(HYDRATED_ATTRIBUTE, &scope)
        .map_err(|_| UiError::InvalidContainer)?;
    let target = content.clone().unchecked_into::<HtmlElement>();
    let roots = vec![content.clone(), overlay.clone()];
    let root_container = container.clone();
    let root_claim = url_owner.clone();
    let unmount = leptos::mount::hydrate_from(target, move || {
        // hydrate_from creates a fresh root Owner; it cannot inherit contexts
        // established around this call.
        crate::id::provide(&scope);
        crate::provide_locale(crate::Locale::English);
        provide_context(ActionSink::new());
        provide_context(ScopeRoots(
            send_wrapper::SendWrapper::new(root_container.clone()),
            config.base.clone(),
        ));
        provide_context(crate::overlay::OverlayStack::new(
            &root_container,
            &content,
            &overlay,
        ));
        crate::router::provide_router(&root_container, root_claim, &config.base);
        view()
    });
    Ok(AppHandle {
        container,
        url_owner,
        roots,
        owner: None,
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
            #[cfg(feature = "hydrate")]
            let _ = self.container.remove_attribute(HYDRATED_ATTRIBUTE);
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
