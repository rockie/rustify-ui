use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::affine::{identity, inverse, local_matrix, point, Matrix, Point, Rect};
use crate::document::{Document, Node};
use crate::hit::{Handle, PathHandle, PathHandleKind, SelectionGeometry};
use crate::layout::constrain_children;
use crate::scene::compose;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn screen_to_world(self, p: Point) -> Point {
        Point::new((p.x - self.x) / self.zoom, (p.y - self.y) / self.zoom)
    }
    pub fn world_to_screen(self, p: Point) -> Point {
        Point::new(p.x * self.zoom + self.x, p.y * self.zoom + self.y)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub command: bool,
}

pub fn fit(bounds: Option<Rect>, viewport: Point) -> Camera {
    let Some(b) = bounds else {
        return Camera {
            x: 80.0,
            y: 100.0,
            zoom: 1.0,
        };
    };
    let zoom = ((viewport.x - 90.0) / b.w.max(1.0))
        .min((viewport.y - 205.0) / b.h.max(1.0))
        .clamp(0.02, 2.0);
    Camera {
        x: (viewport.x - b.w * zoom) / 2.0 - b.x * zoom,
        y: 67.0 - b.y * zoom,
        zoom,
    }
}

pub fn zoom_at(camera: Camera, factor: f64, screen: Point) -> Camera {
    let world = camera.screen_to_world(screen);
    let zoom = (camera.zoom * factor).clamp(0.02, 64.0);
    Camera {
        x: screen.x - world.x * zoom,
        y: screen.y - world.y * zoom,
        zoom,
    }
}

pub fn pan(camera: Camera, start: Point, screen: Point) -> Camera {
    Camera {
        x: camera.x + screen.x - start.x,
        y: camera.y + screen.y - start.y,
        ..camera
    }
}

pub fn pinch(
    camera: Camera,
    start_center: Point,
    start_distance: f64,
    a: Point,
    b: Point,
) -> Camera {
    let center = Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
    let ratio = a.distance(b) / start_distance.max(1.0);
    let zoom = (camera.zoom * ratio).clamp(0.02, 64.0);
    Camera {
        x: center.x - (start_center.x - camera.x) / camera.zoom * zoom,
        y: center.y - (start_center.y - camera.y) / camera.zoom * zoom,
        zoom,
    }
}

pub fn wheel(camera: Camera, delta: Point, cursor: Point, modifiers: Modifiers) -> Camera {
    if modifiers.command || modifiers.alt {
        zoom_at(
            camera,
            (-delta.y.clamp(-100.0, 100.0) * 0.012).exp(),
            cursor,
        )
    } else {
        Camera {
            x: camera.x
                - if modifiers.shift && delta.x == 0.0 {
                    delta.y
                } else {
                    delta.x
                },
            y: camera.y - if modifiers.shift { 0.0 } else { delta.y },
            ..camera
        }
    }
}

pub fn create(
    node: &mut Node,
    local_start: Point,
    world: Point,
    parent_inverse: Matrix,
    modifiers: Modifiers,
) {
    let q = point(parent_inverse, world.x, world.y);
    let mut dx = q.x - local_start.x;
    let mut dy = q.y - local_start.y;
    if modifiers.shift {
        let size = dx.abs().max(dy.abs());
        dx = if dx < 0.0 { -size } else { size };
        dy = if dy < 0.0 { -size } else { size };
    }
    if node.kind == "line" {
        let len = dx.hypot(dy);
        node.x = local_start.x + dx / 2.0 - len / 2.0;
        node.y = local_start.y + dy / 2.0 - 0.05;
        node.w = len.max(0.1);
        node.h = 0.1;
        let angle = dy.atan2(dx).to_degrees();
        node.rotation = if modifiers.shift {
            js_round(angle / 45.0) * 45.0
        } else {
            angle
        };
    } else {
        node.x = local_start.x + dx.min(0.0);
        node.y = local_start.y + dy.min(0.0);
        node.w = dx.abs().max(0.1);
        node.h = dy.abs().max(0.1);
        if modifiers.alt {
            node.x = local_start.x - node.w;
            node.y = local_start.y - node.h;
            node.w *= 2.0;
            node.h *= 2.0;
        }
    }
}

pub fn finish_create(node: &mut Node) {
    if node.w < 3.0 && node.h < 3.0 {
        node.w = if node.kind == "frame" { 400.0 } else { 120.0 };
        node.h = match node.kind.as_str() {
            "frame" => 300.0,
            "line" => 0.1,
            _ => 100.0,
        };
    }
}

pub fn move_delta(start: Point, world: Point, shift: bool) -> Point {
    let mut delta = Point::new(world.x - start.x, world.y - start.y);
    if shift {
        if delta.x.abs() > delta.y.abs() {
            delta.y = 0.0;
        } else {
            delta.x = 0.0;
        }
    }
    delta
}

pub fn move_node(node: &mut Node, original: &Node, parent_inverse: Matrix, delta: Point) {
    node.x = original.x + parent_inverse[0] * delta.x + parent_inverse[2] * delta.y;
    node.y = original.y + parent_inverse[1] * delta.x + parent_inverse[3] * delta.y;
}

pub fn moved(delta: Point, zoom: f64) -> bool {
    delta.x.hypot(delta.y) * zoom > 2.0
}

pub fn marquee_bounds(start: Point, current: Point) -> Rect {
    Rect::new(
        start.x.min(current.x),
        start.y.min(current.y),
        (current.x - start.x).abs(),
        (current.y - start.y).abs(),
    )
}

pub fn save_originals(doc: &Document, roots: &[String]) -> HashMap<String, Node> {
    let mut originals = HashMap::new();
    for id in roots {
        if let Some(n) = doc.get(id) {
            originals.insert(id.clone(), n.clone());
            for child in doc.descendants(id) {
                originals.insert(child.id.clone(), child.clone());
            }
        }
    }
    originals
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resize {
    pub left: f64,
    pub top: f64,
    pub w: f64,
    pub h: f64,
    pub sx: f64,
    pub sy: f64,
}

pub fn resize_geometry(
    g: SelectionGeometry,
    handle: Handle,
    world: Point,
    modifiers: Modifiers,
) -> Resize {
    let local = point(inverse(g.matrix), world.x, world.y);
    let (mut left, mut top, mut right, mut bottom) = (0.0, 0.0, g.w, g.h);
    if handle.west() {
        left = local.x.min(right - 0.1);
    }
    if handle.east() {
        right = local.x.max(left + 0.1);
    }
    if handle.north() {
        top = local.y.min(bottom - 0.1);
    }
    if handle.south() {
        bottom = local.y.max(top + 0.1);
    }
    if modifiers.shift {
        let ratio = g.w / if g.h == 0.0 { 1.0 } else { g.h };
        let (nw, nh) = (right - left, bottom - top);
        if nw / nh > ratio {
            if handle.north() {
                top = bottom - nw / ratio;
            } else {
                bottom = top + nw / ratio;
            }
        } else if handle.west() {
            left = right - nh * ratio;
        } else {
            right = left + nh * ratio;
        }
    }
    if modifiers.alt {
        if handle.west() {
            right = g.w - left;
        }
        if handle.east() {
            left = g.w - right;
        }
        if handle.north() {
            bottom = g.h - top;
        }
        if handle.south() {
            top = g.h - bottom;
        }
    }
    let w = (right - left).max(0.1);
    let h = (bottom - top).max(0.1);
    Resize {
        left,
        top,
        w,
        h,
        sx: w / g.w,
        sy: h / g.h,
    }
}

pub fn resize(
    doc: &mut Document,
    roots: &[String],
    originals: &HashMap<String, Node>,
    geometry: SelectionGeometry,
    handle: Handle,
    world: Point,
    modifiers: Modifiers,
) {
    let r = resize_geometry(geometry, handle, world, modifiers);
    for (id, original) in originals {
        if let Some(n) = doc.get_mut(id) {
            *n = original.clone();
        }
    }
    doc.touch(None);
    let frame = compose(doc);
    for id in roots {
        let Some(original) = originals.get(id) else {
            continue;
        };
        let Some(scene) = frame.world(id) else {
            continue;
        };
        let parent_inverse = original
            .parent_id
            .as_deref()
            .and_then(|p| frame.world(p))
            .map_or_else(identity, |p| p.inverse);
        let Some(n) = doc.get_mut(id) else { continue };
        if roots.len() == 1 {
            let world_tl = point(geometry.matrix, r.left, r.top);
            let tl = point(parent_inverse, world_tl.x, world_tl.y);
            n.w = r.w;
            n.h = r.h;
            position_from_top_left(n, tl);
        } else {
            let old = scene.bounds;
            let new_x = geometry.matrix[4] + r.left + (old.x - geometry.matrix[4]) * r.sx;
            let new_y = geometry.matrix[5] + r.top + (old.y - geometry.matrix[5]) * r.sy;
            move_node(
                n,
                original,
                parent_inverse,
                Point::new(new_x - old.x, new_y - old.y),
            );
            n.w = original.w * r.sx;
            n.h = original.h * r.sy;
            if n.kind == "text" {
                n.font_size = original.font_size * r.sx.min(r.sy);
            }
        }
        constrain_children(doc, id, original.w, original.h);
        if roots.len() == 1 {
            if let Some(n) = doc.get_mut(id) {
                if n.kind == "text" {
                    n.h = n.h.max(n.font_size * n.line_height);
                }
            }
        }
        doc.touch(Some(id));
    }
}

fn position_from_top_left(n: &mut Node, tl: Point) {
    let (s, c) = n.rotation.to_radians().sin_cos();
    n.x = tl.x - n.w / 2.0 + c * n.w / 2.0 - s * n.h / 2.0;
    n.y = tl.y - n.h / 2.0 + s * n.w / 2.0 + c * n.h / 2.0;
}

fn js_round(value: f64) -> f64 {
    (value + 0.5).floor()
}

pub fn rotation_delta(center: Point, start: Point, world: Point, shift: bool) -> f64 {
    let angle = (world.y - center.y).atan2(world.x - center.x);
    let start_angle = (start.y - center.y).atan2(start.x - center.x);
    let delta = (angle - start_angle).to_degrees();
    if shift {
        js_round(delta / 15.0) * 15.0
    } else {
        delta
    }
}

pub fn rotate(
    doc: &mut Document,
    roots: &[String],
    originals: &HashMap<String, Node>,
    center: Point,
    start: Point,
    world: Point,
    shift: bool,
) {
    let delta = rotation_delta(center, start, world, shift);
    let frame = compose(doc);
    for id in roots {
        let Some(original) = originals.get(id) else {
            continue;
        };
        let parent = original
            .parent_id
            .as_deref()
            .and_then(|p| frame.world(p))
            .map_or_else(identity, |s| s.matrix);
        let Some(n) = doc.get_mut(id) else { continue };
        n.rotation = original.rotation + delta;
        if roots.len() > 1 {
            let wc = point(
                parent,
                original.x + original.w / 2.0,
                original.y + original.h / 2.0,
            );
            let (s, c) = delta.to_radians().sin_cos();
            let (dx, dy) = (wc.x - center.x, wc.y - center.y);
            let q = point(
                inverse(parent),
                center.x + dx * c - dy * s,
                center.y + dx * s + dy * c,
            );
            n.x = q.x - n.w / 2.0;
            n.y = q.y - n.h / 2.0;
        }
        doc.touch(Some(id));
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
    #[serde(
        rename = "in",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_handle"
    )]
    pub in_handle: Option<Point>,
    #[serde(
        rename = "out",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_handle"
    )]
    pub out_handle: Option<Point>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

fn optional_handle<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Point>, D::Error> {
    let value = Value::deserialize(deserializer)?;
    let absent = value.is_null()
        || value == Value::Bool(false)
        || value.as_f64() == Some(0.0)
        || value.as_str() == Some("");
    if absent {
        Ok(None)
    } else {
        serde_json::from_value(value)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

pub fn path_points(node: &Node) -> Vec<PathPoint> {
    node.attr("points")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
}

pub fn pen_handle(anchor: &mut PathPoint, start: Point, screen: Point, world: Point) -> bool {
    if screen.distance(start) <= 3.0 {
        return false;
    }
    anchor.out_handle = Some(world);
    anchor.in_handle = Some(Point::new(
        2.0 * anchor.x - world.x,
        2.0 * anchor.y - world.y,
    ));
    true
}

pub fn pen_can_close(first: Point, world: Point, zoom: f64, point_count: usize) -> bool {
    point_count >= 2 && first.distance(world) * zoom < 8.0
}

pub fn pen_anchor(previous: Option<Point>, world: Point, shift: bool) -> PathPoint {
    let p = if let Some(previous) = previous.filter(|_| shift) {
        let dx = world.x - previous.x;
        let dy = world.y - previous.y;
        let angle =
            js_round(dy.atan2(dx) / std::f64::consts::FRAC_PI_4) * std::f64::consts::FRAC_PI_4;
        let length = dx.hypot(dy);
        Point::new(
            previous.x + angle.cos() * length,
            previous.y + angle.sin() * length,
        )
    } else {
        world
    };
    PathPoint {
        x: p.x,
        y: p.y,
        ..PathPoint::default()
    }
}

pub fn edit_path_point(
    original: &PathPoint,
    kind: PathHandleKind,
    q: Point,
    alt: bool,
) -> PathPoint {
    let mut a = original.clone();
    match kind {
        PathHandleKind::Anchor => {
            let dx = q.x - original.x;
            let dy = q.y - original.y;
            a.x = q.x;
            a.y = q.y;
            a.in_handle = original.in_handle.map(|p| Point::new(p.x + dx, p.y + dy));
            a.out_handle = original.out_handle.map(|p| Point::new(p.x + dx, p.y + dy));
        }
        PathHandleKind::In => {
            a.in_handle = Some(q);
            if !alt {
                a.out_handle = Some(Point::new(2.0 * a.x - q.x, 2.0 * a.y - q.y));
            }
        }
        PathHandleKind::Out => {
            a.out_handle = Some(q);
            if !alt {
                a.in_handle = Some(Point::new(2.0 * a.x - q.x, 2.0 * a.y - q.y));
            }
        }
    }
    a
}

pub fn edit_path(node: &mut Node, original: &Node, handle: PathHandle, world: Point, alt: bool) {
    let originals = path_points(original);
    let Some(orig) = originals.get(handle.index) else {
        return;
    };
    let mut points = path_points(node);
    let Some(a) = points.get_mut(handle.index) else {
        return;
    };
    let q = point(inverse(handle.matrix), world.x, world.y);
    // Anchor moves are measured from pointer-down; tangent edits preserve the current opposite handle.
    let source = if handle.kind == PathHandleKind::Anchor {
        orig
    } else {
        &*a
    };
    *a = edit_path_point(source, handle.kind, q, alt);
    node.extra.insert("points".into(), json!(points));
}

pub fn normalize_path(node: &mut Node) {
    let mut points = path_points(node);
    if points.is_empty() {
        return;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for a in &points {
        for p in [Some(Point::new(a.x, a.y)), a.in_handle, a.out_handle]
            .into_iter()
            .flatten()
        {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
    }
    let path_w = node.number("pathW", node.w);
    let path_h = node.number("pathH", node.h);
    let sx = node.w
        / if path_w == 0.0 {
            if node.w == 0.0 {
                1.0
            } else {
                node.w
            }
        } else {
            path_w
        };
    let sy = node.h
        / if path_h == 0.0 {
            if node.h == 0.0 {
                1.0
            } else {
                node.h
            }
        } else {
            path_h
        };
    let tl = point(local_matrix(node), min_x * sx, min_y * sy);
    for a in &mut points {
        a.x = (a.x - min_x) * sx;
        a.y = (a.y - min_y) * sy;
        for p in [&mut a.in_handle, &mut a.out_handle].into_iter().flatten() {
            p.x = (p.x - min_x) * sx;
            p.y = (p.y - min_y) * sy;
        }
    }
    node.w = ((max_x - min_x) * sx).max(0.1);
    node.h = ((max_y - min_y) * sy).max(0.1);
    node.extra.insert("pathW".into(), json!(node.w));
    node.extra.insert("pathH".into(), json!(node.h));
    node.extra.insert("points".into(), json!(points));
    position_from_top_left(node, tl);
}

pub fn record_overrides(doc: &mut Document, originals: &HashMap<String, Node>) {
    for (id, original) in originals {
        let Some(n) = doc.get_mut(id) else { continue };
        if n.string("sourceId").is_none() {
            continue;
        }
        let fields = [
            ("x", n.x, original.x),
            ("y", n.y, original.y),
            ("w", n.w, original.w),
            ("h", n.h, original.h),
            ("rotation", n.rotation, original.rotation),
            ("fontSize", n.font_size, original.font_size),
        ];
        let overrides = n.extra.entry("overrides").or_insert_with(|| json!({}));
        if let Some(overrides) = overrides.as_object_mut() {
            for (key, value, previous) in fields {
                if value != previous {
                    overrides.insert(key.into(), json!(value));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn absent_falsy_handles_preserve_the_path_anchors() {
        let mut n = Node::new("path");
        n.extra.insert(
            "points".into(),
            json!([
                {"x": 0, "y": 0, "in": false, "out": 0},
                {"x": 20, "y": 30, "in": "", "out": null}
            ]),
        );
        let points = path_points(&n);
        assert_eq!(points.len(), 2);
        assert_eq!(
            (
                points[1].x,
                points[1].y,
                points[1].in_handle,
                points[1].out_handle
            ),
            (20.0, 30.0, None, None)
        );
    }

    #[test]
    fn single_resize_in_rotated_parent_preserves_the_fixed_world_corner() {
        let mut doc = Document::empty("resize");
        let mut parent = Node::new("frame");
        parent.id = "parent".into();
        parent.x = 80.0;
        parent.y = 60.0;
        parent.rotation = 30.0;
        doc.add(parent);
        let mut text = Node::new("text");
        text.id = "text".into();
        text.parent_id = Some("parent".into());
        text.x = 30.0;
        text.y = 20.0;
        text.w = 100.0;
        text.h = 50.0;
        text.rotation = 45.0;
        doc.add(text);
        let roots = vec!["text".into()];
        let originals = save_originals(&doc, &roots);
        let frame = compose(&doc);
        let matrix = frame.world("text").unwrap().matrix;
        let fixed = point(matrix, 0.0, 0.0);
        resize(
            &mut doc,
            &roots,
            &originals,
            SelectionGeometry {
                matrix,
                w: 100.0,
                h: 50.0,
            },
            Handle::Se,
            point(matrix, 200.0, 100.0),
            Modifiers::default(),
        );
        let new_frame = compose(&doc);
        let new_fixed = point(new_frame.world("text").unwrap().matrix, 0.0, 0.0);
        let n = doc.get("text").unwrap();
        assert!(fixed.distance(new_fixed) < 1e-10);
        assert!((n.w - 200.0).abs() < 1e-10 && (n.h - 100.0).abs() < 1e-10);
        assert_eq!(n.font_size, 24.0);
    }

    #[test]
    fn rotation_negative_half_step_rounds_toward_positive_infinity() {
        assert_eq!(js_round(-0.5), 0.0);
        assert_eq!(js_round(-1.5), -1.0);
    }

    #[test]
    fn pen_closes_two_point_paths_at_less_than_eight_screen_pixels() {
        assert!(pen_can_close(
            Point::new(0.0, 0.0),
            Point::new(3.9, 0.0),
            2.0,
            2
        ));
        assert!(!pen_can_close(
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            2.0,
            2
        ));
    }

    #[test]
    fn pen_shift_snaps_angle_without_changing_length() {
        let p = pen_anchor(Some(Point::new(10.0, 20.0)), Point::new(110.0, 40.0), true);
        assert!((p.x - 10.0 - 100.0_f64.hypot(20.0)).abs() < 1e-10);
        assert_eq!(p.y, 20.0);
    }

    #[test]
    fn pen_handles_require_more_than_three_screen_pixels() {
        let mut a = PathPoint {
            x: 5.0,
            y: 6.0,
            ..PathPoint::default()
        };
        assert!(!pen_handle(
            &mut a,
            Point::default(),
            Point::new(3.0, 0.0),
            Point::new(7.0, 8.0)
        ));
        assert!(pen_handle(
            &mut a,
            Point::default(),
            Point::new(3.1, 0.0),
            Point::new(7.0, 8.0)
        ));
        assert_eq!(a.in_handle, Some(Point::new(3.0, 4.0)));
    }

    #[test]
    fn moving_threshold_is_strictly_greater_than_two_pixels() {
        assert!(!moved(Point::new(1.0, 0.0), 2.0));
        assert!(moved(Point::new(1.01, 0.0), 2.0));
    }

    #[test]
    fn pinch_preserves_the_initial_world_center() {
        let c = Camera {
            x: 20.0,
            y: 10.0,
            zoom: 2.0,
        };
        let start = Point::new(100.0, 80.0);
        let updated = pinch(
            c,
            start,
            50.0,
            Point::new(100.0, 100.0),
            Point::new(200.0, 100.0),
        );
        assert_eq!(updated.zoom, 4.0);
        assert_eq!(
            updated.screen_to_world(Point::new(150.0, 100.0)),
            c.screen_to_world(start)
        );
    }

    #[test]
    fn wheel_shift_translates_vertically_scrolled_input_horizontally() {
        assert_eq!(
            wheel(
                Camera::default(),
                Point::new(0.0, 25.0),
                Point::default(),
                Modifiers {
                    shift: true,
                    ..Modifiers::default()
                }
            ),
            Camera {
                x: -25.0,
                y: 0.0,
                zoom: 1.0
            }
        );
    }

    #[test]
    fn frame_fit_uses_reference_top_offset_and_zoom_limit() {
        assert_eq!(
            fit(None, Point::new(1000.0, 800.0)),
            Camera {
                x: 80.0,
                y: 100.0,
                zoom: 1.0
            }
        );
        let c = fit(
            Some(Rect::new(10.0, 20.0, 100.0, 100.0)),
            Point::new(1000.0, 800.0),
        );
        assert_eq!(
            c,
            Camera {
                x: 380.0,
                y: 27.0,
                zoom: 2.0
            }
        );
    }

    #[test]
    fn moving_in_a_rotated_parent_converts_world_delta_to_local_axes() {
        let original = Node::new("rect");
        let mut n = original.clone();
        move_node(
            &mut n,
            &original,
            inverse(crate::affine::centered(0.0, 0.0, 100.0, 100.0, 90.0)),
            Point::new(50.0, 0.0),
        );
        assert!(n.x.abs() < 1e-10 && (n.y + 50.0).abs() < 1e-10);
    }

    #[test]
    fn path_anchor_translates_both_handles() {
        let original = PathPoint {
            x: 10.0,
            y: 10.0,
            in_handle: Some(Point::new(0.0, 10.0)),
            out_handle: Some(Point::new(20.0, 10.0)),
            ..PathPoint::default()
        };
        let p = edit_path_point(
            &original,
            PathHandleKind::Anchor,
            Point::new(20.0, 30.0),
            false,
        );
        assert_eq!(
            (p.in_handle, p.out_handle),
            (Some(Point::new(10.0, 30.0)), Some(Point::new(30.0, 30.0)))
        );
    }

    #[test]
    fn create_shift_and_alt_make_a_centered_square() {
        let mut n = Node::new("rect");
        create(
            &mut n,
            Point::new(100.0, 100.0),
            Point::new(120.0, 110.0),
            identity(),
            Modifiers {
                shift: true,
                alt: true,
                command: false,
            },
        );
        assert_eq!((n.x, n.y, n.w, n.h), (80.0, 80.0, 40.0, 40.0));
    }

    #[test]
    fn line_creation_snaps_to_45_degrees() {
        let mut n = Node::new("line");
        create(
            &mut n,
            Point::new(0.0, 0.0),
            Point::new(100.0, 20.0),
            identity(),
            Modifiers {
                shift: true,
                ..Modifiers::default()
            },
        );
        assert_eq!(n.rotation, 45.0);
    }

    #[test]
    fn shift_move_locks_to_larger_axis_with_vertical_ties() {
        assert_eq!(
            move_delta(Point::new(10.0, 10.0), Point::new(20.0, 20.0), true),
            Point::new(0.0, 10.0)
        );
    }

    #[test]
    fn resize_shift_and_alt_preserves_ratio_about_center() {
        let g = SelectionGeometry {
            matrix: identity(),
            w: 100.0,
            h: 50.0,
        };
        let r = resize_geometry(
            g,
            Handle::Se,
            Point::new(150.0, 60.0),
            Modifiers {
                shift: true,
                alt: true,
                command: false,
            },
        );
        assert_eq!((r.left, r.top, r.w, r.h), (-50.0, -25.0, 200.0, 100.0));
    }

    #[test]
    fn multiple_resize_scales_text_and_positions() {
        let mut doc = Document::empty("resize");
        let mut a = Node::new("text");
        a.id = "a".into();
        a.w = 100.0;
        a.h = 50.0;
        doc.add(a);
        let mut b = Node::new("rect");
        b.id = "b".into();
        b.x = 100.0;
        b.w = 100.0;
        b.h = 50.0;
        doc.add(b);
        let roots = vec!["a".into(), "b".into()];
        let originals = save_originals(&doc, &roots);
        resize(
            &mut doc,
            &roots,
            &originals,
            SelectionGeometry {
                matrix: identity(),
                w: 200.0,
                h: 50.0,
            },
            Handle::Se,
            Point::new(400.0, 100.0),
            Modifiers::default(),
        );
        assert_eq!(doc.get("a").unwrap().font_size, 48.0);
        assert_eq!(doc.get("b").unwrap().x, 200.0);
    }

    #[test]
    fn rotation_shift_snaps_the_delta_to_fifteen_degrees() {
        let r = rotation_delta(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.98, 0.2),
            true,
        );
        assert_eq!(r, 15.0);
    }

    #[test]
    fn path_alt_breaks_handle_symmetry() {
        let original = PathPoint {
            x: 10.0,
            y: 10.0,
            in_handle: Some(Point::new(0.0, 10.0)),
            out_handle: Some(Point::new(20.0, 10.0)),
            ..PathPoint::default()
        };
        let a = edit_path_point(&original, PathHandleKind::Out, Point::new(30.0, 15.0), true);
        assert_eq!(a.in_handle, original.in_handle);
        let b = edit_path_point(
            &original,
            PathHandleKind::Out,
            Point::new(30.0, 15.0),
            false,
        );
        assert_eq!(b.in_handle, Some(Point::new(-10.0, 5.0)));
    }

    #[test]
    fn normalize_path_preserves_world_points() {
        let mut n = Node::new("path");
        n.w = 100.0;
        n.h = 100.0;
        n.rotation = 30.0;
        n.extra.insert(
            "points".into(),
            json!([{ "x": 20, "y": 30 }, { "x": 80, "y": 90 }]),
        );
        let before = point(local_matrix(&n), 20.0, 30.0);
        normalize_path(&mut n);
        let after = point(local_matrix(&n), 0.0, 0.0);
        assert!(before.distance(after) < 1e-10);
    }

    #[test]
    fn zoom_keeps_the_pointer_world_position_fixed() {
        let c = Camera {
            x: 40.0,
            y: 30.0,
            zoom: 2.0,
        };
        let cursor = Point::new(100.0, 120.0);
        assert_eq!(
            zoom_at(c, 2.0, cursor).screen_to_world(cursor),
            c.screen_to_world(cursor)
        );
    }
}
