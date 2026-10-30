//! A consumer which imports only unstyled parts and their structural CSS.

#[cfg(target_arch = "wasm32")]
mod choices;
#[cfg(target_arch = "wasm32")]
mod fields;
#[cfg(target_arch = "wasm32")]
mod form_choices;
#[cfg(target_arch = "wasm32")]
mod lifecycle;
#[cfg(target_arch = "wasm32")]
mod selects;
#[cfg(target_arch = "wasm32")]
mod sliders;

#[cfg(target_arch = "wasm32")]
mod app {
    use leptos::prelude::*;
    use rustify_components::primitives::{
        AccessibleIcon, DirectionProvider, Portal, Slot, VisuallyHidden,
    };
    use rustify_ui::{mount, AppHandle, Direction, MountConfig};
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    thread_local! {
        static HANDLE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
    }

    #[component]
    fn Parts() -> impl IntoView {
        let direction = RwSignal::new(Direction::RightToLeft);
        let open = RwSignal::new(false);
        let requests = RwSignal::new(0);
        let child_calls = RwSignal::new(0);
        view! {
            <h1>"Unstyled DOM parts"</h1>
            <DirectionProvider direction=direction>
                <button type="button" data-testid="direction-toggle" on:click=move |_| {
                    direction.update(|value| *value = if *value == Direction::LeftToRight { Direction::RightToLeft } else { Direction::LeftToRight });
                }>"Change writing direction"</button>
                <div on:click=move |event| { if !event.default_prevented() { requests.update(|n| *n += 1); } }>
                    {Slot(leptos::html::button().attr("type", "button").attr("data-testid", "cancel-target")
                        .on(leptos::ev::click, move |event| { child_calls.update(|n| *n += 1); event.prevent_default(); })
                        .child("Cancel the default action"), leptos::attr::custom::custom_attribute("data-rustify-part", "trigger"))}
                </div>
                <output data-testid="child-calls">{move || child_calls.get()}</output>
                <output data-testid="requests">{move || requests.get()}</output>
                <button type="button" data-testid="portal-toggle" on:click=move |_| open.update(|value| *value = !*value)>"Toggle portal"</button>
                <Show when=move || open.get()>
                    <Portal><p data-testid="portal-content">"Scope-owned content"</p></Portal>
                </Show>
                <VisuallyHidden><span data-testid="hidden-text">"Screen reader detail"</span></VisuallyHidden>
                <AccessibleIcon label="Status"><svg viewBox="0 0 16 16" width="16" height="16"><circle cx="8" cy="8" r="5" fill="none" stroke="currentColor" /></svg></AccessibleIcon>
            </DirectionProvider>
            <crate::choices::Choices />
            <crate::sliders::Sliders />
            <crate::fields::Fields />
            <crate::form_choices::FormChoices />
            <crate::selects::Selects />
            <crate::selects::SelectLayers />
            <crate::lifecycle::Lifecycle />
        }
    }

    #[wasm_bindgen]
    pub fn primitives_mount() -> Result<(), JsValue> {
        let container = document()
            .get_element_by_id("app")
            .ok_or_else(|| JsValue::from_str("app container missing"))?
            .unchecked_into::<web_sys::HtmlElement>();
        let handle = mount(
            container,
            MountConfig {
                scope: "primitives".into(),
                ..MountConfig::default()
            },
            || view! { <Parts /> },
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        HANDLE.with(|held| *held.borrow_mut() = Some(handle));
        Ok(())
    }

    #[wasm_bindgen]
    pub fn primitives_dispose() {
        HANDLE.with(|held| {
            held.borrow_mut().take();
        });
    }
}
