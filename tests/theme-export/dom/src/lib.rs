#[cfg(target_arch = "wasm32")]
#[path = "../../generated/theme.rs"]
mod exported;

#[cfg(target_arch = "wasm32")]
mod browser {
    use crate::exported;
    use leptos::{prelude::*, wasm_bindgen::JsCast};
    use rustify_components::{Button, ButtonVariant};
    use rustify_ui::{mount, theme::*, AppHandle, MountConfig, ThemeScope};
    use std::{cell::RefCell, sync::Arc};
    use wasm_bindgen::prelude::*;

    thread_local! {
        static HANDLES: RefCell<Vec<AppHandle>> = const { RefCell::new(Vec::new()) };
    }

    fn container(id: &str) -> Result<leptos::web_sys::HtmlElement, JsValue> {
        document()
            .get_element_by_id(id)
            .ok_or_else(|| JsValue::from_str("missing DOM export container"))?
            .dyn_into()
            .map_err(Into::into)
    }

    #[wasm_bindgen]
    pub fn dom_export_mount(base: &str) -> Result<(), JsValue> {
        if HANDLES.with(|handles| !handles.borrow().is_empty()) {
            return Err(JsValue::from_str("the DOM consumer is already mounted"));
        }
        // This function is compiled verbatim from the file offered by Studio.
        let snippet = exported::mount_exported_theme(container("dom-snippet")?)
            .map_err(|error| JsValue::from_str(&error))?;
        HANDLES.with(|handles| handles.borrow_mut().push(snippet));

        for (id, mode, preview) in [
            ("dom-json", ThemeMode::Light, false),
            ("dom-preview-light", ThemeMode::Light, true),
            ("dom-preview-dark", ThemeMode::Dark, true),
        ] {
            if preview {
                let handle = exported::mount_exported_preview(
                    container(id)?,
                    MountConfig {
                        scope: id.into(),
                        url_owner: false,
                        base: base.into(),
                    },
                    mode,
                    &font_context(16., false),
                )
                .map_err(|error| JsValue::from_str(&error))?;
                HANDLES.with(|handles| handles.borrow_mut().push(handle));
                continue;
            }
            // The JSON path also consumes the downloaded file, without Studio state.
            let context = font_context(16., false);
            let document = import_json(include_str!("../../generated/theme.json"), &context)
                .map_err(|error| JsValue::from_str(&error.to_string()))?
                .document;
            let initial = Arc::new(
                resolve(&document, mode, &context)
                    .map_err(|error| JsValue::from_str(&error.to_string()))?,
            );
            let handle = mount(
                container(id)?,
                MountConfig { scope: id.into(), url_owner: false, base: base.into() },
                move || {
                    let theme = RwSignal::new(initial);
                    view! {
                        <ThemeScope resolved=theme />
                        <main class="sample bg-background text-foreground font-sans p-4 rounded-lg shadow-lg">
                            <h2>{id}</h2>
                            <p class="font-sans">"English 中文 é fi 😀"</p>
                            <p class="font-serif">"Serif 中文"</p>
                            <p class="font-mono">"const theme = true;"</p>
                            <Button variant=ButtonVariant::Solid on_click=|| {}>"Continue"</Button>
                        </main>
                    }
                },
            ).map_err(|error| JsValue::from_str(&error.to_string()))?;
            HANDLES.with(|handles| handles.borrow_mut().push(handle));
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn dom_export_dispose() {
        HANDLES.with(|handles| handles.borrow_mut().clear());
    }
}
