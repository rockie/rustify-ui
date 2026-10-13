#![recursion_limit = "256"]

use rustify_docs::SiteRegistry;

mod catalog;
pub mod demos;

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 2, label: String = "Counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
            #[client_only(class = "min-h-48", message = "This counter starts in the browser.")]
            BrowserCounter { initial: i32 = 2, label: String = "Browser counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
        }
        demos {
            CounterDemo => crate::demos::counter_demo,
            AsyncAfterAttachDemo => crate::demos::async_after_attach_demo,
            FailureDemo => crate::demos::failure_demo,
            CatalogProxy => crate::demos::catalog_proxy,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
