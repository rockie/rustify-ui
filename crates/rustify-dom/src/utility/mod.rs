//! Scope-owned runtime utility compilation with an independent Worker.

mod cn;
mod state;
pub use cn::CnContext;
pub use rustify_style::protocol::{SourceContent, SourceSnapshot, VirtualSourceEntry};
pub use state::{CommittedSnapshot, Outcome, UtilityConfig, UtilityError};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub use browser::UtilityScope;
