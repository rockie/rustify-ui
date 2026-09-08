//! Admission and ordering for the actions one mount scope accepts.
//!
//! Regions hand their actions here as they arrive. Discrete actions - a click,
//! a submit, a cancel - are queued in arrival order and never merged, because
//! dropping or reordering one changes what the application did. Continuous
//! actions - a pointer that keeps moving - carry no history worth replaying, so
//! only the latest one survives, and it keeps the position it arrived at so it
//! still lands on the correct side of the clicks around it.

use std::collections::VecDeque;

/// Arrival order of an accepted action within a scope. Monotonic, and only
/// spent on actions that were actually accepted.
pub type Seq = u64;

/// Engineering defaults, not budgets: the queue is deep enough that ordinary
/// input never reaches it, and a task hands back control often enough that a
/// flood cannot hold the frame.
pub const QUEUE_CAPACITY: usize = 1024;
pub const BATCH: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admission {
    Accepted(Seq),
    /// The queue is full. The action was not accepted, has no sequence number
    /// and must be reported to the user as not executed rather than dropped.
    Backpressure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pace {
    /// Queued in order and never merged.
    Discrete,
    /// Only the most recent one is worth delivering.
    Continuous,
}

pub struct Scheduler<A> {
    discrete: VecDeque<(Seq, A)>,
    /// Outside the queue on purpose: a stream of pointer moves must not be
    /// able to fill it and push a click into backpressure.
    continuous: Option<(Seq, A)>,
    next_seq: Seq,
    capacity: usize,
    batch: usize,
}

impl<A> Default for Scheduler<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Scheduler<A> {
    pub fn new() -> Self {
        Self::with_limits(QUEUE_CAPACITY, BATCH)
    }

    pub fn with_limits(capacity: usize, batch: usize) -> Self {
        Self {
            discrete: VecDeque::new(),
            continuous: None,
            next_seq: 0,
            capacity,
            batch,
        }
    }

    pub fn accept(&mut self, pace: Pace, action: A) -> Admission {
        match pace {
            Pace::Continuous => {
                let seq = self.take_seq();
                self.continuous = Some((seq, action));
                Admission::Accepted(seq)
            }
            Pace::Discrete if self.discrete.len() < self.capacity => {
                let seq = self.take_seq();
                self.discrete.push_back((seq, action));
                Admission::Accepted(seq)
            }
            Pace::Discrete => Admission::Backpressure,
        }
    }

    /// Takes the next actions in arrival order, at most `batch` of them. The
    /// caller yields between batches; anything still pending stays queued in
    /// order.
    pub fn take_batch(&mut self) -> Vec<A> {
        let mut batch = Vec::new();
        while batch.len() < self.batch {
            let take_continuous = match (self.continuous.as_ref(), self.discrete.front()) {
                (Some((continuous, _)), Some((discrete, _))) => continuous < discrete,
                (Some(_), None) => true,
                (None, _) => false,
            };
            let next = if take_continuous {
                self.continuous.take()
            } else {
                self.discrete.pop_front()
            };
            match next {
                Some((_, action)) => batch.push(action),
                None => break,
            }
        }
        batch
    }

    pub fn is_empty(&self) -> bool {
        self.discrete.is_empty() && self.continuous.is_none()
    }

    pub fn pending(&self) -> usize {
        self.discrete.len() + usize::from(self.continuous.is_some())
    }

    fn take_seq(&mut self) -> Seq {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheduler() -> Scheduler<&'static str> {
        Scheduler::with_limits(4, 3)
    }

    #[test]
    fn discrete_actions_keep_arrival_order_and_consecutive_sequence_numbers() {
        let mut scheduler = scheduler();
        assert_eq!(
            scheduler.accept(Pace::Discrete, "a"),
            Admission::Accepted(0)
        );
        assert_eq!(
            scheduler.accept(Pace::Discrete, "b"),
            Admission::Accepted(1)
        );
        assert_eq!(scheduler.take_batch(), vec!["a", "b"]);
        assert!(scheduler.is_empty());
    }

    #[test]
    fn a_full_queue_reports_backpressure_and_spends_no_sequence_number() {
        let mut scheduler = scheduler();
        for _ in 0..4 {
            assert!(matches!(
                scheduler.accept(Pace::Discrete, "a"),
                Admission::Accepted(_)
            ));
        }
        assert_eq!(
            scheduler.accept(Pace::Discrete, "rejected"),
            Admission::Backpressure
        );
        assert_eq!(
            scheduler.accept(Pace::Discrete, "still rejected"),
            Admission::Backpressure
        );
        assert_eq!(scheduler.pending(), 4);
        scheduler.take_batch();
        // The rejected actions never entered, so the next accepted one
        // continues from the last sequence number actually handed out.
        assert_eq!(
            scheduler.accept(Pace::Discrete, "next"),
            Admission::Accepted(4)
        );
    }

    #[test]
    fn only_the_latest_continuous_action_survives_and_it_uses_no_queue_capacity() {
        let mut scheduler = scheduler();
        for _ in 0..4 {
            scheduler.accept(Pace::Discrete, "click");
        }
        for value in ["move 1", "move 2", "move 3"] {
            assert!(matches!(
                scheduler.accept(Pace::Continuous, value),
                Admission::Accepted(_)
            ));
        }
        assert_eq!(scheduler.pending(), 5);
        let drained: Vec<&str> = std::iter::from_fn(|| {
            let batch = scheduler.take_batch();
            (!batch.is_empty()).then_some(batch)
        })
        .flatten()
        .collect();
        assert_eq!(drained, vec!["click", "click", "click", "click", "move 3"]);
    }

    #[test]
    fn a_continuous_action_keeps_its_place_among_the_discrete_ones() {
        let mut scheduler = scheduler();
        scheduler.accept(Pace::Discrete, "press");
        scheduler.accept(Pace::Continuous, "move");
        scheduler.accept(Pace::Discrete, "release");
        assert_eq!(scheduler.take_batch(), vec!["press", "move", "release"]);
    }

    #[test]
    fn a_batch_is_bounded_and_the_rest_stays_in_order() {
        let mut scheduler = scheduler();
        for value in ["a", "b", "c", "d"] {
            scheduler.accept(Pace::Discrete, value);
        }
        assert_eq!(scheduler.take_batch(), vec!["a", "b", "c"]);
        assert_eq!(scheduler.take_batch(), vec!["d"]);
        assert_eq!(scheduler.take_batch(), Vec::<&str>::new());
    }
}
