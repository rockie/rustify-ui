use std::collections::BTreeSet;
use std::sync::Arc;

use leptos::prelude::*;
use rustify_components::accordion;
use rustify_components::collapsible;
use rustify_components::data_table::Cell;
use rustify_components::layout::{Grid, GridTracks};
use rustify_components::navigation_menu;
use rustify_components::tab_nav;
use rustify_components::toolbar;
use rustify_components::workspace::{
    columns, Command, CommandPalette, PanelTab, PanelTabs, Splitter,
};
use rustify_components::{
    Button, ButtonVariant, Column, DataTable, DropZone, FilePicker, Responsive, ScrollArea, Tab,
    TabPanel, Tabs, Tree, TreeNode,
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
            <p data-demo-state-line="" class="text-sm">"Active panel: "<output data-demo-state="">{move || active.get()}</output></p>
        </div>
    }
}

/// Tab nav renders real links for page navigation.
pub fn tab_nav_demo() -> impl IntoView {
    let current = RwSignal::new("components".to_string());
    view! {
        <div class="space-y-4">
            <tab_nav::Root aria_label="Site sections">
                <tab_nav::Link href="#tab-nav-demo" active=Signal::derive(move || current.get() == "components")>
                    "Components"
                </tab_nav::Link>
                <tab_nav::Link href="#tab-nav-demo" active=Signal::derive(move || current.get() == "themes")>
                    "Themes"
                </tab_nav::Link>
            </tab_nav::Root>
            <div class="flex gap-3">
                <Button variant=ButtonVariant::Outline on_click=move || current.set("components".to_string())>"Visit components"</Button>
                <Button variant=ButtonVariant::Outline on_click=move || current.set("themes".to_string())>"Visit themes"</Button>
            </div>
            <p data-demo-state-line="" id="tab-nav-demo" class="text-sm">"Current section: "<output data-demo-state="">{move || current.get()}</output></p>
        </div>
    }
}

/// An accordion opens one panel at a time; the open panel belongs to the
/// application, and arrow keys move between the headers.
pub fn accordion_demo() -> impl IntoView {
    let open = RwSignal::new(accordion::AccordionValue::Single(Some("first".to_string())));
    view! {
        <div class="space-y-4">
            <accordion::Root value=open on_change=move |next| open.set(next) collapsible=true class="max-w-md space-y-2">
                <accordion::Item value="first">
                    <accordion::Header><accordion::Trigger>"What stays local?"</accordion::Trigger></accordion::Header>
                    <accordion::Content>"Every sample value stays in this page's application state."</accordion::Content>
                </accordion::Item>
                <accordion::Item value="second">
                    <accordion::Header><accordion::Trigger>"Does it work without scripts?"</accordion::Trigger></accordion::Header>
                    <accordion::Content>"The panels render in static HTML and hydrate on demand."</accordion::Content>
                </accordion::Item>
            </accordion::Root>
            <p data-demo-state-line="" class="text-sm">"Open panel: "<output data-demo-state="">{move || match open.get() {
                accordion::AccordionValue::Single(Some(value)) => value,
                _ => "none".to_string(),
            }}</output></p>
        </div>
    }
}

/// A collapsible discloses one region on demand.
pub fn collapsible_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <collapsible::Root open=Signal::derive(move || open.get()) on_open_change=Callback::new(move |next| open.set(next))>
                <div class="max-w-md space-y-2">
                    <collapsible::Trigger>"Show deployment notes"</collapsible::Trigger>
                    <collapsible::Content>"The static build deploys as files; no server runtime is required."</collapsible::Content>
                </div>
            </collapsible::Root>
            <p data-demo-state-line="" class="text-sm">"Notes: "<output data-demo-state="">{move || if open.get() { "Shown" } else { "Hidden" }}</output></p>
        </div>
    }
}

/// A navigation menu separates site links from application commands.
pub fn navigation_menu_demo() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    view! {
        <div class="space-y-4">
            <navigation_menu::Root value=Signal::derive(move || value.get()) aria_label="Documentation"
                on_value_change=Callback::new(move |next| value.set(next))>
                <navigation_menu::List>
                    <navigation_menu::Item value="learn">
                        <navigation_menu::Trigger>"Learn"</navigation_menu::Trigger>
                        <navigation_menu::Content>
                            <div class="flex flex-col gap-1 p-2">
                                <navigation_menu::Link href="#navigation-menu-demo">"Components"</navigation_menu::Link>
                                <navigation_menu::Link href="#navigation-menu-demo">"Typography"</navigation_menu::Link>
                            </div>
                        </navigation_menu::Content>
                    </navigation_menu::Item>
                    <navigation_menu::Item value="reference">
                        <navigation_menu::Trigger>"Reference"</navigation_menu::Trigger>
                        <navigation_menu::Content>
                            <div class="flex flex-col gap-1 p-2">
                                <navigation_menu::Link href="#navigation-menu-demo">"Colors"</navigation_menu::Link>
                                <navigation_menu::Link href="#navigation-menu-demo">"Primitives"</navigation_menu::Link>
                            </div>
                        </navigation_menu::Content>
                    </navigation_menu::Item>
                </navigation_menu::List>
            </navigation_menu::Root>
            <p data-demo-state-line="" id="navigation-menu-demo" class="text-sm">"Open item: "
                <output data-demo-state="">{move || value.get().unwrap_or_else(|| "none".into())}</output>
            </p>
        </div>
    }
}

/// A toolbar groups commands with roving focus between them.
pub fn toolbar_demo() -> impl IntoView {
    let last = RwSignal::new("None".to_string());
    let pressed = RwSignal::new(vec!["grid".to_string()]);
    view! {
        <div class="space-y-4">
            <toolbar::Root aria_label="Editor commands">
                <toolbar::Button on_click=Some(Callback::new(move |_| last.set("Copy".to_string())))>"Copy"</toolbar::Button>
                <toolbar::Button on_click=Some(Callback::new(move |_| last.set("Rename".to_string())))>"Rename"</toolbar::Button>
                <toolbar::Separator />
                <toolbar::ToggleGroup value=Signal::derive(move || pressed.get())
                    on_change=Callback::new(move |next| pressed.set(next))>
                    <toolbar::ToggleItem class="data-[state=on]:bg-primary/15 data-[state=on]:text-foreground" value="grid">"Grid"</toolbar::ToggleItem>
                    <toolbar::ToggleItem class="data-[state=on]:bg-primary/15 data-[state=on]:text-foreground" value="list">"List"</toolbar::ToggleItem>
                </toolbar::ToggleGroup>
            </toolbar::Root>
            <p data-demo-state-line="" class="text-sm">"Last command: "
                <output data-demo-state="">{move || format!("{}, layout {}", last.get(), pressed.get().join("+"))}</output>
            </p>
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
            <p data-demo-state-line="" class="text-sm">"Items: "<output data-demo-state="">{move || count.get()}</output></p>
        </div>
    }
}

/// A virtual table receives its data, row identities, selection and sort from
/// outside.
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
    let sorted = RwSignal::new(None::<(usize, bool)>);
    // Display position -> entry index; row identity, selection and activation
    // follow the entry, not the position it is drawn at.
    let order = Memo::new(move |_| {
        let mut order: Vec<usize> = (0..entries.len()).collect();
        if let Some((column, ascending)) = sorted.get() {
            order.sort_by(|left, right| {
                let ordering = entries[*left][column].cmp(entries[*right][column]);
                if ascending {
                    ordering
                } else {
                    ordering.reverse()
                }
            });
        }
        order
    });
    let entry = move |row: usize| order.with(|order| order[row]);
    let chosen = move |row: usize| order.with_untracked(|order| order[row]);
    let on_sort: std::sync::Arc<dyn Fn(usize) + Send + Sync> =
        std::sync::Arc::new(move |column: usize| {
            sorted.update(|current| {
                *current = match *current {
                    Some((active, ascending)) if active == column => Some((column, !ascending)),
                    _ => Some((column, true)),
                };
            });
        });
    view! {
        <div class="min-w-0 space-y-4 [&_[role=row][aria-selected=true]]:bg-primary/15 [&_[role=row][aria-selected=true]]:text-foreground [&_[role=row][aria-selected=true]_[role=gridcell]:focus-visible]:outline-ring">
            <DataTable rows=entries.len() columns=vec![Column::new("name", "Component"), Column::new("group", "Group")]
                cell=Arc::new(move |row, column| entries[entry(row)][column].to_string())
                row_id=Arc::new(move |row| entry(row) as u32) selected=selected goto=goto focus=focus
                sorted=sorted
                on_sort=on_sort
                on_select=move |row| { selected.update(|selected| { selected.toggle(chosen(row) as u32); }); }
                on_activate=move |row| activated.set(Some(chosen(row)))
                initial_viewport=(360.0, 128.0) row_height=32.0 column_width=180.0
                aria_label="Example components" class="w-full" />
            <p data-demo-state-line="" class="text-sm break-words">"Selected: "<output data-demo-state="">{move || selected.with(|selection| {
                if selection.is_empty() {
                    "None".to_string()
                } else {
                    selection.ids().map(|row| entries[row as usize][0]).collect::<Vec<_>>().join(", ")
                }
            })}</output></p>
            <p class="text-sm">"Activated: "<output>{move || activated.get().map_or("None", |row| entries[row][0])}</output></p>
            <p class="text-sm">"Sort: "<output>{move || sorted.get().map_or("none".to_string(), |(column, ascending)| format!("column {} {}", column + 1, if ascending { "↑" } else { "↓" }))}</output></p>
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
            <p data-demo-state-line="" class="text-sm">"Selected: "<output data-demo-state="">{move || selected.get().unwrap_or_else(|| "None".to_string())}</output></p>
            <p class="text-sm">"Expanded: "<output>{move || expanded.with(|groups| {
                if groups.is_empty() { "None".to_string() } else { groups.iter().cloned().collect::<Vec<_>>().join(", ") }
            })}</output></p>
        </div>
    }
}

/// A file picker reports the files the user actually chose.
pub fn file_picker_demo() -> impl IntoView {
    let names = RwSignal::new(Vec::<String>::new());
    view! {
        <div class="space-y-4">
            <FilePicker aria_label="Import a theme" accept=".json"
                on_files=move |files| {
                    if let Some(files) = files {
                        names.set((0..files.length()).filter_map(|index| files.item(index).map(|file| file.name())).collect());
                    } else {
                        names.set(Vec::new());
                    }
                } />
            <p data-demo-state-line="" class="text-sm break-words">"Chosen: "
                <output data-demo-state="">{move || names.with(|names| if names.is_empty() { "None".to_string() } else { names.join(", ") })}</output>
            </p>
        </div>
    }
}

/// A drop zone accepts dragged files and shows invalid intent.
pub fn drop_zone_demo() -> impl IntoView {
    let names = RwSignal::new(Vec::<String>::new());
    let invalid = RwSignal::new(false);
    let reason = "drop-zone-demo-reason";
    view! {
        <div class="space-y-4">
            <DropZone label="Drop export files here" invalid=Signal::derive(move || invalid.get())
                described_by=reason.to_string()
                on_files=move |files| {
                    if let Some(files) = files {
                        names.set((0..files.length()).filter_map(|index| files.item(index).map(|file| file.name())).collect());
                    }
                } />
            <p id=reason class="text-sm text-muted-foreground">"Drop any file you own; nothing leaves this page."</p>
            <Button variant=ButtonVariant::Outline on_click=move || invalid.set(!invalid.get_untracked())>
                {move || if invalid.get() { "Clear the invalid state" } else { "Mark the zone invalid" }}
            </Button>
            <p data-demo-state-line="" class="text-sm break-words">"Dropped: "
                <output data-demo-state="">{move || names.with(|names| if names.is_empty() { "None".to_string() } else { names.join(", ") })}</output>
            </p>
        </div>
    }
}

/// A splitter resizes columns by drag or keyboard, with an application reset.
/// The application lays the row out from the same pixel sizes the dividers
/// are placed from.
pub fn splitter_demo() -> impl IntoView {
    const INITIAL: [f64; 3] = [96.0, 96.0, 96.0];
    let sizes = RwSignal::new(INITIAL.to_vec());
    view! {
        <div class="space-y-4">
            <div class="max-w-full overflow-x-auto rounded-md border border-border p-3">
                <Grid columns=Signal::derive(move || Responsive::from(GridTracks::from(columns(&sizes.get()))))
                    class="relative h-24 w-fit">
                    <div class="truncate bg-muted p-3 text-sm">"Sources"</div>
                    <div class="truncate border-x border-border p-3 text-sm">"Preview"</div>
                    <div class="truncate bg-muted p-3 text-sm">"Inspector"</div>
                    <Splitter sizes=Signal::derive(move || sizes.get()) mins=vec![64.0, 64.0, 64.0]
                        divider_label="Resize panels" on_resize=move |next| sizes.set(next) />
                </Grid>
            </div>
            <Button variant=ButtonVariant::Outline on_click=move || sizes.set(INITIAL.to_vec())>"Reset columns"</Button>
            <p data-demo-state-line="" class="text-sm">"Columns: "
                <output data-demo-state="">{move || sizes.with(|sizes| sizes.iter().map(|size| format!("{size:.0}px")).collect::<Vec<_>>().join(" / "))}</output>
            </p>
        </div>
    }
}

/// Panel tabs switch and close working surfaces.
pub fn panel_tabs_demo() -> impl IntoView {
    let tabs = RwSignal::new(vec![
        PanelTab::new("draft", "Draft"),
        PanelTab::new("tokens", "Tokens"),
        PanelTab::new("export", "Export"),
    ]);
    let active = RwSignal::new("draft".to_string());
    view! {
        <div class="space-y-4">
            <PanelTabs tabs=Signal::derive(move || tabs.get()) active=Signal::derive(move || active.get())
                aria_label="Working panels" close_label="Close panel"
                on_activate=move |next| active.set(next)
                on_close=move |closed| {
                    tabs.update(|tabs| tabs.retain(|tab| tab.id != closed));
                    if active.get_untracked() == closed {
                        if let Some(first) = tabs.get_untracked().first() { active.set(first.id.clone()); }
                    }
                } />
            <p data-demo-state-line="" class="text-sm">"Open panels: "
                <output data-demo-state="">{move || {
                    let open: Vec<String> = tabs.get().iter().map(|tab| tab.label.clone()).collect();
                    format!("{} · active {}", open.join(", "), active.get())
                }}</output>
            </p>
        </div>
    }
}

/// A command palette searches and runs commands by name.
pub fn command_palette_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let run = RwSignal::new("None".to_string());
    view! {
        <div class="demo-modal-workspace min-h-96 space-y-4">
            <Button on_click=move || open.set(true)>"Run a command"</Button>
            <CommandPalette open=Signal::derive(move || open.get()) aria_label="Commands"
                placeholder="Search commands"
                on_open_change=move |next| open.set(next)
                commands=Signal::stored(vec![
                    Command::new("open-studio", "Open the Theme Studio"),
                    Command::new("copy-json", "Copy the theme as JSON"),
                    Command::new("toggle-grid", "Toggle the preview grid"),
                ])
                on_run=move |command| { run.set(command); open.set(false); } />
            <p data-demo-state-line="" class="text-sm">"Ran: "<output data-demo-state="">{move || run.get()}</output></p>
        </div>
    }
}
