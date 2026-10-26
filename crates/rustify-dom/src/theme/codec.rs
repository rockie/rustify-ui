//! Data-only theme exchange. Importing returns a candidate and never applies CSS.

use super::{
    parse_color, parse_font_stack, parse_length, ColorFormat, LengthRule, ResolveContext,
    ResolvedTheme, ThemeDiagnostic, ThemeDocument, ThemeError, ThemeMode, ThemeStyles,
    COLOR_TOKENS, VALUE_TOKENS,
};
use std::collections::BTreeMap;

mod export;
pub use export::{export_json_v1_lossy, LegacyExportLoss, LegacyJsonExport};

pub const THEME_INPUT_LIMIT: usize = 256 * 1024;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeImport {
    pub document: ThemeDocument,
    pub diagnostics: Vec<ThemeDiagnostic>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CssProfile {
    TailwindV4,
    /// Mark a static target with data-rustify-theme-css equal to the document id.
    RustifyScoped,
}

fn check_size(input: &str) -> Result<(), ThemeError> {
    if input.len() > THEME_INPUT_LIMIT {
        return Err(ThemeError::new("input", "theme input exceeds 256 KiB"));
    }
    Ok(())
}

fn validate(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<([ResolvedTheme; 2], Vec<ThemeDiagnostic>), ThemeError> {
    let source = super::ThemeSource::new(std::sync::Arc::new(document.clone()));
    let resolved = [
        source.resolve(ThemeMode::Light, context)?,
        source.resolve(ThemeMode::Dark, context)?,
    ];
    let mut diagnostics = Vec::new();
    for theme in &resolved {
        for diagnostic in &theme.diagnostics {
            if !diagnostics.contains(diagnostic) {
                diagnostics.push(diagnostic.clone());
            }
        }
    }
    Ok((resolved, diagnostics))
}

/// JSON is the lossless, complete two-mode exchange format. Unknown tokens
/// remain in the document, but the resolver never projects them to CSS/GPU.
pub fn import_json(input: &str, context: &ResolveContext<'_>) -> Result<ThemeImport, ThemeError> {
    check_size(input)?;
    let document: ThemeDocument = serde_json::from_str(input).map_err(|error| {
        let message = error.to_string();
        let field = ["config", "base", "overrides", "styles", "schema_version"]
            .into_iter()
            .find(|field| message.starts_with(&format!("{field}:")))
            .unwrap_or("json");
        ThemeError::new(field, message)
    })?;
    let (_, diagnostics) = validate(&document, context)?;
    check_document_size(&document)?;
    Ok(ThemeImport {
        document,
        diagnostics,
    })
}

/// CSS has no document identity; the candidate keeps the supplied base's id/name.
/// Missing dark declarations inherit imported light values, then use the base.
pub fn import_css(
    input: &str,
    base: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<ThemeImport, ThemeError> {
    check_size(input)?;
    let cleaned = without_comments(input)?;
    let mut parser = CssParser {
        source: &cleaned,
        lines: std::iter::once(0)
            .chain(
                cleaned
                    .bytes()
                    .enumerate()
                    .filter_map(|(pos, byte)| (byte == b'\n').then_some(pos + 1)),
            )
            .collect(),
        pos: 0,
        light: BTreeMap::new(),
        dark: BTreeMap::new(),
        diagnostics: Vec::new(),
        context,
    };
    parser.rules(false, 0)?;
    if parser.light.is_empty() && parser.dark.is_empty() {
        return Err(ThemeError::new(
            "css",
            "no supported theme declarations were found",
        ));
    }
    let (base_pair, _) = validate(base, context)?;
    let mut styles = ThemeStyles {
        light: base_pair[0].values.clone(),
        dark: base_pair[1].values.clone(),
    };
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let imported = if mode == ThemeMode::Light {
            &parser.light
        } else {
            &parser.dark
        };
        let missing: Vec<_> = COLOR_TOKENS
            .iter()
            .chain(VALUE_TOKENS)
            .filter(|&&token| {
                !imported.contains_key(token)
                    && (mode == ThemeMode::Light || !parser.light.contains_key(token))
            })
            .copied()
            .collect();
        if !missing.is_empty() {
            parser.diagnostics.push(ThemeDiagnostic {
                field: format!("styles.{}", mode.as_str()),
                message: format!(
                    "Filled missing tokens from the supplied base: {}",
                    missing.join(", ")
                ),
            });
        }
        let values = match mode {
            ThemeMode::Light => &mut styles.light,
            ThemeMode::Dark => &mut styles.dark,
        };
        if mode == ThemeMode::Dark {
            values.0.extend(parser.light.clone());
        }
        values.0.extend(imported.clone());
    }
    let document = ThemeDocument::custom(&base.id, &base.name, styles);
    let (_, diagnostics) = validate(&document, context)?;
    check_document_size(&document)?;
    parser.diagnostics.extend(diagnostics);
    Ok(ThemeImport {
        document,
        diagnostics: parser.diagnostics,
    })
}

/// Checks the canonical complete exchange representation without resolving again.
pub fn check_document_size(document: &ThemeDocument) -> Result<(), ThemeError> {
    let canonical = serde_json::to_string(document)
        .map_err(|error| ThemeError::new("json", error.to_string()))?;
    check_size(&canonical)
}

pub fn export_json(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<String, ThemeError> {
    validate(document, context)?;
    let output = serde_json::to_string(document)
        .map_err(|error| ThemeError::new("json", error.to_string()))?;
    check_size(&output)?;
    Ok(output)
}

/// CSS colors describe resolved sRGB. Use JSON to preserve original color
/// notation, gamut and unknown tokens. This emits data and build-time mapping;
/// it does not replace values owned by a running ThemeScope.
pub fn export_css(
    document: &ThemeDocument,
    profile: CssProfile,
    context: &ResolveContext<'_>,
    color_format: ColorFormat,
) -> Result<String, ThemeError> {
    check_document_size(document)?;
    let (themes, _) = validate(document, context)?;
    let scoped = profile == CssProfile::RustifyScoped;
    let light = if scoped {
        format!(
            "[data-rustify-scope][data-rustify-theme-css={}]",
            css_string(&document.id)
        )
    } else {
        ":root".into()
    };
    let dark = if scoped {
        format!("{light}[data-theme=\"dark\"]")
    } else {
        ".dark".into()
    };
    let mut output = String::from("/* Resolved sRGB CSS: the first selector selects light mode; the second selects dark mode.\n   Original author intent and unknown strings are preserved only by Theme JSON.\n   Font files are external references, never embedded. Distribute their original licenses;\n   SDK font notices: makepad/widgets/resources/FONT-LICENSES.md. */\n");
    if scoped {
        output.push_str("/* Import after SDK sdk.css in one Tailwind v4 input. Mark the static target with\n   data-rustify-theme-css equal to the JSON id. data-theme=\"dark\" selects dark;\n   omit it for light. For ThemeScope and authored appearance use JSON instead. */\n");
    } else {
        output.push_str("@import \"tailwindcss\";\n");
        output.push_str(
            "/* :root selects light; .dark selects dark. Authored appearance is kept in JSON. */\n",
        );
    }
    output.push_str(&format!("@custom-variant dark (&:is({dark} *));\n\n"));
    let mut font_faces = Vec::new();
    for theme in &themes {
        for font in &theme.fonts {
            let requested = css_string(&font.requested.join(", ")).replace("*/", "* /");
            let family = css_string(&font.family).replace("*/", "* /");
            let resource = font
                .resource_path
                .as_deref()
                .unwrap_or("no packaged file")
                .replace("*/", "* /");
            output.push_str(&format!(
                "/* Requested font: {requested}; resolved: {family}; resource: {resource}. */\n"
            ));
            if let Some(path) = &font.resource_path {
                let scheme = path.trim_start().to_ascii_lowercase();
                if path.chars().any(char::is_control)
                    || ["data:", "javascript:", "vbscript:"]
                        .iter()
                        .any(|prefix| scheme.starts_with(prefix))
                {
                    return Err(ThemeError::new("fonts", "export only external font resource URLs, without control characters, embedded data or script schemes"));
                }
                let declaration = format!(
                    "@font-face {{ font-family: {}; src: url({}); font-display: swap; }}\n",
                    css_string(&font.family),
                    css_string(path)
                );
                if !font_faces.contains(&declaration) {
                    font_faces.push(declaration);
                }
            }
        }
    }
    if themes
        .iter()
        .any(|theme| theme.fonts.iter().any(|font| font.resource_path.is_some()))
    {
        for face in super::FONT_GLYPH_FALLBACKS {
            let declaration = format!(
                "@font-face {{ font-family: {}; src: url({}); font-display: swap; }}\n",
                css_string(face.family),
                css_string(face.resource_path)
            );
            if !font_faces.contains(&declaration) {
                font_faces.push(declaration);
            }
        }
    }
    for declaration in font_faces {
        output.push_str(&declaration);
    }
    for (theme, selector) in themes.iter().zip([&light, &dark]) {
        output.push_str(&format!("\n{selector} {{\n"));
        for (property, value) in export::css_properties(theme, scoped, context, color_format)? {
            output.push_str(&format!("  {property}: {value};\n"));
        }
        output.push_str("}\n");
    }
    output.push_str("\n@theme inline reference {\n");
    for &token in COLOR_TOKENS {
        output.push_str(&format!("  --color-{token}: var(--{token});\n"));
    }
    output.push_str(if scoped {
        "  --spacing: var(--rustify-unit, 0.25rem);\n"
    } else {
        "  --spacing: var(--spacing);\n"
    });
    // Keep SDK legacy utilities in force outside the exported scopes. The
    // SDK default radius is 6px; a bare host element has no scope token yet.
    for (size, fallback) in [
        ("sm", "calc(var(--radius, 6px) - 2px)"),
        ("md", "var(--radius, 6px)"),
        ("lg", "calc(var(--radius, 6px) + 2px)"),
        ("xl", "0.75rem"),
    ] {
        let fallback = if scoped {
            format!(", {fallback}")
        } else {
            String::new()
        };
        output.push_str(&format!(
            "  --radius-{size}: var(--rustify-radius-{size}{fallback});\n"
        ));
    }
    for (slot, fallback) in [
        (
            "sans",
            r#"ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji""#,
        ),
        (
            "serif",
            r#"ui-serif, Georgia, Cambria, "Times New Roman", Times, serif"#,
        ),
        (
            "mono",
            r#"ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#,
        ),
    ] {
        let fallback = if scoped {
            format!(", {fallback}")
        } else {
            String::new()
        };
        output.push_str(&format!(
            "  --font-{slot}: var(--rustify-font-{slot}{fallback});\n"
        ));
    }
    for &(name, fallback) in super::resolve::SHADOW_FALLBACKS {
        output.push_str(&format!(
            "  --{name}: {};\n",
            super::resolve::shadow_utility(name, fallback)
        ));
    }
    output.push_str("}\n\n@layer utilities {\n");
    output.push_str(&format!("  :where({light}, {dark}) {{ font-family: var(--rustify-font-sans); font-size: var(--rustify-font-size); letter-spacing: var(--rustify-letter-spacing); }}\n"));
    output.push_str(&format!("  :where({light}, {dark}) :where(button, input, textarea, select) {{ font-family: inherit; letter-spacing: inherit; }}\n"));
    output.push_str("}\n");
    check_size(&output)?;
    Ok(output)
}

fn css_token(token: &str, scoped: bool) -> String {
    match token {
        "font-size" | "layout-gap" | "reduce-motion" => format!("rustify-{token}"),
        "spacing" if scoped => "rustify-unit".into(),
        _ => token.into(),
    }
}

fn css_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn known(token: &str) -> bool {
    COLOR_TOKENS.contains(&token) || VALUE_TOKENS.contains(&token)
}

fn validate_value(
    token: &str,
    value: &str,
    context: &ResolveContext<'_>,
) -> Result<(), ThemeError> {
    if COLOR_TOKENS.contains(&token) {
        parse_color(value).map_err(|message| ThemeError::new(token, message))?;
    } else {
        match token {
            "font-sans" | "font-serif" | "font-mono" => {
                if context.font_resolution == super::FontResolution::Css {
                    super::parse_css_font_stack(value)?;
                } else {
                    parse_font_stack(value)?;
                }
            }
            "font-size" | "spacing" => {
                parse_length(value, context.rem_px, LengthRule::Positive)?;
            }
            "radius" | "layout-gap" | "shadow-blur" => {
                parse_length(value, context.rem_px, LengthRule::NonNegative)?;
            }
            "shadow-spread" | "shadow-offset-x" | "shadow-offset-y" => {
                parse_length(value, context.rem_px, LengthRule::Signed)?;
            }
            "shadow-opacity" => {
                super::metrics::number(value, 0., 1.)?;
            }
            "letter-spacing" => {
                if value.ends_with("px") || value.ends_with("rem") {
                    parse_length(value, context.rem_px, LengthRule::Signed)?;
                } else {
                    super::metrics::letter_spacing(value, 16., context.rem_px)?;
                }
            }
            "reduce-motion" if value == "true" || value == "false" => {}
            _ => return Err(ThemeError::new(token, "unsupported theme value")),
        }
    }
    Ok(())
}

fn without_comments(source: &str) -> Result<String, ThemeError> {
    let mut bytes = source.as_bytes().to_vec();
    let mut pos = 0;
    let mut quote = None;
    while pos < bytes.len() {
        match (quote, bytes[pos]) {
            (Some(_), b'\\') => {
                pos += 2;
                continue;
            }
            (Some(open), close) if open == close => quote = None,
            (None, b'\'' | b'"') => quote = Some(bytes[pos]),
            (None, b'/') if bytes.get(pos + 1) == Some(&b'*') => {
                let start = pos;
                pos += 2;
                while pos + 1 < bytes.len() && !(bytes[pos] == b'*' && bytes[pos + 1] == b'/') {
                    pos += 1;
                }
                if pos + 1 >= bytes.len() {
                    return Err(ThemeError::new(
                        "css",
                        format!("unterminated comment at byte {start}"),
                    ));
                }
                pos += 2;
                for byte in &mut bytes[start..pos] {
                    if *byte != b'\n' {
                        *byte = b' ';
                    }
                }
                continue;
            }
            _ => {}
        }
        pos += 1;
    }
    String::from_utf8(bytes)
        .map_err(|_| ThemeError::new("css", "invalid UTF-8 after comment parsing"))
}

struct CssParser<'a, 'c> {
    source: &'a str,
    lines: Vec<usize>,
    pos: usize,
    light: BTreeMap<String, String>,
    dark: BTreeMap<String, String>,
    diagnostics: Vec<ThemeDiagnostic>,
    context: &'a ResolveContext<'c>,
}

impl CssParser<'_, '_> {
    fn whitespace(&mut self) {
        while self
            .source
            .as_bytes()
            .get(self.pos)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.pos += 1;
        }
    }

    fn location(&self, pos: usize) -> String {
        let line = self.lines.partition_point(|&start| start <= pos) - 1;
        format!(
            "line {}, byte column {}",
            line + 1,
            pos - self.lines[line] + 1
        )
    }

    fn ignored(&mut self, pos: usize, message: impl Into<String>) {
        self.diagnostics.push(ThemeDiagnostic {
            field: format!("css.{}", self.location(pos)),
            message: message.into(),
        });
    }

    fn scan(&self, start: usize, delimiters: &[u8]) -> Result<(usize, u8), ThemeError> {
        let bytes = self.source.as_bytes();
        let mut quote = None;
        let mut parens = 0usize;
        let mut brackets = 0usize;
        let mut pos = start;
        while pos < bytes.len() {
            let byte = bytes[pos];
            if quote.is_some() && byte == b'\\' {
                pos += 2;
                continue;
            }
            if let Some(open) = quote {
                if byte == open {
                    quote = None;
                } else if byte == b'\n' {
                    return Err(ThemeError::new(
                        "css",
                        format!("newline in string at {}", self.location(pos)),
                    ));
                }
            } else {
                match byte {
                    b'\'' | b'"' => quote = Some(byte),
                    b'(' => parens += 1,
                    b')' if parens > 0 => parens -= 1,
                    b'[' => brackets += 1,
                    b']' if brackets > 0 => brackets -= 1,
                    b')' | b']' => {
                        return Err(ThemeError::new(
                            "css",
                            format!("unmatched delimiter at {}", self.location(pos)),
                        ))
                    }
                    _ if parens == 0 && brackets == 0 && delimiters.contains(&byte) => {
                        return Ok((pos, byte))
                    }
                    _ => {}
                }
            }
            pos += 1;
        }
        if quote.is_some() || parens > 0 || brackets > 0 {
            return Err(ThemeError::new(
                "css",
                format!(
                    "unterminated string or delimiter at {}",
                    self.location(start)
                ),
            ));
        }
        Ok((bytes.len(), 0))
    }

    fn rules(&mut self, enclosed: bool, depth: usize) -> Result<(), ThemeError> {
        if depth > 32 {
            return Err(ThemeError::new("css", "CSS nesting exceeds 32 blocks"));
        }
        loop {
            self.whitespace();
            let start = self.pos;
            let (end, delimiter) = self.scan(start, b";{}")?;
            let header = self.source[start..end].trim();
            self.pos = end + usize::from(delimiter != 0);
            match delimiter {
                0 if enclosed => return Err(ThemeError::new("css", "missing closing brace")),
                0 if header.is_empty() => return Ok(()),
                0 => {
                    return Err(ThemeError::new(
                        "css",
                        format!("incomplete rule at {}", self.location(start)),
                    ))
                }
                b'}' if enclosed && header.is_empty() => return Ok(()),
                b'}' => {
                    return Err(ThemeError::new(
                        "css",
                        format!("unexpected closing brace at {}", self.location(start)),
                    ))
                }
                b';' => self.ignored(start, format!("Ignored statement: {header}")),
                b'{' if header == "@layer"
                    || header
                        .strip_prefix("@layer")
                        .is_some_and(|suffix| suffix.starts_with(char::is_whitespace)) =>
                {
                    self.rules(true, depth + 1)?
                }
                b'{' => {
                    let selections: Vec<_> = selector_parts(header)
                        .into_iter()
                        .filter_map(selector_mode)
                        .collect();
                    if selections.is_empty() {
                        self.ignored(start, format!("Ignored selector or at-rule: {header}"));
                        self.skip_block(depth + 1)?;
                    } else {
                        if selections.len() != selector_parts(header).len() {
                            self.ignored(
                                start,
                                "Ignored unrecognized selectors in this selector list",
                            );
                        }
                        self.declarations(&selections)?;
                    }
                }
                _ => return Err(ThemeError::new("css", "invalid rule")),
            }
        }
    }

    fn skip_block(&mut self, mut depth: usize) -> Result<(), ThemeError> {
        let initial = depth;
        loop {
            let (end, delimiter) = self.scan(self.pos, b"{}")?;
            self.pos = end + usize::from(delimiter != 0);
            match delimiter {
                b'{' => {
                    depth += 1;
                    if depth > 32 {
                        return Err(ThemeError::new("css", "CSS nesting exceeds 32 blocks"));
                    }
                }
                b'}' if depth == initial => return Ok(()),
                b'}' => depth -= 1,
                _ => return Err(ThemeError::new("css", "missing closing brace")),
            }
        }
    }

    fn declarations(&mut self, selections: &[(ThemeMode, bool)]) -> Result<(), ThemeError> {
        loop {
            self.whitespace();
            let start = self.pos;
            let (end, delimiter) = self.scan(start, b";{}")?;
            let declaration = self.source[start..end].trim();
            self.pos = end + usize::from(delimiter != 0);
            if delimiter == 0 || delimiter == b'{' {
                return Err(ThemeError::new(
                    "css",
                    format!("invalid declaration block at {}", self.location(start)),
                ));
            }
            if !declaration.is_empty() {
                let (property, value) = declaration.split_once(':').ok_or_else(|| {
                    ThemeError::new(
                        "css",
                        format!("missing declaration colon at {}", self.location(start)),
                    )
                })?;
                let property = property.trim();
                let value = value.trim();
                let Some(raw_token) = property.strip_prefix("--") else {
                    self.ignored(
                        start,
                        if property == "box-shadow" {
                            "Box-shadow cannot be imported as editable shadow parameters"
                        } else {
                            "Ignored non-theme declaration"
                        },
                    );
                    if delimiter == b'}' {
                        return Ok(());
                    }
                    continue;
                };
                for &(mode, scoped) in selections {
                    let token = match raw_token {
                        "shadow-x" => "shadow-offset-x",
                        "shadow-y" => "shadow-offset-y",
                        "rustify-font-size" => "font-size",
                        "rustify-layout-gap" => "layout-gap",
                        "rustify-reduce-motion" => "reduce-motion",
                        "rustify-unit" => "spacing",
                        "spacing" if scoped => "layout-gap",
                        _ => raw_token,
                    };
                    if !known(token) {
                        self.ignored(start, format!("Ignored unknown variable {property}"));
                        continue;
                    }
                    validate_value(token, value, self.context).map_err(|error| {
                        ThemeError::new(
                            format!("styles.{}.{token}", mode.as_str()),
                            format!("{} at {}", error.message, self.location(start)),
                        )
                    })?;
                    let values = if mode == ThemeMode::Light {
                        &mut self.light
                    } else {
                        &mut self.dark
                    };
                    if values.insert(token.into(), value.into()).is_some() {
                        self.ignored(
                            start,
                            format!("Duplicate {} {token}; last declaration wins", mode.as_str()),
                        );
                    }
                }
            }
            if delimiter == b'}' {
                return Ok(());
            }
        }
    }
}

fn selector_parts(header: &str) -> Vec<&str> {
    let mut quote = None;
    let mut escaped = false;
    let mut start = 0;
    let mut parts = Vec::new();
    for (pos, byte) in header.bytes().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote.is_some() && byte == b'\\' {
            escaped = true;
            continue;
        }
        match (quote, byte) {
            (Some(open), close) if open == close => quote = None,
            (None, b'\'' | b'"') => quote = Some(byte),
            (None, b',') => {
                parts.push(header[start..pos].trim());
                start = pos + 1;
            }
            _ => {}
        }
    }
    parts.push(header[start..].trim());
    parts
}

fn selector_mode(selector: &str) -> Option<(ThemeMode, bool)> {
    match selector {
        ":root" => return Some((ThemeMode::Light, false)),
        ".dark" => return Some((ThemeMode::Dark, false)),
        _ => {}
    }
    let rest = selector.strip_prefix("[data-rustify-scope][data-rustify-theme-css=")?;
    let quote = rest.as_bytes().first().copied()?;
    if quote != b'\'' && quote != b'"' {
        return None;
    }
    let bytes = rest.as_bytes();
    let mut pos = 1;
    while pos < bytes.len() {
        if bytes[pos] == b'\\' {
            pos += 2;
            continue;
        }
        if bytes[pos] == quote {
            return match &rest[pos + 1..] {
                "]" => Some((ThemeMode::Light, true)),
                "][data-theme=\"dark\"]" | "][data-theme='dark']" => Some((ThemeMode::Dark, true)),
                _ => None,
            };
        }
        pos += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{format_color, resolve, Theme, ThemeMode};

    fn base() -> ThemeDocument {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document.overrides = Default::default();
        document
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn scoped_css_preserves_legacy_host_radius_spacing_and_font_defaults() {
        let sdk = include_str!("../../../rustify-components/css/sdk.css");
        let mappings = include_str!("../../../rustify-components/css/theme-v4.css");
        assert!(sdk.contains("--radius: 6px;"));
        for notation in [
            ColorFormat::Hex,
            ColorFormat::Rgb,
            ColorFormat::Hsl,
            ColorFormat::Oklch,
        ] {
            let css = export_css(
                &base(),
                CssProfile::RustifyScoped,
                &ResolveContext::default(),
                notation,
            )
            .unwrap();
            for declaration in [
                "--spacing: var(--rustify-unit, 0.25rem);",
                "--radius-sm: var(--rustify-radius-sm, calc(var(--radius, 6px) - 2px));",
                "--radius-md: var(--rustify-radius-md, var(--radius, 6px));",
                "--radius-lg: var(--rustify-radius-lg, calc(var(--radius, 6px) + 2px));",
                "--radius-xl: var(--rustify-radius-xl, 0.75rem);",
            ] {
                assert!(
                    css.contains(declaration),
                    "{notation:?}: missing host fallback {declaration}"
                );
            }
            for mapping in mappings
                .lines()
                .map(str::trim)
                .filter(|line| line.starts_with("--font-"))
            {
                assert!(
                    css.contains(mapping),
                    "{notation:?}: missing SDK font default {mapping}"
                );
            }
            assert!(
                !css.contains("\n:root {"),
                "scoped values must not be written onto the host root"
            );
        }
    }

    #[test]
    fn css_exports_keep_shadow_color_utilities_live_in_every_profile_and_notation() {
        let mut document = base();
        for values in [&mut document.base.light, &mut document.base.dark] {
            for (token, value) in [
                ("shadow-color", "rgb(30 10 20 / .6)"),
                ("shadow-opacity", ".4"),
                ("shadow-offset-x", "2px"),
                ("shadow-offset-y", "4px"),
                ("shadow-blur", "8px"),
                ("shadow-spread", "1px"),
            ] {
                values.0.insert(token.into(), value.into());
            }
        }
        let context = ResolveContext::default();
        let theme = resolve(&document, ThemeMode::Light, &context).unwrap();
        for profile in [CssProfile::TailwindV4, CssProfile::RustifyScoped] {
            for notation in [
                ColorFormat::Hex,
                ColorFormat::Rgb,
                ColorFormat::Hsl,
                ColorFormat::Oklch,
            ] {
                let css = export_css(&document, profile, &context, notation).unwrap();
                assert!(
                    css.contains("var(--tw-shadow-color, var(--rustify-shadow-lg-0-color,"),
                    "{profile:?} {notation:?}: color must be read on the utility element"
                );
                assert!(css.contains("--rustify-shadow-lg-0-geometry: 2px 4px 8px 1px;"));
                assert!(css.contains("--rustify-shadow-lg-1-geometry: 2px 4px 6px 0px;"));
                assert!(css.contains("--rustify-shadow-lg-0-inactive: initial;"));
                assert!(css.contains(&format!(
                    "--rustify-shadow-lg-0-color: {};",
                    format_color(theme.shadows["shadow-lg"][0].color, notation)
                )));
                assert!(
                    !css.contains("box-shadow:"),
                    "Tailwind must continue composing shadows and rings"
                );
            }
        }
    }

    #[test]
    fn css_merges_layered_blocks_with_last_declaration_diagnostics() {
        let source = "/* root */ @layer base { :root { --primary:#abc; --font-sans: 'A, B', sans-serif; } .dark { --primary:rgb(1 2 3 / .5); } } :root { --primary:#def; --shadow-x:2px; }";
        let imported = import_css(source, &base(), &ResolveContext::default()).unwrap();
        assert_eq!(imported.document.base.light.get("primary").unwrap(), "#def");
        assert_eq!(
            imported.document.base.dark.get("primary").unwrap(),
            "rgb(1 2 3 / .5)"
        );
        assert_eq!(
            imported.document.base.light.get("font-sans").unwrap(),
            "'A, B', sans-serif"
        );
        assert_eq!(
            imported.document.base.light.get("shadow-offset-x").unwrap(),
            "2px"
        );
        assert!(imported
            .diagnostics
            .iter()
            .any(|d| d.message.contains("last declaration")));
    }

    #[test]
    fn json_round_trip_keeps_author_values_and_unknown_tokens() {
        let mut document = base();
        document
            .base
            .dark
            .0
            .insert("primary".into(), "oklch(.7 .3 20)".into());
        document
            .base
            .light
            .0
            .insert("future-color".into(), "var(--anything)".into());
        let json = export_json(&document, &ResolveContext::default()).unwrap();
        let imported = import_json(&json, &ResolveContext::default()).unwrap();
        assert_eq!(imported.document, document);
        assert!(imported
            .diagnostics
            .iter()
            .any(|d| d.field.ends_with("future-color")));
    }

    #[test]
    fn css_profiles_round_trip_effective_metrics_fonts_and_colors() {
        let mut document = base();
        document
            .base
            .light
            .0
            .insert("radius".into(), ".75rem".into());
        document
            .base
            .dark
            .0
            .insert("font-serif".into(), "Georgia, serif".into());
        for profile in [CssProfile::TailwindV4, CssProfile::RustifyScoped] {
            let css = export_css(
                &document,
                profile,
                &ResolveContext::default(),
                ColorFormat::Rgb,
            )
            .unwrap();
            let imported = import_css(&css, &document, &ResolveContext::default()).unwrap();
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                let expected = resolve(&document, mode, &ResolveContext::default()).unwrap();
                let actual = resolve(&imported.document, mode, &ResolveContext::default()).unwrap();
                assert_eq!(actual.radius_px, expected.radius_px);
                assert_eq!(actual.font_size_px, expected.font_size_px);
                assert_eq!(actual.spacing_px, expected.spacing_px);
                assert_eq!(actual.layout_gap_px, expected.layout_gap_px);
                for (actual, expected) in actual.fonts.iter().zip(&expected.fonts) {
                    assert_eq!(actual.family, expected.family);
                }
                for token in COLOR_TOKENS {
                    assert_eq!(
                        format_color(actual.colors[token], ColorFormat::Rgb),
                        format_color(expected.colors[token], ColorFormat::Rgb)
                    );
                }
            }
        }
    }

    #[test]
    fn unsupported_known_values_reject_the_whole_candidate_even_if_overwritten() {
        let original = base();
        for value in [
            "var(--evil)",
            "url(https://example.test)",
            "calc(1px)",
            "red",
            "expression(alert(1))",
        ] {
            let css = format!(":root {{ --background:#fff; --primary:{value}; --primary:#fff; }}");
            assert!(
                import_css(&css, &original, &ResolveContext::default()).is_err(),
                "{value}"
            );
            assert_eq!(original, base());
        }
    }

    #[test]
    fn malformed_quotes_braces_and_oversize_inputs_are_rejected() {
        for source in [
            ":root { --primary:'#fff; }",
            ":root { --primary:#fff",
            "/* unfinished",
            ":root{--primary:rgb(1 2 3;}",
        ] {
            assert!(
                import_css(source, &base(), &ResolveContext::default()).is_err(),
                "{source}"
            );
        }
        let source = " ".repeat(THEME_INPUT_LIMIT + 1);
        assert!(import_css(&source, &base(), &ResolveContext::default()).is_err());
        assert!(import_json(&source, &ResolveContext::default()).is_err());
    }

    #[test]
    fn imports_never_visit_urls_and_ignore_mapping_rules_and_unknown_token_prefixes() {
        let css = "@import url('https://example.test/a.css'); @theme inline { --color-primary:var(--primary); } .unrelated { --primary: url('https://example.test/a'); } :root { --primary-extra:url('https://example.test'); --primary:#abc; box-shadow:0 1px 2px black; }";
        let candidate = import_css(css, &base(), &ResolveContext::default()).unwrap();
        assert_eq!(
            candidate.document.base.light.get("primary").unwrap(),
            "#abc"
        );
        assert!(!candidate
            .document
            .base
            .light
            .0
            .contains_key("primary-extra"));
        assert!(candidate
            .diagnostics
            .iter()
            .any(|d| d.message.contains("Box-shadow")));
        assert!(candidate
            .diagnostics
            .iter()
            .any(|d| d.message.contains("@theme")));
    }

    #[test]
    fn json_rejects_duplicates_missing_modes_schema_and_non_string_tokens() {
        let json = export_json(&base(), &ResolveContext::default()).unwrap();
        let duplicate = json.replacen(
            "\"primary\":",
            "\"primary\": \"url(evil)\", \"primary\":",
            1,
        );
        assert!(import_json(&duplicate, &ResolveContext::default())
            .unwrap_err()
            .message
            .contains("duplicate token primary"));
        let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
        value["base"].as_object_mut().unwrap().remove("dark");
        assert!(import_json(&value.to_string(), &ResolveContext::default()).is_err());
        let mut document = base();
        document.schema_version = 3;
        assert!(import_json(
            &serde_json::to_string(&document).unwrap(),
            &ResolveContext::default()
        )
        .is_err());
        let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
        value["base"]["light"]["primary"] = 123.into();
        assert!(import_json(&value.to_string(), &ResolveContext::default()).is_err());
    }

    #[test]
    fn css_scoped_identifiers_are_escaped_and_never_become_selector_code() {
        let mut document = base();
        document.id = "主题\"; } body { /*".into();
        let exported = export_css(
            &document,
            CssProfile::RustifyScoped,
            &ResolveContext::default(),
            ColorFormat::Rgb,
        )
        .unwrap();
        let imported = import_css(&exported, &base(), &ResolveContext::default()).unwrap();
        assert_eq!(imported.document.id, base().id);
        assert_eq!(imported.document.name, base().name);
        assert!(exported.contains("data-rustify-theme-css=\"主题\\\"; } body { /*\"]"));
    }

    #[test]
    fn exports_skip_unknown_json_values_and_include_packaged_font_notices() {
        let mut document = base();
        document.base.light.0.insert(
            "future-token".into(),
            "}; body { background: url(evil) }".into(),
        );
        let css = export_css(
            &document,
            CssProfile::TailwindV4,
            &crate::theme::font_context(16., false),
            ColorFormat::Rgb,
        )
        .unwrap();
        assert!(!css.contains("future-token"));
        assert!(!css.contains("url(evil)"));
        assert!(css.contains("@font-face"));
        assert!(css.contains("Requested font:"));
        assert!(css.contains("resource:"));
        for face in crate::theme::FONT_GLYPH_FALLBACKS {
            assert!(css.contains(face.resource_path));
        }
    }

    #[test]
    fn source_positions_and_nested_limits_are_bounded() {
        let error = import_css(
            "/* 🦀 */\n:root {\n--primary:var(--x);\n}",
            &base(),
            &ResolveContext::default(),
        )
        .unwrap_err();
        assert_eq!(error.field, "styles.light.primary");
        assert!(error.message.contains("line 3"));
        let nested = format!(
            "{}:root{{--primary:#fff;}}{}",
            "@layer base {".repeat(34),
            "}".repeat(34)
        );
        assert!(import_css(&nested, &base(), &ResolveContext::default()).is_err());
        assert!(import_css(
            ".outside { --primary:#fff; }",
            &base(),
            &ResolveContext::default()
        )
        .is_err());
    }

    #[test]
    fn every_css_color_notation_preserves_resolved_channels_and_alpha() {
        let mut document = base();
        document
            .base
            .light
            .0
            .insert("primary".into(), "oklch(.7 .3 20 / .4)".into());
        let context = ResolveContext::default();
        let expected = crate::theme::resolve(&document, ThemeMode::Light, &context)
            .unwrap()
            .colors["primary"];
        for format in [
            ColorFormat::Hex,
            ColorFormat::Rgb,
            ColorFormat::Hsl,
            ColorFormat::Oklch,
        ] {
            let css = export_css(&document, CssProfile::TailwindV4, &context, format).unwrap();
            let candidate = import_css(&css, &base(), &context).unwrap();
            let actual = crate::theme::resolve(&candidate.document, ThemeMode::Light, &context)
                .unwrap()
                .colors["primary"];
            let tolerance = if format == ColorFormat::Hex {
                1. / 255.
            } else {
                0.000_001
            };
            for (a, b) in [
                (actual.r, expected.r),
                (actual.g, expected.g),
                (actual.b, expected.b),
                (actual.a, expected.a),
            ] {
                assert!(
                    (a - b).abs() <= tolerance,
                    "{format:?}: {actual:?} != {expected:?}"
                );
            }
        }
    }

    #[test]
    fn v2_css_uses_scaled_resolved_metrics_and_formats_local_roles() {
        let mut document = ThemeDocument::standard("export-v2", "Export v2");
        document.config.scaling = super::super::ThemeScaling::P110;
        document.config.radius = Some(super::super::ThemeRadius::Full);
        let context = ResolveContext::default();
        let theme = resolve(&document, ThemeMode::Light, &context).unwrap();
        let css = export_css(
            &document,
            CssProfile::RustifyScoped,
            &context,
            ColorFormat::Hex,
        )
        .unwrap();
        assert!(
            css.contains(&format!("  --radius: {}px;", theme.radius_px)),
            "{css}"
        );
        assert!(
            css.contains(&format!("  --rustify-font-size: {}px;", theme.font_size_px)),
            "{css}"
        );
        assert!(
            css.contains(&format!("  --rustify-unit: {}px;", theme.spacing_px)),
            "{css}"
        );
        assert!(
            css.contains(&format!(
                "  --rustify-brand-9: {};",
                format_color(theme.brand.solid[8], ColorFormat::Hex)
            )),
            "{css}"
        );
        assert!(css.contains("--rustify-radius-full: 9999px;"), "{css}");
    }
}
