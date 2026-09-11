//! A table of a hundred thousand rows, of which it draws sixty.
//!
//! The table holds no data. It is given how many rows there are and a way to
//! read a cell, and it decides which cells that means right now; the
//! application keeps the rows, the order they are in and what is selected.
//! That division is what lets the same table show a sorted view, a filtered
//! view and an edited row without the table knowing any of those words.
//!
//! The two parts that go wrong silently have their own modules and their own
//! tests: which rows are on screen (`window`) and what a key press means
//! (`keys`).

pub mod keys;
#[cfg(target_arch = "wasm32")]
mod view;
pub mod window;

pub use keys::{Cell, Command, Shape};
#[cfg(target_arch = "wasm32")]
pub use view::{CellText, Column, DataTable, RowId};
pub use window::Range;
