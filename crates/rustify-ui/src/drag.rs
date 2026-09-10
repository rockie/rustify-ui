//! Carrying something from one place to another, at most once.
//!
//! The hard part is not the pointer. It is that a target inside a GPU region
//! cannot answer immediately - the region answers on its next pump - so a
//! release can arrive while the answer to "would you take this?" is still in
//! flight, and answers can arrive about targets the pointer has already left.
//! Every rule here is about which answer still counts and how many times a
//! drop can happen, which is once.

/// What a target said, or has not said yet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub session: u64,
    /// Rises with every question this session asks. An answer carrying an
    /// older one is about a target the pointer has left.
    pub seq: u64,
    pub target: String,
}

/// What releasing turned into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Nothing takes it. The drag is over and nothing happened.
    Nothing,
    /// The named target takes it, once.
    Drop { target: String, payload: String },
    /// A target was asked and has not answered. The answer, when it comes,
    /// decides - see [`Drags::answer`].
    Waiting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Dragging,
    /// Released, and waiting on the answer to the last question.
    Releasing,
    /// Over, one way or the other. Nothing more is accepted.
    Done,
}

#[derive(Clone, Debug)]
struct Session {
    id: u64,
    state: State,
    source: String,
    payload: String,
    /// What the pointer is over now.
    over: Option<String>,
    /// The target that said yes, and the question it was answering.
    confirmed: Option<(String, u64)>,
    /// The question that has not been answered.
    pending: Option<(String, u64)>,
    next_seq: u64,
}

/// The one drag a scope can have in flight.
#[derive(Clone, Debug, Default)]
pub struct Drags {
    current: Option<Session>,
    next_id: u64,
}

impl Drags {
    pub fn new() -> Self {
        Self::default()
    }

    /// Begins a drag, and returns the number that identifies it.
    ///
    /// Starting one while another is in flight cancels the first: a pointer
    /// cannot be in two drags, and the alternative is two sessions racing to
    /// deliver one drop.
    pub fn start(&mut self, source: impl Into<String>, payload: impl Into<String>) -> u64 {
        self.next_id += 1;
        self.current = Some(Session {
            id: self.next_id,
            state: State::Dragging,
            source: source.into(),
            payload: payload.into(),
            over: None,
            confirmed: None,
            pending: None,
            next_seq: 0,
        });
        self.next_id
    }

    pub fn session(&self) -> Option<u64> {
        self.current.as_ref().map(|session| session.id)
    }

    pub fn source(&self) -> Option<&str> {
        self.current.as_ref().map(|session| session.source.as_str())
    }

    pub fn payload(&self) -> Option<&str> {
        self.current
            .as_ref()
            .map(|session| session.payload.as_str())
    }

    /// Whether a target has said it would take this.
    pub fn confirmed(&self) -> Option<&str> {
        self.current
            .as_ref()
            .and_then(|session| session.confirmed.as_ref())
            .map(|(target, _)| target.as_str())
    }

    pub fn waiting(&self) -> bool {
        self.current
            .as_ref()
            .is_some_and(|session| session.pending.is_some())
    }

    pub fn dragging(&self) -> bool {
        self.current
            .as_ref()
            .is_some_and(|session| session.state != State::Done)
    }

    /// The pointer moved. `target` is what is under it, or `None` for
    /// somewhere that takes nothing.
    ///
    /// Moving off a target withdraws its answer immediately: the thing under
    /// the pointer is what a drop would land on, and a stale yes is how a
    /// drop lands somewhere nobody pointed at. A new target is asked, and the
    /// question supersedes any that was outstanding.
    pub fn over(&mut self, target: Option<&str>) -> Option<Query> {
        let session = self.current.as_mut()?;
        if session.state != State::Dragging {
            return None;
        }
        if session.over.as_deref() == target {
            return None;
        }
        session.over = target.map(str::to_string);
        session.confirmed = None;
        session.pending = None;
        let target = target?;
        session.next_seq += 1;
        session.pending = Some((target.to_string(), session.next_seq));
        Some(Query {
            session: session.id,
            seq: session.next_seq,
            target: target.to_string(),
        })
    }

    /// A target answered.
    ///
    /// Dropped unless it is this session's answer to the question still
    /// outstanding. The return value is what a release that was waiting has
    /// become, and `None` when none was.
    pub fn answer(&mut self, session_id: u64, seq: u64, accept: bool) -> Option<Outcome> {
        let session = self.current.as_mut()?;
        if session.id != session_id || session.state == State::Done {
            return None;
        }
        let (target, pending_seq) = session.pending.clone()?;
        if pending_seq != seq {
            return None;
        }
        session.pending = None;
        session.confirmed = accept.then_some((target, seq));
        if session.state != State::Releasing {
            return None;
        }
        Some(self.finish())
    }

    /// The pointer went up.
    pub fn release(&mut self) -> Outcome {
        let Some(session) = self.current.as_mut() else {
            return Outcome::Nothing;
        };
        match session.state {
            State::Done => Outcome::Nothing,
            State::Releasing => Outcome::Waiting,
            State::Dragging => {
                if session.pending.is_some() {
                    // A question is outstanding. Its answer decides, and no
                    // earlier one can: dropping now would be dropping on a
                    // target that has not agreed.
                    session.state = State::Releasing;
                    Outcome::Waiting
                } else {
                    self.finish()
                }
            }
        }
    }

    /// Escape, a lost pointer, a window that went away. Nothing is delivered.
    pub fn cancel(&mut self) {
        if let Some(session) = self.current.as_mut() {
            session.state = State::Done;
            session.confirmed = None;
            session.pending = None;
        }
    }

    /// The drop, or not, exactly once.
    fn finish(&mut self) -> Outcome {
        let Some(session) = self.current.as_mut() else {
            return Outcome::Nothing;
        };
        session.state = State::Done;
        session.pending = None;
        match session.confirmed.take() {
            Some((target, _)) => Outcome::Drop {
                target,
                payload: session.payload.clone(),
            },
            None => Outcome::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Drags, Outcome};

    fn dropped_on(outcome: &Outcome) -> Option<&str> {
        match outcome {
            Outcome::Drop { target, .. } => Some(target),
            _ => None,
        }
    }

    #[test]
    fn a_drop_on_a_target_that_agreed_happens_once() {
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");
        assert_eq!(drags.answer(session, query.seq, true), None);
        assert_eq!(drags.confirmed(), Some("group-a"));

        let outcome = drags.release();
        assert_eq!(dropped_on(&outcome), Some("group-a"));
        // And not again: the session is over.
        assert_eq!(drags.release(), Outcome::Nothing);
        assert!(!drags.dragging());
    }

    #[test]
    fn accepted_then_released_over_a_target_that_refuses_delivers_nothing() {
        // §5.4's first required sequence.
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let first = drags.over(Some("group-a")).expect("asked");
        drags.answer(session, first.seq, true);

        let second = drags.over(Some("group-b")).expect("asked");
        assert_eq!(
            drags.confirmed(),
            None,
            "moving off withdraws the first yes"
        );
        drags.answer(session, second.seq, false);

        assert_eq!(drags.release(), Outcome::Nothing);
    }

    #[test]
    fn accepted_then_released_over_nothing_delivers_nothing() {
        // The second required sequence.
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");
        drags.answer(session, query.seq, true);

        assert_eq!(drags.over(None), None, "blank space is asked nothing");
        assert_eq!(drags.confirmed(), None);
        assert_eq!(drags.release(), Outcome::Nothing);
    }

    #[test]
    fn a_late_yes_about_a_target_the_pointer_left_is_dropped() {
        // The third. A region answering a question from two targets ago must
        // not put the drop back on a target nobody is pointing at.
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let first = drags.over(Some("group-a")).expect("asked");
        let second = drags.over(Some("group-b")).expect("asked");

        assert_eq!(drags.answer(session, first.seq, true), None);
        assert_eq!(drags.confirmed(), None, "the answer was about group-a");
        assert!(drags.waiting(), "group-b has still not answered");

        drags.answer(session, second.seq, false);
        assert_eq!(drags.release(), Outcome::Nothing);
    }

    #[test]
    fn releasing_with_a_question_outstanding_waits_for_its_answer() {
        // The fourth, both ways.
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");

        assert_eq!(drags.release(), Outcome::Waiting);
        assert_eq!(
            drags.release(),
            Outcome::Waiting,
            "asking again changes nothing"
        );

        let outcome = drags.answer(session, query.seq, true).expect("decided");
        assert_eq!(dropped_on(&outcome), Some("group-a"));

        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");
        drags.release();
        assert_eq!(
            drags.answer(session, query.seq, false),
            Some(Outcome::Nothing)
        );
    }

    #[test]
    fn an_answer_from_another_session_is_not_this_session_s() {
        let mut drags = Drags::new();
        let first = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");
        // The drag was restarted - a second pointer, a re-grab - and the old
        // region answers the old question.
        let second = drags.start("object-2", "2");
        assert_ne!(first, second);
        assert_eq!(drags.answer(first, query.seq, true), None);
        assert_eq!(drags.confirmed(), None);
    }

    #[test]
    fn a_cancelled_drag_delivers_nothing_and_ignores_what_arrives_after() {
        let mut drags = Drags::new();
        let session = drags.start("object-1", "1");
        let query = drags.over(Some("group-a")).expect("asked");
        drags.answer(session, query.seq, true);

        drags.cancel();
        assert!(!drags.dragging());
        assert_eq!(drags.release(), Outcome::Nothing);
        assert_eq!(drags.answer(session, query.seq, true), None);
        assert_eq!(drags.over(Some("group-b")), None);
    }

    #[test]
    fn staying_over_one_target_asks_it_once() {
        // A pointer moving inside a target is not a new question; asking on
        // every move would be a region answering thousands of times a second.
        let mut drags = Drags::new();
        drags.start("object-1", "1");
        assert!(drags.over(Some("group-a")).is_some());
        assert_eq!(drags.over(Some("group-a")), None);
        assert_eq!(drags.over(Some("group-a")), None);
    }

    #[test]
    fn a_hundred_drags_deliver_exactly_a_hundred_drops() {
        let mut drags = Drags::new();
        let mut drops = 0;
        for round in 0..100 {
            let session = drags.start(format!("object-{round}"), round.to_string());
            let query = drags.over(Some("group-a")).expect("asked");
            drags.answer(session, query.seq, true);
            if dropped_on(&drags.release()).is_some() {
                drops += 1;
            }
        }
        assert_eq!(drops, 100);
    }

    #[test]
    fn a_release_with_no_drag_at_all_is_not_a_drop() {
        let mut drags = Drags::new();
        assert_eq!(drags.release(), Outcome::Nothing);
        assert_eq!(drags.answer(1, 1, true), None);
        drags.cancel();
    }
}
