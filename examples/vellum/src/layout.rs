use crate::document::Document;

pub fn apply_layout(doc: &mut Document, id: &str) {
    let Some(parent) = doc.get(id) else { return };
    let Some(layout) = parent.string("layout") else {
        return;
    };
    if layout == "none" || layout.is_empty() {
        return;
    }
    let horizontal = layout == "horizontal";
    let pad = parent.number("padding", 16.0);
    let gap = parent.number("gap", 16.0);
    let available = if horizontal { parent.h } else { parent.w } - pad * 2.0;
    let align = parent.string("layoutAlign").unwrap_or("start").to_owned();
    let ids: Vec<_> = doc
        .children(Some(id))
        .into_iter()
        .filter(|n| n.visible)
        .map(|n| n.id.clone())
        .collect();
    let mut pos = pad;
    for child in ids {
        if let Some(n) = doc.get_mut(&child) {
            let size = if horizontal { n.h } else { n.w };
            let cross = pad
                + match align.as_str() {
                    "center" => (available - size) / 2.0,
                    "end" => available - size,
                    _ => 0.0,
                };
            if horizontal {
                n.x = pos;
                n.y = cross;
                pos += n.w + gap;
            } else {
                n.y = pos;
                n.x = cross;
                pos += n.h + gap;
            }
        }
        doc.touch(Some(&child));
    }
}

pub fn apply_all_layouts(doc: &mut Document) {
    let ids: Vec<_> = doc.nodes().iter().rev().map(|n| n.id.clone()).collect();
    for id in ids {
        apply_layout(doc, &id);
    }
}

pub fn constrain_children(doc: &mut Document, id: &str, old_w: f64, old_h: f64) {
    let Some(parent) = doc.get(id) else { return };
    let group = parent.kind == "group";
    if !group && parent.kind != "frame" {
        return;
    }
    let dx = parent.w - old_w;
    let dy = parent.h - old_h;
    let sx = parent.w / if old_w == 0.0 { 1.0 } else { old_w };
    let sy = parent.h / if old_h == 0.0 { 1.0 } else { old_h };
    let ids: Vec<_> = doc
        .children(Some(id))
        .into_iter()
        .map(|n| n.id.clone())
        .collect();
    for child in ids {
        let Some(n) = doc.get_mut(&child) else {
            continue;
        };
        let (cw, ch) = (n.w, n.h);
        if group {
            n.x *= sx;
            n.y *= sy;
            n.w *= sx;
            n.h *= sy;
            if n.kind == "text" {
                n.font_size *= sx.min(sy);
            }
            constrain_children(doc, &child, cw, ch);
        } else {
            match n.string("constraintH") {
                Some("right") => n.x += dx,
                Some("center") => n.x += dx / 2.0,
                Some("stretch") => n.w = (n.w + dx).max(1.0),
                Some("scale") => {
                    n.x *= sx;
                    n.w *= sx;
                }
                _ => {}
            }
            match n.string("constraintV") {
                Some("bottom") => n.y += dy,
                Some("center") => n.y += dy / 2.0,
                Some("stretch") => n.h = (n.h + dy).max(1.0),
                Some("scale") => {
                    n.y *= sy;
                    n.h *= sy;
                }
                _ => {}
            }
        }
        doc.touch(Some(&child));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Node;
    use serde_json::json;

    fn layout_doc(direction: &str) -> Document {
        let mut doc = Document::empty("test");
        let mut parent = Node::new("frame");
        parent.id = "parent".into();
        parent.extra.insert("layout".into(), json!(direction));
        doc.add(parent);
        for id in ["a", "b"] {
            let mut child = Node::new("rect");
            child.id = id.into();
            child.parent_id = Some("parent".into());
            child.w = 84.0;
            child.h = 44.0;
            doc.add(child);
        }
        doc
    }

    #[test]
    fn horizontal_layout_matches_reference_offsets() {
        let mut doc = layout_doc("horizontal");
        apply_all_layouts(&mut doc);
        let a = doc.get("a").unwrap();
        let b = doc.get("b").unwrap();
        assert_eq!((a.x, a.y, b.x, b.y), (16.0, 16.0, 116.0, 16.0));
    }

    #[test]
    fn vertical_layout_matches_reference_offsets() {
        let mut doc = layout_doc("vertical");
        apply_all_layouts(&mut doc);
        let b = doc.get("b").unwrap();
        assert_eq!((b.x, b.y), (16.0, 76.0));
    }

    #[test]
    fn frame_right_constraint_tracks_width_delta() {
        let mut doc = layout_doc("none");
        doc.get_mut("a")
            .unwrap()
            .extra
            .insert("constraintH".into(), json!("right"));
        doc.get_mut("parent").unwrap().w += 100.0;
        constrain_children(&mut doc, "parent", 400.0, 300.0);
        assert_eq!(doc.get("a").unwrap().x, 100.0);
    }

    #[test]
    fn group_resize_scales_descendants_and_font_size() {
        let mut doc = Document::empty("test");
        let mut group = Node::new("group");
        group.id = "g".into();
        group.w = 200.0;
        group.h = 300.0;
        doc.add(group);
        let mut text = Node::new("text");
        text.id = "t".into();
        text.parent_id = Some("g".into());
        text.x = 10.0;
        text.y = 20.0;
        doc.add(text);
        constrain_children(&mut doc, "g", 100.0, 100.0);
        let t = doc.get("t").unwrap();
        assert_eq!(
            (t.x, t.y, t.w, t.h, t.font_size),
            (20.0, 60.0, 480.0, 132.0, 48.0)
        );
    }
}
