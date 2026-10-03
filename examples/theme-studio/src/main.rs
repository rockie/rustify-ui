#[cfg(target_arch = "wasm32")]
mod app;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod color_editor;
#[cfg(target_arch = "wasm32")]
mod controller;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod data_tools;
#[cfg(target_arch = "wasm32")]
mod gpu_preview;
#[cfg(target_arch = "wasm32")]
mod inspector;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod persistence;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod presets;
#[cfg(target_arch = "wasm32")]
mod preview_region;
#[cfg(target_arch = "wasm32")]
mod provider_fixture;
#[cfg(target_arch = "wasm32")]
mod scenes;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod state;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod theme_controls;

fn main() {}
