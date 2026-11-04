//! Themed point-anchored command menus.
use crate::dropdown_menu::{content, surface, Appearance, Surface};
pub use crate::dropdown_menu::{
    Arrow, CheckboxItem, Group, Item, ItemIndicator, Label, MenuSize, MenuVariant, Portal,
    RadioGroup, RadioItem, Separator, Sub, SubContent, SubTrigger,
};
pub use crate::primitives::context_menu::{trigger_as, Root, Trigger};
use leptos::prelude::*;
use rustify_ui::{AccentColor, Cancelable, FloatingPosition, LayerDismiss};
content!(
    Content,
    Context,
    Signal::default(),
    Signal::default(),
    Signal::default(),
    Signal::default()
);
