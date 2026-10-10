//! Names the GPU API without asking for it. This must not compile: a DOM
//! application that reaches for a region is told so by the compiler, and
//! links no Makepad either way.

pub fn first_region() -> (Option<rustify_ui::RegionId>, rustify_ui::RegionState) {
    (None, rustify_ui::RegionState::Starting)
}

pub use rustify_ui::GpuRegion;
