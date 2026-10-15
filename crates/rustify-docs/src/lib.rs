//! Rustify Docs: Markdown content rendered on the host into a static
//! documentation site.
//!
//! Every page is complete HTML before any script runs, rendered with the
//! SDK's components through Leptos on the host. Content sites use the
//! `pipeline` feature; component sites share a `site!` registry through
//! `site-ssr` for the build helper and `hydrate` for browser interaction.
//! A page without interactive islands downloads no wasm.

// A page's view is one nested Leptos type; erasing it (`into_any`) needs
// more query depth than the default allows.
#![recursion_limit = "256"]

extern crate self as rustify_docs;

#[cfg(all(feature = "site-ssr", feature = "hydrate"))]
compile_error!("build rustify-docs site-ssr and hydrate as separate targets; disable default features for an island");

pub use rustify_docs_macros::site;

pub mod diagnostic_text;

#[cfg(any(feature = "site-ssr", feature = "hydrate"))]
pub mod site;
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub use site::hydrate_site;
#[cfg(feature = "site-ssr")]
pub use site::serve_site;
#[cfg(any(feature = "site-ssr", feature = "hydrate"))]
pub use site::{RenderContext, SiteRegistry, Slot};

/// Dependencies used by the generated declaration; authors need not name
/// or independently version them.
#[doc(hidden)]
pub mod __private {
    pub use leptos;
    pub use serde;
    pub use serde_json;
}

#[cfg(feature = "pipeline")]
pub mod autolink;
#[cfg(feature = "pipeline")]
pub mod builtins;
#[cfg(feature = "pipeline")]
pub mod classes;
#[cfg(feature = "pipeline")]
pub mod color;
#[cfg(feature = "pipeline")]
pub mod config;
#[cfg(feature = "pipeline")]
pub mod content;
#[cfg(feature = "pipeline")]
pub mod demo;
#[cfg(feature = "pipeline")]
pub mod diagnostic;
#[cfg(feature = "pipeline")]
pub mod embed;
#[cfg(feature = "pipeline")]
pub mod escape;
#[cfg(feature = "pipeline")]
pub mod frontmatter;
#[cfg(feature = "pipeline")]
pub mod generation;
#[cfg(feature = "pipeline")]
pub mod highlight;
#[cfg(feature = "pipeline")]
pub mod history;
#[cfg(feature = "pipeline")]
pub mod html_check;
#[cfg(feature = "pipeline")]
pub mod icons;
#[cfg(feature = "pipeline")]
pub mod imagesize;
#[cfg(feature = "pipeline")]
pub mod include;
#[cfg(feature = "pipeline")]
pub mod labels;
#[cfg(feature = "pipeline")]
pub mod markdown;
#[cfg(feature = "pipeline")]
pub mod publish;
#[cfg(feature = "pipeline")]
pub mod shell;
#[cfg(feature = "pipeline")]
pub mod slug;
#[cfg(feature = "pipeline")]
pub mod style;
