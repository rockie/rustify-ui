use leptos::prelude::*;
use rustify_components::primitives;
use rustify_components::{context_menu, dropdown_menu, menubar, Glyph, Icon};

/// A dropdown menu runs commands, keeps checkbox state and nests a submenu.
pub fn dropdown_menu_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let sub = RwSignal::new(false);
    let grid = RwSignal::new(false);
    let action = RwSignal::new("None".to_string());
    view! {
        <div class="space-y-4">
            <dropdown_menu::Root open=open on_open_change=move |next| open.set(next)>
                <dropdown_menu::Trigger class="rustify-dialog-button gap-2">"Document actions"<Icon glyph=Glyph::ChevronDown label="" /></dropdown_menu::Trigger>
                <dropdown_menu::Portal>
                    <dropdown_menu::Content class="rounded-md border border-border bg-background shadow-none" aria_label="File actions">
                        <dropdown_menu::Label>"Actions"</dropdown_menu::Label>
                        <dropdown_menu::Item on_select=move |_| action.set("Copy".to_string())>"Copy"</dropdown_menu::Item>
                        <dropdown_menu::Item on_select=move |_| action.set("Rename".to_string())>"Rename"</dropdown_menu::Item>
                        <dropdown_menu::Separator />
                        <dropdown_menu::CheckboxItem checked=Signal::derive(move || primitives::CheckedState::from(grid.get()))
                            on_checked_change=move |checked| grid.set(checked)>"Show grid"</dropdown_menu::CheckboxItem>
                        <dropdown_menu::Sub open=sub on_open_change=move |next| sub.set(next)>
                            <dropdown_menu::SubTrigger>"Share"</dropdown_menu::SubTrigger>
                            <dropdown_menu::Portal>
                                <dropdown_menu::SubContent class="rounded-md border border-border bg-background shadow-none">
                                    <dropdown_menu::Item on_select=move |_| action.set("Copy link".to_string())>"Copy link"</dropdown_menu::Item>
                                    <dropdown_menu::Item on_select=move |_| action.set("Email file".to_string())>"Email file"</dropdown_menu::Item>
                                </dropdown_menu::SubContent>
                            </dropdown_menu::Portal>
                        </dropdown_menu::Sub>
                        <dropdown_menu::Separator />
                        <dropdown_menu::Item disabled=true on_select=move |_| ()>"Delete (unavailable)"</dropdown_menu::Item>
                    </dropdown_menu::Content>
                </dropdown_menu::Portal>
            </dropdown_menu::Root>
            <p class="text-sm">"Menu state: "
                <output data-demo-state="">{move || format!("grid {}, action {}", if grid.get() { "on" } else { "off" }, action.get())}</output>
            </p>
        </div>
    }
}

/// A context menu opens on right-click, long-press or Shift+F10 on its
/// surface.
pub fn context_menu_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let action = RwSignal::new("None".to_string());
    view! {
        <div class="space-y-4">
            <context_menu::Root open=open on_open_change=move |next| open.set(next)>
                <context_menu::Trigger class="flex h-24 w-full max-w-md items-center justify-center rounded-md border border-dashed border-border text-sm">
                    "Right-click, long-press or Shift+F10 on this surface"
                </context_menu::Trigger>
                <context_menu::Portal>
                    <context_menu::Content class="rounded-md border border-border bg-background shadow-none">
                        <context_menu::Item on_select=move |_| action.set("Pin".to_string())>"Pin"</context_menu::Item>
                        <context_menu::Item on_select=move |_| action.set("Duplicate".to_string())>"Duplicate"</context_menu::Item>
                        <context_menu::Separator />
                        <context_menu::Item disabled=true on_select=move |_| ()>"Delete (unavailable)"</context_menu::Item>
                    </context_menu::Content>
                </context_menu::Portal>
            </context_menu::Root>
            <p class="text-sm">"Last command: "<output data-demo-state="">{move || action.get()}</output></p>
        </div>
    }
}

/// A menubar switches between menus with the arrow keys and runs commands.
pub fn menubar_demo() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    let command = RwSignal::new("None".to_string());
    view! {
        <div class="space-y-4">
            <menubar::Root value=value on_value_change=move |next| value.set(next) aria_label="Example menubar">
                <menubar::Menu value="file">
                    <menubar::Trigger>"File"</menubar::Trigger>
                    <menubar::Portal>
                        <menubar::Content class="rounded-md border border-border bg-background shadow-none" aria_label="File commands">
                            <menubar::Item on_select=move |_| command.set("New project".to_string())>"New project"</menubar::Item>
                            <menubar::Item on_select=move |_| command.set("Open folder".to_string())>"Open folder"</menubar::Item>
                        </menubar::Content>
                    </menubar::Portal>
                </menubar::Menu>
                <menubar::Menu value="view">
                    <menubar::Trigger>"View"</menubar::Trigger>
                    <menubar::Portal>
                        <menubar::Content class="rounded-md border border-border bg-background shadow-none" aria_label="View commands">
                            <menubar::Item on_select=move |_| command.set("Toggle sidebar".to_string())>"Toggle sidebar"</menubar::Item>
                            <menubar::Item disabled=true on_select=move |_| ()>"Zen mode (unavailable)"</menubar::Item>
                        </menubar::Content>
                    </menubar::Portal>
                </menubar::Menu>
            </menubar::Root>
            <p class="text-sm">"Menubar: "
                <output data-demo-state="">{move || format!("open {}, last {}", value.get().unwrap_or_else(|| "none".into()), command.get())}</output>
            </p>
            <p class="text-sm text-muted-foreground">"Arrow keys move between menus and their items; Escape closes and focus returns to the bar."</p>
        </div>
    }
}
