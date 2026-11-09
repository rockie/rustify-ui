#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
//! The extension components' static contract: parts, states, empty and
//! refused semantics, and the SDK's own words in both languages.
use leptos::prelude::*;
use rustify_components::data_table::{Cell as GridCell, Column, DataTable};
use rustify_components::icon::Glyph;
use rustify_components::workspace::{PanelTab, PanelTabs, Splitter};
use rustify_components::{DropZone, FilePicker, Icon, Tree, TreeNode};
use rustify_ui::Selection;
use std::collections::BTreeSet;
use std::sync::Arc;

fn render(view: impl FnOnce() -> AnyView + Send + 'static) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "extensions".into(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    })
}

fn attribute<'a>(tag: &'a str, key: &str) -> &'a str {
    tag.split(&format!(" {key}=\""))
        .nth(1)
        .unwrap_or_default()
        .split('"')
        .next()
        .unwrap_or_default()
}

fn tag<'a>(html: &'a str, id: &str) -> &'a str {
    html.split('<')
        .find(|tag| attribute(tag, "data-testid") == id)
        .unwrap()
}

fn table() -> impl IntoView {
    view! {
        <DataTable
            test_id="table"
            aria_label="the sample"
            rows=100_000usize
            columns=vec![Column::new("a", "A")]
            cell=Arc::new(|row, _| row.to_string())
            row_id=Arc::new(|row| row as u32)
            selected=Selection::new()
            goto=RwSignal::new(None)
            focus=RwSignal::new(GridCell::default())
            on_select=|_| {}
            on_activate=|_| {}
            initial_viewport=(120.0, 96.0)
        />
    }
}

#[test]
fn a_data_table_marks_its_parts_and_stays_windowed() {
    let html = render(|| table().into_any());
    assert!(html.contains("data-rustify-part=\"data-table\""), "{html}");
    assert!(html.contains("data-state=\"ready\""), "{html}");
    assert!(
        html.contains("data-rustify-part=\"data-table-header\""),
        "{html}"
    );
    assert!(
        html.contains("data-rustify-part=\"data-table-row\""),
        "{html}"
    );
    assert_eq!(html.matches("data-row-id=").count(), 25, "{html}");
    assert!(html.contains("aria-rowcount=\"100001\""), "{html}");
    // The text colour of the table is a theme role, not an inherited accident.
    assert!(html.contains("text-foreground"), "{html}");
}

#[test]
fn an_empty_data_table_announces_one_row_and_stays_out_of_the_selection() {
    let html = render(|| {
        view! {
            <DataTable
                test_id="table"
                rows=0usize
                empty_label="no rows match"
                columns=vec![Column::new("a", "A"), Column::new("b", "B")]
                cell=Arc::new(|row, _| row.to_string())
                row_id=Arc::new(|row| row as u32)
                selected=Selection::new()
                goto=RwSignal::new(None)
                focus=RwSignal::new(GridCell::default())
                on_select=|_| {}
                on_activate=|_| {}
                initial_viewport=(120.0, 96.0)
            />
        }
        .into_any()
    });
    assert!(html.contains("data-state=\"empty\""), "{html}");
    assert!(html.contains("aria-rowcount=\"2\""), "{html}");
    let empty = tag(&html, "table-empty");
    assert!(empty.contains("role=\"row\""), "{empty}");
    assert_eq!(attribute(empty, "aria-rowindex"), "2");
    assert!(html.contains("role=\"gridcell\""), "{html}");
    assert!(html.contains("aria-colspan=\"2\""), "{html}");
    assert!(html.contains("no rows match"), "{html}");
    assert!(!html.contains("data-row-id="), "{html}");
    assert!(!html.contains("data-selected"), "{html}");
}

#[test]
fn a_tree_marks_items_and_drops_the_tree_role_when_empty() {
    let nothing: Option<String> = None;
    let full = render(|| {
        view! {
            <Tree
                test_id="tree"
                nodes=vec![
                    TreeNode::new("g0", "group").with(vec![TreeNode::new("g0-0", "child")]),
                    TreeNode::new("g1", "leaf"),
                ]
                expanded=RwSignal::new(BTreeSet::new())
                selected=nothing
                on_select=|_| {}
            />
        }
        .into_any()
    });
    assert!(full.contains("data-rustify-part=\"tree\""), "{full}");
    assert!(full.contains("data-state=\"ready\""), "{full}");
    assert!(full.contains("role=\"tree\""), "{full}");
    assert!(full.contains("data-rustify-part=\"tree-item\""), "{full}");
    // A closed group says so, and its marker is a drawing rather than text.
    let group = tag(&full, "tree-g0");
    assert_eq!(attribute(group, "data-state"), "closed");
    assert!(full.contains("data-glyph=\"chevron-right\""), "{full}");
    // A leaf has no state to report and a dot, not a chevron.
    let leaf = tag(&full, "tree-g1");
    assert_eq!(attribute(leaf, "data-state"), "");
    assert!(full.contains("data-glyph=\"dot\""), "{full}");

    let no_nodes: Vec<TreeNode> = Vec::new();
    let no_choice: Option<String> = None;
    let empty = render(|| {
        view! {
            <Tree
                test_id="tree"
                nodes=no_nodes
                empty_label="no nodes match"
                expanded=RwSignal::new(BTreeSet::new())
                selected=no_choice
                on_select=|_| {}
            />
        }
        .into_any()
    });
    assert!(!empty.contains("role=\"tree\""), "{empty}");
    assert!(!empty.contains("aria-label=\""), "{empty}");
    assert!(empty.contains("data-state=\"empty\""), "{empty}");
    assert!(
        empty.contains("data-rustify-part=\"tree-empty\""),
        "{empty}"
    );
    assert!(empty.contains("data-testid=\"tree-empty\""), "{empty}");
    assert!(empty.contains("no nodes match"), "{empty}");
}

#[test]
fn a_panel_tab_is_a_tab_and_its_close_button_does_not_choose_it() {
    let html = render(|| {
        view! {
            <PanelTabs
                test_id="panels"
                tabs=vec![
                    PanelTab::new("all", "all").permanent(),
                    PanelTab::new("blue", "blue"),
                ]
                active="blue".to_string()
                on_activate=|_| {}
                on_close=|_| {}
            />
        }
        .into_any()
    });
    assert!(html.contains("data-rustify-part=\"panel-tabs\""), "{html}");
    assert!(html.contains("role=\"presentation\""), "{html}");
    assert_eq!(html.matches("role=\"tab\"").count(), 2, "{html}");
    assert_eq!(
        html.matches("data-rustify-part=\"panel-tab\"").count(),
        2,
        "{html}"
    );
    // The showing panel and the resting one differ on the frame, and only the
    // frame: the tab itself is always the same control.
    assert!(html.contains("data-state=\"active\""), "{html}");
    assert!(html.contains("data-state=\"inactive\""), "{html}");
    let tab = tag(&html, "panel-tab-blue");
    assert!(tab.starts_with("div "), "{tab}");
    assert_eq!(attribute(tab, "role"), "tab");
    assert_eq!(attribute(tab, "aria-selected"), "true");
    // The close control follows the tab's own focus: a tab-stop control
    // inside the tab trips axe (no-focusable-content), so it is a
    // button-like span without its own tab stop and the Delete key on the
    // tab closes instead.
    assert_eq!(html.matches("<button").count(), 0, "{html}");
    let close = tag(&html, "close-blue");
    assert!(close.starts_with("span "), "{close}");
    assert_eq!(attribute(close, "role"), "");
    assert_eq!(attribute(close, "tabindex"), "");
    // Mouse users still get the words, as a hover hint the tab's own
    // accessible name does not absorb.
    assert_eq!(attribute(close, "aria-hidden"), "true");
    assert_eq!(attribute(close, "title"), "close blue");
}

#[test]
fn file_and_drop_controls_carry_invalid_and_group_semantics() {
    let html = render(|| {
        view! {
            <FilePicker test_id="picker" aria_label="Import" invalid=true on_files=|_| {} />
            <DropZone test_id="zone" invalid=true on_files=|_| {} />
            <DropZone test_id="off-zone" disabled=true on_files=|_| {} />
        }
        .into_any()
    });
    assert!(html.contains("data-rustify-part=\"file-picker\""), "{html}");
    let picker = tag(&html, "picker");
    assert_eq!(attribute(picker, "aria-invalid"), "true");
    assert!(picker.contains("data-invalid"), "{picker}");
    assert_eq!(
        html.matches("data-rustify-part=\"drop-zone\"").count(),
        2,
        "{html}"
    );
    let zone = tag(&html, "zone");
    assert_eq!(attribute(zone, "role"), "group");
    assert_eq!(attribute(zone, "aria-label"), "drop a file here");
    assert!(zone.contains("border-destructive"), "{zone}");
    assert!(!zone.contains("aria-invalid"), "{zone}");
    // Being unable to take a file outranks being unwilling to: a disabled
    // zone says so and never also says its refusal is the reason.
    let off = tag(&html, "off-zone");
    assert_eq!(attribute(off, "aria-disabled"), "true");
    assert!(off.contains("opacity-50"), "{off}");
    assert!(!off.contains("border-destructive"), "{off}");
}

#[test]
fn the_default_words_follow_the_locale() {
    let html = Owner::new().with(|| {
        rustify_ui::id::provide("extensions-zh");
        rustify_ui::provide_locale(rustify_ui::Locale::Chinese);
        let no_nodes: Vec<TreeNode> = Vec::new();
        let no_choice: Option<String> = None;
        view! {
            <Splitter sizes=vec![120.0, 180.0] mins=vec![100.0, 100.0] on_resize=|_| {} />
            <DropZone test_id="zh-zone" on_files=|_| {} />
            <Tree
                test_id="zh-tree"
                nodes=no_nodes
                empty_label=""
                expanded=RwSignal::new(BTreeSet::new())
                selected=no_choice
                on_select=|_| {}
            />
            <DataTable
                test_id="zh-table"
                rows=0usize
                empty_label=""
                columns=vec![Column::new("a", "A")]
                cell=Arc::new(|row, _| row.to_string())
                row_id=Arc::new(|row| row as u32)
                selected=Selection::new()
                goto=RwSignal::new(None)
                focus=RwSignal::new(GridCell::default())
                on_select=|_| {}
                on_activate=|_| {}
            />
            <PanelTabs
                tabs=vec![PanelTab::new("blue", "blue")]
                active="blue".to_string()
                on_activate=|_| {}
                on_close=|_| {}
            />
        }
        .to_html()
    });
    assert!(html.contains("调整面板大小 1"), "{html}");
    assert!(html.contains("把文件拖到这里"), "{html}");
    // The tree and the table each say "nothing to show" in the reader's
    // language rather than the SDK's own.
    assert_eq!(html.matches("没有内容").count(), 2, "{html}");
    assert!(html.contains("title=\"关闭 blue\""), "{html}");
}

#[test]
fn a_divider_glyph_draws_a_line() {
    let html = render(|| {
        view! {
            <Icon glyph=Glyph::DividerHorizontal test_id="divider" />
        }
        .into_any()
    });
    let divider = tag(&html, "divider");
    assert_eq!(attribute(divider, "data-glyph"), "divider-horizontal");
    assert!(html.contains("M5 12h14"), "{html}");
    assert_eq!(attribute(divider, "aria-hidden"), "true");
}
