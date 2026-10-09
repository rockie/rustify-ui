use super::super::{merge_configs, ConfigExtension};
use super::*;
use std::rc::Rc;

/// Stylesheet discovery, an explicit entry, or disabled theme registration.
#[derive(Clone, Default)]
pub enum CssInput {
    #[default]
    Discover,
    Disabled,
    File(String),
}
/// Project build options corresponding to cn/build, with explicit source ownership.
#[derive(Clone)]
pub struct ProjectOptions {
    pub cwd: Option<String>,
    pub content: Vec<String>,
    pub out: String,
    pub safelist: Option<String>,
    pub config: Option<String>,
    pub tokens: Option<String>,
    pub full: bool,
    pub css: CssInput,
}
impl Default for ProjectOptions {
    fn default() -> Self {
        Self {
            cwd: None,
            content: Vec::new(),
            out: "cn-tables.mjs".into(),
            safelist: None,
            config: None,
            tokens: None,
            full: false,
            css: CssInput::Discover,
        }
    }
}
/// Host module loader. It resolves a module's default/config export and applies
/// its extension or transform to the supplied base. Errors from a transform
/// propagate unchanged. JS/WASM hosts must preserve their module/callback realm.
pub type ConfigLoader = Rc<dyn Fn(&BuildSourceTree, &str, Config) -> Result<Config, CnError>>;
/// Complete project report, including the fitted artifact and source diagnostics.
pub struct ProjectReport {
    pub artifact: BuildReport,
    pub out_path: String,
    pub scanned_files: usize,
    pub css_files: Vec<String>,
    pub warnings: Vec<String>,
}
/// Load the JSON-safe marker representation for native Rust command-line callers.
/// JavaScript modules use the supplied `ConfigLoader` instead of this adapter.
pub fn load_json_config(
    tree: &BuildSourceTree,
    path: &str,
    base: Config,
) -> Result<Config, CnError> {
    let text = tree
        .read_text(path)
        .map_err(|err| CnError::new(format!("cannot load config {path}: {err}")))?;
    let value = serde_json::from_str(&text)
        .map_err(|err| CnError::new(format!("cannot load config {path}: {err}")))?;
    Ok(merge_configs(&base, &ConfigExtension::from_json(&value)?))
}
fn read_words(
    tree: &BuildSourceTree,
    cwd: &str,
    file: &str,
    what: &str,
    tokens: &mut Vec<String>,
) -> Result<(), CnError> {
    let text = tree
        .read_text(&tree.resolve_path(cwd, file)?)
        .map_err(|err| CnError::new(format!("cannot read {what} {file}: {err}")))?;
    add_project_words(&text, tokens);
    Ok(())
}
/// Add words from a tokens or safelist read to an ordered candidate set.
pub fn add_project_words(text: &str, tokens: &mut Vec<String>) {
    let mut seen: std::collections::HashSet<String> = tokens.iter().cloned().collect();
    for word in text
        .split(super::super::validators::is_whitespace)
        .filter(|s| !s.is_empty())
    {
        if seen.insert(word.into()) {
            tokens.push(word.into());
        }
    }
}
/// Sources and CSS base resolved before an asynchronous module import.
pub struct PreparedProject {
    pub cwd: String,
    pub out_path: String,
    pub config: Config,
    pub css_files: Vec<String>,
}
pub struct ProjectScan {
    pub tokens: Vec<String>,
    pub scanned_files: usize,
    pub warnings: Vec<String>,
}
pub fn prepare_project(
    tree: &BuildSourceTree,
    options: &ProjectOptions,
) -> Result<PreparedProject, CnError> {
    let cwd = tree.cwd(options.cwd.as_deref().unwrap_or("."))?;
    let out_path = tree.resolve_path(&cwd, &options.out)?;
    prepare_project_at(tree, cwd, out_path, &options.css)
}
/// Prepare CSS after the host has read cwd/out/css at their observable stages.
pub fn prepare_project_at(
    tree: &BuildSourceTree,
    cwd: String,
    out_path: String,
    css: &CssInput,
) -> Result<PreparedProject, CnError> {
    let mut config = Config::default();
    let mut css_files = Vec::new();
    if !matches!(css, CssInput::Disabled) {
        let entries = match css {
            CssInput::File(file) => vec![tree.resolve_path(&cwd, file)?],
            CssInput::Discover => {
                expand_globs(tree, &["**/*.css".into()], &cwd, &mut ScanState::default())?
                    .into_iter()
                    .filter(|file| tree.read_text(file).is_ok_and(|css| is_tailwind_css(&css)))
                    .collect()
            }
            CssInput::Disabled => Vec::new(),
        };
        let theme = theme_from_css(tree, &entries)?;
        config = merge_configs(&config, &theme.extension);
        css_files = theme.files;
    }
    Ok(PreparedProject {
        cwd,
        out_path,
        config,
        css_files,
    })
}
/// Scan after configuration loading, preserving upstream observable IO order.
pub fn scan_project(
    tree: &BuildSourceTree,
    options: &ProjectOptions,
    prepared: &PreparedProject,
) -> Result<ProjectScan, CnError> {
    let mut tokens = Vec::new();
    let mut warnings = Vec::new();
    let cwd = &prepared.cwd;
    let mut scanned_files = 0;
    if !options.full {
        if let Some(file) = &options.tokens {
            read_words(tree, cwd, file, "tokens file", &mut tokens)?;
        } else {
            let scan = scan_project_sources(tree, &content_patterns(options), prepared)?;
            tokens = scan.tokens;
            scanned_files = scan.scanned_files;
            warnings = scan.warnings;
        }
        if let Some(file) = &options.safelist {
            read_words(tree, cwd, file, "safelist", &mut tokens)?;
        }
    }
    Ok(ProjectScan {
        tokens,
        scanned_files,
        warnings,
    })
}
/// Scan only content paths; the host reads safelist options after this stage.
pub fn scan_project_sources(
    tree: &BuildSourceTree,
    patterns: &[String],
    prepared: &PreparedProject,
) -> Result<ProjectScan, CnError> {
    let cwd = &prepared.cwd;
    let out_path = &prepared.out_path;
    let mut tokens = Vec::new();
    let mut warnings = Vec::new();
    let mut scanned_files = 0;
    let mut state = ScanState::default();
    let mut skipped_long = 0;
    let out_real = tree.realpath(out_path).ok();
    let files = expand_globs(tree, patterns, cwd, &mut state)?
        .into_iter()
        .filter(|file| {
            file != out_path && (out_real.is_none() || tree.realpath(file).ok() != out_real)
        })
        .collect::<Vec<_>>();
    if files.is_empty() {
        return Err(CnError::new(
            "no files matched the content globs; pass --content or use --full",
        ));
    }
    for file in files {
        match tree.read_text(&file) {
            Ok(text) => {
                skipped_long += extract_tokens(&text, &mut tokens);
                scanned_files += 1;
            }
            Err(_) => state.skipped_unreadable += 1,
        }
    }
    if state.skipped_unreadable > 0 {
        warnings.push(format!(
            "skipped {} unreadable path(s)",
            state.skipped_unreadable
        ));
    }
    if skipped_long > 0 {
        warnings.push(format!(
            "skipped {skipped_long} candidate token(s) longer than 8192 chars"
        ));
    }
    Ok(ProjectScan {
        tokens,
        scanned_files,
        warnings,
    })
}
/// Scan sources, resolve theme/config/safelist, compile, and leave identical
/// output untouched. Native and virtual projects share all build semantics.
pub fn build(
    tree: &BuildSourceTree,
    options: &ProjectOptions,
    loader: Option<&ConfigLoader>,
) -> Result<ProjectReport, CnError> {
    let prepared = prepare_project(tree, options)?;
    let cwd = &prepared.cwd;
    let out_path = prepared.out_path.clone();
    let css_files = prepared.css_files.clone();
    let mut config = prepared.config.clone();
    if let Some(file) = &options.config {
        let path = tree.resolve_path(cwd, file)?;
        config = match loader {
            Some(load) => load(tree, &path, config)?,
            None => load_json_config(tree, &path, config)?,
        };
    }
    let ProjectScan {
        tokens,
        scanned_files,
        warnings,
    } = scan_project(tree, options, &prepared)?;
    let previous = tree.read_text(&out_path).ok();
    let inputs: Vec<_> = tokens.iter().map(String::as_str).collect();
    let artifact = build_tokens(
        &config,
        &inputs,
        BuildOptions {
            out: &out_path,
            full: options.full,
            previous_source: previous.as_deref(),
        },
    )?;
    if artifact.changed {
        tree.write_text(&out_path, &artifact.source)
            .map_err(|err| CnError::new(format!("cannot write {}: {err}", options.out)))?;
    }
    Ok(ProjectReport {
        artifact,
        out_path,
        scanned_files,
        css_files,
        warnings,
    })
}
pub(crate) fn content_patterns(options: &ProjectOptions) -> Vec<String> {
    if options.content.is_empty() {
        DEFAULT_CONTENT.iter().map(|s| (*s).into()).collect()
    } else {
        options.content.clone()
    }
}
