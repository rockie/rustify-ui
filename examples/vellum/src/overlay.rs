//! Canvas overlays use screen-space strokes above the GPU scene.

use std::f64::consts::{FRAC_PI_2, TAU};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::affine::{identity, point, Matrix, Point};
use crate::document::Document;
use crate::gesture::{path_points, Camera, PathPoint};
use crate::hit::{handles_for, path_matrix, selection_geometry, Axis, Handle, SelectionGeometry};
use crate::pointer::InputState;
use crate::scene::Frame;

pub struct OverlayOptions<'a> {
    pub selection: &'a [String],
    pub input: &'a InputState,
    pub tool: &'a str,
    pub dark: bool,
    pub rulers: bool,
}

/// Paints frame labels, interaction feedback, path controls, and rulers.
pub fn draw_overlay(
    canvas: &HtmlCanvasElement,
    doc: &Document,
    frame: &Frame,
    camera: Camera,
    viewport: (f64, f64, f64),
    options: OverlayOptions<'_>,
) -> Result<(), JsValue> {
    let (width, height, dpr) = viewport;
    let dpr = dpr.min(3.0);
    let pixels_w = (width * dpr).round().max(0.0) as u32;
    let pixels_h = (height * dpr).round().max(0.0) as u32;
    if canvas.width() != pixels_w {
        canvas.set_width(pixels_w);
    }
    if canvas.height() != pixels_h {
        canvas.set_height(pixels_h);
    }
    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("Canvas 2D unavailable"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
    // Not clearRect: when a full-canvas clear is a frame's only operation (the
    // overlay once nothing is selected), Chromium's accelerated canvas can
    // present a recycled buffer that still holds an earlier frame. A "copy"
    // fill is a recorded draw that replaces every pixel.
    ctx.set_global_composite_operation("copy")?;
    ctx.set_fill_style_str("transparent");
    ctx.fill_rect(0.0, 0.0, f64::from(pixels_w), f64::from(pixels_h));
    ctx.set_global_composite_operation("source-over")?;
    ctx.scale(dpr, dpr)?;

    let accent = if options.dark { "#ad96ff" } else { "#8a64e8" };
    let controls = if options.dark { "#242426" } else { "#ffffff" };
    let input = options.input;
    let paint = OverlayPaint {
        ctx: &ctx,
        camera,
        accent,
        controls,
    };

    for item in &frame.items {
        if item.node.kind != "frame"
            || item
                .node
                .parent_id
                .as_deref()
                .is_some_and(|id| !id.is_empty())
        {
            continue;
        }
        let p = camera.world_to_screen(Point::new(item.bounds.x, item.bounds.y));
        if p.y < -30.0 || p.x > width || p.x + item.bounds.w * camera.zoom < 0.0 {
            continue;
        }
        ctx.set_font("10px Inter, -apple-system, BlinkMacSystemFont, sans-serif");
        ctx.set_fill_style_str(if options.selection.contains(&item.node.id) {
            accent
        } else if options.dark {
            "#8d839a"
        } else {
            "#8c8297"
        });
        ctx.fill_text(&item.node.name, p.x, p.y - 12.0)?;
    }

    if let Some(item) = input
        .hover
        .as_deref()
        .filter(|id| {
            !options
                .selection
                .iter()
                .any(|selected| selected.as_str() == *id)
                && options.tool == "select"
                && input.gesture.is_none()
                && input.editing.is_none()
        })
        .and_then(|id| frame.world(id))
    {
        paint.outline(
            SelectionGeometry {
                matrix: item.matrix,
                w: item.node.w,
                h: item.node.h,
            },
            "#a18ae7",
            0.8,
        );
    }

    let roots: Vec<_> = doc
        .roots(options.selection)
        .iter()
        .map(|node| node.id.clone())
        .collect();
    if let Some(geometry) = selection_geometry(frame, &roots, options.selection)
        .filter(|_| input.editing.is_none() && input.path_edit.is_none())
    {
        let corners = paint.outline(geometry, accent, 1.2);
        let gesture = input.gesture.as_ref().map(|gesture| gesture.kind());
        if !matches!(gesture, Some("move" | "marquee")) {
            let handles = handles_for(geometry, camera);
            ctx.set_stroke_style_str(accent);
            ctx.set_fill_style_str(controls);
            let top = handles.iter().find(|handle| handle.name == Handle::N);
            let rotate = handles.iter().find(|handle| handle.name == Handle::Rotate);
            if let (Some(top), Some(rotate)) = (top, rotate) {
                ctx.begin_path();
                ctx.move_to(top.position.x, top.position.y);
                ctx.line_to(rotate.position.x, rotate.position.y);
                ctx.stroke();
            }
            for handle in handles {
                let p = handle.position;
                if handle.name == Handle::Rotate {
                    ctx.begin_path();
                    ctx.arc(p.x, p.y, 3.0, 0.0, TAU)?;
                    ctx.fill();
                    ctx.stroke();
                } else {
                    ctx.fill_rect(p.x - 3.0, p.y - 3.0, 6.0, 6.0);
                    ctx.stroke_rect(p.x - 3.0, p.y - 3.0, 6.0, 6.0);
                }
            }
        }
        let label = format!(
            "{} × {}",
            rounded_label(geometry.w, 100.0),
            rounded_label(geometry.h, 100.0)
        );
        let p = Point::new(
            (corners[2].x + corners[3].x) / 2.0,
            corners[2].y.max(corners[3].y) + 15.0,
        );
        ctx.set_font("9px Inter, sans-serif");
        let text_width = ctx.measure_text(&label)?.width();
        ctx.set_fill_style_str(accent);
        ctx.begin_path();
        ctx.round_rect_with_f64(
            p.x - text_width / 2.0 - 7.0,
            p.y - 5.0,
            text_width + 14.0,
            18.0,
            4.0,
        )?;
        ctx.fill();
        ctx.set_fill_style_str("#ffffff");
        ctx.fill_text(&label, p.x - text_width / 2.0, p.y + 7.0)?;
    }

    if let Some(marquee) = input.marquee {
        ctx.set_stroke_style_str(accent);
        ctx.set_fill_style_str("#a38bff16");
        ctx.set_line_width(1.0);
        ctx.fill_rect(marquee.x, marquee.y, marquee.w, marquee.h);
        ctx.stroke_rect(marquee.x + 0.5, marquee.y + 0.5, marquee.w, marquee.h);
    }

    ctx.set_stroke_style_str("#e77aa0");
    ctx.set_line_width(0.8);
    ctx.set_line_dash(&js_sys::Array::of2(&4.0.into(), &3.0.into()))?;
    for guide in &input.guides {
        ctx.begin_path();
        match guide.axis {
            Axis::X => {
                let x = camera.world_to_screen(Point::new(guide.value, 0.0)).x;
                ctx.move_to(x, 40.0);
                ctx.line_to(x, height - 85.0);
            }
            Axis::Y => {
                let y = camera.world_to_screen(Point::new(0.0, guide.value)).y;
                ctx.move_to(0.0, y);
                ctx.line_to(width, y);
            }
        }
        ctx.stroke();
    }
    ctx.set_line_dash(&js_sys::Array::new())?;
    if !input.pen.is_empty() {
        paint.draw_pen(&input.pen, identity(), input.pen_hover, false)?;
    }
    if let Some((node, item)) = input
        .path_edit
        .as_deref()
        .and_then(|id| doc.get(id))
        .and_then(|node| frame.world(&node.id).map(|item| (node, item)))
    {
        paint.draw_pen(
            &path_points(node),
            path_matrix(node, item.matrix),
            None,
            node.flag("closed"),
        )?;
    }
    if options.rulers {
        paint.draw_rulers(width, height, options.dark)?;
    }
    Ok(())
}

struct OverlayPaint<'a> {
    ctx: &'a CanvasRenderingContext2d,
    camera: Camera,
    accent: &'static str,
    controls: &'static str,
}

impl OverlayPaint<'_> {
    fn transform(&self, matrix: Matrix, p: Point) -> Point {
        self.camera.world_to_screen(point(matrix, p.x, p.y))
    }

    fn outline(&self, geometry: SelectionGeometry, color: &str, width: f64) -> [Point; 4] {
        let corners = [
            Point::new(0.0, 0.0),
            Point::new(geometry.w, 0.0),
            Point::new(geometry.w, geometry.h),
            Point::new(0.0, geometry.h),
        ]
        .map(|p| self.transform(geometry.matrix, p));
        self.ctx.set_stroke_style_str(color);
        self.ctx.set_line_width(width);
        self.ctx.begin_path();
        self.ctx.move_to(corners[0].x, corners[0].y);
        for corner in &corners[1..] {
            self.ctx.line_to(corner.x, corner.y);
        }
        self.ctx.close_path();
        self.ctx.stroke();
        corners
    }

    fn draw_pen(
        &self,
        points: &[PathPoint],
        matrix: Matrix,
        hover: Option<Point>,
        closed: bool,
    ) -> Result<(), JsValue> {
        let Some(first) = points.first() else {
            return Ok(());
        };
        let ctx = self.ctx;
        ctx.set_stroke_style_str(self.accent);
        ctx.set_fill_style_str(self.controls);
        ctx.set_line_width(1.3);
        ctx.begin_path();
        let first = self.transform(matrix, Point::new(first.x, first.y));
        ctx.move_to(first.x, first.y);
        for pair in points.windows(2) {
            let (a, b) = (&pair[0], &pair[1]);
            let p = self.transform(matrix, Point::new(b.x, b.y));
            if a.out_handle.is_some() || b.in_handle.is_some() {
                let c1 = self.transform(matrix, a.out_handle.unwrap_or(Point::new(a.x, a.y)));
                let c2 = self.transform(matrix, b.in_handle.unwrap_or(Point::new(b.x, b.y)));
                ctx.bezier_curve_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y);
            } else {
                ctx.line_to(p.x, p.y);
            }
        }
        if let Some(hover) = hover {
            let p = self.transform(matrix, hover);
            ctx.line_to(p.x, p.y);
        }
        if closed {
            ctx.close_path();
        }
        ctx.stroke();
        for anchor in points {
            let p = self.transform(matrix, Point::new(anchor.x, anchor.y));
            for handle in [anchor.in_handle, anchor.out_handle].into_iter().flatten() {
                let h = self.transform(matrix, handle);
                ctx.begin_path();
                ctx.move_to(p.x, p.y);
                ctx.line_to(h.x, h.y);
                ctx.stroke();
                ctx.begin_path();
                ctx.arc(h.x, h.y, 3.0, 0.0, TAU)?;
                ctx.fill();
                ctx.stroke();
            }
            ctx.fill_rect(p.x - 3.0, p.y - 3.0, 6.0, 6.0);
            ctx.stroke_rect(p.x - 3.0, p.y - 3.0, 6.0, 6.0);
        }
        Ok(())
    }

    fn draw_rulers(&self, width: f64, height: f64, dark: bool) -> Result<(), JsValue> {
        let ctx = self.ctx;
        let zoom = self.camera.zoom;
        let mut step = 100.0;
        while step * zoom < 45.0 {
            step *= 2.0;
        }
        while step * zoom > 160.0 {
            step /= 2.0;
        }
        ctx.set_fill_style_str(if dark { "#242426ee" } else { "#fffffff0" });
        ctx.fill_rect(0.0, 0.0, width, 17.0);
        ctx.fill_rect(0.0, 0.0, 17.0, height);
        ctx.set_stroke_style_str(if dark { "#5a5364" } else { "#c4bccd" });
        ctx.set_fill_style_str(if dark { "#8d839a" } else { "#8c8297" });
        ctx.set_font("8px sans-serif");

        let mut x = (-self.camera.x / zoom / step).floor() * step;
        while x < (width - self.camera.x) / zoom {
            let screen_x = x * zoom + self.camera.x;
            ctx.begin_path();
            ctx.move_to(screen_x, 12.0);
            ctx.line_to(screen_x, 17.0);
            ctx.stroke();
            ctx.fill_text(&rounded_label(x, 10.0), screen_x + 3.0, 10.0)?;
            x += step;
        }
        let mut y = (-self.camera.y / zoom / step).floor() * step;
        while y < (height - self.camera.y) / zoom {
            let screen_y = y * zoom + self.camera.y;
            ctx.begin_path();
            ctx.move_to(12.0, screen_y);
            ctx.line_to(17.0, screen_y);
            ctx.stroke();
            ctx.save();
            let label = (|| {
                ctx.translate(9.0, screen_y - 3.0)?;
                ctx.rotate(-FRAC_PI_2)?;
                ctx.fill_text(&rounded_label(y, 10.0), 0.0, 0.0)
            })();
            ctx.restore();
            label?;
            y += step;
        }
        Ok(())
    }
}

fn rounded_label(value: f64, precision: f64) -> String {
    // JavaScript rounds ties toward positive infinity and prints negative zero as 0.
    let value = js_sys::Math::round(value * precision) / precision;
    if value == 0.0 {
        "0".into()
    } else {
        value.to_string()
    }
}
