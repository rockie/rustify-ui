//! DOM components for applications built on the Rustify UI SDK.
//!
//! The class strings and the two macros that build them come from Rust/UI; the
//! provenance of every imported file, and whether it is still verbatim, is in
//! the `rust_ui` section of `sources.lock.json`. What the fork changes is the
//! part a browser can observe: no inline script or style, a value that only
//! ever comes from the application, and a name, role, value and state for
//! everything a person can reach.

pub mod macros;

/// `clx.rs` refers to `crate::utils`, which is where that module sat in the
/// crate it came from. This alias is what lets the imported file stay
/// byte-identical to what was received; the rewrite removes both.
pub use macros::utils;
