use super::{
    metrics::{letter_spacing, number},
    parse_color, parse_font_stack, parse_length, ColorFormat, LengthRule, Rgba, ThemeDocument,
    ThemeError, ThemeMode, ThemeValues, COLOR_TOKENS, COMMON_TOKENS, VALUE_TOKENS,
};
use std::collections::BTreeMap;

thread_local! {
    static RESOLUTION_COUNT: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Cumulative resolver calls on this thread, including validation and failures.
/// Compare two samples to measure an operation without resetting other consumers.
pub fn resolution_count() -> u64 {
    RESOLUTION_COUNT.with(std::cell::Cell::get)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFace {
    pub family: &'static str,
    pub resource_path: &'static str,
}

#[derive(Clone, Debug)]
pub struct ResolveContext<'a> {
    pub rem_px: f64,
    pub reduce_motion: bool,
    pub fonts: &'a [FontFace],
    pub fallback_fonts: [&'a str; 3],
}

impl Default for ResolveContext<'_> {
    fn default() -> Self {
        Self {
            rem_px: 16.,
            reduce_motion: false,
            fonts: &[],
            fallback_fonts: ["sans-serif", "serif", "monospace"],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFont {
    pub requested: Vec<String>,
    pub family: String,
    pub resource_path: Option<String>,
    pub fallback: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeDiagnostic {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowLayer {
    pub color: Rgba,
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur: f64,
    pub spread: f64,
}

// The legacy Tailwind 4.1.13 layers remain the fallback when a scope does not
// publish the new layer variables. Keep geometry in a single var() so Tailwind
// leaves the color expression at the utility element instead of rewriting it.
pub(crate) const SHADOW_FALLBACKS: &[(&str, &[(&str, &str)])] = &[
    ("shadow-2xs", &[("0 1px", "rgb(0 0 0 / 0.05)")]),
    ("shadow-xs", &[("0 1px 2px 0", "rgb(0 0 0 / 0.05)")]),
    (
        "shadow-sm",
        &[
            ("0 1px 3px 0", "rgb(0 0 0 / 0.1)"),
            ("0 1px 2px -1px", "rgb(0 0 0 / 0.1)"),
        ],
    ),
    (
        "shadow",
        &[
            ("0 1px 3px 0", "rgb(0 0 0 / 0.1)"),
            ("0 1px 2px -1px", "rgb(0 0 0 / 0.1)"),
        ],
    ),
    (
        "shadow-md",
        &[
            ("0 4px 6px -1px", "rgb(0 0 0 / 0.1)"),
            ("0 2px 4px -2px", "rgb(0 0 0 / 0.1)"),
        ],
    ),
    (
        "shadow-lg",
        &[
            ("0 10px 15px -3px", "rgb(0 0 0 / 0.1)"),
            ("0 4px 6px -4px", "rgb(0 0 0 / 0.1)"),
        ],
    ),
    (
        "shadow-xl",
        &[
            ("0 20px 25px -5px", "rgb(0 0 0 / 0.1)"),
            ("0 8px 10px -6px", "rgb(0 0 0 / 0.1)"),
        ],
    ),
    ("shadow-2xl", &[("0 25px 50px -12px", "rgb(0 0 0 / 0.25)")]),
];

pub(crate) fn shadow_utility(name: &str, fallback: &[(&str, &str)]) -> String {
    fallback.iter().enumerate().map(|(index, (geometry, color))| {
        let prefix = format!("--rustify-{name}-{index}");
        format!("var({prefix}-inactive, var({prefix}-geometry, {geometry}) var(--tw-shadow-color, var({prefix}-color, {color})))")
    }).collect::<Vec<_>>().join(", ")
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTheme {
    pub mode: ThemeMode,
    pub document_id: String,
    pub name: String,
    pub values: ThemeValues,
    pub colors: BTreeMap<&'static str, Rgba>,
    pub radius_px: f64,
    /// Small, medium, large and extra-large, in that order.
    pub radii_px: [f64; 4],
    pub spacing_px: f64,
    pub layout_gap_px: f64,
    pub font_size_px: f64,
    pub letter_spacing_em: f64,
    /// Sans, serif and mono, in that order.
    pub fonts: [ResolvedFont; 3],
    pub shadows: BTreeMap<&'static str, Vec<ShadowLayer>>,
    pub reduce_motion: bool,
    pub diagnostics: Vec<ThemeDiagnostic>,
    pub rem_px: f64,
    font_catalog: Vec<FontFace>,
    fallback_fonts: [String; 3],
    host_reduce_motion: bool,
}

impl ResolvedTheme {
    /// Complete CSSOM projection, including the opt-in runtime variables.
    pub fn properties(&self) -> Vec<(String, String)> {
        let mut props: Vec<_> = self
            .colors
            .iter()
            .map(|(token, color)| {
                (
                    format!("--{token}"),
                    super::format_color(*color, super::ColorFormat::Rgb),
                )
            })
            .collect();
        for (name, value) in [
            ("--radius", self.radius_px),
            ("--font-size", self.font_size_px),
            ("--spacing", self.layout_gap_px),
            ("--layout-gap", self.layout_gap_px),
            ("--rustify-unit", self.spacing_px),
        ] {
            props.push((name.into(), format!("{value}px")));
        }
        for (size, radius) in ["sm", "md", "lg", "xl"].into_iter().zip(self.radii_px) {
            props.push((format!("--rustify-radius-{size}"), format!("{radius}px")));
        }
        props.push((
            "--rustify-letter-spacing".into(),
            format!("{}em", self.letter_spacing_em),
        ));
        props.push((
            "--motion-duration".into(),
            super::motion(self.reduce_motion),
        ));
        for (slot, font) in ["sans", "serif", "mono"].into_iter().zip(&self.fonts) {
            let family = super::dom_font_stack(font);
            props.push((format!("--rustify-font-{slot}"), family));
        }
        props.extend(self.shadow_properties(ColorFormat::Rgb));
        props
    }

    pub(crate) fn shadow_properties(&self, format: ColorFormat) -> Vec<(String, String)> {
        let mut props = Vec::new();
        for &(size, fallback) in SHADOW_FALLBACKS {
            let layers = &self.shadows[size];
            let value = if layers.is_empty() {
                // Tailwind composes shadows and rings in one list, where
                // the standalone `none` keyword would invalidate the list.
                "0 0 #0000".into()
            } else {
                layers
                    .iter()
                    .map(|layer| {
                        format!(
                            "{}px {}px {}px {}px {}",
                            layer.offset_x,
                            layer.offset_y,
                            layer.blur,
                            layer.spread,
                            super::format_color(layer.color, format)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            props.push((format!("--rustify-{size}"), value));
            for index in 0..fallback.len() {
                let prefix = format!("--rustify-{size}-{index}");
                let (geometry, color, inactive) = match layers.get(index) {
                    Some(layer) => (
                        format!(
                            "{}px {}px {}px {}px",
                            layer.offset_x, layer.offset_y, layer.blur, layer.spread
                        ),
                        super::format_color(layer.color, format),
                        "initial",
                    ),
                    None => ("0 0".into(), "#0000".into(), "0 0 #0000"),
                };
                props.push((format!("{prefix}-geometry"), geometry));
                props.push((format!("{prefix}-color"), color));
                // `initial` makes this unregistered custom property invalid,
                // so var() evaluates its fallback at the utility element.
                // Empty layers instead select a complete transparent shadow,
                // even when that element has a shadow-color utility.
                props.push((format!("{prefix}-inactive"), inactive.into()));
            }
        }
        props
    }

    /// Re-resolve a local patch using the enclosing snapshot's context.
    pub fn patched(&self, patch: &super::ThemeValuePatch) -> Result<Self, ThemeError> {
        let values = self.values.patched(patch)?;
        let document = ThemeDocument {
            schema_version: 1,
            id: self.document_id.clone(),
            name: self.name.clone(),
            styles: super::ThemeStyles {
                light: values.clone(),
                dark: values,
            },
        };
        resolve(
            &document,
            self.mode,
            &ResolveContext {
                rem_px: self.rem_px,
                reduce_motion: self.host_reduce_motion,
                fonts: &self.font_catalog,
                fallback_fonts: std::array::from_fn(|i| self.fallback_fonts[i].as_str()),
            },
        )
    }
}

/// Resolve once, then share this snapshot between DOM and GPU projections.
pub fn resolve(
    document: &ThemeDocument,
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<ResolvedTheme, ThemeError> {
    RESOLUTION_COUNT.with(|count| count.set(count.get() + 1));
    document.validate_identity()?;
    let values = document.values(mode);
    let field_error = |token: &str, error: ThemeError| {
        ThemeError::new(format!("styles.{}.{token}", mode.as_str()), error.message)
    };
    let value = |token: &str| values.get(token).map_err(|error| field_error(token, error));
    let mut diagnostics = Vec::new();
    let mut colors = BTreeMap::new();
    for &token in COLOR_TOKENS {
        let parsed = parse_color(value(token)?)
            .map_err(|message| field_error(token, ThemeError::new(token, message)))?;
        colors.insert(token, parsed.rgba);
        if parsed.out_of_gamut {
            diagnostics.push(ThemeDiagnostic {
                field: format!("styles.{}.{token}", mode.as_str()),
                message: "Preview clipped this color to sRGB; the author value is preserved".into(),
            });
        }
    }
    for token in values.0.keys() {
        if !COLOR_TOKENS.contains(&token.as_str()) && !VALUE_TOKENS.contains(&token.as_str()) {
            diagnostics.push(ThemeDiagnostic {
                field: format!("styles.{}.{token}", mode.as_str()),
                message: "Unknown token is preserved but not used by the preview".into(),
            });
        }
    }
    for &token in COMMON_TOKENS {
        if document.styles.light.get(token)? != document.styles.dark.get(token)? {
            diagnostics.push(ThemeDiagnostic {
                field: token.into(),
                message: "Light and dark have different author values; editing this common field applies to both".into(),
            });
        }
    }
    let length = |token: &str, rule| {
        parse_length(value(token)?, context.rem_px, rule).map_err(|error| field_error(token, error))
    };
    let radius_px = length("radius", LengthRule::NonNegative)?;
    let spacing_px = length("spacing", LengthRule::Positive)?;
    let layout_gap_px = length("layout-gap", LengthRule::NonNegative)?;
    let font_size_px = length("font-size", LengthRule::Positive)?;
    let letter_spacing_em = letter_spacing(value("letter-spacing")?, font_size_px, context.rem_px)
        .map_err(|error| field_error("letter-spacing", error))?;
    let opacity = number(value("shadow-opacity")?, 0., 1.)
        .map_err(|error| field_error("shadow-opacity", error))?;
    let base_shadow = ShadowLayer {
        color: Rgba {
            a: colors["shadow-color"].a * opacity,
            ..colors["shadow-color"]
        },
        offset_x: length("shadow-offset-x", LengthRule::Signed)?,
        offset_y: length("shadow-offset-y", LengthRule::Signed)?,
        blur: length("shadow-blur", LengthRule::NonNegative)?,
        spread: length("shadow-spread", LengthRule::Signed)?,
    };
    let mut shadows = BTreeMap::new();
    for (name, multiplier, second) in [
        ("shadow-2xs", 0.5, None),
        ("shadow-xs", 0.5, None),
        ("shadow-sm", 1., Some((1., 2.))),
        ("shadow", 1., Some((1., 2.))),
        ("shadow-md", 1., Some((2., 4.))),
        ("shadow-lg", 1., Some((4., 6.))),
        ("shadow-xl", 1., Some((8., 10.))),
        ("shadow-2xl", 2.5, None),
    ] {
        let mut layers = Vec::new();
        if base_shadow.color.a > 0. {
            layers.push(ShadowLayer {
                color: Rgba {
                    a: (base_shadow.color.a * multiplier).min(1.),
                    ..base_shadow.color
                },
                ..base_shadow
            });
            if let Some((offset_y, blur)) = second {
                layers.push(ShadowLayer {
                    offset_y,
                    blur,
                    spread: base_shadow.spread - 1.,
                    ..base_shadow
                });
            }
        }
        shadows.insert(name, layers);
    }
    let requested_motion = match value("reduce-motion")? {
        "true" => true,
        "false" => false,
        _ => {
            return Err(field_error(
                "reduce-motion",
                ThemeError::new("reduce-motion", "use true or false"),
            ))
        }
    };
    let mut font = |token: &str, index: usize| -> Result<ResolvedFont, ThemeError> {
        let requested =
            parse_font_stack(value(token)?).map_err(|error| field_error(token, error))?;
        let matched = requested.iter().find_map(|family| {
            context
                .fonts
                .iter()
                .find(|font| font.family.eq_ignore_ascii_case(family))
        });
        let face = matched.or_else(|| {
            context.fonts.iter().find(|font| {
                font.family
                    .eq_ignore_ascii_case(context.fallback_fonts[index])
            })
        });
        let family = face
            .map(|font| font.family)
            .unwrap_or(context.fallback_fonts[index]);
        let fallback = face.is_none() || !family.eq_ignore_ascii_case(&requested[0]);
        if fallback {
            diagnostics.push(ThemeDiagnostic {
                field: format!("styles.{}.{token}", mode.as_str()),
                message: format!(
                    "Requested {}; preview uses {family}{}",
                    requested[0],
                    if face.is_none() {
                        " (no packaged font available)"
                    } else {
                        ""
                    }
                ),
            });
        }
        Ok(ResolvedFont {
            requested,
            family: family.into(),
            resource_path: face.map(|font| font.resource_path.to_owned()),
            fallback,
        })
    };
    let fonts = [
        font("font-sans", 0)?,
        font("font-serif", 1)?,
        font("font-mono", 2)?,
    ];
    Ok(ResolvedTheme {
        mode,
        document_id: document.id.clone(),
        name: document.name.clone(),
        values: values.clone(),
        colors,
        radius_px,
        radii_px: [
            (radius_px - 4.).max(0.),
            (radius_px - 2.).max(0.),
            radius_px,
            radius_px + 4.,
        ],
        spacing_px,
        layout_gap_px,
        font_size_px,
        letter_spacing_em,
        fonts,
        shadows,
        reduce_motion: context.reduce_motion || requested_motion,
        diagnostics,
        rem_px: context.rem_px,
        font_catalog: context.fonts.to_vec(),
        fallback_fonts: context.fallback_fonts.map(str::to_owned),
        host_reduce_motion: context.reduce_motion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn document() -> ThemeDocument {
        ThemeDocument::from_legacy(Theme::light(), Theme::dark())
    }

    #[test]
    fn legacy_resolves_without_losing_alpha_or_coupling_sizes() {
        let mut document = document();
        document
            .styles
            .light
            .0
            .insert("primary".into(), "#12345680".into());
        document
            .styles
            .light
            .0
            .insert("radius".into(), "0.3rem".into());
        let context = ResolveContext {
            rem_px: 20.,
            ..Default::default()
        };
        let resolved = resolve(&document, ThemeMode::Light, &context).unwrap();
        assert_eq!(resolved.radius_px, 6.);
        assert_eq!(resolved.radii_px, [2., 4., 6., 10.]);
        assert_eq!(resolved.spacing_px, 5.);
        assert_eq!(resolved.layout_gap_px, 8.);
        assert_eq!(resolved.colors["primary"].a, 128. / 255.);
        assert_eq!(resolved.values.get("primary").unwrap(), "#12345680");
        assert!(resolved.shadows.values().all(Vec::is_empty));
    }

    #[test]
    fn radius_zero_is_valid_and_never_derives_negative_corners() {
        let mut document = document();
        document.styles.light.0.insert("radius".into(), "0".into());
        assert_eq!(
            resolve(&document, ThemeMode::Light, &ResolveContext::default())
                .unwrap()
                .radii_px,
            [0., 0., 0., 4.]
        );
    }

    #[test]
    fn invalid_fields_report_mode_and_token_without_modifying_author_data() {
        for (token, invalid) in [
            ("primary", "var(--x)"),
            ("radius", "-1px"),
            ("spacing", "0px"),
            ("font-size", "0px"),
            ("shadow-opacity", "1.1"),
            ("shadow-blur", "-2px"),
            ("reduce-motion", "maybe"),
        ] {
            let mut document = document();
            document.styles.dark.0.insert(token.into(), invalid.into());
            let before = document.clone();
            assert_eq!(
                resolve(&document, ThemeMode::Dark, &ResolveContext::default())
                    .unwrap_err()
                    .field,
                format!("styles.dark.{token}")
            );
            assert_eq!(document, before);
        }
    }

    #[test]
    fn missing_tokens_fail_and_unknown_tokens_generate_diagnostics() {
        let mut document = document();
        document.styles.light.0.remove("chart-5");
        assert_eq!(
            resolve(&document, ThemeMode::Light, &ResolveContext::default())
                .unwrap_err()
                .field,
            "styles.light.chart-5"
        );
        document
            .styles
            .light
            .0
            .insert("chart-5".into(), "#123456".into());
        document
            .styles
            .light
            .0
            .insert("custom".into(), "ignored".into());
        assert!(
            resolve(&document, ThemeMode::Light, &ResolveContext::default())
                .unwrap()
                .diagnostics
                .iter()
                .any(|d| d.field == "styles.light.custom")
        );
    }

    #[test]
    fn gamut_and_common_field_differences_are_visible_and_author_values_survive() {
        let mut document = document();
        document
            .styles
            .light
            .0
            .insert("primary".into(), "oklch(0.7 0.4 40)".into());
        document.styles.dark.0.insert("radius".into(), "2px".into());
        let resolved = resolve(&document, ThemeMode::Light, &ResolveContext::default()).unwrap();
        assert!(resolved
            .diagnostics
            .iter()
            .any(|d| d.field == "styles.light.primary"));
        assert!(resolved.diagnostics.iter().any(|d| d.field == "radius"));
        assert_eq!(
            document.styles.light.get("primary").unwrap(),
            "oklch(0.7 0.4 40)"
        );
    }

    #[test]
    fn fonts_pick_packaged_families_and_report_the_requested_fallback() {
        let mut document = document();
        document
            .styles
            .light
            .0
            .insert("font-sans".into(), "Unknown, Noto Sans, sans-serif".into());
        let fonts = [FontFace {
            family: "Noto Sans",
            resource_path: "resources/NotoSans-Regular.ttf",
        }];
        let context = ResolveContext {
            fonts: &fonts,
            fallback_fonts: ["Noto Sans"; 3],
            ..Default::default()
        };
        let resolved = resolve(&document, ThemeMode::Light, &context).unwrap();
        assert_eq!(resolved.fonts[0].family, "Noto Sans");
        assert!(resolved.fonts[0].fallback);
        assert_eq!(resolved.fonts[0].requested[0], "Unknown");
        assert_eq!(
            resolved.fonts[1].resource_path.as_deref(),
            Some("resources/NotoSans-Regular.ttf")
        );
    }

    #[test]
    fn shadows_derive_layers_from_the_same_geometry_and_rgba() {
        let mut document = document();
        for (token, value) in [
            ("shadow-color", "rgba(20,40,60,0.5)"),
            ("shadow-opacity", "0.4"),
            ("shadow-blur", "3px"),
            ("shadow-offset-x", "-2px"),
            ("shadow-offset-y", "5px"),
            ("shadow-spread", "-1px"),
        ] {
            document.styles.light.0.insert(token.into(), value.into());
        }
        let resolved = resolve(&document, ThemeMode::Light, &ResolveContext::default()).unwrap();
        assert_eq!(resolved.shadows["shadow-lg"].len(), 2);
        let layer = resolved.shadows["shadow-lg"][0];
        assert!((layer.color.a - 0.2).abs() < 1e-9);
        assert_eq!(
            (layer.offset_x, layer.offset_y, layer.blur, layer.spread),
            (-2., 5., 3., -1.)
        );
        let second = resolved.shadows["shadow-lg"][1];
        assert_eq!(
            (second.offset_x, second.offset_y, second.blur, second.spread),
            (-2., 4., 6., -2.)
        );
    }

    #[test]
    fn host_reduced_motion_overrides_the_document() {
        let context = ResolveContext {
            reduce_motion: true,
            ..Default::default()
        };
        assert!(
            resolve(&document(), ThemeMode::Light, &context)
                .unwrap()
                .reduce_motion
        );
    }

    #[test]
    fn css_projection_separates_legacy_gap_and_new_unit_and_emits_zero_shadows() {
        let resolved = resolve(&document(), ThemeMode::Light, &ResolveContext::default()).unwrap();
        let props: BTreeMap<_, _> = resolved.properties().into_iter().collect();
        assert_eq!(props["--spacing"], "8px");
        assert_eq!(props["--rustify-unit"], "4px");
        assert_eq!(props["--rustify-radius-sm"], "2px");
        assert_eq!(props["--rustify-radius-xl"], "10px");
        assert_eq!(props["--rustify-shadow-lg"], "0 0 #0000");
        assert_eq!(props["--rustify-font-sans"], "sans-serif");
        assert!(props.contains_key("--card-foreground"));
        assert!(props.contains_key("--sidebar-ring"));
    }

    #[test]
    fn empty_shadow_layers_are_transparent_even_with_a_color_utility() {
        let theme = resolve(&document(), ThemeMode::Light, &ResolveContext::default()).unwrap();
        let properties: BTreeMap<_, _> = theme.properties().into_iter().collect();
        for layer in 0..2 {
            assert_eq!(
                properties
                    .get(&format!("--rustify-shadow-lg-{layer}-inactive"))
                    .map(String::as_str),
                Some("0 0 #0000")
            );
            assert_eq!(
                properties
                    .get(&format!("--rustify-shadow-lg-{layer}-geometry"))
                    .map(String::as_str),
                Some("0 0")
            );
        }
    }

    #[test]
    fn sdk_and_export_use_the_same_shadow_utility_with_legacy_fallbacks() {
        let sdk = include_str!("../../../rustify-components/css/theme-v4.css");
        for &(name, fallback) in SHADOW_FALLBACKS {
            assert!(sdk.contains(&format!("--{name}: {};", shadow_utility(name, fallback))));
            for &(geometry, rgba) in fallback {
                assert!(sdk.contains(geometry));
                assert!(sdk.contains(rgba));
            }
        }
    }

    #[test]
    fn boundary_patch_recomputes_geometry_and_shadows_using_host_rem_context() {
        let resolved = resolve(
            &document(),
            ThemeMode::Dark,
            &ResolveContext {
                rem_px: 20.,
                ..Default::default()
            },
        )
        .unwrap();
        let patch = super::super::ThemeValuePatch(BTreeMap::from([
            ("radius".into(), ".5rem".into()),
            ("shadow-opacity".into(), "0.5".into()),
            ("shadow-blur".into(), "4px".into()),
        ]));
        let local = resolved.patched(&patch).unwrap();
        assert_eq!(local.mode, ThemeMode::Dark);
        assert_eq!(local.radii_px, [6., 8., 10., 14.]);
        assert_eq!(local.shadows["shadow-lg"][0].color.a, 0.5);
        assert_eq!(local.colors["primary"], resolved.colors["primary"]);
        assert_eq!(
            resolved
                .patched(&super::super::ThemeValuePatch::default())
                .unwrap()
                .radii_px,
            resolved.radii_px
        );
    }
}
