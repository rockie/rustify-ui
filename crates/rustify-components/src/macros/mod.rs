//! The two class-string macros, as received from Rust/UI.
//!
//! This commit imports them unchanged so that what the rewrite changes can be
//! read as a diff rather than described. The rewrite that follows removes the
//! nightly feature dependency and the router coupling.

pub mod clx;
pub mod utils;
pub mod variants;
