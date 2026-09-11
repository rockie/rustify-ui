//! Which rows and columns a scrolled grid is showing.
//!
//! A windowed table is arithmetic and a pool of elements. The elements are
//! easy; the arithmetic is where a table goes wrong in ways nobody sees until
//! somebody scrolls to the end and finds blank rows, or jumps to row 100,000
//! and lands on 99,940. So it lives here, on its own, with the four cases that
//! break it written down: the start, the end, a table shorter than its own
//! viewport, and a jump to a row that is already on screen.

/// Rows drawn above and below the viewport, so a scroll of a few pixels does
/// not need new elements to appear before it can be shown.
pub const OVERSCAN: usize = 10;

/// The most rows the window will ever ask for, whatever the viewport says.
///
/// A windowed table stands on one assumption: its scroller has a height of its
/// own. Give it a container that grows to fit its contents and the assumption
/// inverts - the scroller becomes as tall as the spacer, the viewport becomes
/// as tall as the table, and the window asks for every row there is. At a
/// hundred thousand rows that is not a slow page, it is a dead one: the module
/// runs out of room for the JavaScript values it has to hold and aborts.
///
/// So the window has a ceiling. A viewport that wants more rows than this is a
/// viewport that has not been given a height, and drawing two hundred rows
/// into it is visibly wrong in a way that drawing all hundred thousand is not.
pub const MAX_ROWS: usize = 200;
/// The same, across.
pub const OVERSCAN_COLUMNS: usize = 2;

/// A half-open range of indices, `start..end`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    pub fn contains(&self, index: usize) -> bool {
        index >= self.start && index < self.end
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.start..self.end
    }
}

/// The rows to draw for a viewport `height` tall scrolled to `scroll_top`,
/// with `overscan` rows of margin either side.
///
/// The range is clamped to the table rather than to the scroll position: a
/// viewport scrolled past the end - which happens for a frame whenever rows
/// are deleted - shows the last rows rather than nothing.
pub fn rows(scroll_top: f64, height: f64, row_height: f64, count: usize, overscan: usize) -> Range {
    if count == 0 || row_height <= 0.0 || height <= 0.0 {
        return Range::default();
    }
    let visible = ((height / row_height).ceil() as usize + 1).min(MAX_ROWS);
    let first = (scroll_top.max(0.0) / row_height).floor() as usize;
    let start = first.saturating_sub(overscan);
    let end = first
        .saturating_add(visible)
        .saturating_add(overscan)
        .min(count);
    // A window that ran off the end is pulled back rather than shortened, so
    // the pool of elements stays the size it was built at.
    let span = (visible + 2 * overscan).min(count);
    let start = start.min(end.saturating_sub(span));
    Range { start, end }
}

/// The same for columns, which are fixed width and far fewer.
pub fn columns(scroll_left: f64, width: f64, column_width: f64, count: usize) -> Range {
    rows(scroll_left, width, column_width, count, OVERSCAN_COLUMNS)
}

/// How far to scroll so that `row` is on screen.
///
/// `None` when it already is: scrolling to a row that is showing would move
/// the table under a person who asked for a row they were looking at, and
/// that is how a keyboard walk turns into a jitter.
pub fn scroll_to(
    row: usize,
    scroll_top: f64,
    height: f64,
    row_height: f64,
    count: usize,
) -> Option<f64> {
    if count == 0 || row_height <= 0.0 || height <= 0.0 {
        return None;
    }
    let row = row.min(count - 1);
    let top = row as f64 * row_height;
    let bottom = top + row_height;
    let limit = (count as f64 * row_height - height).max(0.0);
    if top >= scroll_top && bottom <= scroll_top + height {
        return None;
    }
    // Centred, which is what a jump wants: a row at the very top of the
    // viewport gives no clue what is around it.
    Some((top - (height - row_height) / 2.0).clamp(0.0, limit))
}

/// Rows a page up or down moves by: a screenful, and never fewer than one.
pub fn page(height: f64, row_height: f64) -> usize {
    if row_height <= 0.0 {
        return 1;
    }
    ((height / row_height).floor() as usize).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW: f64 = 24.0;
    const VIEWPORT: f64 = 1_440.0;
    const ROWS: usize = 100_000;

    #[test]
    fn the_top_of_the_table_has_no_margin_above_it() {
        let window = rows(0.0, VIEWPORT, ROW, ROWS, OVERSCAN);
        assert_eq!(window.start, 0);
        assert!(window.contains(0));
        // Sixty visible, one for the partial row, and the margin below.
        assert_eq!(window.len(), 61 + OVERSCAN);
    }

    #[test]
    fn scrolling_moves_the_window_by_whole_rows() {
        let window = rows(100.0 * ROW, VIEWPORT, ROW, ROWS, OVERSCAN);
        assert_eq!(window.start, 100 - OVERSCAN);
        assert!(window.contains(100));
        assert!(window.contains(159));
    }

    #[test]
    fn the_end_of_the_table_is_the_end_of_the_window() {
        let bottom = ROWS as f64 * ROW - VIEWPORT;
        let window = rows(bottom, VIEWPORT, ROW, ROWS, OVERSCAN);
        assert_eq!(window.end, ROWS);
        assert!(window.contains(ROWS - 1));
        // The pool does not shrink at the end: the same number of elements is
        // showing as anywhere else.
        assert_eq!(window.len(), 61 + 2 * OVERSCAN);
    }

    #[test]
    fn scrolled_past_the_end_shows_the_last_rows_rather_than_none() {
        let window = rows(ROWS as f64 * ROW * 2.0, VIEWPORT, ROW, ROWS, OVERSCAN);
        assert_eq!(window.end, ROWS);
        assert!(window.contains(ROWS - 1));
        assert!(!window.is_empty());
    }

    #[test]
    fn a_table_shorter_than_its_viewport_is_all_of_it() {
        let window = rows(0.0, VIEWPORT, ROW, 5, OVERSCAN);
        assert_eq!(window, Range { start: 0, end: 5 });
        assert_eq!(rows(0.0, VIEWPORT, ROW, 0, OVERSCAN), Range::default());
    }

    #[test]
    fn a_viewport_with_no_height_shows_nothing() {
        assert!(rows(0.0, 0.0, ROW, ROWS, OVERSCAN).is_empty());
        assert!(rows(0.0, VIEWPORT, 0.0, ROWS, OVERSCAN).is_empty());
    }

    #[test]
    fn jumping_to_a_row_already_on_screen_does_not_move_the_table() {
        assert_eq!(scroll_to(10, 0.0, VIEWPORT, ROW, ROWS), None);
        assert_eq!(scroll_to(0, 0.0, VIEWPORT, ROW, ROWS), None);
        // The last fully visible row at the top of the table.
        assert_eq!(scroll_to(59, 0.0, VIEWPORT, ROW, ROWS), None);
    }

    #[test]
    fn jumping_centres_the_row_and_stops_at_the_ends() {
        let middle = scroll_to(50_000, 0.0, VIEWPORT, ROW, ROWS).unwrap();
        let window = rows(middle, VIEWPORT, ROW, ROWS, OVERSCAN);
        assert!(window.contains(50_000));
        // Centred: about as many rows above it as below.
        let above = 50_000 - window.start;
        let below = window.end - 50_000;
        assert!(
            above.abs_diff(below) <= 2 * OVERSCAN + 2,
            "{above} vs {below}"
        );

        assert_eq!(scroll_to(0, 10_000.0, VIEWPORT, ROW, ROWS), Some(0.0));
        let last = scroll_to(ROWS - 1, 0.0, VIEWPORT, ROW, ROWS).unwrap();
        assert_eq!(last, ROWS as f64 * ROW - VIEWPORT);
        assert!(rows(last, VIEWPORT, ROW, ROWS, OVERSCAN).contains(ROWS - 1));
    }

    #[test]
    fn a_row_past_the_end_is_the_last_row() {
        assert_eq!(
            scroll_to(ROWS + 500, 0.0, VIEWPORT, ROW, ROWS),
            scroll_to(ROWS - 1, 0.0, VIEWPORT, ROW, ROWS)
        );
    }

    #[test]
    fn a_viewport_with_no_height_of_its_own_still_asks_for_a_window() {
        // A container that grew to fit its own spacer: the viewport is the
        // whole table. Without a ceiling the window would be every row there
        // is, and the pool built from it would take the page down.
        let window = rows(0.0, ROWS as f64 * ROW, ROW, ROWS, OVERSCAN);
        assert!(window.len() <= MAX_ROWS + 2 * OVERSCAN, "{}", window.len());
        assert!(window.contains(0));
    }

    #[test]
    fn a_page_is_a_screenful_of_whole_rows() {
        assert_eq!(page(VIEWPORT, ROW), 60);
        assert_eq!(page(10.0, ROW), 1);
        assert_eq!(page(VIEWPORT, 0.0), 1);
    }

    #[test]
    fn columns_window_the_same_way_with_a_smaller_margin() {
        let window = columns(0.0, 1_200.0, 120.0, 20);
        assert_eq!(window.start, 0);
        assert!(window.contains(0));
        assert!(window.end <= 20);
        let far = columns(20.0 * 120.0, 1_200.0, 120.0, 20);
        assert_eq!(far.end, 20);
    }
}
