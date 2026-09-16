use std::collections::HashSet;

use crate::affine::{multiply, point, Matrix, Point, Rect};
use crate::document::Node;
use crate::gesture::{path_points, Camera, PathPoint};
use crate::scene::Frame;

pub trait PathTester {
    fn in_path(&self, node: &Node, p: Point) -> bool;
    fn in_stroke(&self, node: &Node, p: Point, width: f64) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct HitOptions<'a> {
    pub deep: bool,
    pub frames_only: bool,
    pub zoom: f64,
    pub editing: Option<&'a str>,
}

impl Default for HitOptions<'_> {
    fn default() -> Self {
        Self {
            deep: false,
            frames_only: false,
            zoom: 1.0,
            editing: None,
        }
    }
}

pub fn inside_round(p: Point, w: f64, h: f64, radius: f64) -> bool {
    if p.x < 0.0 || p.y < 0.0 || p.x > w || p.y > h {
        return false;
    }
    let r = radius.min(w / 2.0).min(h / 2.0);
    let cx = p.x.max(r).min(w - r);
    let cy = p.y.max(r).min(h - r);
    (p.x - cx).powi(2) + (p.y - cy).powi(2) <= r * r + 1e-6
}

pub fn hit_test<'a>(
    frame: &'a Frame,
    world: Point,
    options: HitOptions<'_>,
    tester: &impl PathTester,
) -> Option<&'a Node> {
    for s in frame.items.iter().rev() {
        let n = &s.node;
        if s.locked
            || options.editing == Some(n.id.as_str())
            || n.kind == "group"
            || (options.frames_only && n.kind != "frame")
        {
            continue;
        }
        if s.clips.iter().any(|clip| {
            !inside_round(
                point(clip.inverse, world.x, world.y),
                clip.w,
                clip.h,
                clip.radius,
            )
        }) {
            continue;
        }
        let p = point(s.inverse, world.x, world.y);
        let tol = 4.0 / options.zoom;
        let hit = match n.kind.as_str() {
            "ellipse" => {
                ((p.x - n.w / 2.0) / (n.w / 2.0 + tol)).powi(2)
                    + ((p.y - n.h / 2.0) / (n.h / 2.0 + tol)).powi(2)
                    <= 1.0
            }
            "path" | "line" => {
                (n.fill != "none" && tester.in_path(n, p))
                    || tester.in_stroke(n, p, n.stroke_width.max(8.0 / options.zoom))
            }
            _ => p.x >= -tol && p.y >= -tol && p.x <= n.w + tol && p.y <= n.h + tol,
        };
        if !hit {
            continue;
        }
        if !options.deep && !options.frames_only {
            let mut parent = n.parent_id.as_deref();
            while let Some(ancestor) = parent.and_then(|id| frame.world(id)) {
                let a = &ancestor.node;
                if a.kind == "group" || a.flag("isInstance") || a.flag("component") {
                    return Some(a);
                }
                parent = a.parent_id.as_deref();
            }
        }
        return Some(n);
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectionGeometry {
    pub matrix: Matrix,
    pub w: f64,
    pub h: f64,
}

pub fn selection_geometry(
    frame: &Frame,
    roots: &[String],
    selection: &[String],
) -> Option<SelectionGeometry> {
    match roots {
        [] => None,
        [id] => frame.world(id).map(|s| SelectionGeometry {
            matrix: s.matrix,
            w: s.node.w,
            h: s.node.h,
        }),
        _ => frame.bounds(Some(selection)).map(|b| SelectionGeometry {
            matrix: Matrix([1.0, 0.0, 0.0, 1.0, b.x, b.y]),
            w: b.w,
            h: b.h,
        }),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Nw,
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Rotate,
}

impl Handle {
    pub fn west(self) -> bool {
        matches!(self, Self::Nw | Self::W | Self::Sw)
    }
    pub fn east(self) -> bool {
        matches!(self, Self::Ne | Self::E | Self::Se)
    }
    pub fn north(self) -> bool {
        matches!(self, Self::Nw | Self::N | Self::Ne)
    }
    pub fn south(self) -> bool {
        matches!(self, Self::Sw | Self::S | Self::Se)
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Nw => "nw",
            Self::N => "n",
            Self::Ne => "ne",
            Self::E => "e",
            Self::Se => "se",
            Self::S => "s",
            Self::Sw => "sw",
            Self::W => "w",
            Self::Rotate => "rotate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectionHandle {
    pub name: Handle,
    pub position: Point,
}

pub fn handles_for(g: SelectionGeometry, camera: Camera) -> Vec<SelectionHandle> {
    let positions = [
        (Handle::Nw, 0.0, 0.0),
        (Handle::N, 0.5, 0.0),
        (Handle::Ne, 1.0, 0.0),
        (Handle::E, 1.0, 0.5),
        (Handle::Se, 1.0, 1.0),
        (Handle::S, 0.5, 1.0),
        (Handle::Sw, 0.0, 1.0),
        (Handle::W, 0.0, 0.5),
    ];
    let mut handles: Vec<_> = positions
        .into_iter()
        .map(|(name, x, y)| SelectionHandle {
            name,
            position: camera.world_to_screen(point(g.matrix, g.w * x, g.h * y)),
        })
        .collect();
    handles.push(SelectionHandle {
        name: Handle::Rotate,
        position: camera.world_to_screen(point(g.matrix, g.w / 2.0, -25.0 / camera.zoom)),
    });
    handles
}

pub fn handle_at(handles: &[SelectionHandle], screen: Point) -> Option<&SelectionHandle> {
    handles.iter().find(|h| h.position.distance(screen) < 7.0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathHandleKind {
    In,
    Out,
    Anchor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathHandle {
    pub index: usize,
    pub kind: PathHandleKind,
    pub matrix: Matrix,
}

pub fn path_matrix(node: &Node, world: Matrix) -> Matrix {
    let sx = node.w / path_extent(node.number("pathW", node.w), node.w);
    let sy = node.h / path_extent(node.number("pathH", node.h), node.h);
    multiply(world, Matrix([sx, 0.0, 0.0, sy, 0.0, 0.0]))
}

fn path_extent(value: f64, size: f64) -> f64 {
    if value != 0.0 {
        value
    } else if size != 0.0 {
        size
    } else {
        1.0
    }
}

pub fn path_handle_at(
    node: &Node,
    matrix: Matrix,
    camera: Camera,
    screen: Point,
) -> Option<PathHandle> {
    let matrix = path_matrix(node, matrix);
    for (index, a) in path_points(node).iter().enumerate() {
        for (kind, p) in path_handles(a) {
            if let Some(p) = p {
                let sp = camera.world_to_screen(point(matrix, p.x, p.y));
                if sp.distance(screen) < 7.0 {
                    return Some(PathHandle {
                        index,
                        kind,
                        matrix,
                    });
                }
            }
        }
    }
    None
}

fn path_handles(a: &PathPoint) -> [(PathHandleKind, Option<Point>); 3] {
    [
        (PathHandleKind::In, a.in_handle),
        (PathHandleKind::Out, a.out_handle),
        (PathHandleKind::Anchor, Some(Point::new(a.x, a.y))),
    ]
}

pub fn marquee(frame: &Frame, bounds: Rect, deep: bool, old: &[String]) -> Vec<String> {
    let mut selected = old.to_vec();
    for s in &frame.items {
        if s.locked || (!deep && s.node.parent_id.as_deref().is_some_and(|id| !id.is_empty())) {
            continue;
        }
        if bounds.contains(s.bounds) && !selected.contains(&s.node.id) {
            selected.push(s.node.id.clone());
        }
    }
    selected
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Guide {
    pub axis: Axis,
    pub value: f64,
}

pub struct SnapRequest<'a> {
    pub delta: Point,
    pub bounds: Option<Rect>,
    pub moved: &'a HashSet<String>,
    pub roots: &'a [String],
    pub zoom: f64,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnapResult {
    pub delta: Point,
    pub guides: Vec<Guide>,
}

pub fn smart_snap(frame: &Frame, request: SnapRequest<'_>) -> SnapResult {
    let mut result = SnapResult {
        delta: request.delta,
        guides: Vec::new(),
    };
    let Some(mut bounds) = request.bounds else {
        return result;
    };
    if request.disabled {
        return result;
    }
    bounds.x += request.delta.x;
    bounds.y += request.delta.y;
    let parents: HashSet<_> = request
        .roots
        .iter()
        .filter_map(|id| frame.world(id))
        .map(|s| s.node.parent_id.as_deref())
        .collect();
    let mut best_x = 5.0 / request.zoom;
    let mut best_y = best_x;
    let mut adjust = Point::default();
    for s in &frame.items {
        if request.moved.contains(&s.node.id)
            || !parents.contains(&s.node.parent_id.as_deref())
            || s.locked
        {
            continue;
        }
        let b = s.bounds;
        for x in [b.x, b.x + b.w / 2.0, b.x + b.w] {
            for bx in [bounds.x, bounds.x + bounds.w / 2.0, bounds.x + bounds.w] {
                if (x - bx).abs() < best_x {
                    best_x = (x - bx).abs();
                    adjust.x = x - bx;
                    result.guides.retain(|g| g.axis != Axis::X);
                    result.guides.push(Guide {
                        axis: Axis::X,
                        value: x,
                    });
                }
            }
        }
        for y in [b.y, b.y + b.h / 2.0, b.y + b.h] {
            for by in [bounds.y, bounds.y + bounds.h / 2.0, bounds.y + bounds.h] {
                if (y - by).abs() < best_y {
                    best_y = (y - by).abs();
                    adjust.y = y - by;
                    result.guides.retain(|g| g.axis != Axis::Y);
                    result.guides.push(Guide {
                        axis: Axis::Y,
                        value: y,
                    });
                }
            }
        }
    }
    result.delta.x += adjust.x;
    result.delta.y += adjust.y;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::affine::identity;
    use crate::document::Document;
    use crate::scene::compose;

    #[test]
    fn shallow_marquee_includes_empty_parent_ids() {
        let mut doc = Document::empty("empty parent");
        let mut n = Node::new("rect");
        n.id = "root".into();
        n.parent_id = Some(String::new());
        doc.add(n);
        assert_eq!(
            marquee(
                &compose(&doc),
                Rect::new(0.0, 0.0, 160.0, 100.0),
                false,
                &[]
            ),
            ["root"]
        );
    }

    #[test]
    fn zero_path_dimensions_fall_back_to_node_dimensions() {
        let mut n = Node::new("path");
        n.extra.insert("pathW".into(), serde_json::json!(0));
        n.extra.insert("pathH".into(), serde_json::json!(0));
        assert_eq!(path_matrix(&n, identity()), identity());
    }

    #[test]
    fn inherited_lock_skips_the_entire_subtree() {
        let mut doc = Document::empty("locked");
        let mut parent = Node::new("group");
        parent.id = "g".into();
        parent.locked = true;
        doc.add(parent);
        let mut child = Node::new("rect");
        child.parent_id = Some("g".into());
        doc.add(child);
        assert!(hit_test(
            &compose(&doc),
            Point::new(50.0, 50.0),
            HitOptions {
                deep: true,
                ..HitOptions::default()
            },
            &NoPath
        )
        .is_none());
    }

    #[test]
    fn rounded_frame_clip_excludes_child_corners() {
        let mut doc = Document::empty("clip");
        let mut parent = Node::new("frame");
        parent.id = "f".into();
        parent.radius = 20.0;
        parent.w = 100.0;
        parent.h = 100.0;
        doc.add(parent);
        let mut child = Node::new("rect");
        child.id = "child".into();
        child.parent_id = Some("f".into());
        doc.add(child);
        let f = compose(&doc);
        assert_eq!(
            hit_test(
                &f,
                Point::new(0.0, 0.0),
                HitOptions {
                    deep: true,
                    ..HitOptions::default()
                },
                &NoPath
            )
            .unwrap()
            .id,
            "f"
        );
        assert_eq!(
            hit_test(
                &f,
                Point::new(20.0, 20.0),
                HitOptions {
                    deep: true,
                    ..HitOptions::default()
                },
                &NoPath
            )
            .unwrap()
            .id,
            "child"
        );
    }

    #[test]
    fn path_tester_receives_local_coordinates_and_zoom_stroke_tolerance() {
        use std::cell::Cell;
        struct Probe {
            width: Cell<f64>,
            point: Cell<Point>,
            fill: Cell<bool>,
        }
        impl PathTester for Probe {
            fn in_path(&self, _: &Node, _: Point) -> bool {
                self.fill.set(true);
                false
            }
            fn in_stroke(&self, _: &Node, p: Point, width: f64) -> bool {
                self.width.set(width);
                self.point.set(p);
                true
            }
        }
        let mut doc = Document::empty("path");
        let mut path = Node::new("path");
        path.x = 20.0;
        path.y = 30.0;
        path.stroke_width = 1.0;
        doc.add(path);
        let probe = Probe {
            width: Cell::new(0.0),
            point: Cell::new(Point::default()),
            fill: Cell::new(false),
        };
        assert!(hit_test(
            &compose(&doc),
            Point::new(24.0, 36.0),
            HitOptions {
                zoom: 2.0,
                ..HitOptions::default()
            },
            &probe
        )
        .is_some());
        assert_eq!(
            (probe.width.get(), probe.point.get(), probe.fill.get()),
            (4.0, Point::new(4.0, 6.0), true)
        );
    }

    struct NoPath;
    impl PathTester for NoPath {
        fn in_path(&self, _: &Node, _: Point) -> bool {
            false
        }
        fn in_stroke(&self, _: &Node, _: Point, _: f64) -> bool {
            false
        }
    }

    fn frame() -> Frame {
        let mut doc = Document::empty("hit");
        let mut group = Node::new("group");
        group.id = "group".into();
        doc.add(group);
        let mut child = Node::new("ellipse");
        child.id = "ellipse".into();
        child.parent_id = Some("group".into());
        child.w = 100.0;
        child.h = 100.0;
        doc.add(child);
        compose(&doc)
    }

    #[test]
    fn deep_selection_reaches_the_child() {
        let f = frame();
        let p = Point::new(50.0, 50.0);
        assert_eq!(
            hit_test(&f, p, HitOptions::default(), &NoPath).unwrap().id,
            "group"
        );
        assert_eq!(
            hit_test(
                &f,
                p,
                HitOptions {
                    deep: true,
                    ..HitOptions::default()
                },
                &NoPath
            )
            .unwrap()
            .id,
            "ellipse"
        );
    }

    #[test]
    fn ellipse_uses_curved_boundary_and_zoom_tolerance() {
        let f = frame();
        assert!(hit_test(&f, Point::new(0.0, 0.0), HitOptions::default(), &NoPath).is_none());
        assert!(hit_test(&f, Point::new(104.0, 50.0), HitOptions::default(), &NoPath).is_some());
        assert!(hit_test(&f, Point::new(104.1, 50.0), HitOptions::default(), &NoPath).is_none());
    }

    #[test]
    fn handles_have_a_strict_seven_pixel_radius() {
        let handles = handles_for(
            SelectionGeometry {
                matrix: identity(),
                w: 100.0,
                h: 50.0,
            },
            Camera::default(),
        );
        assert_eq!(
            handle_at(&handles, Point::new(6.999, 0.0)).unwrap().name,
            Handle::Nw
        );
        assert!(handle_at(&handles, Point::new(7.0, 0.0)).is_none());
        assert_eq!(handles.last().unwrap().position, Point::new(50.0, -25.0));
    }

    #[test]
    fn marquee_requires_full_containment() {
        let f = frame();
        assert!(marquee(&f, Rect::new(0.0, 0.0, 99.0, 100.0), true, &[]).is_empty());
        assert_eq!(
            marquee(&f, Rect::new(0.0, 0.0, 100.0, 100.0), true, &[]),
            ["ellipse"]
        );
    }

    #[test]
    fn snap_threshold_scales_with_zoom_and_reports_the_axis() {
        let mut doc = Document::empty("snap");
        let mut n = Node::new("rect");
        n.id = "target".into();
        n.x = 100.0;
        n.y = 500.0;
        n.w = 10.0;
        doc.add(n);
        let mut moving = Node::new("rect");
        moving.id = "moving".into();
        doc.add(moving);
        let f = compose(&doc);
        let moved = HashSet::from(["moving".to_owned()]);
        let roots = ["moving".to_owned()];
        let snapped = smart_snap(
            &f,
            SnapRequest {
                delta: Point::new(0.0, 0.0),
                bounds: Some(Rect::new(88.0, 0.0, 10.0, 10.0)),
                moved: &moved,
                roots: &roots,
                zoom: 2.0,
                disabled: false,
            },
        );
        assert_eq!(snapped.delta.x, 2.0);
        assert_eq!(
            snapped.guides,
            [Guide {
                axis: Axis::X,
                value: 100.0
            }]
        );
        let exact_boundary = smart_snap(
            &f,
            SnapRequest {
                delta: Point::new(-0.5, 0.0),
                bounds: Some(Rect::new(88.0, 0.0, 10.0, 10.0)),
                moved: &moved,
                roots: &roots,
                zoom: 2.0,
                disabled: false,
            },
        );
        assert_eq!(exact_boundary.delta.x, -0.5);
    }
}
