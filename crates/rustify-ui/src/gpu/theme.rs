//! Numerical theme projection shared by GPU widgets and application drawings.

use super::DrawRustifyShadow;
use crate::{
    makepad_widgets::*,
    theme::{ResolvedTheme, Rgba, ShadowLayer},
};

pub use crate::theme::FontSlot;

/// Keeps alpha separate until the shader emits its premultiplied fragment.
pub fn rgba_color(color: Rgba) -> Vec4f {
    vec4(
        color.r as f32,
        color.g as f32,
        color.b as f32,
        color.a as f32,
    )
}

/// Makepad layout uses 72 points per inch and CSS uses 96 pixels per inch.
pub fn css_px_to_points(px: f64) -> f32 {
    (px * 72.0 / 96.0) as f32
}

/// Applies the packaged font, point size and shaping letter spacing.
/// Returns whether the drawing changed, so callers can avoid idle redraws.
pub fn apply_text_theme(
    draw: &mut DrawText,
    cx: &mut Cx,
    theme: &ResolvedTheme,
    slot: FontSlot,
) -> bool {
    let size = css_px_to_points(theme.font_size_px);
    let mut changed = draw.text_style.font_size != size
        || draw.text_style.letter_spacing != theme.letter_spacing_em;
    draw.text_style.font_size = size;
    draw.text_style.letter_spacing = theme.letter_spacing_em;
    let index = match slot {
        FontSlot::Sans => 0,
        FontSlot::Serif => 1,
        FontSlot::Mono => 2,
    };
    if let Some(family) = crate::theme::gpu_font_family(cx, &theme.fonts[index]) {
        if draw.text_style.font_family != family {
            draw.text_style.font_family = family;
            changed = true;
        }
    }
    changed
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ThemeMetrics {
    pub spacing: f64,
    pub radii: [f64; 4],
    pub font_size: f64,
    pub ring: Rgba,
    pub reduce_motion: bool,
}

impl From<&ResolvedTheme> for ThemeMetrics {
    fn from(theme: &ResolvedTheme) -> Self {
        Self {
            spacing: theme.spacing_px,
            radii: theme.radii_px,
            font_size: theme.font_size_px,
            ring: theme.colors["ring"],
            reduce_motion: theme.reduce_motion,
        }
    }
}

pub(crate) fn token(theme: &ResolvedTheme, name: &str) -> Vec4f {
    rgba_color(theme.colors[name])
}

pub(crate) fn set_box_radius(draw: &mut DrawColor, cx: &mut Cx2d, radius: f64, stroke: f32) {
    // Sdf2d.box's historical corner uniform is half the actual visual radius.
    draw.set_uniform(cx, id!(corner), &[(radius * 0.5) as f32]);
    draw.set_uniform(cx, id!(stroke_width), &[stroke]);
    draw.set_uniform(cx, id!(flat), &[0.0]);
}

pub(crate) fn focus_hit(hit: &Hit, cx: &mut Cx, area: Area, disabled: bool) -> bool {
    match hit {
        Hit::FingerDown(fe) if !disabled && fe.is_primary_hit() => {
            cx.set_key_focus(area);
            true
        }
        Hit::KeyFocus(_) | Hit::KeyFocusLost(_) => true,
        _ => false,
    }
}

pub(crate) fn draw_focus(
    draw: &mut DrawRustifyShadow,
    cx: &mut Cx2d,
    body: Rect,
    radius: f64,
    metrics: ThemeMetrics,
    focused: bool,
) {
    if focused {
        draw.draw_layers(
            cx,
            body,
            radius,
            &[ShadowLayer {
                color: Rgba {
                    a: metrics.ring.a * 0.5,
                    ..metrics.ring
                },
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: 3.0,
            }],
        );
    }
}

pub(crate) fn disabled_color(mut color: Vec4f, disabled: bool) -> Vec4f {
    if disabled {
        color.w *= 0.5;
    }
    color
}
