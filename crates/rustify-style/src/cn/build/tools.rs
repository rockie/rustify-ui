//! CLI and bundler lifecycle cores; hosts provide process streams and watchers.
use super::*;
use std::io::Write;
use std::rc::Rc;

pub const UPSTREAM_VERSION: &str = "0.4.0";
pub const CLI_HELP: &str = r#"Usage: cn build [options]

Options:
  --content <glob>     source globs to scan (repeatable or comma-separated;
                       braces like *.{ts,tsx} are supported; a literal
                       directory scans every file in it)
                       default: **/*.{js,jsx,ts,tsx,html,vue,svelte,astro,mdx}
  -o, --out <path>     output module path (default: cn-tables.mjs; .ts emits TS)
  --safelist <file>    extra class names, whitespace-separated
  --config <file>      config extension module (default export)
  --tokens <file>      pre-extracted tokens (skips scanning)
  --full               keep all class groups (no subsetting)
  --css <file>         read @theme scales from this stylesheet only
                       default: every .css under cwd that imports Tailwind
                       or declares a theme
  --no-css             do not read theme scales from stylesheets
  --cwd <dir>          base directory (default: process.cwd())
  -q, --quiet          suppress summary output
  -h, --help           show this help
  -v, --version        show version
"#;
/// A parsed cn tool invocation; callers decide process exit and stream ownership.
pub enum CliCommand {
    Help,
    Version,
    Build {
        options: ProjectOptions,
        quiet: bool,
    },
}
fn split_content(value: &str) -> Vec<String> {
    let mut depth = 0usize;
    let mut start = 0;
    let mut parts = Vec::new();
    for (index, c) in value.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(&value[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(&value[start..]);
    parts
        .into_iter()
        .map(|s| s.trim_matches(super::super::validators::is_whitespace))
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
/// Parse every upstream flag, including comma-separated content outside braces.
pub fn parse_cli(args: &[String]) -> Result<CliCommand, CnError> {
    if args.is_empty() || args.iter().any(|s| s == "-h" || s == "--help") {
        return Ok(CliCommand::Help);
    }
    if args.iter().any(|s| s == "-v" || s == "--version") {
        return Ok(CliCommand::Version);
    }
    if args[0] != "build" {
        return Err(CnError::new(format!(
            "unknown command \"{}\" (expected: build)",
            args[0]
        )));
    }
    let mut options = ProjectOptions::default();
    let mut quiet = false;
    let mut i = 1;
    while i < args.len() {
        let flag = &args[i];
        let mut next = || {
            i += 1;
            args.get(i)
                .ok_or_else(|| CnError::new(format!("missing value for {flag}")))
        };
        match flag.as_str() {
            "--content" => options.content.extend(split_content(next()?)),
            "-o" | "--out" => options.out = next()?.clone(),
            "--safelist" => options.safelist = Some(next()?.clone()),
            "--config" => options.config = Some(next()?.clone()),
            "--tokens" => options.tokens = Some(next()?.clone()),
            "--full" => options.full = true,
            "--css" => options.css = CssInput::File(next()?.clone()),
            "--no-css" => options.css = CssInput::Disabled,
            "--cwd" => options.cwd = Some(next()?.clone()),
            "-q" | "--quiet" => quiet = true,
            _ => return Err(CnError::new(format!("unknown option \"{flag}\""))),
        };
        i += 1;
    }
    Ok(CliCommand::Build { options, quiet })
}
/// Actual gzip size with level 9 and a pure Rust DEFLATE implementation.
pub fn gzip_size(source: &str) -> Result<usize, CnError> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(9));
    encoder
        .write_all(source.as_bytes())
        .map_err(|err| CnError::new(err.to_string()))?;
    Ok(encoder
        .finish()
        .map_err(|err| CnError::new(err.to_string()))?
        .len())
}
/// Format the build summary using the real emitted UTF-16/source and gzip sizes.
pub fn cli_summary(report: &ProjectReport, options: &ProjectOptions) -> Result<String, CnError> {
    let parts = if options.full {
        "all class groups kept (--full)".into()
    } else {
        let scanned = if report.scanned_files > 0 {
            format!("scanned {} files, ", report.scanned_files)
        } else {
            String::new()
        };
        format!(
            "{scanned}{} candidate tokens, {}/{} class groups kept",
            report.artifact.tokens.len(),
            report.artifact.used_groups.unwrap_or_default(),
            report.artifact.total_groups.unwrap_or_default()
        )
    };
    let size = (report.artifact.source.encode_utf16().count() as f64 * 10. / 1024.).round() / 10.;
    let gzip = (gzip_size(&report.artifact.source)? as f64 * 10. / 1024.).round() / 10.;
    Ok(format!(
        "cn build: {parts}\n  → {} ({size:.1} KB source, {gzip:.1} KB gzip)\n",
        options.out
    ))
}
/// CLI output with stdout/stderr and exit status, without global process mutation.
pub struct CliOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
pub fn run_cli(
    tree: &BuildSourceTree,
    args: &[String],
    loader: Option<&ConfigLoader>,
) -> CliOutput {
    let result: Result<(String, String), CnError> = (|| match parse_cli(args)? {
        CliCommand::Help => Ok((CLI_HELP.to_owned() + "\n", String::new())),
        CliCommand::Version => Ok((UPSTREAM_VERSION.to_owned() + "\n", String::new())),
        CliCommand::Build { options, quiet } => {
            let report = build(tree, &options, loader)?;
            if quiet {
                return Ok((String::new(), String::new()));
            }
            let stderr = report
                .warnings
                .iter()
                .map(|w| format!("cn: {w}\n"))
                .collect();
            Ok((cli_summary(&report, &options)?, stderr))
        }
    })();
    match result {
        Ok((stdout, stderr)) => CliOutput {
            stdout,
            stderr,
            exit_code: 0,
        },
        Err(error) => CliOutput {
            stdout: String::new(),
            stderr: format!("cn: {error}\n"),
            exit_code: 1,
        },
    }
}
/// Rust equivalent of cn/vite's configResolved/buildStart/watchChange hooks.
pub struct ViteAdapter {
    tree: Rc<BuildSourceTree>,
    options: ProjectOptions,
    loader: Option<ConfigLoader>,
    builder: Option<Builder>,
}
impl ViteAdapter {
    pub const NAME: &str = "cn:build";
    pub fn new(
        tree: Rc<BuildSourceTree>,
        options: ProjectOptions,
        loader: Option<ConfigLoader>,
    ) -> Self {
        Self {
            tree,
            options,
            loader,
            builder: None,
        }
    }
    pub fn config_resolved(&mut self, root: &str) -> Result<(), CnError> {
        let mut options = self.options.clone();
        if options.cwd.is_none() {
            options.cwd = Some(root.into());
        }
        self.builder = Some(create_builder(
            self.tree.clone(),
            options,
            self.loader.clone(),
        )?);
        Ok(())
    }
    pub fn build_start(&mut self) -> Result<Rc<ProjectReport>, CnError> {
        if self.builder.is_none() {
            self.builder = Some(create_builder(
                self.tree.clone(),
                self.options.clone(),
                self.loader.clone(),
            )?);
        }
        self.builder
            .as_ref()
            .ok_or_else(|| CnError::new("cn: builder initialization failed"))?
            .run()
    }
    /// Awaitable lifecycle entry that preserves builder flight coalescing.
    pub fn build_start_async(&mut self) -> Result<BuildFuture, CnError> {
        if self.builder.is_none() {
            self.builder = Some(create_builder(
                self.tree.clone(),
                self.options.clone(),
                self.loader.clone(),
            )?);
        }
        Ok(self
            .builder
            .as_ref()
            .ok_or_else(|| CnError::new("cn: builder initialization failed"))?
            .run_async())
    }
    pub fn watch_change_async(
        &self,
        path: &str,
        event: &str,
    ) -> Result<Option<BuildFuture>, CnError> {
        if event == "delete" {
            return Ok(None);
        }
        self.builder
            .as_ref()
            .map(|builder| builder.changed_async(path))
            .transpose()
    }
    pub fn watch_change(
        &self,
        path: &str,
        event: &str,
    ) -> Result<Option<Rc<ProjectReport>>, CnError> {
        if event == "delete" {
            return Ok(None);
        }
        self.builder
            .as_ref()
            .map(|builder| builder.changed(path))
            .transpose()
    }
}
/// Prepared Next lifecycle. The host subscribes a nonpersistent recursive watch
/// in development and routes its filenames through `watch_event`.
pub struct NextBuild<T> {
    pub config: T,
    pub builder: Builder,
    pub watch_directory: Option<String>,
}
impl<T> NextBuild<T> {
    pub fn watch_event_async(
        &self,
        filename: Option<&str>,
    ) -> Result<Option<BuildFuture>, CnError> {
        if self.watch_directory.is_none() {
            return Ok(None);
        }
        filename
            .map(|filename| self.builder.changed_async(filename))
            .transpose()
    }
    pub fn watch_event(
        &self,
        filename: Option<&str>,
    ) -> Result<Option<Rc<ProjectReport>>, CnError> {
        if self.watch_directory.is_none() {
            return Ok(None);
        }
        filename
            .map(|filename| self.builder.changed(filename))
            .transpose()
    }
}
/// Generate before evaluating the Next config factory for either bundler. The
/// phase/context remain owned by the caller; development exposes watcher setup.
pub fn with_cn<T, C>(
    tree: Rc<BuildSourceTree>,
    options: ProjectOptions,
    loader: Option<ConfigLoader>,
    phase: &str,
    context: &C,
    factory: impl FnOnce(&str, &C) -> Result<T, CnError>,
) -> Result<NextBuild<T>, CnError> {
    let builder = create_builder(tree.clone(), options.clone(), loader)?;
    builder.run()?;
    let watch_directory = if phase == "phase-development-server" {
        Some(tree.cwd(options.cwd.as_deref().unwrap_or("."))?)
    } else {
        None
    };
    let config = factory(phase, context)?;
    Ok(NextBuild {
        config,
        builder,
        watch_directory,
    })
}

/// Await a Next config factory after generating the module. Hosts may pass
/// either an immediately ready value or a genuinely asynchronous factory.
pub async fn with_cn_async<T, C, F>(
    tree: Rc<BuildSourceTree>,
    options: ProjectOptions,
    loader: Option<ConfigLoader>,
    phase: &str,
    context: &C,
    factory: impl FnOnce(&str, &C) -> F,
) -> Result<NextBuild<T>, CnError>
where
    F: std::future::Future<Output = Result<T, CnError>>,
{
    let builder = create_builder(tree.clone(), options.clone(), loader)?;
    builder.run_async().await?;
    let watch_directory = if phase == "phase-development-server" {
        Some(tree.cwd(options.cwd.as_deref().unwrap_or("."))?)
    } else {
        None
    };
    let config = factory(phase, context).await?;
    Ok(NextBuild {
        config,
        builder,
        watch_directory,
    })
}
