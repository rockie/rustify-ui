//! How much of a producer's stream is worth keeping.
//!
//! A click, a submit or a cancel is its own event: dropping or reordering one
//! changes what the application did, so they are queued in arrival order and
//! never merged. A pointer that keeps moving is a stream of states rather than
//! a history, so only the latest one is worth delivering - and it must not be
//! able to fill the queue and push a click out of it.

/// How the scope admits one action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pace {
    /// Queued in order and never merged.
    Discrete,
    /// Only the most recent one is worth delivering - of *this* stream.
    ///
    /// The name is what makes that true. One region can have several streams
    /// running at once: where the pointer is, what the drag would land on, how
    /// far a list has scrolled. They supersede themselves and nothing else,
    /// because a newer answer to one question is not an answer to another -
    /// and a single slot for the whole scope means whichever stream reported
    /// last silently ate the others.
    Continuous(&'static str),
}
