//! Static projection of imported themes and project-owned local fonts.

use rustify_docs::content::{is_private, Diagnostic, Project, Snapshot};
use rustify_docs::diagnostic::{Issue, Severity};
use rustify_docs::diagnostic_text::{code_span, literal};
use rustify_ui::theme::{
    import_json, parse_css_font_stack, resolve, FontResolution, FontResourceDigest, ResolveContext,
    ResolvedTheme, ThemeDocument, ThemeMode, FONT_RESOURCE_DIGESTS, THEME_INPUT_LIMIT,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

const V4_MAPPINGS: &str = rustify_components::styles::THEME_V4;
const LIGHT: &str = ":root[data-rustify-scope], :root[data-rustify-scope] [data-rustify-scope]";
const DARK: &str =
    ":root[data-rustify-scope][data-theme=\"dark\"], :root[data-rustify-scope][data-theme=\"dark\"] [data-rustify-scope]";
const SYSTEM: &str =
    ":root[data-rustify-scope][data-theme=\"system\"], :root[data-rustify-scope][data-theme=\"system\"] [data-rustify-scope]";

type FontFiles = BTreeMap<String, Arc<[u8]>>;

pub(super) struct InputFile {
    pub path: String,
    pub bytes: Arc<[u8]>,
    real: PathBuf,
}

pub(super) struct PreparedTheme {
    pub document: Option<ThemeDocument>,
    pub inputs: Vec<InputFile>,
    pub utilities: String,
    pub css: String,
    pub files: FontFiles,
}

// This describes the SDK's logical catalogue, not bytes embedded in this CLI.
pub(super) fn font_catalog() -> &'static [FontResourceDigest] {
    &FONT_RESOURCE_DIGESTS
}

fn css_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

pub(super) fn prepare(project: &Project) -> (Option<PreparedTheme>, Vec<Diagnostic>) {
    let mut prepared = PreparedTheme {
        document: None,
        inputs: Vec::new(),
        utilities: String::new(),
        css: String::new(),
        files: FontFiles::new(),
    };
    let mut diagnostics = Vec::new();
    for (index, font) in project.config.theme_fonts.iter().enumerate() {
        let label = format!("theme.fonts[{index}]");
        let read = input_path(project, &font.src, &label).and_then(|real| {
            let bytes = fs::read(&real).map_err(|error| {
                format!("cannot read {label} {}: {error}", code_span(&font.src))
            })?;
            if bytes.is_empty() {
                return Err(format!("{label} {} is empty", code_span(&font.src)));
            }
            Ok((real, Arc::<[u8]>::from(bytes)))
        });
        let (real, bytes) = match read {
            Ok(read) => read,
            Err(message) => {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::error(font.span.clone(), "theme-font", message),
                ));
                continue;
            }
        };
        let extension = Path::new(&font.src)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let format = match extension.as_str() {
            "woff2" => "woff2",
            "woff" => "woff",
            "ttf" => "truetype",
            "otf" => "opentype",
            _ => {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::error(
                        font.span.clone(),
                        "theme-font",
                        format!("{label} has an unsupported font extension"),
                    ),
                ));
                continue;
            }
        };
        let path = format!("assets/fonts/{:x}.{extension}", Sha256::digest(&bytes));
        let url = format!("{}{path}", project.config.site.base);
        prepared.css.push_str(&format!(
            "@font-face {{font-family:{};src:url({}) format(\"{format}\");font-weight:{};font-style:{};font-display:swap;}}\n",
            css_string(&font.family), css_string(&url), font.weight, font.style,
        ));
        prepared.files.insert(path, Arc::clone(&bytes));
        prepared.inputs.push(InputFile {
            path: font.src.clone(),
            bytes,
            real,
        });
    }
    if let Some((path, span)) = &project.config.theme_document {
        let read = document_path(project, path)
            .and_then(|real| read_document(project, path).map(|text| (real, text)));
        let (real, input) = match read {
            Ok(read) => read,
            Err(message) => {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::error(span.clone(), "theme-document", message),
                ));
                return (None, diagnostics);
            }
        };
        let context = ResolveContext {
            font_resolution: FontResolution::Css,
            ..Default::default()
        };
        let imported = match import_json(&input, &context) {
            Ok(imported) => imported,
            Err(error) => {
                diagnostics.push(Diagnostic::in_file(
                    path,
                    Severity::Error,
                    "theme-document",
                    error.to_string(),
                ));
                return (None, diagnostics);
            }
        };
        diagnostics.extend(imported.diagnostics.into_iter().map(|warning| {
            Diagnostic::in_file(
                path,
                Severity::Warning,
                "theme-token",
                format!("{}: {}", literal(&warning.field), warning.message),
            )
        }));
        let themes = [ThemeMode::Light, ThemeMode::Dark]
            .map(|mode| resolve(&imported.document, mode, &context));
        let [Ok(light), Ok(dark)] = themes else {
            diagnostics.extend(themes.into_iter().filter_map(Result::err).map(|error| {
                Diagnostic::in_file(path, Severity::Error, "theme-document", error.to_string())
            }));
            return (None, diagnostics);
        };
        let themes = [light, dark];
        let declared: BTreeSet<_> = project
            .config
            .theme_fonts
            .iter()
            .map(|font| font.family.to_lowercase())
            .collect();
        let mut system = BTreeMap::new();
        for stack in themes
            .iter()
            .flat_map(|theme| &theme.fonts)
            .filter_map(|font| font.css_stack.as_deref())
        {
            if let Ok(families) = parse_css_font_stack(stack) {
                for (family, generic) in families {
                    let key = family.to_lowercase();
                    if !generic && !declared.contains(&key) {
                        system.entry(key).or_insert(family);
                    }
                }
            }
        }
        for family in system.into_values() {
            diagnostics.push(Diagnostic::in_file(path, Severity::Warning, "theme-font-system",
                format!("Font {} is not declared in theme.fonts; the browser uses installed fonts and CSS fallback. Built-in CJK/emoji fallback fonts are no longer shipped. Declare a licensed local font or select a generic font stack.", literal(&family))));
        }
        prepared.css.push_str(&project_css(&themes));
        prepared.utilities = V4_MAPPINGS.to_owned();
        prepared.inputs.push(InputFile {
            path: path.clone(),
            bytes: Arc::from(input.as_bytes()),
            real,
        });
        prepared.document = Some(imported.document);
    }
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
    {
        return (None, diagnostics);
    }
    if prepared.document.is_none() && project.config.theme_fonts.is_empty() {
        (None, diagnostics)
    } else {
        (Some(prepared), diagnostics)
    }
}

/// Bind every parsed document/font read to the generation's snapshot and real target.
pub(super) fn validate_snapshot(
    project: &Project,
    snapshot: &Snapshot,
    theme: &PreparedTheme,
) -> Vec<Diagnostic> {
    theme
        .inputs
        .iter()
        .filter_map(|input| {
            let current = input_path(project, &input.path, "theme input").and_then(|real| {
                if real != input.real {
                    return Err("theme input target changed during generation".into());
                }
                fs::read(real).map_err(|error| error.to_string())
            });
            if snapshot
                .files
                .get(&input.path)
                .is_some_and(|bytes| bytes == &input.bytes)
                && current.is_ok_and(|bytes| bytes.as_slice() == input.bytes.as_ref())
            {
                None
            } else {
                Some(Diagnostic::in_file(
                    &input.path,
                    Severity::Error,
                    "rust-generation-changed",
                    "theme input changed during generation; retry with one input snapshot",
                ))
            }
        })
        .collect()
}

fn input_path(project: &Project, relative: &str, label: &str) -> Result<PathBuf, String> {
    if relative.is_empty()
        || Path::new(relative)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        || is_private(relative)
    {
        return Err(format!(
            "{label} must name a non-private file inside the project"
        ));
    }
    let root = fs::canonicalize(&project.root)
        .map_err(|error| format!("cannot resolve the theme's project root: {error}"))?;
    let path = fs::canonicalize(root.join(relative))
        .map_err(|error| format!("cannot read {label} {}: {error}", code_span(relative)))?;
    let canonical_relative = path
        .strip_prefix(&root)
        .map_err(|_| format!("{label} symlink leads outside the project"))?
        .to_string_lossy()
        .replace('\\', "/");
    let output = &project.config.out_dir;
    if is_private(&canonical_relative)
        || canonical_relative == *output
        || canonical_relative.starts_with(&format!("{output}/"))
    {
        return Err(format!(
            "{label} must not read private files or generated output"
        ));
    }
    if !path.is_file() {
        return Err(format!("{label} {} is not a file", code_span(relative)));
    }
    Ok(path)
}

fn document_path(project: &Project, relative: &str) -> Result<PathBuf, String> {
    input_path(project, relative, "theme.document")
}

fn read_document(project: &Project, relative: &str) -> Result<String, String> {
    let path = document_path(project, relative)?;
    let file = fs::File::open(path).map_err(|error| {
        format!(
            "cannot read theme.document {}: {error}",
            code_span(relative)
        )
    })?;
    let mut bytes = Vec::new();
    file.take((THEME_INPUT_LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > THEME_INPUT_LIMIT {
        return Err("theme input exceeds 256 KiB".into());
    }
    String::from_utf8(bytes).map_err(|error| format!("theme.document must be UTF-8: {error}"))
}

fn project_css(themes: &[ResolvedTheme; 2]) -> String {
    let mut css = String::new();
    properties_rule(&mut css, LIGHT, &themes[0]);
    properties_rule(&mut css, DARK, &themes[1]);
    css.push_str("@media (prefers-color-scheme: dark) {\n");
    properties_rule(&mut css, SYSTEM, &themes[1]);
    css.push_str("}\n@layer utilities {\n");
    css.push_str(&format!(":where({LIGHT}) {{font-family:var(--rustify-font-sans);font-size:var(--font-size);letter-spacing:var(--rustify-letter-spacing);}}\n"));
    css.push_str(":root :where(button,input,textarea,select) {font-family:inherit;letter-spacing:inherit;}\n}\n");
    css
}

fn properties_rule(css: &mut String, selector: &str, theme: &ResolvedTheme) {
    css.push_str(selector);
    css.push_str(" {\n");
    for (name, value) in theme.properties() {
        css.push_str(&format!("  {name}:{value};\n"));
    }
    css.push_str("  --rd-radius:var(--radius);\n");
    css.push_str("}\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::theme::{dom_font_stack, font_context, ResolveContext, Theme, ThemeValues};
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "rustify-docs-theme-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&root).unwrap();
            Self(fs::canonicalize(root).unwrap())
        }

        fn write(&self, relative: &str, bytes: impl AsRef<[u8]>) {
            let path = self.0.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }

        fn project(&self, document: Option<&str>) -> Project {
            let mut text =
                "schema_version = 1\n[site]\nname = \"Theme test\"\nbase = \"/sub/\"\n".to_owned();
            if let Some(path) = document {
                text.push_str(&format!("[theme]\ndocument = {path:?}\n"));
            }
            let (config, _) = rustify_docs::config::parse(&text).unwrap();
            Project {
                root: self.0.clone(),
                config_file: "rustify-docs.toml".into(),
                config,
                config_text: text,
                inputs: Default::default(),
            }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn document() -> ThemeDocument {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for values in [&mut document.styles.light, &mut document.styles.dark] {
            for (token, family) in [
                ("font-sans", "sans-serif"),
                ("font-serif", "serif"),
                ("font-mono", "monospace"),
            ] {
                values.0.insert(token.into(), family.into());
            }
        }
        document
    }

    fn write_document(fixture: &Fixture, document: &ThemeDocument) -> Project {
        fixture.write("theme.json", serde_json::to_vec_pretty(document).unwrap());
        fixture.project(Some("theme.json"))
    }

    #[test]
    fn no_import_does_not_prepare_css_or_font_files() {
        let fixture = Fixture::new();
        let (prepared, diagnostics) = prepare(&fixture.project(None));
        assert!(prepared.is_none());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn local_fonts_register_without_selecting_a_theme_and_deduplicate_bytes() {
        let fixture = Fixture::new();
        fixture.write("fonts/brand.woff2", b"author font bytes");
        let mut project = fixture.project(None);
        project.config_text.push_str("[[theme.fonts]]\nfamily='Brand Sans'\nsrc='fonts/brand.woff2'\nweight='100 900'\n[[theme.fonts]]\nfamily='Other Name'\nsrc='fonts/brand.woff2'\n");
        project.config = rustify_docs::config::parse(&project.config_text).unwrap().0;
        let (prepared, diagnostics) = prepare(&project);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let prepared = prepared.expect("declared fonts must be registered without a document");
        assert_eq!(prepared.files.len(), 1);
        assert_eq!(prepared.css.matches("@font-face").count(), 2);
        assert!(prepared.css.contains("format(\"woff2\")"));
        assert!(prepared.css.contains("font-weight:100 900"));
        assert!(!prepared.css.contains("--rustify-font-sans"));
        assert!(prepared.css.contains("/sub/assets/fonts/"));
    }

    #[test]
    fn the_complete_example_matches_legacy_tokens_with_system_stacks_in_both_modes() {
        let input = include_str!("../templates/theme.example.json");
        let context = ResolveContext {
            font_resolution: FontResolution::Css,
            ..Default::default()
        };
        let imported = import_json(input, &context).unwrap();
        let mut expected = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for values in [&mut expected.styles.light, &mut expected.styles.dark] {
            for (token, generic) in [
                ("font-sans", "sans-serif"),
                ("font-serif", "serif"),
                ("font-mono", "monospace"),
            ] {
                values.0.insert(token.into(), generic.into());
            }
        }
        assert_eq!(imported.document.styles, expected.styles);
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let resolved = resolve(&imported.document, mode, &context).unwrap();
            assert!(resolved
                .fonts
                .iter()
                .all(|font| font.resource_path.is_none()));
        }
    }

    #[test]
    fn font_reads_are_guarded_against_changes_and_family_names_are_css_strings() {
        let fixture = Fixture::new();
        fixture.write("fonts/brand.ttf", b"first font bytes");
        let mut project = fixture.project(None);
        let family = "Brand\"; } :root { color: red; } \\";
        project.config_text.push_str(&format!(
            "[[theme.fonts]]\nfamily={}\nsrc='fonts/brand.ttf'\n",
            serde_json::to_string(family).unwrap()
        ));
        project.config = rustify_docs::config::parse(&project.config_text).unwrap().0;
        let (prepared, diagnostics) = prepare(&project);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let prepared = prepared.unwrap();
        assert!(prepared
            .css
            .contains(&format!("font-family:{};src:", css_string(family))));
        let (snapshot, _) = rustify_docs::content::snapshot(&project);
        assert!(validate_snapshot(&project, &snapshot, &prepared).is_empty());
        fixture.write("fonts/brand.ttf", b"changed font bytes");
        assert_eq!(
            validate_snapshot(&project, &snapshot, &prepared)[0].rule,
            "rust-generation-changed"
        );
    }

    #[test]
    fn input_receipt_keeps_the_exact_imported_bytes_after_the_file_changes() {
        let fixture = Fixture::new();
        let mut original = document();
        original.name = "Light 亮 and dark".into();
        let json = serde_json::to_string_pretty(&original).unwrap();
        let input = format!("\n \t{}\r\n", json.replace('\n', "\r\n"));
        fixture.write("themes/current.json", &input);
        let (prepared, diagnostics) = prepare(&fixture.project(Some("themes/current.json")));
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let prepared = prepared.unwrap();
        assert_eq!(prepared.inputs[0].path, "themes/current.json");
        assert_eq!(prepared.inputs[0].bytes.as_ref(), input.as_bytes());
        assert_eq!(prepared.document, Some(original));

        fixture.write("themes/current.json", b"new bytes after the imported read");
        assert_eq!(prepared.inputs[0].bytes.as_ref(), input.as_bytes());
        assert_ne!(
            prepared.inputs[0].bytes.as_ref(),
            fs::read(fixture.0.join("themes/current.json")).unwrap()
        );
    }

    #[test]
    fn light_dark_and_system_project_the_resolvers_validated_properties() {
        let fixture = Fixture::new();
        let mut document = document();
        document
            .styles
            .light
            .0
            .insert("primary".into(), "#102030".into());
        document
            .styles
            .dark
            .0
            .insert("primary".into(), "#fedcba".into());
        for values in [&mut document.styles.light, &mut document.styles.dark] {
            for (token, value) in [
                ("radius", "10px"),
                ("spacing", "0.375rem"),
                ("font-size", "18px"),
                ("letter-spacing", "0.025em"),
            ] {
                values.0.insert(token.into(), value.into());
            }
        }
        let project = write_document(&fixture, &document);
        let (prepared, diagnostics) = prepare(&project);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let prepared = prepared.unwrap();
        assert_eq!(prepared.document, Some(document.clone()));
        let mut expected = String::new();
        let context = ResolveContext {
            font_resolution: FontResolution::Css,
            ..Default::default()
        };
        properties_rule(
            &mut expected,
            LIGHT,
            &resolve(&document, ThemeMode::Light, &context).unwrap(),
        );
        properties_rule(
            &mut expected,
            DARK,
            &resolve(&document, ThemeMode::Dark, &context).unwrap(),
        );
        assert!(prepared.css.contains(&expected));
        let system = prepared
            .css
            .split("@media (prefers-color-scheme: dark) {")
            .nth(1)
            .unwrap();
        assert!(system.starts_with(&format!("\n{SYSTEM} {{")));
        assert!(system.contains("--primary:rgb(254 220 186)"));
        assert!(prepared.css.contains("--primary:rgb(16 32 48)"));
        assert!(prepared.css.contains("--rustify-radius-md:8px"));
        assert!(prepared.css.contains("--rustify-unit:6px"));
        assert!(prepared.css.contains("--font-size:18px"));
        assert!(prepared.css.contains("--rustify-letter-spacing:0.025em"));
        assert!(prepared
            .utilities
            .contains("--spacing: var(--rustify-unit, 0.25rem)"));
        assert_eq!(prepared.utilities, V4_MAPPINGS);
        assert!(!prepared.css.contains("@theme"));
        assert!(!prepared.css.contains("makepad"));
        assert!(!prepared.css.contains("theme.json"));
    }

    #[test]
    fn unknown_tokens_and_font_fallbacks_keep_the_resolvers_diagnostics() {
        let fixture = Fixture::new();
        let mut document = document();
        document.styles.light.0.insert(
            "unknown-token".into(),
            "url(https://untrusted.invalid/injected)".into(),
        );
        document
            .styles
            .light
            .0
            .insert("font-sans".into(), "Unknown Family".into());
        let (prepared, diagnostics) = prepare(&write_document(&fixture, &document));
        let prepared = prepared.unwrap();
        assert!(diagnostics.iter().all(|warning| {
            warning.severity == Severity::Warning && warning.file.as_deref() == Some("theme.json")
        }));
        assert!(diagnostics
            .iter()
            .any(|warning| warning.message.contains("unknown-token")));
        assert!(diagnostics.iter().any(|warning| {
            warning.message.contains("Unknown Family") && warning.rule == "theme-font-system"
        }));
        assert_eq!(
            prepared.document.as_ref().unwrap().styles.light.0["unknown-token"],
            "url(https://untrusted.invalid/injected)"
        );
        assert!(!prepared.css.contains("untrusted"));
        assert!(prepared.css.contains("Unknown Family"));
    }

    #[test]
    fn generic_unpackaged_fonts_preserve_the_sdk_stack_and_emit_no_fonts() {
        let mut document = document();
        for values in [&mut document.styles.light, &mut document.styles.dark] {
            for (token, family) in [
                ("font-sans", "sans-serif"),
                ("font-serif", "serif"),
                ("font-mono", "monospace"),
            ] {
                values.0.insert(token.into(), family.into());
            }
        }
        let context = ResolveContext::default();
        let themes = [ThemeMode::Light, ThemeMode::Dark]
            .map(|mode| resolve(&document, mode, &context).unwrap());
        let css = project_css(&themes);
        assert!(!css.contains("@font-face"));
        for (slot, font) in ["sans", "serif", "mono"].into_iter().zip(&themes[0].fonts) {
            assert!(css.contains(&format!("--rustify-font-{slot}:{};", dom_font_stack(font))));
            assert!(!dom_font_stack(font).contains("Noto Color Emoji"));
        }
    }

    #[test]
    fn invalid_tokens_duplicates_and_oversize_json_fail_before_assets_are_prepared() {
        let fixture = Fixture::new();
        let mut invalid = document();
        invalid
            .styles
            .light
            .0
            .insert("primary".into(), "red; background: url(evil)".into());
        let (prepared, diagnostics) = prepare(&write_document(&fixture, &invalid));
        assert!(prepared.is_none());
        assert!(diagnostics.iter().any(|error| {
            error.severity == Severity::Error && error.message.contains("styles.light.primary")
        }));
        let input = serde_json::to_string(&document()).unwrap();
        fixture.write(
            "theme.json",
            input.replacen("\"primary\":", "\"primary\":\"#ffffff\",\"primary\":", 1),
        );
        let (prepared, diagnostics) = prepare(&fixture.project(Some("theme.json")));
        assert!(prepared.is_none());
        assert!(diagnostics[0].message.contains("duplicate token primary"));
        fixture.write("theme.json", vec![b' '; THEME_INPUT_LIMIT + 1]);
        let (prepared, diagnostics) = prepare(&fixture.project(Some("theme.json")));
        assert!(prepared.is_none());
        assert!(diagnostics[0].message.contains("256 KiB"));
        assert_eq!(diagnostics[0].file.as_deref(), Some("rustify-docs.toml"));
    }

    #[test]
    fn document_paths_reject_traversal_private_files_and_generated_output() {
        let fixture = Fixture::new();
        fixture.write("theme.json", b"{}");
        fixture.write(".env", b"{}");
        fixture.write(".rustify-docs/theme.json", b"{}");
        fixture.write("dist/theme.json", b"{}");
        let project = fixture.project(None);
        for path in [
            "",
            "../theme.json",
            "/theme.json",
            ".env",
            ".rustify-docs/theme.json",
            "dist/theme.json",
        ] {
            assert!(document_path(&project, path).is_err(), "{path}");
        }
        assert!(document_path(&project, "theme.json").is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_must_remain_inside_the_public_project_boundary() {
        let fixture = Fixture::new();
        let outside = Fixture::new();
        outside.write("theme.json", b"{}");
        fixture.write("theme.json", b"{}");
        fixture.write(".env", b"{}");
        fixture.write("dist/theme.json", b"{}");
        for (link, target) in [
            ("escape.json", outside.0.join("theme.json")),
            ("private.json", fixture.0.join(".env")),
            ("output.json", fixture.0.join("dist/theme.json")),
            ("inside.json", fixture.0.join("theme.json")),
        ] {
            std::os::unix::fs::symlink(target, fixture.0.join(link)).unwrap();
        }
        let project = fixture.project(None);
        for path in ["escape.json", "private.json", "output.json"] {
            assert!(document_path(&project, path).is_err(), "{path}");
        }
        assert!(document_path(&project, "inside.json").is_ok());
    }

    #[test]
    fn incomplete_theme_documents_preserve_the_import_errors() {
        let fixture = Fixture::new();
        let mut document = document();
        document.styles.dark = ThemeValues(BTreeMap::new());
        let expected = import_json(
            &serde_json::to_string(&document).unwrap(),
            &font_context(16., false),
        )
        .unwrap_err()
        .to_string();
        let (prepared, diagnostics) = prepare(&write_document(&fixture, &document));
        assert!(prepared.is_none());
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert_eq!(diagnostics[0].message, expected);
    }
}
