// This fork only targets the browser: the native backends were not carried
// over, so the platform layer is the web backend and the shared core.
#[macro_use]
pub mod cx_shared;

pub mod shared_framebuf;

pub mod web;

pub use crate::os::web::*;
