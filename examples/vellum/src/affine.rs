use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::document::Node;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(self, other: Self) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.x + other.w <= self.x + self.w
            && other.y + other.h <= self.y + self.h
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Matrix(pub [f64; 6]);

impl Default for Matrix {
    fn default() -> Self {
        identity()
    }
}

impl Index<usize> for Matrix {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub const fn identity() -> Matrix {
    Matrix([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
}

pub fn multiply(a: Matrix, b: Matrix) -> Matrix {
    Matrix([
        a[0] * b[0] + a[2] * b[1],
        a[1] * b[0] + a[3] * b[1],
        a[0] * b[2] + a[2] * b[3],
        a[1] * b[2] + a[3] * b[3],
        a[0] * b[4] + a[2] * b[5] + a[4],
        a[1] * b[4] + a[3] * b[5] + a[5],
    ])
}

pub fn inverse(m: Matrix) -> Matrix {
    let d = m[0] * m[3] - m[1] * m[2];
    if d.abs() < 1e-12 {
        return identity();
    }
    Matrix([
        m[3] / d,
        -m[1] / d,
        -m[2] / d,
        m[0] / d,
        (m[2] * m[5] - m[3] * m[4]) / d,
        (m[1] * m[4] - m[0] * m[5]) / d,
    ])
}

pub fn point(m: Matrix, x: f64, y: f64) -> Point {
    Point {
        x: m[0] * x + m[2] * y + m[4],
        y: m[1] * x + m[3] * y + m[5],
    }
}

pub fn local_matrix(n: &Node) -> Matrix {
    centered(n.x, n.y, n.w, n.h, n.rotation)
}

pub fn centered(x: f64, y: f64, w: f64, h: f64, rotation: f64) -> Matrix {
    let r = rotation.to_radians();
    let (s, c) = r.sin_cos();
    let cx = w / 2.0;
    let cy = h / 2.0;
    Matrix([
        c,
        s,
        -s,
        c,
        x + cx - c * cx + s * cy,
        y + cy - s * cx - c * cy,
    ])
}

pub fn box_of(m: Matrix, w: f64, h: f64) -> Rect {
    let points = [
        point(m, 0.0, 0.0),
        point(m, w, 0.0),
        point(m, w, h),
        point(m, 0.0, h),
    ];
    let x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let right = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let bottom = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    Rect {
        x,
        y,
        w: right - x,
        h: bottom - y,
    }
}

pub fn union(boxes: impl IntoIterator<Item = Rect>) -> Option<Rect> {
    let mut boxes = boxes.into_iter();
    let first = boxes.next()?;
    Some(boxes.fold(first, |a, b| {
        let x = a.x.min(b.x);
        let y = a.y.min(b.y);
        Rect {
            x,
            y,
            w: (a.x + a.w).max(b.x + b.w) - x,
            h: (a.y + a.h).max(b.y + b.h) - y,
        }
    }))
}

pub fn intersects(a: Rect, b: Rect) -> bool {
    a.x <= b.x + b.w && a.x + a.w >= b.x && a.y <= b.y + b.h && a.y + a.h >= b.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_keeps_the_center_fixed() {
        let m = centered(10.0, 20.0, 100.0, 40.0, 90.0);
        let center = point(m, 50.0, 20.0);
        assert!((center.x - 60.0).abs() < 1e-10 && (center.y - 40.0).abs() < 1e-10);
    }

    #[test]
    fn inverse_restores_a_point_after_composition() {
        let m = multiply(
            centered(20.0, 30.0, 60.0, 20.0, 30.0),
            Matrix([2.0, 0.0, 0.0, 3.0, 5.0, 9.0]),
        );
        let p = point(m, 17.0, -3.0);
        let restored = point(inverse(m), p.x, p.y);
        assert!((restored.x - 17.0).abs() < 1e-10 && (restored.y + 3.0).abs() < 1e-10);
    }

    #[test]
    fn singular_inverse_matches_reference_identity() {
        assert_eq!(inverse(Matrix([0.0; 6])), identity());
    }

    #[test]
    fn rotated_bounds_enclose_every_corner() {
        let m = centered(0.0, 0.0, 100.0, 100.0, 30.0);
        let b = box_of(m, 100.0, 100.0);
        assert!(b.w > 100.0 && b.h > 100.0);
    }

    #[test]
    fn empty_bounds_have_no_union() {
        assert_eq!(union([]), None);
    }
}
