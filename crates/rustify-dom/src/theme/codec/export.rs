//! Prepared legacy exchange and the resolved properties shared by CSS profiles.

use super::{check_document_size, check_size, known, validate, validate_value};
use crate::theme::{
    dom_font_stack, format_color, parse_color, Appearance, ColorFormat, FontResolution,
    ResolveContext, ResolvedTheme, ThemeConfig, ThemeDocument, ThemeError, ThemeMode, ThemeStyles,
    ThemeValues, COLOR_TOKENS, VALUE_TOKENS,
};
use std::collections::BTreeMap;

/// One concrete limitation to review before using a legacy v1 candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacyExportLoss {
    pub field: &'static str,
    pub message: String,
}

/// A prepared v1 snapshot. Present `losses` before copying or downloading `json`.
/// The JSON preserves legacy token values, rather than complete v2 appearance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacyJsonExport {
    pub json: String,
    pub losses: Vec<LegacyExportLoss>,
}

/// Explicitly prepares a lossy v1 snapshot of both resolved modes.
///
/// Unknown base strings remain intact. Known values are flattened into the
/// legacy token grammar, with already-scaled metrics. An unrepresentable value
/// or a canonical input/output larger than 256 KiB rejects the whole export.
pub fn export_json_v1_lossy(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
) -> Result<LegacyJsonExport, ThemeError> {
    check_document_size(document)?;
    let (themes, _) = validate(document, context)?;
    let styles = ThemeStyles {
        light: legacy_values(&themes[0]),
        dark: legacy_values(&themes[1]),
    };
    let legacy_context = ResolveContext {
        font_resolution: FontResolution::Packaged,
        ..context.clone()
    };
    for (mode, values) in [
        (ThemeMode::Light, &styles.light),
        (ThemeMode::Dark, &styles.dark),
    ] {
        for &token in COLOR_TOKENS.iter().chain(VALUE_TOKENS) {
            validate_value(token, values.get(token)?, &legacy_context).map_err(|error| {
                ThemeError::new(
                    format!("legacy.styles.{}.{token}", mode.as_str()),
                    format!("v1 cannot represent the resolved value: {}", error.message),
                )
            })?;
        }
    }
    #[derive(serde::Serialize)]
    struct Wire<'a> {
        schema_version: u32,
        id: &'a str,
        name: &'a str,
        styles: &'a ThemeStyles,
    }
    let json = serde_json::to_string(&Wire {
        schema_version: 1,
        id: &document.id,
        name: &document.name,
        styles: &styles,
    })
    .map_err(|error| ThemeError::new("json", error.to_string()))?;
    check_size(&json)?;
    Ok(LegacyJsonExport {
        json,
        losses: legacy_losses(document, &themes),
    })
}

fn legacy_losses(document: &ThemeDocument, themes: &[ResolvedTheme; 2]) -> Vec<LegacyExportLoss> {
    let mut losses = Vec::new();
    let mut loss = |field, message| losses.push(LegacyExportLoss { field, message });
    if let Some(color) = document.config.accent_color {
        loss("config.accent_color", format!("Brand palette selection {} is replaced by final legacy colors; its palette identity is lost.", color.as_str()));
    }
    if let Some(gray) = document.config.gray_color {
        loss("config.gray_color", format!("Neutral selection {} (resolved {}) is flattened; auto matching and the selected family cannot be recovered.", gray.as_str(), themes[0].gray.family.name()));
    }
    if let Some(panel) = document.config.panel_background {
        loss("config.panel_background", format!("Panel selection {} is flattened; the independent panel and surface roles are not v1 tokens.", panel.as_str()));
    }
    if document.config.appearance != Appearance::Inherit {
        loss("config.appearance", format!("Authored appearance {} is lost; v1 contains two modes without a mode-selection instruction.", document.config.appearance.as_str()));
    }
    if let Some(radius) = document.config.radius {
        loss("config.radius", format!("Radius selection {} becomes one finite legacy radius; full, thumb and component radius roles cannot be reconstructed.", radius.as_str()));
    }
    if document.config.scaling != ThemeConfig::custom().scaling {
        loss("config.scaling", format!("Scaling {}% is baked into legacy metrics; its author intent and future scaling behavior are lost.", document.config.scaling.percent()));
    }
    loss("sources", "Known author values are replaced by resolved values. Base/configuration/override origins, original notation and requested font fallback intent cannot be recovered.".into());
    loss("derived_roles", "The old 49-token table cannot encode brand/gray solid and alpha scales, supplementary color roles, panel/surface roles, finite/full/thumb radii or component spacing, typography and control-size scales. Importing v1 does not restore the complete v2 component appearance.".into());
    if document.config.scaling != ThemeConfig::custom().scaling
        && themes
            .iter()
            .any(|theme| theme.shadows.values().any(|layers| layers.len() > 1))
    {
        loss("shadows", "The resolved fixed secondary shadow layers are not expressible in v1's scalar shadow parameters; their scaled offsets, blur and spread cannot be restored exactly.".into());
    }
    losses
}

/// Final legacy values used by both exports. Unknown strings are JSON-only.
fn legacy_values(theme: &ResolvedTheme) -> ThemeValues {
    let mut values: BTreeMap<_, _> = theme
        .values
        .0
        .iter()
        .filter(|(token, _)| !known(token))
        .map(|(token, value)| (token.clone(), value.clone()))
        .collect();
    for &token in COLOR_TOKENS {
        values.insert(
            token.into(),
            format_color(theme.colors[token], ColorFormat::Rgb),
        );
    }
    for (token, value) in [
        ("radius", theme.radius_px),
        ("font-size", theme.font_size_px),
        ("spacing", theme.spacing_px),
        ("layout-gap", theme.layout_gap_px),
    ] {
        values.insert(token.into(), format!("{value}px"));
    }
    for (token, font) in ["font-sans", "font-serif", "font-mono"]
        .into_iter()
        .zip(&theme.fonts)
    {
        values.insert(token.into(), dom_font_stack(font));
    }
    values.insert(
        "letter-spacing".into(),
        format!("{}em", theme.letter_spacing_em),
    );
    values.insert("reduce-motion".into(), theme.reduce_motion.to_string());
    // Invisible shadows have no effective geometry. Visible first layers are
    // the resolved scalar layer; later fixed layers are disclosed as a loss.
    let shadow = theme.shadows["shadow-lg"].first();
    for (token, value) in [
        ("shadow-offset-x", shadow.map_or(0., |layer| layer.offset_x)),
        ("shadow-offset-y", shadow.map_or(0., |layer| layer.offset_y)),
        ("shadow-blur", shadow.map_or(0., |layer| layer.blur)),
        ("shadow-spread", shadow.map_or(0., |layer| layer.spread)),
    ] {
        values.insert(token.into(), format!("{value}px"));
    }
    if let Some(opacity) = theme.values.0.get("shadow-opacity") {
        values.insert("shadow-opacity".into(), opacity.clone());
    }
    ThemeValues(values)
}

pub(super) fn css_properties(
    theme: &ResolvedTheme,
    scoped: bool,
    context: &ResolveContext<'_>,
    format: ColorFormat,
) -> Result<BTreeMap<String, String>, ThemeError> {
    let mut properties: BTreeMap<_, _> = theme.properties().into_iter().collect();
    for value in properties.values_mut() {
        if value.starts_with("rgb(") {
            let color = parse_color(value).map_err(|message| ThemeError::new("css", message))?;
            *value = format_color(color.rgba, format);
        }
    }
    properties.extend(theme.shadow_properties(format));
    let values = legacy_values(theme);
    for &token in VALUE_TOKENS {
        let value = values.get(token)?;
        validate_value(token, value, context)?;
        properties.insert(
            format!("--{}", super::css_token(token, scoped)),
            value.into(),
        );
    }
    // Tailwind's spacing variable is its utility unit. The Rustify runtime
    // reserves that spelling for layout gap and reads its unit separately.
    if !scoped {
        properties.insert("--spacing".into(), properties["--rustify-unit"].clone());
    }
    Ok(properties)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{
        export_css, export_json, font_context, import_css, import_json, resolve, AccentColor,
        CssProfile, GrayColor, Theme, ThemeRadius, ThemeScaling,
    };

    fn configured() -> ThemeDocument {
        let mut document = ThemeDocument::standard("exchange", "Exchange");
        document.config.accent_color = Some(AccentColor::Mint);
        document.config.gray_color = Some(GrayColor::Auto);
        document.config.appearance = Appearance::Dark;
        document.config.radius = Some(ThemeRadius::Full);
        document.config.scaling = ThemeScaling::P110;
        for (mode, size, spacing, radius) in [
            (ThemeMode::Light, "18px", "0.5rem", "8px"),
            (ThemeMode::Dark, "20px", "0.75rem", "10px"),
        ] {
            let values = document.overrides.values_mut(mode);
            for (token, value) in [
                ("font-size", size),
                ("spacing", spacing),
                ("radius", radius),
                ("shadow-color", "rgb(30 10 20 / .6)"),
                ("shadow-opacity", ".4"),
                ("shadow-offset-x", "2px"),
                ("shadow-offset-y", "4px"),
                ("shadow-blur", "8px"),
                ("shadow-spread", "1px"),
                ("letter-spacing", "1px"),
                ("reduce-motion", "true"),
            ] {
                values.0.insert(token.into(), value.into());
            }
        }
        document.base.light.0.insert(
            "future-🦀".into(),
            "light \\\" }; body { url(never) }".into(),
        );
        document
            .base
            .dark
            .0
            .insert("future-🦀".into(), "dark 未知 / \"quoted\"".into());
        document
    }

    #[test]
    fn lossy_v1_is_a_strict_two_mode_snapshot_and_discloses_unrepresentable_intent() {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct LegacyWire {
            schema_version: u32,
            id: String,
            name: String,
            styles: ThemeStyles,
        }
        let document = configured();
        let original = document.clone();
        let context = font_context(20., false);
        let candidate = export_json_v1_lossy(&document, &context).unwrap();
        let wire: LegacyWire = serde_json::from_str(&candidate.json).unwrap();
        assert_eq!(wire.schema_version, 1);
        assert_eq!(wire.id, document.id);
        assert_eq!(wire.name, document.name);
        let fields: Vec<_> = candidate.losses.iter().map(|loss| loss.field).collect();
        for field in [
            "config.accent_color",
            "config.gray_color",
            "config.panel_background",
            "config.appearance",
            "config.radius",
            "config.scaling",
            "sources",
            "derived_roles",
            "shadows",
        ] {
            assert!(fields.contains(&field), "missing {field}: {fields:?}");
        }
        assert!(candidate
            .losses
            .iter()
            .any(|loss| loss.message.contains("auto") && loss.message.contains("sage")));
        assert!(candidate
            .losses
            .iter()
            .any(|loss| loss.message.contains("full")));
        assert!(candidate
            .losses
            .iter()
            .any(|loss| loss.message.contains("110%")));
        let imported = import_json(&candidate.json, &context).unwrap().document;
        assert_eq!(imported.config, ThemeConfig::custom());
        for (mode, legacy_values) in [
            (ThemeMode::Light, wire.styles.light),
            (ThemeMode::Dark, wire.styles.dark),
        ] {
            let expected = resolve(&document, mode, &context).unwrap();
            let actual = resolve(&imported, mode, &context).unwrap();
            assert_eq!(
                legacy_values.get("future-🦀").unwrap(),
                document.base_values(mode).get("future-🦀").unwrap()
            );
            for &token in COLOR_TOKENS.iter().chain(VALUE_TOKENS) {
                assert!(legacy_values.0.contains_key(token), "{mode:?}: {token}");
            }
            for token in COLOR_TOKENS {
                assert_eq!(
                    format_color(actual.colors[token], ColorFormat::Rgb),
                    format_color(expected.colors[token], ColorFormat::Rgb),
                    "{mode:?}: {token}"
                );
            }
            assert_eq!(actual.radius_px, expected.radius_px);
            assert_eq!(actual.font_size_px, expected.font_size_px);
            assert_eq!(actual.spacing_px, expected.spacing_px);
            assert_eq!(actual.layout_gap_px, expected.layout_gap_px);
            assert_eq!(actual.letter_spacing_em, expected.letter_spacing_em);
            assert_eq!(actual.reduce_motion, expected.reduce_motion);
            assert_eq!(actual.fonts[0].family, expected.fonts[0].family);
            assert_eq!(
                actual.shadows["shadow-lg"][0],
                expected.shadows["shadow-lg"][0]
            );
            assert_ne!(
                actual.shadows["shadow-lg"][1],
                expected.shadows["shadow-lg"][1]
            );
        }
        assert_eq!(document, original);
    }

    #[test]
    fn default_json_stays_complete_v2_after_preparing_legacy_exchange() {
        let document = configured();
        let context = font_context(16., false);
        let _ = export_json_v1_lossy(&document, &context).unwrap();
        let json = export_json(&document, &context).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["schema_version"], 2);
        for field in ["base", "config", "overrides"] {
            assert!(value.get(field).is_some(), "missing {field}");
        }
        assert_eq!(import_json(&json, &context).unwrap().document, document);
    }

    fn compact_with_short_colors() -> ThemeDocument {
        let mut styles = ThemeDocument::from_legacy(Theme::light(), Theme::dark()).base;
        for values in [&mut styles.light, &mut styles.dark] {
            for &token in COLOR_TOKENS {
                values.0.insert(token.into(), "#fff".into());
            }
        }
        styles.light.0.insert("future".into(), String::new());
        ThemeDocument::custom("bounded", "Bounded", styles)
    }

    fn utf8_payload(bytes: usize) -> String {
        "🦀".repeat(bytes / 4) + &"x".repeat(bytes % 4)
    }

    #[test]
    fn legacy_utf8_output_is_bounded_without_dropping_unknown_strings() {
        let mut document = compact_with_short_colors();
        let context = ResolveContext::default();
        let input_size = export_json(&document, &context).unwrap().len();
        let output_size = export_json_v1_lossy(&document, &context)
            .unwrap()
            .json
            .len();
        assert!(output_size > input_size);
        let payload = utf8_payload(super::super::THEME_INPUT_LIMIT - output_size);
        document
            .base
            .light
            .0
            .insert("future".into(), payload.clone());
        let candidate = export_json_v1_lossy(&document, &context).unwrap();
        assert_eq!(candidate.json.len(), super::super::THEME_INPUT_LIMIT);
        assert_eq!(
            import_json(&candidate.json, &context)
                .unwrap()
                .document
                .base
                .light
                .get("future")
                .unwrap(),
            payload
        );
        document.base.light.0.get_mut("future").unwrap().push('x');
        check_document_size(&document).unwrap();
        assert!(export_json_v1_lossy(&document, &context)
            .unwrap_err()
            .message
            .contains("256 KiB"));
        document.base.light.0.insert(
            "future".into(),
            utf8_payload(super::super::THEME_INPUT_LIMIT),
        );
        assert!(export_json_v1_lossy(&document, &context).is_err());
    }

    #[test]
    fn an_unrepresentable_final_v1_length_rejects_instead_of_clipping() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document.config.scaling = ThemeScaling::P110;
        document
            .overrides
            .light
            .0
            .insert("radius".into(), "4096px".into());
        let error = export_json_v1_lossy(&document, &ResolveContext::default()).unwrap_err();
        assert_eq!(error.field, "legacy.styles.light.radius");
        assert!(error.message.contains("v1 cannot represent"));
    }

    fn block(css: &str, selector: &str) -> BTreeMap<String, String> {
        let header = format!("\n{selector} {{\n");
        let content = css
            .split_once(&header)
            .unwrap()
            .1
            .split_once("\n}")
            .unwrap()
            .0;
        let declarations: Vec<_> = content
            .lines()
            .map(|line| {
                let (name, value) = line.trim().trim_end_matches(';').split_once(": ").unwrap();
                (name.to_owned(), value.to_owned())
            })
            .collect();
        let count = declarations.len();
        let result: BTreeMap<_, _> = declarations.into_iter().collect();
        assert_eq!(result.len(), count, "duplicate runtime declarations");
        result
    }

    #[test]
    fn every_profile_and_notation_contains_the_complete_resolved_two_mode_projection() {
        let document = configured();
        let context = font_context(20., false);
        for profile in [CssProfile::TailwindV4, CssProfile::RustifyScoped] {
            let light = if profile == CssProfile::TailwindV4 {
                ":root"
            } else {
                "[data-rustify-scope][data-rustify-theme-css=\"exchange\"]"
            };
            let dark = if profile == CssProfile::TailwindV4 {
                ".dark"
            } else {
                "[data-rustify-scope][data-rustify-theme-css=\"exchange\"][data-theme=\"dark\"]"
            };
            for notation in [
                ColorFormat::Hex,
                ColorFormat::Rgb,
                ColorFormat::Hsl,
                ColorFormat::Oklch,
            ] {
                let css = export_css(&document, profile, &context, notation).unwrap();
                assert!(!css.contains("future-🦀"));
                assert!(!css.contains("url(never)"));
                assert!(!css.contains("box-shadow:"));
                assert!(css.contains("FONT-LICENSES.md"));
                assert!(css.contains("var(--tw-shadow-color, var(--rustify-shadow-lg-0-color,"));
                if profile == CssProfile::RustifyScoped {
                    assert!(!css.contains("\n:root {"));
                    assert!(!css.contains("\n.dark {"));
                }
                for (mode, selector) in [(ThemeMode::Light, light), (ThemeMode::Dark, dark)] {
                    let expected = resolve(&document, mode, &context).unwrap();
                    let actual = block(&css, selector);
                    let shadows: BTreeMap<_, _> =
                        expected.shadow_properties(notation).into_iter().collect();
                    for (property, value) in expected.properties() {
                        let found = actual.get(&property).unwrap_or_else(|| {
                            panic!("{profile:?} {notation:?} {mode:?}: missing {property}")
                        });
                        if let Some(value) = shadows.get(&property) {
                            assert_eq!(found, value, "{property}");
                        } else if let Ok(color) = parse_color(&value) {
                            let actual = parse_color(found).unwrap().rgba;
                            let tolerance = if notation == ColorFormat::Hex {
                                1. / 255.
                            } else {
                                0.000_001
                            };
                            for (a, b) in [
                                (actual.r, color.rgba.r),
                                (actual.g, color.rgba.g),
                                (actual.b, color.rgba.b),
                                (actual.a, color.rgba.a),
                            ] {
                                assert!(
                                    (a - b).abs() <= tolerance,
                                    "{property}: {found} != {value}"
                                );
                            }
                        } else if property == "--spacing" && profile == CssProfile::TailwindV4 {
                            assert_eq!(found, &format!("{}px", expected.spacing_px));
                        } else {
                            assert_eq!(found, &value, "{property}");
                        }
                    }
                    assert_eq!(
                        actual["--rustify-font-size"],
                        format!("{}px", expected.font_size_px)
                    );
                    assert_eq!(
                        actual["--rustify-layout-gap"],
                        format!("{}px", expected.layout_gap_px)
                    );
                    assert_eq!(actual["--rustify-radius-full"], "9999px");
                    assert_eq!(actual["--motion-duration"], "0ms");
                }
                let candidate = import_css(&css, &document, &context).unwrap();
                assert_eq!(candidate.document.config, ThemeConfig::custom());
                for mode in [ThemeMode::Light, ThemeMode::Dark] {
                    let expected = resolve(&document, mode, &context).unwrap();
                    let actual = resolve(&candidate.document, mode, &context).unwrap();
                    assert_eq!(actual.radius_px, expected.radius_px);
                    assert_eq!(actual.font_size_px, expected.font_size_px);
                    assert_eq!(actual.spacing_px, expected.spacing_px);
                    assert_eq!(actual.layout_gap_px, expected.layout_gap_px);
                }
            }
        }
    }

    #[test]
    fn font_urls_are_escaped_external_references_with_license_notices() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document
            .base
            .light
            .0
            .insert("font-sans".into(), "Brand Font".into());
        let path = "resources/Font\\\"; } body { /*.ttf";
        let fonts = [crate::theme::FontFace {
            family: "Brand Font",
            resource_path: path,
        }];
        let context = ResolveContext {
            fonts: &fonts,
            fallback_fonts: ["Brand Font"; 3],
            ..ResolveContext::default()
        };
        let css = export_css(
            &document,
            CssProfile::RustifyScoped,
            &context,
            ColorFormat::Rgb,
        )
        .unwrap();
        let url = format!("src: url({})", super::super::css_string(path));
        assert_eq!(css.matches(&url).count(), 1);
        assert!(css.contains("FONT-LICENSES.md"));
        assert!(!css.contains("data:"));
        assert!(!css.contains("\nbody {"));
        assert!(import_css(&css, &document, &context).is_ok());
        for path in [
            "data:font/ttf;base64,AAAA",
            "JavaScript:alert(1)",
            "vbscript:execute",
            "resources/\nFont.ttf",
        ] {
            let fonts = [crate::theme::FontFace {
                family: "Brand Font",
                resource_path: path,
            }];
            let context = ResolveContext {
                fonts: &fonts,
                ..context.clone()
            };
            assert_eq!(
                export_css(
                    &document,
                    CssProfile::TailwindV4,
                    &context,
                    ColorFormat::Rgb
                )
                .unwrap_err()
                .field,
                "fonts"
            );
        }
    }

    #[test]
    fn css_font_strings_remain_data_and_unrepresentable_v1_fonts_are_rejected() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document.base.light.0.insert(
            "font-sans".into(),
            r#""A \"quoted\" \\ face */ ; } body { color:red }", serif"#.into(),
        );
        let context = ResolveContext {
            font_resolution: FontResolution::Css,
            ..Default::default()
        };
        let expected = resolve(&document, ThemeMode::Light, &context).unwrap();
        for profile in [CssProfile::TailwindV4, CssProfile::RustifyScoped] {
            let css = export_css(&document, profile, &context, ColorFormat::Rgb).unwrap();
            let candidate = import_css(&css, &document, &context).unwrap();
            let actual = resolve(&candidate.document, ThemeMode::Light, &context).unwrap();
            assert_eq!(
                dom_font_stack(&actual.fonts[0]),
                dom_font_stack(&expected.fonts[0])
            );
            assert!(!css.contains("\nbody {"));
            assert!(!css.contains("@font-face"));
        }
        let error = export_json_v1_lossy(&document, &context).unwrap_err();
        assert_eq!(error.field, "legacy.styles.light.font-sans");
    }
}
