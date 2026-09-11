//! Which rows the table is showing, in which order.
//!
//! The sample is one thing and the view of it is another. A job builds a view,
//! sorted or filtered or both, and until the next job runs that view is what
//! the table reads through. Everything the table is shown is a position in
//! here; everything the sample is asked is a position in the sample.
//!
//! A write to the sample does not wait for a job. Rows that went are taken out
//! and the ones after them move up; rows that arrived go where they were put
//! if the view has no order of its own, and at the end if it has - a sorted
//! view has no correct place for a row nobody has compared yet. Either way the
//! view is marked as built for an older version, which is what "stale" on the
//! screen means: still usable, no longer the answer.

/// The rows a table is showing, in the order it shows them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    order: Vec<u32>,
    /// The sample version this order was built from.
    built_for: u64,
    /// Whether a job built it. An identity view has no order of its own.
    built: bool,
}

impl View {
    /// Every row, in the order the sample holds them.
    pub fn identity(len: usize, version: u64) -> Self {
        View {
            order: (0..len as u32).collect(),
            built_for: version,
            built: false,
        }
    }

    /// What a job produced.
    pub fn built(order: Vec<u32>, version: u64) -> Self {
        View {
            order,
            built_for: version,
            built: true,
        }
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// The sample position the table's row `at` is showing.
    pub fn row(&self, at: usize) -> Option<u32> {
        self.order.get(at).copied()
    }

    pub fn order(&self) -> &[u32] {
        &self.order
    }

    /// Whether the sample has been written to since this view was built.
    pub fn is_stale(&self, version: u64) -> bool {
        self.built_for != version
    }

    /// `count` rows were removed from the sample at position `at`.
    pub fn removed(&mut self, at: usize, count: usize) {
        let (at, end) = (at as u32, (at + count) as u32);
        self.order.retain(|row| *row < at || *row >= end);
        for row in self.order.iter_mut() {
            if *row >= end {
                *row -= end - at;
            }
        }
    }

    /// `count` rows were inserted into the sample at position `at`.
    pub fn inserted(&mut self, at: usize, count: usize) {
        let (at, count) = (at as u32, count as u32);
        for row in self.order.iter_mut() {
            if *row >= at {
                *row += count;
            }
        }
        if self.built {
            // A sorted or filtered view has nowhere to put a row that has not
            // been compared or matched, so it goes where it can be seen.
            self.order.extend(at..at + count);
        } else {
            // An identity view has no order of its own: the new rows belong
            // where the person put them.
            let position = at as usize;
            self.order
                .splice(position..position, at..at + count)
                .for_each(drop);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn built(order: &[u32]) -> View {
        View::built(order.to_vec(), 3)
    }

    #[test]
    fn an_identity_view_is_every_row_in_the_order_the_sample_has_them() {
        let view = View::identity(4, 0);
        assert_eq!(view.order(), [0, 1, 2, 3]);
        assert_eq!(view.row(2), Some(2));
        assert_eq!(view.row(4), None);
        assert!(!view.is_stale(0));
        assert!(view.is_stale(1));
    }

    #[test]
    fn deleting_takes_the_rows_out_and_moves_the_rest_up() {
        // A sorted view: what it is showing is scattered through the sample,
        // so a deletion in the middle of the sample is a deletion from several
        // places in the view and a renumbering of the rest.
        let mut view = built(&[3, 0, 4, 1, 2]);
        view.removed(1, 2);
        // Rows 1 and 2 are gone; 3 and 4 are now 1 and 2.
        assert_eq!(view.order(), [1, 0, 2]);
        assert!(view.is_stale(4));
    }

    #[test]
    fn deleting_from_the_end_of_the_sample_leaves_the_front_where_it_was() {
        let mut view = View::identity(5, 0);
        view.removed(3, 2);
        assert_eq!(view.order(), [0, 1, 2]);
    }

    #[test]
    fn deleting_more_than_there_is_leaves_what_is_below_it() {
        let mut view = View::identity(5, 0);
        view.removed(2, 99);
        assert_eq!(view.order(), [0, 1]);
    }

    #[test]
    fn inserting_into_an_identity_view_puts_the_rows_where_they_went() {
        let mut view = View::identity(4, 0);
        view.inserted(1, 2);
        assert_eq!(view.order(), [0, 1, 2, 3, 4, 5]);
        // The two new ones are at 1 and 2, which is where the sample has them.
        assert_eq!(view.row(1), Some(1));
        assert_eq!(view.row(3), Some(3));
    }

    #[test]
    fn inserting_into_a_view_with_an_order_puts_them_at_the_end() {
        let mut view = built(&[3, 0, 4, 1, 2]);
        view.inserted(1, 2);
        // Everything from 1 up is two further along, and the two new rows -
        // which no sort has compared and no filter has matched - are last.
        assert_eq!(view.order(), [5, 0, 6, 3, 4, 1, 2]);
    }

    #[test]
    fn inserting_at_the_end_appends_without_moving_anything() {
        let mut view = View::identity(3, 0);
        view.inserted(3, 1);
        assert_eq!(view.order(), [0, 1, 2, 3]);
    }

    #[test]
    fn a_filtered_view_keeps_only_what_it_had_through_both() {
        // Rows 1 and 3 of five, then row 0 goes and two arrive at the end.
        let mut view = built(&[1, 3]);
        view.removed(0, 1);
        assert_eq!(view.order(), [0, 2]);
        view.inserted(4, 2);
        assert_eq!(view.order(), [0, 2, 4, 5]);
        assert_eq!(view.len(), 4);
    }
}
