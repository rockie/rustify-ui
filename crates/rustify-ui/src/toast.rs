//! A short status message that goes away on its own.
//!
//! One at a time: a newer message replaces the one showing, rather than
//! queueing behind it, because a status that arrives late describes a state the
//! application has already left. Each message gets a number that only ever
//! rises, so a timer set for one of them can tell that it has been replaced
//! and must not hide its successor.
//!
//! The region it is drawn in sits in the scope's overlay plane but is not a
//! layer: it takes no focus, answers no Escape, and stays reachable while a
//! modal makes the rest of the scope inert.

/// How long a message stays up when the caller does not say: long enough to
/// read a sentence, short enough not to cover what it is about for long.
const DEFAULT_DURATION_MS: u32 = 3200;

/// What kind of news a message is. It changes how the message looks and
/// nothing else.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToastTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Error,
}

impl ToastTone {
    /// A stable name, for a `data-tone` attribute a stylesheet can select on.
    pub fn key(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// How one message is shown. The default is 3.2 seconds, neutral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastOptions {
    /// How long it stays up, in milliseconds.
    pub duration_ms: u32,
    pub tone: ToastTone,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            duration_ms: DEFAULT_DURATION_MS,
            tone: ToastTone::Neutral,
        }
    }
}

/// The message that is showing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Toast {
    /// Rises with every message shown in this scope.
    pub seq: u64,
    pub message: String,
    pub tone: ToastTone,
}

/// The rules, apart from the browser: numbering, replacement and expiry, with
/// time passed in.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, Default)]
struct Toasts {
    last: u64,
    showing: Option<(Toast, f64)>,
}

/// What a message's timer finds when it fires.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Expiry {
    /// The message was showing and its time was up; it is gone now.
    Hidden,
    /// The message had already been replaced or dismissed.
    Gone,
    /// The timer fired early - timers are coarse - and this many
    /// milliseconds are left.
    Early(f64),
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Toasts {
    /// Shows `message` from `now` on, in place of whatever was showing.
    fn show(&mut self, message: String, options: ToastOptions, now: f64) -> u64 {
        self.last += 1;
        let toast = Toast {
            seq: self.last,
            message,
            tone: options.tone,
        };
        self.showing = Some((toast, now + f64::from(options.duration_ms)));
        self.last
    }

    fn current(&self) -> Option<&Toast> {
        self.showing.as_ref().map(|(toast, _)| toast)
    }

    /// Hides message `seq` if it is still the one showing.
    fn dismiss(&mut self, seq: u64) -> bool {
        let showing = self.current().is_some_and(|toast| toast.seq == seq);
        if showing {
            self.showing = None;
        }
        showing
    }

    /// Hides message `seq` if it is still showing and its time is up at `now`.
    fn expire(&mut self, seq: u64, now: f64) -> Expiry {
        match &self.showing {
            Some((toast, until)) if toast.seq == seq => {
                if now < *until {
                    Expiry::Early(until - now)
                } else {
                    self.showing = None;
                    Expiry::Hidden
                }
            }
            _ => Expiry::Gone,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use dom::{provide_toasts, use_toasts, ToastHandle, ToastRegion};

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{Expiry, Toast, ToastOptions, Toasts};
    use crate::i18n::{use_locale, Message};
    use crate::overlay::use_overlay;
    use leptos::portal::Portal;
    use leptos::prelude::*;

    /// The scope's toasts. `Copy`, so it goes into every closure that needs it.
    #[derive(Clone, Copy)]
    pub struct ToastHandle {
        toasts: RwSignal<Toasts>,
    }

    /// Puts the scope's toasts in context and returns them.
    pub fn provide_toasts() -> ToastHandle {
        let handle = ToastHandle {
            toasts: RwSignal::new(Toasts::default()),
        };
        provide_context(handle);
        handle
    }

    /// The scope's toasts.
    ///
    /// # Panics
    ///
    /// If no scope provided them. That is a wiring mistake, and a component
    /// that quietly made its own would be showing messages no region draws.
    pub fn use_toasts() -> ToastHandle {
        use_context::<ToastHandle>()
            .expect("no toasts in scope; call provide_toasts in the scope root")
    }

    fn now() -> f64 {
        window()
            .performance()
            .map(|performance| performance.now())
            .unwrap_or_default()
    }

    impl ToastHandle {
        /// Shows `message` in place of whatever is showing, and returns its
        /// number.
        ///
        /// It hides itself after `options.duration_ms`, on a timer the runtime
        /// owns: an instance that fails first drops the timer rather than
        /// letting it run.
        pub fn show(&self, message: impl Into<String>, options: ToastOptions) -> u64 {
            let message = message.into();
            let Some(seq) = self
                .toasts
                .try_update(|toasts| toasts.show(message, options, now()))
            else {
                return 0;
            };
            self.hide_after(seq, f64::from(options.duration_ms));
            seq
        }

        /// Hides message `seq`, if it is still the one showing.
        pub fn dismiss(&self, seq: u64) {
            self.toasts
                .try_maybe_update(|toasts| (toasts.dismiss(seq), ()));
        }

        /// The message showing now, tracked.
        pub fn current(&self) -> Option<Toast> {
            self.toasts.with(|toasts| toasts.current().cloned())
        }

        fn hide_after(self, seq: u64, wait: f64) {
            rustify_makepad::defer_after(wait.ceil() as i32, move || {
                let expiry = self.toasts.try_maybe_update(|toasts| {
                    let expiry = toasts.expire(seq, now());
                    (expiry == Expiry::Hidden, expiry)
                });
                if let Some(Expiry::Early(left)) = expiry {
                    self.hide_after(seq, left);
                }
            });
        }
    }

    /// Where the scope's toasts are drawn: a polite live region in the
    /// scope's overlay plane.
    ///
    /// The region itself is always there, empty between messages, because a
    /// screen reader only announces changes to a live region it already knew
    /// about. It is unstyled; `class` is the caller's.
    #[component]
    pub fn ToastRegion(
        /// The markup for the message showing. Left out, the message and a
        /// button that dismisses it.
        #[prop(optional, into)]
        render: Option<Callback<Toast, AnyView>>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let toasts = use_toasts();
        let locale = use_locale();
        let dismiss_id = format!("{test_id}-dismiss");
        use_overlay().map(|stack| {
            let root = stack.overlay_root();
            view! {
                <Portal mount=root>
                    <div
                        class=class.clone()
                        data-testid=test_id.clone()
                        role="status"
                        aria-live="polite"
                    >
                        {
                            let dismiss_id = dismiss_id.clone();
                            move || {
                                toasts
                                    .current()
                                    .map(|toast| match render {
                                        Some(render) => render.run(toast),
                                        None => {
                                            let seq = toast.seq;
                                            view! {
                                                <div
                                                    class="rustify-toast"
                                                    data-tone=toast.tone.key()
                                                >
                                                    <span>{toast.message}</span>
                                                    <button
                                                        type="button"
                                                        data-testid=dismiss_id.clone()
                                                        aria-label=move || {
                                                            locale.text(Message::Dismiss)
                                                        }
                                                        on:click=move |_| toasts.dismiss(seq)
                                                    >
                                                        "\u{d7}"
                                                    </button>
                                                </div>
                                            }
                                                .into_any()
                                        }
                                    })
                            }
                        }
                    </div>
                </Portal>
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Expiry, Toast, ToastOptions, ToastTone, Toasts, DEFAULT_DURATION_MS};

    fn options(duration_ms: u32) -> ToastOptions {
        ToastOptions {
            duration_ms,
            ..ToastOptions::default()
        }
    }

    #[test]
    fn a_message_shows_with_the_default_options_until_its_time_is_up() {
        assert_eq!(ToastOptions::default().duration_ms, 3200);
        assert_eq!(ToastOptions::default().tone, ToastTone::Neutral);

        let mut toasts = Toasts::default();
        assert_eq!(toasts.current(), None);
        let seq = toasts.show("saved".into(), ToastOptions::default(), 1000.0);
        assert_eq!(
            toasts.current(),
            Some(&Toast {
                seq,
                message: "saved".into(),
                tone: ToastTone::Neutral,
            })
        );
        let due = 1000.0 + f64::from(DEFAULT_DURATION_MS);
        assert_eq!(toasts.expire(seq, due), Expiry::Hidden);
        assert_eq!(toasts.current(), None);
    }

    #[test]
    fn every_message_gets_a_higher_number_and_replaces_the_one_before() {
        let mut toasts = Toasts::default();
        let first = toasts.show("one".into(), options(3200), 0.0);
        let second = toasts.show(
            "two".into(),
            ToastOptions {
                duration_ms: 3200,
                tone: ToastTone::Error,
            },
            10.0,
        );
        assert!(second > first);
        let showing = toasts.current().expect("the second message");
        assert_eq!(showing.seq, second);
        assert_eq!(showing.message, "two");
        assert_eq!(showing.tone, ToastTone::Error);
        // Numbers keep rising after the region has emptied.
        toasts.dismiss(second);
        assert!(toasts.show("three".into(), options(3200), 20.0) > second);
    }

    #[test]
    fn a_replaced_message_timer_leaves_its_successor_alone() {
        let mut toasts = Toasts::default();
        let first = toasts.show("one".into(), options(1000), 0.0);
        let second = toasts.show("two".into(), options(1000), 900.0);
        // The first message's timer fires at 1000: the second has 900 to go.
        assert_eq!(toasts.expire(first, 1000.0), Expiry::Gone);
        assert_eq!(toasts.current().map(|toast| toast.seq), Some(second));
        assert_eq!(toasts.expire(second, 1900.0), Expiry::Hidden);
    }

    #[test]
    fn a_timer_that_fires_early_is_told_how_long_is_left() {
        let mut toasts = Toasts::default();
        let seq = toasts.show("saved".into(), options(3200), 100.0);
        assert_eq!(toasts.expire(seq, 3299.5), Expiry::Early(0.5));
        assert!(toasts.current().is_some());
        assert_eq!(toasts.expire(seq, 3300.0), Expiry::Hidden);
        // And a second timer for the same message finds nothing to do.
        assert_eq!(toasts.expire(seq, 3400.0), Expiry::Gone);
    }

    #[test]
    fn dismissing_closes_only_the_message_it_names() {
        let mut toasts = Toasts::default();
        let first = toasts.show("one".into(), options(3200), 0.0);
        let second = toasts.show("two".into(), options(3200), 1.0);
        assert!(!toasts.dismiss(first), "already replaced");
        assert_eq!(toasts.current().map(|toast| toast.seq), Some(second));
        assert!(toasts.dismiss(second));
        assert_eq!(toasts.current(), None);
        assert!(!toasts.dismiss(second), "already gone");
        assert_eq!(toasts.expire(second, 5000.0), Expiry::Gone);
    }

    #[test]
    fn every_tone_has_a_name_of_its_own() {
        let keys: std::collections::HashSet<&str> = [
            ToastTone::Neutral,
            ToastTone::Success,
            ToastTone::Warning,
            ToastTone::Error,
        ]
        .into_iter()
        .map(ToastTone::key)
        .collect();
        assert_eq!(keys.len(), 4);
    }
}
