use std::collections::BTreeSet;
use std::sync::Arc;

use leptos::prelude::*;
use rustify_components::data_table::Cell;
use rustify_components::{
    Button, ButtonVariant, Column, DataTable, ScrollArea, Tab, TabPanel, Tabs, Tree, TreeNode,
};
use rustify_ui::Selection;

/// Tabs retain their panels while the active value belongs to the application.
pub fn tabs_demo() -> impl IntoView {
    let active = RwSignal::new("overview".to_string());
    view! {
        <div class="space-y-4">
            <Tabs active=active on_activate=move |value| active.set(value) aria_label="Example panels"
                tabs=vec![Tab::new("overview", "Overview"), Tab::new("details", "Details")]>
                <TabPanel value="overview"><p>"The first panel is visible initially."</p></TabPanel>
                <TabPanel value="details"><p>"Each panel keeps its place and state in the document."</p></TabPanel>
            </Tabs>
            <p class="text-sm">"Active panel: "<output data-demo-state="">{move || active.get()}</output></p>
        </div>
    }
}

/// A bounded scroll area stays keyboard reachable as its content grows.
pub fn scroll_area_demo() -> impl IntoView {
    let count = RwSignal::new(8_usize);
    view! {
        <div class="space-y-4">
            <ScrollArea aria_label="Example items" class="h-40 rounded-md border border-border p-3">
                <ol class="space-y-3">
                    <For each=move || 0..count.get() key=|item| *item let:item>
                        <li>{format!("Item {}", item + 1)}</li>
                    </For>
                </ol>
            </ScrollArea>
            <Button variant=ButtonVariant::Outline on_click=move || count.update(|count| *count += 1)>
                "Add item"
            </Button>
            <p class="text-sm">"Items: "<output data-demo-state="">{move || count.get()}</output></p>
        </div>
    }
}

/// A virtual table receives its data, row identities and selection from outside.
pub fn data_table_demo() -> impl IntoView {
    let entries = [
        ["Button", "Actions"],
        ["Text field", "Input"],
        ["Data table", "Collections"],
        ["Dialog", "Overlays"],
    ];
    let selected = RwSignal::new(Selection::new());
    let activated = RwSignal::new(None::<usize>);
    let focus = RwSignal::new(Cell { row: 0, column: 0 });
    let goto = RwSignal::new(None);
    view! {
        <div class="min-w-0 space-y-4">
            <DataTable rows=entries.len() columns=vec![Column::new("name", "Component"), Column::new("group", "Group")]
                cell=Arc::new(move |row, column| entries[row][column].to_string())
                row_id=Arc::new(|row| row as u32) selected=selected goto=goto focus=focus
                on_select=move |row| { selected.update(|selected| { selected.toggle(row as u32); }); }
                on_activate=move |row| activated.set(Some(row))
                initial_viewport=(360.0, 128.0) row_height=32.0 column_width=180.0
                aria_label="Example components" class="h-48 w-full" />
            <p class="text-sm break-words">"Selected: "<output data-demo-state="">{move || selected.with(|selection| {
                if selection.is_empty() {
                    "None".to_string()
                } else {
                    selection.ids().map(|row| entries[row as usize][0]).collect::<Vec<_>>().join(", ")
                }
            })}</output></p>
            <p class="text-sm">"Activated: "<output>{move || activated.get().map_or("None", |row| entries[row][0])}</output></p>
        </div>
    }
}

/// A two-level tree shares expansion and selection with the application.
pub fn tree_demo() -> impl IntoView {
    let expanded = RwSignal::new(BTreeSet::from(["components".to_string()]));
    let selected = RwSignal::new(None::<String>);
    view! {
        <div class="space-y-4">
            <Tree expanded=expanded selected=selected on_select=move |value| selected.set(Some(value))
                aria_label="Example documentation tree"
                nodes=vec![
                    TreeNode::new("components", "Components").with(vec![
                        TreeNode::new("button", "Button"),
                        TreeNode::new("text-field", "Text field"),
                    ]),
                    TreeNode::new("guides", "Guides").with(vec![
                        TreeNode::new("quickstart", "Quick start"),
                        TreeNode::new("themes", "Themes"),
                    ]),
                ] />
            <p class="text-sm">"Selected: "<output data-demo-state="">{move || selected.get().unwrap_or_else(|| "None".to_string())}</output></p>
            <p class="text-sm">"Expanded: "<output>{move || expanded.with(|groups| {
                if groups.is_empty() { "None".to_string() } else { groups.iter().cloned().collect::<Vec<_>>().join(", ") }
            })}</output></p>
        </div>
    }
}
