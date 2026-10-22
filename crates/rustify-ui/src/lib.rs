//! Rustify UI: the SDK for Leptos DOM applications that also draw GPU regions.
//!
//! The DOM half lives in `rustify-dom`, which links no Makepad at all. This
//! crate is the other half - regions, the bridge that owns them, and the GPU
//! halves of the controls that have one on both sides of the boundary.
//!
//! It re-exports the core under the same paths, so `rustify_ui::Theme` *is*
//! `rustify_dom::Theme`: one type, one identity, whichever side of the boundary
//! it was named from. A region and a DOM scope handed to each other exchange
//! real values, not look-alikes.

#[cfg(all(feature = "csr", feature = "hydrate"))]
compile_error!(
    "csr and hydrate are separate rendering modes; disable default features for hydrate"
);

pub use rustify_dom::*;

// These three shadow the core's modules of the same name: each is the core's
// module plus what only a GPU host can add. An explicit item beats a glob
// import of the same name, so `rustify_ui::theme` is the one declared here.
pub mod diagnostics;
pub mod theme;
#[cfg(feature = "utility")]
pub mod utility;

#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub mod gpu;
#[cfg(feature = "gpu")]
pub mod region;

#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub use gpu::{
    Glyph as RegionGlyph, RustifyButton, RustifyCheckBox, RustifyDropDown, RustifyIcon,
    RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar, RustifyToggle,
};
#[cfg(feature = "gpu")]
pub use region::{GpuRegion, RegionState};
#[cfg(feature = "gpu")]
pub use rustify_makepad::RegionId;
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub use rustify_makepad::{makepad_widgets, RegionApp};
