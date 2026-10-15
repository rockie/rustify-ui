extern crate self as rustify_docs;

use rustify_docs_macros::site;
use rustify_docs_syntax::{parse_registry, PageManifest};
use serde_json::{json, Value};

pub mod __private {
    pub use serde_json;
}

pub struct SiteRegistry {
    manifest: Value,
    pages: Vec<String>,
}

impl SiteRegistry {
    pub fn new(manifest: Value) -> Self {
        Self {
            manifest,
            pages: Vec::new(),
        }
    }

    pub fn page(mut self, name: &str, _render: fn()) -> Self {
        self.pages.push(name.to_string());
        self
    }
}

fn home() {}
fn home_zh() {}

fn registry() -> SiteRegistry {
    site! {
        pages {
            Home { route: "/", title: "Home" } => home,
            HomeZh { route: "/", title: "首页", lang: "zh" } => home_zh,
        }
    }
}

#[test]
fn compiled_macro_and_cold_reader_preserve_page_languages() {
    let compiled = registry();
    let cold = parse_registry(include_str!("registry.rs"))
        .expect("cold declaration")
        .manifest();
    assert_eq!(compiled.manifest, serde_json::to_value(cold).unwrap());
    assert_eq!(compiled.manifest["pages"][0]["lang"], Value::Null);
    assert_eq!(compiled.manifest["pages"][1]["lang"], "zh");
    assert_eq!(compiled.pages, ["Home", "HomeZh"]);
}

#[test]
fn a_legacy_page_manifest_defaults_to_no_declared_language() {
    let legacy = json!({
        "name": "Home",
        "route": "/",
        "title": "Home",
        "description": null,
        "function": "home"
    });
    let decoded: PageManifest = serde_json::from_value(legacy.clone()).unwrap();
    let mut expected = legacy;
    expected["lang"] = Value::Null;
    assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
}
