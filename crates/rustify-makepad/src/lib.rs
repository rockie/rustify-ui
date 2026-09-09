//! Private Makepad integration for Rustify UI.
//!
//! The Makepad platform layer is only compiled for `wasm32`; everything that
//! can be reasoned about without a browser (region identity, ordering) lives
//! in the target-independent modules so it can be unit tested on the host.

mod pace;
mod registry;

pub use pace::Pace;
pub use registry::{RegionId, Registry};

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
