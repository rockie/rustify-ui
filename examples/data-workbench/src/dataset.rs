//! The fixed sample the large-data views work on.
//!
//! It is generated rather than loaded: a hundred thousand rows of twenty
//! columns is thirty-two megabytes, and a file that size would make every
//! measurement a measurement of the network. The generator is arithmetic with
//! a fixed seed, so the same build always produces the same rows, and a second
//! implementation elsewhere can produce them too - which is what makes a
//! sorted or filtered result checkable against something other than the code
//! that produced it.

/// Rows in the sample. PRD load B2.
pub const ROWS: usize = 100_000;
/// Columns per row.
pub const COLUMNS: usize = 20;
/// Bytes per cell. Fixed width, so a cell is a slice rather than a string and
/// a row is one allocation rather than twenty.
pub const CELL: usize = 16;
/// Bytes per row.
pub const ROW_BYTES: usize = COLUMNS * CELL;

/// The characters a cell is made of: upper case and digits, one byte each, so
/// byte order and character order are the same thing.
pub const ALPHABET: &[u8; 36] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// xorshift32's starting state.
pub const SEED: u32 = 0x9E37_79B9;

pub type Row = [u8; ROW_BYTES];

/// Stable business identity. Assigned once, never reused, so a selection
/// survives sorting, filtering and the deletion of other rows.
pub type Id = u32;

/// xorshift32. Small, exactly specified, and reproducible in any language
/// with 32-bit integers - which is the only property that matters here.
#[derive(Clone, Copy, Debug)]
pub struct Random(u32);

impl Random {
    pub fn new() -> Self {
        Random(SEED)
    }

    pub fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

/// The sample, its identities and its version.
pub struct Dataset {
    ids: Vec<Id>,
    cells: Vec<Row>,
    version: u64,
}

impl Dataset {
    /// Generates the sample. Rows, then columns, then characters: the order
    /// the second implementation has to use as well.
    pub fn generate() -> Self {
        let mut random = Random::new();
        let mut cells = Vec::with_capacity(ROWS);
        for _ in 0..ROWS {
            let mut row = [0u8; ROW_BYTES];
            for byte in row.iter_mut() {
                *byte = ALPHABET[(random.next() % 36) as usize];
            }
            cells.push(row);
        }
        Dataset {
            ids: (1..=ROWS as Id).collect(),
            cells,
            version: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn id(&self, row: usize) -> Option<Id> {
        self.ids.get(row).copied()
    }

    /// Every write bumps this. Sorting, filtering and finding all read cell
    /// values, so any write at all makes a job that is still running stale -
    /// not only the writes that change which rows exist.
    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn rows(&self) -> &[Row] {
        &self.cells
    }

    pub fn cell(&self, row: usize, column: usize) -> Option<&[u8]> {
        let row = self.cells.get(row)?;
        let start = column.checked_mul(CELL)?;
        row.get(start..start + CELL)
    }

    /// FNV-1a over every cell byte in row order.
    ///
    /// One number that says whether two implementations produced the same
    /// thirty-two megabytes. It is a checksum, not a digest: nothing here is
    /// defended, and what it has to be is cheap and easy to write twice.
    pub fn hash(&self) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for row in &self.cells {
            for byte in row {
                hash ^= *byte as u64;
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_generator_is_the_published_sequence() {
        let mut random = Random::new();
        // The first four steps of xorshift32 from the seed, so a second
        // implementation can be checked against the sequence rather than
        // against a whole dataset.
        let mut x = SEED;
        for _ in 0..4 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            assert_eq!(random.next(), x);
        }
    }

    #[test]
    fn cells_are_alphabet_bytes() {
        let mut random = Random::new();
        let mut row = [0u8; ROW_BYTES];
        for byte in row.iter_mut() {
            *byte = ALPHABET[(random.next() % 36) as usize];
        }
        assert!(row.iter().all(|byte| ALPHABET.contains(byte)));
    }

    #[test]
    fn every_row_has_an_identity_of_its_own() {
        let data = Dataset::generate();
        assert_eq!(data.len(), ROWS);
        assert_eq!(data.id(0), Some(1));
        assert_eq!(data.id(ROWS - 1), Some(ROWS as Id));
        assert_eq!(data.id(ROWS), None);
        assert_eq!(data.version(), 0);
    }
}
