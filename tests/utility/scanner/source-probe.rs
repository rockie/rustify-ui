//! Plain-WASM execution of the fixed source selection and incremental corpus.
use rustify_style::scanner::*;
#[path = "source-contracts.rs"]
mod source_contracts;
#[no_mangle]
pub extern "C" fn run_source_fixtures() -> u32 {
    source_contracts::run().len() as u32
}
