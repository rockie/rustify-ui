//! Embeds the same fixed corpus used by native tests in a plain WASM module.

use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    content: String,
    extension: String,
    candidates: Vec<String>,
    variables: Vec<String>,
}

/// Return the number of mismatching scanner cases, or `u32::MAX` for a bad corpus.
#[no_mangle]
pub extern "C" fn run_scanner_fixtures() -> u32 {
    let Ok(fixtures) = serde_json::from_str::<Vec<Fixture>>(include_str!(
        "../../../tests/utility/scanner/fixtures.json"
    )) else {
        return u32::MAX;
    };
    fixtures
        .into_iter()
        .filter(|case| {
            let result = rustify_style::scanner::scan(&case.content, &case.extension);
            result.candidates != case.candidates || result.variables != case.variables
        })
        .count() as u32
}
