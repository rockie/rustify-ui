//! Scope utility compilation with an independent Worker, and the GPU half of it.

pub use rustify_dom::utility::*;

#[cfg(feature = "gpu")]
pub mod gpu;
