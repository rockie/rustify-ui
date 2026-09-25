//! Two levels of groups, for getting about rather than for holding anything.
//!
//! Not a general tree: no arbitrary depth, no virtualisation, no drag. What it
//! is for is the one thing a hundred thousand rows need that a scrollbar
//! cannot give - a way to say "that part" - and the smallest thing that does
//! that is a list of groups you can open.
//!
//! The keyboard is the part a tree is usually wrong about. One tab stop for
//! the whole tree; the arrows move within it; right opens a closed group and
//! steps into an open one, left closes an open one and steps out of a child.

#[cfg(target_arch = "wasm32")]
use leptos::ev::KeyboardEvent;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
use std::collections::BTreeSet;
#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
const TREE: &str = "flex flex-col gap-0.5 overflow-auto p-1 text-sm outline-none";
#[cfg(target_arch = "wasm32")]
const ITEM: &str = "flex w-full items-center gap-1 rounded px-2 py-1 text-left cursor-pointer select-none outline-none hover:bg-accent/50 focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-selected:bg-accent";

/// One group. A group with children is a level the arrows can open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub key: String,
    pub label: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            children: Vec::new(),
        }
    }

    pub fn with(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

/// One row of the tree as it is currently shown: a closed group's children are
/// not here at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub key: String,
    pub label: String,
    pub depth: usize,
    pub expandable: bool,
    pub expanded: bool,
    /// The group this one is inside, for the left arrow to step out to.
    pub parent: Option<String>,
}

/// The tree flattened to what a person can see and walk.
pub fn rows(nodes: &[TreeNode], expanded: &BTreeSet<String>) -> Vec<Row> {
    let mut out = Vec::new();
    for node in nodes {
        let open = expanded.contains(&node.key);
        out.push(Row {
            key: node.key.clone(),
            label: node.label.clone(),
            depth: 0,
            expandable: !node.children.is_empty(),
            expanded: open,
            parent: None,
        });
        if !open {
            continue;
        }
        for child in &node.children {
            out.push(Row {
                key: child.key.clone(),
                label: child.label.clone(),
                depth: 1,
                expandable: false,
                expanded: false,
                parent: Some(node.key.clone()),
            });
        }
    }
    out
}

/// What a key does to the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    /// Put the keyboard on this group.
    To(String),
    /// Open or close this group.
    Toggle(String),
    /// Choose this group.
    Select(String),
}

/// What `key` does when the keyboard is on `at`, or `None` when the tree does
/// not claim it.
pub fn command(key: &str, at: &str, rows: &[Row]) -> Option<Move> {
    if rows.is_empty() {
        return None;
    }
    let here = rows.iter().position(|row| row.key == at);
    let step = |delta: i32| {
        crate::roving::step(rows.len(), |_| true, here, delta)
            .map(|index| Move::To(rows[index].key.clone()))
    };
    match key {
        "ArrowDown" => step(1),
        "ArrowUp" => step(-1),
        "Home" => Some(Move::To(rows[0].key.clone())),
        "End" => Some(Move::To(rows[rows.len() - 1].key.clone())),
        "ArrowRight" => {
            let row = rows.get(here?)?;
            if row.expandable && !row.expanded {
                return Some(Move::Toggle(row.key.clone()));
            }
            // An open group's right arrow steps into it; a leaf's does
            // nothing, which is what the tree not claiming the key means.
            row.expanded
                .then(|| rows.get(here? + 1).map(|next| Move::To(next.key.clone())))
                .flatten()
        }
        "ArrowLeft" => {
            let row = rows.get(here?)?;
            if row.expandable && row.expanded {
                return Some(Move::Toggle(row.key.clone()));
            }
            row.parent.clone().map(Move::To)
        }
        " " | "Enter" => Some(Move::Select(at.to_string())),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn Tree(
    #[prop(into)] nodes: Signal<Vec<TreeNode>>,
    /// Which groups are open. Owned outside, because what is open is part of
    /// what a person is looking at rather than of the tree's own furniture.
    expanded: RwSignal<BTreeSet<String>>,
    /// The group whose rows are being shown, if any.
    #[prop(into)]
    selected: Signal<Option<String>>,
    on_select: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let name = if test_id.is_empty() {
        "tree".to_string()
    } else {
        test_id.clone()
    };
    let focused = RwSignal::new(String::new());
    let on_select = Arc::new(on_select);
    let shown = Memo::new(move |_| rows(&nodes.get(), &expanded.get()));
    // The keyboard starts on the first group, and goes back to it if whatever
    // it was on has gone.
    let current = Memo::new(move |_| {
        let rows = shown.get();
        let at = focused.get();
        if rows.iter().any(|row| row.key == at) {
            return at;
        }
        rows.first().map(|row| row.key.clone()).unwrap_or_default()
    });
    let item_id = {
        let name = name.clone();
        move |key: &str| format!("{name}-item-{key}")
    };
    let on_keydown = {
        let on_select = on_select.clone();
        let item_id = item_id.clone();
        move |ev: KeyboardEvent| {
            let Some(command) =
                command(&ev.key(), &current.get_untracked(), &shown.get_untracked())
            else {
                return;
            };
            ev.prevent_default();
            match command {
                Move::To(key) => {
                    focused.set(key.clone());
                    crate::dom::focus_id(&item_id(&key));
                }
                Move::Toggle(key) => expanded.update(|open| {
                    if !open.remove(&key) {
                        open.insert(key);
                    }
                }),
                Move::Select(key) => on_select(key),
            }
        }
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    view! {
        <div
            class=crate::macros::merge(TREE, &class)
            data-name="Tree"
            data-testid=test_id
            role="tree"
            aria-label=aria_label
            on:keydown=on_keydown
        >
            <For each=move || shown.get() key=|row| row.key.clone() let:row>
                {
                    let key = row.key.clone();
                    let on_select = on_select.clone();
                    let item_id = item_id.clone();
                    let mine = key.clone();
                    let chosen = Memo::new(move |_| selected.get().as_deref() == Some(&mine));
                    let tabbable = {
                        let mine = key.clone();
                        Memo::new(move |_| current.get() == mine)
                    };
                    // Whether this group is open has to be read rather than
                    // captured: the list is keyed by the group, so opening one
                    // keeps the element it already has and only its children
                    // come and go.
                    let open = {
                        let mine = key.clone();
                        Memo::new(move |_| expanded.get().contains(&mine))
                    };
                    let expandable = row.expandable;
                    let toggle_key = key.clone();
                    let select_key = key.clone();
                    view! {
                        <div
                            role="treeitem"
                            id=item_id(&key)
                            class=ITEM
                            data-testid=format!("{}-{}", name.clone(), key)
                            {leptos::tachys::html::attribute::custom::custom_attribute(
                                "aria-level",
                                (row.depth + 1).to_string(),
                            )}
                            aria-expanded=move || expandable.then(|| open.get().to_string())
                            aria-selected=move || chosen.get().to_string()
                            tabindex=move || if tabbable.get() { "0" } else { "-1" }
                            style:padding-left=format!("{}px", 8 + row.depth * 16)
                            on:click=move |_| {
                                focused.set(select_key.clone());
                                if expandable {
                                    expanded
                                        .update(|open| {
                                            if !open.remove(&toggle_key) {
                                                open.insert(toggle_key.clone());
                                            }
                                        });
                                }
                                on_select(select_key.clone());
                            }
                        >
                            <span aria-hidden="true">
                                {move || {
                                    if !expandable {
                                        "\u{00b7}"
                                    } else if open.get() {
                                        "\u{25be}"
                                    } else {
                                        "\u{25b8}"
                                    }
                                }}
                            </span>
                            {row.label.clone()}
                        </div>
                    }
                }
            </For>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<TreeNode> {
        vec![
            TreeNode::new("g0", "group 1").with(vec![
                TreeNode::new("g0-0", "group 1.1"),
                TreeNode::new("g0-1", "group 1.2"),
            ]),
            TreeNode::new("g1", "group 2").with(vec![TreeNode::new("g1-0", "group 2.1")]),
        ]
    }

    fn open(keys: &[&str]) -> BTreeSet<String> {
        keys.iter().map(|key| key.to_string()).collect()
    }

    #[test]
    fn a_closed_group_does_not_put_its_children_in_the_document() {
        let shown = rows(&groups(), &open(&[]));
        assert_eq!(
            shown.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(),
            vec!["g0", "g1"]
        );
        assert!(shown.iter().all(|row| row.expandable && !row.expanded));
    }

    #[test]
    fn an_open_group_shows_its_children_below_it() {
        let shown = rows(&groups(), &open(&["g0"]));
        assert_eq!(
            shown.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(),
            vec!["g0", "g0-0", "g0-1", "g1"]
        );
        assert_eq!(shown[1].depth, 1);
        assert_eq!(shown[1].parent.as_deref(), Some("g0"));
        assert!(!shown[1].expandable);
    }

    #[test]
    fn the_arrows_walk_what_is_showing_and_join_the_ends() {
        let shown = rows(&groups(), &open(&["g0"]));
        assert_eq!(
            command("ArrowDown", "g0", &shown),
            Some(Move::To("g0-0".into()))
        );
        assert_eq!(
            command("ArrowUp", "g0", &shown),
            Some(Move::To("g1".into()))
        );
        assert_eq!(command("Home", "g1", &shown), Some(Move::To("g0".into())));
        assert_eq!(command("End", "g0", &shown), Some(Move::To("g1".into())));
    }

    #[test]
    fn right_opens_a_closed_group_and_then_steps_into_it() {
        let closed = rows(&groups(), &open(&[]));
        assert_eq!(
            command("ArrowRight", "g0", &closed),
            Some(Move::Toggle("g0".into()))
        );
        let opened = rows(&groups(), &open(&["g0"]));
        assert_eq!(
            command("ArrowRight", "g0", &opened),
            Some(Move::To("g0-0".into()))
        );
    }

    #[test]
    fn left_closes_an_open_group_and_steps_out_of_a_child() {
        let opened = rows(&groups(), &open(&["g0"]));
        assert_eq!(
            command("ArrowLeft", "g0", &opened),
            Some(Move::Toggle("g0".into()))
        );
        assert_eq!(
            command("ArrowLeft", "g0-1", &opened),
            Some(Move::To("g0".into()))
        );
        // A closed group at the top level has nowhere further out to go.
        let closed = rows(&groups(), &open(&[]));
        assert_eq!(command("ArrowLeft", "g0", &closed), None);
    }

    #[test]
    fn space_and_enter_choose_the_group_the_keyboard_is_on() {
        let shown = rows(&groups(), &open(&[]));
        assert_eq!(command(" ", "g1", &shown), Some(Move::Select("g1".into())));
        assert_eq!(
            command("Enter", "g1", &shown),
            Some(Move::Select("g1".into()))
        );
    }

    #[test]
    fn a_key_the_tree_does_not_claim_is_left_to_the_page() {
        let shown = rows(&groups(), &open(&[]));
        for key in ["Tab", "Escape", "x"] {
            assert_eq!(command(key, "g0", &shown), None, "{key}");
        }
        assert_eq!(command("ArrowDown", "g0", &[]), None);
    }
}
