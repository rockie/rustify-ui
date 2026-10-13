use super::registry::{
    children_view, placeholder_view, prepare_environment, PreparedComponent, ViewFactory,
};
use super::{IslandMetadata, RenderTarget, SiteError, SiteRegistry, PROTOCOL_VERSION};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlElement;
use std::cell::RefCell;
use std::collections::BTreeSet;

thread_local! {
    static APPS: RefCell<Vec<rustify_ui::AppHandle>> = const { RefCell::new(Vec::new()) };
}

struct Attachment {
    root: HtmlElement,
    metadata: IslandMetadata,
    prepared: Option<PreparedComponent>,
    demo: Option<ViewFactory>,
    environment: super::registry::PreparedEnvironment,
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
            let handle = rustify_ui::hydrate(root.clone(), config, move || {
                placeholder_environment.provide();
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
            let handle = rustify_ui::mount(
                host,
                rustify_ui::MountConfig {
                    scope: seed.clone(),
                    base: metadata.environment.base,
                    url_owner: false,
                },
                move || {
                    rustify_ui::id::provide(&seed);
                    environment.provide();
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
            let handle = rustify_ui::hydrate(root, config, move || {
                environment.provide();
                rustify_components::geometry::provide_geometry_styles(Default::default());
                body()
            })
            .map_err(|error| SiteError::new("hydrate_failed", error.to_string()))?;
            apps.push(handle);
        }
    }
    APPS.with_borrow_mut(|retained| retained.extend(apps));
    Ok(())
}
