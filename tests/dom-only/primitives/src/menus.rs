use leptos::prelude::*;
use rustify_components::primitives::navigation_menu as nav;
use rustify_components::primitives::{
    context_menu as cm, dropdown_menu as dm, menubar as mb, toolbar as tb,
};
use rustify_components::primitives::{CheckedState, DirectionProvider};
use rustify_ui::{Cancelable, Direction};

#[component]
pub fn Menus() -> impl IntoView {
    let accept = RwSignal::new(true);
    let reason_id = StoredValue::new(rustify_components::id::next("menu-reason"));
    let help_id = StoredValue::new(rustify_components::id::next("menu-help"));
    let search_error_id = StoredValue::new(rustify_components::id::next("menu-search-error"));
    let search_invalid = RwSignal::new(false);
    let empty_exports = RwSignal::new(false);
    let resources_loading = RwSignal::new(false);
    let cancel = RwSignal::new(false);
    let cancel_outside = RwSignal::new(false);
    let direction = RwSignal::new(Direction::LeftToRight);
    let open = RwSignal::new(false);
    let context = RwSignal::new(false);
    let sub = RwSignal::new(false);
    let sub_second = RwSignal::new(false);
    let sub_trigger = RwSignal::new(true);
    let toolbar_bold = RwSignal::new(true);
    let checked = RwSignal::new(CheckedState::Indeterminate);
    let radio = RwSignal::new(Some("name".to_string()));
    let requests = RwSignal::new(0_u32);
    let selections = RwSignal::new(0_u32);
    let changes = RwSignal::new(0_u32);
    let last = RwSignal::new(String::new());
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let navigation = RwSignal::new(None::<String>);
    let navigation_sub = RwSignal::new(None::<String>);
    let navigation_log = RwSignal::new(Vec::<String>::new());
    let navigation_trigger = RwSignal::new(true);
    let navigation_menu = RwSignal::new(false);
    let native_selections = RwSignal::new(0_u32);
    let bar_value = RwSignal::new(None::<String>);
    let bar_log = RwSignal::new(Vec::<String>::new());
    let bar_sub = RwSignal::new(false);
    let tools_value = RwSignal::new(vec!["italic".to_string()]);
    let tools_log = RwSignal::new(Vec::<String>::new());
    let tools_menu = RwSignal::new(false);
    let tools_disabled = RwSignal::new(false);
    let select = move |label: &'static str, event: Cancelable<()>| {
        selections.update(|n| *n += 1);
        last.set(label.into());
        if cancel.get_untracked() {
            event.prevent_default();
        }
    };
    view! {<section data-testid="menu-parts" aria-label="Document menu parts" on:keydown=move |event|{
        if !event.default_prevented() && !event.is_composing() && event.alt_key() && event.shift_key() && event.key().eq_ignore_ascii_case("n") {event.prevent_default();select("new",Cancelable::new(()));return;}
        match event.key().as_str(){"F2"=>{event.prevent_default();toolbar_bold.set(false);},"F3"=>{event.prevent_default();navigation_trigger.set(false);},"F4"=>{event.prevent_default();sub_trigger.set(false);},"F6"=>{event.prevent_default();mounted.set(false);},"F7"=>{event.prevent_default();accept.update(|value|*value=!*value);},_=>{}}
    }>
        <h2>"Document commands"</h2>
        <details><summary>"Demo controls"</summary>
        <button data-testid="menu-empty-demo" on:click=move |_|empty_exports.update(|value|*value=!*value)>"Toggle export destinations"</button>
        <button data-testid="navigation-loading-demo" on:click=move |_|resources_loading.update(|value|*value=!*value)>"Toggle reference loading"</button>
        <button data-testid="navigation-remove" on:click=move |_|navigation_trigger.update(|value|*value=!*value)>"Toggle Learn trigger"</button>
        <button data-testid="toolbar-disable" on:click=move |_|tools_disabled.update(|value|*value=!*value)>"Disable formatting"</button>
        <button data-testid="menu-accept" on:click=move |_|accept.update(|v|*v=!*v)>"Accept requests"</button>
        <button data-testid="menu-cancel" on:click=move |_|cancel.update(|v|*v=!*v)>"Keep menu after selection"</button>
        <button data-testid="menu-cancel-outside" on:click=move |_|cancel_outside.update(|v|*v=!*v)>"Keep tree on outside pointer"</button>
        <button data-testid="menu-direction" on:click=move |_|direction.update(|v|*v=if *v==Direction::LeftToRight{Direction::RightToLeft}else{Direction::LeftToRight})>"Change direction"</button>
        <button data-testid="menu-mount" on:click=move |_|mounted.update(|v|*v=!*v)>"Mount menu"</button>
        <button data-testid="menu-disable-context" on:click=move |_|disabled.update(|v|*v=!*v)>"Disable context trigger"</button>
        </details>
        <DirectionProvider direction=direction>
        <mb::Root value=bar_value on_value_change={move |value:Option<String>|{bar_log.update(|log|log.push(value.clone().unwrap_or_else(||"closed".into())));if accept.get_untracked(){bar_value.set(value);}}} aria_label="Application commands" test_id="menubar">
            <mb::Menu value="file"><mb::Trigger test_id="menubar-file">"File"</mb::Trigger>
                <mb::Portal><mb::Content test_id="menubar-file-content">
                    <mb::Item test_id="menubar-new" attr:aria-keyshortcuts="Alt+Shift+N" on_select=move |event|select("new",event)>"New document"<kbd class="menu-command-shortcut" aria-hidden="true">"Alt+Shift+N"</kbd></mb::Item>
                    <mb::Sub open=bar_sub on_open_change=move |value|if accept.get_untracked(){bar_sub.set(value);}>
                        <mb::SubTrigger test_id="menubar-sub-trigger">"Share"</mb::SubTrigger>
                        <mb::Portal><mb::SubContent test_id="menubar-sub-content"><mb::Item test_id="menubar-sub-copy" on_select=move |event|select("bar-copy",event)>"Copy link"</mb::Item></mb::SubContent></mb::Portal>
                    </mb::Sub>
                </mb::Content></mb::Portal>
            </mb::Menu>
            <mb::Menu value="unavailable"><mb::Trigger disabled=true test_id="menubar-disabled">"Review"</mb::Trigger><mb::Portal><mb::Content>"Review requires approval"</mb::Content></mb::Portal></mb::Menu>
            <mb::Menu value="edit"><mb::Trigger test_id="menubar-edit">"Edit"</mb::Trigger>
                <mb::Portal><mb::Content test_id="menubar-edit-content"><mb::Item test_id="menubar-undo" on_select=move |event|select("undo",event)>"Undo"</mb::Item></mb::Content></mb::Portal>
            </mb::Menu>
            <mb::Menu value="view"><mb::Trigger test_id="menubar-view">"View"</mb::Trigger>
                <mb::Portal><mb::Content test_id="menubar-view-content">
                    <mb::CheckboxItem test_id="menubar-details" checked=checked on_checked_change=move |value:bool|if accept.get_untracked(){checked.set(value.into());}>"Show details"</mb::CheckboxItem>
                    <mb::RadioGroup value=radio on_value_change=move |value|if accept.get_untracked(){radio.set(Some(value));}>
                        <mb::RadioItem value="name" test_id="menubar-sort-name">"By name"</mb::RadioItem>
                        <mb::RadioItem value="date" test_id="menubar-sort-date">"By date"</mb::RadioItem>
                    </mb::RadioGroup>
                </mb::Content></mb::Portal>
            </mb::Menu>
        </mb::Root>
        <nav::Root value=navigation on_value_change={move |value:Option<String>|{navigation_log.update(|log|log.push(value.clone().unwrap_or_else(||"closed".into())));if accept.get_untracked(){navigation.set(value);}}} aria_label="Document resources" test_id="navigation">
            <nav::List test_id="navigation-list">
                <nav::Item value="learn">
                    <Show when=move ||navigation_trigger.get()><nav::Trigger test_id="navigation-learn">"Learn"</nav::Trigger></Show>
                    <nav::Content force_mount=true test_id="navigation-learn-content">
                        <nav::Link href=format!("#{}",help_id.get_value()) active=true test_id="navigation-guide" on_select=Some(Callback::new(move |event:Cancelable<()>|{native_selections.update(|n|*n+=1);if cancel.get_untracked(){event.prevent_default();}}))>"Getting started"</nav::Link>
                        <nav::Link href=format!("#{}",help_id.get_value()) target="_blank" rel="noopener" test_id="navigation-new-tab">"Open guide in a new tab"</nav::Link>
                        <nav::Sub value=navigation_sub on_value_change=move |value|if accept.get_untracked(){navigation_sub.set(value);} test_id="navigation-sub">
                            <nav::List><nav::Item value="examples"><nav::Trigger test_id="navigation-examples">"Examples"</nav::Trigger>
                                <nav::Content test_id="navigation-examples-content"><nav::Link href=format!("#{}",help_id.get_value()) test_id="navigation-example-link">"Document example"</nav::Link></nav::Content>
                            </nav::Item></nav::List>
                        </nav::Sub>
                        <dm::Root open=navigation_menu on_open_change=move |value|navigation_menu.set(value)>
                            <dm::Trigger test_id="navigation-layer-trigger">"Guide actions"</dm::Trigger><dm::Portal><dm::Content test_id="navigation-layer"><dm::Item on_select=move |event|select("guide-action",event)>"Copy guide"</dm::Item></dm::Content></dm::Portal>
                        </dm::Root>
                    </nav::Content>
                </nav::Item>
                <nav::Item value="disabled"><nav::Trigger disabled=true test_id="navigation-disabled">"Upcoming"</nav::Trigger></nav::Item>
                <nav::Item value="resources"><nav::Trigger test_id="navigation-resources">"Resources"</nav::Trigger>
                    <nav::Content test_id="navigation-resources-content"><nav::Link href=format!("#{}",help_id.get_value()) test_id="navigation-reference">"API reference"</nav::Link><Show when=move ||resources_loading.get() fallback=||view!{<p>"Reference links and application examples."</p>}><p role="status" data-testid="navigation-loading">"Refreshing reference links…"</p></Show></nav::Content>
                </nav::Item>
                <nav::Item value="help"><nav::Link href=format!("#{}",help_id.get_value()) test_id="navigation-help">"Help"</nav::Link></nav::Item>
            </nav::List>
            <nav::Indicator test_id="navigation-indicator" /><nav::Viewport test_id="navigation-viewport" />
        </nav::Root>
        <tb::Root aria_label="Document formatting" test_id="toolbar">
            <tb::Button test_id="toolbar-save" on_click=Some(Callback::new(move |_|last.set("save".into())))>"Save"</tb::Button>
            <tb::Button test_id="toolbar-disabled" disabled=true>"Publish"</tb::Button>
            <tb::Separator />
            <tb::ToggleGroup value=tools_value mode=tb::SelectionMode::Multiple disabled=tools_disabled on_change={move |value:Vec<String>|{tools_log.update(|log|log.push(value.join(",")));if accept.get_untracked(){tools_value.set(value);}}}>
                <Show when=move ||toolbar_bold.get()><tb::ToggleItem value="bold" test_id="toolbar-bold">"Bold"</tb::ToggleItem></Show>
                <tb::ToggleItem value="italic" test_id="toolbar-italic">"Italic"</tb::ToggleItem>
            </tb::ToggleGroup>
            <dm::Root open=tools_menu on_open_change=move |value|if accept.get_untracked(){tools_menu.set(value);}>
                {dm::trigger_as(leptos::html::button().attr("data-testid","toolbar-menu").add_any_attr(tb::item(false.into())).child("More"))}
                <dm::Portal><dm::Content test_id="toolbar-menu-content"><Show when=move ||empty_exports.get() fallback=move ||view!{<dm::Item on_select=move |event|select("export",event)>"Export"</dm::Item>}><dm::Item disabled=true test_id="toolbar-empty" on_select=|_|{}>"No export destinations"</dm::Item></Show></dm::Content></dm::Portal>
            </dm::Root>
            <tb::Link href=format!("#{}",help_id.get_value()) test_id="toolbar-link">"Help"</tb::Link>
            <input type="search" aria-label="Find in document" data-testid="toolbar-search" aria-invalid=move ||search_invalid.get().to_string() aria-describedby=move ||search_invalid.get().then(||search_error_id.get_value()) on:input=move |event|search_invalid.set(event_target_value(&event).trim().chars().count()==1) />
        </tb::Root>
        <Show when=move ||search_invalid.get()><p id=search_error_id.get_value() role="alert" data-testid="toolbar-search-error">"Use at least two characters."</p></Show>
        <tb::Root aria_label="Document history" orientation=rustify_components::Orientation::Vertical test_id="toolbar-vertical">
            <tb::Button test_id="toolbar-back">"Back"</tb::Button><tb::Separator test_id="toolbar-vertical-separator" /><tb::Button test_id="toolbar-forward">"Forward"</tb::Button>
        </tb::Root>
        <Show when=move ||mounted.get()>
            <dm::Root open=open on_open_change=move |value|{requests.update(|n|*n+=1);if accept.get_untracked(){open.set(value);}}>
                <dm::Trigger test_id="menu-trigger">"Document actions"</dm::Trigger>
                <dm::Portal><dm::Content test_id="menu-content" aria_label="Document commands"
                    on_pointer_down_outside=Some(Callback::new(move |event:Cancelable<_>|if cancel_outside.get_untracked(){event.prevent_default();}))>
                    <dm::Group><dm::Label>"Document"</dm::Label>
                        <dm::Item test_id="menu-alpha" on_select=move |e|select("alpha",e)>"Alpha"</dm::Item>
                        <dm::Item test_id="menu-disabled" disabled=true described_by=reason_id.get_value() on_select=move |e|select("disabled",e)>
                            "Publish"<span id=reason_id.get_value()>"Requires approval"</span>
                        </dm::Item>
                        <dm::Item test_id="menu-archive" on_select=move |e|select("archive",e)>"Archive"</dm::Item>
                    </dm::Group>
                    <dm::Separator />
                    <dm::CheckboxItem test_id="menu-checked" checked=checked on_select=Some(Callback::new(move |e|select("checked",e)))
                        on_checked_change=move |value|{changes.update(|n|*n+=1);if accept.get_untracked(){checked.set(if value{CheckedState::Checked}else{CheckedState::Unchecked});}}>"Show details"</dm::CheckboxItem>
                    <dm::RadioGroup value=radio on_value_change=move |value|{changes.update(|n|*n+=1);if accept.get_untracked(){radio.set(Some(value));}}>
                        <dm::RadioItem value="name" test_id="menu-radio-name">"By name"</dm::RadioItem>
                        <dm::RadioItem value="date" test_id="menu-radio-date">"By date"</dm::RadioItem>
                    </dm::RadioGroup>
                    <dm::Sub open=sub on_open_change=move |value|{requests.update(|n|*n+=1);if accept.get_untracked(){sub.set(value);}}>
                        <Show when=move ||sub_trigger.get()><dm::SubTrigger test_id="menu-sub-trigger">"Share"</dm::SubTrigger></Show>
                        <dm::Portal><dm::SubContent test_id="menu-sub-content">
                            <dm::Item test_id="menu-sub-copy" on_select=move |e|select("copy-link",e)>"Copy link"</dm::Item>
                            <dm::Item test_id="menu-sub-email" on_select=move |e|select("email",e)>"Email"</dm::Item>
                            <dm::Sub open=sub_second on_open_change=move |value|if accept.get_untracked(){sub_second.set(value);}>
                                <dm::SubTrigger test_id="menu-second-trigger">"Advanced"</dm::SubTrigger>
                                <dm::Portal><dm::SubContent test_id="menu-second-content"><dm::Item test_id="menu-second-action" on_select=move |event|select("advanced",event)>"Copy public link"</dm::Item></dm::SubContent></dm::Portal>
                            </dm::Sub>
                        </dm::SubContent></dm::Portal>
                    </dm::Sub>
                </dm::Content></dm::Portal>
            </dm::Root>
        </Show>
        <cm::Root open=context on_open_change=move |value|{requests.update(|n|*n+=1);if accept.get_untracked(){context.set(value);}}>
            <cm::Trigger test_id="context-trigger" disabled=disabled>"Document preview: right click or Shift+F10"</cm::Trigger>
            <cm::Portal><cm::Content test_id="context-content">
                <cm::Item test_id="context-copy" on_select=move |e|select("context-copy",e)>"Copy document"</cm::Item>
                <cm::Item test_id="context-delete" on_select=move |e|select("context-delete",e)>"Remove document"</cm::Item>
            </cm::Content></cm::Portal>
        </cm::Root>
        </DirectionProvider>
        <button data-testid="menu-outside">"Continue editing"</button>
        <details><summary>"Request results"</summary>
        <output data-testid="navigation-log">{move ||navigation_log.get().join(",")}</output>
        <output data-testid="navigation-value">{move ||navigation.get().unwrap_or_default()}</output>
        <output data-testid="navigation-selections">{move ||native_selections.get()}</output>
        <output data-testid="menubar-log">{move ||bar_log.get().join(",")}</output>
        <output data-testid="menubar-value">{move ||bar_value.get().unwrap_or_default()}</output>
        <output data-testid="toolbar-log">{move ||tools_log.get().join(";")}</output>
        <output data-testid="toolbar-value">{move ||tools_value.get().join(",")}</output>
        <output data-testid="menu-requests">{move ||requests.get()}</output>
        <output data-testid="menu-selections">{move ||selections.get()}</output>
        <output data-testid="menu-changes">{move ||changes.get()}</output>
        <output data-testid="menu-last">{move ||last.get()}</output>
        <output data-testid="menu-open">{move ||open.get().to_string()}</output>
        <output data-testid="menu-sub-open">{move ||sub.get().to_string()}</output>
        </details>
        <p id=help_id.get_value()>"Document help"</p>
    </section>}
}

use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
thread_local! {static HANDLE:RefCell<Option<rustify_ui::AppHandle>>=const{RefCell::new(None)};}
#[wasm_bindgen]
pub fn menus_mount() -> Result<(), JsValue> {
    menus_mount_in("menu-benchmark")
}
#[wasm_bindgen]
pub fn menus_mount_in(container_id: &str) -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("menu container missing"))?
        .unchecked_into();
    let handle = rustify_ui::mount(
        container,
        rustify_ui::MountConfig {
            scope: container_id.into(),
            ..Default::default()
        },
        || view! {<Menus />},
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    Ok(())
}
#[wasm_bindgen]
pub fn menus_dispose() {
    HANDLE.with(|slot| {
        slot.borrow_mut().take();
    });
}

#[wasm_bindgen]
pub fn menus_trap() {
    unreachable!("deliberate menu instance trap");
}
