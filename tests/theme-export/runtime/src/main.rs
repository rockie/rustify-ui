#[cfg(target_arch = "wasm32")]
#[path = "../../generated/theme.rs"]
mod exported;

#[cfg(target_arch = "wasm32")]
mod browser {
    use crate::exported;
    use leptos::{prelude::*, wasm_bindgen::JsCast};
    use rustify_ui::{mount, AppHandle, MountConfig, ThemeScope};
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    thread_local! {
        static HANDLES: RefCell<Vec<AppHandle>> = const { RefCell::new(Vec::new()) };
    }

    #[wasm_bindgen]
    pub fn theme_export_mount() -> Result<(), JsValue> {
        if HANDLES.with(|handles| !handles.borrow().is_empty()) {
            return Err(JsValue::from_str("the export fixture is already mounted"));
        }
        let container = |id| -> Result<leptos::web_sys::HtmlElement, JsValue> {
            Ok(document()
                .get_element_by_id(id)
                .ok_or_else(|| JsValue::from_str("missing export fixture container"))?
                .dyn_into()?)
        };
        let light = exported::mount_exported_theme(container("runtime-light")?)
            .map_err(|error| JsValue::from_str(&error))?;
        let dark = exported::load_exported_theme(rustify_ui::ThemeMode::Dark)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        let dark = mount(container("runtime-dark")?, MountConfig {
            scope: "export-dark".into(), url_owner: false, base: "/".into(),
        }, move || {
            let theme = RwSignal::new(dark);
            view! {
                <ThemeScope resolved=theme />
                <main class="bg-background text-foreground font-sans p-4 rounded-lg shadow-lg" data-testid="runtime-dark-main">
                    <p data-testid="runtime-sans">"English 中文 é fi 😀"</p>
                    <p class="font-serif" data-testid="runtime-serif">"Serif 中文"</p>
                    <p class="font-mono" data-testid="runtime-mono">"const theme = true;"</p>
                    <button class="p-4 rounded-sm ring-2 ring-ring shadow-none" data-testid="runtime-ring">"Ring with no shadow"</button>
                </main>
            }
        }).map_err(|error| JsValue::from_str(&error.to_string()))?;
        HANDLES.with(|handles| handles.borrow_mut().extend([light, dark]));
        let status = document()
            .get_element_by_id("runtime-status")
            .ok_or_else(|| JsValue::from_str("missing fixture status"))?;
        status.set_text_content(Some("Mounted the exported Rust snippet and JSON."));
        status.set_attribute("data-status", "ready")?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn theme_export_dispose() {
        HANDLES.with(|handles| handles.borrow_mut().clear());
    }
}

fn main() {}
