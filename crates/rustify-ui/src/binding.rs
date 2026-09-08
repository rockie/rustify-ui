//! Delivery of region actions to the application, ordered per mount scope.
//!
//! Regions collect actions while Makepad is dispatching an event and hand them
//! here at the next safe point, so an application callback never runs while a
//! region's `Cx` is borrowed. One sink per scope gives every action in that
//! scope a single acceptance order, whichever region produced it.

use crate::scheduler::{Admission, Pace, Scheduler};
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// The first key that appears twice, if any.
///
/// A component keyed by business identity cannot be bound to two different
/// values for the same key: whichever one wins, the other silently disappears
/// and the user has no way to tell. Callers report the key and refuse the
/// binding rather than guess.
pub fn duplicate_key<K: Eq + Hash + Clone>(keys: impl IntoIterator<Item = K>) -> Option<K> {
    let mut seen = HashSet::new();
    keys.into_iter().find(|key| !seen.insert(key.clone()))
}

/// One component's right to have its actions delivered.
///
/// A delivery already queued holds the component's callback directly, so
/// dropping the component is not enough to stop it: the binding is what the
/// queued item checks when its turn finally comes.
#[derive(Clone)]
pub struct Binding(Arc<AtomicBool>);

impl Binding {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }

    /// Ends the binding. Every delivery still queued for it is skipped.
    pub fn close(&self) {
        self.0.store(false, Ordering::Relaxed);
    }

    pub fn is_open(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    /// Wraps `deliver` so it runs only while the binding is still open.
    pub fn guard(&self, deliver: impl FnOnce() + Send + 'static) -> impl FnOnce() + Send + 'static {
        let binding = self.clone();
        move || {
            if binding.is_open() {
                deliver();
            }
        }
    }
}

impl Default for Binding {
    fn default() -> Self {
        Self::new()
    }
}

/// A typed action already bound to the callback that will receive it. The
/// scope orders deliveries; it never inspects what they carry.
type Delivery = Box<dyn FnOnce() + Send>;

#[derive(Default)]
struct Inner {
    scheduler: Scheduler<Delivery>,
    draining: bool,
    armed: bool,
}

/// Handle to one scope's action order. Cloning shares the queue.
///
/// `Arc<Mutex<_>>` rather than `Rc<RefCell<_>>` because Leptos context values
/// must be `Send + Sync`; the sink is only ever touched from the browser's
/// single thread.
#[derive(Clone, Default)]
pub struct ActionSink(Arc<Mutex<Inner>>);

impl ActionSink {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues one action. The returned admission is the caller's answer to the
    /// user: an accepted action carries its place in the scope's order, and a
    /// refused one was never given a place and did not run.
    pub fn submit(&self, pace: Pace, deliver: impl FnOnce() + Send + 'static) -> Admission {
        self.lock().scheduler.accept(pace, Box::new(deliver))
    }

    /// Claims the right to drive the drain. Returns false when a drain is
    /// already running or already arranged, in which case that one will pick
    /// up whatever was just submitted.
    pub fn arm(&self) -> bool {
        let mut inner = self.lock();
        if inner.armed {
            return false;
        }
        inner.armed = true;
        true
    }

    /// Runs at most one batch of the pending actions and reports whether more
    /// are waiting, in which case the caller owes the browser a turn before
    /// calling again. Actions submitted by a callback join the queue instead
    /// of running inside this drain, so a callback can freely write signals
    /// that feed the regions it is being called from.
    pub fn drain_once(&self) -> bool {
        let batch = {
            let mut inner = self.lock();
            if inner.draining {
                return false;
            }
            inner.draining = true;
            inner.scheduler.take_batch()
        };
        for deliver in batch {
            deliver();
        }
        let mut inner = self.lock();
        inner.draining = false;
        inner.armed = !inner.scheduler.is_empty();
        inner.armed
    }

    pub fn pending(&self) -> usize {
        self.lock().scheduler.pending()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.0
            .lock()
            .expect("action sink is never held across a panic")
    }
}

/// Queues one pump's worth of a component's actions and returns how many the
/// scope refused.
///
/// A refused action was never given a place in the order and never runs, so
/// the count is the caller's answer to the user: that many actions did not
/// happen and can be tried again.
pub fn submit_all<A: Send + 'static>(
    sink: &ActionSink,
    binding: &Binding,
    actions: Vec<A>,
    on_action: impl Fn(A) + Clone + Send + 'static,
) -> usize {
    let mut refused = 0;
    for action in actions {
        let on_action = on_action.clone();
        let deliver = binding.guard(move || on_action(action));
        if matches!(
            sink.submit(Pace::Discrete, deliver),
            Admission::Backpressure
        ) {
            refused += 1;
        }
    }
    refused
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_keys_have_no_duplicate() {
        assert_eq!(duplicate_key([1, 2, 3]), None);
        assert_eq!(duplicate_key(Vec::<u32>::new()), None);
    }

    #[test]
    fn the_second_appearance_of_a_key_is_reported() {
        assert_eq!(duplicate_key([1, 2, 1, 3, 2]), Some(1));
        assert_eq!(duplicate_key(["a", "b", "b"]), Some("b"));
    }

    type Log = Arc<Mutex<Vec<u32>>>;

    fn record(log: &Log, value: u32) -> impl FnOnce() + Send {
        let log = log.clone();
        move || log.lock().unwrap().push(value)
    }

    fn read(log: &Log) -> Vec<u32> {
        log.lock().unwrap().clone()
    }

    #[test]
    fn actions_reach_the_application_in_acceptance_order() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        for value in 0..3 {
            sink.submit(Pace::Discrete, record(&log, value));
        }
        assert!(!sink.drain_once());
        assert_eq!(read(&log), vec![0, 1, 2]);
    }

    #[test]
    fn more_than_one_batch_needs_more_than_one_drain() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        for value in 0..100 {
            sink.submit(Pace::Discrete, record(&log, value));
        }
        assert!(sink.drain_once());
        assert_eq!(read(&log).len(), 64);
        assert!(!sink.drain_once());
        assert_eq!(read(&log), (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn an_action_submitted_by_a_callback_waits_for_the_next_drain() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        let nested = {
            let sink = sink.clone();
            let log = log.clone();
            move || {
                log.lock().unwrap().push(0);
                sink.submit(Pace::Discrete, record(&log, 1));
            }
        };
        sink.submit(Pace::Discrete, nested);
        assert!(sink.drain_once());
        assert_eq!(read(&log), vec![0]);
        assert!(!sink.drain_once());
        assert_eq!(read(&log), vec![0, 1]);
    }

    fn append(log: &Log) -> impl Fn(u32) + Clone + Send {
        let log = log.clone();
        move |value| log.lock().unwrap().push(value)
    }

    #[test]
    fn actions_queued_before_a_binding_closed_do_not_reach_the_application_after() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        let binding = Binding::new();
        assert_eq!(
            submit_all(&sink, &binding, (0..100).collect(), append(&log)),
            0
        );
        assert!(sink.drain_once());
        assert_eq!(read(&log).len(), crate::scheduler::BATCH);
        // The component goes away between two batches. Every queued item holds
        // its callback directly, so only the binding can stop them.
        binding.close();
        assert!(!sink.drain_once());
        assert_eq!(
            read(&log),
            (0..crate::scheduler::BATCH as u32).collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_actions_a_full_scope_refuses_are_counted_for_the_application() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        let binding = Binding::new();
        let over = 76;
        let refused = submit_all(
            &sink,
            &binding,
            (0..crate::scheduler::QUEUE_CAPACITY as u32 + over).collect(),
            append(&log),
        );
        assert_eq!(refused as u32, over);
        while sink.drain_once() {}
        assert_eq!(read(&log).len(), crate::scheduler::QUEUE_CAPACITY);
    }

    #[test]
    fn a_refused_action_never_runs() {
        let sink = ActionSink::new();
        let log: Log = Log::default();
        for value in 0..crate::scheduler::QUEUE_CAPACITY as u32 {
            assert!(matches!(
                sink.submit(Pace::Discrete, record(&log, value)),
                Admission::Accepted(_)
            ));
        }
        assert_eq!(
            sink.submit(Pace::Discrete, record(&log, 9999)),
            Admission::Backpressure
        );
        while sink.drain_once() {}
        assert_eq!(read(&log).len(), crate::scheduler::QUEUE_CAPACITY);
        assert!(!read(&log).contains(&9999));
    }
}
