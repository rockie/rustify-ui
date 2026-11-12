//! Real DOM examples, registered individually by the documentation site.

pub mod basics;
pub mod choices;
pub mod collections;
pub mod colors;
pub mod composition;
pub mod content;
pub mod feedback;
pub mod fields;
pub mod layout;
pub mod menus;
pub mod typography;

pub use basics::{button_demo, icon_button_demo, icon_demo, label_demo, link_demo, loading_demo};
pub use choices::{
    checkbox_cards_demo, checkbox_demo, checkbox_group_demo, radio_cards_demo, radio_demo,
    range_slider_demo, segmented_control_demo, select_demo, slider_demo, switch_demo, toggle_demo,
    toggle_group_demo,
};
pub use collections::{
    accordion_demo, collapsible_demo, command_palette_demo, data_table_demo, drop_zone_demo,
    file_picker_demo, navigation_menu_demo, panel_tabs_demo, scroll_area_demo, splitter_demo,
    tab_nav_demo, tabs_demo, toolbar_demo, tree_demo,
};
pub use colors::{colors_demo, theme_boundary_demo};
pub use composition::{field_parts_demo, form_demo, select_grouped_demo, select_parts_demo};
pub use content::{
    aspect_ratio_demo, avatar_demo, badge_demo, callout_demo, card_demo, data_list_demo,
    scroll_area_parts_demo, separator_demo, table_demo,
};
pub use feedback::{
    alert_dialog_demo, dialog_demo, hover_card_demo, menu_demo, popover_demo, progress_demo,
    skeleton_demo, spinner_demo, toast_demo, tooltip_demo,
};
pub use fields::{color_field_demo, number_field_demo, text_area_demo, text_field_demo};
pub use layout::{
    container_demo, direction_demo, inset_demo, layout_demo, primitives_demo, reset_demo,
};
pub use menus::{context_menu_demo, dropdown_menu_demo, menubar_demo};
pub use typography::{text_controls_demo, typography_demo};
