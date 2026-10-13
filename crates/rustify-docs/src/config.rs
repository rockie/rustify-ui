//! `rustify-docs.toml`, schema version 1.
//!
//! Every key is known: an unknown one is an error rather than a setting that
//! silently does nothing, so a future version can add keys without an old
//! site meaning something else by them. Paths are relative to the project
//! root (the directory holding the file); the URL base is normalized once
//! here and every URL the build writes goes through it.

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
    content: RawDir,
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
    primary: Option<Spanned<RawPair>>,
    logo: Option<Spanned<RawLogo>>,
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
    groups: Vec<RawGroup>,
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
    pub assets_dir: String,
    pub out_dir: String,
    pub repo: Option<Repo>,
    pub primary: Option<Pair>,
    pub logo: Option<Logo>,
    pub nav: Nav,
    pub dev_port: u16,
    pub rust: Option<RustConfig>,
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
            "`{path}` is absolute; paths are relative to the project root"
        ));
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(format!("`{path}` leaves the project"));
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
            return Err(format!(
                "`{base}` is not a base path; use `/` or segments of letters, digits, `-`, `_`, `.` and `~`, like `/docs/`"
            ));
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

fn absolute_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://"))
        && url.len() > url.find("://").unwrap_or(0) + 3
        && !url.contains(char::is_whitespace)
}

impl Config {
    /// Builds for another base path than the configured one (`--base`).
    /// The site's URL, when set, must still end in it.
    pub fn override_base(&mut self, base: &str) -> Result<(), String> {
        let base = normalize_base(base)?;
        if let Some(url) = &self.site.url {
            if !url.ends_with(&base) {
                return Err(format!(
                    "--base {base} does not match site.url {url}; change both together"
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
    let raw: RawConfig = toml::from_str(text).map_err(|e| {
        let span = e.span().unwrap_or(0..0);
        vec![error(span, "config", e.message().to_owned())]
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
    let inside = |inner: &str, outer: &str| {
        outer.is_empty() || inner == outer || inner.starts_with(&format!("{outer}/"))
    };
    for (input, name) in [(&content_dir, "content.dir"), (&assets_dir, "assets.dir")] {
        if inside(input, &out_dir) || inside(&out_dir, input) {
            errors.push(
                error(out_span.clone(), "config-out", format!("build.out `{out_dir}` overlaps {name} `{input}`; a build replaces its output whole"))
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
                    format!("build.out `{out_dir}` would replace {name} `{input}`"),
                ));
            }
        }
    }
    if out_dir.is_empty() {
        errors.push(error(
            out_span.clone(),
            "config-out",
            "build.out cannot be the project root",
        ));
    }

    let lang = match raw.site.lang {
        None => "en".to_owned(),
        Some(lang) => {
            if !valid_lang(lang.get_ref()) {
                errors.push(
                    error(
                        lang.span(),
                        "config-lang",
                        format!("`{}` is not a language tag", lang.get_ref()),
                    )
                    .suggest("use a tag like `en` or `zh-CN`"),
                );
            }
            lang.into_inner()
        }
    };
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
                format!("`{value}` is not an absolute http(s) URL without a query or fragment"),
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
                        "site.url's path `{}` is not site.base `{base}`",
                        &value[path_start..]
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
                    format!("`{}` is not a #RRGGBB colour", color.get_ref()),
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

    let mut groups = Vec::new();
    for group in raw.nav.groups {
        let scope = match &group.scope {
            None => "/".to_owned(),
            Some(scope) => match normalize_base(scope.get_ref()) {
                Ok(scope) => scope,
                Err(_) => {
                    errors.push(error(
                        scope.span(),
                        "config-nav",
                        format!(
                            "nav scope `{}` is not a route prefix like `/guide/`",
                            scope.get_ref()
                        ),
                    ));
                    continue;
                }
            },
        };
        let source = match (group.dir, group.items) {
            (Some(dir), None) => match relative_path(dir.get_ref()) {
                Ok(path) => GroupSource::Dir {
                    dir: path,
                    span: dir.span(),
                },
                Err(message) => {
                    errors.push(error(
                        dir.span(),
                        "config-nav",
                        format!("nav group `{}`: {message}", group.label),
                    ));
                    continue;
                }
            },
            (None, Some(items)) => {
                let mut list = Vec::new();
                for item in items {
                    match relative_path(item.get_ref()) {
                        Ok(path) => list.push((path, item.span())),
                        Err(message) => errors.push(error(item.span(), "config-nav", message)),
                    }
                }
                GroupSource::Items(list)
            }
            _ => {
                errors.push(error(
                    0..0,
                    "config-nav",
                    format!(
                        "nav group `{}` needs exactly one of `dir` and `items`",
                        group.label
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
    let links = |links: Vec<RawLink>| -> Vec<NavLink> {
        links
            .into_iter()
            .map(|link| NavLink {
                label: link.label,
                span: link.link.span(),
                link: link.link.into_inner(),
            })
            .collect()
    };
    let nav = Nav {
        groups,
        top: links(raw.nav.top),
        social: links(raw.nav.social),
    };

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
            assets_dir,
            out_dir,
            repo,
            primary,
            logo,
            nav,
            dev_port: raw.dev.port.unwrap_or(DEFAULT_DEV_PORT),
            rust,
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
}
