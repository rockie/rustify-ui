use std::{collections::HashMap, fmt::Write};

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, MouseEvent};

use super::{dialogs, report, ShellState};
use crate::{
    app::Editor,
    document::{Document, Node},
    icons,
};

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn matches(node: &Node, query: &str) -> bool {
    node.name.to_lowercase().contains(query) || node.text.to_lowercase().contains(query)
}

fn render(doc: &Document, shell: &ShellState) -> String {
    let mut children: HashMap<Option<&str>, Vec<&Node>> = HashMap::new();
    for node in doc.nodes() {
        children
            .entry(node.parent_id.as_deref())
            .or_default()
            .push(node);
    }
    fn contains(node: &Node, query: &str, children: &HashMap<Option<&str>, Vec<&Node>>) -> bool {
        matches(node, query)
            || children
                .get(&Some(node.id.as_str()))
                .is_some_and(|nodes| nodes.iter().any(|child| contains(child, query, children)))
    }
    fn walk(
        parent: Option<&str>,
        depth: usize,
        children: &HashMap<Option<&str>, Vec<&Node>>,
        shell: &ShellState,
        query: &str,
        html: &mut String,
    ) {
        for node in children.get(&parent).into_iter().flatten().rev() {
            if !query.is_empty() && !contains(node, query, children) {
                continue;
            }
            let has_children = children
                .get(&Some(node.id.as_str()))
                .is_some_and(|nodes| !nodes.is_empty());
            let open = shell.expanded.contains(&node.id) || !query.is_empty();
            let component = node.flag("component");
            let instance = node.flag("isInstance");
            let id = escape(&node.id);
            let name = escape(&node.name);
            let expanded = if has_children {
                format!(" aria-expanded=\"{open}\"")
            } else {
                String::new()
            };
            let _=write!(html,"<div class=\"layer-row{}{}\" data-layer=\"{id}\" data-depth=\"{depth}\" role=\"treeitem\" aria-selected=\"false\"{expanded} draggable=\"true\"><button class=\"expand\" data-expand=\"{id}\" aria-label=\"{} {name}\">{}</button><span class=\"layer-icon\">{}</span><span class=\"layer-name\" title=\"{name}\">{name}</span><button class=\"lock{}\" data-lock=\"{id}\" title=\"{}\">{}</button><button class=\"visibility{}\" data-visibility=\"{id}\" title=\"{}\">{}</button></div>",
                if !node.visible{" dim"}else{""},if component||instance{" component"}else{""},if open{"Collapse"}else{"Expand"},if has_children {if open{"⌄"}else{"›"}}else{""},icons::markup(if component{"component"}else if instance{"instance"}else{&node.kind},13),if node.locked{" is-locked"}else{""},if node.locked{"Unlock"}else{"Lock"},icons::markup(if node.locked{"lock"}else{"unlock"},12),if !node.visible{" is-hidden"}else{""},if node.visible{"Hide"}else{"Show"},icons::markup(if node.visible{"eye"}else{"eyeOff"},12));
            if has_children && open {
                walk(Some(&node.id), depth + 1, children, shell, query, html);
            }
        }
    }
    let mut html = String::new();
    walk(
        None,
        0,
        &children,
        shell,
        &shell.search.trim().to_lowercase(),
        &mut html,
    );
    if html.is_empty() {
        html.push_str(
            "<div class=\"empty-state\">A fresh canvas.<br>Press R to draw your first shape.</div>",
        );
    }
    html
}

fn closest(event: &web_sys::Event, selector: &str) -> Option<Element> {
    event
        .target()?
        .dyn_into::<Element>()
        .ok()?
        .closest(selector)
        .ok()
        .flatten()
}

fn click(editor: Editor, event: MouseEvent) {
    for (selector, attribute) in [
        ("[data-expand]", "data-expand"),
        ("[data-visibility]", "data-visibility"),
        ("[data-lock]", "data-lock"),
    ] {
        if let Some(element) = closest(&event, selector) {
            let Some(id) = element.get_attribute(attribute) else {
                return;
            };
            if attribute == "data-expand" {
                editor.shell.update(|shell| {
                    if !shell.expanded.remove(&id) {
                        shell.expanded.insert(id);
                    }
                });
            } else {
                report(
                    editor,
                    editor.edit(
                        if attribute == "data-lock" {
                            "Toggle layer lock"
                        } else {
                            "Toggle layer visibility"
                        },
                        |doc| {
                            if let Some(node) = doc.get_mut(&id) {
                                if attribute == "data-lock" {
                                    node.locked = !node.locked;
                                } else {
                                    node.visible = !node.visible;
                                }
                                doc.touch(Some(&id));
                            }
                            Ok(())
                        },
                    ),
                );
            }
            return;
        }
    }
    if let Some(id) =
        closest(&event, "[data-layer]").and_then(|row| row.get_attribute("data-layer"))
    {
        let mut selected = if event.shift_key() || event.meta_key() || event.ctrl_key() {
            editor.selection.get_untracked()
        } else {
            Vec::new()
        };
        if let Some(index) = selected.iter().position(|value| value == &id) {
            selected.remove(index);
        } else {
            selected.push(id);
        }
        editor.select(selected);
    }
}

fn drop_layer(editor: Editor, event: DragEvent) {
    event.prevent_default();
    clear_drop_targets();
    let Some(transfer) = event.data_transfer() else {
        return;
    };
    let Ok(id) = transfer.get_data("application/x-vellum-layer") else {
        return;
    };
    let Some(target_id) =
        closest(&event, "[data-layer]").and_then(|row| row.get_attribute("data-layer"))
    else {
        return;
    };
    let mut parent = None;
    let mut changed = false;
    let result = editor.edit("Reorder layer", |doc| {
        let Some(target) = doc.get(&target_id).cloned() else {
            return Ok(());
        };
        if id == target_id
            || doc.get(&id).is_none()
            || doc.descendants(&id).iter().any(|node| node.id == target_id)
        {
            return Ok(());
        }
        let frame = crate::scene::compose(doc);
        let Some(world) = frame.world(&id) else {
            return Ok(());
        };
        parent = if event.shift_key() && matches!(target.kind.as_str(), "frame" | "group") {
            Some(target_id.clone())
        } else {
            target.parent_id.clone()
        };
        let parent_matrix = parent
            .as_deref()
            .and_then(|id| frame.world(id))
            .map_or_else(crate::affine::identity, |item| item.matrix);
        if let Some(node) = doc.get_mut(&id) {
            node.parent_id = parent.clone();
            crate::commands::set_from_matrix(
                node,
                crate::affine::multiply(crate::affine::inverse(parent_matrix), world.matrix),
            );
        }
        let nodes = &mut doc.page_mut().nodes;
        if let Some(index) = nodes.iter().position(|node| node.id == id) {
            let node = nodes.remove(index);
            if let Some(index) = nodes.iter().position(|node| node.id == target_id) {
                nodes.insert(index + 1, node);
            }
        }
        doc.refresh();
        changed = true;
        Ok(())
    });
    report(editor, result);
    if changed {
        if let Some(parent) = parent {
            editor.shell.update(|shell| {
                shell.expanded.insert(parent);
            });
        }
        editor.select(vec![id]);
    }
}

fn clear_drop_targets() {
    if let Ok(nodes) = document().query_selector_all(".vellum .drop-target") {
        for index in 0..nodes.length() {
            if let Some(element) = nodes
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            {
                let _ = element.class_list().remove_1("drop-target");
            }
        }
    }
}

fn update_selection(tree: &Element, selected: &[String]) {
    if let Ok(rows) = tree.query_selector_all("[data-layer]") {
        for index in 0..rows.length() {
            if let Some(row) = rows
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            {
                let active = row
                    .get_attribute("data-layer")
                    .is_some_and(|id| selected.contains(&id));
                let _ = row.class_list().toggle_with_force("selected", active);
                let _ = row.set_attribute("aria-selected", if active { "true" } else { "false" });
            }
        }
    }
}

#[component]
pub fn LayerTree(editor: Editor) -> impl IntoView {
    let tree = NodeRef::<leptos::html::Div>::new();
    // Keep rows attached while selection, menus, or unrelated node properties change:
    // replacing a click/drag target interrupts the browser's native gesture sequence.
    let markup = Memo::new(move |_| {
        editor
            .doc
            .with(|doc| editor.shell.with(|shell| render(doc, shell)))
    });
    Effect::new(move || {
        let html = markup.get();
        if let Some(tree) = tree.get() {
            tree.set_inner_html(&html);
            if let Ok(nodes) = tree.query_selector_all("[data-depth]") {
                for index in 0..nodes.length() {
                    if let Some(element) = nodes
                        .item(index)
                        .and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        if let Some(depth) = element.get_attribute("data-depth") {
                            let _ = element.style().set_property("--depth", &depth);
                        }
                    }
                }
            }
            editor
                .selection
                .with_untracked(|selected| update_selection(&tree, selected));
        }
    });
    Effect::new(move || {
        editor.selection.with(|selected| {
            if let Some(tree) = tree.get() {
                update_selection(&tree, selected);
            }
        });
    });
    view! {<div id="layer-tree" class="layer-tree" class:hidden=move||editor.shell.with(|shell|shell.left_tab!="layers") role="tree" aria-label="Layers" node_ref=tree
        on:click=move|event|click(editor,event)
        on:dblclick=move|event:MouseEvent|{if let Some(id)=closest(&event,"[data-layer]").and_then(|row|row.get_attribute("data-layer")){editor.select(vec![id.clone()]);dialogs::prompt(editor,dialogs::PromptKind::RenameLayer(id));}}
        on:contextmenu=move|event:MouseEvent|{if let Some(id)=closest(&event,"[data-layer]").and_then(|row|row.get_attribute("data-layer")){event.prevent_default();if !editor.selection.with_untracked(|selected|selected.contains(&id)){editor.select(vec![id]);}super::menus::open_selection(editor,Some(crate::affine::Point::new(event.client_x()as f64,event.client_y()as f64)));}}
        on:dragstart=move|event:DragEvent|{if let (Some(id),Some(data))=(closest(&event,"[data-layer]").and_then(|row|row.get_attribute("data-layer")),event.data_transfer()){let _=data.set_data("application/x-vellum-layer",&id);data.set_effect_allowed("move");}}
        on:dragover=move|event:DragEvent|{if event.data_transfer().is_some_and(|data|data.types().includes(&"application/x-vellum-layer".into(),0)){if let Some(row)=closest(&event,"[data-layer]"){event.prevent_default();clear_drop_targets();let _=row.class_list().add_1("drop-target");}}}
        on:dragleave=move|event:DragEvent|{if let Some(row)=closest(&event,".drop-target"){let _=row.class_list().remove_1("drop-target");}}
        on:drop=move|event|drop_layer(editor,event)
    ></div>}
}
