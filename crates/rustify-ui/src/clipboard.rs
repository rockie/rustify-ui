//! Copying and pasting, and what to do when the browser says no.
//!
//! The clipboard is the one capability here that a person can refuse and a
//! browser can withhold, so every path through this module ends somewhere the
//! application can act on. A refusal is never reported as a success: a control
//! that says "copied" when nothing was copied is worse than one that says it
//! could not, because the user finds out later, somewhere else.

/// Why the clipboard did not answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipboardError {
    /// The page does not have permission - the user declined, the document is
    /// not focused, or the browser requires a gesture this was not part of.
    Denied,
    /// This build cannot reach the clipboard at all. See
    /// [`available`]: the browser's own API is behind a compile-time flag,
    /// and a build without it should offer the manual path from the start
    /// rather than at the moment somebody presses copy.
    Unavailable,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Denied => f.write_str("the browser refused the clipboard"),
            Self::Unavailable => f.write_str("this build cannot reach the clipboard"),
        }
    }
}

/// Whether this build can reach the browser's clipboard.
///
/// `web-sys` puts `navigator.clipboard` behind `--cfg=web_sys_unstable_apis`,
/// so it is a property of how the wasm was compiled rather than of the browser
/// it runs in. `cargo xtask build-web` sets the flag; a build made another way
/// may not have it, and this is how an application finds out without waiting
/// for a failure.
pub const fn available() -> bool {
    cfg!(all(target_arch = "wasm32", web_sys_unstable_apis))
}

#[cfg(all(target_arch = "wasm32", web_sys_unstable_apis))]
pub use browser::{copy, paste};

#[cfg(all(target_arch = "wasm32", web_sys_unstable_apis))]
mod browser {
    use super::ClipboardError;
    use leptos::web_sys::window;
    use wasm_bindgen_futures::JsFuture;

    /// Puts `text` on the clipboard, and says what happened.
    ///
    /// The answer arrives later, so the caller is told rather than returned
    /// to: a control that has to show "copied" or "could not" needs the
    /// answer, and it needs it in the same place either way.
    pub fn copy(text: &str, done: impl FnOnce(Result<(), ClipboardError>) + 'static) {
        let Some(clipboard) = window().map(|window| window.navigator().clipboard()) else {
            done(Err(ClipboardError::Unavailable));
            return;
        };
        let promise = clipboard.write_text(text);
        leptos::task::spawn_local(async move {
            match JsFuture::from(promise).await {
                Ok(_) => done(Ok(())),
                // Everything the browser rejects with is a refusal from here:
                // permission, focus, or a gesture this was not part of. What
                // the application does about it is the same in each case.
                Err(_) => done(Err(ClipboardError::Denied)),
            }
        });
    }

    /// Reads the clipboard.
    ///
    /// Only for a deliberate "paste" control on something that is not a text
    /// field. A text field's own paste is the browser's, needs no permission,
    /// and is the path a person expects.
    pub fn paste(done: impl FnOnce(Result<String, ClipboardError>) + 'static) {
        let Some(clipboard) = window().map(|window| window.navigator().clipboard()) else {
            done(Err(ClipboardError::Unavailable));
            return;
        };
        let promise = clipboard.read_text();
        leptos::task::spawn_local(async move {
            match JsFuture::from(promise)
                .await
                .ok()
                .and_then(|v| v.as_string())
            {
                Some(text) => done(Ok(text)),
                // A resolved promise that is not a string is as unusable as a
                // rejected one, and the application does the same thing about
                // it, so it arrives the same way.
                None => done(Err(ClipboardError::Denied)),
            }
        });
    }
}

/// The same two entry points for a build that cannot reach the clipboard, so
/// an application compiles either way and finds out through [`available`]
/// rather than through a missing function.
#[cfg(not(all(target_arch = "wasm32", web_sys_unstable_apis)))]
pub fn copy(_text: &str, done: impl FnOnce(Result<(), ClipboardError>) + 'static) {
    done(Err(ClipboardError::Unavailable));
}

#[cfg(not(all(target_arch = "wasm32", web_sys_unstable_apis)))]
pub fn paste(done: impl FnOnce(Result<String, ClipboardError>) + 'static) {
    done(Err(ClipboardError::Unavailable));
}

#[cfg(test)]
mod tests {
    use super::{available, copy, paste, ClipboardError};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The answer outlives the call - in the browser it arrives a turn later -
    /// so a caller keeps somewhere to put it rather than a borrow of a local.
    fn answer<T: 'static>() -> (Rc<RefCell<Option<T>>>, impl FnOnce(T) + 'static) {
        let cell: Rc<RefCell<Option<T>>> = Rc::default();
        let write = Rc::clone(&cell);
        (cell, move |value| *write.borrow_mut() = Some(value))
    }

    #[test]
    fn a_build_without_the_flag_says_so_rather_than_failing_later() {
        // On the host there is no browser at all, so this is the shape an
        // application sees when the capability is missing: an answer, at the
        // moment it asks, in the same form as a refusal.
        assert!(!available());
        let (copied, done) = answer();
        copy("anything", done);
        assert_eq!(*copied.borrow(), Some(Err(ClipboardError::Unavailable)));

        let (pasted, done) = answer();
        paste(done);
        assert_eq!(*pasted.borrow(), Some(Err(ClipboardError::Unavailable)));
    }

    #[test]
    fn every_failure_says_what_to_do_about_it() {
        assert_eq!(
            ClipboardError::Denied.to_string(),
            "the browser refused the clipboard"
        );
        assert_eq!(
            ClipboardError::Unavailable.to_string(),
            "this build cannot reach the clipboard"
        );
    }
}
