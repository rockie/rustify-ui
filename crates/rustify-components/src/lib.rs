//! DOM components for applications built on the Rustify UI SDK.
//!
//! The class strings and the two macros that build them come from Rust/UI; the
//! provenance of every imported file, and whether it is still verbatim, is in
//! the `rust_ui` section of `sources.lock.json`. What the fork changes is the
//! part a browser can observe: no inline script or style, a value that only
//! ever comes from the application, and a name, role, value and state for
//! everything a person can reach.

pub mod button;
pub mod catalog;
pub mod checkbox;
mod dom;
#[cfg(target_arch = "wasm32")]
pub mod drop_zone;
#[cfg(target_arch = "wasm32")]
pub mod file_picker;
pub mod form;
pub mod icon;
pub mod id;
pub mod input;
pub mod label;
pub mod link;
pub mod macros;
pub mod progress;
pub mod radio;
pub mod roving;
pub mod scroll_area;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod workspace;

// The four that float. They stand on the SDK's overlay stack - positioning,
// the Escape order and the return of focus are its, not theirs - and the stack
// only exists in a browser.
#[cfg(target_arch = "wasm32")]
pub mod dialog;
#[cfg(target_arch = "wasm32")]
pub mod menu;
#[cfg(target_arch = "wasm32")]
pub mod select;
#[cfg(target_arch = "wasm32")]
pub mod tooltip;

pub use button::{Button, ButtonSize, ButtonVariant};
pub use catalog::{Capability, Category, Entry, Presentation, Support, CATALOG};
pub use checkbox::Checkbox;
#[cfg(target_arch = "wasm32")]
pub use drop_zone::DropZone;
#[cfg(target_arch = "wasm32")]
pub use file_picker::FilePicker;
pub use form::{provide_form, use_form, Field, FieldBinding, Form, FormStatus, SubmitButton};
pub use icon::{Glyph, Icon};
pub use input::{TextField, TextKind};
pub use label::Label;
pub use link::{Link, LinkMatch};
pub use progress::Progress;
pub use radio::{RadioGroup, RadioOption};
pub use scroll_area::{Boundary, ScrollArea};
pub use slider::Slider;
pub use spinner::Spinner;
pub use switch::Switch;
pub use tabs::{Orientation, Tab, TabPanel, Tabs};
pub use textarea::TextArea;
#[cfg(target_arch = "wasm32")]
pub use workspace::{Command, CommandPalette};
pub use workspace::{PanelTab, PanelTabs, Splitter};

#[cfg(target_arch = "wasm32")]
pub use dialog::Dialog;
#[cfg(target_arch = "wasm32")]
pub use menu::{Menu, MenuItem};
#[cfg(target_arch = "wasm32")]
pub use select::{Select, SelectOption};
#[cfg(target_arch = "wasm32")]
pub use tooltip::Tooltip;

/// Re-exported for the macros: a caller writes `clx!` or `variants!` without
/// having to take a dependency on the crates their expansion happens to use.
pub use paste;
pub use tw_merge;

/// The path the application is currently showing, for the components that need
/// to know whether they point at it.
///
/// This crate has no router. A link marks itself current by comparing its
/// `href` with this signal, and an application that provides no signal simply
/// has no current link - which is the right answer for a page that is not
/// navigating anywhere.
#[derive(Clone, Copy)]
pub struct CurrentPath(pub leptos::prelude::Signal<String>);

/// Publishes the current path to every component below this point.
pub fn provide_current_path(path: leptos::prelude::Signal<String>) {
    leptos::prelude::provide_context(CurrentPath(path));
}

pub fn current_path() -> Option<leptos::prelude::Signal<String>> {
    leptos::prelude::use_context::<CurrentPath>().map(|path| path.0)
}

/// Whether `path` is at, or below, `target`.
///
/// Below counts, so a link to `/objects` stays marked while the page shows
/// `/objects/17`. The segment boundary is what stops `/objects-archive` from
/// counting as one of them.
pub fn is_within(path: &str, target: &str) -> bool {
    if target.is_empty() {
        return false;
    }
    let path = path.trim_end_matches('/');
    let target = target.trim_end_matches('/');
    if target.is_empty() {
        return path.is_empty();
    }
    path == target
        || path
            .strip_prefix(target)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::is_within;

    #[test]
    fn a_link_is_current_for_its_own_page_and_the_pages_under_it() {
        assert!(is_within("/objects", "/objects"));
        assert!(is_within("/objects/17", "/objects"));
        assert!(is_within("/objects/17/notes", "/objects"));
        assert!(is_within("/objects/", "/objects"));
    }

    #[test]
    fn a_link_is_not_current_for_a_path_that_merely_starts_the_same() {
        // The bug this exists to prevent: `starts_with` alone marks the
        // archive link current while the user is looking at the object list.
        assert!(!is_within("/objects-archive", "/objects"));
        assert!(!is_within("/other", "/objects"));
        assert!(!is_within("/", "/objects"));
    }

    #[test]
    fn a_link_with_no_target_is_never_current() {
        assert!(!is_within("/objects", ""));
    }
}

/// The token table and the stylesheet have to name the same things.
///
/// The SDK writes `--primary` onto the scope root; a utility reads
/// `var(--primary)`. Either one renamed alone leaves a component drawing with
/// whatever the browser falls back to, which looks like a theme bug and is a
/// spelling one. The check is here, in the crate that has both in reach.
#[cfg(test)]
mod stylesheet {
    const INPUT: &str = include_str!("../css/rustify.tailwind.css");
    const OUTPUT: &str = include_str!("../css/rustify.css");

    #[test]
    fn every_token_the_sdk_writes_is_a_token_the_stylesheet_names() {
        for (name, _) in rustify_ui::Theme::light().properties() {
            assert!(
                INPUT.contains(&format!("{name}:")),
                "{name} is written by the theme and named nowhere in the stylesheet input"
            );
        }
    }

    #[test]
    fn the_stylesheet_resets_nothing_and_selects_no_bare_element() {
        // Preflight would reset the host page's own elements; a bare element
        // selector would restyle them. Neither is allowed into the product,
        // and both are easy to reintroduce with one import.
        assert!(
            !OUTPUT.contains("@layer base"),
            "preflight is in the product"
        );
        for line in OUTPUT.lines() {
            let selector = line.trim_end_matches(" {");
            if !line.ends_with(" {") || !selector.starts_with(char::is_alphabetic) {
                continue;
            }
            // What is left is either an at-rule or a bare element selector.
            assert!(
                !matches!(
                    selector,
                    "html" | "body" | "input" | "button" | "a" | "select" | "textarea" | "*"
                ),
                "{selector} selects an element of the host page"
            );
        }
    }

    #[test]
    fn dark_is_this_scope_in_dark_and_not_any_dark_ancestor() {
        // A host page using the shadcn `.dark` convention must not change what
        // a component in our scope looks like.
        assert!(INPUT
            .contains(r#"@custom-variant dark (&:is([data-rustify-scope][data-theme="dark"] *))"#));
    }
}
