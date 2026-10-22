//! Private Makepad integration for Rustify UI.
//!
//! The Makepad platform layer is only compiled for `wasm32`; everything that
//! can be reasoned about without a browser (region identity, ordering) lives
//! in the target-independent modules so it can be unit tested on the host.
//!
//! It depends on the DOM core, and never the other way round: the core knows
//! nothing about regions, and the one type the two share - the pace of an
//! action - is defined by the core and re-exported here so a region app can
//! name it from either side.

mod registry;

// The core owns this type; a region app names it from either side.
pub use registry::{RegionId, Registry};
pub use rustify_dom::Pace;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
