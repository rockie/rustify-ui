//! Rustify UI: experimental SDK for composing Leptos CSR DOM with Makepad GPU
//! regions inside one browser page and one wasm module.

pub mod diagnostics;
pub mod mount;
pub mod region;

pub use diagnostics::UiError;
pub use mount::{mount, AppHandle, MountConfig};
pub use region::GpuRegion;
pub use rustify_makepad::RegionId;

#[cfg(target_arch = "wasm32")]
pub use rustify_makepad::{makepad_widgets, RegionApp};
