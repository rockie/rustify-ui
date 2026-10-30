//! Controlled DOM behavior without a theme or an automatically loaded skin.
//!
//! Load [`PRIMITIVES_CSS`] explicitly for structural and accessibility rules.

pub mod accessible;
pub mod choice;
pub mod direction;
pub mod field;
pub mod form;
pub mod portal;
pub mod select;
pub mod slider;
pub mod slot;

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
