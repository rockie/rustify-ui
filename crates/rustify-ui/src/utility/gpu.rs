//! Finite renderer projection of compiler declarations, independent of drawing.

use crate::{
    i18n::Direction,
    theme::{css_px_to_points, ResolvedFont, ResolvedTheme, Rgba, ShadowLayer, FONT_FACES},
};
use rustify_style::{program::GpuProgram, text::Text};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GpuApplyPolicy {
    #[default]
    Strict,
    Partial,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuResolutionStatus {
    Applied,
    Partial,
    Rejected,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuDiagnosticKind {
    UnsupportedUtility,
    UnsupportedVariant,
    UnsupportedValue,
    InvalidContext,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuDiagnostic {
    pub candidate: Text,
    pub property: Option<String>,
    pub kind: GpuDiagnosticKind,
    pub reason: String,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputModality {
    Keyboard,
    Pointer,
    #[default]
    Programmatic,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpuControlState {
    pub hover: bool,
    pub active: bool,
    pub focused: bool,
    pub focus_visible: bool,
    pub disabled: bool,
    pub modality: InputModality,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GpuLength {
    Pixels(f64),
    Auto,
    Unbounded,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GpuLineHeight {
    Normal,
    Pixels(f64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuFlow {
    Row,
    Column,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuAlignment {
    Start,
    Center,
    End,
    Stretch,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuJustification {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
/// A weight that the host can actually render from a packaged face.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuFontFace {
    pub family: String,
    pub weight: u16,
    pub resource_path: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuCapabilities {
    pub flex_layout: bool,
    pub text: bool,
    pub outer_shadows: bool,
    pub nonuniform_corners: bool,
    pub nonuniform_borders: bool,
    /// True only when opacity needs no offscreen/group composition.
    pub independent_leaf_opacity: bool,
}
impl Default for GpuCapabilities {
    fn default() -> Self {
        Self {
            flex_layout: true,
            text: true,
            outer_shadows: true,
            nonuniform_corners: false,
            nonuniform_borders: false,
            independent_leaf_opacity: true,
        }
    }
}
/// A complete snapshot. Edge arrays are top/right/bottom/left; corner arrays
/// are top-left/top-right/bottom-right/bottom-left. Shadow order is CSS order.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuComputedStyle {
    pub background: Rgba,
    pub color: Rgba,
    pub border_colors: [Rgba; 4],
    pub padding: [f64; 4],
    pub gap: [f64; 2],
    pub width: GpuLength,
    pub height: GpuLength,
    pub min_width: GpuLength,
    pub min_height: GpuLength,
    pub max_width: GpuLength,
    pub max_height: GpuLength,
    pub flow: GpuFlow,
    pub items: GpuAlignment,
    pub justify: GpuJustification,
    pub radii: [f64; 4],
    pub border_widths: [f64; 4],
    pub opacity: f64,
    pub font: ResolvedFont,
    pub font_size_px: f64,
    pub font_size_points: f32,
    pub font_weight: u16,
    pub line_height: GpuLineHeight,
    pub letter_spacing_em: f64,
    pub shadows: Vec<ShadowLayer>,
}
impl Default for GpuComputedStyle {
    fn default() -> Self {
        let transparent = Rgba {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
        };
        Self {
            background: transparent,
            color: Rgba {
                a: 1.,
                ..transparent
            },
            border_colors: [transparent; 4],
            padding: [0.; 4],
            gap: [0.; 2],
            width: GpuLength::Auto,
            height: GpuLength::Auto,
            min_width: GpuLength::Pixels(0.),
            min_height: GpuLength::Pixels(0.),
            max_width: GpuLength::Unbounded,
            max_height: GpuLength::Unbounded,
            flow: GpuFlow::Column,
            items: GpuAlignment::Start,
            justify: GpuJustification::Start,
            radii: [0.; 4],
            border_widths: [0.; 4],
            opacity: 1.,
            font: ResolvedFont {
                requested: vec!["sans-serif".into()],
                family: "sans-serif".into(),
                resource_path: None,
                fallback: false,
            },
            font_size_px: 16.,
            font_size_points: css_px_to_points(16.),
            font_weight: 400,
            line_height: GpuLineHeight::Normal,
            letter_spacing_em: 0.,
            shadows: Vec::new(),
        }
    }
}
impl GpuComputedStyle {
    /// Compare values used by rendering, excluding font request diagnostics.
    pub fn rendered_eq(&self, other: &Self) -> bool {
        let mut style = self.clone();
        style.font.requested.clone_from(&other.font.requested);
        style.font.fallback = other.font.fallback;
        style == *other
    }
    /// Neutral sample defaults; widgets pass their own apply_theme baseline.
    pub fn from_theme(theme: &ResolvedTheme) -> Self {
        let mut style = Self {
            radii: [theme.radii_px[1]; 4],
            font: theme.fonts[0].clone(),
            font_size_px: theme.font_size_px,
            font_size_points: css_px_to_points(theme.font_size_px),
            letter_spacing_em: theme.letter_spacing_em,
            gap: [theme.layout_gap_px; 2],
            ..Self::default()
        };
        if let Some(color) = theme.colors.get("background") {
            style.background = *color;
        }
        if let Some(color) = theme.colors.get("foreground") {
            style.color = *color;
        }
        if let Some(color) = theme.colors.get("border") {
            style.border_colors = [*color; 4];
        }
        style
    }
}
/// All measurements use CSS pixels. Media rem basis is explicitly separate
/// from the scope's layout rem. Variables describe compiler/host bindings and
/// override SDK token values; Tailwind --spacing is never guessed from SDK gap.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuResolveContext {
    pub viewport: [f64; 2],
    pub parent_size: [Option<f64>; 2],
    pub control_size: [Option<f64>; 2],
    pub rem_px: f64,
    pub media_rem_px: f64,
    pub parent_font_size_px: f64,
    pub direction: Direction,
    pub state: GpuControlState,
    pub hover_capable: bool,
    pub variables: BTreeMap<rustify_style::text::Text, rustify_style::text::Text>,
    pub fonts: Vec<GpuFontFace>,
    pub capabilities: GpuCapabilities,
    pub base_style: GpuComputedStyle,
    pub previous_style: Option<GpuComputedStyle>,
    pub policy: GpuApplyPolicy,
}
impl GpuResolveContext {
    pub fn new(base_style: GpuComputedStyle) -> Self {
        Self {
            viewport: [0.; 2],
            parent_size: [None; 2],
            control_size: [None; 2],
            rem_px: 16.,
            media_rem_px: 16.,
            parent_font_size_px: base_style.font_size_px,
            direction: Direction::LeftToRight,
            state: GpuControlState::default(),
            hover_capable: true,
            variables: BTreeMap::new(),
            fonts: FONT_FACES
                .iter()
                .map(|face| GpuFontFace {
                    family: face.family.into(),
                    weight: 400,
                    resource_path: face.resource_path.into(),
                })
                .collect(),
            capabilities: GpuCapabilities::default(),
            base_style,
            previous_style: None,
            policy: GpuApplyPolicy::Strict,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GpuResolution {
    pub status: GpuResolutionStatus,
    pub style: GpuComputedStyle,
    pub diagnostics: Vec<GpuDiagnostic>,
    pub changed: bool,
}
/// Resolve a complete program set against current theme/state/viewport values.
pub fn resolve_gpu(
    programs: &[GpuProgram],
    theme: &ResolvedTheme,
    context: &GpuResolveContext,
) -> GpuResolution {
    cascade::resolve(programs, theme, context)
}
#[cfg(test)]
mod tests;
mod values;

mod cascade;
mod conditions;
