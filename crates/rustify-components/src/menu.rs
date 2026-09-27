//! A list of commands, opened against something.
//!
//! Rust/UI's dropdown menu was 570 lines of Rust around a `format!`-built
//! `<script>` that measured the trigger, flipped the panel and closed it on an
//! outside click - none of which a strict policy will run. The measuring, the
//! Escape order and the return of focus are the SDK overlay stack's here; what
//! is left is a list with the roles and the arrow keys a menu is expected to
//! have.
//!
//! The items are plain data, so the rules about them - what a heading names,
//! where a separator ends a group, which rows a keyboard can reach - are
//! checked without a browser.

use rustify_ui::Shortcut;

#[cfg(target_arch = "wasm32")]
pub use dom::{anchor_at, Menu};

/// What a row of a menu is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemKind {
    /// Something to run.
    #[default]
    Command,
    /// Names the commands after it, up to the next heading or separator.
    Heading,
    /// A line between two sets of commands.
    Separator,
}

/// One row of a menu: a command, or a heading or separator between them.
///
/// A command that cannot run stays in the list and says why, rather than
/// disappearing: a menu whose contents change with the state is a menu nobody
/// can learn. Headings and separators are there to be read and seen, and the
/// keyboard passes over them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItem {
    /// What activating it asks for. Empty for a heading or a separator.
    pub id: String,
    pub label: String,
    pub disabled: bool,
    /// Why it cannot run. Announced with the item and shown beside it.
    pub reason: String,
    pub kind: MenuItemKind,
    /// The application's own shortcut for the same command, shown beside it.
    /// The menu shows it and announces it; listening for it is the
    /// application's.
    pub shortcut: Option<Shortcut>,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            reason: String::new(),
            kind: MenuItemKind::Command,
            shortcut: None,
        }
    }

    /// A heading over the commands that follow it.
    pub fn heading(label: impl Into<String>) -> Self {
        Self {
            kind: MenuItemKind::Heading,
            ..Self::new("", label)
        }
    }

    /// A line between the commands before it and the ones after.
    pub fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
            ..Self::new("", "")
        }
    }

    pub fn disabled(mut self, reason: impl Into<String>) -> Self {
        self.disabled = true;
        self.reason = reason.into();
        self
    }

    pub fn shortcut(mut self, shortcut: Shortcut) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Whether an arrow key can land on it.
    pub fn reachable(&self) -> bool {
        self.kind == MenuItemKind::Command && !self.disabled
    }
}

/// The rows as the menu draws them.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, PartialEq)]
enum Block {
    Separator,
    /// A run of commands, named by the heading over it if there is one.
    Group {
        heading: Option<String>,
        items: Vec<MenuItem>,
    },
}

/// Groups the rows: a heading opens a group of its own, a separator closes
/// the one before it, and commands join whichever group is open.
///
/// A group exists so that a reader hears the heading as the name of the
/// commands under it rather than as one more line of the menu.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blocks(items: &[MenuItem]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for item in items {
        match item.kind {
            MenuItemKind::Heading => blocks.push(Block::Group {
                heading: Some(item.label.clone()),
                items: Vec::new(),
            }),
            MenuItemKind::Separator => blocks.push(Block::Separator),
            MenuItemKind::Command => match blocks.last_mut() {
                Some(Block::Group { items, .. }) => items.push(item.clone()),
                _ => blocks.push(Block::Group {
                    heading: None,
                    items: vec![item.clone()],
                }),
            },
        }
    }
    blocks
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{blocks, Block, MenuItem};
    use leptos::ev::KeyboardEvent;
    use leptos::prelude::*;
    use leptos::web_sys::Element;
    use rustify_ui::{Anchor, Layer, LocalRect};

    // `m-0` and `list-none` because nothing resets a list for us: the scope
    // leaves the host page's defaults alone, and a list's are an em of margin
    // above and below and a marker beside every row.
    const PANEL: &str =
        "m-0 min-w-40 list-none rounded-md border border-border bg-popover p-1 shadow-lg";
    const GROUP: &str = "m-0 p-0";
    const ITEM: &str = "flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm text-foreground text-left transition-colors outline-none cursor-pointer hover:bg-muted focus-visible:bg-muted aria-disabled:opacity-50 aria-disabled:cursor-not-allowed aria-disabled:hover:bg-transparent";
    const HEADING: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground select-none";
    const SEPARATOR: &str = "-mx-1 my-1 h-px bg-border";
    const HINT: &str = "ms-auto ps-4 text-xs tracking-widest text-muted-foreground";

    /// Where a menu asked for by a pointer opens: at the point the pointer
    /// was, held relative to `element`, so that the menu moves with the
    /// element when anything around it scrolls.
    ///
    /// `client_x` and `client_y` are the event's own, in the viewport.
    pub fn anchor_at(element: &Element, client_x: f64, client_y: f64) -> Anchor {
        let bounds = element.get_bounding_client_rect();
        Anchor::region(
            element,
            LocalRect::new(client_x - bounds.left(), client_y - bounds.top(), 0.0, 0.0),
        )
    }

    /// A menu. It stays inside the viewport: it moves left rather than past
    /// the right edge, and opens above its anchor when there is no room below.
    #[component]
    pub fn Menu(
        #[prop(into)] open: Signal<bool>,
        on_open_change: impl Fn(bool) + Send + Sync + 'static,
        /// What the menu is placed against: an element of the scope, a
        /// rectangle inside a GPU region, or a point ([`anchor_at`]).
        #[prop(into)]
        anchor: Signal<Anchor>,
        #[prop(into)] items: Signal<Vec<MenuItem>>,
        on_activate: impl Fn(String) + Send + Sync + 'static,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let group = StoredValue::new(crate::id::next("menu"));
        let panel_class = StoredValue::new(crate::macros::merge(PANEL, &class));
        let panel_test_id = StoredValue::new(test_id);
        let label = StoredValue::new((!aria_label.is_empty()).then_some(aria_label));
        // Stored rather than cloned: `Show` and `Layer` each take a children
        // function, and a value moved through them makes the outermost
        // callable once.
        let on_open_change = StoredValue::new(on_open_change);
        let on_activate = StoredValue::new(on_activate);
        let list = NodeRef::<leptos::html::Ul>::new();
        // Only a change to the rows redraws them: a signal that sends the same
        // list again would otherwise take the keyboard off the row it is on.
        let drawn = Memo::new(move |_| items.with(|items| blocks(items)));

        let item_id = move |id: &str| format!("{}-item-{id}", group.get_value());

        // The keyboard lands in the menu, not behind it: a menu is not modal,
        // so the stack does not move focus for it.
        Effect::new(move || {
            if !open.get() || list.get().is_none() {
                return;
            }
            let items = items.get_untracked();
            if let Some(first) =
                crate::roving::edge(items.len(), |index| items[index].reachable(), false)
            {
                crate::dom::focus_id(&item_id(&items[first].id));
            }
        });

        let close = move || on_open_change.with_value(|close| close(false));

        let on_keydown = move |ev: KeyboardEvent| {
            let items = items.get_untracked();
            let here = leptos::prelude::document()
                .active_element()
                .and_then(|active| active.get_attribute("data-item"))
                .and_then(|id| {
                    items
                        .iter()
                        .position(|item| item.reachable() && item.id == id)
                });
            let reachable = |index: usize| items[index].reachable();
            let next = match ev.key().as_str() {
                "ArrowDown" => crate::roving::step(items.len(), reachable, here, 1),
                "ArrowUp" => crate::roving::step(items.len(), reachable, here, -1),
                "Home" => crate::roving::edge(items.len(), reachable, false),
                "End" => crate::roving::edge(items.len(), reachable, true),
                _ => None,
            };
            let Some(next) = next else {
                return;
            };
            ev.prevent_default();
            crate::dom::focus_id(&item_id(&items[next].id));
        };

        let command = move |item: MenuItem| {
            let MenuItem {
                id,
                label,
                disabled,
                reason,
                shortcut,
                ..
            } = item;
            let element_id = item_id(&id);
            let reason_id = format!("{element_id}-reason");
            let described = (!reason.is_empty()).then(|| reason_id.clone());
            let asked = id.clone();
            let keys = shortcut.map(|shortcut| shortcut.to_string());
            let announced = keys.clone();
            view! {
                <li role="none">
                    <button
                        type="button"
                        role="menuitem"
                        id=element_id
                        class=ITEM
                        data-name="MenuItem"
                        data-item=id.clone()
                        data-testid=format!("menu-item-{id}")
                        aria-disabled=disabled.then_some("true")
                        aria-describedby=described
                        aria-keyshortcuts=announced
                        on:click=move |_| {
                            if disabled {
                                return;
                            }
                            on_activate.with_value(|run| run(asked.clone()));
                            on_open_change.with_value(|close| close(false));
                        }
                    >
                        <span class="flex-1">{label}</span>
                        {(!reason.is_empty())
                            .then(|| {
                                view! {
                                    <span id=reason_id class="text-xs text-muted-foreground">
                                        {reason}
                                    </span>
                                }
                            })}
                        // Seen, and not read out a second time: the item
                        // already carries it as `aria-keyshortcuts`.
                        {keys
                            .map(|keys| {
                                view! {
                                    <kbd
                                        class=HINT
                                        data-name="MenuShortcut"
                                        data-testid=format!("menu-shortcut-{id}")
                                        aria-hidden="true"
                                    >
                                        {keys}
                                    </kbd>
                                }
                            })}
                    </button>
                </li>
            }
        };

        let rows = move || {
            drawn
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, block)| match block {
                    Block::Separator => view! {
                        <li role="separator" class=SEPARATOR data-name="MenuSeparator" />
                    }
                    .into_any(),
                    Block::Group {
                        heading: None,
                        items,
                    } => items.into_iter().map(command).collect_view().into_any(),
                    Block::Group {
                        heading: Some(heading),
                        items,
                    } => {
                        let heading_id = format!("{}-heading-{index}", group.get_value());
                        let labelled_by = heading_id.clone();
                        // The heading names the group rather than being a row
                        // of its own: hidden from the tree, it is still what
                        // `aria-labelledby` reads.
                        view! {
                            <li role="none">
                                <ul role="group" class=GROUP aria-labelledby=labelled_by>
                                    <li
                                        id=heading_id
                                        class=HEADING
                                        data-name="MenuHeading"
                                        aria-hidden="true"
                                    >
                                        {heading}
                                    </li>
                                    {items.into_iter().map(command).collect_view()}
                                </ul>
                            </li>
                        }
                        .into_any()
                    }
                })
                .collect_view()
        };

        view! {
            <Show when=move || open.get() fallback=|| ()>
                <Layer anchor=anchor fit=true on_close=close class="rui-menu-layer">
                    <ul
                        node_ref=list
                        class=move || panel_class.get_value()
                        data-name="Menu"
                        data-testid=move || panel_test_id.get_value()
                        role="menu"
                        aria-label=move || label.get_value()
                        on:keydown=on_keydown
                    >
                        {rows}
                    </ul>
                </Layer>
            </Show>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{blocks, Block, MenuItem, MenuItemKind};
    use rustify_ui::{Platform, Shortcut};

    fn group(heading: Option<&str>, ids: &[&str]) -> Block {
        Block::Group {
            heading: heading.map(str::to_string),
            items: ids.iter().map(|id| MenuItem::new(*id, *id)).collect(),
        }
    }

    #[test]
    fn a_heading_names_the_commands_up_to_the_next_heading_or_separator() {
        let items = [
            MenuItem::new("a", "a"),
            MenuItem::heading("edit"),
            MenuItem::new("b", "b"),
            MenuItem::new("c", "c"),
            MenuItem::separator(),
            MenuItem::new("d", "d"),
            MenuItem::heading("view"),
            MenuItem::new("e", "e"),
        ];
        assert_eq!(
            blocks(&items),
            [
                group(None, &["a"]),
                group(Some("edit"), &["b", "c"]),
                Block::Separator,
                group(None, &["d"]),
                group(Some("view"), &["e"]),
            ]
        );
    }

    #[test]
    fn a_menu_of_plain_commands_is_one_unnamed_group() {
        let items = [MenuItem::new("a", "a"), MenuItem::new("b", "b")];
        assert_eq!(blocks(&items), [group(None, &["a", "b"])]);
    }

    #[test]
    fn the_keyboard_reaches_only_commands_that_can_run() {
        assert!(MenuItem::new("a", "a").reachable());
        assert!(!MenuItem::new("a", "a").disabled("why").reachable());
        assert!(!MenuItem::heading("edit").reachable());
        assert!(!MenuItem::separator().reachable());
        assert_eq!(MenuItem::separator().kind, MenuItemKind::Separator);
    }

    #[test]
    fn a_shortcut_is_shown_the_way_aria_keyshortcuts_names_it() {
        let shortcut = Shortcut::parse_for("Mod+Shift+K", Platform::Other).unwrap();
        let item = MenuItem::new("palette", "all commands").shortcut(shortcut);
        assert_eq!(
            item.shortcut.map(|shortcut| shortcut.to_string()),
            Some("Control+Shift+K".to_string())
        );
    }
}
