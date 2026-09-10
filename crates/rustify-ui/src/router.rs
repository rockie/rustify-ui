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

#[cfg(target_arch = "wasm32")]
pub use browser::{
    claim_url, navigate, provide_router, provide_routes, use_location, use_params, use_route, Link,
    NavigationGuard, UrlClaim,
};

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::{strip_base, Arrival, History, Location, Navigation, Params, Routes};
    use crate::diagnostics::{note, record, ErrorKind};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::{JsCast, JsValue};
    use leptos::web_sys::{Element, Event, HtmlElement, MouseEvent};
    use send_wrapper::SendWrapper;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::Arc;

    thread_local! {
        /// One page, one address bar, one owner. A slot rather than a counter:
        /// the question is not how many asked but whether anyone has it.
        static URL_TAKEN: Cell<bool> = const { Cell::new(false) };
    }

    /// Proof that this scope owns the page's URL, for as long as it is held.
    ///
    /// The token is never read; it is dropped. Cloning it is what lets the
    /// handle and the router both hold the claim while the slot is released
    /// only when the last of them goes - which is why this is an `Rc` around a
    /// `Drop` type rather than a `Drop` on the claim itself.
    #[derive(Clone)]
    pub struct UrlClaim(#[allow(dead_code)] Rc<ClaimToken>);

    struct ClaimToken;

    impl Drop for ClaimToken {
        fn drop(&mut self) {
            URL_TAKEN.with(|taken| taken.set(false));
        }
    }

    /// Takes the page's URL, if nobody has it.
    pub fn claim_url() -> Option<UrlClaim> {
        URL_TAKEN.with(|taken| {
            if taken.get() {
                None
            } else {
                taken.set(true);
                Some(UrlClaim(Rc::new(ClaimToken)))
            }
        })
    }

    /// What every scope has, whether or not it owns the address bar.
    #[derive(Clone)]
    struct Router {
        location: RwSignal<Location>,
        base: Arc<str>,
        /// `None` for a scope with a location of its own: it routes, and the
        /// page's address bar is somebody else's.
        owner: Option<SendWrapper<Owned>>,
        guards: RwSignal<Vec<Signal<bool>>>,
    }

    #[derive(Clone)]
    struct Owned {
        history: Rc<RefCell<History>>,
        /// Kept alive for the life of the scope; dropping them unregisters.
        _listeners: Rc<Listeners>,
        _claim: UrlClaim,
    }

    struct Listeners {
        popstate: Closure<dyn FnMut(Event)>,
        click: Closure<dyn FnMut(MouseEvent)>,
        before_unload: RefCell<Option<Closure<dyn FnMut(Event)>>>,
        container: Element,
    }

    impl Drop for Listeners {
        fn drop(&mut self) {
            let _ = window().remove_event_listener_with_callback(
                "popstate",
                self.popstate.as_ref().unchecked_ref(),
            );
            let _ = self
                .container
                .remove_event_listener_with_callback("click", self.click.as_ref().unchecked_ref());
            if let Some(before_unload) = self.before_unload.borrow().as_ref() {
                let _ = window().remove_event_listener_with_callback(
                    "beforeunload",
                    before_unload.as_ref().unchecked_ref(),
                );
            }
        }
    }

    fn read_index() -> Option<u64> {
        let state = window().history().ok()?.state().ok()?;
        let rustify = js_sys::Reflect::get(&state, &JsValue::from_str("rustify")).ok()?;
        let index = js_sys::Reflect::get(&rustify, &JsValue::from_str("index")).ok()?;
        index.as_f64().map(|index| index as u64)
    }

    fn state_for(index: u64) -> JsValue {
        let rustify = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &rustify,
            &JsValue::from_str("index"),
            &JsValue::from_f64(index as f64),
        );
        let state = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&state, &JsValue::from_str("rustify"), &rustify);
        state.into()
    }

    fn here(base: &str) -> Location {
        let location = window().location();
        let path = location.pathname().unwrap_or_else(|_| "/".to_string());
        let search = location.search().unwrap_or_default();
        Location::new(strip_base(base, &path).unwrap_or("/"), search)
    }

    /// Installs a scope's router. Called from `mount`, inside the scope's owner
    /// so that the listeners go when the scope does.
    pub fn provide_router(container: &HtmlElement, claim: Option<UrlClaim>, base: &str) {
        let base: Arc<str> = if base.is_empty() { "/" } else { base }.into();
        let guards: RwSignal<Vec<Signal<bool>>> = RwSignal::new(Vec::new());

        let Some(claim) = claim else {
            // A location of its own: it routes, it just does not touch the
            // page. Nothing is registered on `window`, which is the point -
            // an embedded instance must not answer for a page it does not own.
            provide_context(Router {
                location: RwSignal::new(Location::new("/", "")),
                base,
                owner: None,
                guards,
            });
            return;
        };

        let location = RwSignal::new(here(&base));
        // The entry the page loaded on keeps whatever number it already had:
        // a reload must not renumber the history behind the user.
        let index = read_index().unwrap_or(0);
        if read_index().is_none() {
            let _ = window()
                .history()
                .and_then(|history| history.replace_state(&state_for(index), ""));
        }
        let history = Rc::new(RefCell::new(History::new(index)));

        let blocked = {
            let guards = guards;
            move || guards.with_untracked(|guards| guards.iter().any(|guard| guard.get_untracked()))
        };

        let popstate = {
            let history = history.clone();
            let base = base.clone();
            Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let entry = read_index();
                let arrival = history.borrow_mut().arrived(entry, blocked());
                match arrival {
                    Arrival::Accept => location.set(here(&base)),
                    Arrival::Restore { delta } => {
                        record(note(
                            ErrorKind::NavigationBlocked,
                            "a guard refused a move through history",
                        ));
                        let _ = window()
                            .history()
                            .map(|history| history.go_with_delta(delta as i32));
                    }
                    Arrival::GiveUp => {
                        record(note(
                            ErrorKind::NavigationRestoreFailed,
                            "the recovery did not converge; the view follows the browser",
                        ));
                        location.set(here(&base));
                    }
                }
            })
        };
        let _ = window()
            .add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref());

        let click = {
            let history = history.clone();
            let base = base.clone();
            Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
                let Some(href) = anchor_target(&event, &base) else {
                    return;
                };
                // From here on it is ours: the browser must not also navigate.
                event.prevent_default();
                let asked = go(&history, location, &base, &href, false, &blocked);
                report(asked);
            })
        };
        let _ = container.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());

        provide_context(Router {
            location,
            base,
            owner: Some(SendWrapper::new(Owned {
                history,
                _listeners: Rc::new(Listeners {
                    popstate,
                    click,
                    before_unload: RefCell::new(None),
                    container: container.clone().into(),
                }),
                _claim: claim,
            })),
            guards,
        });
    }

    /// Whether a click is one this scope should answer for, and where to.
    ///
    /// The conditions are the browser's own for "this link opens here": the
    /// primary button, no modifier, same origin, under our base, no `target`,
    /// no `download`, not an external `rel`, and nothing has already claimed
    /// it. A link that fails any of them is the page's business, not ours.
    fn anchor_target(event: &MouseEvent, base: &str) -> Option<String> {
        if event.default_prevented() || event.button() != 0 {
            return None;
        }
        if event.meta_key() || event.ctrl_key() || event.shift_key() || event.alt_key() {
            return None;
        }
        let target = event.target()?.dyn_into::<Element>().ok()?;
        let anchor = target.closest("a").ok().flatten()?;
        if anchor.has_attribute("download") || anchor.has_attribute("target") {
            return None;
        }
        if anchor
            .get_attribute("rel")
            .is_some_and(|rel| rel.split_whitespace().any(|part| part == "external"))
        {
            return None;
        }
        let href = anchor.get_attribute("href")?;
        let url =
            leptos::web_sys::Url::new_with_base(&href, &window().location().href().ok()?).ok()?;
        if url.origin() != window().location().origin().ok()? {
            return None;
        }
        let path = url.pathname();
        let rest = strip_base(base, &path)?;
        Some(format!("{rest}{}", url.search()))
    }

    fn report(asked: Navigation) {
        match asked {
            Navigation::Done => {}
            Navigation::Blocked => record(note(
                ErrorKind::NavigationBlocked,
                "a guard refused a navigation",
            )),
            Navigation::Busy => record(note(
                ErrorKind::NavigationBusy,
                "asked while the router was putting the user back",
            )),
        }
    }

    /// The one place an owner's address bar changes.
    fn go(
        history: &Rc<RefCell<History>>,
        location: RwSignal<Location>,
        base: &str,
        to: &str,
        replace: bool,
        blocked: &impl Fn() -> bool,
    ) -> Navigation {
        if history.borrow().restoring() {
            return Navigation::Busy;
        }
        if blocked() {
            return Navigation::Blocked;
        }
        let (path, search) = match to.split_once('?') {
            Some((path, search)) => (path, format!("?{search}")),
            None => (to, String::new()),
        };
        let next = Location::new(path, search);
        let index = if replace {
            history.borrow_mut().replace()
        } else {
            history.borrow_mut().push()
        };
        let url = next.href(base);
        let _ = window().history().map(|window_history| {
            if replace {
                window_history.replace_state_with_url(&state_for(index), "", Some(&url))
            } else {
                window_history.push_state_with_url(&state_for(index), "", Some(&url))
            }
        });
        location.set(next);
        Navigation::Done
    }

    fn router() -> Option<Router> {
        use_context::<Router>()
    }

    /// Where this scope is.
    pub fn use_location() -> Signal<Location> {
        match router() {
            Some(router) => router.location.into(),
            None => Signal::derive(|| Location::new("/", "")),
        }
    }

    /// Asks this scope to go somewhere.
    pub fn navigate(to: &str, replace: bool) -> Navigation {
        let Some(router) = router() else {
            return Navigation::Done;
        };
        let blocked = {
            let guards = router.guards;
            move || guards.with_untracked(|guards| guards.iter().any(|guard| guard.get_untracked()))
        };
        let asked = match &router.owner {
            Some(owner) => go(
                &owner.history,
                router.location,
                &router.base,
                to,
                replace,
                &blocked,
            ),
            None => {
                // A scope with its own location has no history to move
                // through, but it has the same guards.
                if blocked() {
                    Navigation::Blocked
                } else {
                    let (path, search) = match to.split_once('?') {
                        Some((path, search)) => (path, format!("?{search}")),
                        None => (to, String::new()),
                    };
                    router.location.set(Location::new(path, search));
                    Navigation::Done
                }
            }
        };
        report(asked);
        asked
    }

    /// The application's route table, for `use_route` and `use_params`.
    pub fn provide_routes(routes: Routes) {
        provide_context(Arc::new(routes));
    }

    /// Which route the current location is, and what it bound.
    pub fn use_route() -> Signal<Option<(usize, Params)>> {
        let location = use_location();
        let routes = use_context::<Arc<Routes>>();
        Signal::derive(move || {
            let routes = routes.clone()?;
            routes.match_path(&location.get().path)
        })
    }

    /// What the current route bound. Empty when nothing matched, which is also
    /// what a route with no parameters binds - the not-found view reads the
    /// location, not the parameters.
    pub fn use_params() -> Signal<Params> {
        let route = use_route();
        Signal::derive(move || route.get().map(|(_, params)| params).unwrap_or_default())
    }

    /// Registers a signal that says this view has work the user would lose.
    ///
    /// While any registered guard is true, moving away is refused and the
    /// browser is told to ask before the page is closed. Deregistered when the
    /// view goes, so a guard cannot outlive the thing it was guarding.
    pub struct NavigationGuard;

    impl NavigationGuard {
        pub fn register(has_unsaved: Signal<bool>) {
            let Some(router) = router() else {
                return;
            };
            let guards = router.guards;
            guards.update(|guards| guards.push(has_unsaved));
            let at = guards.with_untracked(Vec::len) - 1;

            // The browser's own "are you sure" is registered only while there
            // is something to lose: a page that always asks is a page whose
            // users learn to dismiss it.
            if let Some(owner) = router.owner.clone() {
                Effect::new(move || {
                    let unsaved = has_unsaved.get();
                    let listeners = &owner._listeners;
                    let mut slot = listeners.before_unload.borrow_mut();
                    match (unsaved, slot.is_some()) {
                        (true, false) => {
                            let handler = Closure::<dyn FnMut(Event)>::new(|event: Event| {
                                event.prevent_default();
                            });
                            let _ = window().add_event_listener_with_callback(
                                "beforeunload",
                                handler.as_ref().unchecked_ref(),
                            );
                            *slot = Some(handler);
                        }
                        (false, true) => {
                            if let Some(handler) = slot.take() {
                                let _ = window().remove_event_listener_with_callback(
                                    "beforeunload",
                                    handler.as_ref().unchecked_ref(),
                                );
                            }
                        }
                        _ => {}
                    }
                });
            }

            on_cleanup(move || {
                guards.update(|guards| {
                    if at < guards.len() {
                        guards.remove(at);
                    }
                });
            });
        }
    }

    /// A link within this scope.
    ///
    /// It is a real `<a>` with a real `href`, so it can be opened in a new tab,
    /// copied, and read by everything that reads links. What the scope's own
    /// click handler does is take the ordinary case.
    #[component]
    pub fn Link(
        #[prop(into)] href: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        let location = use_location();
        let base = router()
            .map(|router| router.base.to_string())
            .unwrap_or_default();
        let target = href.clone();
        let current = move || (location.get().path == super::normalise(&target)).then_some("page");
        let full = Location::new(
            href.split('?').next().unwrap_or("/"),
            href.split_once('?')
                .map(|(_, q)| format!("?{q}"))
                .unwrap_or_default(),
        )
        .href(&base);
        view! {
            <a class=class data-testid=test_id href=full aria-current=current>
                {children()}
            </a>
        }
    }
}
