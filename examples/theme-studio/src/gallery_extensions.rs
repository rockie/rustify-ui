//! The extension components' gallery samples: one working surface per
//! extension family, driven the way an application drives them, and the local
//! theme boundary that overlays them with a preview patch.

use crate::controller::StudioState;
use crate::gallery::StateToggle;
use leptos::prelude::*;
use rustify_components::data_table::{Cell as GridCell, Column, DataTable};
use rustify_components::workspace::{
    columns, Command, CommandPalette, PanelTab, PanelTabs, Splitter,
};
use rustify_components::{DropZone, FilePicker, Glyph, Icon, Tree, TreeNode};
use rustify_ui::Selection;
use std::collections::BTreeSet;
use std::sync::Arc;

#[component]
pub(crate) fn DataTableSample() -> impl IntoView {
    let rows = RwSignal::new(100_000usize);
    let empty = RwSignal::new(false);
    let picked = RwSignal::new(false);
    let selection = RwSignal::new(Selection::new());
    let sorted = RwSignal::new(None::<(usize, bool)>);
    let activated = RwSignal::new("no row opened".to_string());
    let columns = Signal::stored(
        (0..6)
            .map(|index| Column::new(format!("c{index}"), format!("column {}", index + 1)))
            .collect::<Vec<_>>(),
    );
    Effect::new(move |_| {
        rows.set(if empty.get() { 0 } else { 100_000 });
    });
    Effect::new(move |_| {
        let want = picked.get();
        selection.update(|selection| {
            if selection.contains(1) != want {
                selection.toggle(1);
            }
        });
    });
    view! {
        <div class="gallery-controls"><StateToggle label="Selected" value=picked /><StateToggle label="Empty" value=empty /></div>
        <Show when=move || empty.get() fallback=|| ()>
            <button type="button" on:click=move |_| empty.set(false)>"Restore rows"</button>
        </Show>
        <DataTable
            test_id="gallery-table"
            aria_label="a hundred thousand rows"
            class="h-48 w-full"
            rows=rows
            columns=columns
            cell=Arc::new(|row, column| format!("r{row}c{column}"))
            row_id=Arc::new(|row| row as u32 + 1)
            selected=selection
            goto=RwSignal::new(None)
            focus=RwSignal::new(GridCell::default())
            sorted=sorted
            on_sort=Arc::new(move |column| {
                sorted.update(|current| {
                    *current = match *current {
                        Some((at, true)) if at == column => Some((column, false)),
                        _ => Some((column, true)),
                    };
                });
            })
            on_select=move |row| {
                selection.update(|selection| {
                    selection.toggle(row as u32 + 1);
                });
            }
            on_activate=move |row| activated.set(format!("opened row {}", row + 1))
        />
        <output>{move || activated.get()}</output>
    }
}

fn sample_groups() -> Vec<TreeNode> {
    (0..3)
        .map(|group| {
            TreeNode::new(format!("g{group}"), format!("group {}", group + 1)).with(
                (0..3)
                    .map(|child| {
                        TreeNode::new(
                            format!("g{group}-{child}"),
                            format!("group {}.{}", group + 1, child + 1),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

#[component]
pub(crate) fn TreeSample() -> impl IntoView {
    let empty = RwSignal::new(false);
    let nodes = RwSignal::new(sample_groups());
    let expanded = RwSignal::new(BTreeSet::from(["g0".to_string()]));
    let picked = RwSignal::new(Option::<String>::None);
    Effect::new(move |_| {
        nodes.set(if empty.get() {
            Vec::new()
        } else {
            sample_groups()
        });
    });
    view! {
        <StateToggle label="Empty" value=empty />
        <Show when=move || empty.get() fallback=|| ()>
            <button type="button" on:click=move |_| empty.set(false)>"Restore tree"</button>
        </Show>
        <Tree
            test_id="gallery-tree"
            aria_label="sample groups"
            class="h-48 w-full"
            nodes=nodes
            expanded=expanded
            selected=Signal::derive(move || picked.get())
            on_select=move |key| picked.set(Some(key))
        />
    }
}

#[component]
pub(crate) fn FilesSample() -> impl IntoView {
    let names = RwSignal::new(Vec::<String>::new());
    let invalid = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let reason_id = rustify_components::id::next("gallery-files-reason");
    let reason_element_id = reason_id.clone();
    let described = Signal::derive(move || {
        if invalid.get() {
            reason_id.clone()
        } else {
            String::new()
        }
    });
    view! {
        <div class="gallery-controls"><StateToggle label="Disabled" value=disabled /><StateToggle label="Invalid" value=invalid /></div>
        <FilePicker
            test_id="gallery-file"
            aria_label="Import a file"
            accept=".json,.txt"
            disabled=disabled
            invalid=invalid
            described_by=described.clone()
            on_files=move |files| keep(names, files)
        />
        <DropZone
            test_id="gallery-drop"
            disabled=disabled
            invalid=invalid
            described_by=described
            on_files=move |files| keep(names, files)
        />
        <p id=reason_element_id class="gallery-files-reason">{move || {
            if invalid.get() { "Only .json and .txt files are accepted." } else { "" }
        }}</p>
        <output>{move || {
            let names = names.get();
            if names.is_empty() {
                "No files yet.".to_string()
            } else {
                names.join(", ")
            }
        }}</output>
    }
}

fn keep(names: RwSignal<Vec<String>>, files: Option<leptos::web_sys::FileList>) {
    let Some(files) = files else { return };
    let mut kept = names.get_untracked();
    for index in 0..files.length() {
        if let Some(file) = files.item(index) {
            kept.push(file.name());
        }
    }
    names.set(kept);
}

fn workspace_tabs() -> Vec<PanelTab> {
    vec![
        PanelTab::new("overview", "overview").permanent(),
        PanelTab::new("drafts", "drafts"),
        PanelTab::new("editor", "editor"),
        PanelTab::new("notes", "notes"),
        PanelTab::new("preview", "preview"),
    ]
}

#[component]
pub(crate) fn WorkspaceSample() -> impl IntoView {
    let sizes = RwSignal::new(vec![220.0, 320.0, 220.0]);
    let tabs = RwSignal::new(workspace_tabs());
    let active = RwSignal::new("overview".to_string());
    view! {
        <button type="button" on:click=move |_| {
            sizes.set(vec![220.0, 320.0, 220.0]);
            tabs.set(workspace_tabs());
            active.set("overview".to_string());
        }>"Reset"</button>
        <div
            class="gallery-workspace"
            style:grid-template-columns=move || columns(&sizes.get())
        >
            <div class="gallery-workspace-panel">"Drafts"</div>
            <div class="gallery-workspace-panel">"Editor"</div>
            <div class="gallery-workspace-panel">"Notes"</div>
            <Splitter
                test_id="gallery-split"
                sizes=sizes
                mins=vec![120.0, 160.0, 120.0]
                on_resize=move |next| sizes.set(next)
            />
        </div>
        <PanelTabs
            test_id="gallery-tabs"
            class="w-full"
            tabs=tabs
            active=active
            on_activate=move |next| active.set(next)
            on_close=move |closed| {
                let list = tabs.get_untracked();
                if active.get_untracked() == closed {
                    let next = list
                        .iter()
                        .position(|tab| tab.id == closed)
                        .and_then(|at| {
                            list.get(at + 1)
                                .or_else(|| at.checked_sub(1).and_then(|left| list.get(left)))
                        })
                        .map(|tab| tab.id.clone());
                    if let Some(next) = next {
                        active.set(next);
                    }
                }
                tabs.set(
                    list.into_iter()
                        .filter(|tab| tab.id != closed)
                        .collect(),
                );
            }
        />
    }
}

#[component]
pub(crate) fn CommandSample() -> impl IntoView {
    let open = RwSignal::new(false);
    let ran = RwSignal::new("no command run".to_string());
    view! {
        <button type="button" aria-haspopup="dialog" on:click=move |_| open.set(true)>"Open the command palette"</button>
        <CommandPalette
            test_id="gallery-command"
            open=open
            on_open_change=move |next| open.set(next)
            commands=vec![
                Command::new("save", "Save the file").keywords("write disk"),
                Command::new("export", "Export as PDF").keywords("write disk"),
                Command::new("delete", "Delete the draft").unavailable("nothing is selected"),
                Command::new("close", "Close the preview").unavailable("the preview is pinned"),
            ]
            on_run=move |id| ran.set(id)
        />
        <output>{move || ran.get()}</output>
    }
}

#[component]
pub(crate) fn GlyphSample() -> impl IntoView {
    view! {
        <div class="gallery-row">
            <Icon glyph=Glyph::Check label="Done" />
            <Icon glyph=Glyph::ChevronDown />
            <Icon glyph=Glyph::ChevronUp />
            <Icon glyph=Glyph::ChevronRight />
            <Icon glyph=Glyph::Close />
            <Icon glyph=Glyph::Dot />
            <Icon glyph=Glyph::DividerHorizontal />
        </div>
        <p>"The first icon carries a name; the others are decoration."</p>
    }
}

fn local_groups() -> Vec<TreeNode> {
    vec![
        TreeNode::new("l0", "local group").with(vec![TreeNode::new("l0-0", "local item")]),
        TreeNode::new("l1", "another group").with(vec![TreeNode::new("l1-0", "another item")]),
    ]
}

#[component]
pub(crate) fn ExtensionLocalSample(state: StudioState) -> impl IntoView {
    let low = RwSignal::new(false);
    let open = RwSignal::new(false);
    let picked = RwSignal::new(Option::<String>::None);
    let tabs = RwSignal::new(vec![
        PanelTab::new("surface", "surface").permanent(),
        PanelTab::new("shared", "shared"),
    ]);
    let active = RwSignal::new("surface".to_string());
    let patch = Signal::derive(move || {
        rustify_ui::ThemeBoundaryPatch::from(rustify_ui::ThemeValuePatch(if low.get() {
            [
                ("accent".into(), "#7fcfac".into()),
                ("accent-foreground".into(), "#9adcc3".into()),
                ("border".into(), "#e7ebee".into()),
                ("popover".into(), "#f3f5f7".into()),
                ("popover-foreground".into(), "#c3cbd2".into()),
            ]
            .into()
        } else {
            Default::default()
        }))
    });
    view! {
        <section class="gallery-local-theme">
            <h3>{move || state.tr("Extension local theme", "扩展局部主题")}</h3>
            <StateToggle label="Low contrast" value=low />
            <rustify_ui::ThemeBoundary patch=patch test_id="extension-boundary">
                <crate::controller::RegisterPreviewSource
                    state=state
                    label=["Extension local theme", "扩展局部主题"]
                />
                <div
                    class="gallery-local-card"
                    data-theme-tokens="accent,accent-foreground,border,popover,popover-foreground"
                    data-testid="extension-local-card"
                >
                    <Tree
                        class="h-32"
                        aria_label="local groups"
                        nodes=Signal::stored(local_groups())
                        expanded=RwSignal::new(BTreeSet::from(["l0".to_string()]))
                        selected=Signal::derive(move || picked.get())
                        on_select=move |key| picked.set(Some(key))
                    />
                    <PanelTabs
                        tabs=tabs
                        active=active
                        on_activate=move |next| active.set(next)
                        on_close=move |closed| {
                            let list = tabs.get_untracked();
                            if active.get_untracked() == closed {
                                let next = list
                                    .iter()
                                    .position(|tab| tab.id == closed)
                                    .and_then(|at| {
                                        list.get(at + 1).or_else(|| {
                                            at.checked_sub(1).and_then(|left| list.get(left))
                                        })
                                    })
                                    .map(|tab| tab.id.clone());
                                if let Some(next) = next {
                                    active.set(next);
                                }
                            }
                            tabs.set(
                                list.into_iter()
                                    .filter(|tab| tab.id != closed)
                                    .collect(),
                            );
                        }
                    />
                    <button type="button" on:click=move |_| open.set(true)>
                        {move || state.tr("Run a command", "运行命令")}
                    </button>
                    <CommandPalette
                        test_id="gallery-command-local"
                        open=open
                        on_open_change=move |next| open.set(next)
                        commands=vec![Command::new("local", "Run a local command")]
                        on_run=move |_| open.set(false)
                    />
                </div>
            </rustify_ui::ThemeBoundary>
        </section>
    }
}
