//! Controlled DOM behavior without a theme or an automatically loaded skin.
//!
//! Load [`PRIMITIVES_CSS`] explicitly for structural and accessibility rules.

pub mod accessible;
pub mod accordion;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod choice;
pub mod collapsible;
pub mod dialog;
pub mod direction;
pub mod field;
pub(crate) mod floating;
pub mod form;
mod hover;
pub mod hover_card;
#[cfg(any(target_arch = "wasm32", test))]
mod hover_grace;
mod loading;
pub mod popover;
pub mod portal;
pub mod progress;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod slot;
pub mod spinner;
pub mod tabs;
pub mod toast;
pub mod tooltip;

pub use accessible::{AccessibleIcon, VisuallyHidden};
pub use direction::DirectionProvider;
pub use portal::Portal;
pub use rustify_ui::{provide_direction, use_direction, Cancelable, Direction};
pub use slot::Slot;

/// Minimal, theme-independent structural styles for primitive parts.
pub const PRIMITIVES_CSS: &str = crate::styles::PRIMITIVES;

pub use choice::{
    checkbox, checkbox_group, radio, radio_group, switch, toggle, toggle_group, CheckedState,
    ItemAttributes, SelectionMode,
};

pub use field::{color_field, number_field, text_area, text_field, Label, SlotSide};
