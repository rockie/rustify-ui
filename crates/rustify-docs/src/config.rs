//! `rustify-docs.toml`, schema version 1.
//!
//! Every key is known: an unknown one is an error rather than a setting that
//! silently does nothing, so a future version can add keys without an old
//! site meaning something else by them. Paths are relative to the project
//! root (the directory holding the file); the URL base is normalized once
//! here and every URL the build writes goes through it.

use crate::diagnostic_text::{code_span, literal};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use serde::Deserialize;
use toml::Spanned;

use crate::diagnostic::Issue;

pub const FILE_NAME: &str = "rustify-docs.toml";
pub const SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_DEV_PORT: u16 = 4188;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    schema_version: Spanned<u32>,
    site: RawSite,
    #[serde(default)]
    content: RawContent,
    #[serde(default)]
    assets: RawDir,
    #[serde(default)]
    build: RawBuild,
    repo: Option<RawRepo>,
    #[serde(default)]
    theme: RawTheme,
    #[serde(default)]
    nav: RawNav,
    #[serde(default)]
    dev: RawDev,
    rust: Option<RawRust>,
    i18n: Option<Spanned<RawI18n>>,
    landing: Option<RawLanding>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLanding {
    #[serde(default)]
    profiles: BTreeMap<String, Spanned<RawProfile>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProfile {
    #[serde(default)]
    scripts: Vec<RawScript>,
    #[serde(default)]
    sources: RawSources,
    #[serde(default)]
    carry: Vec<Spanned<String>>,
    #[serde(default)]
    footer: Vec<RawLink>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawScript {
    src: Spanned<String>,
    load: Option<crate::landing::Load>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSources {
    #[serde(default)]
    script: Vec<Spanned<String>>,
    #[serde(default)]
    connect: Vec<Spanned<String>>,
    #[serde(default)]
    img: Vec<Spanned<String>>,
    #[serde(default)]
    frame: Vec<Spanned<String>>,
    #[serde(default)]
    form: Vec<Spanned<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawI18n {
    #[serde(default)]
    locales: Vec<RawLocale>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLocale {
    lang: Spanned<String>,
    label: String,
    prefix: Spanned<String>,
    messages: Option<Spanned<String>>,
    nav: Option<RawNav>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRust {
    manifest: Spanned<String>,
    registry: Spanned<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSite {
    name: String,
    description: Option<String>,
    lang: Option<Spanned<String>>,
    url: Option<Spanned<String>>,
    base: Option<Spanned<String>>,
    image: Option<Spanned<String>>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawContent {
    dir: Option<Spanned<String>>,
    include: Option<Vec<Spanned<String>>>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDir {
    dir: Option<Spanned<String>>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBuild {
    out: Option<Spanned<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRepo {
    url: Spanned<String>,
    #[serde(rename = "ref")]
    reference: String,
    #[serde(default)]
    path: String,
    edit_url: Option<Spanned<String>>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTheme {
    document: Option<Spanned<String>>,
    primary: Option<Spanned<RawPair>>,
    logo: Option<Spanned<RawLogo>>,
    #[serde(default)]
    fonts: Vec<Spanned<toml::Table>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPair {
    light: Spanned<String>,
    dark: Spanned<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawLogo {
    One(String),
    Pair { light: String, dark: String },
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawNav {
    #[serde(default)]
    groups: Vec<Spanned<RawGroup>>,
    #[serde(default)]
    top: Vec<RawLink>,
    #[serde(default)]
    social: Vec<RawLink>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGroup {
    label: String,
    dir: Option<Spanned<String>>,
    items: Option<Vec<Spanned<String>>>,
    routes: Option<Vec<Spanned<String>>>,
    scope: Option<Spanned<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLink {
    label: String,
    link: Spanned<String>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDev {
    port: Option<u16>,
}

/// A site's configuration, validated and normalized.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub site: SiteConfig,
    /// Directories relative to the project root, normalized with `/`.
    pub content_dir: String,
    /// Explicit content-relative Markdown files and their configuration spans.
    /// `None` discovers recursively; an empty list selects no Markdown pages.
    pub content_include: Option<Vec<(String, Range<usize>)>>,
    pub assets_dir: String,
    pub out_dir: String,
    pub repo: Option<Repo>,
    pub primary: Option<Pair>,
    /// A project-relative Theme Studio JSON file and its configuration span.
    pub theme_document: Option<(String, Range<usize>)>,
    pub theme_fonts: Vec<FontConfig>,
    pub logo: Option<Logo>,
    pub nav: Nav,
    /// Declared languages; empty preserves the single-language configuration.
    pub locales: Vec<LocaleConfig>,
    pub dev_port: u16,
    pub rust: Option<RustConfig>,
    /// Landing-page profiles by name.
    pub landing: BTreeMap<String, crate::landing::Profile>,
}

/// A component site's Cargo project and single `site!` declaration source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustConfig {
    pub manifest: String,
    pub registry: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteConfig {
    pub name: String,
    pub description: Option<String>,
    pub lang: String,
    /// The public URL of the site's root, ending in `/`, when known.
    pub url: Option<String>,
    /// `/` or `/segments/`.
    pub base: String,
    /// The image link previews show, project-relative, with its span.
    pub image: Option<(String, Range<usize>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocaleConfig {
    pub lang: String,
    /// Where the language was declared, for missing-translation diagnostics.
    pub lang_span: Range<usize>,
    pub label: String,
    pub prefix: String,
    /// A project-relative message catalog and its configuration span.
    pub messages: Option<(String, Range<usize>)>,
    pub nav: Option<Nav>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Repo {
    pub url: String,
    pub reference: String,
    /// Where the project root sits inside the repository, `""` at its root.
    pub path: String,
    pub edit_url: Option<String>,
}

/// A colour for each mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair {
    pub light: String,
    pub dark: String,
    /// Where each was written, for contrast warnings.
    pub spans: (Range<usize>, Range<usize>),
}

/// The logo for each mode, as project-relative paths; one file serves both
/// when only one is given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logo {
    pub light: String,
    pub dark: String,
    pub span: Range<usize>,
}

/// A project-owned local font face; registration does not select the site's stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontConfig {
    pub family: String,
    pub src: String,
    pub weight: String,
    pub style: String,
    pub span: Range<usize>,
}

fn parse_fonts(
    raw: Vec<Spanned<toml::Table>>,
    out: &str,
    errors: &mut Vec<Issue>,
) -> Vec<FontConfig> {
    let mut faces = BTreeSet::new();
    raw.into_iter()
        .enumerate()
        .filter_map(|(index, raw)| {
            let span = raw.span();
            let fields = raw.into_inner();
            let parse = || -> Result<FontConfig, String> {
                if let Some(key) = fields
                    .keys()
                    .find(|key| !["family", "src", "weight", "style"].contains(&key.as_str()))
                {
                    return Err(format!("unknown font field {}", code_span(key)));
                }
                let string = |key: &str, default: Option<&str>| -> Result<String, String> {
                    match fields.get(key) {
                        Some(toml::Value::String(value)) => Ok(value.clone()),
                        None if default.is_some() => Ok(default.unwrap_or_default().to_owned()),
                        _ => Err(format!(
                            "{key} must be a string{}",
                            if default.is_none() {
                                " and is required"
                            } else {
                                ""
                            }
                        )),
                    }
                };
                let family = string("family", None)?;
                if family.trim().is_empty() || family.chars().any(char::is_control) {
                    return Err("family must be a non-empty name without control characters".into());
                }
                let literal = string("src", None)?;
                if literal.contains(':') {
                    return Err(
                        "src must be a project-local file, not a URL or absolute path".into(),
                    );
                }
                let src = relative_path(&literal)?;
                if src.is_empty()
                    || crate::content::is_private(&src)
                    || src == out
                    || src.starts_with(&format!("{out}/"))
                {
                    return Err("src must name a non-private project file outside build.out".into());
                }
                let extension = std::path::Path::new(&src)
                    .extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                if !["woff2", "woff", "ttf", "otf"].contains(&extension.as_str()) {
                    return Err("src must be a woff2, woff, ttf or otf file".into());
                }
                let weight = string("weight", Some("400"))?;
                let weights: Vec<_> = weight
                    .split_whitespace()
                    .map(|part| {
                        if part.bytes().all(|byte| byte.is_ascii_digit()) {
                            part.parse::<u16>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                let weights: Option<Vec<_>> = weights.into_iter().collect();
                let weights = weights
                    .filter(|values| {
                        matches!(values.len(), 1 | 2)
                            && values.iter().all(|weight| (1..=1000).contains(weight))
                            && (values.len() == 1 || values[0] < values[1])
                    })
                    .ok_or_else(|| {
                        "weight must be one integer or two ascending integers in 1..1000".to_owned()
                    })?;
                let weight = weights
                    .iter()
                    .map(u16::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                let style = string("style", Some("normal"))?;
                if !["normal", "italic"].contains(&style.as_str()) {
                    return Err("style must be normal or italic".into());
                }
                Ok(FontConfig {
                    family,
                    src,
                    weight,
                    style,
                    span: span.clone(),
                })
            };
            match parse() {
                Ok(font)
                    if faces.insert((
                        font.family.to_lowercase(),
                        font.weight.clone(),
                        font.style.clone(),
                    )) =>
                {
                    Some(font)
                }
                Ok(_) => {
                    errors.push(error(
                        span,
                        "theme-font",
                        format!("theme.fonts[{index}] duplicates a family/weight/style face"),
                    ));
                    None
                }
                Err(message) => {
                    errors.push(error(
                        span,
                        "theme-font",
                        format!("theme.fonts[{index}]: {message}"),
                    ));
                    None
                }
            }
        })
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Nav {
    pub groups: Vec<NavGroupConfig>,
    pub top: Vec<NavLink>,
    pub social: Vec<NavLink>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupSource {
    /// Every page under this content directory, in `sidebar.order`.
    Dir { dir: String, span: Range<usize> },
    /// These Markdown files, relative to the content directory, in order.
    Items(Vec<(String, Range<usize>)>),
    /// Registered page routes, without the deployment base, in order.
    Routes(Vec<(String, Range<usize>)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavGroupConfig {
    pub label: String,
    pub source: GroupSource,
    /// The routes under which the group is shown: `/` for every page.
    pub scope: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavLink {
    pub label: String,
    pub link: String,
    pub span: Range<usize>,
}

fn error(span: Range<usize>, rule: &'static str, message: impl Into<String>) -> Issue {
    Issue::error(span, rule, message)
}

/// A project-relative path, normalized: no absolute paths, no `..` out of
/// the project, `/` separators, no `.` segments.
pub fn relative_path(path: &str) -> Result<String, String> {
    let path = path.replace('\\', "/");
    if path.starts_with('/') || path.chars().nth(1) == Some(':') {
        return Err(format!(
            "{} is absolute; paths are relative to the project root",
            code_span(&path)
        ));
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(format!("{} leaves the project", code_span(&path)));
                }
            }
            part => parts.push(part),
        }
    }
    Ok(parts.join("/"))
}

/// `site.base`: `/` or `/a/b/`, made of URL-safe segments.
pub fn normalize_base(base: &str) -> Result<String, String> {
    let trimmed = base.trim().trim_matches('/');
    if trimmed.is_empty() {
        return Ok("/".into());
    }
    for segment in trimmed.split('/') {
        let valid = !segment.is_empty()
            && segment != "."
            && segment != ".."
            && segment
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~'));
        if !valid {
            return Err(format!("{} is not a base path; use `/` or segments of letters, digits, `-`, `_`, `.` and `~`, like `/docs/`", code_span(base)));
        }
    }
    Ok(format!("/{trimmed}/"))
}

/// A language tag in the shape BCP 47 gives it: a 2–3 letter language, then
/// subtags of 1–8 letters or digits.
pub fn valid_lang(tag: &str) -> bool {
    let mut parts = tag.split('-');
    let language = parts.next().unwrap_or("");
    (2..=3).contains(&language.len())
        && language.chars().all(|c| c.is_ascii_alphabetic())
        && parts.all(|part| {
            (1..=8).contains(&part.len()) && part.chars().all(|c| c.is_ascii_alphanumeric())
        })
}

/// A non-default locale's URL segment, excluding generated and private output.
pub fn valid_locale_prefix(prefix: &str) -> bool {
    !prefix.is_empty()
        && !matches!(prefix, "." | ".." | "assets" | "404" | "index.html")
        && prefix
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~'))
        && !crate::content::is_private(prefix)
        && !crate::generation::SITE_FILES.contains(&prefix)
}

fn absolute_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://"))
        && url.len() > url.find("://").unwrap_or(0) + 3
        && !url.contains(char::is_whitespace)
}

fn parse_nav(raw: RawNav, rule: &'static str, errors: &mut Vec<Issue>) -> Nav {
    let mut groups = Vec::new();
    for group in raw.groups {
        let span = group.span();
        let group = group.into_inner();
        let scope = match &group.scope {
            None => "/".to_owned(),
            Some(scope) => match normalize_base(scope.get_ref()) {
                Ok(scope) => scope,
                Err(_) => {
                    errors.push(error(
                        scope.span(),
                        rule,
                        format!(
                            "nav scope {} is not a route prefix like `/guide/`",
                            code_span(scope.get_ref())
                        ),
                    ));
                    continue;
                }
            },
        };
        let source = match (group.dir, group.items, group.routes) {
            (Some(dir), None, None) => match relative_path(dir.get_ref()) {
                Ok(path) => GroupSource::Dir {
                    dir: path,
                    span: dir.span(),
                },
                Err(message) => {
                    errors.push(error(
                        dir.span(),
                        rule,
                        format!("nav group {}: {message}", code_span(&group.label)),
                    ));
                    continue;
                }
            },
            (None, Some(items), None) => {
                let mut list = Vec::new();
                for item in items {
                    match relative_path(item.get_ref()) {
                        Ok(path) => list.push((path, item.span())),
                        Err(message) => errors.push(error(item.span(), rule, message)),
                    }
                }
                GroupSource::Items(list)
            }
            (None, None, Some(routes)) => {
                let mut list = Vec::new();
                for route in routes {
                    let value = route.get_ref();
                    match normalize_base(value) {
                        Ok(normalized) if normalized == *value => {
                            list.push((normalized, route.span()));
                        }
                        _ => errors.push(error(route.span(), rule, "nav.routes needs canonical routes like `/custom/`, without the deployment base, query or fragment")),
                    }
                }
                GroupSource::Routes(list)
            }
            _ => {
                errors.push(error(
                    span,
                    rule,
                    format!(
                        "nav group {} needs exactly one of `dir`, `items` and `routes`",
                        code_span(&group.label)
                    ),
                ));
                continue;
            }
        };
        groups.push(NavGroupConfig {
            label: group.label,
            source,
            scope,
        });
    }
    Nav {
        groups,
        top: nav_links(raw.top),
        social: nav_links(raw.social),
    }
}

fn nav_links(links: Vec<RawLink>) -> Vec<NavLink> {
    links
        .into_iter()
        .map(|link| NavLink {
            label: link.label,
            span: link.link.span(),
            link: link.link.into_inner(),
        })
        .collect()
}

fn parse_landing(
    raw: Option<RawLanding>,
    errors: &mut Vec<Issue>,
) -> BTreeMap<String, crate::landing::Profile> {
    use crate::landing::{self, Profile, Script, Sources};
    let mut profiles = BTreeMap::new();
    for (name, profile) in raw.map(|raw| raw.profiles).unwrap_or_default() {
        let span = profile.span();
        let raw = profile.into_inner();
        if !landing::valid_name(&name) {
            errors.push(error(
                span.clone(),
                "landing-profile-name",
                format!(
                    "landing profile name {} must be lower-case letters, digits and `-`, at most 32, starting with a letter or digit",
                    code_span(&name)
                ),
            ));
        }
        let mut origins = |list: Vec<Spanned<String>>| -> Vec<String> {
            let mut kept: Vec<String> = Vec::new();
            for source in list {
                match landing::parse_source(source.get_ref()) {
                    Ok(origin) => {
                        if !kept.contains(&origin) {
                            kept.push(origin);
                        }
                    }
                    Err(message) => errors.push(error(source.span(), "landing-source", message)),
                }
            }
            kept
        };
        let sources = Sources {
            script: origins(raw.sources.script),
            connect: origins(raw.sources.connect),
            img: origins(raw.sources.img),
            frame: origins(raw.sources.frame),
            form: origins(raw.sources.form),
        };
        let mut scripts = Vec::new();
        for script in raw.scripts {
            let span = script.src.span();
            match landing::parse_script_src(script.src.get_ref()) {
                Ok(src) => scripts.push(Script {
                    src,
                    load: script.load.unwrap_or_default(),
                    span,
                }),
                Err(message) => errors.push(error(span, "landing-script", message)),
            }
        }
        let mut carry: Vec<String> = Vec::new();
        for parameter in raw.carry {
            let value = parameter.get_ref();
            if !landing::valid_carry(value) {
                errors.push(error(
                    parameter.span(),
                    "landing-carry",
                    format!(
                        "carried parameter {} must be 1 to 64 letters, digits, `_`, `.` or `-`",
                        code_span(value)
                    ),
                ));
            } else if carry.contains(value) {
                errors.push(error(
                    parameter.span(),
                    "landing-carry",
                    format!("carry lists {} more than once", code_span(value)),
                ));
            } else {
                carry.push(value.clone());
            }
        }
        profiles.insert(
            name.clone(),
            Profile {
                name,
                scripts,
                sources,
                carry,
                footer: nav_links(raw.footer),
                span,
            },
        );
    }
    profiles
}

fn parse_locales(
    i18n: Option<Spanned<RawI18n>>,
    site_lang: &str,
    site_lang_span: Range<usize>,
    out_dir: &str,
    errors: &mut Vec<Issue>,
) -> Vec<LocaleConfig> {
    let Some(i18n) = i18n else {
        return Vec::new();
    };
    let span = i18n.span();
    let raw = i18n.into_inner();
    if raw.locales.is_empty() {
        errors.push(error(
            span,
            "i18n-config",
            "i18n.locales must declare at least the default language",
        ));
        return Vec::new();
    }
    let mut languages = BTreeSet::new();
    let mut prefixes = BTreeSet::new();
    let mut default_count = 0;
    let mut locales = Vec::new();
    for locale in raw.locales {
        let lang_span = locale.lang.span();
        let lang = locale.lang.into_inner();
        if !valid_lang(&lang) {
            errors.push(
                error(
                    lang_span.clone(),
                    "i18n-config",
                    format!("{} is not a language tag", code_span(&lang)),
                )
                .suggest("use a tag like `en` or `zh-CN`"),
            );
        }
        if !languages.insert(lang.to_ascii_lowercase()) {
            errors.push(error(
                lang_span.clone(),
                "i18n-config",
                format!("i18n.locales declares {} more than once", code_span(&lang)),
            ));
        }
        let default = lang.eq_ignore_ascii_case(site_lang);
        default_count += usize::from(default);
        let prefix_span = locale.prefix.span();
        let prefix = locale.prefix.into_inner();
        if default && !prefix.is_empty() {
            errors.push(error(
                prefix_span.clone(),
                "i18n-config",
                "the locale matching site.lang must have an empty prefix",
            ));
        } else if !default && !valid_locale_prefix(&prefix) {
            errors.push(error(
                prefix_span.clone(),
                "i18n-config",
                format!(
                    "locale prefix {} must be one URL-safe segment, outside reserved directories and site files",
                    code_span(&prefix)
                ),
            ));
        }
        if !prefixes.insert(prefix.clone()) {
            errors.push(error(
                prefix_span,
                "i18n-config",
                format!(
                    "i18n.locales uses prefix {} more than once",
                    code_span(&prefix)
                ),
            ));
        }
        let messages = locale.messages.and_then(|messages| {
            let span = messages.span();
            let path = match relative_path(messages.get_ref()) {
                Ok(path) => path,
                Err(message) => {
                    errors.push(error(
                        span,
                        "i18n-config",
                        format!("locale messages: {message}"),
                    ));
                    return None;
                }
            };
            if path.is_empty() || crate::content::is_private(&path) {
                errors.push(error(
                    span,
                    "i18n-config",
                    "locale messages must name a non-private project file",
                ));
                return None;
            }
            if out_dir.is_empty() || path == out_dir || path.starts_with(&format!("{out_dir}/")) {
                errors.push(error(
                    span,
                    "i18n-config",
                    format!(
                        "build.out would replace locale messages {}",
                        code_span(&path)
                    ),
                ));
                return None;
            }
            Some((path, span))
        });
        locales.push(LocaleConfig {
            lang,
            lang_span,
            label: locale.label,
            prefix,
            messages,
            nav: locale.nav.map(|nav| parse_nav(nav, "i18n-config", errors)),
        });
    }
    if default_count != 1 {
        errors.push(error(
            if site_lang_span.is_empty() {
                span
            } else {
                site_lang_span
            },
            "i18n-config",
            format!(
                "site.lang {} must match exactly one i18n locale",
                code_span(site_lang)
            ),
        ));
    }
    locales
}

impl Config {
    /// Builds for another base path than the configured one (`--base`).
    /// The site's URL, when set, must still end in it.
    pub fn override_base(&mut self, base: &str) -> Result<(), String> {
        let base = normalize_base(base)?;
        if let Some(url) = &self.site.url {
            if !url.ends_with(&base) {
                return Err(format!(
                    "--base {} does not match site.url {}; change both together",
                    literal(&base),
                    literal(url)
                ));
            }
        }
        self.site.base = base;
        Ok(())
    }
}

/// Reads `rustify-docs.toml`. On success, also returns the warnings; on
/// failure, every error found (an unreadable file stops at its first).
pub fn parse(text: &str) -> Result<(Config, Vec<Issue>), Vec<Issue>> {
    let deserializer = toml::Deserializer::parse(text).map_err(|e| {
        let span = e.span().unwrap_or(0..0);
        vec![error(span, "config", e.message().to_owned())]
    })?;
    let raw: RawConfig = serde_path_to_error::deserialize(deserializer).map_err(|e| {
        let rule = if matches!(e.path().iter().next(), Some(serde_path_to_error::Segment::Map { key }) if key == "i18n") {
            "i18n-config"
        } else {
            "config"
        };
        let e = e.into_inner();
        vec![error(e.span().unwrap_or(0..0), rule, e.message().to_owned())]
    })?;
    let mut errors = Vec::new();
    let warnings = Vec::new();
    if *raw.schema_version.get_ref() != SCHEMA_VERSION {
        errors.push(
            error(
                raw.schema_version.span(),
                "config-schema-version",
                format!("schema_version {} is not one this rustify-docs reads (it reads {SCHEMA_VERSION})", raw.schema_version.get_ref()),
            )
            .suggest("install the rustify-docs version the site was written for"),
        );
    }
    let mut path =
        |value: Option<Spanned<String>>, default: &str, what: &str| -> (String, Range<usize>) {
            match value {
                None => (default.to_owned(), 0..0),
                Some(value) => {
                    let span = value.span();
                    match relative_path(value.get_ref()) {
                        Ok(path) if !path.is_empty() || what == "build.out" => (path, span),
                        Ok(_) => {
                            errors.push(error(
                                span.clone(),
                                "config-path",
                                format!("{what} cannot be the project root"),
                            ));
                            (default.to_owned(), span)
                        }
                        Err(message) => {
                            errors.push(error(
                                span.clone(),
                                "config-path",
                                format!("{what}: {message}"),
                            ));
                            (default.to_owned(), span)
                        }
                    }
                }
            }
        };
    let (content_dir, _) = path(raw.content.dir, "content", "content.dir");
    let (assets_dir, _) = path(raw.assets.dir, "static", "assets.dir");
    let (out_dir, out_span) = path(raw.build.out, "dist", "build.out");
    let rust = raw.rust.map(|rust| {
        let (manifest, _) = path(Some(rust.manifest), "site/Cargo.toml", "rust.manifest");
        let (registry, _) = path(Some(rust.registry), "site/src/lib.rs", "rust.registry");
        RustConfig { manifest, registry }
    });
    let theme_document = raw
        .theme
        .document
        .map(|document| path(Some(document), "theme.json", "theme.document"));
    let content_include = raw.content.include.map(|files| {
        let mut selected = BTreeSet::new();
        files.into_iter().filter_map(|file| {
            let span = file.span();
            let literal = file.into_inner();
            if literal.contains(['*', '?', '[', ']', '{', '}']) {
                errors.push(error(span, "content-include", "content.include names literal Markdown files; glob patterns are not supported"));
                return None;
            }
            let file = match relative_path(&literal) {
                Ok(file) => file,
                Err(message) => {
                    errors.push(error(span, "content-include", format!("content.include: {message}")));
                    return None;
                }
            };
            if !file.ends_with(".md") {
                errors.push(error(span, "content-include", "content.include must name a Markdown file ending in `.md`"));
                return None;
            }
            if crate::content::is_private(&format!("{content_dir}/{file}")) {
                errors.push(error(span, "content-include", "content.include cannot select a private path"));
                return None;
            }
            if !selected.insert(file.clone()) {
                errors.push(error(span, "content-include", format!("content.include lists {} more than once", code_span(&file))));
                return None;
            }
            Some((file, span))
        }).collect()
    });
    let inside = |inner: &str, outer: &str| {
        outer.is_empty() || inner == outer || inner.starts_with(&format!("{outer}/"))
    };
    for (input, name) in [(&content_dir, "content.dir"), (&assets_dir, "assets.dir")] {
        if inside(input, &out_dir) || inside(&out_dir, input) {
            errors.push(
                error(
                    out_span.clone(),
                    "config-out",
                    format!(
                        "build.out {} overlaps {name} {}; a build replaces its output whole",
                        code_span(&out_dir),
                        code_span(input)
                    ),
                )
                .suggest("put the output in a directory of its own, like `dist`"),
            );
        }
    }
    if let Some(rust) = &rust {
        for (input, name) in [
            (&rust.manifest, "rust.manifest"),
            (&rust.registry, "rust.registry"),
        ] {
            if inside(input, &out_dir) {
                errors.push(error(
                    out_span.clone(),
                    "config-out",
                    format!(
                        "build.out {} would replace {name} {}",
                        code_span(&out_dir),
                        code_span(input)
                    ),
                ));
            }
        }
    }
    if let Some((document, span)) = &theme_document {
        if crate::content::is_private(document) {
            errors.push(error(
                span.clone(),
                "config-path",
                "theme.document is a private project path",
            ));
        }
        if inside(document, &out_dir) {
            errors.push(error(
                out_span.clone(),
                "config-out",
                "build.out would replace theme.document",
            ));
        }
        if raw.theme.primary.is_some() {
            errors.push(error(
                span.clone(),
                "config-theme",
                "theme.document and theme.primary cannot both define the site's theme",
            ));
        }
    }
    if out_dir.is_empty() {
        errors.push(error(
            out_span.clone(),
            "config-out",
            "build.out cannot be the project root",
        ));
    }

    let lang_span = raw.site.lang.as_ref().map(Spanned::span).unwrap_or(0..0);
    let lang = match raw.site.lang {
        None => "en".to_owned(),
        Some(lang) => {
            if !valid_lang(lang.get_ref()) {
                errors.push(
                    error(
                        lang.span(),
                        "config-lang",
                        format!("{} is not a language tag", code_span(lang.get_ref())),
                    )
                    .suggest("use a tag like `en` or `zh-CN`"),
                );
            }
            lang.into_inner()
        }
    };
    let locales = parse_locales(raw.i18n, &lang, lang_span, &out_dir, &mut errors);
    let base = match raw.site.base {
        None => "/".to_owned(),
        Some(base) => normalize_base(base.get_ref()).unwrap_or_else(|message| {
            errors.push(error(base.span(), "config-base", message));
            "/".into()
        }),
    };
    let url = raw.site.url.and_then(|url| {
        let value = url.get_ref().trim();
        if !absolute_url(value) || value.contains(['?', '#']) {
            errors.push(error(
                url.span(),
                "config-url",
                format!(
                    "{} is not an absolute http(s) URL without a query or fragment",
                    code_span(value)
                ),
            ));
            return None;
        }
        let value = if value.ends_with('/') {
            value.to_owned()
        } else {
            format!("{value}/")
        };
        let path_start = value
            .find("://")
            .map(|i| i + 3)
            .and_then(|i| value[i..].find('/').map(|j| i + j))
            .unwrap_or(value.len());
        if value[path_start..] != base {
            errors.push(
                error(
                    url.span(),
                    "config-url",
                    format!(
                        "site.url's path {} is not site.base {}",
                        code_span(&value[path_start..]),
                        code_span(&base)
                    ),
                )
                .suggest("site.url is the public address of the site's root, base path included"),
            );
        }
        Some(value)
    });

    let repo = raw.repo.and_then(|repo| {
        if !absolute_url(repo.url.get_ref()) {
            errors.push(error(
                repo.url.span(),
                "config-repo",
                "repo.url must be an absolute http(s) URL",
            ));
            return None;
        }
        let edit_url = repo.edit_url.and_then(|edit| {
            if absolute_url(edit.get_ref()) {
                Some(edit.into_inner())
            } else {
                errors.push(error(
                    edit.span(),
                    "config-repo",
                    "repo.edit_url must be an absolute http(s) URL",
                ));
                None
            }
        });
        let path = relative_path(&repo.path).unwrap_or_else(|message| {
            errors.push(error(0..0, "config-repo", format!("repo.path: {message}")));
            String::new()
        });
        Some(Repo {
            url: repo.url.into_inner().trim_end_matches('/').to_owned(),
            reference: repo.reference,
            path,
            edit_url,
        })
    });

    let primary = raw.theme.primary.and_then(|pair| {
        let span = pair.span();
        let pair = pair.into_inner();
        let mut good = true;
        for color in [&pair.light, &pair.dark] {
            if crate::color::parse_hex(color.get_ref()).is_none() {
                errors.push(error(
                    color.span(),
                    "config-color",
                    format!("{} is not a #RRGGBB colour", code_span(color.get_ref())),
                ));
                good = false;
            }
        }
        let _ = span;
        good.then(|| Pair {
            spans: (pair.light.span(), pair.dark.span()),
            light: pair.light.into_inner().to_ascii_lowercase(),
            dark: pair.dark.into_inner().to_ascii_lowercase(),
        })
    });

    let image = raw.site.image.and_then(|image| {
        let span = image.span();
        let path = match relative_path(image.get_ref()) {
            Ok(path) => path,
            Err(message) => {
                errors.push(error(
                    span,
                    "config-image",
                    format!("site.image: {message}"),
                ));
                return None;
            }
        };
        let extension = path
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_ascii_lowercase());
        if !matches!(
            extension.as_deref(),
            Some("png" | "jpg" | "jpeg" | "gif" | "webp")
        ) {
            errors.push(
                error(
                    span,
                    "config-image",
                    "site.image must be a PNG, JPEG, GIF or WebP image",
                )
                .suggest("link previews do not show SVG; export the image as PNG"),
            );
            return None;
        }
        Some((path, span))
    });

    let logo = raw.theme.logo.and_then(|logo| {
        let span = logo.span();
        let (light, dark) = match logo.into_inner() {
            RawLogo::One(path) => (path.clone(), path),
            RawLogo::Pair { light, dark } => (light, dark),
        };
        match (relative_path(&light), relative_path(&dark)) {
            (Ok(light), Ok(dark)) => Some(Logo { light, dark, span }),
            (Err(message), _) | (_, Err(message)) => {
                errors.push(error(span, "config-logo", format!("theme.logo: {message}")));
                None
            }
        }
    });

    let nav = parse_nav(raw.nav, "config-nav", &mut errors);
    let landing = parse_landing(raw.landing, &mut errors);
    let theme_fonts = parse_fonts(raw.theme.fonts, &out_dir, &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok((
        Config {
            site: SiteConfig {
                name: raw.site.name,
                description: raw.site.description,
                lang,
                url,
                base,
                image,
            },
            content_dir,
            content_include,
            assets_dir,
            out_dir,
            repo,
            primary,
            theme_document,
            theme_fonts,
            logo,
            nav,
            locales,
            dev_port: raw.dev.port.unwrap_or(DEFAULT_DEV_PORT),
            rust,
            landing,
        },
        warnings,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = "schema_version = 1\n[site]\nname = \"Docs\"\n";

    #[test]
    fn defaults() {
        let (config, warnings) = parse(MINIMAL).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(
            (
                config.content_dir.as_str(),
                config.assets_dir.as_str(),
                config.out_dir.as_str()
            ),
            ("content", "static", "dist")
        );
        assert_eq!(config.site.base, "/");
        assert_eq!(config.site.lang, "en");
        assert!(config.locales.is_empty());
        assert_eq!(config.dev_port, 4188);
        assert_eq!(config.primary, None);
    }

    #[test]
    fn unknown_keys_are_errors_with_a_location() {
        let text = "schema_version = 1\n[site]\nname = \"Docs\"\ncolour = \"red\"\n";
        let errors = parse(text).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].message.contains("colour"),
            "{}",
            errors[0].message
        );
        assert_eq!(&text[errors[0].span.clone()], "colour");
    }

    #[test]
    fn component_sites_accept_explicit_rust_inputs() {
        let text = format!(
            "{MINIMAL}[rust]\nmanifest = \"site/Cargo.toml\"\nregistry = \"site/src/lib.rs\"\n"
        );
        let (config, _) = parse(&text).unwrap();
        assert_eq!(
            config.rust,
            Some(RustConfig {
                manifest: "site/Cargo.toml".into(),
                registry: "site/src/lib.rs".into(),
            })
        );
    }

    #[test]
    fn static_theme_and_registered_navigation_are_known_configuration() {
        let text = format!(
            "{MINIMAL}[theme]\ndocument = \"themes/paper.json\"\n\
             [[nav.groups]]\nlabel = \"Examples\"\nroutes = [\"/second/\", \"/first/\"]\n"
        );
        assert!(
            parse(&text).is_ok(),
            "{}",
            parse(&text).unwrap_err()[0].message
        );
    }

    #[test]
    fn theme_document_rejects_conflicting_primary_and_unsafe_paths() {
        let text = format!(
            "{MINIMAL}[theme]\ndocument = \"themes/paper.json\"\n\
             primary = {{ light = \"#175cd3\", dark = \"#53b1fd\" }}\n"
        );
        let errors = parse(&text).unwrap_err();
        assert!(errors.iter().any(|issue| issue.rule == "config-theme"));
        for path in [
            "../outside.json",
            "/outside.json",
            ".git/theme.json",
            "dist/theme.json",
        ] {
            let text = format!("{MINIMAL}[theme]\ndocument = \"{path}\"\n");
            let errors = parse(&text).unwrap_err();
            assert!(
                errors
                    .iter()
                    .any(|issue| matches!(issue.rule, "config-path" | "config-out")),
                "{path}: {errors:?}"
            );
        }
    }

    #[test]
    fn theme_fonts_validate_descriptors_paths_and_duplicate_faces() {
        let declaration = "[[theme.fonts]]\nfamily='Brand Sans'\nsrc='fonts/brand.woff2'\n";
        let (config, _) = parse(&format!("{MINIMAL}{declaration}")).unwrap();
        assert_eq!(config.theme_fonts[0].weight, "400");
        assert_eq!(config.theme_fonts[0].style, "normal");
        let variable = format!("{declaration}weight='100 900'\nstyle='italic'\n");
        assert!(parse(&format!("{MINIMAL}{variable}")).is_ok());
        for extra in [
            "weight='0'",
            "weight='1001'",
            "weight='900 100'",
            "weight='400 400'",
            "weight='1 2 3'",
            "weight=400",
            "style='oblique'",
            "other='ignored'",
        ] {
            let errors = parse(&format!("{MINIMAL}{declaration}{extra}\n")).unwrap_err();
            assert!(
                errors.iter().any(|issue| issue.rule == "theme-font"),
                "{extra}: {errors:?}"
            );
        }
        for path in [
            "",
            "https://example.test/f.woff2",
            "/font.ttf",
            "../font.ttf",
            ".git/font.ttf",
            "dist/font.ttf",
            "fonts/font.svg",
        ] {
            let text = format!("{MINIMAL}[[theme.fonts]]\nfamily='Brand'\nsrc={path:?}\n");
            assert!(
                parse(&text)
                    .unwrap_err()
                    .iter()
                    .any(|issue| issue.rule == "theme-font"),
                "{path}"
            );
        }
        let duplicate = format!("{MINIMAL}{declaration}{declaration}");
        assert!(parse(&duplicate)
            .unwrap_err()
            .iter()
            .any(|issue| issue.rule == "theme-font"));
    }

    #[test]
    fn registered_navigation_requires_one_source_and_canonical_routes() {
        for extra in ["dir = \"guide\"", "items = [\"guide.md\"]"] {
            let text = format!(
                "{MINIMAL}[[nav.groups]]\nlabel = \"Examples\"\nroutes = [\"/custom/\"]\n{extra}\n"
            );
            assert!(parse(&text)
                .unwrap_err()
                .iter()
                .any(|issue| issue.rule == "config-nav"));
        }
        for route in [
            "custom/",
            "/custom",
            "/a/../b/",
            "/custom/?query",
            "/custom/#anchor",
        ] {
            let text =
                format!("{MINIMAL}[[nav.groups]]\nlabel = \"Examples\"\nroutes = [\"{route}\"]\n");
            assert!(
                parse(&text)
                    .unwrap_err()
                    .iter()
                    .any(|issue| issue.rule == "config-nav"),
                "{route}"
            );
        }
    }

    #[test]
    fn rust_inputs_cannot_escape_or_be_replaced_by_output() {
        let text = format!(
            "{MINIMAL}[rust]\nmanifest = \"../Cargo.toml\"\nregistry = \"../src/lib.rs\"\n"
        );
        let errors = parse(&text).unwrap_err();
        assert_eq!(
            errors
                .iter()
                .filter(|issue| issue.rule == "config-path")
                .count(),
            2
        );
        let text = format!(
            "{MINIMAL}[rust]\nmanifest = \"dist/Cargo.toml\"\nregistry = \"dist/src/lib.rs\"\n"
        );
        assert!(parse(&text)
            .unwrap_err()
            .iter()
            .any(|issue| issue.rule == "config-out"));
    }

    const ADS: &str = "[landing.profiles.ads]\n\
         carry = [\"gclid\", \"utm_source\"]\n\
         [[landing.profiles.ads.scripts]]\n\
         src = \"https://www.googletagmanager.com/gtag/js?id=AW-1\"\n\
         load = \"async\"\n\
         [[landing.profiles.ads.scripts]]\n\
         src = \"static/./landing/init.js\"\n\
         [landing.profiles.ads.sources]\n\
         connect = [\"https://*.Google-Analytics.com\", \"https://*.google-analytics.com:443\"]\n\
         img = [\"https://www.google.com\"]\n\
         frame = [\"https://jinshuju.net\"]\n\
         form = [\"http://127.0.0.1:4211\"]\n\
         [[landing.profiles.ads.footer]]\n\
         label = \"Privacy\"\n\
         link = \"privacy.md\"\n\
         [landing.profiles.plain]\n\
         carry = [\"gclid\"]\n";

    #[test]
    fn landing_profiles_are_parsed_and_normalized() {
        use crate::landing::{Load, ScriptSrc};
        let text = format!("{MINIMAL}{ADS}");
        let (config, warnings) = parse(&text).unwrap_or_else(|errors| panic!("{errors:?}"));
        assert!(warnings.is_empty());
        assert_eq!(config.landing.keys().collect::<Vec<_>>(), ["ads", "plain"]);
        let ads = &config.landing["ads"];
        assert_eq!(ads.name, "ads");
        assert_eq!(
            ads.scripts
                .iter()
                .map(|script| (&script.src, script.load))
                .collect::<Vec<_>>(),
            [
                (
                    &ScriptSrc::External("https://www.googletagmanager.com/gtag/js?id=AW-1".into()),
                    Load::Async
                ),
                (
                    &ScriptSrc::Local("static/landing/init.js".into()),
                    Load::Defer
                ),
            ]
        );
        assert_eq!(
            &text[ads.scripts[1].span.clone()],
            "\"static/./landing/init.js\""
        );
        assert!(ads.sources.script.is_empty());
        assert_eq!(ads.sources.connect, ["https://*.google-analytics.com"]);
        assert_eq!(ads.sources.img, ["https://www.google.com"]);
        assert_eq!(ads.sources.frame, ["https://jinshuju.net"]);
        assert_eq!(ads.sources.form, ["http://127.0.0.1:4211"]);
        assert_eq!(ads.carry, ["gclid", "utm_source"]);
        assert_eq!(ads.footer[0].link, "privacy.md");
        assert_eq!(&text[ads.span.clone()], "[landing.profiles.ads]");
        assert!(config.landing["plain"].scripts.is_empty());
        assert!(parse(MINIMAL).unwrap().0.landing.is_empty());
        assert!(parse(&format!("{MINIMAL}[landing]\n"))
            .unwrap()
            .0
            .landing
            .is_empty());
    }

    #[test]
    fn landing_profiles_reject_what_they_cannot_express() {
        for (profile, rule, token) in [
            (
                "[landing.profiles.Ads]\n",
                "landing-profile-name",
                Some("[landing.profiles.Ads]"),
            ),
            (
                "[landing.profiles.-ads]\n",
                "landing-profile-name",
                Some("[landing.profiles.-ads]"),
            ),
            (
                "[landing.profiles.ads.sources]\nconnect = [\"https://example.com/path\"]\n",
                "landing-source",
                Some("\"https://example.com/path\""),
            ),
            (
                "[landing.profiles.ads.sources]\nscript = [\"'unsafe-inline'\"]\n",
                "landing-source",
                Some("\"'unsafe-inline'\""),
            ),
            (
                "[landing.profiles.ads.sources]\nimg = [\"data:\"]\n",
                "landing-source",
                Some("\"data:\""),
            ),
            (
                "[landing.profiles.ads.sources]\nframe = [\"http://[::1]:4211\"]\n",
                "landing-source",
                Some("\"http://[::1]:4211\""),
            ),
            (
                "[landing.profiles.ads.sources]\nform = [\"https://10.0.0.1\"]\n",
                "landing-source",
                Some("\"https://10.0.0.1\""),
            ),
            (
                "[landing.profiles.ads.sources]\nconnect = [\"http://example.com\"]\n",
                "landing-source",
                Some("\"http://example.com\""),
            ),
            (
                "[[landing.profiles.ads.scripts]]\nsrc = \"http://[::1]:4211/tag.js\"\n",
                "landing-script",
                Some("\"http://[::1]:4211/tag.js\""),
            ),
            (
                "[[landing.profiles.ads.scripts]]\nsrc = \"https://192.168.0.1/tag.js\"\n",
                "landing-script",
                Some("\"https://192.168.0.1/tag.js\""),
            ),
            (
                "[[landing.profiles.ads.scripts]]\nsrc = \"javascript:alert(1)\"\n",
                "landing-script",
                Some("\"javascript:alert(1)\""),
            ),
            (
                "[[landing.profiles.ads.scripts]]\nsrc = \"../init.js\"\n",
                "landing-script",
                Some("\"../init.js\""),
            ),
            (
                "[landing.profiles.ads]\ncarry = [\"a b\"]\n",
                "landing-carry",
                Some("\"a b\""),
            ),
            (
                "[landing.profiles.ads]\ncarry = [\"gclid\", \"gclid\"]\n",
                "landing-carry",
                None,
            ),
            (
                "[landing.profiles.ads]\ninline = \"x\"\n",
                "config",
                Some("inline"),
            ),
            (
                "[landing.profiles.ads.sources]\nstyle = []\n",
                "config",
                Some("style"),
            ),
            (
                "[[landing.profiles.ads.scripts]]\nsrc = \"a.js\"\nload = \"sync\"\n",
                "config",
                Some("\"sync\""),
            ),
            ("[landing]\ndefault = \"ads\"\n", "config", Some("default")),
        ] {
            let text = format!("{MINIMAL}{profile}");
            let errors = parse(&text).unwrap_err();
            assert_eq!(errors.len(), 1, "{profile}: {errors:?}");
            assert_eq!(errors[0].rule, rule, "{profile}: {errors:?}");
            if let Some(token) = token {
                assert_eq!(&text[errors[0].span.clone()], token, "{profile}");
            } else {
                assert!(
                    errors[0].span.start >= MINIMAL.len(),
                    "{profile}: {errors:?}"
                );
            }
        }
    }

    #[test]
    fn schema_version_must_be_one() {
        let errors = parse("schema_version = 2\n[site]\nname = \"Docs\"\n").unwrap_err();
        assert_eq!(errors[0].rule, "config-schema-version");
    }

    #[test]
    fn base_and_url_normalize_together() {
        let (config, _) = parse("schema_version = 1\n[site]\nname = \"D\"\nbase = \"rustify-ui\"\nurl = \"https://example.com/rustify-ui\"\n").unwrap();
        assert_eq!(config.site.base, "/rustify-ui/");
        assert_eq!(
            config.site.url.as_deref(),
            Some("https://example.com/rustify-ui/")
        );
        for bad in ["/a b/", "/../", "/a//b?x"] {
            assert!(normalize_base(bad).is_err(), "{bad}");
        }
        let errors = parse("schema_version = 1\n[site]\nname = \"D\"\nbase = \"/docs/\"\nurl = \"https://example.com/\"\n").unwrap_err();
        assert_eq!(errors[0].rule, "config-url");
    }

    #[test]
    fn output_may_not_overlap_inputs() {
        for out in ["content", "content/dist", ".", "static"] {
            let text =
                format!("schema_version = 1\n[site]\nname = \"D\"\n[build]\nout = \"{out}\"\n");
            let errors = parse(&text).unwrap_err();
            assert!(
                errors
                    .iter()
                    .any(|e| e.rule == "config-out" || e.rule == "config-path"),
                "{out}: {errors:?}"
            );
        }
        assert!(
            parse("schema_version = 1\n[site]\nname = \"D\"\n[build]\nout = \"../outside\"\n")
                .is_err()
        );
    }

    #[test]
    fn brand_colours_and_logo() {
        let (config, _) = parse("schema_version = 1\n[site]\nname = \"D\"\n[theme]\nprimary = { light = \"#175CD3\", dark = \"#53b1fd\" }\nlogo = { light = \"static/l.svg\", dark = \"static/d.svg\" }\n").unwrap();
        let primary = config.primary.unwrap();
        assert_eq!(
            (primary.light.as_str(), primary.dark.as_str()),
            ("#175cd3", "#53b1fd")
        );
        let logo = config.logo.unwrap();
        assert_eq!(
            (logo.light.as_str(), logo.dark.as_str()),
            ("static/l.svg", "static/d.svg")
        );
        let (config, _) = parse(
            "schema_version = 1\n[site]\nname = \"D\"\n[theme]\nlogo = \"static/logo.svg\"\n",
        )
        .unwrap();
        assert_eq!(config.logo.unwrap().dark, "static/logo.svg");
        let errors = parse("schema_version = 1\n[site]\nname = \"D\"\n[theme]\nprimary = { light = \"#175cd3\" }\n").unwrap_err();
        assert!(errors[0].message.contains("dark"), "{errors:?}");
        let errors = parse("schema_version = 1\n[site]\nname = \"D\"\n[theme]\nprimary = { light = \"blue\", dark = \"#000000\" }\n").unwrap_err();
        assert_eq!(errors[0].rule, "config-color");
    }

    #[test]
    fn nav_groups_need_one_source() {
        let (config, _) = parse("schema_version = 1\n[site]\nname = \"D\"\n[[nav.groups]]\nlabel = \"Guide\"\ndir = \"guide\"\nscope = \"/guide\"\n[[nav.groups]]\nlabel = \"More\"\nitems = [\"a.md\", \"b/c.md\"]\n").unwrap();
        assert_eq!(config.nav.groups[0].scope, "/guide/");
        assert!(
            matches!(&config.nav.groups[0].source, GroupSource::Dir { dir, .. } if dir == "guide")
        );
        assert!(
            matches!(&config.nav.groups[1].source, GroupSource::Items(items) if items.len() == 2)
        );
        assert!(
            parse("schema_version = 1\n[site]\nname = \"D\"\n[[nav.groups]]\nlabel = \"G\"\n")
                .is_err()
        );
    }

    #[test]
    fn languages() {
        for good in ["en", "zh-CN", "zh-Hant-TW", "fil"] {
            assert!(valid_lang(good), "{good}");
        }
        for bad in ["", "e", "english", "zh_CN", "en-", "en-toolongtag"] {
            assert!(!valid_lang(bad), "{bad}");
        }
    }

    #[test]
    fn locales_accept_declared_languages_messages_and_navigation() {
        let text = format!(
            "{MINIMAL}lang = \"EN\"\n\
             [[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n\
             [[i18n.locales]]\nlang = \"zh-Hant\"\nlabel = \"中文\"\nprefix = \"zh\"\nmessages = \"locales/./zh.toml\"\n\
             [i18n.locales.nav]\ngroups = [{{ label = \"指南\", items = [\"guide/start.md\"], scope = \"/guide\" }}]\n\
             top = [{{ label = \"首页\", link = \"/\" }}]\nsocial = [{{ label = \"Code\", link = \"https://example.com\" }}]\n"
        );
        let (config, warnings) = parse(&text).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(config.site.lang, "EN");
        let locale = &config.locales[1];
        assert_eq!(
            (
                locale.lang.as_str(),
                locale.label.as_str(),
                locale.prefix.as_str()
            ),
            ("zh-Hant", "中文", "zh")
        );
        assert_eq!(&text[locale.lang_span.clone()], "\"zh-Hant\"");
        let (messages, span) = locale.messages.as_ref().unwrap();
        assert_eq!(messages, "locales/zh.toml");
        assert_eq!(&text[span.clone()], "\"locales/./zh.toml\"");
        let nav = locale.nav.as_ref().unwrap();
        assert_eq!(nav.groups[0].scope, "/guide/");
        std::assert_matches!(
            &nav.groups[0].source,
            GroupSource::Items(items) if items[0].0 == "guide/start.md"
        );
        assert_eq!(nav.top[0].link, "/");
        assert_eq!(nav.social[0].link, "https://example.com");
    }

    #[test]
    fn enabled_i18n_requires_nonempty_locales() {
        for declaration in ["[i18n]\n", "[i18n]\nlocales = []\n"] {
            let text = format!("{MINIMAL}{declaration}");
            let errors = parse(&text).unwrap_err();
            assert_eq!(errors[0].rule, "i18n-config", "{errors:?}");
            assert!(errors[0].span.start > 0, "{errors:?}");
        }
    }

    #[test]
    fn locales_require_the_site_language_and_only_its_empty_prefix() {
        for declaration in [
            "lang = \"fr\"\nlabel = \"Français\"\nprefix = \"fr\"",
            "lang = \"en\"\nlabel = \"English\"\nprefix = \"en\"",
            "lang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n[[i18n.locales]]\nlang = \"fr\"\nlabel = \"Français\"\nprefix = \"\"",
        ] {
            let text = format!("{MINIMAL}[[i18n.locales]]\n{declaration}\n");
            let errors = parse(&text).unwrap_err();
            assert!(
                errors.iter().all(|issue| issue.rule == "i18n-config"),
                "{errors:?}"
            );
            assert!(errors.iter().all(|issue| issue.span.start > 0), "{errors:?}");
        }
    }

    #[test]
    fn locales_reject_invalid_or_case_insensitively_duplicate_tags() {
        for lang in ["bad_tag", "EN"] {
            let text = format!(
                "{MINIMAL}[[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n\
                 [[i18n.locales]]\nlang = \"{lang}\"\nlabel = \"Other\"\nprefix = \"other\"\n"
            );
            let errors = parse(&text).unwrap_err();
            let issue = errors
                .iter()
                .find(|issue| issue.rule == "i18n-config")
                .unwrap_or_else(|| panic!("{errors:?}"));
            assert_eq!(&text[issue.span.clone()], format!("\"{lang}\""));
        }
    }

    #[test]
    fn locale_prefixes_reject_unsafe_or_reserved_segments() {
        for prefix in [
            " ",
            " zh",
            ".",
            "..",
            "/zh",
            "zh/",
            "a/b",
            "a%2fb",
            "a?b",
            "a#b",
            "a:b",
            "zh\\\\fr",
            "assets",
            "404",
            "index.html",
            "404.html",
            "robots.txt",
            "sitemap.xml",
            "rustify-docs-manifest.json",
            ".nojekyll",
            ".git",
            ".rustify-docs",
            ".env",
        ] {
            let text = format!(
                "{MINIMAL}[[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n\
                 [[i18n.locales]]\nlang = \"fr\"\nlabel = \"Français\"\nprefix = \"{prefix}\"\n"
            );
            let errors = parse(&text).unwrap_err();
            assert!(
                errors.iter().any(|issue| issue.rule == "i18n-config"
                    && text[issue.span.clone()] == format!("\"{prefix}\"")),
                "{prefix}: {errors:?}"
            );
        }
    }

    #[test]
    fn locale_prefixes_are_unique() {
        let text = format!(
            "{MINIMAL}[[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n\
             [[i18n.locales]]\nlang = \"zh\"\nlabel = \"中文\"\nprefix = \"other\"\n\
             [[i18n.locales]]\nlang = \"fr\"\nlabel = \"Français\"\nprefix = \"other\"\n"
        );
        let errors = parse(&text).unwrap_err();
        assert!(errors
            .iter()
            .any(|issue| issue.rule == "i18n-config" && &text[issue.span.clone()] == "\"other\""));
    }

    #[test]
    fn locale_messages_stay_inside_safe_project_inputs() {
        for messages in [
            "../fr.toml",
            "/fr.toml",
            ".git/fr.toml",
            "locales/.env",
            "dist/fr.toml",
            ".",
        ] {
            let text = format!(
                "{MINIMAL}[[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\nmessages = \"{messages}\"\n"
            );
            let errors = parse(&text).unwrap_err();
            assert!(
                errors.iter().any(|issue| issue.rule == "i18n-config"
                    && text[issue.span.clone()] == format!("\"{messages}\"")),
                "{messages}: {errors:?}"
            );
        }
    }

    #[test]
    fn locale_navigation_uses_the_existing_rules_and_reports_its_source() {
        for nav in [
            "groups = [{ label = \"G\" }]",
            "groups = [{ label = \"G\", dir = \"../escape\" }]",
            "groups = [{ label = \"G\", items = [], routes = [] }]",
            "groups = [{ label = \"G\", routes = [\"/invalid\"] }]",
            "groups = [{ label = \"G\", items = [], scope = \"/invalid?query\" }]",
            "unknown = true",
            "top = [{ label = \"L\", link = 3 }]",
        ] {
            let text = format!(
                "{MINIMAL}[[i18n.locales]]\nlang = \"en\"\nlabel = \"English\"\nprefix = \"\"\n[i18n.locales.nav]\n{nav}\n"
            );
            let errors = parse(&text).unwrap_err();
            assert!(
                errors.iter().all(|issue| issue.rule == "i18n-config"),
                "{nav}: {errors:?}"
            );
            assert!(
                errors.iter().all(|issue| issue.span.start > 0),
                "{errors:?}"
            );
        }
    }

    #[test]
    fn locale_schema_errors_keep_the_original_token_location() {
        for (declaration, token) in [
            ("[i18n]\nunknown = true\n", "unknown"),
            ("[i18n]\nlocales = 3\n", "3"),
            (
                "[[i18n.locales]]\nlang = 3\nlabel = 'English'\nprefix = ''\n",
                "3",
            ),
            (
                "[[i18n.locales]]\nlang = 'en'\nlabel = 'English'\nprefix = ''\nmessages = 3\n",
                "3",
            ),
        ] {
            let text = format!("{MINIMAL}{declaration}");
            let errors = parse(&text).unwrap_err();
            assert_eq!(errors[0].rule, "i18n-config", "{errors:?}");
            assert_eq!(&text[errors[0].span.clone()], token, "{errors:?}");
        }
    }
}
