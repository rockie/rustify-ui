#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--starter") {
        println!("{}", vellum::starter::make_starter()?.serialize()?);
    } else {
        eprintln!("Build the browser example with mbx xtask build-web --example vellum --release.");
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
pub use vellum::{
    affine, commands, document, gesture, history, hit, layout, raster_policy, scene, starter,
    svg_export, text_layout,
};

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod assets;
#[cfg(target_arch = "wasm32")]
mod automation;
#[cfg(target_arch = "wasm32")]
mod browser_frame;
#[cfg(target_arch = "wasm32")]
mod fileio;
#[cfg(target_arch = "wasm32")]
mod fonts;
#[cfg(target_arch = "wasm32")]
mod icons;
#[cfg(target_arch = "wasm32")]
mod keys;
#[cfg(target_arch = "wasm32")]
mod overlay;
#[cfg(target_arch = "wasm32")]
mod painter;
#[cfg(target_arch = "wasm32")]
mod pointer;
#[cfg(target_arch = "wasm32")]
mod raster;
#[cfg(target_arch = "wasm32")]
mod region;
pub mod shaders;
#[cfg(target_arch = "wasm32")]
mod shell;
#[cfg(target_arch = "wasm32")]
mod storage;
