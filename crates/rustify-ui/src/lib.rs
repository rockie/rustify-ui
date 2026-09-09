//! Rustify UI: experimental SDK for composing Leptos CSR DOM with Makepad GPU
//! regions inside one browser page and one wasm module.

pub mod binding;
pub mod diagnostics;
pub mod mount;
pub mod overlay;
pub mod region;
pub mod scheduler;
pub mod task;
pub mod text;
pub mod theme;

pub use binding::{duplicate_key, ActionSink};
pub use diagnostics::UiError;
pub use mount::{mount, AppHandle, MountConfig, ScopeRoots};
pub use overlay::{LayerId, LocalRect};
pub use region::{GpuRegion, RegionState};
pub use rustify_makepad::RegionId;
pub use scheduler::{Admission, Pace, Scheduler, Seq};
pub use task::{Load, Requests, Ticket};

#[cfg(target_arch = "wasm32")]
pub use overlay::{use_overlay, Anchor, Layer, OverlayStack};
#[cfg(target_arch = "wasm32")]
pub use rustify_makepad::{makepad_widgets, RegionApp};
#[cfg(target_arch = "wasm32")]
pub use text::TextEdit;
pub use theme::Theme;
#[cfg(target_arch = "wasm32")]
pub use theme::{use_theme, ThemedScope};
