use super::registry::{
    children_view, document_mode, placeholder_view, prepare_environment, PreparedComponent,
    PreparedEnvironment, ViewFactory,
};
use super::{IslandMetadata, RenderTarget, SiteError, SiteRegistry, PROTOCOL_VERSION};
use leptos::prelude::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use leptos::web_sys::{js_sys, HtmlElement, MutationObserver, MutationObserverInit};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

thread_local! {
    static APPS: RefCell<Vec<rustify_ui::AppHandle>> = const { RefCell::new(Vec::new()) };
}

struct Attachment {
    root: HtmlElement,
    metadata: IslandMetadata,
    prepared: Option<PreparedComponent>,
    demo: Option<ViewFactory>,
    environment: PreparedEnvironment,
}

type ThemeSync = Rc<dyn Fn()>;

struct ThemeWatcher {
    observer: MutationObserver,
    _callback: Closure<dyn FnMut()>,
    _abort: rustify_ui::AbortRelease,
    _media_listener: rustify_ui::Listener,
}

impl Drop for ThemeWatcher {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}

/// Sets only the reactive context. The site's stylesheet already owns its CSS.
fn provide_browser_environment(environment: PreparedEnvironment) -> Option<ThemeSync> {
    let current = environment.provide_reactive()?;
    let root = document()
        .document_element()
        .expect("an island's document root exists");
    let media = window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
        .expect("the browser supports color-scheme media queries");
    let root_for_sync = root.clone();
    let media_for_sync = media.clone();
    let sync: ThemeSync = Rc::new(move || {
        let Some(active) = current.try_get_untracked() else {
            return;
        };
        let mode = document_mode(
            root_for_sync.get_attribute("data-theme").as_deref(),
            media_for_sync.matches(),
        );
        if active.mode != mode {
            if let Some(next) = environment.resolved(mode) {
                let _ = current.try_set(next);
            }
        }
    });
    let attribute_sync = Rc::clone(&sync);
    let callback = Closure::<dyn FnMut()>::new(move || attribute_sync());
    let observer = MutationObserver::new(callback.as_ref().unchecked_ref())
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
    // A fatal cannot call Rust cleanups. Bind the native method so aborting
    // disconnects the observer without re-entering this wasm instance.
    let disconnect = js_sys::Reflect::get(observer.as_ref(), &JsValue::from_str("disconnect"))
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error))
        .unchecked_into::<js_sys::Function>()
        .bind0(observer.as_ref())
        .unchecked_into::<js_sys::Function>();
    let abort = rustify_ui::release_on_abort(disconnect);
    let options = MutationObserverInit::new();
    options.set_attributes(true);
    options.set_attribute_filter(&js_sys::Array::of1(&JsValue::from_str("data-theme")));
    observer
        .observe_with_options(&root, &options)
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
    let media_sync = Rc::clone(&sync);
    let media_listener = rustify_ui::listen(
        media.unchecked_ref(),
        "change",
        Default::default(),
        move |_| media_sync(),
    );
    let watcher = StoredValue::new_local(ThemeWatcher {
        observer,
        _callback: callback,
        _abort: abort,
        _media_listener: media_listener,
    });
    on_cleanup(move || watcher.dispose());
    Some(sync)
}

/// Attaches the same registered views and retains all scope lifecycle handles.
/// The external loader must initialize runtime and empty hydration globals first.
pub fn hydrate_site(registry: SiteRegistry) -> Result<(), SiteError> {
    let blocks = document()
        .query_selector_all("[data-rustify-island]")
        .map_err(|_| SiteError::new("invalid_island", "cannot read island containers"))?;
    let mut attachments = Vec::new();
    let mut keys = BTreeSet::new();
    let mut seeds = BTreeSet::new();
    for index in 0..blocks.length() {
        let block = blocks
            .item(index)
            .and_then(|node| node.dyn_into::<HtmlElement>().ok())
            .ok_or_else(|| {
                SiteError::new("invalid_island", "island container is not an HTML element")
            })?;
        let json = block
            .get_attribute("data-rustify-island-metadata")
            .ok_or_else(|| SiteError::new("invalid_island", "island metadata is missing"))?;
        let metadata: IslandMetadata = serde_json::from_str(&json)
            .map_err(|error| SiteError::new("invalid_island", error.to_string()))?;
        if metadata.protocol_version != PROTOCOL_VERSION {
            return Err(SiteError::new(
                "protocol_mismatch",
                "island metadata protocol does not match this framework",
            ));
        }
        if metadata.key.is_empty()
            || metadata.seed.is_empty()
            || block.get_attribute("data-rustify-island").as_deref() != Some(&metadata.key)
            || !keys.insert(metadata.key.clone())
            || !seeds.insert(metadata.seed.clone())
        {
            return Err(SiteError::new(
                "invalid_island",
                "island key/seed must be nonempty, unique and match its container",
            ));
        }
        let root = block
            .query_selector("[data-rustify-scope]")
            .map_err(|_| SiteError::new("invalid_island", "cannot read the island's SDK root"))?
            .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            .ok_or_else(|| SiteError::new("invalid_island", "island SDK root is missing"))?;
        let (prepared, demo) = match &metadata.target {
            RenderTarget::Component { name, props, .. } => {
                (Some(registry.prepare_component(name, props.clone())?), None)
            }
            RenderTarget::Demo { name } => (None, Some(registry.prepare_demo(name)?)),
            _ => {
                return Err(SiteError::new(
                    "invalid_island",
                    "only a component or demo can be hydrated",
                ))
            }
        };
        let environment = prepare_environment(&metadata.environment)?;
        attachments.push(Attachment {
            root,
            metadata,
            prepared,
            demo,
            environment,
        });
    }

    let mut apps = Vec::new();
    let initial_syncs = Rc::new(RefCell::new(Vec::<ThemeSync>::new()));
    for attachment in attachments {
        let Attachment {
            root,
            metadata,
            prepared,
            demo,
            environment,
        } = attachment;
        let config = rustify_ui::MountConfig {
            scope: metadata.seed.clone(),
            base: metadata.environment.base.clone(),
            url_owner: false,
        };
        let is_client_only = prepared
            .as_ref()
            .is_some_and(|prepared| prepared.placeholder.is_some());
        if is_client_only {
            let placeholder = prepared
                .as_ref()
                .and_then(|prepared| prepared.placeholder.clone())
                .ok_or_else(|| {
                    SiteError::new("invalid_island", "client-only placeholder is missing")
                })?;
            let placeholder_environment = environment.clone();
            let placeholder_syncs = Rc::clone(&initial_syncs);
            let handle = rustify_ui::hydrate(root.clone(), config, move || {
                if let Some(sync) = provide_browser_environment(placeholder_environment) {
                    placeholder_syncs.borrow_mut().push(sync);
                }
                rustify_components::geometry::provide_geometry_styles(Default::default());
                placeholder_view(placeholder)
            })
            .map_err(|error| SiteError::new("hydrate_failed", error.to_string()))?;
            apps.push(handle);

            let host = root
                .query_selector("[data-rustify-client-host]")
                .map_err(|_| {
                    SiteError::new("invalid_island", "cannot read client-only mount point")
                })?
                .and_then(|element| element.dyn_into::<HtmlElement>().ok())
                .ok_or_else(|| {
                    SiteError::new("invalid_island", "client-only mount point is missing")
                })?;
            let children = match metadata.target {
                RenderTarget::Component { children_html, .. } => children_html,
                _ => String::new(),
            };
            let prepared = prepared.ok_or_else(|| {
                SiteError::new("invalid_island", "client-only renderer is missing")
            })?;
            let seed = format!("{}-client", metadata.seed);
            let client_syncs = Rc::clone(&initial_syncs);
            let handle = rustify_ui::mount(
                host,
                rustify_ui::MountConfig {
                    scope: seed.clone(),
                    base: metadata.environment.base,
                    url_owner: false,
                },
                move || {
                    rustify_ui::id::provide(&seed);
                    if let Some(sync) = provide_browser_environment(environment) {
                        client_syncs.borrow_mut().push(sync);
                    }
                    rustify_components::geometry::provide_geometry_styles(Default::default());
                    (prepared.render)(children_view(children))
                },
            )
            .map_err(|error| SiteError::new("hydrate_failed", error.to_string()))?;
            apps.push(handle);
            if let Ok(Some(message)) = root.query_selector("[data-rustify-client-message]") {
                message.remove();
            }
        } else {
            let target = metadata.target;
            let body: Box<dyn FnOnce() -> AnyView> = match (prepared, demo, target) {
                (Some(prepared), None, RenderTarget::Component { children_html, .. }) => {
                    Box::new(move || (prepared.render)(children_view(children_html)))
                }
                (None, Some(render), RenderTarget::Demo { .. }) => Box::new(move || render()),
                _ => {
                    return Err(SiteError::new(
                        "invalid_island",
                        "prepared island renderer does not match its metadata",
                    ))
                }
            };
            let body_syncs = Rc::clone(&initial_syncs);
            let handle = rustify_ui::hydrate(root, config, move || {
                if let Some(sync) = provide_browser_environment(environment) {
                    body_syncs.borrow_mut().push(sync);
                }
                rustify_components::geometry::provide_geometry_styles(Default::default());
                body()
            })
            .map_err(|error| SiteError::new("hydrate_failed", error.to_string()))?;
            apps.push(handle);
        }
    }
    APPS.with_borrow_mut(|retained| retained.extend(apps));
    // Every view first took over with its serialized SSR mode. Only a fully
    // successful attachment batch schedules the site's current mode; SDK
    // tasks stop on fatal abort, and disposed signals are ignored by sync.
    for sync in initial_syncs.take() {
        rustify_ui::defer(move || sync());
    }
    Ok(())
}
