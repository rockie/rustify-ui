use serde::{Deserialize, Serialize};

/// The normalized declaration compared by the cold reader and compiled helper.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub components: Vec<ComponentManifest>,
    pub demos: Vec<DemoManifest>,
    pub pages: Vec<PageManifest>,
    pub slots: Vec<SlotManifest>,
}

/// A component's declared properties, adapter syntax and placeholder metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentManifest {
    pub name: String,
    pub fields: Vec<FieldManifest>,
    pub adapter: String,
    pub client_only: Option<ClientOnlyManifest>,
}

/// A property's normalized Rust type and optional default expression syntax.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldManifest {
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
}

/// The literal fixed placeholder for an explicitly client-only component.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientOnlyManifest {
    pub class: String,
    pub message: String,
}

/// A demo's public name and normalized source-function path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DemoManifest {
    pub name: String,
    pub function: String,
}

/// A static page's literal metadata and normalized render-function path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageManifest {
    pub name: String,
    pub route: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub lang: Option<String>,
    pub function: String,
}

/// A documented layout slot and its normalized render-function path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotManifest {
    pub slot: Slot,
    pub function: String,
}

/// The seven slots supported by a site declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Slot {
    /// The exact identifier used in `site!` and the serializable manifest.
    pub fn key(self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Footer => "footer",
            Self::SidebarTop => "sidebar_top",
            Self::SidebarBottom => "sidebar_bottom",
            Self::BeforeContent => "before_content",
            Self::AfterContent => "after_content",
            Self::NotFound => "not_found",
        }
    }
}
