#[cfg(target_arch = "wasm32")]
mod app {
    use leptos::prelude::*;
    use rustify_components::{
        alert_dialog as confirm, callout, context_menu as context, dropdown_menu as menu,
        menubar as bar, navigation_menu as nav, popover, toolbar, Select, SelectOption, Toaster,
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
    fn NotePanel(note: RwSignal<String>) -> impl IntoView {
        let confirm_open = RwSignal::new(false);
        let settings_open = RwSignal::new(false);
        let select_open = RwSignal::new(false);
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

    #[component]
    fn ApplicationShell() -> impl IntoView {
        let note = RwSignal::new("Review the delivery address".to_string());
        let saved_note = RwSignal::new(note.get_untracked());
        let command = RwSignal::new(None::<String>);
        let section = RwSignal::new(None::<String>);
        let more = RwSignal::new(false);
        let context_open = RwSignal::new(false);
        let status = RwSignal::new("Ready to review your note.");
        let save = move || {
            saved_note.set(note.get_untracked());
            status.set("Note saved for this session.");
        };
        let restore = move || {
            note.set(saved_note.get_untracked());
            status.set("Saved note restored.");
        };

        view! {
            <div class="application-shell">
                <header class="shell-header">
                    <p class="shell-title">"Delivery desk"</p>
                    <bar::Root value=command on_value_change=move |next| command.set(next)
                        aria_label="Application commands">
                        <bar::Menu value="file"><bar::Trigger>"File"</bar::Trigger>
                            <bar::Portal><bar::Content aria_label="File commands">
                                <bar::Item on_select=move |_| save()>"Save note"</bar::Item>
                            </bar::Content></bar::Portal>
                        </bar::Menu>
                        <bar::Menu value="edit"><bar::Trigger>"Edit"</bar::Trigger>
                            <bar::Portal><bar::Content aria_label="Edit commands">
                                <bar::Item on_select=move |_| restore()>"Restore saved note"</bar::Item>
                            </bar::Content></bar::Portal>
                        </bar::Menu>
                    </bar::Root>
                </header>
                <nav::Root value=section on_value_change=move |next| section.set(next)
                    aria_label="Delivery workspace">
                    <nav::List>
                        <nav::Item value="note"><nav::Link href="#note" active=true>"Current note"</nav::Link></nav::Item>
                        <nav::Item value="help"><nav::Trigger>"Help"</nav::Trigger>
                            <nav::Content>
                                <nav::Link href="#delivery-help">"Delivery guide"</nav::Link>
                            </nav::Content>
                        </nav::Item>
                    </nav::List>
                    <nav::Indicator />
                    <nav::Viewport />
                </nav::Root>
                <div class="shell-tools">
                    <toolbar::Root aria_label="Note tools">
                        <toolbar::Button on_click=Some(Callback::new(move |_| save()))>"Save note"</toolbar::Button>
                        <toolbar::Separator />
                        <menu::Root open=more on_open_change=move |next| more.set(next)>
                            {menu::trigger_as(leptos::html::button().r#type("button")
                                .class("shell-menu-trigger").child("More")
                                .add_any_attr(toolbar::item(false.into())))}
                            <menu::Portal><menu::Content aria_label="More note commands">
                                <menu::Item on_select=move |_| restore()>"Restore saved note"</menu::Item>
                            </menu::Content></menu::Portal>
                        </menu::Root>
                    </toolbar::Root>
                </div>
                <div id="note"><NotePanel note=note /></div>
                <context::Root open=context_open on_open_change=move |next| context_open.set(next)>
                    <context::Trigger class="note-context">
                        <strong>"Saved note"</strong>
                        <p>{move || if saved_note.get().is_empty() {
                            "Your saved note is empty.".to_string()
                        } else {
                            saved_note.get()
                        }}</p>
                    </context::Trigger>
                    <context::Portal><context::Content aria_label="Saved note commands">
                        <context::Item on_select=move |_| restore()>"Restore saved note"</context::Item>
                        <context::Item on_select=move |_| save()>"Replace with current note"</context::Item>
                    </context::Content></context::Portal>
                </context::Root>
                <p role="status" class="shell-status">{move || status.get()}</p>
                <section id="delivery-help" class="delivery-help" aria-labelledby="delivery-help-heading">
                    <h2 id="delivery-help-heading">"Delivery guide"</h2>
                    <p>"Choose Standard or Express in Delivery options. Save keeps a copy of your note for this session; restore brings that copy back. Reloading starts a new session."</p>
                </section>
            </div>
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
            || view! { <ApplicationShell /> },
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
        Ok(())
    }
}
