//! Real DOM examples, registered individually by the documentation site.

pub mod basics;
pub mod choices;
pub mod collections;
pub mod feedback;
pub mod fields;

pub use basics::{button_demo, icon_demo, label_demo, link_demo, loading_demo};
pub use choices::{
    checkbox_demo, radio_demo, select_demo, slider_demo, switch_demo, toggle_group_demo,
};
pub use collections::{data_table_demo, scroll_area_demo, tabs_demo, tree_demo};
pub use feedback::{dialog_demo, menu_demo, progress_demo, toast_demo, tooltip_demo};
pub use fields::{color_field_demo, number_field_demo, text_area_demo, text_field_demo};
