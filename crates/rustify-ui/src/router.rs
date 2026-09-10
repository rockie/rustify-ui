//! The part of a router that has nothing to do with a browser.
//!
//! Two jobs. Matching a path against the application's routes, which is
//! ordinary. And keeping track of which history entry the user is on, which is
//! not: a guard that refuses a back has to put the user back where they were,
//! and the browser gives it one move to do it with. The probes in
//! `tests/browser/p2-navigation.spec.ts` are where the rules below come from.

use std::collections::BTreeMap;

/// Where the application is, as the router understands it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Location {
    /// Without the base, and always starting with `/`.
    pub path: String,
    /// Everything from `?` onwards, or empty. Handed over as it arrived: what
    /// a query means is the application's business.
    pub search: String,
}

impl Location {
    pub fn new(path: impl Into<String>, search: impl Into<String>) -> Self {
        Self {
            path: normalise(&path.into()),
            search: search.into(),
        }
    }

    /// Path and query as they appear in the address bar, under `base`.
    pub fn href(&self, base: &str) -> String {
        let base = base.trim_end_matches('/');
        format!("{base}{}{}", self.path, self.search)
    }
}

/// The parameters a match bound, by name.
pub type Params = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Static(String),
    Param(String),
}

/// One route the application declared.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    segments: Vec<Segment>,
}

impl Route {
    /// `"/objects/:id"`. A `:name` segment binds; everything else is literal.
    pub fn new(pattern: &str) -> Self {
        Self {
            segments: normalise(pattern)
                .split('/')
                .filter(|segment| !segment.is_empty())
                .map(|segment| match segment.strip_prefix(':') {
                    Some(name) => Segment::Param(name.to_string()),
                    None => Segment::Static(segment.to_string()),
                })
                .collect(),
        }
    }

    fn matches(&self, path: &str) -> Option<Params> {
        let path = normalise(path);
        let parts: Vec<&str> = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        if parts.len() != self.segments.len() {
            return None;
        }
        let mut params = Params::new();
        for (segment, part) in self.segments.iter().zip(parts) {
            match segment {
                Segment::Static(literal) if literal == part => {}
                Segment::Static(_) => return None,
                Segment::Param(name) => {
                    params.insert(name.clone(), decode(part));
                }
            }
        }
        Some(params)
    }
}

/// The application's routes, in the order they are tried.
#[derive(Clone, Debug, Default)]
pub struct Routes(Vec<Route>);

impl Routes {
    pub fn new(patterns: &[&str]) -> Self {
        Self(patterns.iter().map(|pattern| Route::new(pattern)).collect())
    }

    /// Which route a path is, and what it bound. `None` is the application's
    /// not-found view - a real answer, not a failure.
    pub fn match_path(&self, path: &str) -> Option<(usize, Params)> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(index, route)| route.matches(path).map(|params| (index, params)))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A leading slash, no trailing one, and never empty.
fn normalise(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

/// Percent-decoding, for the one place a parameter can carry one: a business
/// id with a slash or a space in it.
fn decode(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).ok();
            if let Some(byte) = hex.and_then(|hex| u8::from_str_radix(hex, 16).ok()) {
                out.push(byte);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| segment.to_string())
}

/// A path with `base` taken off the front, or `None` when it is not under it.
pub fn strip_base<'a>(base: &str, path: &'a str) -> Option<&'a str> {
    let base = base.trim_end_matches('/');
    if base.is_empty() {
        return Some(path);
    }
    let rest = path.strip_prefix(base)?;
    if rest.is_empty() {
        Some("/")
    } else if rest.starts_with('/') {
        Some(rest)
    } else {
        // `/tools/demo-other` is not under `/tools/demo`.
        None
    }
}

/// What the router should do about a history event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrival {
    /// Show it. Either nothing objected, or there is nothing to be done.
    Accept,
    /// A guard refused. Move by this many entries to undo it, and wait.
    Restore { delta: i64 },
    /// The recovery is not converging. Accept where we are and say so.
    GiveUp,
}

/// What asking to navigate turned into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Navigation {
    Done,
    /// A guard refused; the address bar and the view are unchanged.
    Blocked,
    /// A recovery is in flight. Asking during one would race it.
    Busy,
}

/// Which entry the user is on, and what to do when that changes underneath.
///
/// The sequence number is the whole design. A URL is not an identity - the
/// same one appears at several entries - and the number of moves a `go()` will
/// take is not knowable from the number asked for, because the user is pressing
/// back at the same time. So every decision here is made from the number the
/// browser just reported.
#[derive(Clone, Debug)]
pub struct History {
    index: u64,
    next: u64,
    restoring: Option<Restore>,
}

#[derive(Clone, Copy, Debug)]
struct Restore {
    target: u64,
    attempts: u8,
}

impl History {
    /// The entry the page loaded on. Reloading keeps the number it already had.
    pub fn new(index: u64) -> Self {
        Self {
            index,
            next: index + 1,
            restoring: None,
        }
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    /// Whether a recovery is in flight.
    pub fn restoring(&self) -> bool {
        self.restoring.is_some()
    }

    /// The number to stamp on a new entry.
    pub fn push(&mut self) -> u64 {
        let index = self.next;
        self.next += 1;
        self.index = index;
        // Pushing truncates whatever was ahead, so nothing can be restored to
        // it any more.
        self.restoring = None;
        index
    }

    /// Replacing keeps the entry, and therefore its number.
    pub fn replace(&mut self) -> u64 {
        self.index
    }

    /// How many times a recovery may be re-aimed before it is abandoned. Three
    /// is enough for a user pressing back while one is in flight and few
    /// enough that a fight with the browser ends.
    const ATTEMPTS: u8 = 3;

    /// A history event arrived at `entry`, and `blocked` says whether a guard
    /// refused to leave.
    ///
    /// `entry` is `None` for an entry this router did not push - a host
    /// script's, an older build's, a restored session's. Those are not ours to
    /// undo, so they are accepted.
    pub fn arrived(&mut self, entry: Option<u64>, blocked: bool) -> Arrival {
        let Some(entry) = entry else {
            self.restoring = None;
            return Arrival::Accept;
        };
        if let Some(restore) = self.restoring {
            if entry == restore.target {
                self.restoring = None;
                self.index = entry;
                return Arrival::Accept;
            }
            if restore.attempts >= Self::ATTEMPTS {
                self.restoring = None;
                self.index = entry;
                return Arrival::GiveUp;
            }
            self.restoring = Some(Restore {
                target: restore.target,
                attempts: restore.attempts + 1,
            });
            return Arrival::Restore {
                delta: restore.target as i64 - entry as i64,
            };
        }
        if !blocked {
            self.index = entry;
            return Arrival::Accept;
        }
        // Refused. The entry we are on is where the user should stay, and one
        // move of the difference is what puts them there.
        let delta = self.index as i64 - entry as i64;
        if delta == 0 {
            return Arrival::Accept;
        }
        self.restoring = Some(Restore {
            target: self.index,
            attempts: 1,
        });
        Arrival::Restore { delta }
    }
}

#[cfg(test)]
mod tests {
    use super::{strip_base, Arrival, History, Location, Routes};

    fn routes() -> Routes {
        Routes::new(&["/", "/objects", "/objects/:id", "/objects/:id/notes"])
    }

    #[test]
    fn a_static_route_matches_only_itself() {
        assert_eq!(
            routes().match_path("/objects").map(|(index, _)| index),
            Some(1)
        );
        assert_eq!(routes().match_path("/objects-archive"), None);
        assert_eq!(routes().match_path("/"), Some((0, Default::default())));
    }

    #[test]
    fn a_parameter_binds_by_name() {
        let (index, params) = routes().match_path("/objects/17").expect("matched");
        assert_eq!(index, 2);
        assert_eq!(params.get("id").map(String::as_str), Some("17"));

        let (index, params) = routes().match_path("/objects/17/notes").expect("matched");
        assert_eq!(index, 3);
        assert_eq!(params.get("id").map(String::as_str), Some("17"));
    }

    #[test]
    fn a_parameter_arrives_decoded() {
        let (_, params) = routes().match_path("/objects/a%20b").expect("matched");
        assert_eq!(params.get("id").map(String::as_str), Some("a b"));
    }

    #[test]
    fn a_path_that_matches_nothing_is_an_answer_rather_than_an_error() {
        assert_eq!(routes().match_path("/nowhere"), None);
        assert_eq!(routes().match_path("/objects/17/notes/extra"), None);
    }

    #[test]
    fn a_trailing_slash_is_the_same_place() {
        assert_eq!(
            routes().match_path("/objects/17/"),
            routes().match_path("/objects/17")
        );
        assert_eq!(Location::new("/objects/", "").path, "/objects");
    }

    #[test]
    fn an_address_is_the_base_and_the_path_and_the_query() {
        let at = Location::new("/objects/17", "?tab=notes");
        assert_eq!(at.href("/"), "/objects/17?tab=notes");
        assert_eq!(at.href("/tools/demo/"), "/tools/demo/objects/17?tab=notes");
    }

    #[test]
    fn a_path_outside_the_base_is_not_ours() {
        assert_eq!(
            strip_base("/tools/demo/", "/tools/demo/objects"),
            Some("/objects")
        );
        assert_eq!(strip_base("/tools/demo/", "/tools/demo"), Some("/"));
        // The trap: a sibling whose name starts the same.
        assert_eq!(strip_base("/tools/demo/", "/tools/demo-other/x"), None);
        assert_eq!(strip_base("/tools/demo/", "/elsewhere"), None);
        assert_eq!(strip_base("/", "/objects"), Some("/objects"));
    }

    #[test]
    fn a_push_takes_the_next_number_and_a_replace_keeps_the_one_it_has() {
        let mut history = History::new(0);
        assert_eq!(history.push(), 1);
        assert_eq!(history.push(), 2);
        assert_eq!(history.replace(), 2);
        assert_eq!(history.index(), 2);
    }

    #[test]
    fn an_allowed_move_is_simply_accepted() {
        let mut history = History::new(4);
        assert_eq!(history.arrived(Some(1), false), Arrival::Accept);
        assert_eq!(history.index(), 1);
        assert!(!history.restoring());
    }

    #[test]
    fn a_refused_move_is_undone_in_one_go_of_the_difference() {
        // The case the probe measured: at entry 4, the user goes back three.
        let mut history = History::new(4);
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        assert!(history.restoring());
        // Still on 4 as far as the application is concerned: the guard said no.
        assert_eq!(history.index(), 4);

        assert_eq!(history.arrived(Some(4), true), Arrival::Accept);
        assert!(!history.restoring());
        assert_eq!(history.index(), 4);
    }

    #[test]
    fn a_back_pressed_during_a_recovery_is_re_aimed_from_where_it_landed() {
        let mut history = History::new(4);
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        // The user pressed back again; the recovery landed somewhere else.
        assert_eq!(
            history.arrived(Some(2), true),
            Arrival::Restore { delta: 2 }
        );
        assert_eq!(history.arrived(Some(4), true), Arrival::Accept);
        assert_eq!(history.index(), 4);
    }

    #[test]
    fn a_recovery_that_will_not_converge_stops_rather_than_fighting() {
        let mut history = History::new(4);
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        // Three re-aims and it is still not there: the user is somewhere else
        // and the router says so instead of moving them again.
        assert_eq!(history.arrived(Some(1), true), Arrival::GiveUp);
        assert!(!history.restoring());
        assert_eq!(history.index(), 1);
    }

    #[test]
    fn an_entry_this_router_did_not_push_is_left_alone() {
        let mut history = History::new(4);
        // No number: a host script's entry, or a session restored by the
        // browser. Not ours to undo, even when a guard would have refused.
        assert_eq!(history.arrived(None, true), Arrival::Accept);
        assert!(!history.restoring());
    }

    #[test]
    fn a_push_ends_any_recovery_because_it_truncates_what_was_ahead() {
        let mut history = History::new(4);
        assert_eq!(
            history.arrived(Some(1), true),
            Arrival::Restore { delta: 3 }
        );
        history.push();
        assert!(!history.restoring());
    }
}
