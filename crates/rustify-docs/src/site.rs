//! Shared site registration and the private host/browser island bridge.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

mod registry;
pub use registry::{RenderContext, SiteRegistry};

#[cfg(feature = "site-ssr")]
mod ssr;
#[cfg(feature = "site-ssr")]
pub use ssr::serve_site;

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
mod hydrate;
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub use hydrate::hydrate_site;

/// Private helper and island metadata version.
pub const PROTOCOL_VERSION: u32 = 1;

/// An author-facing rendering error, associated with the request key by callers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
}

impl SiteError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            property: None,
        }
    }

    pub fn with_property(mut self, property: impl Into<String>) -> Self {
        self.property = Some(property.into());
        self
    }
}

impl fmt::Display for SiteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SiteError {}

/// The same diagnostic is used in request results and direct rendering APIs.
pub type SiteDiagnostic = SiteError;

/// An immutable theme document and the resolved initial color mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeInput {
    pub document: rustify_ui::ThemeDocument,
    pub mode: rustify_ui::ThemeMode,
    #[serde(default)]
    pub font_resolution: rustify_ui::FontResolution,
}

/// Context supplied identically to static generation and browser attachment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteEnvironment {
    #[serde(default)]
    pub base: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    #[serde(default)]
    pub theme: Option<ThemeInput>,
}

impl Default for SiteEnvironment {
    fn default() -> Self {
        Self {
            base: String::new(),
            lang: default_lang(),
            theme: None,
        }
    }
}

fn default_lang() -> String {
    "en".to_string()
}

/// The seven layout slots, matching the shared parser's normalized names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot {
    Header,
    Footer,
    SidebarTop,
    SidebarBottom,
    BeforeContent,
    AfterContent,
    NotFound,
}

/// A single process request. Each invocation writes one complete JSON response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelperRequest {
    pub protocol_version: u32,
    pub generation: String,
    pub operation: HelperOperation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[expect(
    clippy::large_enum_variant,
    reason = "One helper request carries one render batch; keep its public construction API without a separate allocation for the v2 theme environment"
)]
pub enum HelperOperation {
    Describe,
    Render {
        environment: SiteEnvironment,
        items: Vec<RenderRequest>,
    },
}

/// One keyed render in a batch, with a deterministic scope seed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderRequest {
    pub key: String,
    pub seed: String,
    pub target: RenderTarget,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RenderTarget {
    Component {
        name: String,
        #[serde(default = "empty_props")]
        props: Value,
        #[serde(default)]
        children_html: String,
    },
    Demo {
        name: String,
    },
    Page {
        name: String,
        route: String,
    },
    Slot {
        slot: Slot,
        route: String,
    },
}

fn empty_props() -> Value {
    Value::Object(Default::default())
}

/// Public result fields remain outside the helper process's rendering Owner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderResult {
    pub key: String,
    pub html: Option<String>,
    pub geometry_css: String,
    pub props: Option<Value>,
    pub client_only: bool,
    pub diagnostics: Vec<SiteDiagnostic>,
    pub demos: Vec<RenderedDemo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub markdown_intros: Vec<MarkdownIntro>,
}

/// A static Markdown introduction requested by a registered page or slot.
/// The pipeline reads and renders it; the helper never embeds source bytes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkdownIntro {
    pub key: String,
    pub path: String,
}

/// An explicit demo in a page or slot uses the same scoped renderer as a fence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderedDemo {
    pub key: String,
    pub seed: String,
    pub name: String,
    pub html: Option<String>,
    pub geometry_css: String,
    pub diagnostics: Vec<SiteDiagnostic>,
}

/// Runtime assets are exported from the framework compiled into the helper.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAsset {
    pub path: String,
    pub content: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub registry: Value,
    pub framework_release: String,
    pub runtime_protocol: u32,
    pub island_protocol: u32,
    pub resources: Vec<ResourceAsset>,
    pub fonts: Vec<FontResource>,
}

/// The locked SDK's logical font identities, without bundled font bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontResource {
    pub family: String,
    pub resource_path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelperResponse {
    pub protocol_version: u32,
    pub generation: String,
    pub result: HelperResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HelperResult {
    Describe { description: Description },
    Render { items: Vec<RenderResult> },
}

/// Escaped JSON in an island's external wrapper; no executable strings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IslandMetadata {
    pub protocol_version: u32,
    pub key: String,
    pub seed: String,
    pub target: RenderTarget,
    pub environment: SiteEnvironment,
}
