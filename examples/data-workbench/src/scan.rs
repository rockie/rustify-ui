//! One pass over the rows: what a filter and a find are made of.
//!
//! Both ask the same question of each row in turn, and differ only in what
//! they do with a yes - a filter collects every one, a find stops at the
//! first. Like the sort, it is a state machine that can be stopped anywhere
//! and keeps no borrow of the data between slices, so the caller can check
//! before each one whether these are still the rows it started on.

use crate::dataset::{Dataset, Row, CELL, COLUMNS};

/// Rows between two questions about the budget. The same figure the sort uses,
/// for the same reason.
const CHECK_EVERY: usize = 512;

pub use crate::sort::Exhausted;

/// What a row is being asked.
pub enum Rule {
    /// Some cell of it contains this text. Case sensitive, and matched within
    /// a cell rather than across the row: cells are fixed width and butt up
    /// against each other, so a search over the whole row would find things
    /// that are not in it.
    Text(Vec<u8>),
    /// It is in this group. The tree is navigation over positions, so this is
    /// a question about where a row is rather than about what is in it.
    Group(usize, usize),
}

impl Rule {
    pub fn text(needle: &str) -> Self {
        Rule::Text(needle.as_bytes().to_vec())
    }

    /// Whether the row at `position`, whose cells are `cells`, answers yes.
    fn holds(&self, cells: &Row, position: usize) -> bool {
        match self {
            Rule::Text(needle) => {
                if needle.is_empty() || needle.len() > CELL {
                    // An empty search is not a filter, and one longer than a
                    // cell cannot be inside one.
                    return needle.is_empty();
                }
                (0..COLUMNS).any(|column| {
                    let start = column * CELL;
                    cells[start..start + CELL]
                        .windows(needle.len())
                        .any(|window| window == needle.as_slice())
                })
            }
            Rule::Group(group, child) => Dataset::group(position) == (*group, *child),
        }
    }
}

pub struct Scan {
    rule: Rule,
    /// How far through the order it has got.
    at: usize,
    /// The rows that answered yes, in the order they were met.
    hits: Vec<u32>,
    /// Stop at the first one rather than collecting them all.
    first_only: bool,
    /// Where in the order the first one was, for a find.
    found: Option<usize>,
}

impl Scan {
    /// A filter: every row that answers yes.
    pub fn filter(rule: Rule) -> Self {
        Self::new(rule, false)
    }

    /// A find: where the first row that answers yes is in the order.
    pub fn find(rule: Rule) -> Self {
        Self::new(rule, true)
    }

    fn new(rule: Rule, first_only: bool) -> Self {
        Scan {
            rule,
            at: 0,
            hits: Vec::new(),
            first_only,
            found: None,
        }
    }

    /// Asks the rows named by `order`, in that order, until the budget is
    /// spent. Returns `true` once the pass is complete.
    ///
    /// `order` holds row positions: for a filter that is every row, and the
    /// answer is in the same terms; for a find it is the view the person is
    /// looking at, so the answer is where in *that* the row is.
    pub fn step(&mut self, cells: &[Row], order: &[u32], exhausted: Exhausted<'_>) -> bool {
        let mut since_check = 0;
        while self.at < order.len() {
            let position = order[self.at] as usize;
            if let Some(row) = cells.get(position) {
                if self.rule.holds(row, position) {
                    self.hits.push(order[self.at]);
                    if self.first_only {
                        self.found = Some(self.at);
                        self.at = order.len();
                        return true;
                    }
                }
            }
            self.at += 1;
            since_check += 1;
            if since_check >= CHECK_EVERY {
                since_check = 0;
                if exhausted() {
                    return false;
                }
            }
        }
        true
    }

    /// Rows asked so far. What a progress bar reads.
    pub fn processed(&self) -> usize {
        self.at
    }

    pub fn into_hits(self) -> Vec<u32> {
        self.hits
    }

    /// Where in the order the first match was, once the pass is complete.
    pub fn found(&self) -> Option<usize> {
        self.found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::{Dataset, ROWS, ROW_BYTES};

    /// Rows whose cells are the given strings, padded to a cell each.
    fn rows(cells: &[&[&str]]) -> Vec<Row> {
        cells
            .iter()
            .map(|row| {
                let mut out = [b' '; ROW_BYTES];
                for (column, value) in row.iter().enumerate() {
                    let start = column * CELL;
                    for (slot, byte) in out[start..start + CELL].iter_mut().zip(value.bytes()) {
                        *slot = byte;
                    }
                }
                out
            })
            .collect()
    }

    fn identity(len: usize) -> Vec<u32> {
        (0..len as u32).collect()
    }

    fn run(scan: &mut Scan, cells: &[Row], order: &[u32], slice: usize) -> usize {
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
            if scan.step(cells, order, &mut exhausted) {
                return slices;
            }
            assert!(slices < 10_000, "a scan that never finishes");
        }
    }

    #[test]
    fn a_filter_finds_the_rows_with_the_text_in_any_column() {
        let cells = rows(&[&["ABC", "XYZ"], &["QQQ", "PBCP"], &["AAA", "AAA"]]);
        let order = identity(cells.len());
        let mut scan = Scan::filter(Rule::text("BC"));
        run(&mut scan, &cells, &order, usize::MAX);
        assert_eq!(scan.into_hits(), vec![0, 1]);
    }

    #[test]
    fn a_search_does_not_run_off_the_end_of_a_cell_into_the_next_one() {
        // "ABC" then "DEF" are sixteen bytes apart, not three: a search over
        // the whole row would find "CD" between them, and there is no "CD" in
        // this row.
        let cells = rows(&[&["ABC", "DEF"]]);
        let order = identity(1);
        let mut wrong = Scan::filter(Rule::text("CD"));
        run(&mut wrong, &cells, &order, usize::MAX);
        assert!(wrong.into_hits().is_empty());
        // And the padding is not a match either.
        let mut spaces = Scan::filter(Rule::text("C   D"));
        run(&mut spaces, &cells, &order, usize::MAX);
        assert!(spaces.into_hits().is_empty());
    }

    #[test]
    fn a_search_longer_than_a_cell_matches_nothing_and_an_empty_one_everything() {
        let cells = rows(&[&["ABC"], &["DEF"]]);
        let order = identity(2);
        let mut long = Scan::filter(Rule::text(&"A".repeat(CELL + 1)));
        run(&mut long, &cells, &order, usize::MAX);
        assert!(long.into_hits().is_empty());
        let mut empty = Scan::filter(Rule::text(""));
        run(&mut empty, &cells, &order, usize::MAX);
        assert_eq!(empty.into_hits(), vec![0, 1]);
    }

    #[test]
    fn the_search_is_case_sensitive() {
        let cells = rows(&[&["ABC"], &["abc"]]);
        let order = identity(2);
        let mut upper = Scan::filter(Rule::text("ABC"));
        run(&mut upper, &cells, &order, usize::MAX);
        assert_eq!(upper.into_hits(), vec![0]);
    }

    #[test]
    fn a_group_filter_asks_where_a_row_is_rather_than_what_is_in_it() {
        // Positions, so the cells can all be the same and the answer is still
        // the two the tree names.
        let cells = vec![[b'A'; ROW_BYTES]; 3];
        let order = vec![0u32, 1_000, 10_000];
        let mut first = Scan::filter(Rule::Group(0, 0));
        run(&mut first, &cells, &order, usize::MAX);
        // Only position 0 is in group one, part one; 1,000 is part two and
        // 10,000 is a different group entirely.
        assert_eq!(first.into_hits(), vec![0]);
    }

    #[test]
    fn a_find_stops_at_the_first_one_and_says_where_in_the_order_it_was() {
        let cells = rows(&[&["AAA"], &["ZZZ"], &["ZZZ"]]);
        // A view that is not the identity: the answer is a place in the view,
        // because that is what a person is being scrolled to.
        let order = vec![2u32, 1, 0];
        let mut scan = Scan::find(Rule::text("ZZZ"));
        run(&mut scan, &cells, &order, usize::MAX);
        assert_eq!(scan.found(), Some(0));
        assert_eq!(scan.processed(), 3);
        assert_eq!(scan.into_hits(), vec![2]);

        let mut missing = Scan::find(Rule::text("QQQ"));
        run(&mut missing, &cells, &order, usize::MAX);
        assert_eq!(missing.found(), None);
        assert!(missing.into_hits().is_empty());
    }

    #[test]
    fn a_scan_in_slices_answers_what_a_scan_in_one_would() {
        let data = Dataset::generate();
        let cells = &data.rows()[..8_192];
        let order = identity(cells.len());
        let mut whole = Scan::filter(Rule::text("QQ"));
        run(&mut whole, cells, &order, usize::MAX);
        let mut sliced = Scan::filter(Rule::text("QQ"));
        let slices = run(&mut sliced, cells, &order, 1);
        assert!(slices > 4, "an eight thousand row scan in {slices} slices");
        assert_eq!(whole.into_hits(), sliced.into_hits());
    }

    #[test]
    fn progress_is_rows_asked_and_ends_at_the_order_it_was_given() {
        let data = Dataset::generate();
        let order = identity(ROWS);
        let mut scan = Scan::filter(Rule::text("ZZZZ"));
        run(&mut scan, data.rows(), &order, 3);
        assert_eq!(scan.processed(), ROWS);
    }
}
