//! Anchored geometry, independent of DOM ownership and layer registration.
use super::{Align, LayerPlacement, LocalRect, Side};
use crate::Direction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewportAlign {
    Start,
    #[default]
    Center,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CollisionPadding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Sticky {
    #[default]
    Partial,
    Always,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PositionUpdate {
    #[default]
    Optimized,
    Always,
}
/// Positioning preferences; physical side and logical alignment are independent.
#[derive(Clone, Debug, PartialEq)]
pub struct FloatingPosition {
    pub side: Side,
    pub align: Align,
    pub side_offset: f64,
    pub align_offset: f64,
    pub arrow_padding: f64,
    pub avoid_collisions: bool,
    pub collision_padding: CollisionPadding,
    pub collision_boundary_ids: Vec<String>,
    pub sticky: Sticky,
    pub hide_when_detached: bool,
    pub update: PositionUpdate,
}
impl Default for FloatingPosition {
    fn default() -> Self {
        Self {
            side: Side::Bottom,
            align: Align::Center,
            side_offset: 0.,
            align_offset: 0.,
            arrow_padding: 0.,
            avoid_collisions: true,
            collision_padding: Default::default(),
            collision_boundary_ids: Vec::new(),
            sticky: Sticky::Partial,
            hide_when_detached: false,
            update: PositionUpdate::Optimized,
        }
    }
}
/// Identifies the public numeric field that cannot be used for positioning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatingPositionError {
    pub field: &'static str,
}
impl std::fmt::Display for FloatingPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid floating position field `{}`", self.field)
    }
}
impl std::error::Error for FloatingPositionError {}
impl FloatingPosition {
    /// Rejects non-finite offsets and negative padding before writing geometry.
    pub fn validate(&self) -> Result<(), FloatingPositionError> {
        for (field, value, padding) in [
            ("side_offset", self.side_offset, false),
            ("align_offset", self.align_offset, false),
            ("arrow_padding", self.arrow_padding, true),
            ("collision_padding.top", self.collision_padding.top, true),
            (
                "collision_padding.right",
                self.collision_padding.right,
                true,
            ),
            (
                "collision_padding.bottom",
                self.collision_padding.bottom,
                true,
            ),
            ("collision_padding.left", self.collision_padding.left, true),
        ] {
            if !value.is_finite() || (padding && value < 0.) {
                return Err(FloatingPositionError { field });
            }
        }
        Ok(())
    }
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct FloatingPlacement {
    pub x: f64,
    pub y: f64,
    pub placement: LayerPlacement,
    pub available_width: f64,
    pub available_height: f64,
    pub arrow: Option<(f64, f64)>,
    pub arrow_hidden: bool,
    pub detached: bool,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) fn visible_scope(scope: LocalRect, viewport: (f64, f64)) -> Option<LocalRect> {
    let x = scope.x.max(0.);
    let y = scope.y.max(0.);
    let width = (scope.x + scope.width).min(viewport.0) - x;
    let height = (scope.y + scope.height).min(viewport.1) - y;
    (width > 0. && height > 0.).then_some(LocalRect::new(x, y, width, height))
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) struct ViewportPlacement {
    pub x: f64,
    pub y: f64,
    pub available_width: f64,
    pub available_height: f64,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) fn place_viewport(
    bounds: LocalRect,
    size: (f64, f64),
    align: ViewportAlign,
    inset: f64,
) -> ViewportPlacement {
    let ix = inset.min(bounds.width / 2.);
    let iy = inset.min(bounds.height / 2.);
    let width = (bounds.width - 2. * ix).max(0.);
    let height = (bounds.height - 2. * iy).max(0.);
    ViewportPlacement {
        x: bounds.x + ix + (width - size.0).max(0.) / 2.,
        y: bounds.y
            + iy
            + if align == ViewportAlign::Center {
                (height - size.1).max(0.) / 2.
            } else {
                0.
            },
        available_width: width,
        available_height: height,
    }
}
fn cross(side: Side) -> bool {
    matches!(side, Side::Top | Side::Bottom)
}
fn opposite(side: Side) -> Side {
    match side {
        Side::Top => Side::Bottom,
        Side::Bottom => Side::Top,
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}
fn aligned(start: f64, length: f64, panel: f64, align: Align, reverse: bool) -> f64 {
    match align {
        Align::Center => start + (length - panel) / 2.,
        Align::Start if !reverse => start,
        Align::End if reverse => start,
        _ => start + length - panel,
    }
}
fn coordinates(
    anchor: LocalRect,
    size: (f64, f64),
    side: Side,
    gap: f64,
    config: &FloatingPosition,
    direction: Direction,
) -> (f64, f64) {
    let x = aligned(
        anchor.x,
        anchor.width,
        size.0,
        config.align,
        direction == Direction::RightToLeft,
    );
    let y = aligned(anchor.y, anchor.height, size.1, config.align, false);
    let offset = if cross(side) && direction == Direction::RightToLeft {
        -config.align_offset
    } else {
        config.align_offset
    };
    match side {
        Side::Top => (x + offset, anchor.y - size.1 - gap),
        Side::Bottom => (x + offset, anchor.y + anchor.height + gap),
        Side::Left => (anchor.x - size.0 - gap, y + offset),
        Side::Right => (anchor.x + anchor.width + gap, y + offset),
    }
}
fn overflow(at: (f64, f64), size: (f64, f64), bounds: LocalRect) -> f64 {
    (bounds.x - at.0).max(0.)
        + (at.0 + size.0 - bounds.x - bounds.width).max(0.)
        + (bounds.y - at.1).max(0.)
        + (at.1 + size.1 - bounds.y - bounds.height).max(0.)
}
/// `arrow` is its cross-axis base and protruding height. Inputs are measured DOM rectangles;
/// callers validate the public configuration and intersect scope-local collision boundaries.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) fn place_floating(
    anchor: LocalRect,
    size: (f64, f64),
    arrow: Option<(f64, f64)>,
    bounds: LocalRect,
    direction: Direction,
    config: &FloatingPosition,
) -> FloatingPlacement {
    let p = config.collision_padding;
    let bounds = LocalRect::new(
        bounds.x + p.left,
        bounds.y + p.top,
        (bounds.width - p.left - p.right).max(0.),
        (bounds.height - p.top - p.bottom).max(0.),
    );
    let gap = config.side_offset + arrow.map_or(0., |(_, height)| height);
    let mut side = config.side;
    let mut at = coordinates(anchor, size, side, gap, config, direction);
    if config.avoid_collisions {
        let other = opposite(side);
        let candidate = coordinates(anchor, size, other, gap, config, direction);
        if overflow(candidate, size, bounds) < overflow(at, size, bounds) {
            side = other;
            at = candidate;
        }
        let (start, length, panel, anchor_start, anchor_length) = if cross(side) {
            (bounds.x, bounds.width, size.0, anchor.x, anchor.width)
        } else {
            (bounds.y, bounds.height, size.1, anchor.y, anchor.height)
        };
        let value = if cross(side) { &mut at.0 } else { &mut at.1 };
        *value = value.clamp(start, (start + length - panel).max(start));
        if config.sticky == Sticky::Partial {
            *value = value.clamp(anchor_start - panel, anchor_start + anchor_length);
        }
    }
    let detached = anchor.x + anchor.width < bounds.x
        || anchor.x > bounds.x + bounds.width
        || anchor.y + anchor.height < bounds.y
        || anchor.y > bounds.y + bounds.height;
    let mut arrow_hidden = detached;
    let arrow = arrow.map(|(base, height)| {
        let (a, length, offset, panel) = if cross(side) {
            (anchor.x, anchor.width, at.0, size.0)
        } else {
            (anchor.y, anchor.height, at.1, size.1)
        };
        let min = config.arrow_padding + base / 2.;
        let max = panel - config.arrow_padding - base / 2.;
        let center = if min <= max {
            (a + length / 2. - offset).clamp(min, max)
        } else {
            arrow_hidden = true;
            panel / 2.
        };
        arrow_hidden |= offset + center < a || offset + center > a + length;
        match side {
            Side::Top => (center - base / 2., size.1),
            Side::Bottom => (center - base / 2., -height),
            Side::Left => (size.0, center - base / 2.),
            Side::Right => (-height, center - base / 2.),
        }
    });
    let mut available_width = bounds.width;
    let mut available_height = bounds.height;
    match side {
        Side::Top => available_height = (anchor.y - bounds.y - gap).max(0.),
        Side::Bottom => {
            available_height = (bounds.y + bounds.height - anchor.y - anchor.height - gap).max(0.)
        }
        Side::Left => available_width = (anchor.x - bounds.x - gap).max(0.),
        Side::Right => {
            available_width = (bounds.x + bounds.width - anchor.x - anchor.width - gap).max(0.)
        }
    }
    available_width = available_width.min(bounds.width);
    available_height = available_height.min(bounds.height);
    FloatingPlacement {
        x: at.0,
        y: at.1,
        placement: LayerPlacement {
            side,
            align: config.align,
        },
        available_width,
        available_height,
        arrow,
        arrow_hidden,
        detached,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Align, Direction, LocalRect, Side};
    fn resolve(
        config: FloatingPosition,
        anchor: LocalRect,
        bounds: LocalRect,
    ) -> FloatingPlacement {
        place_floating(
            anchor,
            (60., 40.),
            None,
            bounds,
            Direction::LeftToRight,
            &config,
        )
    }
    #[test]
    fn scope_viewport_uses_only_the_visible_intersection() {
        assert_eq!(
            visible_scope(LocalRect::new(-20., 100., 400., 600.), (800., 500.)),
            Some(LocalRect::new(0., 100., 380., 400.))
        );
        assert!(visible_scope(LocalRect::new(900., 100., 200., 200.), (800., 500.)).is_none());
        assert!(visible_scope(LocalRect::new(100., 500., 200., 200.), (800., 500.)).is_none());
    }
    #[test]
    fn viewport_start_and_center_share_scope_insets_and_available_size() {
        let bounds = LocalRect::new(100., 200., 400., 300.);
        let start = place_viewport(bounds, (160., 80.), ViewportAlign::Start, 16.);
        let center = place_viewport(bounds, (160., 80.), ViewportAlign::Center, 16.);
        assert_eq!(
            (
                start.x,
                start.y,
                start.available_width,
                start.available_height
            ),
            (220., 216., 368., 268.)
        );
        assert_eq!((center.x, center.y), (220., 310.));
        let oversized = place_viewport(bounds, (600., 500.), ViewportAlign::Center, 16.);
        assert_eq!((oversized.x, oversized.y), (116., 216.));
    }

    #[test]
    fn rejects_invalid_public_numbers_before_geometry() {
        let mut config = FloatingPosition {
            side_offset: f64::NAN,
            ..Default::default()
        };
        assert_eq!(config.validate().unwrap_err().field, "side_offset");
        config.side_offset = 0.;
        config.arrow_padding = -1.;
        assert_eq!(config.validate().unwrap_err().field, "arrow_padding");
        config.arrow_padding = 0.;
        config.collision_padding.left = f64::INFINITY;
        assert_eq!(
            config.validate().unwrap_err().field,
            "collision_padding.left"
        );
        config.collision_padding.left = -1.;
        assert!(config.validate().is_err());
        config.collision_padding.left = 0.;
        assert!(config.validate().is_ok());
    }
    #[test]
    fn all_physical_sides_use_the_requested_gap() {
        for (side, expected) in [
            (Side::Top, (90., 52.)),
            (Side::Right, (148., 90.)),
            (Side::Bottom, (90., 128.)),
            (Side::Left, (32., 90.)),
        ] {
            let config = FloatingPosition {
                side,
                side_offset: 8.,
                ..Default::default()
            };
            let result = resolve(
                config,
                LocalRect::new(100., 100., 40., 20.),
                LocalRect::new(0., 0., 300., 300.),
            );
            assert_eq!((result.x, result.y), expected);
            assert_eq!(result.placement.side, side);
        }
    }
    #[test]
    fn flips_to_the_less_overflowing_side_and_keeps_ties() {
        let result = resolve(
            FloatingPosition::default(),
            LocalRect::new(100., 180., 40., 20.),
            LocalRect::new(0., 0., 300., 200.),
        );
        assert_eq!(result.placement.side, Side::Top);
        assert_eq!(result.y, 140.);
        let result = resolve(
            FloatingPosition::default(),
            LocalRect::new(100., 90., 40., 20.),
            LocalRect::new(0., 0., 300., 200.),
        );
        assert_eq!(result.placement.side, Side::Bottom);
    }
    #[test]
    fn rtl_changes_logical_alignment_but_not_physical_sides() {
        let bounds = LocalRect::new(0., 0., 300., 300.);
        let anchor = LocalRect::new(100., 100., 40., 20.);
        let config = FloatingPosition {
            align: Align::Start,
            ..Default::default()
        };
        let ltr = place_floating(
            anchor,
            (60., 40.),
            None,
            bounds,
            Direction::LeftToRight,
            &config,
        );
        let rtl = place_floating(
            anchor,
            (60., 40.),
            None,
            bounds,
            Direction::RightToLeft,
            &config,
        );
        assert_eq!((ltr.x, rtl.x), (100., 80.));
        let config = FloatingPosition {
            side: Side::Right,
            ..config
        };
        assert_eq!(
            place_floating(
                anchor,
                (60., 40.),
                None,
                bounds,
                Direction::LeftToRight,
                &config
            )
            .y,
            place_floating(
                anchor,
                (60., 40.),
                None,
                bounds,
                Direction::RightToLeft,
                &config
            )
            .y
        );
    }
    #[test]
    fn sticky_and_arrow_results_come_from_the_resolved_coordinates() {
        let bounds = LocalRect::new(0., 0., 100., 100.);
        let anchor = LocalRect::new(200., 20., 20., 20.);
        let mut config = FloatingPosition {
            avoid_collisions: true,
            ..Default::default()
        };
        let partial = place_floating(
            anchor,
            (40., 20.),
            Some((10., 5.)),
            bounds,
            Direction::LeftToRight,
            &config,
        );
        config.sticky = Sticky::Always;
        let always = place_floating(
            anchor,
            (40., 20.),
            Some((10., 5.)),
            bounds,
            Direction::LeftToRight,
            &config,
        );
        assert!(partial.x >= 160.);
        assert_eq!(always.x, 60.);
        assert!(always.arrow_hidden);
        assert!(always.detached);
    }
    #[test]
    fn scoped_padding_offsets_and_arrow_share_one_geometry_result() {
        let config = FloatingPosition {
            align: Align::Start,
            side_offset: 3.,
            align_offset: 4.,
            arrow_padding: 6.,
            collision_padding: CollisionPadding {
                top: 10.,
                right: 20.,
                bottom: 30.,
                left: 10.,
            },
            ..Default::default()
        };
        let result = place_floating(
            LocalRect::new(150., 150., 20., 20.),
            (60., 40.),
            Some((10., 5.)),
            LocalRect::new(100., 100., 200., 200.),
            Direction::LeftToRight,
            &config,
        );
        assert_eq!((result.x, result.y), (154., 178.));
        assert_eq!(result.arrow, Some((6., -5.)));
        assert!(!result.arrow_hidden);
        assert_eq!(
            (result.available_width, result.available_height),
            (170., 92.)
        );
        let config = FloatingPosition {
            side: Side::Left,
            side_offset: 2.,
            avoid_collisions: false,
            ..Default::default()
        };
        let result = resolve(
            config,
            LocalRect::new(5., 5., 10., 10.),
            LocalRect::new(0., 0., 100., 100.),
        );
        assert_eq!((result.x, result.y), (-57., -10.));
    }
    #[test]
    fn detached_anchor_cannot_offer_more_space_than_the_visible_boundary() {
        let result = resolve(
            FloatingPosition {
                side: Side::Top,
                ..Default::default()
            },
            LocalRect::new(200., 200., 20., 20.),
            LocalRect::new(0., 0., 100., 100.),
        );
        assert!(result.detached);
        assert!(result.available_height <= 100.);
        assert!(result.available_width <= 100.);
    }
}
