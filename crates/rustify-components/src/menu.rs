//! Data-driven commands assembled from the shared themed menu parts.

use rustify_ui::Shortcut;

#[cfg(target_arch = "wasm32")]
pub use dom::anchor_at;
pub use dom::Menu;

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

mod dom {
    use super::{blocks, Block, MenuItem};
    use crate::dropdown_menu as parts;
    use leptos::prelude::*;
    use rustify_ui::Anchor;

    /// A point in the viewport, retained relative to the anchor as it scrolls.
    #[cfg(target_arch = "wasm32")]
    pub fn anchor_at(element: &leptos::web_sys::Element, client_x: f64, client_y: f64) -> Anchor {
        let bounds = element.get_bounding_client_rect();
        Anchor::region(
            element,
            rustify_ui::LocalRect::new(client_x - bounds.left(), client_y - bounds.top(), 0.0, 0.0),
        )
    }

    /// Data-driven commands with controlled state and an application-owned anchor.
    #[component]
    pub fn Menu(
        #[prop(into)] open: Signal<bool>,
        on_open_change: impl Fn(bool) + Send + Sync + 'static,
        #[prop(into)] anchor: Signal<Anchor>,
        #[prop(into)] items: Signal<Vec<MenuItem>>,
        on_activate: impl Fn(String) + Send + Sync + 'static,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let group = StoredValue::new(crate::id::next("menu"));
        let on_activate = StoredValue::new(on_activate);
        let drawn = Memo::new(move |_| items.with(|items| blocks(items)));
        let command = move |item: MenuItem| {
            let MenuItem {
                id,
                label,
                disabled,
                reason,
                shortcut,
                ..
            } = item;
            let reason_id = format!("{}-item-{id}-reason", group.get_value());
            let described = if reason.is_empty() {
                String::new()
            } else {
                reason_id.clone()
            };
            let asked = id.clone();
            let keys = shortcut.map(|shortcut| shortcut.to_string());
            let announced = keys.clone();
            parts::Item(parts::ItemProps::builder()
                .on_select(move |_|on_activate.with_value(|run|run(asked.clone())))
                .disabled(disabled).text_value(label.clone()).described_by(described)
                .test_id(format!("menu-item-{id}"))
                .children(Box::new(move ||view! {
                    <span class="flex-1">{label}</span>
                    {(!reason.is_empty()).then(||view!{<span id=reason_id class="text-xs text-muted-foreground">{reason}</span>})}
                    {keys.map(|keys|view!{<kbd class="rustify-menu-shortcut" data-name="MenuShortcut" data-testid=format!("menu-shortcut-{id}") aria-hidden="true">{keys}</kbd>})}
                }.into_any())).build())
                .add_any_attr((leptos::attr::aria_keyshortcuts(announced),leptos::attr::custom::custom_attribute("data-name","MenuItem")))
        };
        let rows = move || {
            drawn.get().into_iter().map(|block|match block {
            Block::Separator=>view!{<parts::Separator />}.into_any(),
            Block::Group {heading:None,items}=>items.into_iter().map(command).collect_view().into_any(),
            Block::Group {heading:Some(heading),items}=>view!{<parts::Group><parts::Label>{heading}</parts::Label>{items.into_iter().map(command).collect_view()}</parts::Group>}.into_any(),
        }).collect_view()
        };
        crate::primitives::menu::anchored_root(
            open,
            Callback::new(on_open_change),
            anchor,
            Box::new(move || {
                view!{
            <parts::Portal><parts::Content loop_focus=true aria_label=aria_label.clone() class=class.clone() test_id=test_id.clone()>{rows}</parts::Content></parts::Portal>
        }.into_any()
            }),
        )
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
