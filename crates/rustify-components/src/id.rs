//! Ids for the wiring ARIA does by name.
//!
//! A label points at its control, an error at the field it belongs to, a
//! dialog at its own title. Each of those is an id, and the ids have to be
//! unique on the page without the application having to invent them.

use std::cell::Cell;

/// A page-unique id, prefixed so a person reading the DOM can tell what it
/// belongs to.
///
/// Monotonic rather than random: the same page renders the same ids twice
/// running, which is what lets a test name one and a screenshot diff stay
/// still.
pub fn next(prefix: &str) -> String {
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|next| {
        let id = next.get();
        next.set(id + 1);
        format!("rui-{prefix}-{id}")
    })
}

#[cfg(test)]
mod tests {
    use super::next;

    #[test]
    fn two_ids_are_never_the_same_and_say_what_they_are_for() {
        let first = next("field");
        let second = next("field");
        assert_ne!(first, second);
        assert!(first.starts_with("rui-field-"), "{first}");
    }
}
