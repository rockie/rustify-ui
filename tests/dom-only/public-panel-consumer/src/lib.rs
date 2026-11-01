#[cfg(target_arch = "wasm32")]
mod app {
    use leptos::prelude::*;
    use rustify_components::{
        alert_dialog as confirm, callout, popover, Select, SelectOption, Toaster,
    };
    use rustify_ui::toast::{provide_toasts, ToastOptions, ToastTone};
    use rustify_ui::{mount, AppHandle, MountConfig};
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    thread_local! {
        static HANDLE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
    }

    #[component]
    fn NotePanel() -> impl IntoView {
        let confirm_open = RwSignal::new(false);
        let settings_open = RwSignal::new(false);
        let select_open = RwSignal::new(false);
        let note = RwSignal::new("Review the delivery address".to_string());
        let delivery = RwSignal::new("standard".to_string());
        let result = RwSignal::new("The note has not been cleared.".to_string());
        let choices = Signal::stored(vec![
            SelectOption::new("standard", "Standard"),
            SelectOption::new("express", "Express"),
        ]);
        let notices = provide_toasts();

        view! {
            <section class="note-panel" aria-labelledby="note-panel-heading">
                <h1 id="note-panel-heading">"Delivery note"</h1>
                <p>"Review a local note and choose its delivery speed."</p>
                <label class="note-field">"Note"
                    <input aria-label="Note" prop:value=move || note.get()
                        on:input=move |event| note.set(event_target_value(&event)) />
                </label>
                <div class="note-actions">
                    <confirm::Root open=confirm_open
                        on_open_change=move |next| confirm_open.set(next)>
                        <confirm::Trigger>"Clear note"</confirm::Trigger>
                        <confirm::Portal>
                            <confirm::Overlay />
                            <confirm::Content>
                                <confirm::Title>"Clear this note?"</confirm::Title>
                                <confirm::Description>
                                    "The local note will be emptied. Delivery stays unchanged."
                                </confirm::Description>
                                <div class="note-actions">
                                    <confirm::Cancel>"Keep note"</confirm::Cancel>
                                    <confirm::Action on_action=move |_| {
                                        note.set(String::new());
                                        result.set("The local note was cleared.".to_string());
                                        notices.show("The local note was cleared.", ToastOptions {
                                            tone: ToastTone::Success,
                                            ..Default::default()
                                        });
                                    }>"Clear note"</confirm::Action>
                                </div>
                            </confirm::Content>
                        </confirm::Portal>
                    </confirm::Root>
                    <popover::Root open=settings_open
                        on_open_change=move |next| settings_open.set(next)>
                        <popover::Trigger>"Delivery options"</popover::Trigger>
                        <popover::Portal>
                            <popover::Content aria_label="Delivery options">
                                <div class="delivery-options">
                                    <Select value=delivery options=choices
                                        on_change=move |next| delivery.set(next)
                                        open=select_open on_open_change=move |next| select_open.set(next)
                                        aria_label="Delivery speed" />
                                    <popover::Close>"Done"</popover::Close>
                                </div>
                            </popover::Content>
                        </popover::Portal>
                    </popover::Root>
                </div>
                <callout::Root role=Some(callout::CalloutRole::Status)>
                    <callout::Text>{move || result.get()}</callout::Text>
                </callout::Root>
                <p>"Accepted delivery: "<output>{move || delivery.get()}</output></p>
                <Toaster />
            </section>
        }
    }

    #[wasm_bindgen]
    pub fn public_panel_mount(container_id: &str) -> Result<(), JsValue> {
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| JsValue::from_str("container is not an HTML element"))?;
        let handle = mount(
            container,
            MountConfig {
                scope: "public-panel".to_string(),
                url_owner: false,
                base: String::new(),
            },
            || view! { <NotePanel /> },
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
        Ok(())
    }
}
