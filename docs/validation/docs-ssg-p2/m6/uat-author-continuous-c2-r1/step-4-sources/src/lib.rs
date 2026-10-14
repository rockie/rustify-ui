use rustify_docs::SiteRegistry;

pub mod demos;

pub fn site() -> SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 2, label: String = "Counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
        }
        demos {
            CounterDemo => crate::demos::counter_demo,
            DoubleCounterDemo => crate::demos::double_counter_demo,
        }
        pages {
            ExamplesPage { route: "/examples/", title: "Examples" } => crate::demos::examples_page,
        }
        slots {
            footer => crate::demos::footer,
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
