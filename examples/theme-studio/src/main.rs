#[cfg(target_arch = "wasm32")]
mod app;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod color_editor;
#[cfg(target_arch = "wasm32")]
mod controller;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod data_tools;
#[cfg(target_arch = "wasm32")]
mod form_samples;
#[cfg(target_arch = "wasm32")]
mod form_states;
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
#[cfg(target_arch = "wasm32")]
mod storage_browser;
#[cfg(target_arch = "wasm32")]
mod storage_controls;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod theme_controls;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod write_queue;

fn main() {}
