//! Names the GPU API with the `gpu` feature on: this has to compile.

/// A region's identity and its lifecycle, which only exist with GPU regions.
pub fn first_region() -> (Option<rustify_ui::RegionId>, rustify_ui::RegionState) {
    (None, rustify_ui::RegionState::Starting)
}

#[cfg(target_arch = "wasm32")]
pub use rustify_ui::{makepad_widgets, GpuRegion, RegionApp};
