//! Rustify Docs: Markdown content rendered on the host into a static
//! documentation site.
//!
//! Every page is complete HTML before any script runs, rendered with the
//! SDK's components through Leptos on the host. The site's one stylesheet is
//! compiled in process from the classes its pages use, code is highlighted
//! into classes at build time, and a reader's browser downloads no wasm.
//! The `rustify-docs` command (the `rustify-docs-cli` package) drives this
//! library; nothing here compiles a site's Rust, because a content site has
//! none.

pub mod autolink;
pub mod classes;
pub mod color;
pub mod config;
pub mod content;
pub mod diagnostic;
pub mod escape;
pub mod frontmatter;
pub mod generation;
pub mod highlight;
pub mod history;
pub mod html_check;
pub mod include;
pub mod labels;
pub mod markdown;
pub mod publish;
pub mod shell;
pub mod slug;
pub mod style;
