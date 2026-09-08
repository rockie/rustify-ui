//! Rustify UI: experimental SDK for composing Leptos CSR DOM with Makepad GPU
//! regions inside one browser page and one wasm module.

pub mod binding;
pub mod diagnostics;
pub mod mount;
pub mod region;
pub mod scheduler;

pub use binding::{duplicate_key, ActionSink};
pub use diagnostics::UiError;
pub use mount::{mount, AppHandle, MountConfig};
pub use region::{GpuRegion, RegionState};
pub use rustify_makepad::RegionId;
pub use scheduler::{Admission, Pace, Scheduler, Seq};

#[cfg(target_arch = "wasm32")]
pub use rustify_makepad::{makepad_widgets, RegionApp};
