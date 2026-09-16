//! Bringing files in, and sending bytes back out.
//!
//! Importing has three answers and one non-answer, and the difference matters
//! to the application: a file that was too large, a file of the wrong kind, a
//! file that loaded, and a person who closed the picker without choosing. Only
//! the third writes anything. The other three leave the application exactly as
//! it was, which is what makes "cancel" cost nothing.
//!
//! The refusals are decided from the name and the size alone, before a byte is
//! read. A 2 GB file that will be refused for its size should be refused
//! immediately, not after the browser has read it into memory.

/// What an import will accept.
///
/// The list of kinds is the same one the file picker shows, so that the
/// picker's filter and this check cannot drift apart: a component builds its
/// `accept` attribute from [`Limits::accept_attribute`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// The largest file this import will read, in bytes.
    pub max_bytes: u64,
    /// The extensions this import will read, each written with its dot and in
    /// lower case (`".json"`). Empty means every extension.
    pub kinds: &'static [&'static str],
}

impl Limits {
    /// Whether this file may be read, decided without reading it.
    pub fn check(&self, name: &str, bytes: u64) -> Result<(), Refusal> {
        // Size first: it is the refusal that costs the most to discover late,
        // and a file can be both too large and of the wrong kind.
        if bytes > self.max_bytes {
            return Err(Refusal::TooLarge {
                bytes,
                limit: self.max_bytes,
            });
        }
        if self.kinds.is_empty() {
            return Ok(());
        }
        let name = name.to_ascii_lowercase();
        if self.kinds.iter().any(|kind| name.ends_with(kind)) {
            return Ok(());
        }
        Err(Refusal::WrongKind {
            allowed: self.kinds,
        })
    }

    /// The value for a file input's `accept` attribute, so the picker offers
    /// what this import would take. It is a courtesy, not the check: a person
    /// can defeat any picker filter, and [`check`](Self::check) still runs.
    pub fn accept_attribute(&self) -> String {
        self.kinds.join(",")
    }
}

/// Why a file was not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    TooLarge {
        bytes: u64,
        limit: u64,
    },
    WrongKind {
        allowed: &'static [&'static str],
    },
    /// The browser could not read a file it had already offered - removed from
    /// disk between the choice and the read, or unreadable.
    Unreadable,
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge { bytes, limit } => {
                write!(f, "{bytes} bytes, and the limit is {limit}")
            }
            Self::WrongKind { allowed: [] } => f.write_str("not a kind we read"),
            Self::WrongKind { allowed } => write!(f, "we read {}", allowed.join(", ")),
            Self::Unreadable => f.write_str("the file could not be read"),
        }
    }
}

/// What came of asking for a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Import {
    /// A file, and its bytes. The only variant that should change anything.
    Loaded { name: String, bytes: Vec<u8> },
    /// A file was chosen and will not be read. The application says why; it
    /// does not change.
    Refused { name: String, why: Refusal },
    /// Nobody chose anything: the picker was dismissed, or a drop carried no
    /// files. Not an error, and not worth reporting to the user - they know,
    /// they just did it.
    Aborted,
}

impl Import {
    /// The bytes, if there are any. Written so that the ordinary use reads as
    /// one line and the two other outcomes cannot be forgotten by accident.
    pub fn loaded(&self) -> Option<(&str, &[u8])> {
        match self {
            Self::Loaded { name, bytes } => Some((name.as_str(), bytes.as_slice())),
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use browser::{export, import_files, pick};

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::{Import, Limits, Refusal};
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::{JsCast, JsValue};
    use leptos::web_sys::{js_sys, Blob, BlobPropertyBag, File, FileList, HtmlAnchorElement, Url};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Reads what a picker or a drop handed over.
    ///
    /// One file, because every import in this SDK takes one: a list with two
    /// files in it reads the first, which is what the picker's own
    /// single-selection mode would have given.
    pub fn import_files(
        files: Option<FileList>,
        limits: Limits,
        done: impl FnOnce(Import) + 'static,
    ) {
        let Some(file) = files.and_then(|files| files.get(0)) else {
            done(Import::Aborted);
            return;
        };
        import_file(file, limits, done);
    }

    fn import_file(file: File, limits: Limits, done: impl FnOnce(Import) + 'static) {
        let name = file.name();
        // `File::size` is an f64 because JS has no integers; a negative or
        // fractional size is not a thing a browser produces, and saturating is
        // the reading that refuses rather than wraps.
        let size = file.size().max(0.0) as u64;
        if let Err(why) = limits.check(&name, size) {
            done(Import::Refused { name, why });
            return;
        }
        let promise = file.array_buffer();
        leptos::task::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(buffer) => {
                    let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
                    done(Import::Loaded { name, bytes })
                }
                Err(_) => done(Import::Refused {
                    name,
                    why: Refusal::Unreadable,
                }),
            }
        });
    }

    /// Opens the browser's file picker and reports what came back.
    ///
    /// The input is built, clicked and dropped here rather than rendered,
    /// because a picker opened from a command has no place to live in the
    /// view. Components that show a file field render their own input and call
    /// [`import_files`] from its `change` event instead.
    ///
    /// A picker that is dismissed fires no event in most browsers, so the
    /// `cancel` event is listened for as well; whichever arrives first
    /// answers, and the other is dropped. Without this, cancelling would
    /// simply never answer, and a control that had said "importing…" would say
    /// it forever.
    pub fn pick(limits: Limits, done: impl FnOnce(Import) + 'static) {
        let Some(document) = leptos::web_sys::window().and_then(|window| window.document()) else {
            done(Import::Aborted);
            return;
        };
        let Ok(input) = document.create_element("input") else {
            done(Import::Aborted);
            return;
        };
        let input: leptos::web_sys::HtmlInputElement = match input.dyn_into() {
            Ok(input) => input,
            Err(_) => {
                done(Import::Aborted);
                return;
            }
        };
        input.set_type("file");
        if !limits.kinds.is_empty() {
            let _ = input.set_attribute("accept", &limits.accept_attribute());
        }

        let done = Rc::new(RefCell::new(Some(done)));
        let listeners = Rc::new(RefCell::new(
            None::<(Closure<dyn FnMut()>, Closure<dyn FnMut()>)>,
        ));
        let cleanup = {
            let input = input.clone();
            let listeners = Rc::clone(&listeners);
            move || {
                let callbacks = listeners.borrow_mut().take();
                if let Some((chosen, dismissed)) = callbacks {
                    let _ = input.remove_event_listener_with_callback(
                        "change",
                        chosen.as_ref().unchecked_ref(),
                    );
                    let _ = input.remove_event_listener_with_callback(
                        "cancel",
                        dismissed.as_ref().unchecked_ref(),
                    );
                }
            }
        };

        let chosen = {
            let input = input.clone();
            let done = Rc::clone(&done);
            let cleanup = cleanup.clone();
            Closure::<dyn FnMut()>::new(move || {
                let answer = done.borrow_mut().take();
                cleanup();
                if let Some(answer) = answer {
                    import_files(input.files(), limits, answer);
                }
            })
        };
        let dismissed = {
            let done = Rc::clone(&done);
            let cleanup = cleanup.clone();
            Closure::<dyn FnMut()>::new(move || {
                let answer = done.borrow_mut().take();
                cleanup();
                if let Some(answer) = answer {
                    answer(Import::Aborted);
                }
            })
        };
        let options = crate::listeners::page_level();
        let registered = input
            .add_event_listener_with_callback_and_add_event_listener_options(
                "change",
                chosen.as_ref().unchecked_ref(),
                &options,
            )
            .and_then(|()| {
                input.add_event_listener_with_callback_and_add_event_listener_options(
                    "cancel",
                    dismissed.as_ref().unchecked_ref(),
                    &options,
                )
            });
        // Either event takes the callback and breaks this ownership cycle before
        // reading bytes. A queued second event can neither answer nor keep the input alive.
        *listeners.borrow_mut() = Some((chosen, dismissed));
        if registered.is_err() {
            cleanup();
            if let Some(answer) = done.borrow_mut().take() {
                answer(Import::Aborted);
            }
            return;
        }
        input.click();
    }

    /// Hands `bytes` to the browser as a download named `name`.
    ///
    /// The object URL is revoked on the next turn rather than immediately: a
    /// click on an anchor starts the download asynchronously, and revoking in
    /// the same turn cancels it in some browsers.
    pub fn export(name: &str, bytes: &[u8], mime: &str) -> Result<(), JsValue> {
        let array = js_sys::Uint8Array::from(bytes);
        let parts = js_sys::Array::of1(&array.buffer());
        let options = BlobPropertyBag::new();
        options.set_type(mime);
        let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &options)?;
        let url = Url::create_object_url_with_blob(&blob)?;

        let document = leptos::web_sys::window()
            .and_then(|window| window.document())
            .ok_or_else(|| JsValue::from_str("no document"))?;
        let anchor: HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
        anchor.set_href(&url);
        anchor.set_download(name);
        anchor.click();

        let revoke = Closure::once_into_js(move || {
            let _ = Url::revoke_object_url(&url);
        });
        if let Some(window) = leptos::web_sys::window() {
            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(revoke.unchecked_ref(), 0);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Import, Limits, Refusal};

    const JSON: Limits = Limits {
        max_bytes: 1 << 20,
        kinds: &[".json"],
    };

    #[test]
    fn a_file_that_fits_and_is_the_right_kind_is_read() {
        assert_eq!(JSON.check("objects.json", 4_096), Ok(()));
        // The name is the user's, so its case is not ours to depend on.
        assert_eq!(JSON.check("OBJECTS.JSON", 4_096), Ok(()));
    }

    #[test]
    fn size_is_decided_before_kind_so_a_huge_file_is_never_read() {
        // Wrong in both ways. The answer names the reason that would have cost
        // the most to discover after reading it.
        assert_eq!(
            JSON.check("objects.bin", 8 << 20),
            Err(Refusal::TooLarge {
                bytes: 8 << 20,
                limit: 1 << 20
            })
        );
        assert!(JSON.check("objects.json", (1 << 20) + 1).is_err());
        // Exactly at the limit is within it.
        assert_eq!(JSON.check("objects.json", 1 << 20), Ok(()));
    }

    #[test]
    fn a_kind_we_do_not_read_is_refused_with_the_kinds_we_do() {
        assert_eq!(
            JSON.check("objects.exe", 10),
            Err(Refusal::WrongKind {
                allowed: &[".json"]
            })
        );
        assert_eq!(
            Refusal::WrongKind {
                allowed: &[".json"]
            }
            .to_string(),
            "we read .json"
        );
        // A name that merely contains the extension is not that extension.
        assert!(JSON.check("json.exe", 10).is_err());
    }

    #[test]
    fn an_import_with_no_kinds_takes_any_name() {
        let anything = Limits {
            max_bytes: 16,
            kinds: &[],
        };
        assert_eq!(anything.check("whatever", 16), Ok(()));
        assert_eq!(anything.accept_attribute(), "");
    }

    #[test]
    fn the_picker_offers_exactly_what_the_check_would_take() {
        let limits = Limits {
            max_bytes: 1,
            kinds: &[".json", ".txt"],
        };
        assert_eq!(limits.accept_attribute(), ".json,.txt");
    }

    #[test]
    fn only_a_loaded_import_carries_bytes() {
        let loaded = Import::Loaded {
            name: "a.json".into(),
            bytes: vec![1, 2, 3],
        };
        assert_eq!(loaded.loaded(), Some(("a.json", [1u8, 2, 3].as_slice())));
        assert_eq!(Import::Aborted.loaded(), None);
        assert_eq!(
            Import::Refused {
                name: "a.exe".into(),
                why: Refusal::WrongKind {
                    allowed: &[".json"]
                },
            }
            .loaded(),
            None
        );
    }

    #[test]
    fn a_refusal_says_what_was_wrong_with_the_file() {
        assert_eq!(
            Refusal::TooLarge {
                bytes: 2_048,
                limit: 1_024
            }
            .to_string(),
            "2048 bytes, and the limit is 1024"
        );
        assert_eq!(
            Refusal::Unreadable.to_string(),
            "the file could not be read"
        );
    }
}
