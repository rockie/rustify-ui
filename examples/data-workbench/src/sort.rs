//! A stable sort of a row order, done a slice at a time.
//!
//! Sorting a hundred thousand rows on the main thread is not something that
//! can be done between two frames, and there is no second thread to move it
//! to: an ordinary deployment is not cross-origin isolated, so there is no
//! shared memory and no worker that could see these rows without copying
//! thirty-two megabytes to them.
//!
//! So the sort is a state machine that can be stopped anywhere. It is asked
//! whether its slice is over often enough that a slice is a few milliseconds,
//! and it keeps no borrow of the data between slices - the caller hands the
//! rows back in on every step, which is what lets it check first whether they
//! are still the rows the sort was started for.

use crate::dataset::{Row, CELL};

/// Rows sorted directly before the merge passes begin. Insertion sort over a
/// short block beats a merge pass over single elements, and it takes five
/// passes off the total.
const BLOCK: usize = 32;

/// How many rows are handled between two questions about the budget. Reading
/// a clock is not free, and the answer cannot change within a few hundred
/// comparisons.
const CHECK_EVERY: usize = 512;

/// Whether this slice is over. The caller owns the clock: this module is
/// arithmetic, and arithmetic is what can be tested without a browser.
pub type Exhausted<'a> = &'a mut dyn FnMut() -> bool;

pub struct Sort {
    order: Vec<u32>,
    scratch: Vec<u32>,
    column: usize,
    ascending: bool,
    /// Width of the runs that are already in order. Doubles once a pass is
    /// complete; the initial blocks are the first width.
    width: usize,
    /// Where the current pass has got to, in rows.
    at: usize,
    /// True until the initial blocks have been sorted.
    blocking: bool,
}

impl Sort {
    pub fn new(order: Vec<u32>, column: usize, ascending: bool) -> Self {
        let len = order.len();
        Sort {
            scratch: Vec::with_capacity(len),
            order,
            column,
            ascending,
            width: BLOCK,
            at: 0,
            blocking: true,
        }
    }

    /// Works until the budget is spent. Returns `true` once the order is
    /// complete; calling again after that does nothing.
    pub fn step(&mut self, cells: &[Row], exhausted: Exhausted<'_>) -> bool {
        let len = self.order.len();
        if len <= 1 {
            return true;
        }
        if self.blocking && !self.sort_blocks(cells, len, exhausted) {
            return false;
        }
        while self.width < len {
            if !self.merge_pass(cells, len, exhausted) {
                return false;
            }
        }
        true
    }

    pub fn into_order(self) -> Vec<u32> {
        self.order
    }

    fn key<'a>(&self, cells: &'a [Row], row: u32) -> &'a [u8] {
        let start = self.column * CELL;
        &cells[row as usize][start..start + CELL]
    }

    /// True when the blocks are all sorted.
    fn sort_blocks(&mut self, cells: &[Row], len: usize, exhausted: Exhausted<'_>) -> bool {
        let mut since_check = 0;
        while self.at < len {
            let end = (self.at + BLOCK).min(len);
            for i in (self.at + 1)..end {
                let mut j = i;
                while j > self.at && self.before(cells, self.order[j], self.order[j - 1]) {
                    self.order.swap(j, j - 1);
                    j -= 1;
                }
            }
            since_check += end - self.at;
            self.at = end;
            if since_check >= CHECK_EVERY {
                since_check = 0;
                if exhausted() {
                    return false;
                }
            }
        }
        self.blocking = false;
        self.at = 0;
        true
    }

    /// True when this pass is complete.
    fn merge_pass(&mut self, cells: &[Row], len: usize, exhausted: Exhausted<'_>) -> bool {
        let mut since_check = 0;
        while self.at < len {
            let left = self.at;
            let middle = (left + self.width).min(len);
            let right = (left + 2 * self.width).min(len);
            if middle < right {
                self.merge(cells, left, middle, right);
            }
            since_check += right - left;
            self.at = right;
            if since_check >= CHECK_EVERY {
                since_check = 0;
                if exhausted() {
                    return false;
                }
            }
        }
        self.width *= 2;
        self.at = 0;
        true
    }

    fn merge(&mut self, cells: &[Row], left: usize, middle: usize, right: usize) {
        self.scratch.clear();
        self.scratch.extend_from_slice(&self.order[left..right]);
        let split = middle - left;
        let (mut i, mut j) = (0, split);
        let end = right - left;
        for slot in left..right {
            // Equal keys take the left run first: that is what makes the sort
            // stable, and a stable sort is what makes a second sort on another
            // column a refinement rather than a reshuffle.
            let take_left = if i >= split {
                false
            } else if j >= end {
                true
            } else {
                !self.before(cells, self.scratch[j], self.scratch[i])
            };
            if take_left {
                self.order[slot] = self.scratch[i];
                i += 1;
            } else {
                self.order[slot] = self.scratch[j];
                j += 1;
            }
        }
    }

    fn before(&self, cells: &[Row], a: u32, b: u32) -> bool {
        let ordering = self.key(cells, a).cmp(self.key(cells, b));
        if self.ascending {
            ordering.is_lt()
        } else {
            ordering.is_gt()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::{Dataset, ROW_BYTES};

    fn rows(keys: &[&str]) -> Vec<Row> {
        keys.iter()
            .map(|key| {
                let mut row = [b' '; ROW_BYTES];
                for (slot, byte) in row.iter_mut().zip(key.bytes()) {
                    *slot = byte;
                }
                row
            })
            .collect()
    }

    fn run(cells: &[Row], column: usize, ascending: bool, slice: usize) -> (Vec<u32>, usize) {
        let mut sort = Sort::new((0..cells.len() as u32).collect(), column, ascending);
        let mut slices = 0;
        loop {
            slices += 1;
            let mut left = slice;
            let mut exhausted = move || {
                if left == 0 {
                    return true;
                }
                left -= 1;
                false
            };
            if sort.step(cells, &mut exhausted) {
                return (sort.into_order(), slices);
            }
            assert!(slices < 10_000, "a sliced sort that never finishes");
        }
    }

    #[test]
    fn a_short_order_is_finished_without_asking_about_the_budget() {
        // The budget is consulted once every few hundred rows, so an order
        // shorter than that is done in the slice it was started in however
        // little time that slice was given.
        let cells = rows(&["D", "B", "E", "A", "C", "B"]);
        let (order, slices) = run(&cells, 0, true, 0);
        assert_eq!(slices, 1);
        assert_eq!(order, vec![3, 1, 5, 4, 0, 2]);
    }

    #[test]
    fn equal_keys_keep_their_order() {
        // Two hundred rows of two distinct keys: a stable sort leaves each
        // group in the order it found it, and an unstable one will not.
        let keys: Vec<&str> = (0..200)
            .map(|i| if i % 2 == 0 { "A" } else { "B" })
            .collect();
        let cells = rows(&keys);
        let (order, _) = run(&cells, 0, true, 3);
        let evens: Vec<u32> = (0..200).filter(|i| i % 2 == 0).collect();
        let odds: Vec<u32> = (0..200).filter(|i| i % 2 == 1).collect();
        assert_eq!(order[..100], evens[..]);
        assert_eq!(order[100..], odds[..]);
    }

    #[test]
    fn descending_is_the_reverse_of_ascending_for_distinct_keys() {
        let cells = rows(&["D", "B", "E", "A", "C"]);
        let (up, _) = run(&cells, 0, true, 2);
        let (mut down, _) = run(&cells, 0, false, 2);
        down.reverse();
        assert_eq!(up, down);
    }

    #[test]
    fn a_second_column_is_sorted_independently() {
        let mut cells = rows(&["A", "A", "A"]);
        for (row, value) in cells.iter_mut().zip(["C", "A", "B"]) {
            row[CELL..CELL + value.len()].copy_from_slice(value.as_bytes());
        }
        let (order, _) = run(&cells, 1, true, usize::MAX);
        assert_eq!(order, vec![1, 2, 0]);
    }

    #[test]
    fn the_generated_sample_sorts_the_same_whole_or_sliced() {
        // Only the first few thousand rows: the property is the slicing, and
        // the whole sample is what the browser probe measures.
        let data = Dataset::generate();
        let cells = &data.rows()[..4_096];
        let (whole, _) = run(cells, 3, true, usize::MAX);
        let (sliced, slices) = run(cells, 3, true, 1);
        assert_eq!(whole, sliced);
        assert!(slices > 8, "a four thousand row sort in {slices} slices");
        let keys: Vec<&[u8]> = whole
            .iter()
            .map(|row| &cells[*row as usize][3 * CELL..4 * CELL])
            .collect();
        assert!(keys.windows(2).all(|pair| pair[0] <= pair[1]));
    }
}
