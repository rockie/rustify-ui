//! Browser facade for the target-independent style core.

#[cfg(feature = "cn")]
mod cn;
#[cfg(feature = "cn")]
mod cn_build;
#[cfg(feature = "cn")]
pub use cn::*;
#[cfg(feature = "cn")]
pub use cn_build::*;

#[cfg(any(feature = "baseline", feature = "scanner"))]
use wasm_bindgen::prelude::*;

#[cfg(feature = "compiler")]
mod session;
#[cfg(feature = "compiler")]
pub use session::CompilerSession;

#[cfg(all(feature = "compiler-api", any(target_arch = "wasm32", test)))]
mod public_api;
#[cfg(all(feature = "compiler-api", any(target_arch = "wasm32", test)))]
pub use public_api::{compile_ast, AstCompiler, CssCompiler, DesignSystem};

/// Exercises candidate transport without loading the scanner or a compiler.
#[cfg(feature = "baseline")]
#[wasm_bindgen]
pub fn transport_probe(input: &str) -> Result<String, JsValue> {
    let candidates: Vec<String> =
        serde_json::from_str(input).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&candidates).map_err(|error| JsValue::from_str(&error.to_string()))
}

/// Extracts complete candidates and CSS variables from in-memory source text.
#[cfg(feature = "scanner")]
#[wasm_bindgen]
pub fn scan_content(content: &str, extension: &str) -> Result<String, JsValue> {
    let result = rustify_style::scanner::scan(content, extension);
    serde_json::to_string(&serde_json::json!({
        "candidates": result.candidates,
        "variables": result.variables,
    }))
    .map_err(|error| JsValue::from_str(&error.to_string()))
}
