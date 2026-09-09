mod bridge;
mod build;
mod doctor;
mod serve;
mod sources;

use build::BuildRequest;
use serve::{CspMode, ServeConfig};

const USAGE: &str = "\
cargo xtask <command> [options]

  doctor
  build-web --example <name> [--release]
  serve       --example <name> [--release] [--base /path/] [--port N] [--csp strict|no-wasm|off]
  report-size --example <name> [--release]
  sources     verify
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("doctor") => doctor::run(),
        Some("build-web") => build_web(&args[1..]).map(|_| ()),
        Some("serve") => serve_example(&args[1..]),
        Some("report-size") => report_size(&args[1..]),
        Some("sources") => sources::run(&args[1..]),
        _ => Err(USAGE.to_string()),
    };
    if let Err(message) = result {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn option<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn build_request(args: &[String]) -> Result<BuildRequest, String> {
    Ok(BuildRequest {
        example: option(args, "--example")
            .ok_or("--example <name> is required")?
            .to_string(),
        release: args.iter().any(|a| a == "--release"),
    })
}

fn build_web(args: &[String]) -> Result<std::path::PathBuf, String> {
    build::build(&build_request(args)?)
}

/// Every byte of a built example, in the six categories the plan reports, read
/// off the produced directory rather than off the build that made it: a report
/// that describes what a build meant to write is not a report of what a
/// deployment will serve.
fn report_size(args: &[String]) -> Result<(), String> {
    let request = build_request(args)?;
    let root = build::app_dir(&build::repo_root(), &request);
    if !root.join("index.html").is_file() {
        return Err(format!(
            "{} has no build; run `cargo xtask build-web --example {}{}` first",
            root.display(),
            request.example,
            if request.release { " --release" } else { "" }
        ));
    }
    let files = build::list_files(&root)?;
    let report = build::size_report(&files);
    println!("{}", root.display());
    println!("{}", build::format_size_report(&report));
    println!("{} files", files.len());
    for (name, size) in &files {
        println!("  {size:>12}  {name}");
    }
    Ok(())
}

fn serve_example(args: &[String]) -> Result<(), String> {
    let request = build_request(args)?;
    let root = build::app_dir(&build::repo_root(), &request);
    if !root.join("index.html").is_file() {
        return Err(format!(
            "{} has no build; run `cargo xtask build-web --example {}{}` first",
            root.display(),
            request.example,
            if request.release { " --release" } else { "" }
        ));
    }
    let port = match option(args, "--port") {
        Some(value) => value.parse().map_err(|_| format!("invalid port {value}"))?,
        None => 0,
    };
    serve::serve(ServeConfig {
        root,
        base: serve::normalize_base(option(args, "--base").unwrap_or("/")),
        port,
        csp: CspMode::parse(option(args, "--csp").unwrap_or("strict"))?,
    })
}
