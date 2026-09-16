use std::collections::{HashMap, HashSet, VecDeque};

use js_sys::{Array, Intl, JsString, Object, Reflect, RegExp};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Path2d};

use crate::{
    affine::{Matrix, Point},
    assets::ImageCache,
    document::{Document, Node},
    gesture::path_points,
    hit::PathTester,
    raster_policy,
    scene::{compose, Frame, SceneItem},
    text_layout::{character_segments, font_spec, layout_text, Measure, TextLayout},
};

pub fn canvas_context() -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| JsValue::from_str("Browser document is unavailable."))?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    // Texture uploads require RGBA readback. Prefer software Canvas from the start to
    // avoid GPU readback stalls and mid-session backend changes. Fractional-edge
    // antialiasing can differ from the reference's GPU-backed Canvas.
    let options = Object::new();
    Reflect::set(
        options.as_ref(),
        &JsValue::from_str("willReadFrequently"),
        &JsValue::TRUE,
    )?;
    let context = canvas
        .get_context_with_context_options("2d", options.as_ref())?
        .ok_or_else(|| JsValue::from_str("Canvas 2D is unavailable."))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    Ok((canvas, context))
}

struct BrowserText {
    segmenter: Option<Intl::Segmenter>,
    title_words: RegExp,
}

impl BrowserText {
    fn new() -> Self {
        let available = Reflect::get(&js_sys::global(), &JsValue::from_str("Intl"))
            .and_then(|intl| Reflect::get(&intl, &JsValue::from_str("Segmenter")))
            .is_ok_and(|constructor| constructor.is_function());
        let segmenter = available.then(|| {
            let options = Intl::SegmenterOptions::new();
            options.set_granularity(Intl::SegmenterGranularity::Grapheme);
            Intl::Segmenter::new(&Array::new(), options.unchecked_ref::<Object>())
        });
        Self {
            segmenter,
            title_words: RegExp::new(r"\p{L}[\p{L}\p{M}]*", "gu"),
        }
    }

    fn segments<'a>(&self, text: &'a str) -> Option<Vec<&'a str>> {
        let segmenter = self.segmenter.as_ref()?;
        let segments = segmenter.segment(text);
        let iterator = js_sys::try_iter(&segments).ok()??;
        let mut offset = 0;
        let mut result = Vec::new();
        for segment in iterator {
            let segment = segment.ok()?.unchecked_into::<Intl::SegmentData>();
            let length = String::from(segment.segment()).len();
            result.push(text.get(offset..offset + length)?);
            offset += length;
        }
        (offset == text.len()).then_some(result)
    }

    fn display_text(&self, node: &Node) -> String {
        let text = JsString::from(node.text.as_str());
        match node.text_case.as_str() {
            "upper" => text.to_locale_upper_case(None).into(),
            "lower" => text.to_locale_lower_case(None).into(),
            "title" => {
                let replacement =
                    Closure::<dyn FnMut(JsString) -> JsString>::new(|word: JsString| {
                        let first = word.char_at(0).to_locale_upper_case(None);
                        let tail = word.slice(1, word.length()).to_locale_lower_case(None);
                        first.concat(tail.as_ref())
                    });
                text.replace_by_pattern_with_function(
                    &self.title_words,
                    replacement.as_ref().unchecked_ref(),
                )
                .into()
            }
            _ => node.text.clone(),
        }
    }
}

thread_local! {
    static BROWSER_TEXT: BrowserText = BrowserText::new();
}

#[derive(Clone)]
pub struct CanvasMeasure {
    context: CanvasRenderingContext2d,
}

impl CanvasMeasure {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self {
            context: canvas_context()?.1,
        })
    }

    pub fn from_context(context: &CanvasRenderingContext2d) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl Measure for CanvasMeasure {
    fn width(&self, node: &Node, text: &str) -> f64 {
        configure_font(&self.context, node);
        self.context
            .measure_text(text)
            .map_or(0.0, |metrics| metrics.width())
    }

    fn segments<'a>(&self, text: &'a str) -> Vec<&'a str> {
        BROWSER_TEXT
            .with(|browser| browser.segments(text))
            .unwrap_or_else(|| character_segments(text))
    }

    fn display_text(&self, node: &Node) -> String {
        BROWSER_TEXT.with(|browser| browser.display_text(node))
    }
}

impl PathTester for CanvasMeasure {
    fn in_path(&self, node: &Node, p: Point) -> bool {
        path_for(node).is_ok_and(|path| {
            self.context
                .is_point_in_path_with_path_2d_and_f64(&path, p.x, p.y)
        })
    }

    fn in_stroke(&self, node: &Node, p: Point, width: f64) -> bool {
        self.context.set_line_width(width);
        path_for(node).is_ok_and(|path| {
            self.context
                .is_point_in_stroke_with_path_and_x_and_y(&path, p.x, p.y)
        })
    }
}

fn configure_font(context: &CanvasRenderingContext2d, node: &Node) {
    context.set_font(&font_spec(node));
    let _ = Reflect::set(
        context.as_ref(),
        &JsValue::from_str("letterSpacing"),
        &JsValue::from_str(&format!("{}px", node.letter_spacing)),
    );
}

pub fn paint_text(context: &CanvasRenderingContext2d, node: &Node) -> Result<TextLayout, JsValue> {
    let layout = layout_text(node, &CanvasMeasure::from_context(context));
    configure_font(context, node);
    context.set_text_baseline("alphabetic");
    context.set_text_align(if node.text_align.is_empty() {
        "left"
    } else {
        &node.text_align
    });
    let direction = match node.direction.as_str() {
        "ltr" => "ltr",
        "rtl" => "rtl",
        _ => "inherit",
    };
    Reflect::set(
        context.as_ref(),
        &JsValue::from_str("direction"),
        &JsValue::from_str(direction),
    )?;
    apply_fill(context, node)?;
    context.set_global_alpha(context.global_alpha() * node.fill_opacity);
    let x = match node.text_align.as_str() {
        "center" => node.w / 2.0,
        "right" => node.w,
        _ => 0.0,
    };
    for (index, line) in layout.lines.iter().enumerate() {
        let y = layout.baseline + index as f64 * layout.line_height;
        if y - layout.line_height > node.h {
            break;
        }
        context.fill_text(line, x, y)?;
        if matches!(node.text_decoration.as_str(), "underline" | "line-through") {
            let width = context.measure_text(line)?.width();
            let start = x - match node.text_align.as_str() {
                "center" => width / 2.0,
                "right" => width,
                _ => 0.0,
            };
            let offset = node.font_size
                * if node.text_decoration == "underline" {
                    0.12
                } else {
                    -0.3
                };
            context.fill_rect(start, y + offset, width, (node.font_size * 0.055).max(1.0));
        }
    }
    Ok(layout)
}

pub fn path_for(node: &Node) -> Result<Path2d, JsValue> {
    let path = Path2d::new()?;
    match node.kind.as_str() {
        "ellipse" => path.ellipse(
            node.w / 2.0,
            node.h / 2.0,
            (node.w / 2.0).max(0.001),
            (node.h / 2.0).max(0.001),
            0.0,
            0.0,
            std::f64::consts::TAU,
        )?,
        "line" => {
            path.move_to(0.0, 0.0);
            path.line_to(node.w, node.h);
        }
        "path" => {
            let points = path_points(node);
            let Some(first) = points.first() else {
                return Ok(path);
            };
            let sx = node.w / nonzero(node.number("pathW", 0.0), nonzero(node.w, 1.0));
            let sy = node.h / nonzero(node.number("pathH", 0.0), nonzero(node.h, 1.0));
            path.move_to(first.x * sx, first.y * sy);
            let curve = |a: &crate::gesture::PathPoint, b: &crate::gesture::PathPoint| {
                let out = a.out_handle.unwrap_or(Point::new(a.x, a.y));
                let incoming = b.in_handle.unwrap_or(Point::new(b.x, b.y));
                path.bezier_curve_to(
                    out.x * sx,
                    out.y * sy,
                    incoming.x * sx,
                    incoming.y * sy,
                    b.x * sx,
                    b.y * sy,
                );
            };
            for pair in points.windows(2) {
                if pair[0].out_handle.is_some() || pair[1].in_handle.is_some() {
                    curve(&pair[0], &pair[1]);
                } else {
                    path.line_to(pair[1].x * sx, pair[1].y * sy);
                }
            }
            if node.flag("closed") {
                if let Some(last) = points.last() {
                    if last.out_handle.is_some() || first.in_handle.is_some() {
                        curve(last, first);
                    }
                }
                path.close_path();
            }
        }
        _ => path.round_rect_with_f64(
            0.0,
            0.0,
            node.w.max(0.0),
            node.h.max(0.0),
            node.radius.min(node.w / 2.0).min(node.h / 2.0).max(0.0),
        )?,
    }
    Ok(path)
}

pub fn apply_fill(context: &CanvasRenderingContext2d, node: &Node) -> Result<(), JsValue> {
    if node.fill.is_empty() || node.fill == "none" {
        context.set_fill_style_str("transparent");
    } else if node.fill_type != "linear" {
        context.set_fill_style_str(&node.fill);
    } else {
        let angle = node.gradient_angle * std::f64::consts::PI / 180.0;
        let c = angle.cos();
        let s = angle.sin();
        let length = (node.w * c).abs() + (node.h * s).abs();
        let gradient = context.create_linear_gradient(
            node.w / 2.0 - c * length / 2.0,
            node.h / 2.0 - s * length / 2.0,
            node.w / 2.0 + c * length / 2.0,
            node.h / 2.0 + s * length / 2.0,
        );
        gradient.add_color_stop(0.0, &node.fill)?;
        gradient.add_color_stop(1.0, &node.fill2)?;
        context.set_fill_style_canvas_gradient(&gradient);
    }
    Ok(())
}

pub fn image_fit(image: &HtmlImageElement, width: f64, height: f64) -> [f64; 4] {
    let image_width = f64::from(image.width());
    let image_height = f64::from(image.height());
    let scale = (width / image_width).max(height / image_height);
    let draw_width = image_width * scale;
    let draw_height = image_height * scale;
    [
        (width - draw_width) / 2.0,
        (height - draw_height) / 2.0,
        draw_width,
        draw_height,
    ]
}

fn paint_image(
    context: &CanvasRenderingContext2d,
    node: &Node,
    path: &Path2d,
    images: &ImageCache,
) -> Result<(), JsValue> {
    if let Some(image) = images.get(node.string("assetId").unwrap_or("")) {
        context.save();
        context.clip_with_path_2d(path);
        let [x, y, width, height] = image_fit(&image, node.w, node.h);
        let result =
            context.draw_image_with_html_image_element_and_dw_and_dh(&image, x, y, width, height);
        context.restore();
        result?;
    }
    if node.stroke_width != 0.0 {
        context.set_stroke_style_str(stroke_color(node));
        context.set_line_width(node.stroke_width);
        context.stroke_with_path(path);
    }
    Ok(())
}

fn paint_vector(
    context: &CanvasRenderingContext2d,
    node: &Node,
    path: &Path2d,
    shadows: bool,
) -> Result<(), JsValue> {
    let opacity = context.global_alpha();
    if shadows && node.shadow {
        let transform = context.get_transform()?;
        let scale = transform.a().hypot(transform.b());
        context.set_shadow_color(&shadow_color(node));
        context.set_shadow_blur(nonzero(node.shadow_blur, 20.0) * scale);
        context.set_shadow_offset_x(node.shadow_x * scale);
        context.set_shadow_offset_y(node.shadow_y * scale);
    }
    apply_fill(context, node)?;
    context.set_global_alpha(opacity * node.fill_opacity);
    if node.fill != "none" {
        context.fill_with_path_2d(path);
    }
    context.set_shadow_color("transparent");
    context.set_global_alpha(opacity);
    if node.stroke_width != 0.0 {
        context.set_stroke_style_str(stroke_color(node));
        context.set_line_width(node.stroke_width);
        context.set_line_cap("round");
        context.set_line_join("round");
        context.stroke_with_path(path);
    }
    Ok(())
}

/// Paints local content for GPU raster upload; scene transforms and opacity are applied later.
pub fn paint_node(
    context: &CanvasRenderingContext2d,
    node: &Node,
    images: &ImageCache,
) -> Result<(), JsValue> {
    let path = path_for(node)?;
    match node.kind.as_str() {
        "text" => {
            context.begin_path();
            context.rect(0.0, 0.0, node.w, node.h);
            context.clip();
            paint_text(context, node)?;
        }
        "image" => paint_image(context, node, &path, images)?,
        "group" => {}
        _ => paint_vector(context, node, &path, false)?,
    }
    Ok(())
}

struct TextCanvas {
    version: u64,
    canvas: HtmlCanvasElement,
}

#[derive(Default)]
pub struct Painter {
    text: HashMap<String, TextCanvas>,
    order: VecDeque<String>,
}

impl Painter {
    pub fn new() -> Self {
        Self::default()
    }

    fn paint_text_cached(
        &mut self,
        context: &CanvasRenderingContext2d,
        node: &Node,
    ) -> Result<(), JsValue> {
        let transform = context.get_transform()?;
        let desired = raster_policy::resolution(transform.a().hypot(transform.b()), 1.0);
        let scale = desired.min(4096.0 / node.w.max(node.h).max(1.0));
        let key = format!("{}:{scale}", node.id);
        if self
            .text
            .get(&key)
            .is_none_or(|entry| entry.version != node.version)
        {
            let (canvas, text_context) = canvas_context()?;
            canvas.set_width((node.w * scale).ceil().max(1.0) as u32);
            canvas.set_height((node.h * scale).ceil().max(1.0) as u32);
            text_context.scale(scale, scale)?;
            text_context.begin_path();
            text_context.rect(0.0, 0.0, node.w, node.h);
            text_context.clip();
            paint_text(&text_context, node)?;
            if self.text.len() >= 512 {
                if let Some(oldest) = self.order.pop_front() {
                    self.text.remove(&oldest);
                }
            }
            if !self.text.contains_key(&key) {
                self.order.push_back(key.clone());
            }
            self.text.insert(
                key.clone(),
                TextCanvas {
                    version: node.version,
                    canvas,
                },
            );
        }
        if let Some(entry) = self.text.get(&key) {
            context.draw_image_with_html_canvas_element_and_dw_and_dh(
                &entry.canvas,
                0.0,
                0.0,
                node.w,
                node.h,
            )?;
        }
        Ok(())
    }

    fn paint_item(
        &mut self,
        context: &CanvasRenderingContext2d,
        item: &SceneItem,
        images: &ImageCache,
    ) -> Result<(), JsValue> {
        for clip in &item.clips {
            let original = context.get_transform()?;
            transform(context, clip.matrix)?;
            let path = Path2d::new()?;
            path.round_rect_with_f64(
                0.0,
                0.0,
                clip.w,
                clip.h,
                clip.radius.min(clip.w / 2.0).min(clip.h / 2.0),
            )?;
            context.clip_with_path_2d(&path);
            // Restoring only the transform preserves the newly intersected clip.
            context.set_transform(
                original.a(),
                original.b(),
                original.c(),
                original.d(),
                original.e(),
                original.f(),
            )?;
        }
        transform(context, item.matrix)?;
        context.set_global_alpha(item.opacity);
        let node = &item.node;
        match node.kind.as_str() {
            "text" => self.paint_text_cached(context, node),
            "image" => paint_image(context, node, &path_for(node)?, images),
            _ => paint_vector(context, node, &path_for(node)?, true),
        }
    }

    pub fn paint_scene(
        &mut self,
        context: &CanvasRenderingContext2d,
        frame: &Frame,
        images: &ImageCache,
        included: Option<&HashSet<String>>,
        skip: Option<&str>,
    ) -> Result<(), JsValue> {
        for item in &frame.items {
            if item.node.kind == "group"
                || item.hidden
                || skip == Some(item.node.id.as_str())
                || included.is_some_and(|ids| !ids.contains(&item.node.id))
            {
                continue;
            }
            context.save();
            let result = self.paint_item(context, item, images);
            context.restore();
            result?;
        }
        Ok(())
    }

    pub async fn export_canvas(
        &mut self,
        document: &Document,
        ids: &[String],
        scale: f64,
        images: &ImageCache,
    ) -> Result<HtmlCanvasElement, JsValue> {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(JsValue::from_str(
                "Export scale must be a positive finite number.",
            ));
        }
        images.sync(document);
        images.ready().await;
        let frame = compose(document);
        let bounds = frame
            .bounds(Some(ids))
            .ok_or_else(|| JsValue::from_str("Nothing to export."))?;
        if bounds.w * scale > 16_384.0
            || bounds.h * scale > 16_384.0
            || bounds.w * bounds.h * scale * scale > 64e6
        {
            return Err(JsValue::from_str(
                "Export exceeds 64 megapixels or 16,384 pixels per side. Choose a lower scale.",
            ));
        }
        let mut included: HashSet<_> = ids.iter().cloned().collect();
        for id in ids {
            included.extend(
                document
                    .descendants(id)
                    .into_iter()
                    .map(|node| node.id.clone()),
            );
        }
        let (canvas, context) = canvas_context()?;
        canvas.set_width((bounds.w * scale).ceil().max(1.0) as u32);
        canvas.set_height((bounds.h * scale).ceil().max(1.0) as u32);
        context.scale(scale, scale)?;
        context.translate(-bounds.x, -bounds.y)?;
        self.paint_scene(&context, &frame, images, Some(&included), None)?;
        Ok(canvas)
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.order.clear();
    }
}

pub fn paint_scene(
    context: &CanvasRenderingContext2d,
    frame: &Frame,
    images: &ImageCache,
    included: Option<&HashSet<String>>,
) -> Result<(), JsValue> {
    Painter::new().paint_scene(context, frame, images, included, None)
}

fn transform(context: &CanvasRenderingContext2d, matrix: Matrix) -> Result<(), JsValue> {
    let [a, b, c, d, e, f] = matrix.0;
    context.transform(a, b, c, d, e, f)
}

fn nonzero(value: f64, fallback: f64) -> f64 {
    if value == 0.0 {
        fallback
    } else {
        value
    }
}

fn stroke_color(node: &Node) -> &str {
    if node.stroke == "none" {
        "transparent"
    } else {
        &node.stroke
    }
}

fn shadow_color(node: &Node) -> String {
    if node.shadow_color == "none" {
        return "rgba(0,0,0,0)".into();
    }
    let value = if node.shadow_color.is_empty() {
        "#000000"
    } else {
        &node.shadow_color
    };
    let mut hex = value.trim_start_matches('#').to_owned();
    if hex.len() == 3 {
        hex = hex.chars().flat_map(|ch| [ch, ch]).collect();
    }
    let channel = |start| {
        hex.get(start..start + 2)
            .and_then(|part| u8::from_str_radix(part, 16).ok())
            .unwrap_or(0)
    };
    let alpha = if hex.len() == 8 {
        f64::from(channel(6)) / 255.0
    } else {
        1.0
    };
    format!(
        "rgba({},{},{},{})",
        channel(0),
        channel(2),
        channel(4),
        alpha * node.shadow_opacity
    )
}
