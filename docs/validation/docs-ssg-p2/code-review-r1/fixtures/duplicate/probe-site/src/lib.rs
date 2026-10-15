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
        }
    }
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn docs_hydrate() -> Result<(), wasm_bindgen::JsValue> {
    rustify_docs::hydrate_site(site())
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
thread_local! {
    static PROBE_APPS: std::cell::RefCell<Vec<rustify_ui::AppHandle>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn mount_branches() -> Result<(), wasm_bindgen::JsValue> {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    let container = document().create_element("div")?.unchecked_into::<leptos::web_sys::HtmlElement>();
    container.set_id("branch-probe");
    document().body().unwrap().append_child(&container)?;
    let previous = Owner::current();
    if let Some(previous) = &previous { previous.clone().unset(); }
    let handle = rustify_ui::mount(container, rustify_ui::MountConfig {
        scope: "branch-probe".to_string(), ..Default::default()
    }, || {
        let a = RwSignal::new(false);
        let b = RwSignal::new(false);
        view! {
            <div class="w-64 space-y-3">
                <button id="open-a" on:click=move |_| a.update(|v| *v = !*v)>"Toggle A"</button>
                <button id="open-b" on:click=move |_| b.update(|v| *v = !*v)>"Toggle B"</button>
                <Show when=move || a.get()><rustify_components::Progress test_id="bar-a" value=50.0 aria_label="A"/></Show>
                <Show when=move || b.get()><rustify_components::Progress test_id="bar-b" value=25.0 aria_label="B"/></Show>
            </div>
        }
    }).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    if let Some(previous) = previous { previous.set(); }
    PROBE_APPS.with_borrow_mut(|apps| apps.push(handle));
    Ok(())
}
