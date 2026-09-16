use crate::{app::Editor, icons::icon};
use leptos::prelude::*;
use rustify_ui::RegionState;

#[component]
pub fn LeftPanel(editor: Editor) -> impl IntoView {
    let search = NodeRef::<leptos::html::Input>::new();
    let toggle_search = move |_| {
        let mut open = false;
        editor.shell.update(|shell| {
            shell.search_open = !shell.search_open;
            open = shell.search_open;
            if !open {
                shell.search.clear();
            }
        });
        if open {
            crate::browser_frame::request_animation_frame(move || {
                if let Some(input) = search.try_get().flatten() {
                    let _ = input.focus();
                }
            });
        }
    };
    view! {<aside class="left-panel" id="left-panel" aria-label="Pages and layers">
        <div class="left-heading"><span>"Design file"</span><button class="icon-button small" id="search-button" title="Search layers" aria-label="Search layers" on:click=toggle_search>{icon("search",18)}</button></div>
        <div class="left-tabs"><button class:active=move||editor.shell.with(|s|s.left_tab=="layers") data-left-tab="layers" on:click=move |_|editor.shell.update(|s|s.left_tab="layers".into())>"Layers"</button><button class:active=move||editor.shell.with(|s|s.left_tab=="assets") data-left-tab="assets" on:click=move |_|editor.shell.update(|s|s.left_tab="assets".into())>"Assets"</button><button class="icon-button small" id="collapse-all" title="Collapse layers" aria-label="Collapse layers" on:click=move |_|editor.shell.update(|s|s.expanded.clear())>{icon("collapse",18)}</button></div>
        <div class="layer-search" class:hidden=move||!editor.shell.with(|s|s.search_open) id="layer-search">{icon("search",18)}<input id="search-layers" node_ref=search placeholder="Find a layer…" aria-label="Find a layer" prop:value=move||editor.shell.with(|s|s.search.clone()) on:input=move|event|editor.shell.update(|s|s.search=event_target_value(&event))/></div>
        <super::pages::Pages editor/>
        <div class="layers-heading"><span id="tree-heading">{move||editor.shell.with(|s|if s.left_tab=="layers"{"Layers"}else{"Local components"})}</span><span class="muted" id="layer-count">{move||editor.doc.with(|doc|doc.nodes().len())}</span></div>
        <super::layer_tree::LayerTree editor/>
        <super::assets::Assets editor/>
        <div class="left-footer"><div class="engine-dot"></div><span id="engine-label">{move||if editor.blank_badges.get(){"Renderer"}else if editor.nogpu||matches!(editor.region_state.get(),RegionState::Failed(_)){"GPU unavailable"}else if matches!(editor.region_state.get(),RegionState::Lost){"Restoring"}else{"Makepad WebGL2"}}</span><button class="icon-button small" id="settings" title="Editor settings" aria-label="Editor settings" on:click=move |_|super::dispatch(editor,"settings")>{icon("sliders",18)}</button></div>
    </aside>}
}
