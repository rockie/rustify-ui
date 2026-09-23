use crate::makepad_shell::*;
use crate::makepad_wasm_strip::*;
use crate::utils::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Default)]
pub struct WasmConfig {
    pub strip: bool,
    pub optimize_size: bool,
}

const WASM_TARGET_TRIPLE: &str = "wasm32-unknown-unknown";
/// Single-threaded browser build: no shared memory, so plain static hosting
/// without cross-origin isolation works.
const WASM_RUSTFLAGS: &str = "-C codegen-units=1 -C debuginfo=0 -C link-arg=--export=__stack_pointer -C link-arg=--compress-relocations -C link-arg=--strip-debug -C opt-level=z";

fn format_section_counts(summary: &WasmSectionSummary) -> String {
    if summary.counts.is_empty() {
        return "none".to_string();
    }
    summary
        .counts
        .iter()
        .map(|(name, count)| format!("{name}x{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_wasm_size_report(report: &WasmSizeReport) {
    println!("Wasm size report:");
    println!("  original:  {} bytes", report.original_bytes);
    println!("  stripped:  {} bytes", report.stripped_bytes);
    println!("  optimized: {} bytes", report.optimized_bytes);
    println!(
        "  debug sections removed:  {} bytes ({})",
        report.debug_sections.total_bytes,
        format_section_counts(&report.debug_sections)
    );
    println!(
        "  custom sections removed: {} bytes ({})",
        report.custom_sections.total_bytes,
        format_section_counts(&report.custom_sections)
    );
}

fn minify_js(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut string_char = '\0';
    let mut in_regex = false;

    while let Some(c) = chars.next() {
        if in_string {
            out.push(c);
            if c == '\\' {
                if let Some(next_c) = chars.next() {
                    out.push(next_c);
                }
            } else if c == string_char {
                in_string = false;
            }
        } else if in_regex {
            out.push(c);
            if c == '\\' {
                if let Some(next_c) = chars.next() {
                    out.push(next_c);
                }
            } else if c == '/' {
                in_regex = false;
            }
        } else {
            match c {
                '\'' | '"' | '`' => {
                    in_string = true;
                    string_char = c;
                    out.push(c);
                }
                '/' => match chars.peek() {
                    Some(&'/') => {
                        while let Some(&next_c) = chars.peek() {
                            if next_c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    }
                    Some(&'*') => {
                        chars.next();
                        while let Some(next_c) = chars.next() {
                            if next_c == '*' {
                                if let Some(&'/') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        out.push(c);
                        // A slash after a value-like character divides; anything
                        // else starts a regex literal. Heuristic, like the source.
                        if let Some(last_c) = out.trim_end().chars().last() {
                            if "(,=:[!&|?<>~;{+*-".contains(last_c) {
                                in_regex = true;
                            }
                        }
                    }
                },
                ' ' | '\t' | '\r' => {
                    if out.ends_with(|c: char| c.is_alphanumeric() || c == '_' || c == '$') {
                        if let Some(&next_c) = chars.peek() {
                            if next_c.is_alphanumeric() || next_c == '_' || next_c == '$' {
                                out.push(' ');
                            }
                        }
                    }
                }
                '\n' => {
                    out.push('\n');
                    while let Some(&next_c) = chars.peek() {
                        if next_c == ' ' || next_c == '\t' || next_c == '\r' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                _ => out.push(c),
            }
        }
    }

    out.lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn cp_minified(source_path: &Path, dest_path: &Path) -> Result<(), String> {
    if let Some(parent) = dest_path.parent() {
        mkdir(parent)?;
    }
    if source_path.extension().and_then(|s| s.to_str()) == Some("js") {
        let content = fs::read_to_string(source_path)
            .map_err(|e| format!("cannot read {}: {e}", source_path.display()))?;
        fs::write(dest_path, minify_js(&content))
            .map_err(|e| format!("cannot write {}: {e}", dest_path.display()))
    } else {
        cp(source_path, dest_path, false)
    }
}

/// wasm-bindgen runs in-process so the glue generator is locked to the exact
/// crate version in the lockfile; a schema mismatch fails here instead of at
/// runtime.
fn run_wasm_bindgen(wasm_path: &Path, out_dir: &Path) -> Result<(), String> {
    let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
    bindgen
        .input_path(wasm_path)
        .web(true)
        .map_err(|e| format!("wasm-bindgen: {e}"))?
        .typescript(false)
        .out_name("bindgen")
        .generate(out_dir)
        .map_err(|e| format!("wasm-bindgen failed on {}: {e:#}", wasm_path.display()))
}

pub fn build(config: WasmConfig, args: &[String]) -> Result<PathBuf, String> {
    let build_crate = get_build_crate_from_args(args)?;
    let cwd = std::env::current_dir().unwrap();

    let mut args_out = vec![
        "build".to_string(),
        format!("--target={WASM_TARGET_TRIPLE}"),
    ];
    let profile = get_profile_from_args(args);
    args_out.extend(args.iter().cloned());
    let args_out_refs: Vec<&str> = args_out.iter().map(|arg| arg.as_str()).collect();

    let inherited_rustflags = std::env::var("RUSTFLAGS").ok();
    let rustflags =
        super::rustflags::compose_rustflags(WASM_RUSTFLAGS, inherited_rustflags.as_deref());
    // mbx rather than `$CARGO`: a program started by `mbx run` gets the plain
    // toolchain Cargo and none of mbx's wrapping, so a nested build has to ask
    // for the cache by name.
    //
    // Learned incremental is off for this build. mbx compiles a freshly edited
    // workspace crate with private incremental state, and that comes out
    // differently from a clean compile: the same fusion-basic sources gave
    // 8,673,018 wasm bytes that way and 8,671,140 without. A product must not
    // depend on which crates were edited since the last build. The switch is
    // set here, on the build itself, because one given to an outer mbx does
    // not reach this one while that outer session is running.
    shell_env(
        &[
            ("RUSTFLAGS", rustflags.as_str()),
            ("MBX_LEARNED_INCREMENTAL", "0"),
        ],
        &cwd,
        "mbx",
        &args_out_refs,
    )?;

    let app_dir = cwd.join(format!("target/makepad-wasm-app/{profile}/{}", build_crate));
    let build_dir = cwd.join(format!("target/{WASM_TARGET_TRIPLE}/{profile}"));

    let build_crate_dir = get_crate_dir(build_crate)?;
    let local_resources_path = build_crate_dir.join("resources");
    if local_resources_path.is_dir() {
        let underscore_build_crate = build_crate.replace('-', "_");
        let dst_dir = app_dir.join(underscore_build_crate).join("resources");
        mkdir(&dst_dir)?;
        walk_all(&local_resources_path, &dst_dir, &mut |source_path, dest_dir| {
            let source_file_name = source_path
                .file_name()
                .ok_or_else(|| format!("Unable to get filename for {:?}", source_path))?
                .to_string_lossy()
                .to_string();
            cp(source_path, &dest_dir.join(&source_file_name), false)
        })?;
    }

    let resources = get_crate_dep_dirs(build_crate, &build_dir, WASM_TARGET_TRIPLE);
    for (name, dep_dir) in resources.iter() {
        if name == "makepad-wasm-bridge" {
            cp_minified(
                &dep_dir.join("src/wasm_bridge.js"),
                &app_dir.join("makepad_wasm_bridge/wasm_bridge.js"),
            )?;
        }
        if name == "makepad-platform" {
            for file in ["web_gl.js", "web.js"] {
                cp_minified(
                    &dep_dir.join("src/os/web").join(file),
                    &app_dir.join("makepad_platform").join(file),
                )?;
            }
        }
        let name = name.replace('-', "_");
        let resources_path = dep_dir.join("resources");
        if resources_path.is_dir() {
            let dst_dir = app_dir.join(&name).join("resources");
            mkdir(&dst_dir)?;
            walk_all(&resources_path, &dst_dir, &mut |source_path, dest_dir| {
                let source_file_name = source_path
                    .file_name()
                    .ok_or_else(|| format!("Unable to get filename for {:?}", source_path))?
                    .to_string_lossy()
                    .to_string();
                cp(source_path, &dest_dir.join(&source_file_name), false)
            })?;
        }
    }

    run_wasm_bindgen(&build_dir.join(format!("{build_crate}.wasm")), &build_dir)?;
    let jsfile = build_dir.join("bindgen.js");
    let glue = fs::read_to_string(&jsfile)
        .map_err(|e| format!("Unable to find wasm-bindgen generated file {e:?}"))?;
    let patched = super::bindgen_glue::patch_bindgen_glue(&glue)?;
    fs::write(&jsfile, patched).map_err(|e| format!("cannot write {}: {e}", jsfile.display()))?;
    cp_minified(&jsfile, &app_dir.join("bindgen.js"))?;

    let wasm_source = build_dir.join("bindgen_bg.wasm");
    let wasm_dest = app_dir.join(format!("{}.wasm", build_crate));
    let data =
        fs::read(&wasm_source).map_err(|_| format!("Cannot read wasm file {:?}", wasm_source))?;
    let output = if config.optimize_size {
        let report =
            wasm_size_report(&data).map_err(|_| format!("Cannot parse wasm {:?}", wasm_source))?;
        print_wasm_size_report(&report);
        wasm_optimize_size(&data).map_err(|_| format!("Cannot parse wasm {:?}", wasm_source))?
    } else if config.strip {
        wasm_strip_custom_sections(&data)
            .map_err(|_| format!("Cannot parse wasm {:?}", wasm_source))?
    } else {
        data
    };
    fs::write(&wasm_dest, output)
        .map_err(|e| format!("Can't write file {:?} {:?} ", wasm_dest, e))?;

    println!("Created wasm package: {:?}", app_dir);
    Ok(app_dir)
}
