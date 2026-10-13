//! Static projection of an imported theme and its offline font resources.

use rustify_docs::content::{is_private, Diagnostic, Project, Snapshot};
use rustify_docs::diagnostic::{Issue, Severity};
use rustify_ui::theme::{
    font_context, import_json, resolve, FontResourceDigest, ResolvedTheme, ThemeDocument,
    ThemeMode, FONT_GLYPH_FALLBACKS, FONT_RESOURCE_DIGESTS, THEME_INPUT_LIMIT,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

const V4_MAPPINGS: &str = include_str!("../../rustify-components/css/theme-v4.css");
const LIGHT: &str = ":root[data-rustify-scope], :root[data-rustify-scope] [data-rustify-scope]";
const DARK: &str =
    ":root[data-rustify-scope][data-theme=\"dark\"], :root[data-rustify-scope][data-theme=\"dark\"] [data-rustify-scope]";
const SYSTEM: &str =
    ":root[data-rustify-scope][data-theme=\"system\"], :root[data-rustify-scope][data-theme=\"system\"] [data-rustify-scope]";
const LIBERATION_SOURCE_SHA256: &str =
    "fe3ea5f7a2d3bdea8b8f0d82cdc6c07d14ace67c6e06d7aa33b83fd9e640adae";
const LIBERATION_SOURCE: &[u8] =
    include_bytes!("../assets/fonts/liberation-fonts-1.04.93.devel.src.tar.gz");

type FontFiles = BTreeMap<String, Arc<[u8]>>;

pub(super) struct PreparedTheme {
    pub document: ThemeDocument,
    /// The exact imported read, for the generation's input guard.
    pub input_path: String,
    pub input_bytes: Arc<[u8]>,
    input_real_path: PathBuf,
    /// Tailwind input mappings, consumed before compiling utility candidates.
    pub utilities: String,
    /// Browser CSS, appended after the site's default token rules.
    pub css: String,
    pub files: FontFiles,
}

struct EmbeddedFont {
    identity: FontResourceDigest,
    bytes: &'static [u8],
    license: &'static [u8],
}

const FONTS: [EmbeddedFont; 7] = [
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[0],
        bytes: include_bytes!("../assets/fonts/IBMPlexSans-Text.ttf"),
        license: include_bytes!("../assets/fonts/IBMPlexSans-Text.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[1],
        bytes: include_bytes!("../assets/fonts/NotoSans-Regular.ttf"),
        license: include_bytes!("../assets/fonts/NotoSans-Regular.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[2],
        bytes: include_bytes!("../assets/fonts/LXGWWenKaiRegular.ttf"),
        license: include_bytes!("../assets/fonts/LXGWWenKaiRegular.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[3],
        bytes: include_bytes!("../assets/fonts/NotoSerif-Regular.ttf"),
        license: include_bytes!("../assets/fonts/NotoSerif-Regular.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[4],
        bytes: include_bytes!("../assets/fonts/LiberationMono-Regular.ttf"),
        license: include_bytes!("../assets/fonts/LiberationMono-Regular.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[5],
        bytes: include_bytes!("../assets/fonts/jetbrains_mono_variable.ttf"),
        license: include_bytes!("../assets/fonts/jetbrains_mono_variable.license.txt"),
    },
    EmbeddedFont {
        identity: FONT_RESOURCE_DIGESTS[6],
        bytes: include_bytes!("../assets/fonts/NotoColorEmoji.ttf"),
        license: include_bytes!("../assets/fonts/NotoColorEmoji.license.txt"),
    },
];

pub(super) fn font_catalog() -> &'static [FontResourceDigest] {
    &FONT_RESOURCE_DIGESTS
}

/// No document is the P1 path: no theme CSS and no font assets.
pub(super) fn prepare(project: &Project) -> (Option<PreparedTheme>, Vec<Diagnostic>) {
    let Some((path, span)) = &project.config.theme_document else {
        return (None, Vec::new());
    };
    let (input_real_path, input) = match document_path(project, path)
        .and_then(|real| read_document(project, path).map(|input| (real, input)))
    {
        Ok(input) => input,
        Err(message) => {
            return (
                None,
                vec![Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::error(span.clone(), "theme-document", message),
                )],
            );
        }
    };
    let context = font_context(16., false);
    let imported = match import_json(&input, &context) {
        Ok(imported) => imported,
        Err(error) => {
            return (
                None,
                vec![Diagnostic::in_file(
                    path,
                    Severity::Error,
                    "theme-document",
                    error.to_string(),
                )],
            );
        }
    };
    let mut diagnostics: Vec<_> = imported
        .diagnostics
        .into_iter()
        .map(|warning| {
            Diagnostic::in_file(
                path,
                Severity::Warning,
                "theme-token",
                format!("{}: {}", warning.field, warning.message),
            )
        })
        .collect();
    let resolved =
        [ThemeMode::Light, ThemeMode::Dark].map(|mode| resolve(&imported.document, mode, &context));
    let [Ok(light), Ok(dark)] = resolved else {
        for error in resolved.into_iter().filter_map(Result::err) {
            diagnostics.push(Diagnostic::in_file(
                path,
                Severity::Error,
                "theme-document",
                error.to_string(),
            ));
        }
        return (None, diagnostics);
    };
    match project_css(&[light, dark], &project.config.site.base, &FONTS) {
        Ok((css, files)) => (
            Some(PreparedTheme {
                document: imported.document,
                input_path: path.to_owned(),
                input_bytes: Arc::from(input.as_bytes()),
                input_real_path,
                utilities: V4_MAPPINGS.to_owned(),
                css,
                files,
            }),
            diagnostics,
        ),
        Err(message) => {
            diagnostics.push(Diagnostic::in_file(
                path,
                Severity::Error,
                "theme-font-resource",
                message,
            ));
            (None, diagnostics)
        }
    }
}

/// Bind the parsed document to the content and compiled generation's reads.
pub(super) fn validate_snapshot(
    project: &Project,
    snapshot: &Snapshot,
    theme: &PreparedTheme,
) -> Vec<Diagnostic> {
    let current = document_path(project, &theme.input_path).and_then(|path| {
        if path != theme.input_real_path {
            return Err("theme.document target changed during generation".into());
        }
        read_document(project, &theme.input_path)
    });
    if snapshot
        .files
        .get(&theme.input_path)
        .is_some_and(|bytes| bytes == &theme.input_bytes)
        && current.is_ok_and(|text| text.as_bytes() == theme.input_bytes.as_ref())
    {
        Vec::new()
    } else {
        vec![Diagnostic::in_file(
            &theme.input_path,
            Severity::Error,
            "rust-generation-changed",
            "theme.document changed during generation; retry with one input snapshot",
        )]
    }
}

fn document_path(project: &Project, relative: &str) -> Result<PathBuf, String> {
    if relative.is_empty()
        || Path::new(relative)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        || is_private(relative)
    {
        return Err("theme.document must name a non-private file inside the project".into());
    }
    let root = fs::canonicalize(&project.root)
        .map_err(|error| format!("cannot resolve the theme's project root: {error}"))?;
    let path = fs::canonicalize(root.join(relative))
        .map_err(|error| format!("cannot read theme.document `{relative}`: {error}"))?;
    let canonical_relative = path
        .strip_prefix(&root)
        .map_err(|_| "theme.document symlink leads outside the project".to_string())?
        .to_string_lossy()
        .replace('\\', "/");
    let output = &project.config.out_dir;
    if is_private(&canonical_relative)
        || canonical_relative == *output
        || canonical_relative.starts_with(&format!("{output}/"))
    {
        return Err("theme.document must not read private files or generated output".into());
    }
    if !path.is_file() {
        return Err(format!("theme.document `{relative}` is not a file"));
    }
    Ok(path)
}

fn read_document(project: &Project, relative: &str) -> Result<String, String> {
    let path = document_path(project, relative)?;
    let file = fs::File::open(path)
        .map_err(|error| format!("cannot read theme.document `{relative}`: {error}"))?;
    let mut bytes = Vec::new();
    file.take((THEME_INPUT_LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read theme.document `{relative}`: {error}"))?;
    if bytes.len() > THEME_INPUT_LIMIT {
        return Err("theme input exceeds 256 KiB".into());
    }
    String::from_utf8(bytes).map_err(|error| format!("theme.document must be UTF-8: {error}"))
}

fn project_css(
    themes: &[ResolvedTheme; 2],
    base: &str,
    assets: &[EmbeddedFont],
) -> Result<(String, FontFiles), String> {
    let mut resources = BTreeSet::new();
    for font in themes.iter().flat_map(|theme| &theme.fonts) {
        if let Some(path) = &font.resource_path {
            resources.insert(path.clone());
        }
    }
    if !resources.is_empty() {
        for font in FONT_GLYPH_FALLBACKS {
            resources.insert(font.resource_path.to_owned());
        }
    }
    let mut css = String::new();
    let mut files = FontFiles::new();
    for resource in resources {
        let font = assets
            .iter()
            .find(|font| font.identity.resource_path == resource)
            .filter(|font| !font.bytes.is_empty() && !font.license.is_empty())
            .ok_or_else(|| {
                format!("the paired CLI has no packaged bytes or license for font `{resource}`")
            })?;
        let digest = font.identity.sha256;
        let path = format!("assets/fonts/{digest}.ttf");
        let url = format!("{base}{path}");
        let family = font.identity.family;
        css.push_str(&format!(
            "@font-face {{font-family:\"{family}\";src:url(\"{url}\") format(\"truetype\");font-display:swap;}}\n"
        ));
        files.insert(path, Arc::from(font.bytes));
        files.insert(
            format!(
                "assets/fonts/{:x}.license.txt",
                Sha256::digest(font.license)
            ),
            Arc::from(font.license),
        );
        if resource == FONT_RESOURCE_DIGESTS[4].resource_path {
            files.insert(
                format!("assets/fonts/{LIBERATION_SOURCE_SHA256}.tar.gz"),
                Arc::from(LIBERATION_SOURCE),
            );
        }
    }
    properties_rule(&mut css, LIGHT, &themes[0]);
    properties_rule(&mut css, DARK, &themes[1]);
    css.push_str("@media (prefers-color-scheme: dark) {\n");
    properties_rule(&mut css, SYSTEM, &themes[1]);
    css.push_str("}\n@layer utilities {\n");
    css.push_str(&format!(
        ":where({LIGHT}) {{font-family:var(--rustify-font-sans);font-size:var(--font-size);letter-spacing:var(--rustify-letter-spacing);}}\n"
    ));
    css.push_str(
        ":root :where(button,input,textarea,select) {font-family:inherit;letter-spacing:inherit;}\n}\n",
    );
    Ok((css, files))
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
    use rustify_ui::theme::{dom_font_stack, ResolveContext, Theme, ThemeValues};
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
            values.0.insert("font-serif".into(), "Noto Serif".into());
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
        assert_eq!(prepared.input_path, "themes/current.json");
        assert_eq!(prepared.input_bytes.as_ref(), input.as_bytes());
        assert_eq!(prepared.document, original);

        fixture.write("themes/current.json", b"new bytes after the imported read");
        assert_eq!(prepared.input_bytes.as_ref(), input.as_bytes());
        assert_ne!(
            prepared.input_bytes.as_ref(),
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
        assert_eq!(prepared.document, document);
        let mut expected = String::new();
        let context = font_context(16., false);
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
    fn all_seven_font_bytes_and_notices_are_paired_with_the_sdk_metadata() {
        assert_eq!(FONTS.len(), font_catalog().len());
        for font in &FONTS {
            assert_eq!(
                format!("{:x}", Sha256::digest(font.bytes)),
                font.identity.sha256
            );
            let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../makepad/widgets/resources")
                .join(font.identity.resource_path.rsplit('/').next().unwrap());
            assert_eq!(fs::read(source).unwrap(), font.bytes);
            let notice = std::str::from_utf8(font.license).unwrap();
            assert!(notice.contains("Copyright") || notice.contains("copyright"));
            assert!(!notice.contains("makepad"));
            if font.identity.family == "Liberation Mono" {
                assert!(notice.contains("GNU GENERAL PUBLIC LICENSE"));
                assert!(notice.contains("As a special exception"));
                assert!(notice.contains("2007 Ascender"));
            } else {
                assert!(notice.contains("SIL OPEN FONT LICENSE Version 1.1"));
            }
            if font.identity.family == "Rustify WenKai" {
                assert!(notice.contains("Modified by Rustify"));
                assert!(notice.contains("Reserved Font Names"));
            }
        }
    }

    #[test]
    fn emitted_fonts_are_the_referenced_faces_and_glyph_fallbacks_once() {
        let fixture = Fixture::new();
        let mut document = document();
        for values in [&mut document.styles.light, &mut document.styles.dark] {
            for token in ["font-sans", "font-serif", "font-mono"] {
                values.0.insert(token.into(), "Rustify WenKai".into());
            }
        }
        let (prepared, diagnostics) = prepare(&write_document(&fixture, &document));
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let prepared = prepared.unwrap();
        assert_eq!(prepared.css.matches("@font-face").count(), 2);
        assert_eq!(prepared.files.len(), 4);
        for font in [&FONTS[2], &FONTS[6]] {
            let path = format!("assets/fonts/{}.ttf", font.identity.sha256);
            assert_eq!(prepared.files[&path].as_ref(), font.bytes);
            assert!(prepared.css.contains(&format!("url(\"/sub/{path}\")")));
            assert_eq!(
                prepared.files[&format!(
                    "assets/fonts/{:x}.license.txt",
                    Sha256::digest(font.license)
                )]
                    .as_ref(),
                font.license
            );
        }
        assert!(!prepared.css.contains("https://"));
        assert!(prepared.files.keys().all(|path| {
            path.starts_with("assets/fonts/")
                && (path.ends_with(".ttf") || path.ends_with(".license.txt"))
        }));
    }

    #[test]
    fn different_mode_faces_are_packaged_together_without_unused_faces() {
        let mut document = document();
        document
            .styles
            .dark
            .0
            .insert("font-sans".into(), "Noto Sans".into());
        document
            .styles
            .dark
            .0
            .insert("font-mono".into(), "JetBrains Mono".into());
        let context = font_context(16., false);
        let themes = [ThemeMode::Light, ThemeMode::Dark]
            .map(|mode| resolve(&document, mode, &context).unwrap());
        let (css, files) = project_css(&themes, "/", &FONTS).unwrap();
        assert_eq!(css.matches("@font-face").count(), 7);
        assert_eq!(files.len(), 15);
    }

    #[test]
    fn corresponding_source_is_only_emitted_with_the_referenced_liberation_font() {
        assert_eq!(
            format!("{:x}", Sha256::digest(LIBERATION_SOURCE)),
            LIBERATION_SOURCE_SHA256
        );
        let mut document = document();
        for values in [&mut document.styles.light, &mut document.styles.dark] {
            values.0.insert("font-mono".into(), "JetBrains Mono".into());
        }
        let context = font_context(16., false);
        let resolved = |document: &ThemeDocument| {
            [ThemeMode::Light, ThemeMode::Dark]
                .map(|mode| resolve(document, mode, &context).unwrap())
        };
        let (_, without) = project_css(&resolved(&document), "/", &FONTS).unwrap();
        assert!(without.keys().all(|path| !path.ends_with(".tar.gz")));

        document
            .styles
            .dark
            .0
            .insert("font-mono".into(), "Liberation Mono".into());
        let (css, with) = project_css(&resolved(&document), "/", &FONTS).unwrap();
        let path = format!("assets/fonts/{LIBERATION_SOURCE_SHA256}.tar.gz");
        assert_eq!(with[&path].as_ref(), LIBERATION_SOURCE);
        assert!(!css.contains(".tar.gz"));
        assert!(std::str::from_utf8(FONTS[4].license)
            .unwrap()
            .contains(&format!("{LIBERATION_SOURCE_SHA256}.tar.gz")));
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
            warning.message.contains("Unknown Family") && warning.message.contains("IBM Plex Sans")
        }));
        assert_eq!(
            prepared.document.styles.light.0["unknown-token"],
            "url(https://untrusted.invalid/injected)"
        );
        assert!(!prepared.css.contains("untrusted"));
        assert!(!prepared.css.contains("Unknown Family"));
    }

    #[test]
    fn a_missing_embedded_face_or_license_is_an_error_without_a_network_fallback() {
        let context = font_context(16., false);
        let document = document();
        let themes = [ThemeMode::Light, ThemeMode::Dark]
            .map(|mode| resolve(&document, mode, &context).unwrap());
        let error = project_css(&themes, "/", &[]).unwrap_err();
        assert!(error.contains("no packaged bytes or license"));
        let mut assets = FONTS;
        assets[0].license = &[];
        assert!(project_css(&themes, "/", &assets).is_err());
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
        let (css, files) = project_css(&themes, "/", &[]).unwrap();
        assert!(files.is_empty());
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
