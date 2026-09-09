//! Rustify UI: experimental SDK for composing Leptos CSR DOM with Makepad GPU
//! regions inside one browser page and one wasm module.

pub mod binding;
pub mod catalog;
pub mod components;
pub mod diagnostics;
#[cfg(target_arch = "wasm32")]
pub mod gpu;
pub mod mount;
pub mod overlay;
pub mod region;
pub mod scheduler;
pub mod task;
pub mod text;
pub mod theme;

pub use binding::{duplicate_key, ActionSink};
pub use catalog::{Capability, Category, Entry, Presentation, Support, CATALOG};
pub use components::snap;
pub use diagnostics::UiError;
pub use mount::{mount, AppHandle, MountConfig, ScopeRoots};
pub use overlay::{LayerId, LocalRect};
pub use region::{GpuRegion, RegionState};
pub use rustify_makepad::RegionId;
pub use scheduler::{Admission, Pace, Scheduler, Seq};
pub use task::{Load, Requests, Ticket};

#[cfg(target_arch = "wasm32")]
pub use components::{Button, Checkbox, Label, LoadView, Slider, TextArea, TextField};
#[cfg(target_arch = "wasm32")]
pub use gpu::{RustifyCheckBox, RustifySlider};
#[cfg(target_arch = "wasm32")]
pub use overlay::{use_overlay, Anchor, Layer, OverlayStack};
#[cfg(target_arch = "wasm32")]
pub use rustify_makepad::{makepad_widgets, RegionApp};
#[cfg(target_arch = "wasm32")]
pub use text::TextEdit;
#[cfg(target_arch = "wasm32")]
pub use theme::{use_theme, use_theme_values, ThemeOverride, ThemedScope};
pub use theme::{Theme, ThemePatch};
