//! Registering a listener on something the page owns.
//!
//! `window`, `document` and a scope's container all outlive the scope, and a
//! `Drop` is what normally takes a listener back off them. After a trap there
//! is no `Drop`: `panic = "abort"` runs no destructors, so every listener the
//! dead instance left behind is still there, still holding a closure into a
//! module nothing can trust, and still being called.
//!
//! So each of them is registered with the instance's own abort signal as well.
//! Dropping is still the normal path and still what happens on an ordinary
//! unmount; aborting is the one removal that works when no Rust can run.

#[cfg(target_arch = "wasm32")]
use leptos::web_sys::AddEventListenerOptions;

/// How this instance's page-level listeners are registered.
///
/// Bare options when there is no host yet - a scope mounted before the runtime
/// booted has nothing to be aborted by, and is no worse off than it was.
/// Callers that also want capture or passive set those on what they get back.
#[cfg(target_arch = "wasm32")]
pub(crate) fn page_level() -> AddEventListenerOptions {
    rustify_makepad::listener_options().unwrap_or_else(AddEventListenerOptions::new)
}
