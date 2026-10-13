//! The pieces a workspace is made of: panels side by side, tabs over them, and
//! everything the application can do in one list.

// Its closed initial view is shared; opening uses the SDK overlay stack.
pub mod command_palette;
pub mod panel_tabs;
pub mod splitter;

pub use command_palette::{Command, CommandPalette};
pub use panel_tabs::{PanelTab, PanelTabs};
pub use splitter::{columns, Splitter};
