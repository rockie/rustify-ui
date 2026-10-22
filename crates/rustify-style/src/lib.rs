//! Pure Rust class tools, content extraction, and utility compilation.

#[cfg(feature = "cn")]
pub mod cn;

#[cfg(feature = "compiler")]
pub mod compiler;

#[cfg(feature = "protocol")]
pub mod program;
#[cfg(feature = "protocol")]
pub mod protocol;
#[cfg(feature = "protocol")]
pub mod text;

#[cfg(feature = "compiler")]
pub mod scope;

#[cfg(feature = "scanner")]
pub mod scanner;

// Keep upstream module paths unchanged so its state machines and tests remain exact copies.
#[cfg(feature = "scanner")]
#[allow(dead_code, unused_parens, clippy::all)]
#[rustfmt::skip]
#[path = "../upstream/oxide/cursor.rs"]
mod cursor;
#[cfg(feature = "scanner")]
#[allow(dead_code, unused_parens, clippy::all)]
#[rustfmt::skip]
#[path = "../upstream/oxide/extractor/mod.rs"]
mod extractor;
#[cfg(feature = "scanner")]
#[allow(dead_code, unused_parens, clippy::all)]
#[rustfmt::skip]
#[path = "../upstream/oxide/fast_skip.rs"]
mod fast_skip;
#[cfg(all(test, feature = "scanner"))]
#[allow(dead_code, unused_parens, clippy::all)]
#[rustfmt::skip]
#[path = "../upstream/oxide/throughput.rs"]
mod throughput;

#[cfg(all(test, rustify_repo_tests))]
mod repo_tests;
