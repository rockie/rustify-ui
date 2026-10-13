#![recursion_limit = "256"]

use rustify_docs::SiteRegistry;

mod catalog;
pub mod custom;
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
            ThemeProbeDemo => crate::custom::theme_probe_demo,
        }
        pages {
            CustomPage { route: "/custom/", title: "Custom page", description: "A registered page with shared demos, layout slots and an imported theme." } => crate::custom::custom_page,
            PlainPage { route: "/plain/", title: "Static custom page" } => crate::custom::plain_page,
        }
        slots {
            header => crate::custom::header,
            footer => crate::custom::footer,
            sidebar_top => crate::custom::sidebar_top,
            sidebar_bottom => crate::custom::sidebar_bottom,
            before_content => crate::custom::before_content,
            after_content => crate::custom::after_content,
            not_found => crate::custom::not_found,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
