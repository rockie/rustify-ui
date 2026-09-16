use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::affine::{box_of, identity, inverse, multiply, union, Matrix, Point};
use crate::document::{Document, DocumentError, Node};
use crate::layout::{apply_layout, constrain_children};
use crate::scene::{compose, Frame};
use crate::text_layout::{layout_text, Measure};

pub fn set_property(
    doc: &mut Document,
    ids: &[String],
    prop: &str,
    value: Value,
    measure: &(impl Measure + ?Sized),
) -> Result<(), serde_json::Error> {
    let Some(value) = property_value(prop, value) else {
        return Ok(());
    };
    for id in ids {
        let Some(node) = doc.get_mut(id) else {
            continue;
        };
        let (old_w, old_h) = (node.w, node.h);
        let mut properties = serde_json::to_value(&*node)?;
        properties[prop] = value.clone();
        let mut next: Node = serde_json::from_value(properties)?;
        if next
            .string("sourceId")
            .is_some_and(|source| !source.is_empty())
        {
            let overrides = next.extra.entry("overrides").or_insert_with(|| json!({}));
            if !overrides.is_object() {
                *overrides = json!({});
            }
            overrides[prop] = value.clone();
        }
        *node = next;
        if matches!(prop, "w" | "h") {
            constrain_children(doc, id, old_w, old_h);
        }
        if let Some(node) = doc.get_mut(id) {
            if node.kind == "text"
                && matches!(
                    prop,
                    "fontFamily"
                        | "fontSize"
                        | "fontWeight"
                        | "lineHeight"
                        | "letterSpacing"
                        | "fontStyle"
                        | "textCase"
                )
            {
                node.h = node.h.max(layout_text(node, measure).height);
            }
        }
        if matches!(prop, "layout" | "gap" | "padding" | "layoutAlign") {
            apply_layout(doc, id);
        }
        doc.touch(Some(id));
    }
    Ok(())
}

fn property_value(prop: &str, value: Value) -> Option<Value> {
    if !matches!(
        prop,
        "opacity"
            | "fillOpacity"
            | "shadowOpacity"
            | "lineHeight"
            | "fontWeight"
            | "w"
            | "h"
            | "radius"
            | "strokeWidth"
            | "shadowBlur"
            | "gap"
            | "padding"
            | "fontSize"
    ) {
        return Some(value);
    }
    let number = property_number(&value);
    if number.is_nan() {
        return None;
    }
    let number = match prop {
        "opacity" | "fillOpacity" | "shadowOpacity" => (number / 100.0).clamp(0.0, 1.0),
        "lineHeight" => (number / 100.0).clamp(0.5, 5.0),
        "w" | "h" => number.max(0.1),
        "radius" | "strokeWidth" | "shadowBlur" | "gap" | "padding" => number.max(0.0),
        "fontSize" => number.clamp(1.0, 512.0),
        _ => number,
    };
    serde_json::Number::from_f64(number).map(Value::Number)
}

fn property_number(value: &Value) -> f64 {
    match value {
        Value::Null => 0.0,
        Value::Bool(value) => f64::from(*value),
        Value::Number(value) => value.as_f64().unwrap_or(f64::NAN),
        Value::String(value) => {
            let value = value.trim_matches(|ch: char| ch.is_whitespace() || ch == '\u{feff}');
            if value.is_empty() {
                0.0
            } else if matches!(value, "Infinity" | "+Infinity" | "-Infinity") {
                if value.starts_with('-') {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                }
            } else if let Some((digits, radix)) = value
                .strip_prefix("0x")
                .or_else(|| value.strip_prefix("0X"))
                .map(|digits| (digits, 16))
                .or_else(|| {
                    value
                        .strip_prefix("0b")
                        .or_else(|| value.strip_prefix("0B"))
                        .map(|digits| (digits, 2))
                })
                .or_else(|| {
                    value
                        .strip_prefix("0o")
                        .or_else(|| value.strip_prefix("0O"))
                        .map(|digits| (digits, 8))
                })
            {
                if digits.is_empty() {
                    return f64::NAN;
                }
                digits
                    .chars()
                    .try_fold(0.0, |number, digit| {
                        digit
                            .to_digit(radix)
                            .map(|digit| number * f64::from(radix) + f64::from(digit))
                    })
                    .unwrap_or(f64::NAN)
            } else if value
                .bytes()
                .any(|ch| ch.is_ascii_alphabetic() && !matches!(ch, b'e' | b'E'))
            {
                f64::NAN
            } else {
                value.parse().unwrap_or(f64::NAN)
            }
        }
        _ => f64::NAN,
    }
}

fn fresh_id(doc: &Document) -> String {
    loop {
        let id = Node::new("rect").id;
        if !doc
            .data
            .pages
            .iter()
            .any(|p| p.nodes.iter().any(|n| n.id == id))
        {
            return id;
        }
    }
}

pub fn set_from_matrix(node: &mut Node, matrix: Matrix) {
    let m = matrix.0;
    let angle = m[1].atan2(m[0]);
    let (s, c) = angle.sin_cos();
    node.rotation = angle.to_degrees();
    node.x = m[4] - node.w / 2. + c * node.w / 2. - s * node.h / 2.;
    node.y = m[5] - node.h / 2. + s * node.w / 2. + c * node.h / 2.;
}

fn parent_matrix(frame: &Frame, node: &Node) -> Matrix {
    node.parent_id
        .as_deref()
        .and_then(|id| frame.world(id))
        .map_or_else(identity, |s| s.matrix)
}

pub fn group_selection(doc: &mut Document, ids: &[String], as_frame: bool) -> Option<String> {
    let roots: Vec<_> = doc.roots(ids).into_iter().cloned().collect();
    let first = roots.first()?;
    let common_parent = roots
        .iter()
        .all(|n| n.parent_id == first.parent_id)
        .then(|| first.parent_id.clone())
        .flatten();
    let frame = compose(doc);
    let pm = common_parent
        .as_deref()
        .and_then(|id| frame.world(id))
        .map_or_else(identity, |s| s.matrix);
    let inv = inverse(pm);
    let rects: Vec<_> = roots
        .iter()
        .filter_map(|n| {
            frame
                .world(&n.id)
                .map(|s| box_of(multiply(inv, s.matrix), n.w, n.h))
        })
        .collect();
    let bounds = union(rects)?;
    let mut group = Node::new(if as_frame { "frame" } else { "group" });
    group.id = fresh_id(doc);
    group.parent_id = common_parent;
    group.x = bounds.x;
    group.y = bounds.y;
    group.w = bounds.w;
    group.h = bounds.h;
    group.clip = as_frame;
    let id = group.id.clone();
    let first_index = doc
        .nodes()
        .iter()
        .position(|n| roots.iter().any(|r| r.id == n.id))?;
    doc.page_mut().nodes.insert(first_index, group);
    doc.refresh();
    doc.touch(Some(&id));
    let grouped = compose(doc);
    let gi = grouped.world(&id)?.inverse;
    for root in roots {
        if let (Some(before), Some(node)) = (frame.world(&root.id), doc.get_mut(&root.id)) {
            set_from_matrix(node, multiply(gi, before.matrix));
            node.parent_id = Some(id.clone());
        }
        doc.touch(Some(&root.id));
    }
    Some(id)
}

pub fn ungroup_selection(doc: &mut Document, ids: &[String]) -> Vec<String> {
    let roots: Vec<_> = doc.roots(ids).into_iter().cloned().collect();
    let mut selected = Vec::new();
    for group in roots {
        if !matches!(group.kind.as_str(), "group" | "frame") {
            continue;
        }
        let frame = compose(doc);
        let inv = inverse(parent_matrix(&frame, &group));
        let children: Vec<_> = doc
            .children(Some(&group.id))
            .iter()
            .map(|n| n.id.clone())
            .collect();
        for id in children {
            if let (Some(before), Some(node)) = (frame.world(&id), doc.get_mut(&id)) {
                node.parent_id = group.parent_id.clone();
                set_from_matrix(node, multiply(inv, before.matrix));
                selected.push(id.clone());
            }
            doc.touch(Some(&id));
        }
        doc.remove(&[group.id]);
    }
    selected
}

pub fn clone_nodes(
    doc: &mut Document,
    ids: &[String],
    offset: f64,
    as_instance: bool,
) -> Vec<String> {
    let roots: Vec<_> = doc.roots(ids).into_iter().cloned().collect();
    let mut selected = Vec::new();
    for root in roots {
        let mut sources = vec![root.clone()];
        sources.extend(doc.descendants(&root.id).into_iter().cloned());
        let remap: HashMap<_, _> = sources
            .iter()
            .map(|n| (n.id.clone(), fresh_id(doc)))
            .collect();
        for src in sources {
            let mut copy = src.clone();
            copy.id = remap[&src.id].clone();
            copy.parent_id = if src.id == root.id {
                src.parent_id.clone()
            } else {
                src.parent_id.as_ref().and_then(|id| remap.get(id)).cloned()
            };
            copy.version = 0;
            if src.id == root.id {
                copy.x += offset;
                copy.y += offset;
                if !as_instance {
                    copy.name.push_str(" copy");
                }
                selected.push(copy.id.clone());
            }
            if as_instance {
                copy.extra.insert("sourceId".into(), json!(src.id));
                copy.extra.insert("component".into(), json!(false));
                copy.extra
                    .insert("isInstance".into(), json!(src.id == root.id));
                copy.extra.insert("overrides".into(), json!({}));
            }
            doc.add(copy);
        }
    }
    selected
}

#[derive(Clone, Copy, Debug)]
pub enum Order {
    Front,
    Back,
    Forward,
    Backward,
}

pub fn reorder(doc: &mut Document, ids: &[String], direction: Order) {
    let mut roots: Vec<_> = doc.roots(ids).into_iter().cloned().collect();
    if matches!(direction, Order::Back | Order::Backward) {
        roots.reverse();
    }
    for root in roots {
        let siblings = doc.children(root.parent_id.as_deref());
        let Some(index) = siblings.iter().position(|n| n.id == root.id) else {
            continue;
        };
        let target = match direction {
            Order::Front => siblings.last(),
            Order::Back => siblings.first(),
            Order::Forward => siblings.get(index + 1),
            Order::Backward => index.checked_sub(1).and_then(|i| siblings.get(i)),
        }
        .map(|n| n.id.clone());
        let Some(target) = target.filter(|id| *id != root.id) else {
            continue;
        };
        let nodes = &mut doc.page_mut().nodes;
        let Some(old) = nodes.iter().position(|n| n.id == root.id) else {
            continue;
        };
        let moved = nodes.remove(old);
        let at = nodes
            .iter()
            .position(|n| n.id == target)
            .unwrap_or(nodes.len());
        let after = usize::from(matches!(direction, Order::Front | Order::Forward));
        nodes.insert((at + after).min(nodes.len()), moved);
    }
    doc.refresh();
}

#[derive(Clone, Copy, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
    Top,
    Middle,
    Bottom,
}

pub fn align_selection(doc: &mut Document, ids: &[String], mode: Align) {
    let roots: Vec<_> = doc.roots(ids).into_iter().cloned().collect();
    let frame = compose(doc);
    let bounds = if roots.len() == 1
        && roots[0]
            .parent_id
            .as_deref()
            .is_some_and(|id| !id.is_empty())
    {
        roots[0]
            .parent_id
            .as_deref()
            .and_then(|id| frame.world(id))
            .map(|s| s.bounds)
    } else {
        let ids: Vec<_> = roots.iter().map(|n| n.id.clone()).collect();
        frame.bounds(Some(&ids))
    };
    let Some(b) = bounds else { return };
    for root in roots {
        let Some(world) = frame.world(&root.id) else {
            continue;
        };
        let w = world.bounds;
        let (dx, dy) = match mode {
            Align::Left => (b.x - w.x, 0.),
            Align::Center => (b.x + b.w / 2. - w.x - w.w / 2., 0.),
            Align::Right => (b.x + b.w - w.x - w.w, 0.),
            Align::Top => (0., b.y - w.y),
            Align::Middle => (0., b.y + b.h / 2. - w.y - w.h / 2.),
            Align::Bottom => (0., b.y + b.h - w.y - w.h),
        };
        let inv = inverse(parent_matrix(&frame, &root)).0;
        if let Some(node) = doc.get_mut(&root.id) {
            node.x += inv[0] * dx + inv[2] * dy;
            node.y += inv[1] * dx + inv[3] * dy;
        }
        doc.touch(Some(&root.id));
    }
}

pub fn distribute(doc: &mut Document, ids: &[String], horizontal: bool) -> bool {
    let frame = compose(doc);
    let mut roots: Vec<_> = doc
        .roots(ids)
        .into_iter()
        .filter_map(|n| frame.world(&n.id).map(|s| (n.clone(), s.bounds)))
        .collect();
    if roots.len() < 3 {
        return false;
    }
    roots.sort_by(|(_, a), (_, b)| {
        if horizontal {
            a.x.total_cmp(&b.x)
        } else {
            a.y.total_cmp(&b.y)
        }
    });
    let first = roots[0].1;
    let last = roots[roots.len() - 1].1;
    let total: f64 = roots
        .iter()
        .map(|(_, b)| if horizontal { b.w } else { b.h })
        .sum();
    let extent = if horizontal {
        last.x + last.w - first.x
    } else {
        last.y + last.h - first.y
    };
    let gap = (extent - total) / (roots.len() - 1) as f64;
    let mut at = if horizontal { first.x } else { first.y };
    for (root, bounds) in roots {
        let delta = at - if horizontal { bounds.x } else { bounds.y };
        let inv = inverse(parent_matrix(&frame, &root)).0;
        if let Some(node) = doc.get_mut(&root.id) {
            node.x += if horizontal {
                inv[0] * delta
            } else {
                inv[2] * delta
            };
            node.y += if horizontal {
                inv[1] * delta
            } else {
                inv[3] * delta
            };
        }
        doc.touch(Some(&root.id));
        at += if horizontal { bounds.w } else { bounds.h } + gap;
    }
    true
}

pub fn make_component(doc: &mut Document, ids: &[String]) -> Option<String> {
    let id = if ids.len() > 1 {
        group_selection(doc, ids, false)?
    } else {
        ids.first()?.clone()
    };
    let node = doc.get_mut(&id)?;
    node.extra.insert("component".into(), json!(true));
    node.extra.insert("isInstance".into(), json!(false));
    doc.touch(Some(&id));
    Some(id)
}

pub fn instantiate(doc: &mut Document, id: &str, center: Point) -> Option<String> {
    let page = doc
        .data
        .pages
        .iter()
        .find(|p| p.nodes.iter().any(|n| n.id == id))?
        .id
        .clone();
    let active = doc.data.page_id.clone();
    let instance = if page == active {
        clone_nodes(doc, &[id.to_owned()], 0., true)
            .into_iter()
            .next()?
    } else {
        doc.data.page_id = page;
        doc.refresh();
        let mut sources = doc.get(id).cloned().into_iter().collect::<Vec<_>>();
        sources.extend(doc.descendants(id).into_iter().cloned());
        doc.data.page_id = active;
        doc.refresh();
        let remap: HashMap<_, _> = sources
            .iter()
            .map(|n| (n.id.clone(), fresh_id(doc)))
            .collect();
        for mut node in sources {
            let source = node.id.clone();
            node.id = remap[&source].clone();
            node.parent_id = node
                .parent_id
                .as_ref()
                .and_then(|id| remap.get(id))
                .cloned();
            node.extra.insert("sourceId".into(), json!(source));
            node.extra.insert("component".into(), json!(false));
            node.extra.insert("isInstance".into(), json!(source == id));
            node.extra.insert("overrides".into(), json!({}));
            doc.add(node);
        }
        remap.get(id)?.clone()
    };
    let root = doc.get_mut(&instance)?;
    root.parent_id = None;
    root.x = center.x - root.w / 2.;
    root.y = center.y - root.h / 2.;
    doc.touch(Some(&instance));
    Some(instance)
}

pub fn sync_components(doc: &mut Document) -> Result<(), serde_json::Error> {
    let mut sources: HashMap<_, _> = doc
        .data
        .pages
        .iter()
        .flat_map(|p| &p.nodes)
        .map(|n| serde_json::to_value(n).map(|value| (n.id.clone(), value)))
        .collect::<Result<_, _>>()?;
    for page in &mut doc.data.pages {
        for node in &mut page.nodes {
            let Some(source) = node
                .string("sourceId")
                .and_then(|id| sources.get(id))
                .and_then(Value::as_object)
            else {
                continue;
            };
            let mut value = serde_json::to_value(&*node)?;
            let Some(target) = value.as_object_mut() else {
                continue;
            };
            let mut omitted: HashSet<&str> = [
                "id",
                "parentId",
                "component",
                "sourceId",
                "isInstance",
                "version",
                "overrides",
                "prototypeTarget",
                "name",
            ]
            .into_iter()
            .collect();
            if node.flag("isInstance") {
                omitted.extend(["x", "y", "rotation"]);
            }
            let overrides = node.extra.get("overrides").and_then(Value::as_object);
            let mut changes = 0;
            for (key, value) in source {
                if omitted.contains(key.as_str())
                    || overrides.is_some_and(|map| map.contains_key(key))
                {
                    continue;
                }
                if target.get(key) != Some(value) {
                    target.insert(key.clone(), value.clone());
                    changes += 1;
                }
            }
            if changes > 0 {
                let version = node.version + changes;
                *node = serde_json::from_value(value)?;
                node.version = version;
                sources.insert(node.id.clone(), serde_json::to_value(&*node)?);
            }
        }
    }
    doc.touch(None);
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clipboard {
    pub format: String,
    pub root_ids: Vec<String>,
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub assets: BTreeMap<String, Rc<str>>,
}

pub fn copy_selection(doc: &Document, ids: &[String]) -> Option<Clipboard> {
    let roots = doc.roots(ids);
    if roots.is_empty() {
        return None;
    }
    let frame = compose(doc);
    let root_ids = roots.iter().map(|n| n.id.clone()).collect();
    let mut nodes = Vec::new();
    for root in roots {
        let mut copy = root.clone();
        if let Some(world) = frame.world(&root.id) {
            set_from_matrix(&mut copy, world.matrix);
        }
        copy.parent_id = None;
        nodes.push(copy);
        nodes.extend(doc.descendants(&root.id).into_iter().cloned());
    }
    Some(Clipboard {
        format: "vellum-clipboard".into(),
        root_ids,
        nodes,
        assets: doc.data.assets.clone(),
    })
}

pub fn paste_selection(
    doc: &mut Document,
    payload: &Clipboard,
) -> Result<Vec<String>, DocumentError> {
    let temp = json!({"format":"vellum", "version":1, "name":"Clipboard", "pageId":"paste",
        "pages":[{"id":"paste","name":"Paste","nodes":payload.nodes}],"assets":payload.assets});
    let parsed = Document::parse(&temp.to_string())?;
    let remap: HashMap<_, _> = parsed
        .nodes()
        .iter()
        .map(|n| (n.id.clone(), fresh_id(doc)))
        .collect();
    doc.data.assets.extend(parsed.data.assets.clone());
    let mut roots = Vec::new();
    for mut node in parsed.nodes().iter().cloned() {
        let source = node.id.clone();
        node.id = remap[&source].clone();
        node.parent_id = node
            .parent_id
            .as_ref()
            .and_then(|id| remap.get(id))
            .cloned();
        if payload.root_ids.contains(&source) {
            node.x += 24.;
            node.y += 24.;
            roots.push(node.id.clone());
        }
        doc.add(node);
    }
    Ok(roots)
}

pub fn remap_color_tokens(doc: &mut Document, next: Value) {
    let Some(colors) = next.as_array() else {
        return;
    };
    let replacements: HashMap<_, _> = doc.data.tokens["colors"]
        .as_array()
        .into_iter()
        .flatten()
        .zip(colors)
        .filter_map(|(old, new)| {
            Some((
                old["value"].as_str()?.to_lowercase(),
                new["value"].as_str()?.to_owned(),
            ))
        })
        .collect();
    for page in &mut doc.data.pages {
        for node in &mut page.nodes {
            for color in [
                &mut node.fill,
                &mut node.fill2,
                &mut node.stroke,
                &mut node.shadow_color,
            ] {
                if let Some(next) = replacements.get(&color.to_lowercase()) {
                    *color = next.clone();
                    node.version += 1;
                }
            }
        }
    }
    doc.data.tokens["colors"] = next;
    doc.touch(None);
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::text_layout::Measure;

    struct FixedWidth;
    impl Measure for FixedWidth {
        fn width(&self, node: &Node, text: &str) -> f64 {
            text.chars().count() as f64 * node.font_size / 2.0
        }
    }

    #[test]
    fn property_percentages_and_numeric_limits_match_reference() {
        let mut doc = Document::empty("Properties");
        let id = doc.add(Node::new("text"));
        for (prop, input, expected) in [
            ("opacity", json!(25), 0.25),
            ("fillOpacity", json!(140), 1.0),
            ("shadowOpacity", json!(-5), 0.0),
            ("lineHeight", json!(200), 2.0),
            ("lineHeight", json!(0), 0.5),
            ("lineHeight", json!(1000), 5.0),
            ("fontWeight", json!("650"), 650.0),
            ("w", json!(-5), 0.1),
            ("h", json!(-5), 0.1),
            ("radius", json!(-5), 0.0),
            ("strokeWidth", json!(-5), 0.0),
            ("shadowBlur", json!(-5), 0.0),
            ("gap", json!(-5), 0.0),
            ("padding", json!(-5), 0.0),
            ("fontSize", json!(0), 1.0),
            ("fontSize", json!(999), 512.0),
        ] {
            set_property(
                &mut doc,
                std::slice::from_ref(&id),
                prop,
                input,
                &FixedWidth,
            )
            .unwrap();
            let node = serde_json::to_value(doc.get(&id).unwrap()).unwrap();
            assert_eq!(node[prop].as_f64(), Some(expected), "{prop}");
        }
    }

    #[test]
    fn property_invalid_numeric_input_leaves_document_untouched() {
        let mut doc = Document::empty("Properties");
        let id = doc.add(Node::new("rect"));
        let before = doc.data.clone();
        for prop in [
            "opacity",
            "lineHeight",
            "fontWeight",
            "w",
            "radius",
            "fontSize",
        ] {
            set_property(
                &mut doc,
                std::slice::from_ref(&id),
                prop,
                json!("invalid"),
                &FixedWidth,
            )
            .unwrap();
        }
        assert_eq!(doc.data, before);
    }

    #[test]
    fn property_numeric_strings_follow_javascript_number_coercion() {
        let mut doc = Document::empty("Properties");
        let id = doc.add(Node::new("text"));
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "fontWeight",
            json!("0x2bc"),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().font_weight, 700.0);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "fontWeight",
            json!(""),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().font_weight, 0.0);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "opacity",
            json!("Infinity"),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().opacity, 1.0);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "opacity",
            json!(50),
            &FixedWidth,
        )
        .unwrap();
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "opacity",
            json!("inf"),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().opacity, 0.5);
    }

    #[test]
    fn property_edit_creates_instance_override_that_survives_propagation() {
        let mut doc = Document::empty("Properties");
        let source = doc.add(Node::new("rect"));
        let instance = instantiate(&mut doc, &source, Point::new(500.0, 400.0)).unwrap();
        set_property(
            &mut doc,
            std::slice::from_ref(&instance),
            "opacity",
            json!(25),
            &FixedWidth,
        )
        .unwrap();
        doc.get_mut(&source).unwrap().opacity = 0.8;
        sync_components(&mut doc).unwrap();
        let node = doc.get(&instance).unwrap();
        assert_eq!(node.opacity, 0.25);
        assert_eq!(node.extra["overrides"]["opacity"], json!(0.25));
    }

    #[test]
    fn property_frame_width_moves_right_constrained_child() {
        let mut doc = Document::empty("Properties");
        let parent = doc.add(Node::new("frame"));
        let mut child = Node::new("rect");
        child.parent_id = Some(parent.clone());
        child.x = 40.0;
        child.extra.insert("constraintH".into(), json!("right"));
        let child = doc.add(child);
        set_property(&mut doc, &[parent], "w", json!(500), &FixedWidth).unwrap();
        assert_eq!(doc.get(&child).unwrap().x, 140.0);
    }

    #[test]
    fn property_typography_increases_height_without_shrinking_existing_box() {
        let mut doc = Document::empty("Properties");
        let mut node = Node::new("text");
        node.text = "one\ntwo".into();
        node.h = 10.0;
        node.line_height = 1.5;
        let id = doc.add(node);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "fontSize",
            json!(40),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().h, 120.0);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "fontSize",
            json!(20),
            &FixedWidth,
        )
        .unwrap();
        assert_eq!(doc.get(&id).unwrap().h, 120.0);
    }

    #[test]
    fn property_layout_change_immediately_places_children() {
        let mut doc = Document::empty("Properties");
        let parent = doc.add(Node::new("frame"));
        let mut child = Node::new("rect");
        child.parent_id = Some(parent.clone());
        let child = doc.add(child);
        set_property(
            &mut doc,
            std::slice::from_ref(&parent),
            "layout",
            json!("horizontal"),
            &FixedWidth,
        )
        .unwrap();
        set_property(&mut doc, &[parent], "padding", json!(24), &FixedWidth).unwrap();
        let node = doc.get(&child).unwrap();
        assert_eq!((node.x, node.y), (24.0, 24.0));
    }

    #[test]
    fn property_unknown_values_are_retained_without_disturbing_other_extensions() {
        let mut doc = Document::empty("Properties");
        let mut node = Node::new("rect");
        node.extra.insert("existing".into(), json!([1, true]));
        let id = doc.add(node);
        set_property(
            &mut doc,
            std::slice::from_ref(&id),
            "custom",
            json!({"nested":"kept"}),
            &FixedWidth,
        )
        .unwrap();
        let node = doc.get(&id).unwrap();
        assert_eq!(node.extra["existing"], json!([1, true]));
        assert_eq!(node.extra["custom"], json!({"nested":"kept"}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Document, Node};
    use crate::scene::compose;
    use serde_json::json;

    fn fixture() -> (Document, Vec<String>) {
        let mut doc = Document::empty("Commands");
        let ids = [
            (0., 0., 20., 20.),
            (60., 40., 40., 40.),
            (150., 120., 30., 30.),
        ]
        .into_iter()
        .map(|(x, y, w, h)| {
            let mut n = Node::new("rect");
            n.x = x;
            n.y = y;
            n.w = w;
            n.h = h;
            doc.add(n)
        })
        .collect();
        (doc, ids)
    }

    fn assert_matrix(a: crate::affine::Matrix, b: crate::affine::Matrix) {
        for (a, b) in a.0.into_iter().zip(b.0) {
            assert!((a - b).abs() < 1e-8, "{a} != {b}");
        }
    }

    #[test]
    fn grouping_and_ungrouping_preserve_rotated_world_transforms() {
        let (mut doc, ids) = fixture();
        doc.get_mut(&ids[0]).unwrap().rotation = 30.;
        let before = compose(&doc);
        let group = group_selection(&mut doc, &ids, false).unwrap();
        assert_eq!(doc.get(&group).unwrap().version, 1);
        let grouped = compose(&doc);
        for id in &ids {
            assert_matrix(
                before.world(id).unwrap().matrix,
                grouped.world(id).unwrap().matrix,
            );
        }
        assert_eq!(ungroup_selection(&mut doc, &[group]), ids);
        let after = compose(&doc);
        for id in &ids {
            assert_matrix(
                before.world(id).unwrap().matrix,
                after.world(id).unwrap().matrix,
            );
        }
    }

    #[test]
    fn duplicate_remaps_descendants_without_cloning_selected_child_twice() {
        let (mut doc, ids) = fixture();
        let group = group_selection(&mut doc, &ids[..2], true).unwrap();
        let copied = clone_nodes(&mut doc, &[group.clone(), ids[0].clone()], 20., false);
        assert_eq!(copied.len(), 1);
        assert_eq!(doc.descendants(&copied[0]).len(), 2);
        assert_eq!(
            doc.get(&copied[0]).unwrap().x,
            doc.get(&group).unwrap().x + 20.
        );
        let unique: std::collections::HashSet<_> = doc.nodes().iter().map(|n| &n.id).collect();
        assert_eq!(unique.len(), doc.nodes().len());
    }

    #[test]
    fn reorder_supports_all_four_directions() {
        for (direction, expected) in [
            (Order::Front, vec![0, 2, 1]),
            (Order::Back, vec![1, 0, 2]),
            (Order::Forward, vec![0, 2, 1]),
            (Order::Backward, vec![1, 0, 2]),
        ] {
            let (mut doc, ids) = fixture();
            reorder(&mut doc, &ids[1..2], direction);
            assert_eq!(
                doc.nodes().iter().map(|n| n.id.clone()).collect::<Vec<_>>(),
                expected
                    .into_iter()
                    .map(|i| ids[i].clone())
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn align_supports_six_edges_and_centers() {
        for mode in [
            Align::Left,
            Align::Center,
            Align::Right,
            Align::Top,
            Align::Middle,
            Align::Bottom,
        ] {
            let (mut doc, ids) = fixture();
            align_selection(&mut doc, &ids, mode);
            let frame = compose(&doc);
            let positions: Vec<_> = ids
                .iter()
                .map(|id| {
                    let b = frame.world(id).unwrap().bounds;
                    match mode {
                        Align::Left => b.x,
                        Align::Center => b.x + b.w / 2.,
                        Align::Right => b.x + b.w,
                        Align::Top => b.y,
                        Align::Middle => b.y + b.h / 2.,
                        Align::Bottom => b.y + b.h,
                    }
                })
                .collect();
            assert!(positions.iter().all(|x| (x - positions[0]).abs() < 1e-8));
        }
    }

    #[test]
    fn distribution_requires_three_layers_and_equalizes_gaps() {
        let (mut doc, ids) = fixture();
        assert!(!distribute(&mut doc, &ids[..2], true));
        assert!(distribute(&mut doc, &ids, true));
        let nodes: Vec<_> = ids.iter().map(|id| doc.get(id).unwrap()).collect();
        assert_eq!(
            nodes[1].x - nodes[0].x - nodes[0].w,
            nodes[2].x - nodes[1].x - nodes[1].w
        );
    }

    #[test]
    fn component_propagation_respects_overrides_and_instance_position() {
        let (mut doc, ids) = fixture();
        let source = make_component(&mut doc, &ids[..1]).unwrap();
        let instance =
            instantiate(&mut doc, &source, crate::affine::Point { x: 500., y: 400. }).unwrap();
        let x = doc.get(&instance).unwrap().x;
        doc.get_mut(&source).unwrap().fill = "#123456".into();
        doc.get_mut(&source).unwrap().x = 100.;
        sync_components(&mut doc).unwrap();
        assert_eq!(doc.get(&instance).unwrap().fill, "#123456");
        assert_eq!(doc.get(&instance).unwrap().x, x);
        doc.get_mut(&instance)
            .unwrap()
            .extra
            .insert("overrides".into(), json!({"fill":"#ffffff"}));
        doc.get_mut(&instance).unwrap().fill = "#ffffff".into();
        doc.get_mut(&source).unwrap().fill = "#654321".into();
        sync_components(&mut doc).unwrap();
        assert_eq!(doc.get(&instance).unwrap().fill, "#ffffff");
    }

    #[test]
    fn component_instances_can_be_placed_on_another_page() {
        let mut doc = crate::starter::make_starter().unwrap();
        let source = doc.nodes()[0].id.clone();
        let count = doc.descendants(&source).len();
        doc.data.page_id = doc.data.pages[2].id.clone();
        doc.refresh();
        let instance =
            instantiate(&mut doc, &source, crate::affine::Point { x: 0., y: 0. }).unwrap();
        assert_eq!(doc.descendants(&instance).len(), count);
        assert_eq!(
            doc.get(&instance).unwrap().string("sourceId"),
            Some(source.as_str())
        );
        assert!(doc.get(&instance).unwrap().parent_id.is_none());
    }

    #[test]
    fn clipboard_uses_world_space_and_pastes_with_24_pixel_offset() {
        let (mut doc, ids) = fixture();
        let group = group_selection(&mut doc, &ids[..2], true).unwrap();
        doc.get_mut(&group).unwrap().rotation = 30.;
        let before = compose(&doc).world(&ids[0]).unwrap().matrix;
        let clipboard = copy_selection(&doc, &ids[..1]).unwrap();
        let pasted = paste_selection(&mut doc, &clipboard).unwrap();
        let after = compose(&doc).world(&pasted[0]).unwrap().matrix;
        let mut expected = before;
        expected.0[4] += 24.;
        expected.0[5] += 24.;
        assert_matrix(after, expected);
    }

    #[test]
    fn color_tokens_remap_exact_matches_across_pages() {
        let (mut doc, ids) = fixture();
        doc.data.tokens = json!({"colors":[{"name":"Iris","value":"#A38BFF"}],"typography":[]});
        doc.get_mut(&ids[0]).unwrap().stroke = "#a38bff".into();
        remap_color_tokens(&mut doc, json!([{"name":"Mint","value":"#aaffcc"}]));
        assert!(doc.nodes().iter().all(|n| n.fill == "#aaffcc"));
        assert_eq!(doc.get(&ids[0]).unwrap().stroke, "#aaffcc");
        assert_eq!(doc.data.tokens["colors"][0]["name"], "Mint");
    }

    #[test]
    fn grouping_different_rotated_parents_preserves_world_space() {
        let (mut doc, ids) = fixture();
        for (i, id) in ids[..2].iter().enumerate() {
            let mut parent = Node::new("frame");
            parent.x = 100. + i as f64 * 300.;
            parent.y = 80.;
            parent.rotation = 15. + i as f64 * 30.;
            let parent_id = doc.add(parent);
            doc.get_mut(id).unwrap().parent_id = Some(parent_id);
        }
        let before = compose(&doc);
        let group = group_selection(&mut doc, &ids[..2], false).unwrap();
        assert!(doc.get(&group).unwrap().parent_id.is_none());
        let after = compose(&doc);
        for id in &ids[..2] {
            assert_matrix(
                before.world(id).unwrap().matrix,
                after.world(id).unwrap().matrix,
            );
        }
    }

    #[test]
    fn invalid_clipboard_does_not_mutate_document_or_assets() {
        let (mut doc, ids) = fixture();
        let mut clipboard = copy_selection(&doc, &ids).unwrap();
        clipboard.nodes[0].w = -10.;
        clipboard
            .assets
            .insert("image".into(), Rc::from("data:image/png;base64,AA=="));
        let before = doc.data.clone();
        assert!(paste_selection(&mut doc, &clipboard).is_err());
        assert_eq!(doc.data, before);
    }

    #[test]
    fn child_component_geometry_propagates_but_root_geometry_stays_placed() {
        let (mut doc, ids) = fixture();
        let source = make_component(&mut doc, &ids[..2]).unwrap();
        let instance = instantiate(&mut doc, &source, Point { x: 500., y: 400. }).unwrap();
        let child = doc
            .descendants(&instance)
            .into_iter()
            .find(|n| n.string("sourceId") == Some(ids[0].as_str()))
            .unwrap()
            .id
            .clone();
        doc.get_mut(&ids[0]).unwrap().x = 35.;
        sync_components(&mut doc).unwrap();
        assert_eq!(doc.get(&child).unwrap().x, 35.);
        let root = doc.get(&instance).unwrap();
        assert_eq!(root.x + root.w / 2., 500.);
    }
}
