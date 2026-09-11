//! What a key press means in a grid.
//!
//! Separated from the view because it is the part that has to hold at the
//! edges: the arrows must not walk off the table, Home and End mean two
//! different things depending on a modifier, and a page is a screenful of the
//! viewport rather than a number somebody chose.

use super::window;

/// Where the focus is: a cell, by row and column.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
}

/// What a key does to the focused cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Move to this cell.
    Move(Cell),
    /// Select or deselect the focused row.
    Toggle,
    /// Open the focused row for editing.
    Activate,
}

/// The table as a key press has to see it.
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    pub rows: usize,
    pub columns: usize,
    /// Height of the scrolling viewport, in CSS pixels.
    pub viewport: f64,
    pub row_height: f64,
}

/// What `key` does from `at`, or `None` when the grid does not claim it.
///
/// A key the grid does not claim has to reach the page: a table that consumed
/// every key press would be a table nobody could tab out of.
pub fn command(key: &str, ctrl: bool, at: Cell, shape: Shape) -> Option<Command> {
    if shape.rows == 0 || shape.columns == 0 {
        return None;
    }
    let last_row = shape.rows - 1;
    let last_column = shape.columns - 1;
    let page = window::page(shape.viewport, shape.row_height);
    let moved = |row: usize, column: usize| {
        Some(Command::Move(Cell {
            row: row.min(last_row),
            column: column.min(last_column),
        }))
    };
    match key {
        "ArrowDown" => moved(at.row.saturating_add(1), at.column),
        "ArrowUp" => moved(at.row.saturating_sub(1), at.column),
        "ArrowRight" => moved(at.row, at.column.saturating_add(1)),
        "ArrowLeft" => moved(at.row, at.column.saturating_sub(1)),
        "PageDown" => moved(at.row.saturating_add(page), at.column),
        "PageUp" => moved(at.row.saturating_sub(page), at.column),
        // Home and End are about the row; with a modifier they are about the
        // table. Both are what a person expects from every other grid.
        "Home" if ctrl => moved(0, 0),
        "End" if ctrl => moved(last_row, last_column),
        "Home" => moved(at.row, 0),
        "End" => moved(at.row, last_column),
        " " => Some(Command::Toggle),
        "Enter" | "F2" => Some(Command::Activate),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHAPE: Shape = Shape {
        rows: 100_000,
        columns: 20,
        viewport: 1_440.0,
        row_height: 24.0,
    };

    fn at(row: usize, column: usize) -> Cell {
        Cell { row, column }
    }

    fn moved(key: &str, ctrl: bool, from: Cell) -> Cell {
        match command(key, ctrl, from, SHAPE) {
            Some(Command::Move(cell)) => cell,
            other => panic!("{key} gave {other:?}"),
        }
    }

    #[test]
    fn the_arrows_move_one_cell() {
        assert_eq!(moved("ArrowDown", false, at(5, 5)), at(6, 5));
        assert_eq!(moved("ArrowUp", false, at(5, 5)), at(4, 5));
        assert_eq!(moved("ArrowRight", false, at(5, 5)), at(5, 6));
        assert_eq!(moved("ArrowLeft", false, at(5, 5)), at(5, 4));
    }

    #[test]
    fn the_arrows_stop_at_the_edges_rather_than_walking_off() {
        assert_eq!(moved("ArrowUp", false, at(0, 0)), at(0, 0));
        assert_eq!(moved("ArrowLeft", false, at(0, 0)), at(0, 0));
        assert_eq!(moved("ArrowDown", false, at(99_999, 19)), at(99_999, 19));
        assert_eq!(moved("ArrowRight", false, at(99_999, 19)), at(99_999, 19));
    }

    #[test]
    fn a_page_is_a_screenful_and_stops_at_the_ends() {
        assert_eq!(moved("PageDown", false, at(0, 3)), at(60, 3));
        assert_eq!(moved("PageUp", false, at(100, 3)), at(40, 3));
        assert_eq!(moved("PageUp", false, at(5, 3)), at(0, 3));
        assert_eq!(moved("PageDown", false, at(99_990, 3)), at(99_999, 3));
    }

    #[test]
    fn home_and_end_are_about_the_row_until_a_modifier_makes_them_about_the_table() {
        assert_eq!(moved("Home", false, at(7, 9)), at(7, 0));
        assert_eq!(moved("End", false, at(7, 9)), at(7, 19));
        assert_eq!(moved("Home", true, at(7, 9)), at(0, 0));
        assert_eq!(moved("End", true, at(7, 9)), at(99_999, 19));
    }

    #[test]
    fn space_selects_and_enter_opens() {
        assert_eq!(command(" ", false, at(1, 1), SHAPE), Some(Command::Toggle));
        assert_eq!(
            command("Enter", false, at(1, 1), SHAPE),
            Some(Command::Activate)
        );
        assert_eq!(
            command("F2", false, at(1, 1), SHAPE),
            Some(Command::Activate)
        );
    }

    #[test]
    fn a_key_the_grid_does_not_claim_is_left_to_the_page() {
        for key in ["Tab", "Escape", "a", "ArrowDownLeft"] {
            assert_eq!(command(key, false, at(1, 1), SHAPE), None, "{key}");
        }
    }

    #[test]
    fn an_empty_table_claims_nothing() {
        let empty = Shape { rows: 0, ..SHAPE };
        assert_eq!(command("ArrowDown", false, at(0, 0), empty), None);
        let no_columns = Shape {
            columns: 0,
            ..SHAPE
        };
        assert_eq!(command("ArrowDown", false, at(0, 0), no_columns), None);
    }
}
