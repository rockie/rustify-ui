//! The pointer corridor between a floating surface and its trigger.

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub(super) fn polygon(exit: Point, source: Rect, target: Rect) -> Vec<Point> {
    let left = (source.x - exit.x).abs();
    let right = (source.x + source.width - exit.x).abs();
    let top = (source.y - exit.y).abs();
    let bottom = (source.y + source.height - exit.y).abs();
    let nearest = left.min(right).min(top).min(bottom);
    // Padding extends back into the departed surface, so small pointer motion
    // around its edge does not prematurely leave the transit corridor.
    let padded = if nearest == left {
        [
            Point {
                x: exit.x + 5.,
                y: exit.y - 5.,
            },
            Point {
                x: exit.x + 5.,
                y: exit.y + 5.,
            },
        ]
    } else if nearest == right {
        [
            Point {
                x: exit.x - 5.,
                y: exit.y - 5.,
            },
            Point {
                x: exit.x - 5.,
                y: exit.y + 5.,
            },
        ]
    } else if nearest == top {
        [
            Point {
                x: exit.x - 5.,
                y: exit.y + 5.,
            },
            Point {
                x: exit.x + 5.,
                y: exit.y + 5.,
            },
        ]
    } else {
        [
            Point {
                x: exit.x - 5.,
                y: exit.y - 5.,
            },
            Point {
                x: exit.x + 5.,
                y: exit.y - 5.,
            },
        ]
    };
    let mut points = vec![
        padded[0],
        padded[1],
        Point {
            x: target.x,
            y: target.y,
        },
        Point {
            x: target.x + target.width,
            y: target.y,
        },
        Point {
            x: target.x + target.width,
            y: target.y + target.height,
        },
        Point {
            x: target.x,
            y: target.y + target.height,
        },
    ];
    points.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    points.dedup();
    let mut upper = Vec::new();
    for &point in &points {
        push_hull(&mut upper, point);
    }
    let mut lower = Vec::new();
    for &point in points.iter().rev() {
        push_hull(&mut lower, point);
    }
    upper.pop();
    lower.pop();
    upper.extend(lower);
    upper
}

fn push_hull(hull: &mut Vec<Point>, point: Point) {
    while hull.len() >= 2 {
        let q = hull[hull.len() - 1];
        let r = hull[hull.len() - 2];
        if (q.x - r.x) * (point.y - r.y) < (q.y - r.y) * (point.x - r.x) {
            break;
        }
        hull.pop();
    }
    hull.push(point);
}

pub(super) fn contains(point: Point, polygon: &[Point]) -> bool {
    let Some(mut previous) = polygon.last() else {
        return false;
    };
    let mut inside = false;
    for current in polygon {
        if (current.y > point.y) != (previous.y > point.y)
            && point.x
                < (previous.x - current.x) * (point.y - current.y) / (previous.y - current.y)
                    + current.x
        {
            inside = !inside;
        }
        previous = current;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transit_corridor_connects_the_exit_to_the_target_on_every_side() {
        let source = Rect {
            x: 0.,
            y: 0.,
            width: 20.,
            height: 20.,
        };
        for (exit, target, inside, outside) in [
            (
                Point { x: 10., y: 20. },
                Rect {
                    x: 0.,
                    y: 60.,
                    width: 40.,
                    height: 20.,
                },
                Point { x: 10., y: 40. },
                Point { x: 40., y: 40. },
            ),
            (
                Point { x: 10., y: 0. },
                Rect {
                    x: -10.,
                    y: -60.,
                    width: 40.,
                    height: 20.,
                },
                Point { x: 10., y: -20. },
                Point { x: 40., y: -20. },
            ),
            (
                Point { x: 20., y: 10. },
                Rect {
                    x: 60.,
                    y: -10.,
                    width: 20.,
                    height: 40.,
                },
                Point { x: 40., y: 10. },
                Point { x: 40., y: 40. },
            ),
            (
                Point { x: 0., y: 10. },
                Rect {
                    x: -60.,
                    y: -10.,
                    width: 20.,
                    height: 40.,
                },
                Point { x: -20., y: 10. },
                Point { x: -20., y: 40. },
            ),
        ] {
            let corridor = polygon(exit, source, target);
            assert!(
                contains(inside, &corridor),
                "missing transit point {inside:?}: {corridor:?}"
            );
            assert!(
                !contains(outside, &corridor),
                "unrelated point {outside:?} entered {corridor:?}"
            );
            assert!(contains(
                Point {
                    x: target.x + target.width / 2.,
                    y: target.y + target.height / 2.
                },
                &corridor
            ));
        }
    }

    #[test]
    fn exit_padding_extends_five_pixels_back_into_the_source() {
        let corridor = polygon(
            Point { x: 20., y: 10. },
            Rect {
                x: 0.,
                y: 0.,
                width: 20.,
                height: 20.,
            },
            Rect {
                x: 60.,
                y: 0.,
                width: 20.,
                height: 20.,
            },
        );
        assert!(contains(Point { x: 16., y: 10. }, &corridor));
        assert!(!contains(Point { x: 14., y: 10. }, &corridor));
    }

    #[test]
    fn source_exit_side_controls_padding_when_the_target_is_behind_it() {
        let corridor = polygon(
            Point { x: 20., y: 10. },
            Rect {
                x: 0.,
                y: 0.,
                width: 20.,
                height: 20.,
            },
            Rect {
                x: -60.,
                y: 0.,
                width: 20.,
                height: 20.,
            },
        );
        assert!(contains(Point { x: 14., y: 10. }, &corridor));
        assert!(!contains(Point { x: 16., y: 10. }, &corridor));
    }

    #[test]
    fn empty_and_collinear_corridors_have_no_interior() {
        let point = Point { x: 10., y: 10. };
        for corridor in [
            vec![],
            vec![point],
            vec![Point { x: 0., y: 0. }, point, Point { x: 20., y: 20. }],
        ] {
            assert!(!contains(point, &corridor));
        }
    }

    #[test]
    fn a_zero_sized_target_keeps_the_approach_corridor_finite() {
        let corridor = polygon(
            Point { x: 20., y: 10. },
            Rect {
                x: 0.,
                y: 0.,
                width: 20.,
                height: 20.,
            },
            Rect {
                x: 60.,
                y: 10.,
                width: 0.,
                height: 0.,
            },
        );
        assert!(contains(Point { x: 40., y: 10. }, &corridor));
        assert!(!contains(Point { x: 40., y: 15. }, &corridor));
    }
}
