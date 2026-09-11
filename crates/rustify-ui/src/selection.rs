//! What is selected, by identity rather than by position.
//!
//! Three things read one selection - a table, a scene and the strip between
//! them - and each of them shows a different part of it. If each kept its own
//! answer to "is this selected", the three would disagree the first time a row
//! was deleted, and the disagreement would be silent.
//!
//! So the rules live here, and there are only three of them:
//!
//! - **Sorting and filtering keep it.** A selection is of things, not of rows;
//!   moving a thing does not deselect it, and neither does hiding it.
//! - **Deleting clears it.** A selected identity that no longer exists is not
//!   selected, it is a leak that would come back the moment an identity was
//!   reused - which is why identities never are.
//! - **How many are hidden is a question about the view.** The selection does
//!   not know what the view shows, so it is asked, and the answer changes when
//!   the view does without the selection changing at all.

use std::collections::BTreeSet;

/// Business identity. The application decides what it means; what matters here
/// is that it is stable and never reused.
pub type Id = u32;

/// How much of a selection a view is showing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub visible: usize,
    pub hidden: usize,
}

impl Counts {
    pub fn total(&self) -> usize {
        self.visible + self.hidden
    }
}

/// A set of selected identities, in ascending order.
///
/// Ordered rather than hashed: what reads it most often is a view walking its
/// rows in order, and a selection that can be walked in the same order can be
/// compared with one range of the view instead of probed once per row.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selection(BTreeSet<Id>);

impl Selection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, id: Id) -> bool {
        self.0.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn ids(&self) -> impl Iterator<Item = Id> + '_ {
        self.0.iter().copied()
    }

    /// Selects or deselects one identity, and reports what it now is.
    pub fn toggle(&mut self, id: Id) -> bool {
        if self.0.remove(&id) {
            return false;
        }
        self.0.insert(id);
        true
    }

    /// Selects exactly these, and nothing else.
    pub fn set(&mut self, ids: impl IntoIterator<Item = Id>) {
        self.0 = ids.into_iter().collect();
    }

    /// Adds these to what is already selected.
    pub fn add(&mut self, ids: impl IntoIterator<Item = Id>) {
        self.0.extend(ids);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Drops identities that no longer exist, and reports how many went.
    ///
    /// The caller passes what it deleted rather than what survives: a deletion
    /// is a few rows and the sample is a hundred thousand, and asking the
    /// question the other way round would walk all of them.
    pub fn remove_deleted(&mut self, deleted: &[Id]) -> usize {
        let before = self.0.len();
        for id in deleted {
            self.0.remove(id);
        }
        before - self.0.len()
    }

    /// How many selected identities the view shows, and how many it does not.
    ///
    /// `shown` is asked once per selected identity, not once per row: a
    /// selection is small and a view is not.
    pub fn counts(&self, shown: impl Fn(Id) -> bool) -> Counts {
        let visible = self.0.iter().filter(|id| shown(**id)).count();
        Counts {
            visible,
            hidden: self.0.len() - visible,
        }
    }
}

impl FromIterator<Id> for Selection {
    fn from_iter<T: IntoIterator<Item = Id>>(ids: T) -> Self {
        Self(ids.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selection(ids: &[Id]) -> Selection {
        ids.iter().copied().collect()
    }

    #[test]
    fn toggling_says_what_the_identity_now_is() {
        let mut chosen = Selection::new();
        assert!(chosen.toggle(7));
        assert!(chosen.contains(7));
        assert!(!chosen.toggle(7));
        assert!(!chosen.contains(7));
        assert!(chosen.is_empty());
    }

    #[test]
    fn deleting_an_identity_deselects_it_and_leaves_the_rest() {
        let mut chosen = selection(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        // Three of the ten deleted, and one that was never selected.
        assert_eq!(chosen.remove_deleted(&[2, 5, 9, 99]), 3);
        assert_eq!(chosen.len(), 7);
        for id in [1, 3, 4, 6, 7, 8, 10] {
            assert!(chosen.contains(id), "{id} was not deleted");
        }
        for id in [2, 5, 9] {
            assert!(!chosen.contains(id), "{id} was deleted");
        }
    }

    #[test]
    fn reordering_and_hiding_change_nothing_about_what_is_selected() {
        let chosen = selection(&[3, 1, 2]);
        // A sort is a new order of the same identities; a filter is a smaller
        // view of them. Neither reaches the selection at all, which is the
        // property - so what is checked is that the same set answers the same
        // way whatever view is put in front of it.
        assert_eq!(
            chosen.counts(|_| true),
            Counts {
                visible: 3,
                hidden: 0
            }
        );
        assert_eq!(
            chosen.counts(|_| false),
            Counts {
                visible: 0,
                hidden: 3
            }
        );
        assert_eq!(chosen.ids().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn how_many_are_hidden_is_a_question_about_the_view() {
        let chosen = selection(&[1, 2, 3, 4, 5]);
        let view: Vec<Id> = vec![2, 4];
        let counts = chosen.counts(|id| view.contains(&id));
        assert_eq!(
            counts,
            Counts {
                visible: 2,
                hidden: 3
            }
        );
        assert_eq!(counts.total(), chosen.len());
    }

    #[test]
    fn setting_replaces_and_adding_does_not() {
        let mut chosen = selection(&[1, 2]);
        chosen.add([2, 3]);
        assert_eq!(chosen.ids().collect::<Vec<_>>(), vec![1, 2, 3]);
        chosen.set([9]);
        assert_eq!(chosen.ids().collect::<Vec<_>>(), vec![9]);
        chosen.clear();
        assert!(chosen.is_empty());
    }

    #[test]
    fn an_empty_selection_has_nothing_hidden() {
        let chosen = Selection::new();
        assert_eq!(chosen.counts(|_| false), Counts::default());
    }
}
