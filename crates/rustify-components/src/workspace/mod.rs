//! The pieces a workspace is made of: panels side by side, tabs over them, and
//! everything the application can do in one list.

// It stands on the SDK's overlay stack, which only exists in a browser.
#[cfg(target_arch = "wasm32")]
pub mod command_palette;
pub mod panel_tabs;
pub mod splitter;

#[cfg(target_arch = "wasm32")]
pub use command_palette::{Command, CommandPalette};
pub use panel_tabs::{PanelTab, PanelTabs};
pub use splitter::{columns, Splitter};
