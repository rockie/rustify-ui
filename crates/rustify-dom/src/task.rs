//! Which asynchronous result is still allowed to take effect.
//!
//! A request that cannot be cancelled still runs; what it must not do is
//! arrive late and undo a newer answer, or reach a view that has gone. A
//! ticket is a request's right to deliver: it is issued in order, it goes
//! stale the moment a newer one is issued, and it ends with the holder.

use crate::diagnostics::UiError;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// The four states an asynchronous value is shown in.
///
/// `Empty` is a real answer - the request succeeded and there is nothing -
/// which is why it is not spelled `Ready(None)`: a view that cannot tell the
/// two apart shows a spinner forever.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Load<T> {
    #[default]
    Loading,
    Empty,
    Ready(T),
    Error(UiError),
}

impl<T> Load<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn ready(&self) -> Option<&T> {
        match self {
            Self::Ready(value) => Some(value),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<UiError> {
        match self {
            Self::Error(error) => Some(*error),
            _ => None,
        }
    }

    /// The name a view can show without knowing what `T` is.
    pub fn state(&self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::Ready(_) => "ready",
            Self::Error(_) => "error",
        }
    }
}

#[derive(Default)]
struct Inner {
    next: AtomicU64,
    latest: AtomicU64,
    open: AtomicBool,
}

/// One field's requests. Cloning shares them.
#[derive(Clone)]
pub struct Requests(Arc<Inner>);

impl Default for Requests {
    fn default() -> Self {
        Self::new()
    }
}

impl Requests {
    pub fn new() -> Self {
        Self(Arc::new(Inner {
            next: AtomicU64::new(1),
            latest: AtomicU64::new(0),
            open: AtomicBool::new(true),
        }))
    }

    /// Starts a request. Every ticket issued before this one is now stale,
    /// whether or not it has answered.
    pub fn issue(&self) -> Ticket {
        let seq = self.0.next.fetch_add(1, Ordering::Relaxed);
        self.0.latest.store(seq, Ordering::Relaxed);
        Ticket {
            seq,
            owner: self.0.clone(),
        }
    }

    /// Ends the current request without starting one. What a cancel button
    /// does: the ticket that was current stops being current, and the
    /// sequence it lost to belongs to nothing, so nothing can deliver.
    pub fn cancel(&self) {
        drop(self.issue());
    }

    /// Ends every ticket. The view has gone; nothing may deliver into it.
    pub fn close(&self) {
        self.0.open.store(false, Ordering::Relaxed);
    }

    pub fn is_open(&self) -> bool {
        self.0.open.load(Ordering::Relaxed)
    }
}

/// One request's right to deliver its result.
pub struct Ticket {
    seq: u64,
    owner: Arc<Inner>,
}

impl Ticket {
    /// Whether this request may still take effect: it is the newest one, and
    /// its holder is still there.
    pub fn is_current(&self) -> bool {
        self.owner.open.load(Ordering::Relaxed)
            && self.owner.latest.load(Ordering::Relaxed) == self.seq
    }

    /// Applies `value` only if this request may still take effect, and reports
    /// whether it did. A refused delivery is not an error: it is an answer to
    /// a question nobody is asking any more.
    pub fn deliver<T>(&self, value: T, apply: impl FnOnce(T)) -> bool {
        if !self.is_current() {
            return false;
        }
        apply(value);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn the_four_states_name_themselves() {
        assert_eq!(Load::<u32>::Loading.state(), "loading");
        assert_eq!(Load::<u32>::Empty.state(), "empty");
        assert_eq!(Load::Ready(1u32).state(), "ready");
        assert_eq!(Load::<u32>::Error(UiError::Timeout).state(), "error");
        assert_eq!(Load::Ready(7u32).ready(), Some(&7));
        assert_eq!(
            Load::<u32>::Error(UiError::NotFound).error(),
            Some(UiError::NotFound)
        );
    }

    #[test]
    fn a_newer_request_makes_an_older_one_stale() {
        let requests = Requests::new();
        let first = requests.issue();
        assert!(first.is_current());
        let second = requests.issue();
        assert!(!first.is_current());
        assert!(second.is_current());
    }

    #[test]
    fn a_hundred_pairs_answered_backwards_all_end_on_the_newer_answer() {
        let requests = Requests::new();
        let value: Mutex<Option<&str>> = Mutex::new(None);
        for _ in 0..100 {
            let a = requests.issue();
            let b = requests.issue();
            // B answers first, then A - the order a network gives, not the
            // order they were asked in.
            assert!(b.deliver("b", |v| *value.lock().unwrap() = Some(v)));
            assert!(!a.deliver("a", |v| *value.lock().unwrap() = Some(v)));
            assert_eq!(*value.lock().unwrap(), Some("b"));
        }
    }

    #[test]
    fn nothing_delivers_into_a_view_that_has_gone() {
        let requests = Requests::new();
        let delivered = Mutex::new(0u32);
        let tickets: Vec<Ticket> = (0..100).map(|_| requests.issue()).collect();
        requests.close();
        for ticket in &tickets {
            ticket.deliver((), |()| *delivered.lock().unwrap() += 1);
        }
        assert_eq!(*delivered.lock().unwrap(), 0);
        assert!(!requests.is_open());
    }

    #[test]
    fn a_cancel_leaves_nothing_that_can_deliver() {
        let requests = Requests::new();
        let ticket = requests.issue();
        requests.cancel();
        assert!(!ticket.is_current());
        let value = Mutex::new("kept");
        assert!(!ticket.deliver("clobbered", |v| *value.lock().unwrap() = v));
        assert_eq!(*value.lock().unwrap(), "kept");
        // And the next request is a request like any other.
        assert!(requests.issue().is_current());
    }

    #[test]
    fn a_refused_delivery_leaves_the_value_alone() {
        let requests = Requests::new();
        let value = Mutex::new("kept");
        let stale = requests.issue();
        let _fresh = requests.issue();
        assert!(!stale.deliver("clobbered", |v| *value.lock().unwrap() = v));
        assert_eq!(*value.lock().unwrap(), "kept");
    }
}
