//! cargo-makepad, reduced to the single-threaded browser build this
//! repository ships. Invoked by `cargo xtask build-web`.

mod utils;
mod wasm;

pub use makepad_shell;
pub use makepad_wasm_strip;

use std::borrow::Cow;

fn show_help() {
    println!("Makepad's cargo extension (browser build only)");
    println!();
    println!("Usage: cargo makepad wasm [options] build <cargo args>");
    println!();
    println!("    build <cargo args>          Build a single-threaded wasm-bindgen package; pass -p <crate> [--release]");
    println!();
    println!("    [options]:");
    println!();
    println!("       --strip                  Shipping-size wasm optimization pass (implies custom-section stripping)");
    println!("       --strip-custom-sections  Only strip custom wasm sections");
}

fn main() -> Result<(), Cow<'static, str>> {
    let args: Vec<String> = std::env::args().collect();
    let args = if args.len() > 1
        && (args[0].ends_with("cargo-makepad") || args[0] == "cargo" || args[0].ends_with("cargo-makepad.exe"))
    {
        if args.len() > 2 && args[1] == "makepad" {
            args[2..].to_vec()
        } else {
            args[1..].to_vec()
        }
    } else {
        args
    };

    if args.is_empty() {
        show_help();
        return Err("not enough arguments; expected at least one command.".into());
    }
    let result = match args[0].as_ref() {
        "wasm" => wasm::handle_wasm(&args[1..]),
        unsupported => {
            show_help();
            Err(format!("unsupported command: '{unsupported}'"))
        }
    };
    result.map_err(Into::into)
}
