use crate::{
    affine::Matrix,
    document::{Document, Node},
    scene::compose,
    text_layout::{layout_text, Measure},
};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NothingToExport;

impl std::fmt::Display for NothingToExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Nothing to export.")
    }
}

impl std::error::Error for NothingToExport {}

pub fn export_svg(
    document: &Document,
    ids: &[String],
    measure: &(impl Measure + ?Sized),
) -> Result<String, NothingToExport> {
    let roots = document.roots(ids);
    let root_ids: Vec<_> = roots.iter().map(|node| node.id.clone()).collect();
    let frame = compose(document);
    let bounds = frame.bounds(Some(&root_ids)).ok_or(NothingToExport)?;
    let mut included = HashSet::new();
    for root in roots {
        included.insert(root.id.as_str());
        included.extend(
            document
                .descendants(&root.id)
                .iter()
                .map(|node| node.id.as_str()),
        );
    }
    let mut defs = String::new();
    let mut body = String::new();
    for (index, scene) in frame
        .items
        .iter()
        .filter(|item| included.contains(item.node.id.as_str()) && item.node.kind != "group")
        .enumerate()
    {
        let node = &scene.node;
        let prefix = format!("v{index}");
        let mut fill = or_string(&node.fill, "none").to_owned();
        if node.fill_type == "linear" && node.fill != "none" {
            let radians = node.gradient_angle.to_radians();
            let c = radians.cos();
            let s = radians.sin();
            let length = (node.w * c).abs() + (node.h * s).abs();
            defs.push_str(&format!("<linearGradient id=\"{prefix}g\" gradientUnits=\"userSpaceOnUse\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"><stop stop-color=\"{}\"/><stop offset=\"1\" stop-color=\"{}\"/></linearGradient>", number(node.w / 2.0 - c * length / 2.0), number(node.h / 2.0 - s * length / 2.0), number(node.w / 2.0 + c * length / 2.0), number(node.h / 2.0 + s * length / 2.0), escape(&node.fill), escape(&node.fill2)));
            fill = format!("url(#{prefix}g)");
        }
        let mut attributes = format!("fill=\"{}\" fill-opacity=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"", escape(&fill), number(node.fill_opacity), escape(or_string(&node.stroke, "none")), number(node.stroke_width));
        if node.shadow {
            defs.push_str(&format!("<filter id=\"{prefix}s\" x=\"-100%\" y=\"-100%\" width=\"300%\" height=\"300%\" color-interpolation-filters=\"sRGB\"><feDropShadow dx=\"{}\" dy=\"{}\" stdDeviation=\"{}\" flood-color=\"{}\" flood-opacity=\"{}\"/></filter>", number(node.shadow_x), number(node.shadow_y), number(or_number(node.shadow_blur, 20.0) / 2.0), escape(or_string(&node.shadow_color, "#000000")), number(node.shadow_opacity)));
            attributes.push_str(&format!(" filter=\"url(#{prefix}s)\""));
        }
        let item = match node.kind.as_str() {
            "text" => text_element(node, &prefix, &fill, &mut defs, measure),
            "image" => image_element(document, node, &prefix, &mut defs),
            "ellipse" => format!(
                "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {attributes}/>",
                number(node.w / 2.0),
                number(node.h / 2.0),
                number(node.w / 2.0),
                number(node.h / 2.0)
            ),
            "path" | "line" => format!("<path d=\"{}\" {attributes}/>", path_data(node)),
            _ => format!(
                "<rect width=\"{}\" height=\"{}\" rx=\"{}\" {attributes}/>",
                number(node.w),
                number(node.h),
                number(node.radius.min(node.w / 2.0).min(node.h / 2.0))
            ),
        };
        let mut wrapped = format!(
            "<g transform=\"matrix({})\" opacity=\"{}\"><title>{}</title>{item}</g>",
            matrix(scene.matrix),
            number(scene.opacity),
            escape(&node.name)
        );
        for (clip_index, clip) in scene.clips.iter().enumerate() {
            defs.push_str(&format!("<clipPath id=\"{prefix}c{clip_index}\" clipPathUnits=\"userSpaceOnUse\"><rect width=\"{}\" height=\"{}\" rx=\"{}\" transform=\"matrix({})\"/></clipPath>", number(clip.w), number(clip.h), number(clip.radius), matrix(clip.matrix)));
            wrapped = format!("<g clip-path=\"url(#{prefix}c{clip_index})\">{wrapped}</g>");
        }
        body.push_str(&wrapped);
    }
    Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\"><title>{}</title><desc>Created with Vellum. Text uses installed fonts, which are not embedded.</desc><defs>{defs}</defs>{body}</svg>", number(bounds.w), number(bounds.h), number(bounds.x), number(bounds.y), number(bounds.w), number(bounds.h), escape(&document.data.name)))
}

pub fn export_tokens(document: &Document) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&document.data.tokens)
}

pub fn path_data(node: &Node) -> String {
    if node.kind == "line" {
        return format!("M0 0L{} {}", number(node.w), number(node.h));
    }
    let Some(points) = node
        .extra
        .get("points")
        .and_then(serde_json::Value::as_array)
    else {
        return String::new();
    };
    let Some(first) = points.first() else {
        return String::new();
    };
    let sx = node.w
        / or_number(
            node.extra
                .get("pathW")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0),
            or_number(node.w, 1.0),
        );
    let sy = node.h
        / or_number(
            node.extra
                .get("pathH")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0),
            or_number(node.h, 1.0),
        );
    let xy = |point: &serde_json::Value| {
        format!(
            "{} {}",
            number(point["x"].as_f64().unwrap_or(0.0) * sx),
            number(point["y"].as_f64().unwrap_or(0.0) * sy)
        )
    };
    let segment = |a: &serde_json::Value, b: &serde_json::Value| {
        let outgoing = a.get("out").filter(|value| value.is_object());
        let incoming = b.get("in").filter(|value| value.is_object());
        if outgoing.is_some() || incoming.is_some() {
            format!(
                "C{} {} {}",
                xy(outgoing.unwrap_or(a)),
                xy(incoming.unwrap_or(b)),
                xy(b)
            )
        } else {
            format!("L{}", xy(b))
        }
    };
    let mut path = format!("M{}", xy(first));
    for pair in points.windows(2) {
        path.push_str(&segment(&pair[0], &pair[1]));
    }
    if node
        .extra
        .get("closed")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        if let Some(last) = points.last() {
            if last.get("out").is_some_and(|value| value.is_object())
                || first.get("in").is_some_and(|value| value.is_object())
            {
                path.push_str(&segment(last, first));
            }
        }
        path.push('Z');
    }
    path
}

fn text_element(
    node: &Node,
    prefix: &str,
    fill: &str,
    defs: &mut String,
    measure: &(impl Measure + ?Sized),
) -> String {
    let layout = layout_text(node, measure);
    let (anchor, x) = match node.text_align.as_str() {
        "center" => ("middle", node.w / 2.0),
        "right" => ("end", node.w),
        _ => ("start", 0.0),
    };
    defs.push_str(&format!(
        "<clipPath id=\"{prefix}t\"><rect width=\"{}\" height=\"{}\"/></clipPath>",
        number(node.w),
        number(node.h)
    ));
    let mut text = format!("<text clip-path=\"url(#{prefix}t)\" font-family=\"{}, sans-serif\" font-size=\"{}\" font-weight=\"{}\" font-style=\"{}\" letter-spacing=\"{}\" text-anchor=\"{anchor}\" text-decoration=\"{}\" direction=\"{}\" fill=\"{}\" fill-opacity=\"{}\" xml:space=\"preserve\">", escape(&node.font_family), number(node.font_size), node.font_weight, escape(&node.font_style), number(node.letter_spacing), escape(or_string(&node.text_decoration, "none")), if node.direction == "rtl" { "rtl" } else { "ltr" }, escape(fill), number(node.fill_opacity));
    for (index, line) in layout.lines.iter().enumerate() {
        text.push_str(&format!(
            "<tspan x=\"{}\" y=\"{}\">{}</tspan>",
            number(x),
            number(layout.baseline + index as f64 * layout.line_height),
            escape(line)
        ));
    }
    text.push_str("</text>");
    text
}

fn image_element(document: &Document, node: &Node, prefix: &str, defs: &mut String) -> String {
    defs.push_str(&format!(
        "<clipPath id=\"{prefix}i\"><rect width=\"{}\" height=\"{}\" rx=\"{}\"/></clipPath>",
        number(node.w),
        number(node.h),
        number(node.radius)
    ));
    let asset = node
        .extra
        .get("assetId")
        .and_then(serde_json::Value::as_str)
        .and_then(|id| document.data.assets.get(id))
        .map_or("", |value| value.as_ref());
    let mut image = format!("<image href=\"{}\" width=\"{}\" height=\"{}\" preserveAspectRatio=\"xMidYMid slice\" clip-path=\"url(#{prefix}i)\"/>", escape(asset), number(node.w), number(node.h));
    if node.stroke_width != 0.0 {
        image.push_str(&format!("<rect width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>", number(node.w), number(node.h), number(node.radius), escape(&node.stroke), number(node.stroke_width)));
    }
    image
}

fn number(value: f64) -> String {
    let rounded = (value * 10000.0 + 0.5).floor() / 10000.0;
    if rounded == 0.0 {
        "0".into()
    } else {
        rounded.to_string()
    }
}

fn matrix(matrix: Matrix) -> String {
    matrix
        .0
        .into_iter()
        .map(number)
        .collect::<Vec<_>>()
        .join(" ")
}

fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn or_string<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() {
        fallback
    } else {
        value
    }
}

fn or_number(value: f64, fallback: f64) -> f64 {
    if value == 0.0 {
        fallback
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct FixedWidth;
    impl Measure for FixedWidth {
        fn width(&self, _: &Node, text: &str) -> f64 {
            text.chars().count() as f64 * 10.0
        }
    }

    fn document(nodes: Vec<Node>) -> (Document, Vec<String>) {
        let mut document = Document::empty("Artwork & <draft>");
        let ids = nodes.iter().map(|node| node.id.clone()).collect();
        for node in nodes {
            document.add(node);
        }
        (document, ids)
    }

    fn node(kind: &str, id: &str) -> Node {
        let mut node = Node::new(kind);
        node.id = id.into();
        node.w = 100.0;
        node.h = 40.0;
        node
    }

    #[test]
    fn rejects_empty_selection() {
        assert_eq!(
            export_svg(&Document::empty("Empty"), &[], &FixedWidth),
            Err(NothingToExport)
        );
    }

    #[test]
    fn exports_rectangles_ellipses_and_lines_in_paint_order() {
        let mut rectangle = node("rect", "rect");
        rectangle.radius = 60.0;
        let (document, ids) = document(vec![
            rectangle,
            node("ellipse", "ellipse"),
            node("line", "line"),
        ]);
        let svg = export_svg(&document, &ids, &FixedWidth).unwrap();
        assert!(svg.contains("<rect width=\"100\" height=\"40\" rx=\"20\""));
        assert!(svg.contains("<ellipse cx=\"50\" cy=\"20\" rx=\"50\" ry=\"20\""));
        assert!(svg.contains("<path d=\"M0 0L100 40\""));
        assert!(svg.find("<title>Rect").unwrap() < svg.find("<title>Ellipse").unwrap());
        assert!(svg.contains("<title>Artwork &amp; &lt;draft&gt;</title>"));
    }

    #[test]
    fn exports_scaled_open_and_closed_bezier_paths() {
        let mut path = node("path", "path");
        path.extra.insert(
            "points".into(),
            json!([
                {"x":0,"y":0,"in":{"x":0,"y":10},"out":{"x":10,"y":0}},
                {"x":50,"y":20,"in":{"x":40,"y":20},"out":{"x":50,"y":10}}
            ]),
        );
        path.extra.insert("pathW".into(), json!(50));
        path.extra.insert("pathH".into(), json!(20));
        assert_eq!(path_data(&path), "M0 0C20 0 80 40 100 40");
        path.extra.insert("closed".into(), json!(true));
        assert_eq!(path_data(&path), "M0 0C20 0 80 40 100 40C100 20 0 20 0 0Z");
    }

    #[test]
    fn path_defaults_to_its_own_dimensions_and_empty_path_stays_empty() {
        let mut path = node("path", "path");
        assert_eq!(path_data(&path), "");
        path.extra.insert(
            "points".into(),
            json!([{"x":1.123456,"y":-0.00005},{"x":10,"y":20}]),
        );
        assert_eq!(path_data(&path), "M1.1235 0L10 20");
    }

    #[test]
    fn ignores_falsy_optional_handles_accepted_by_the_file_format() {
        let mut path = node("path", "path");
        path.extra.insert(
            "points".into(),
            json!([
                {"x":0,"y":0,"out":false},
                {"x":10,"y":20,"in":0,"out":""}
            ]),
        );
        path.extra.insert("closed".into(), json!(true));
        assert_eq!(path_data(&path), "M0 0L10 20Z");
    }

    #[test]
    fn exports_linear_gradient_and_drop_shadow_with_original_geometry() {
        let mut rectangle = node("rect", "rect");
        rectangle.fill_type = "linear".into();
        rectangle.gradient_angle = 0.0;
        rectangle.fill = "#112233".into();
        rectangle.fill2 = "#445566".into();
        rectangle.shadow = true;
        let (document, ids) = document(vec![rectangle]);
        let svg = export_svg(&document, &ids, &FixedWidth).unwrap();
        assert!(svg.contains("<linearGradient id=\"v0g\" gradientUnits=\"userSpaceOnUse\" x1=\"0\" y1=\"20\" x2=\"100\" y2=\"20\"><stop stop-color=\"#112233\"/><stop offset=\"1\" stop-color=\"#445566\"/></linearGradient>"));
        assert!(svg.contains("<feDropShadow dx=\"0\" dy=\"6\" stdDeviation=\"10\" flood-color=\"#000000\" flood-opacity=\"0.16\"/>"));
        assert!(svg.contains("fill=\"url(#v0g)\""));
        assert!(svg.contains("filter=\"url(#v0s)\""));
    }

    #[test]
    fn uses_shared_text_wrapping_alignment_and_escaped_font_properties() {
        let mut label = node("text", "label");
        label.text = "a<&  bc".into();
        label.w = 40.0;
        label.h = 80.0;
        label.font_size = 20.0;
        label.line_height = 1.5;
        label.font_family = "A\"B'&".into();
        label.text_align = "center".into();
        label.direction = "rtl".into();
        label.text_decoration = "underline".into();
        let (document, ids) = document(vec![label]);
        let svg = export_svg(&document, &ids, &FixedWidth).unwrap();
        assert!(svg.contains("font-family=\"A&quot;B&#39;&amp;, sans-serif\""));
        assert!(
            svg.contains("text-anchor=\"middle\" text-decoration=\"underline\" direction=\"rtl\"")
        );
        assert!(svg.contains(
            "<tspan x=\"20\" y=\"21.4\">a&lt;&amp;</tspan><tspan x=\"20\" y=\"51.4\">bc</tspan>"
        ));
        assert!(svg.contains("<clipPath id=\"v0t\"><rect width=\"40\" height=\"80\"/></clipPath>"));
    }

    #[test]
    fn embeds_images_with_cover_clipping_and_optional_outline() {
        let mut image = node("image", "image");
        image.radius = 8.0;
        image.stroke_width = 2.0;
        image.extra.insert("assetId".into(), json!("asset"));
        let (mut document, ids) = document(vec![image]);
        document
            .data
            .assets
            .insert("asset".into(), "data:image/png;base64,AA==".into());
        let svg = export_svg(&document, &ids, &FixedWidth).unwrap();
        assert!(svg.contains("<image href=\"data:image/png;base64,AA==\" width=\"100\" height=\"40\" preserveAspectRatio=\"xMidYMid slice\" clip-path=\"url(#v0i)\"/>"));
        assert!(svg.contains("<rect width=\"100\" height=\"40\" rx=\"8\" fill=\"none\" stroke=\"#000000\" stroke-width=\"2\"/>"));
    }

    #[test]
    fn retains_nested_world_clips_inherited_opacity_and_selected_descendants() {
        let mut outer = node("frame", "outer");
        outer.x = 10.0;
        outer.y = 20.0;
        outer.opacity = 0.5;
        let mut inner = node("frame", "inner");
        inner.parent_id = Some("outer".into());
        inner.x = 5.0;
        inner.opacity = 0.5;
        let mut shape = node("rect", "shape");
        shape.parent_id = Some("inner".into());
        shape.name = "included".into();
        let mut hidden = node("rect", "hidden");
        hidden.visible = false;
        let mut unrelated = node("rect", "unrelated");
        unrelated.name = "excluded".into();
        let (document, _) = document(vec![outer, inner, shape, hidden, unrelated]);
        let svg = export_svg(&document, &["outer".into(), "shape".into()], &FixedWidth).unwrap();
        assert!(svg.contains("<g clip-path=\"url(#v2c1)\"><g clip-path=\"url(#v2c0)\"><g transform=\"matrix(1 0 0 1 15 20)\" opacity=\"0.25\"><title>included</title>"));
        assert!(!svg.contains("excluded"));
        assert!(!svg.contains("hidden"));
        assert!(svg.contains("viewBox=\"10 20 100 40\""));
    }

    #[test]
    fn exports_group_descendants_without_a_group_shape() {
        let group = node("group", "group");
        let mut shape = node("rect", "shape");
        shape.parent_id = Some("group".into());
        let (document, _) = document(vec![group, shape]);
        let svg = export_svg(&document, &["group".into()], &FixedWidth).unwrap();
        assert!(!svg.contains("<title>Group</title>"));
        assert!(svg.contains("<title>Rect</title>"));
    }

    #[test]
    fn token_export_preserves_color_typography_and_extension_values() {
        let mut document = Document::empty("Tokens");
        document.data.tokens = json!({"colors":[{"name":"Accent","value":"#aaffcc"}],"typography":[{"fontSize":24}],"extension":{"stable":true}});
        let exported: serde_json::Value =
            serde_json::from_str(&export_tokens(&document).unwrap()).unwrap();
        assert_eq!(exported, document.data.tokens);
    }
}
