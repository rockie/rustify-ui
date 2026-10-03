//! Rounded outer shadows with visual bounds independent from hit geometry.

/// A shadow mask and its expanded visual rectangle, in CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowGeometry {
    /// Expanded draw rectangle `[x, y, width, height]`.
    pub visual: [f64; 4],
    /// Spread/offset mask rectangle, relative to the visual origin.
    pub source: [f64; 4],
    /// The original border box, relative to the visual origin, cut out of the shadow.
    pub cutout: [f64; 4],
    pub source_radius: f64,
    pub cutout_radius: f64,
    pub sigma: f64,
}

impl ShadowGeometry {
    /// Resolves one outer shadow. Transparent and fully contracted masks are absent.
    pub fn new(
        body: [f64; 4],
        radius: f64,
        offset: [f64; 2],
        blur: f64,
        spread: f64,
        alpha: f64,
    ) -> Option<Self> {
        let [x, y, width, height] = body;
        let source_width = width + spread * 2.0;
        let source_height = height + spread * 2.0;
        if alpha <= 0.0
            || width <= 0.0
            || height <= 0.0
            || source_width <= 0.0
            || source_height <= 0.0
        {
            return None;
        }
        let sigma = blur * 0.5;
        let padding = (sigma * 3.0).ceil() + 1.0;
        let left = x + offset[0] - spread - padding;
        let top = y + offset[1] - spread - padding;
        Some(Self {
            visual: [
                left,
                top,
                source_width + padding * 2.0,
                source_height + padding * 2.0,
            ],
            source: [padding, padding, source_width, source_height],
            cutout: [x - left, y - top, width, height],
            source_radius: (radius + spread)
                .max(0.0)
                .min(source_width.min(source_height) * 0.5),
            cutout_radius: radius.max(0.0).min(width.min(height) * 0.5),
            sigma,
        })
    }

    /// Visible bounds after intersection with a clip rectangle.
    pub fn clipped_bounds(self, clip: [f64; 4]) -> Option<[f64; 4]> {
        let left = self.visual[0].max(clip[0]);
        let top = self.visual[1].max(clip[1]);
        let right = (self.visual[0] + self.visual[2]).min(clip[0] + clip[2]);
        let bottom = (self.visual[1] + self.visual[3]).min(clip[1] + clip[3]);
        (right > left && bottom > top).then_some([left, top, right - left, bottom - top])
    }
}

#[cfg(target_arch = "wasm32")]
use crate::{makepad_widgets::*, theme::ShadowLayer};

/// A reusable rounded outer-shadow shader; its area is never a control's hit area.
#[cfg(target_arch = "wasm32")]
#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawRustifyShadow {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4f,
    #[live]
    pub source_pos: Vec2f,
    #[live]
    pub source_size: Vec2f,
    #[live]
    pub cutout_pos: Vec2f,
    #[live]
    pub cutout_size: Vec2f,
    #[live]
    pub source_radius: f32,
    #[live]
    pub cutout_radius: f32,
    #[live]
    pub sigma: f32,
    #[live]
    pub pad: f32,
}

#[cfg(target_arch = "wasm32")]
impl DrawRustifyShadow {
    /// Draws all CSS-ordered outer layers, back to front, under the original body.
    pub fn draw_layers(
        &mut self,
        cx: &mut Cx2d,
        body: Rect,
        radius_px: f64,
        layers: &[ShadowLayer],
    ) {
        for layer in layers.iter().rev() {
            let Some(geometry) = ShadowGeometry::new(
                [body.pos.x, body.pos.y, body.size.x, body.size.y],
                radius_px,
                [layer.offset_x, layer.offset_y],
                layer.blur,
                layer.spread,
                layer.color.a,
            ) else {
                continue;
            };
            self.color = super::rgba_color(layer.color);
            self.source_pos = vec2(geometry.source[0] as f32, geometry.source[1] as f32);
            self.source_size = vec2(geometry.source[2] as f32, geometry.source[3] as f32);
            self.cutout_pos = vec2(geometry.cutout[0] as f32, geometry.cutout[1] as f32);
            self.cutout_size = vec2(geometry.cutout[2] as f32, geometry.cutout[3] as f32);
            self.source_radius = geometry.source_radius as f32;
            self.cutout_radius = geometry.cutout_radius as f32;
            self.sigma = geometry.sigma as f32;
            self.draw_abs(
                cx,
                Rect {
                    pos: dvec2(geometry.visual[0], geometry.visual[1]),
                    size: dvec2(geometry.visual[2], geometry.visual[3]),
                },
            );
        }
    }

    /// Applies an explicit clip in addition to the current Makepad view/turtle clip.
    pub fn draw_layers_clipped(
        &mut self,
        cx: &mut Cx2d,
        body: Rect,
        radius_px: f64,
        layers: &[ShadowLayer],
        clip: Rect,
    ) {
        cx.push_clip_rect(clip);
        self.draw_layers(cx, body, radius_px, layers);
        cx.pop_clip_rect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_bounds_expand_without_changing_the_original_body() {
        let geometry =
            ShadowGeometry::new([10.0, 20.0, 100.0, 40.0], 8.0, [3.0, -2.0], 10.0, 4.0, 0.5)
                .unwrap();
        assert_eq!(geometry.visual, [-7.0, -2.0, 140.0, 80.0]);
        assert_eq!(geometry.cutout, [17.0, 22.0, 100.0, 40.0]);
        assert_eq!(geometry.source, [16.0, 16.0, 108.0, 48.0]);
        assert_eq!(
            (
                geometry.source_radius,
                geometry.cutout_radius,
                geometry.sigma
            ),
            (12.0, 8.0, 5.0)
        );
    }

    #[test]
    fn zero_blur_keeps_one_pixel_antialias_margin() {
        let geometry =
            ShadowGeometry::new([0.0, 0.0, 20.0, 20.0], 0.0, [0.0, 0.0], 0.0, 3.0, 1.0).unwrap();
        assert_eq!(geometry.visual, [-4.0, -4.0, 28.0, 28.0]);
        assert_eq!(geometry.sigma, 0.0);
    }

    #[test]
    fn negative_spread_contracts_the_mask_and_radius() {
        let geometry =
            ShadowGeometry::new([0.0, 0.0, 20.0, 10.0], 4.0, [0.0, 0.0], 2.0, -3.0, 1.0).unwrap();
        assert_eq!(geometry.source, [4.0, 4.0, 14.0, 4.0]);
        assert_eq!(geometry.source_radius, 1.0);
    }

    #[test]
    fn contracted_or_transparent_shadow_is_absent() {
        assert!(
            ShadowGeometry::new([0.0, 0.0, 20.0, 10.0], 4.0, [0.0, 0.0], 10.0, -5.0, 1.0).is_none()
        );
        assert!(
            ShadowGeometry::new([0.0, 0.0, 20.0, 10.0], 4.0, [0.0, 0.0], 10.0, 0.0, 0.0).is_none()
        );
    }

    #[test]
    fn oversized_radius_clamps_to_the_mask_half_size() {
        let geometry =
            ShadowGeometry::new([0.0, 0.0, 20.0, 10.0], 50.0, [0.0, 0.0], 0.0, 2.0, 1.0).unwrap();
        assert_eq!((geometry.source_radius, geometry.cutout_radius), (7.0, 5.0));
    }

    #[test]
    fn clip_intersects_expanded_visual_bounds() {
        let geometry =
            ShadowGeometry::new([0.0, 0.0, 20.0, 10.0], 4.0, [0.0, 0.0], 10.0, 0.0, 1.0).unwrap();
        assert_eq!(
            geometry.clipped_bounds([0.0, 0.0, 10.0, 10.0]),
            Some([0.0, 0.0, 10.0, 10.0])
        );
        assert_eq!(geometry.clipped_bounds([100.0, 100.0, 10.0, 10.0]), None);
    }
}
