//! The `rustify-docs` command.

use rustify_docs::diagnostic_text::{code_span, literal};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use rustify_docs::content::{self, Diagnostic, FragmentCache, Mode, Project};
use rustify_docs::diagnostic::Severity;
use rustify_docs::generation;
use rustify_docs::highlight::Highlighter;
use rustify_docs::publish::{self, Area, LockError, Manifest, Recovery};

#[cfg(test)]
mod compile_proxy;
mod dev;
mod links;
mod new;
mod preview;
mod rendering;
mod report;
mod rust_site;
mod rust_source;
mod theme;

use report::{Format, Summary};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOURCE_REVISION: &str = env!("RUSTIFY_DOCS_SOURCE_REVISION");
const EXPECTED_FRAMEWORK_REV: &str = include_str!("../assets/framework-revision.txt");
/// The latest published release. A binary built from this source ships
/// after it, so this is where help and new sites roll back to; it moves on
/// before each release is paired.
const PREVIOUS_REVISION: &str = "88566ba1ed08223a5834f794840fff5e48f4ba8a";
const PREVIOUS_VERSION: &str = "0.1.32";

/// Whether this binary unwinds on a panic. `dev` keeps serving the last good
/// generation after a panic in a new one, which only an unwinding build can;
/// the install command's `--profile docs-cli` is what selects it.
const UNWINDS: bool = cfg!(panic = "unwind");

/// How tests make a command fail where it otherwise would not:
/// `panic:<marker>` makes a `dev` build panic on a page containing
/// `<marker>`; `stop:<step>` halts `build` at that step of publishing (see
/// `publish::Step`) until it is killed. Not part of the command's interface.
const FAULT_VARIABLE: &str = "RUSTIFY_DOCS_FAULT";

/// How to install this command so that it unwinds.
fn install() -> String {
    format!(
        "cargo install --git https://github.com/rockie/rustify-ui --rev {} --locked --profile docs-cli rustify-docs-cli",
        install_revision()
    )
}

fn install_revision() -> &'static str {
    if SOURCE_REVISION.is_empty() {
        PREVIOUS_REVISION
    } else {
        SOURCE_REVISION
    }
}

fn install_version() -> &'static str {
    if SOURCE_REVISION.is_empty() {
        PREVIOUS_VERSION
    } else {
        VERSION
    }
}

fn rollback() -> String {
    format!(
        "cargo install --git https://github.com/rockie/rustify-ui --rev {PREVIOUS_REVISION} --locked --profile docs-cli --force rustify-docs-cli"
    )
}

fn help() -> String {
    format!(
        "rustify-docs {VERSION}\n\
         Build a documentation site from Markdown.\n\n\
         Usage: rustify-docs <command> [options]\n\n\
         Commands:\n\
         \x20 new <dir> [--template content|components]\n\
         \x20                                  Create a site in a new or empty directory\n\
         \x20 check [--external-links]         Report problems without changing source or published output\n\
         \x20 build [--base <path>] [--out <dir>]\n\
         \x20                                  Write the production site (default: build.out)\n\
         \x20 preview [--dir <dir>] [--base <path>] [--port <n>] [--host <address>]\n\
         \x20                                  Serve a production build as a static host would\n\
         \x20 dev [--port <n>]                 Serve the site from memory and update open pages on save\n\n\
         Options:\n\
         \x20 --config <path>        The site's rustify-docs.toml (default: ./rustify-docs.toml)\n\
         \x20 --format text|json     How check and build report (json: one object on stdout)\n\
         \x20 --deny-warnings        Fail on warnings as well as errors\n\
         \x20 --version              Print the version\n\
         \x20 --build-info           Print how this binary was built\n\
         \x20 -h, --help             Print this help\n\n\
         Content sites work offline. Component sites need stable Rust and the\n\
         wasm32-unknown-unknown target; after locked dependencies are cached,\n\
         check and build work offline too. check may write private compilation\n\
         caches and requests external links only with --external-links. preview listens on\n\
         127.0.0.1:{} unless told otherwise (the next free port if that one is\n\
         taken; --port 0 lets the system choose). dev listens on 127.0.0.1 at\n\
         dev.port (default {}; likewise the next free port if taken), shows drafts\n\
         and leaves source and published output unchanged (component sites use\n\
         private compilation caches). Exit status: 0 on success, 1\n\
         when the site has errors (or, with --deny-warnings, warnings) or a\n\
         command fails, 2 for a usage error.\n\n\
         Supported on macOS. Linux and Windows are not verified.\n\n\
         Install rustify-docs {} from {}:\n\
         \x20 {}\n\
         \x20 cargo install --list\n\
         Reinstall or upgrade to that revision:\n\
         \x20 {} --force\n\
         Roll back to rustify-docs {PREVIOUS_VERSION}:\n\
         \x20 {}\n\
         Verify --version, cargo install --list and a fresh build's\n\
         manifest.cli_version after installing. Archive builds without git\n\
         metadata use the previous pinned version for installation.\n",
        preview::DEFAULT_PORT,
        rustify_docs::config::DEFAULT_DEV_PORT,
        install_version(),
        install_revision(),
        install(),
        install(),
        rollback(),
    )
}

fn build_info() -> String {
    format!(
        "rustify-docs {VERSION}\npanic: {}\ngrammars: {}\nsource: {}\n",
        if UNWINDS { "unwind" } else { "abort" },
        rustify_docs::highlight::GRAMMAR_VERSION,
        if SOURCE_REVISION.is_empty() {
            "unknown"
        } else {
            SOURCE_REVISION
        }
    )
}

/// Panics inside `catch_unwind` and reports whether the process survived:
/// the install check that the `docs-cli` profile actually took effect.
fn panic_probe() -> ExitCode {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let survived = std::panic::catch_unwind(|| panic!("probe")).is_err();
    std::panic::set_hook(previous);
    if survived {
        println!("recovered");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn usage(message: impl std::fmt::Display) -> ExitCode {
    eprintln!("rustify-docs: {message}\nRun `rustify-docs --help` for usage.");
    ExitCode::from(2)
}

fn fail(command: &str, message: impl std::fmt::Display) -> ExitCode {
    eprintln!("{command} failed: {message}");
    ExitCode::from(1)
}

/// A command's arguments: `--name value` (or `--name=value`) options, bare
/// flags and positional arguments.
#[derive(Default)]
struct Args {
    values: BTreeMap<&'static str, String>,
    flags: Vec<&'static str>,
    positional: Vec<String>,
}

impl Args {
    fn parse(
        args: &[String],
        valued: &[&'static str],
        flags: &[&'static str],
        positional: usize,
    ) -> Result<Self, String> {
        let mut parsed = Self::default();
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].as_str();
            let (name, inline) = match arg.split_once('=') {
                Some((name, value)) if name.starts_with("--") => (name, Some(value.to_owned())),
                _ => (arg, None),
            };
            if let Some(&known) = valued.iter().find(|known| **known == name) {
                let value = match inline {
                    Some(value) => value,
                    None => {
                        i += 1;
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| format!("{name} needs a value"))?
                    }
                };
                parsed.values.insert(known, value);
            } else if let Some(&known) = flags.iter().find(|known| **known == arg) {
                parsed.flags.push(known);
            } else if arg.starts_with('-') && arg != "-" {
                return Err(format!(
                    "{} is not an option of this command",
                    code_span(arg)
                ));
            } else if parsed.positional.len() < positional {
                parsed.positional.push(arg.to_owned());
            } else {
                return Err(format!("unexpected argument {}", code_span(arg)));
            }
            i += 1;
        }
        Ok(parsed)
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    fn flag(&self, name: &str) -> bool {
        self.flags.contains(&name)
    }

    fn format(&self) -> Result<Format, String> {
        match self.get("--format").unwrap_or("text") {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            other => Err(format!("--format {}: use text or json", literal(other))),
        }
    }

    fn config(&self) -> PathBuf {
        PathBuf::from(
            self.get("--config")
                .unwrap_or(rustify_docs::config::FILE_NAME),
        )
    }
}

fn project_root(config: &Path) -> PathBuf {
    match config.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

fn absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    }
}

fn first_error(diagnostics: &[Diagnostic]) -> Option<String> {
    diagnostics
        .iter()
        .find(|d| d.severity == Severity::Error)
        .map(|d| match (&d.file, d.line, d.column) {
            (Some(file), Some(line), Some(column)) => {
                format!("{} ({file}:{line}:{column})", d.message)
            }
            (Some(file), _, _) => format!("{} ({file})", d.message),
            _ => d.message.clone(),
        })
}

/// Prints `diagnostics`, then the conclusion: one JSON object on stdout, or
/// text on stderr ending with the conclusion line(s).
fn report(
    diagnostics: &[Diagnostic],
    format: Format,
    deny_warnings: bool,
    extra: serde_json::Value,
    conclusion: impl FnOnce(&Summary) -> String,
) -> ExitCode {
    let summary = Summary::of(diagnostics, deny_warnings);
    match format {
        Format::Json => println!("{}", report::json(diagnostics, &summary, extra)),
        Format::Text => {
            for diagnostic in diagnostics {
                eprint!("{}", report::text(diagnostic));
            }
            eprintln!("{}", conclusion(&summary));
        }
    }
    if summary.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn counts(summary: &Summary) -> String {
    format!(
        "{} error(s), {} warning(s){}",
        summary.errors,
        summary.warnings,
        if summary.deny_warnings && summary.warnings > 0 {
            " (warnings denied)"
        } else {
            ""
        }
    )
}

/// Reads the project whose configuration is `config`, with `--base`
/// applied.
fn load(config: &Path, base: Option<&str>) -> Result<(Project, Vec<Diagnostic>), Vec<Diagnostic>> {
    let (mut project, diagnostics) = Project::load_file(config)?;
    if let Some(base) = base {
        project
            .config
            .override_base(base)
            .map_err(|message| vec![Diagnostic::general(Severity::Error, "base", message)])?;
    }
    Ok((project, diagnostics))
}

/// Everything `check` and `build` share: the analysis and the generation,
/// in memory. With `external_links` (only `check` asks), the links to other
/// sites are requested too, and how many addresses that was is returned.
fn analyse(
    project: &Project,
    diagnostics: Vec<Diagnostic>,
    external_links: bool,
    build_mode: rust_site::BuildMode,
) -> (
    Vec<Diagnostic>,
    Option<generation::Generation>,
    usize,
    Option<usize>,
    serde_json::Value,
    Option<rust_site::Prepared>,
) {
    let first = analyse_once(project, diagnostics.clone(), external_links, build_mode);
    let errors: Vec<_> = first
        .0
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
        .collect();
    if !errors.is_empty()
        && errors
            .iter()
            .all(|diagnostic| diagnostic.rule == "rust-generation-changed")
        && std::fs::read_to_string(project.root.join(&project.config_file))
            .is_ok_and(|text| text == project.config_text)
    {
        return analyse_once(project, diagnostics, external_links, build_mode);
    }
    first
}

fn analyse_once(
    project: &Project,
    mut diagnostics: Vec<Diagnostic>,
    external_links: bool,
    build_mode: rust_site::BuildMode,
) -> (
    Vec<Diagnostic>,
    Option<generation::Generation>,
    usize,
    Option<usize>,
    serde_json::Value,
    Option<rust_site::Prepared>,
) {
    let (snapshot, dates, problems) = content::read_inputs(project);
    diagnostics.extend(problems);
    let highlighter = Highlighter::new();
    let mut cache = FragmentCache::default();
    let workers = generation::default_workers();
    let analysis = content::analyze(
        project,
        &snapshot,
        Mode::Production,
        &mut cache,
        &highlighter,
        workers,
    );
    let rendering::Build {
        analysis,
        extensions,
        diagnostics: problems,
        info: rust_build,
        guard: rust_guard,
        theme,
    } = rendering::build(
        project,
        &snapshot,
        analysis,
        &mut cache,
        &highlighter,
        build_mode,
    );
    let pages = analysis.published().count();
    diagnostics.extend(analysis.diagnostics.iter().cloned());
    diagnostics.extend(problems);
    let checked = external_links.then(|| {
        let (problems, checked) = links::check(project, &snapshot, &analysis, VERSION);
        diagnostics.extend(problems);
        checked
    });
    let mut generation = if !diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
    {
        match generation::generate_extended(
            project,
            &snapshot,
            &analysis,
            &dates,
            workers,
            &extensions,
        ) {
            Ok(generation) => Some(generation),
            Err(problems) => {
                diagnostics.extend(problems);
                None
            }
        }
    } else {
        None
    };
    if let Some(guard) = &rust_guard {
        if let Err(problems) = guard.validate_current() {
            diagnostics.extend(problems);
            generation = None;
        }
    }
    if let Some(theme) = &theme {
        let problems = theme::validate_snapshot(project, &snapshot, theme);
        if !problems.is_empty() {
            diagnostics.extend(problems);
            generation = None;
        }
    }
    let problems = content::validate_inputs(project, &snapshot, &analysis);
    if !problems.is_empty() {
        diagnostics.extend(problems);
        generation = None;
    }
    diagnostics.sort();
    diagnostics.dedup();
    (
        diagnostics,
        generation,
        pages,
        checked,
        rust_build,
        rust_guard,
    )
}

fn check(args: &[String]) -> ExitCode {
    let args = match Args::parse(
        args,
        &["--config", "--format"],
        &["--deny-warnings", "--external-links"],
        0,
    ) {
        Ok(args) => args,
        Err(message) => return usage(message),
    };
    let format = match args.format() {
        Ok(format) => format,
        Err(message) => return usage(message),
    };
    let deny = args.flag("--deny-warnings");
    let external = args.flag("--external-links");
    let config = args.config();
    let conclusion = |pages: usize, checked: Option<usize>, diagnostics: &[Diagnostic]| {
        let first = first_error(diagnostics)
            .map(|first| format!("; first: {first}"))
            .unwrap_or_default();
        let links = checked
            .map(|checked| format!(", {checked} external address(es) checked"))
            .unwrap_or_default();
        move |summary: &Summary| format!("check: {pages} pages{links}, {}{first}", counts(summary))
    };
    let (project, diagnostics) = match load(&config, None) {
        Ok(loaded) => loaded,
        Err(diagnostics) => {
            let done = conclusion(0, None, &diagnostics);
            return report(
                &diagnostics,
                format,
                deny,
                serde_json::json!({ "pages": 0 }),
                done,
            );
        }
    };
    if format == Format::Text {
        eprintln!("checking {}", absolute(&project.root).display());
    }
    let (diagnostics, _, pages, checked, rust_build, _) = analyse(
        &project,
        diagnostics,
        external,
        rust_site::BuildMode::Development,
    );
    let done = conclusion(pages, checked, &diagnostics);
    let mut extra = serde_json::json!({ "pages": pages });
    if !rust_build.is_null() {
        extra["rust_build"] = rust_build;
    }
    if let Some(checked) = checked {
        extra["external_addresses"] = checked.into();
    }
    report(&diagnostics, format, deny, extra, done)
}

fn kilobytes(bytes: u64) -> String {
    format!("{:.1} KB", bytes as f64 / 1024.0)
}

fn build(args: &[String]) -> ExitCode {
    let args = match Args::parse(
        args,
        &["--config", "--format", "--base", "--out"],
        &["--deny-warnings"],
        0,
    ) {
        Ok(args) => args,
        Err(message) => return usage(message),
    };
    let format = match args.format() {
        Ok(format) => format,
        Err(message) => return usage(message),
    };
    let deny = args.flag("--deny-warnings");
    let started = Instant::now();
    let failed = |diagnostics: &[Diagnostic]| {
        let first = first_error(diagnostics)
            .map(|first| format!("; first: {first}"))
            .unwrap_or_default();
        move |summary: &Summary| {
            format!(
                "build failed: {}{first}; the output was not changed",
                counts(summary)
            )
        }
    };
    let (project, diagnostics) = match load(&args.config(), args.get("--base")) {
        Ok(loaded) => loaded,
        Err(diagnostics) => {
            let done = failed(&diagnostics);
            return report(
                &diagnostics,
                format,
                deny,
                serde_json::json!({ "pages": 0 }),
                done,
            );
        }
    };
    // A failure before anything is rendered: the output cannot be written.
    // JSON still gets its one object, the problem as its diagnostic.
    let refuse = |rule: &str, message: String| {
        if format == Format::Text {
            return fail("build", message);
        }
        let mut diagnostics = diagnostics.clone();
        diagnostics.push(Diagnostic::general(Severity::Error, rule, message));
        let done = failed(&diagnostics);
        report(
            &diagnostics,
            format,
            deny,
            serde_json::json!({ "pages": 0 }),
            done,
        )
    };
    let out = match args.get("--out") {
        Some(out) => absolute(Path::new(out)),
        None => absolute(&project.root.join(&project.config.out_dir)),
    };
    if let Err(message) = publish::check_out(&project, &out) {
        return refuse("output", message);
    }
    if let Some(parent) = out.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            return refuse(
                "output",
                format!("{}: {error}", literal(&parent.display().to_string())),
            );
        }
    }
    let area = match Area::of(&out) {
        Ok(area) => area,
        Err(message) => return refuse("output", message),
    };
    let lock = match publish::lock(&area, true) {
        Ok(lock) => lock,
        Err(LockError::Busy) => {
            return refuse(
                "output-busy",
                format!(
                    "{} is in use by another build or a preview; stop it and build again",
                    literal(&area.out.display().to_string())
                ),
            )
        }
        Err(LockError::Io(message)) => return refuse("output", message),
    };
    match publish::recover(&lock) {
        Ok(Recovery::Clean) => {}
        Ok(recovery) => eprintln!(
            "build: an interrupted build of {} was {}",
            area.out.display(),
            match recovery {
                Recovery::Discarded => "discarded",
                Recovery::Restored => "undone and the previous output restored",
                _ => "completed",
            }
        ),
        Err(message) => return refuse("write", message),
    }
    // Refused before anything is rendered: an output that is not this
    // tool's to replace.
    if let Err(message) = publish::replaceable(&area.out) {
        return refuse("output", message);
    }
    if format == Format::Text {
        eprintln!(
            "building {} into {}",
            absolute(&project.root).display(),
            area.out.display()
        );
    }
    let (mut diagnostics, generation, pages, _, rust_build, rust_guard) =
        analyse(&project, diagnostics, false, rust_site::BuildMode::Release);
    let blocked = Summary::of(&diagnostics, deny);
    let Some(mut generation) = generation.filter(|_| blocked.success()) else {
        let done = failed(&diagnostics);
        return report(
            &diagnostics,
            format,
            deny,
            serde_json::json!({ "pages": 0 }),
            done,
        );
    };
    if let Some(guard) = rust_guard {
        if let Err(problems) = guard.validate_current() {
            diagnostics.extend(problems);
            let done = failed(&diagnostics);
            return report(
                &diagnostics,
                format,
                deny,
                serde_json::json!({"pages":0}),
                done,
            );
        }
    }
    publish::add_manifest(
        &mut generation.files,
        &project.config.site.base,
        VERSION,
        &generation.locale_not_found,
    );
    let stop_at = std::env::var(FAULT_VARIABLE)
        .ok()
        .and_then(|value| value.strip_prefix("stop:").map(str::to_owned));
    let mut at = |step: publish::Step| {
        if stop_at.as_deref() == Some(step.name()) {
            eprintln!("build: stopped at {} ({FAULT_VARIABLE})", step.name());
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
        }
    };
    if let Err(message) = publish::publish_with(&lock, &generation.files, &mut at) {
        let mut diagnostics = diagnostics;
        diagnostics.push(Diagnostic::general(Severity::Error, "write", message));
        let done = failed(&diagnostics);
        return report(
            &diagnostics,
            format,
            deny,
            serde_json::json!({ "pages": 0 }),
            done,
        );
    }
    drop(lock);

    let total: u64 = generation
        .files
        .values()
        .map(|bytes| bytes.len() as u64)
        .sum();
    let mut largest: Vec<(&String, u64)> = generation
        .files
        .iter()
        .filter(|(path, _)| path.ends_with(".html"))
        .map(|(path, bytes)| (path, bytes.len() as u64))
        .collect();
    largest.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    largest.truncate(5);
    let elapsed = started.elapsed().as_millis();
    let mut extra = serde_json::json!({
        "pages": pages,
        "files": generation.files.len(),
        "bytes": total,
        "largest": largest.iter().map(|(path, bytes)| serde_json::json!({ "path": path, "bytes": bytes })).collect::<Vec<_>>(),
        "out": area.out.display().to_string(),
        "milliseconds": elapsed,
    });
    if !rust_build.is_null() {
        extra["rust_build"] = rust_build;
    }
    let files = generation.files.len();
    let out_text = area.out.display().to_string();
    report(&diagnostics, format, deny, extra, move |summary| {
        let mut text = format!(
            "build: {pages} pages and 404.html in {out_text}: {} in {files} files, {} warning(s), {elapsed} ms\nlargest pages:",
            kilobytes(total),
            summary.warnings
        );
        for (path, bytes) in &largest {
            text.push_str(&format!("\n  {:>10}  {path}", kilobytes(*bytes)));
        }
        text
    })
}

fn preview(args: &[String]) -> ExitCode {
    let args = match Args::parse(
        args,
        &["--config", "--dir", "--base", "--port", "--host"],
        &[],
        0,
    ) {
        Ok(args) => args,
        Err(message) => return usage(message),
    };
    let port = match args.get("--port").map(str::parse::<u16>) {
        None => None,
        Some(Ok(port)) => Some(port),
        Some(Err(_)) => return usage("--port takes a number from 0 to 65535"),
    };
    let host = args.get("--host").unwrap_or("127.0.0.1").to_owned();
    let dir = match args.get("--dir") {
        Some(dir) => absolute(Path::new(dir)),
        None => match Project::load_file(&args.config()) {
            Ok((project, _)) => absolute(&project.root.join(&project.config.out_dir)),
            Err(diagnostics) => {
                for diagnostic in &diagnostics {
                    eprint!("{}", report::text(diagnostic));
                }
                return fail(
                    "preview",
                    "give --dir, or --config naming the site's configuration (default: ./rustify-docs.toml)",
                );
            }
        },
    };
    // A build that stopped half-way may have moved the output aside: say
    // that, rather than that there is no build.
    if let Ok(area) = Area::of(&dir) {
        if area.unfinished() {
            return match publish::lock(&area, false) {
                Err(LockError::Busy) => fail(
                    "preview",
                    format!(
                        "{} is being built; preview it when the build ends",
                        literal(&area.out.display().to_string())
                    ),
                ),
                _ => fail(
                    "preview",
                    format!(
                        "a build of {} stopped half-way; run `rustify-docs build` to recover it",
                        literal(&area.out.display().to_string())
                    ),
                ),
            };
        }
    }
    let area = match Area::of(&dir) {
        Ok(area) => area,
        Err(message) => return fail("preview", message),
    };
    let _lock = match publish::lock(&area, false) {
        Ok(lock) => lock,
        Err(LockError::Busy) => {
            return fail(
                "preview",
                format!(
                    "{} is being built; preview it when the build ends",
                    literal(&area.out.display().to_string())
                ),
            )
        }
        Err(LockError::Io(message)) => return fail("preview", message),
    };
    if area.unfinished() {
        return fail(
            "preview",
            format!(
                "a build of {} stopped half-way; run `rustify-docs build` to recover it",
                literal(&area.out.display().to_string())
            ),
        );
    }
    let manifest = match Manifest::read(&area.out) {
        Ok(manifest) => manifest,
        Err(message) => {
            return fail(
                "preview",
                format!("{message}; run `rustify-docs build` first"),
            )
        }
    };
    if manifest.mode != publish::PRODUCTION {
        return fail(
            "preview",
            format!(
                "{} is a {} output; preview serves production builds only",
                literal(&dir.display().to_string()),
                code_span(&manifest.mode)
            ),
        );
    }
    if let Some(base) = args.get("--base") {
        let wanted = match rustify_docs::config::normalize_base(base) {
            Ok(base) => base,
            Err(message) => return usage(format!("--base: {message}")),
        };
        if wanted != manifest.base {
            return fail(
                "preview",
                format!(
                    "the output was built for base {}, not {wanted}; build again with --base {wanted}, or preview with --base {}",
                    literal(&manifest.base), literal(&manifest.base), wanted = literal(&wanted)
                ),
            );
        }
    }
    let locale_not_found = match manifest.locale_not_found_files(&area.out) {
        Ok(files) => files,
        Err(message) => return fail("preview", message),
    };
    let not_found = locale_not_found.get("").cloned().unwrap_or_else(|| {
        std::fs::read(area.out.join("404.html")).unwrap_or_else(|_| b"404 not found\n".to_vec())
    });
    let listener = match preview::bind(&host, port, preview::DEFAULT_PORT) {
        Ok(listener) => listener,
        Err(message) => return fail("preview", message),
    };
    let address = match listener.local_addr() {
        Ok(address) => address,
        Err(error) => return fail("preview", error),
    };
    println!("preview: http://{address}{}", manifest.base);
    eprintln!(
        "serving {} (built by rustify-docs {}, build {}) until stopped",
        area.out.display(),
        manifest.cli_version,
        manifest.build_id
    );
    preview::serve(
        listener,
        preview::Site {
            dir: area.out.canonicalize().unwrap_or(area.out.clone()),
            base: manifest.base,
            not_found,
            locale_not_found,
        },
    );
    ExitCode::SUCCESS
}

fn dev(args: &[String]) -> ExitCode {
    let args = match Args::parse(args, &["--config", "--port"], &[], 0) {
        Ok(args) => args,
        Err(message) => return usage(message),
    };
    let port = match args.get("--port").map(str::parse::<u16>) {
        None => None,
        Some(Ok(port)) => Some(port),
        Some(Err(_)) => return usage("--port takes a number from 0 to 65535"),
    };
    dev::run(dev::Options {
        config: args.config(),
        port,
    })
}

fn new_site(args: &[String]) -> ExitCode {
    let args = match Args::parse(args, &["--template"], &[], 1) {
        Ok(args) => args,
        Err(message) => return usage(message),
    };
    let Some(dir) = args.positional.first() else {
        return usage("new needs the directory to create the site in");
    };
    let dir = PathBuf::from(dir);
    match new::run(&dir, args.get("--template").unwrap_or("content")) {
        Ok(rustc) => {
            eprintln!("{rustc}");
            println!(
                "created {}\nnext:\n  cd {}\n  rustify-docs dev",
                dir.display(),
                dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(message) => fail("new", message),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("-h" | "--help" | "help") => {
            print!("{}", help());
            ExitCode::SUCCESS
        }
        Some("-V" | "--version") => {
            println!("rustify-docs {VERSION}");
            ExitCode::SUCCESS
        }
        Some("--build-info") => {
            print!("{}", build_info());
            ExitCode::SUCCESS
        }
        Some("__panic-probe") => panic_probe(),
        Some("check") => check(&args[1..]),
        Some("build") => build(&args[1..]),
        Some("preview") => preview(&args[1..]),
        Some("new") => new_site(&args[1..]),
        Some("dev") => dev(&args[1..]),
        Some(other) => usage(format!("unknown command {}", code_span(other))),
    }
}
