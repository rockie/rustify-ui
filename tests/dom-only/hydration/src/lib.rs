//! One deterministic component view shared by independent SSR, hydrate and CSR builds.

#[cfg(any(
    all(feature = "ssr", feature = "hydrate"),
    all(feature = "ssr", feature = "csr"),
    all(feature = "hydrate", feature = "csr"),
    not(any(feature = "ssr", feature = "hydrate", feature = "csr"))
))]
compile_error!("select exactly one of ssr, hydrate or csr");

#[cfg(feature = "ssr")]
use leptos::prelude::*;
use rustify_ui::MountConfig;

mod form_parts;
mod view;

#[cfg(target_arch = "wasm32")]
mod browser;

#[cfg(all(target_arch = "wasm32", feature = "csr"))]
mod geometry_branches;

/// The same seeds in all three separately compiled rendering modes.
pub const SCOPES: [&str; 3] = ["catalog-a", "catalog-b", "catalog-c"];

fn config(scope: String) -> MountConfig {
    MountConfig {
        scope,
        url_owner: false,
        base: String::new(),
    }
}

/// Renders the SDK root and collects its initial geometry into an external sheet.
#[cfg(feature = "ssr")]
pub fn render_scope(scope: &str) -> (String, String) {
    use rustify_components::geometry::{provide_geometry_styles, GeometryStyles};

    let owner = Owner::new();
    let styles = GeometryStyles::default();
    let scope = scope.to_string();
    let html = owner.with(|| {
        rustify_ui::ssr_scope(config(scope.clone()), || {
            provide_geometry_styles(styles.clone());
            view! { <view::CatalogView scope=scope /> }
        })
        .into_view()
        .to_html()
    });
    (html, styles.css())
}
