use std::collections::HashMap;
use std::sync::Arc;

use crate::affine::{box_of, identity, inverse, local_matrix, multiply, union, Matrix, Rect};
use crate::document::{Document, Node};

#[derive(Clone, Debug, PartialEq)]
pub struct Clip {
    pub matrix: Matrix,
    pub inverse: Matrix,
    pub w: f64,
    pub h: f64,
    pub radius: f64,
}

#[derive(Clone, Debug)]
pub struct SceneItem {
    pub node: Node,
    pub matrix: Matrix,
    pub inverse: Matrix,
    pub bounds: Rect,
    pub opacity: f64,
    pub clips: Vec<Clip>,
    pub hidden: bool,
    pub locked: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Frame {
    pub items: Vec<Arc<SceneItem>>,
    pub world: HashMap<String, Arc<SceneItem>>,
    pub revision: u64,
}

impl Frame {
    pub fn world(&self, id: &str) -> Option<&SceneItem> {
        self.world.get(id).map(Arc::as_ref)
    }

    pub fn bounds(&self, ids: Option<&[String]>) -> Option<Rect> {
        match ids {
            Some(ids) => union(ids.iter().filter_map(|id| self.world(id)).map(|s| s.bounds)),
            None => union(
                self.items
                    .iter()
                    .filter(|s| s.node.parent_id.as_deref().is_none_or(str::is_empty))
                    .map(|s| s.bounds),
            ),
        }
    }
}

pub fn compose(doc: &Document) -> Frame {
    let mut children: HashMap<Option<&str>, Vec<&Node>> = HashMap::new();
    for n in doc.nodes() {
        let parent = n.parent_id.as_deref().filter(|id| doc.get(id).is_some());
        children.entry(parent).or_default().push(n);
    }
    let mut frame = Frame {
        revision: doc.revision,
        ..Frame::default()
    };
    let state = Inherited {
        matrix: identity(),
        opacity: 1.0,
        clips: Vec::new(),
        hidden: false,
        locked: false,
    };
    walk(None, &children, &state, 0, &mut frame);
    frame
}

struct Inherited {
    matrix: Matrix,
    opacity: f64,
    clips: Vec<Clip>,
    hidden: bool,
    locked: bool,
}

fn walk(
    parent: Option<&str>,
    children: &HashMap<Option<&str>, Vec<&Node>>,
    state: &Inherited,
    depth: usize,
    frame: &mut Frame,
) {
    if depth > 100 {
        return;
    }
    for &node in children.get(&parent).into_iter().flatten() {
        let matrix = multiply(state.matrix, local_matrix(node));
        let inv = inverse(matrix);
        let mut next = Inherited {
            matrix,
            opacity: state.opacity * node.opacity,
            clips: state.clips.clone(),
            hidden: state.hidden || !node.visible,
            locked: state.locked || node.locked,
        };
        let item = Arc::new(SceneItem {
            node: node.clone(),
            matrix,
            inverse: inv,
            bounds: box_of(matrix, node.w, node.h),
            opacity: next.opacity,
            clips: state.clips.clone(),
            hidden: next.hidden,
            locked: next.locked,
        });
        frame.world.insert(node.id.clone(), Arc::clone(&item));
        if !next.hidden {
            frame.items.push(item);
        }
        if node.kind == "frame" && node.clip {
            next.clips.push(Clip {
                matrix,
                inverse: inv,
                w: node.w,
                h: node.h,
                radius: node.radius,
            });
        }
        walk(Some(&node.id), children, &next, depth + 1, frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_parent_id_is_a_root_for_default_bounds() {
        let mut doc = Document::empty("test");
        let n = node("root", "rect", Some(""));
        doc.add(n);
        assert_eq!(
            compose(&doc).bounds(None),
            Some(Rect::new(0.0, 0.0, 160.0, 100.0))
        );
    }

    fn node(id: &str, kind: &str, parent: Option<&str>) -> Node {
        let mut n = Node::new(kind);
        n.id = id.into();
        n.parent_id = parent.map(str::to_owned);
        n
    }

    #[test]
    fn hidden_nodes_keep_world_geometry_but_leave_paint_order() {
        let mut doc = Document::empty("test");
        let mut root = node("root", "frame", None);
        root.visible = false;
        root.locked = true;
        root.opacity = 0.5;
        let mut child = node("child", "rect", Some("root"));
        child.opacity = 0.4;
        doc.add(root);
        doc.add(child);
        let frame = compose(&doc);
        assert!(frame.items.is_empty());
        let child = frame.world("child").unwrap();
        assert!(child.hidden && child.locked);
        assert_eq!(child.opacity, 0.2);
    }

    #[test]
    fn clipping_chain_contains_only_clipping_frames() {
        let mut doc = Document::empty("test");
        doc.add(node("frame", "frame", None));
        let mut group = node("group", "group", Some("frame"));
        group.clip = true;
        doc.add(group);
        doc.add(node("rect", "rect", Some("group")));
        let frame = compose(&doc);
        assert_eq!(frame.world("rect").unwrap().clips.len(), 1);
        assert!(frame.world("frame").unwrap().clips.is_empty());
    }

    #[test]
    fn hierarchy_drives_paint_order_and_world_translation() {
        let mut doc = Document::empty("test");
        let mut child = node("child", "rect", Some("parent"));
        child.x = 7.0;
        doc.add(child);
        let mut parent = node("parent", "frame", None);
        parent.x = 30.0;
        doc.add(parent);
        let frame = compose(&doc);
        assert_eq!(
            frame
                .items
                .iter()
                .map(|s| s.node.id.as_str())
                .collect::<Vec<_>>(),
            ["parent", "child"]
        );
        assert_eq!(frame.world("child").unwrap().bounds.x, 37.0);
    }
}
