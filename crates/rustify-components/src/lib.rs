//! DOM components for applications built on the Rustify UI SDK.
//!
//! The class strings and the two macros that build them come from Rust/UI; the
//! provenance of every imported file, and whether it is still verbatim, is in
//! the `rust_ui` section of `sources.lock.json`. What the fork changes is the
//! part a browser can observe: no inline script or style, a value that only
//! ever comes from the application, and a name, role, value and state for
//! everything a person can reach.

pub mod macros;

/// The path the application is currently showing, for the components that need
/// to know whether they point at it.
///
/// This crate has no router. A link marks itself current by comparing its
/// `href` with this signal, and an application that provides no signal simply
/// has no current link - which is the right answer for a page that is not
/// navigating anywhere.
#[derive(Clone, Copy)]
pub struct CurrentPath(pub leptos::prelude::Signal<String>);

/// Publishes the current path to every component below this point.
pub fn provide_current_path(path: leptos::prelude::Signal<String>) {
    leptos::prelude::provide_context(CurrentPath(path));
}

pub fn current_path() -> Option<leptos::prelude::Signal<String>> {
    leptos::prelude::use_context::<CurrentPath>().map(|path| path.0)
}

/// Whether `path` is at, or below, `target`.
///
/// Below counts, so a link to `/objects` stays marked while the page shows
/// `/objects/17`. The segment boundary is what stops `/objects-archive` from
/// counting as one of them.
pub fn is_within(path: &str, target: &str) -> bool {
    if target.is_empty() {
        return false;
    }
    let path = path.trim_end_matches('/');
    let target = target.trim_end_matches('/');
    if target.is_empty() {
        return path.is_empty();
    }
    path == target
        || path
            .strip_prefix(target)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::is_within;

    #[test]
    fn a_link_is_current_for_its_own_page_and_the_pages_under_it() {
        assert!(is_within("/objects", "/objects"));
        assert!(is_within("/objects/17", "/objects"));
        assert!(is_within("/objects/17/notes", "/objects"));
        assert!(is_within("/objects/", "/objects"));
    }

    #[test]
    fn a_link_is_not_current_for_a_path_that_merely_starts_the_same() {
        // The bug this exists to prevent: `starts_with` alone marks the
        // archive link current while the user is looking at the object list.
        assert!(!is_within("/objects-archive", "/objects"));
        assert!(!is_within("/other", "/objects"));
        assert!(!is_within("/", "/objects"));
    }

    #[test]
    fn a_link_with_no_target_is_never_current() {
        assert!(!is_within("/objects", ""));
    }
}
