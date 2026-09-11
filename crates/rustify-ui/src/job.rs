//! Work too big for one frame, done a slice at a time.
//!
//! There is no thread to move it to. An ordinary deployment is not
//! cross-origin isolated, so there is no shared memory and no worker that
//! could look at the application's rows without being sent a copy of them, and
//! a hundred thousand rows is thirty-two megabytes. What there is instead is
//! the browser's own turn-taking: a job gives the turn back often enough that
//! a frame still fits between two slices.
//!
//! Three rules, and they are the whole of it.
//!
//! - **A slice is a length of time, not a number of rows.** A row count is an
//!   upper bound so that a step which turns out to be cheap cannot run away
//!   with the turn; the clock is what decides.
//! - **A job that ran while the data changed has nothing to say.** The version
//!   is read before a slice touches anything, so a stale job never reads a row
//!   that has moved under it, and it does not deliver. The application is told
//!   once, and decides whether to ask again.
//! - **Only the newest job may deliver.** That is the ticket's rule. A
//!   cancelled job is not interrupted: it stops before its next slice, which
//!   is why cancelling is instant to look at and never leaves half a result.

use crate::task::Ticket;

/// How long one slice may take, in milliseconds.
///
/// Long enough that the cost of handing the turn back is small beside it,
/// short enough that a frame still fits around it.
pub const SLICE_MS: f64 = 8.0;

/// The most rows one slice may do, whatever the clock says.
pub const SLICE_ROWS: usize = 16_384;

/// What one slice may spend.
///
/// The work asks `exhausted` every few hundred rows rather than every row:
/// reading a clock is not free, and the answer cannot change in between.
pub struct Budget<'a> {
    now: &'a dyn Fn() -> f64,
    deadline: f64,
    allowance: usize,
    spent: usize,
}

impl Budget<'_> {
    /// Records rows done. The job's progress is the sum of these, so work that
    /// does not report is work that does not show.
    pub fn did(&mut self, rows: usize) {
        self.spent += rows;
    }

    /// Whether this slice is over. Ask every few hundred rows: reading a clock
    /// is not free, and the answer cannot change in between.
    pub fn exhausted(&self) -> bool {
        self.spent >= self.allowance || (self.now)() >= self.deadline
    }

    /// Rows this slice has been told about.
    pub fn spent(&self) -> usize {
        self.spent
    }
}

/// What the work says when it hands the turn back.
pub enum Step<T> {
    /// There is more to do.
    More,
    /// This is the answer, and it is delivered whole.
    Done(T),
}

/// How a job ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ended<T> {
    Done(T),
    /// The data changed under it. Nothing was read after the change and
    /// nothing was delivered; whether to ask again is the application's.
    Stale,
    /// A newer job was started, the scope closed, or somebody cancelled.
    Cancelled,
}

impl<T> Ended<T> {
    pub fn done(self) -> Option<T> {
        match self {
            Self::Done(value) => Some(value),
            _ => None,
        }
    }

    /// The name a view can show without knowing what `T` is.
    pub fn state(&self) -> &'static str {
        match self {
            Self::Done(_) => "done",
            Self::Stale => "stale",
            Self::Cancelled => "cancelled",
        }
    }
}

type Work<T> = Box<dyn FnMut(&mut Budget<'_>) -> Step<T>>;

/// One piece of work, sliced.
///
/// Construct it, then call `slice` until it answers. Driving it is somebody
/// else's: in a browser that is `run`, which puts a browser turn between the
/// slices; in a test it is a loop, which is why the rules above can be checked
/// without one.
pub struct Job<T> {
    ticket: Ticket,
    version: Box<dyn Fn() -> u64>,
    clock: Box<dyn Fn() -> f64>,
    /// The version the job was started for.
    started_at: u64,
    work: Work<T>,
    total: usize,
    done: usize,
    slices: usize,
}

impl<T> Job<T> {
    /// `version` is the application's data version, read before every slice;
    /// `clock` is a monotonic millisecond reading; `total` is what the
    /// progress is out of.
    pub fn new(
        ticket: Ticket,
        version: impl Fn() -> u64 + 'static,
        clock: impl Fn() -> f64 + 'static,
        total: usize,
        work: impl FnMut(&mut Budget<'_>) -> Step<T> + 'static,
    ) -> Self {
        let started_at = version();
        Self {
            ticket,
            version: Box::new(version),
            clock: Box::new(clock),
            started_at,
            work: Box::new(work),
            total,
            done: 0,
            slices: 0,
        }
    }

    /// Does one slice. `None` means there is more to do.
    pub fn slice(&mut self) -> Option<Ended<T>> {
        if let Some(end) = self.refused() {
            return Some(end);
        }
        let deadline = (self.clock)() + SLICE_MS;
        let mut budget = Budget {
            now: &*self.clock,
            deadline,
            allowance: SLICE_ROWS,
            spent: 0,
        };
        let step = (self.work)(&mut budget);
        self.done = (self.done + budget.spent()).min(self.total);
        self.slices += 1;
        match step {
            Step::More => None,
            // Asked again at the end for the same reason it is asked at the
            // start: the work between the two could have been the write.
            Step::Done(value) => Some(self.refused().unwrap_or(Ended::Done(value))),
        }
    }

    /// Rows done, and rows there are.
    pub fn progress(&self) -> (usize, usize) {
        (self.done, self.total)
    }

    /// Slices taken so far. What a measurement of the slicing itself reads.
    pub fn slices(&self) -> usize {
        self.slices
    }

    fn refused(&self) -> Option<Ended<T>> {
        if !self.ticket.is_current() {
            return Some(Ended::Cancelled);
        }
        if (self.version)() != self.started_at {
            return Some(Ended::Stale);
        }
        None
    }
}

/// Runs a job to its end, giving the browser a turn between slices.
///
/// The turn is the host's task rather than a timer: chained timers are clamped
/// to four milliseconds each once they are nested, which on a job of a hundred
/// slices is most of the budget spent waiting.
#[cfg(target_arch = "wasm32")]
pub fn run<T: 'static>(mut job: Job<T>, on_end: impl FnOnce(Ended<T>) + 'static) {
    match job.slice() {
        Some(end) => on_end(end),
        None => rustify_makepad::defer(move || run(job, on_end)),
    }
}

/// The page's own clock, for a job started in a browser.
#[cfg(target_arch = "wasm32")]
pub fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Requests;
    use std::cell::Cell;
    use std::rc::Rc;

    /// A clock the test moves, and a version the test changes.
    #[derive(Clone, Default)]
    struct World {
        now: Rc<Cell<f64>>,
        version: Rc<Cell<u64>>,
    }

    impl World {
        fn clock(&self) -> impl Fn() -> f64 + 'static {
            let now = self.now.clone();
            move || now.get()
        }

        fn version(&self) -> impl Fn() -> u64 + 'static {
            let version = self.version.clone();
            move || version.get()
        }
    }

    /// Counts to `total` a row at a time, asking about the budget every
    /// `check` rows and charging `cost` milliseconds to each of those rows.
    fn counting(
        world: &World,
        total: usize,
        check: usize,
        cost: f64,
    ) -> impl FnMut(&mut Budget<'_>) -> Step<usize> + 'static {
        let now = world.now.clone();
        let mut at = 0usize;
        move |budget| loop {
            let rows = check.min(total - at);
            at += rows;
            now.set(now.get() + cost * rows as f64);
            budget.did(rows);
            if at >= total {
                return Step::Done(at);
            }
            if budget.exhausted() {
                return Step::More;
            }
        }
    }

    fn job(world: &World, total: usize, check: usize, cost: f64) -> Job<usize> {
        let requests = Requests::new();
        let ticket = requests.issue();
        // The requests handle is dropped: what keeps a ticket current is the
        // sequence, and nothing else here issues one.
        std::mem::forget(requests);
        Job::new(
            ticket,
            world.version(),
            world.clock(),
            total,
            counting(world, total, check, cost),
        )
    }

    fn run_to_end(job: &mut Job<usize>) -> Ended<usize> {
        for _ in 0..10_000 {
            if let Some(end) = job.slice() {
                return end;
            }
        }
        panic!("a job that never ends");
    }

    #[test]
    fn a_slice_stops_on_the_clock_rather_than_on_a_row_count() {
        let world = World::default();
        // A hundred thousand rows at a hundredth of a millisecond each is a
        // thousand milliseconds of work in slices of eight.
        let mut job = job(&world, 100_000, 512, 0.01);
        assert_eq!(run_to_end(&mut job).done(), Some(100_000));
        assert_eq!(job.progress(), (100_000, 100_000));
        // A batch of five hundred rows costs five milliseconds here, so two of
        // them fit in a slice and the third would not: about a thousand rows a
        // slice, and a thousand seconds of work in a hundred of them.
        assert!(
            (90..=110).contains(&job.slices()),
            "{} slices",
            job.slices()
        );
    }

    #[test]
    fn a_slice_that_costs_nothing_still_stops_at_the_row_allowance() {
        let world = World::default();
        // A free clock: without the row bound this would be one slice, and a
        // job that never hands the turn back is a frozen page.
        let mut job = job(&world, 100_000, 512, 0.0);
        assert_eq!(run_to_end(&mut job).done(), Some(100_000));
        assert_eq!(job.slices(), 100_000usize.div_ceil(SLICE_ROWS));
    }

    #[test]
    fn work_that_fits_in_one_slice_takes_one_slice() {
        let world = World::default();
        let mut job = job(&world, 100, 512, 0.0);
        assert_eq!(job.slice().and_then(Ended::done), Some(100));
        assert_eq!(job.slices(), 1);
        assert_eq!(job.progress(), (100, 100));
    }

    #[test]
    fn progress_only_ever_goes_forward_and_stops_at_the_total() {
        let world = World::default();
        let mut job = job(&world, 50_000, 512, 0.01);
        let mut previous = (0, 50_000);
        while job.slice().is_none() {
            let now = job.progress();
            assert!(now.0 >= previous.0, "{:?} after {:?}", now, previous);
            assert!(now.0 <= now.1);
            previous = now;
        }
        assert_eq!(job.progress(), (50_000, 50_000));
    }

    #[test]
    fn a_write_between_two_slices_stops_the_job_before_it_reads_anything() {
        let world = World::default();
        let read = Rc::new(Cell::new(0usize));
        let requests = Requests::new();
        let ticket = requests.issue();
        let mut job = {
            let read = read.clone();
            let now = world.now.clone();
            Job::new(
                ticket,
                world.version(),
                world.clock(),
                10_000,
                move |budget| {
                    read.set(read.get() + 1);
                    now.set(now.get() + 10.0);
                    budget.did(512);
                    if budget.exhausted() {
                        Step::More
                    } else {
                        Step::Done(0)
                    }
                },
            )
        };
        assert!(job.slice().is_none());
        assert_eq!(read.get(), 1);
        world.version.set(1);
        assert_eq!(job.slice(), Some(Ended::Stale));
        // The slice that met the new version did not run the work at all.
        assert_eq!(read.get(), 1);
        drop(requests);
    }

    #[test]
    fn a_write_during_the_last_slice_is_caught_at_the_end_of_it() {
        let world = World::default();
        let version = world.version.clone();
        let requests = Requests::new();
        let mut job = Job::new(
            requests.issue(),
            world.version(),
            world.clock(),
            1,
            move |_| {
                // The write lands while this slice is running, so the check at
                // the start of it saw the old version.
                version.set(version.get() + 1);
                Step::Done(7)
            },
        );
        assert_eq!(job.slice(), Some(Ended::Stale));
    }

    #[test]
    fn a_newer_job_stops_the_older_one_at_its_next_slice() {
        let world = World::default();
        let requests = Requests::new();
        let mut first = Job::new(
            requests.issue(),
            world.version(),
            world.clock(),
            10_000,
            counting(&world, 10_000, 512, 1.0),
        );
        assert!(first.slice().is_none());
        let _second = requests.issue();
        assert_eq!(first.slice(), Some(Ended::Cancelled));
        // And it stopped where it was rather than finishing: a cancelled job
        // leaves no half-result behind, because nothing is delivered.
        assert!(first.progress().0 < 10_000);
    }

    #[test]
    fn a_cancelled_job_stops_and_a_closed_scope_stops_everything() {
        let world = World::default();
        let requests = Requests::new();
        let mut job = Job::new(
            requests.issue(),
            world.version(),
            world.clock(),
            10_000,
            counting(&world, 10_000, 512, 1.0),
        );
        assert!(job.slice().is_none());
        requests.cancel();
        assert_eq!(job.slice(), Some(Ended::Cancelled));

        let requests = Requests::new();
        let mut job = Job::new(
            requests.issue(),
            world.version(),
            world.clock(),
            10_000,
            counting(&world, 10_000, 512, 1.0),
        );
        requests.close();
        assert_eq!(job.slice(), Some(Ended::Cancelled));
    }

    #[test]
    fn an_answer_that_arrives_after_a_newer_question_is_not_an_answer() {
        let world = World::default();
        let requests = Requests::new();
        // Two jobs from one handle, finished in the order they were started:
        // only the newer one may deliver, whichever ends first.
        let mut older = Job::new(requests.issue(), world.version(), world.clock(), 1, |_| {
            Step::Done("older")
        });
        let mut newer = Job::new(requests.issue(), world.version(), world.clock(), 1, |_| {
            Step::Done("newer")
        });
        assert_eq!(older.slice(), Some(Ended::Cancelled));
        assert_eq!(newer.slice(), Some(Ended::Done("newer")));
    }

    #[test]
    fn how_a_job_ended_has_a_name_a_view_can_show() {
        assert_eq!(Ended::Done(1u8).state(), "done");
        assert_eq!(Ended::<u8>::Stale.state(), "stale");
        assert_eq!(Ended::<u8>::Cancelled.state(), "cancelled");
        assert_eq!(Ended::<u8>::Stale.done(), None);
    }
}
