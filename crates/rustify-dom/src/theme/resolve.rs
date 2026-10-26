use super::{
    metrics::{letter_spacing, number},
    parse_color, parse_css_font_stack, parse_font_stack, parse_length, ColorFormat, ConfigChange,
    EffectiveThemeConfig, LengthRule, ResolvedPalette, Rgba, ThemeBoundaryPatch, ThemeConfigGroup,
    ThemeDocument, ThemeError, ThemeMode, ThemeRadius, ThemeValues, COLOR_TOKENS, COMMON_TOKENS,
    VALUE_TOKENS,
};
use std::collections::BTreeMap;
use std::sync::Arc;

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

/// Whether font names select SDK resources or remain CSS choices for the browser.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontResolution {
    #[default]
    Packaged,
    Css,
}

#[derive(Clone, Debug)]
pub struct ResolveContext<'a> {
    pub rem_px: f64,
    pub reduce_motion: bool,
    pub fonts: &'a [FontFace],
    pub fallback_fonts: [&'a str; 3],
    pub font_resolution: FontResolution,
}

impl Default for ResolveContext<'_> {
    fn default() -> Self {
        Self {
            rem_px: 16.,
            reduce_motion: false,
            fonts: &[],
            fallback_fonts: ["sans-serif", "serif", "monospace"],
            font_resolution: FontResolution::Packaged,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFont {
    pub requested: Vec<String>,
    pub family: String,
    pub resource_path: Option<String>,
    pub fallback: bool,
    pub font_resolution: FontResolution,
    /// A validated browser stack. It says nothing about installed or loaded faces.
    pub css_stack: Option<String>,
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

/// Where an effective, unscaled semantic author value came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeValueOrigin {
    Base,
    Configuration(ThemeConfigGroup),
    DocumentOverride,
    BoundaryConfiguration {
        depth: usize,
        group: ThemeConfigGroup,
    },
    BoundaryOverride {
        depth: usize,
    },
}

impl ThemeValueOrigin {
    fn field(self, mode: ThemeMode, token: &str) -> String {
        match self {
            Self::Base => format!("base.{}.{token}", mode.as_str()),
            Self::DocumentOverride => format!("overrides.{}.{token}", mode.as_str()),
            Self::Configuration(group) => format!("config.{}", group.field()),
            Self::BoundaryConfiguration { depth, group } => {
                format!("boundary[{depth}].config.{}", group.field())
            }
            Self::BoundaryOverride { depth } => {
                format!("boundary[{depth}].{}.{}", mode.as_str(), token)
            }
        }
    }
}

/// Immutable two-mode author data and the runtime boundaries applied to it.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeSource {
    document: Arc<ThemeDocument>,
    boundaries: Vec<ThemeBoundaryPatch>,
}

struct AuthorValues {
    values: ThemeValues,
    config: EffectiveThemeConfig,
    origins: BTreeMap<&'static str, ThemeValueOrigin>,
}

impl ThemeSource {
    pub fn new(document: Arc<ThemeDocument>) -> Self {
        Self {
            document,
            boundaries: Vec::new(),
        }
    }
    pub fn document(&self) -> &ThemeDocument {
        &self.document
    }
    pub fn boundaries(&self) -> &[ThemeBoundaryPatch] {
        &self.boundaries
    }

    fn compose(
        document: &ThemeDocument,
        boundaries: &[ThemeBoundaryPatch],
        mode: ThemeMode,
    ) -> Result<AuthorValues, ThemeError> {
        let mut values = document.base_values(mode).clone();
        let mut origins = BTreeMap::new();
        for &token in COLOR_TOKENS.iter().chain(VALUE_TOKENS) {
            values.get(token).map_err(|e| {
                ThemeError::new(format!("base.{}.{token}", mode.as_str()), e.message)
            })?;
            origins.insert(token, ThemeValueOrigin::Base);
        }
        let mut config = EffectiveThemeConfig::root(document.config);
        let mut depths = [None; 4];
        for (depth, patch) in boundaries.iter().enumerate() {
            for (slot, changed) in [
                !matches!(patch.config.accent_color, ConfigChange::Keep),
                !matches!(patch.config.gray_color, ConfigChange::Keep),
                !matches!(patch.config.panel_background, ConfigChange::Keep),
                !matches!(patch.config.radius, ConfigChange::Keep),
            ]
            .into_iter()
            .enumerate()
            {
                if changed {
                    depths[slot] = Some(depth);
                }
            }
            config = config.patched(patch.config);
        }
        let generated = super::config::generated_values(config, mode)?;
        for (i, &group) in ThemeConfigGroup::ALL.iter().enumerate() {
            for &token in group.tokens() {
                if let Some(value) = generated.0.get(token) {
                    values.0.insert(token.to_owned(), value.clone());
                    origins.insert(
                        token,
                        match depths[i] {
                            Some(depth) => ThemeValueOrigin::BoundaryConfiguration { depth, group },
                            None => ThemeValueOrigin::Configuration(group),
                        },
                    );
                }
            }
        }
        let mut apply =
            |patch: &super::ThemeValuePatch, origin: ThemeValueOrigin| -> Result<(), ThemeError> {
                values = values
                    .patched(patch)
                    .map_err(|e| ThemeError::new(origin.field(mode, &e.field), e.message))?;
                for (&token, current) in &mut origins {
                    if patch.0.contains_key(token) {
                        *current = origin;
                    }
                }
                Ok(())
            };
        apply(
            document.overrides.values(mode),
            ThemeValueOrigin::DocumentOverride,
        )?;
        for (depth, patch) in boundaries.iter().enumerate() {
            apply(
                patch.values(mode),
                ThemeValueOrigin::BoundaryOverride { depth },
            )?;
        }
        Ok(AuthorValues {
            values,
            config,
            origins,
        })
    }

    fn author_values(&self, mode: ThemeMode) -> Result<AuthorValues, ThemeError> {
        Self::compose(&self.document, &self.boundaries, mode)
    }

    pub(super) fn document_values(
        document: &ThemeDocument,
        mode: ThemeMode,
    ) -> Result<ThemeValues, ThemeError> {
        Self::compose(document, &[], mode).map(|author| author.values)
    }

    /// Explicit mode resolution, independent of document appearance (for exports/validation).
    pub fn resolve(
        &self,
        mode: ThemeMode,
        context: &ResolveContext<'_>,
    ) -> Result<ResolvedTheme, ThemeError> {
        resolve_source(Arc::new(self.clone()), mode, context)
    }
}

/// All finite component scales are derived once, in CSS px, from unscaled values.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeGeometry {
    pub spaces_px: [f64; 10],
    pub font_sizes_px: [f64; 9],
    pub line_heights_px: [f64; 9],
    pub heading_line_heights_px: [f64; 9],
    pub radii_px: [f64; 6],
    pub radius_full_px: f64,
    pub radius_thumb_px: f64,
    pub control_heights_px: [f64; 4],
}

impl ThemeGeometry {
    fn new(radius: f64, spacing: f64, font_size: f64, config: EffectiveThemeConfig) -> Self {
        let scale = config.selected.scaling.factor();
        let space = spacing / 4. * scale;
        let font = font_size / super::Theme::light().font_size * scale;
        Self {
            spaces_px: [0., 4., 8., 12., 16., 24., 32., 40., 48., 64.].map(|v| v * space),
            font_sizes_px: [12., 14., 16., 18., 20., 24., 28., 35., 60.].map(|v| v * font),
            line_heights_px: [16., 20., 24., 26., 28., 30., 36., 40., 60.].map(|v| v * font),
            heading_line_heights_px: [16., 18., 22., 24., 26., 30., 36., 40., 60.]
                .map(|v| v * font),
            radii_px: [3., 4., 6., 8., 12., 16.].map(|v| v * radius / 6. * scale),
            radius_full_px: if config.selected.radius == Some(ThemeRadius::Full) {
                9999.
            } else {
                0.
            },
            radius_thumb_px: if matches!(
                config.selected.radius,
                Some(ThemeRadius::None | ThemeRadius::Small)
            ) {
                0.5 * scale
            } else {
                9999.
            },
            control_heights_px: [24., 32., 40., 48.].map(|v| v * space),
        }
    }
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
    pub source: Arc<ThemeSource>,
    pub config: EffectiveThemeConfig,
    pub origins: BTreeMap<&'static str, ThemeValueOrigin>,
    pub brand: Arc<ResolvedPalette>,
    pub gray: Arc<ResolvedPalette>,
    pub panel: Rgba,
    pub surface: Rgba,
    pub geometry: ThemeGeometry,
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
    font_resolution: FontResolution,
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
        for (prefix, values) in [
            ("space", self.geometry.spaces_px.as_slice()),
            ("font-size", self.geometry.font_sizes_px.as_slice()),
            ("line-height", self.geometry.line_heights_px.as_slice()),
            (
                "heading-line-height",
                self.geometry.heading_line_heights_px.as_slice(),
            ),
            ("radius", self.geometry.radii_px.as_slice()),
            (
                "control-height",
                self.geometry.control_heights_px.as_slice(),
            ),
        ] {
            let start = usize::from(prefix != "space");
            for (i, value) in values.iter().enumerate() {
                props.push((
                    format!("--rustify-{prefix}-{}", i + start),
                    format!("{value}px"),
                ));
            }
        }
        for (name, value) in [
            ("full", self.geometry.radius_full_px),
            ("thumb", self.geometry.radius_thumb_px),
        ] {
            props.push((format!("--rustify-radius-{name}"), format!("{value}px")));
        }
        for (name, palette) in [("brand", &self.brand), ("gray", &self.gray)] {
            for (i, (solid, alpha)) in palette.solid.iter().zip(palette.alpha).enumerate() {
                props.push((
                    format!("--rustify-{name}-{}", i + 1),
                    super::format_color(*solid, ColorFormat::Rgb),
                ));
                props.push((
                    format!("--rustify-{name}-a{}", i + 1),
                    super::format_color(alpha, ColorFormat::Rgb),
                ));
            }
            for (role, color) in [
                ("contrast", palette.contrast),
                ("surface", palette.surface),
                ("indicator", palette.indicator),
                ("track", palette.track),
            ] {
                props.push((
                    format!("--rustify-{name}-{role}"),
                    super::format_color(color, ColorFormat::Rgb),
                ));
            }
        }
        for (role, color) in [("panel", self.panel), ("surface", self.surface)] {
            props.push((
                format!("--rustify-{role}"),
                super::format_color(color, ColorFormat::Rgb),
            ));
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
        let patch = match self.mode {
            ThemeMode::Light => ThemeBoundaryPatch {
                light: patch.clone(),
                ..Default::default()
            },
            ThemeMode::Dark => ThemeBoundaryPatch {
                dark: patch.clone(),
                ..Default::default()
            },
        };
        self.with_boundary(&patch)
    }

    pub fn with_boundary(&self, patch: &ThemeBoundaryPatch) -> Result<Self, ThemeError> {
        if patch.is_empty() {
            return Ok(self.clone());
        }
        let mut source = (*self.source).clone();
        source.boundaries.push(patch.clone());
        let mode = match patch.config.appearance {
            ConfigChange::Keep => self.mode,
            ConfigChange::Set(appearance) => appearance.mode(self.mode),
        };
        resolve_source(
            Arc::new(source),
            mode,
            &ResolveContext {
                rem_px: self.rem_px,
                reduce_motion: self.host_reduce_motion,
                fonts: &self.font_catalog,
                fallback_fonts: std::array::from_fn(|i| self.fallback_fonts[i].as_str()),
                font_resolution: self.font_resolution,
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
    resolve_source(
        Arc::new(ThemeSource::new(Arc::new(document.clone()))),
        mode,
        context,
    )
}

fn validate_masked_values(
    source: &ThemeSource,
    mode: ThemeMode,
    effective: &ThemeValues,
    context: &ResolveContext<'_>,
) -> Result<(), ThemeError> {
    // The ordinary resolver validates effective values. Validate overwritten
    // inputs too, so configuration/boundaries cannot hide an invalid author base.
    let base = source.document().base_values(mode);
    let parse_font_size = |value: &str, origin: ThemeValueOrigin| {
        parse_length(value, context.rem_px, LengthRule::Positive)
            .map_err(|e| ThemeError::new(origin.field(mode, "font-size"), e.message))
    };
    let mut font = parse_font_size(base.get("font-size")?, ThemeValueOrigin::Base)?;
    let validate =
        |token: &str, value: &str, origin: ThemeValueOrigin, font: f64| -> Result<(), ThemeError> {
            if effective
                .0
                .get(token)
                .is_some_and(|final_value| final_value == value)
                && token != "letter-spacing"
            {
                return Ok(());
            }
            let field = origin.field(mode, token);
            let error = |message: String| ThemeError::new(&field, message);
            if COLOR_TOKENS.contains(&token) {
                return parse_color(value).map(|_| ()).map_err(error);
            }
            let length = match token {
                "spacing" | "font-size" => Some(LengthRule::Positive),
                "radius" | "layout-gap" | "shadow-blur" => Some(LengthRule::NonNegative),
                "shadow-spread" | "shadow-offset-x" | "shadow-offset-y" => Some(LengthRule::Signed),
                _ => None,
            };
            if let Some(rule) = length {
                return parse_length(value, context.rem_px, rule)
                    .map(|_| ())
                    .map_err(|e| error(e.message));
            }
            match token {
                "font-sans" | "font-serif" | "font-mono" => {
                    if context.font_resolution == FontResolution::Css {
                        parse_css_font_stack(value)
                            .map(|_| ())
                            .map_err(|e| error(e.message))
                    } else {
                        parse_font_stack(value)
                            .map(|_| ())
                            .map_err(|e| error(e.message))
                    }
                }
                "letter-spacing" => letter_spacing(value, font, context.rem_px)
                    .map(|_| ())
                    .map_err(|e| error(e.message)),
                "shadow-opacity" => number(value, 0., 1.)
                    .map(|_| ())
                    .map_err(|e| error(e.message)),
                "reduce-motion" if matches!(value, "true" | "false") => Ok(()),
                "reduce-motion" => Err(error("use true or false".into())),
                _ => Ok(()),
            }
        };
    for &token in COLOR_TOKENS.iter().chain(VALUE_TOKENS) {
        validate(token, base.get(token)?, ThemeValueOrigin::Base, font)?;
    }
    let overrides = source.document().overrides.values(mode);
    if let Some(value) = overrides.0.get("font-size") {
        font = parse_font_size(value, ThemeValueOrigin::DocumentOverride)?;
    }
    for (token, value) in &overrides.0 {
        validate(token, value, ThemeValueOrigin::DocumentOverride, font)?;
    }
    for (depth, patch) in source.boundaries.iter().enumerate() {
        let patch = patch.values(mode);
        if let Some(value) = patch.0.get("font-size") {
            font = parse_font_size(value, ThemeValueOrigin::BoundaryOverride { depth })?;
        }
        for (token, value) in &patch.0 {
            validate(
                token,
                value,
                ThemeValueOrigin::BoundaryOverride { depth },
                font,
            )?;
        }
    }
    Ok(())
}

fn resolve_source(
    source: Arc<ThemeSource>,
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<ResolvedTheme, ThemeError> {
    RESOLUTION_COUNT.with(|count| count.set(count.get() + 1));
    let document = source.document();
    document.validate_identity()?;
    let AuthorValues {
        values,
        config,
        origins,
    } = source.author_values(mode)?;
    validate_masked_values(&source, mode, &values, context)?;
    let field_error = |token: &str, error: ThemeError| {
        ThemeError::new(
            origins
                .get(token)
                .copied()
                .unwrap_or(ThemeValueOrigin::Base)
                .field(mode, token),
            error.message,
        )
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
                field: origins[token].field(mode, token),
                message: "Preview clipped this color to sRGB; the author value is preserved".into(),
            });
        }
    }
    for token in values.0.keys() {
        if !COLOR_TOKENS.contains(&token.as_str()) && !VALUE_TOKENS.contains(&token.as_str()) {
            diagnostics.push(ThemeDiagnostic {
                field: format!("base.{}.{token}", mode.as_str()),
                message: "Unknown token is preserved but not used by the preview".into(),
            });
        }
    }
    for &token in COMMON_TOKENS {
        if document.base.light.get(token)? != document.base.dark.get(token)?
            || document.overrides.light.0.get(token) != document.overrides.dark.0.get(token)
        {
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
        let scale = config.selected.scaling.factor();
        for layer in &mut layers {
            layer.offset_x *= scale;
            layer.offset_y *= scale;
            layer.blur *= scale;
            layer.spread *= scale;
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
        if context.font_resolution == FontResolution::Css {
            let families =
                parse_css_font_stack(value(token)?).map_err(|error| field_error(token, error))?;
            let requested: Vec<_> = families.iter().map(|(name, _)| name.clone()).collect();
            let mut stack: Vec<_> = families
                .iter()
                .map(|(name, generic)| {
                    if *generic {
                        name.clone()
                    } else {
                        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
                    }
                })
                .collect();
            if !families.last().is_some_and(|(_, generic)| *generic) {
                stack.push(["sans-serif", "serif", "monospace"][index].into());
            }
            return Ok(ResolvedFont {
                family: requested[0].clone(),
                requested,
                resource_path: None,
                fallback: false,
                font_resolution: FontResolution::Css,
                css_stack: Some(stack.join(", ")),
            });
        }
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
                field: origins[token].field(mode, token),
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
            font_resolution: FontResolution::Packaged,
            css_stack: None,
        })
    };
    let fonts = [
        font("font-sans", 0)?,
        font("font-serif", 1)?,
        font("font-mono", 2)?,
    ];
    let geometry = ThemeGeometry::new(radius_px, spacing_px, font_size_px, config);
    let radii_px = if config.selected.radius.is_some() {
        std::array::from_fn(|i| geometry.radii_px[i])
    } else {
        [
            (radius_px - 4.).max(0.),
            (radius_px - 2.).max(0.),
            radius_px,
            radius_px + 4.,
        ]
        .map(|v| v * config.selected.scaling.factor())
    };
    let scale = config.selected.scaling.factor();
    let brand = super::config::palette(config.accent.family(), mode)?;
    let gray = super::config::palette(config.gray, mode)?;
    let panel = match (mode, config.selected.panel_background) {
        (_, None) => colors["card"],
        (ThemeMode::Light, Some(super::PanelBackground::Solid)) => Rgba {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        },
        (ThemeMode::Light, Some(super::PanelBackground::Translucent)) => {
            super::palette_roles::LIGHT_PANEL
        }
        (ThemeMode::Dark, Some(super::PanelBackground::Solid)) => gray.solid[1],
        (ThemeMode::Dark, Some(super::PanelBackground::Translucent)) => gray.alpha[1],
    };
    let surface = super::palette_roles::SURFACES[usize::from(mode == ThemeMode::Dark)];
    Ok(ResolvedTheme {
        mode,
        document_id: document.id.clone(),
        name: document.name.clone(),
        values,
        source: source.clone(),
        config,
        origins,
        brand,
        gray,
        panel,
        surface,
        geometry,
        colors,
        radius_px: radius_px * scale,
        radii_px,
        spacing_px: spacing_px * scale,
        layout_gap_px: layout_gap_px * scale,
        font_size_px: font_size_px * scale,
        letter_spacing_em,
        fonts,
        shadows,
        reduce_motion: context.reduce_motion || requested_motion,
        diagnostics,
        rem_px: context.rem_px,
        font_catalog: context.fonts.to_vec(),
        fallback_fonts: context.fallback_fonts.map(str::to_owned),
        host_reduce_motion: context.reduce_motion,
        font_resolution: context.font_resolution,
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
            .overrides
            .light
            .0
            .insert("primary".into(), "#12345680".into());
        document
            .overrides
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
        document
            .overrides
            .light
            .0
            .insert("radius".into(), "0".into());
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
            document.base.dark.0.insert(token.into(), invalid.into());
            let before = document.clone();
            assert_eq!(
                resolve(&document, ThemeMode::Dark, &ResolveContext::default())
                    .unwrap_err()
                    .field,
                format!("base.dark.{token}")
            );
            assert_eq!(document, before);
        }
    }

    #[test]
    fn missing_tokens_fail_and_unknown_tokens_generate_diagnostics() {
        let mut document = document();
        document.base.light.0.remove("chart-5");
        assert_eq!(
            resolve(&document, ThemeMode::Light, &ResolveContext::default())
                .unwrap_err()
                .field,
            "base.light.chart-5"
        );
        document
            .base
            .light
            .0
            .insert("chart-5".into(), "#123456".into());
        document
            .base
            .light
            .0
            .insert("custom".into(), "ignored".into());
        assert!(
            resolve(&document, ThemeMode::Light, &ResolveContext::default())
                .unwrap()
                .diagnostics
                .iter()
                .any(|d| d.field == "base.light.custom")
        );
    }

    #[test]
    fn gamut_and_common_field_differences_are_visible_and_author_values_survive() {
        let mut document = document();
        document
            .overrides
            .light
            .0
            .insert("primary".into(), "oklch(0.7 0.4 40)".into());
        document
            .overrides
            .dark
            .0
            .insert("radius".into(), "2px".into());
        let resolved = resolve(&document, ThemeMode::Light, &ResolveContext::default()).unwrap();
        assert!(resolved
            .diagnostics
            .iter()
            .any(|d| d.field == "overrides.light.primary"));
        assert!(resolved.diagnostics.iter().any(|d| d.field == "radius"));
        assert_eq!(
            document.overrides.light.0.get("primary").unwrap(),
            "oklch(0.7 0.4 40)"
        );
    }

    #[test]
    fn fonts_pick_packaged_families_and_report_the_requested_fallback() {
        let mut document = document();
        document
            .base
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
            document.base.light.0.insert(token.into(), value.into());
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
    #[cfg(rustify_repo_tests)]
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
