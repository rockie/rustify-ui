//! Admission and ordering for the actions one mount scope accepts.
//!
//! Regions hand their actions here as they arrive, each with the [`Pace`] its
//! producer declared. A continuous action keeps the position it arrived at, so
//! that even after it has replaced an older one it still lands on the correct
//! side of the clicks around it.

use std::collections::VecDeque;

pub use rustify_makepad::Pace;

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

pub struct Scheduler<A> {
    discrete: VecDeque<(Seq, A)>,
    /// One slot per named stream, outside the queue on purpose: a stream of
    /// pointer moves must not be able to fill it and push a click into
    /// backpressure. A `Vec` because a region has a handful of streams at
    /// most and the order they are searched in does not matter - what matters
    /// is that each keeps its own latest.
    continuous: Vec<(&'static str, Seq, A)>,
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
            continuous: Vec::new(),
            next_seq: 0,
            capacity,
            batch,
        }
    }

    pub fn accept(&mut self, pace: Pace, action: A) -> Admission {
        match pace {
            Pace::Continuous(stream) => {
                let seq = self.take_seq();
                match self
                    .continuous
                    .iter_mut()
                    .find(|(name, _, _)| *name == stream)
                {
                    // A newer state takes the newer arrival position, so it
                    // still lands on the correct side of the clicks around it.
                    Some(slot) => *slot = (stream, seq, action),
                    None => self.continuous.push((stream, seq, action)),
                }
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
            let earliest = self
                .continuous
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, seq, _))| *seq)
                .map(|(index, (_, seq, _))| (index, *seq));
            let take_continuous = match (earliest, self.discrete.front()) {
                (Some((_, continuous)), Some((discrete, _))) => continuous < *discrete,
                (Some(_), None) => true,
                (None, _) => false,
            };
            let next = match (take_continuous, earliest) {
                (true, Some((index, _))) => {
                    let (_, seq, action) = self.continuous.remove(index);
                    Some((seq, action))
                }
                _ => self.discrete.pop_front(),
            };
            match next {
                Some((_, action)) => batch.push(action),
                None => break,
            }
        }
        batch
    }

    pub fn is_empty(&self) -> bool {
        self.discrete.is_empty() && self.continuous.is_empty()
    }

    pub fn pending(&self) -> usize {
        self.discrete.len() + self.continuous.len()
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
                scheduler.accept(Pace::Continuous("move"), value),
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
        scheduler.accept(Pace::Continuous("pointer"), "move");
        scheduler.accept(Pace::Discrete, "release");
        assert_eq!(scheduler.take_batch(), vec!["press", "move", "release"]);
    }

    #[test]
    fn two_streams_supersede_themselves_and_not_each_other() {
        let mut scheduler = scheduler();
        scheduler.accept(Pace::Continuous("hover"), "hover 1");
        scheduler.accept(Pace::Continuous("scroll"), "scroll 1");
        scheduler.accept(Pace::Continuous("hover"), "hover 2");
        // Two streams, so two things pending - not one that ate the other.
        assert_eq!(scheduler.pending(), 2);
        // And each arrives where its own latest arrived.
        assert_eq!(scheduler.take_batch(), vec!["scroll 1", "hover 2"]);
        assert!(scheduler.is_empty());
    }

    #[test]
    fn a_second_stream_still_cannot_push_a_click_into_backpressure() {
        let mut scheduler = scheduler();
        for _ in 0..4 {
            scheduler.accept(Pace::Discrete, "click");
        }
        // The queue is full of clicks. Streams live outside it, however many
        // of them there are, so reporting on three of them refuses nothing.
        for stream in ["hover", "scroll", "hit"] {
            assert!(matches!(
                scheduler.accept(Pace::Continuous(stream), "state"),
                Admission::Accepted(_)
            ));
        }
        assert_eq!(
            scheduler.accept(Pace::Discrete, "one click too many"),
            Admission::Backpressure
        );
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
