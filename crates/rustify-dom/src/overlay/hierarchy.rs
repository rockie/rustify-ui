//! Layer ownership and notification reparenting without DOM effects.
use super::LayerId;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug)]
struct Node {
    parent: Option<LayerId>,
    modal: bool,
    notification: bool,
}
#[derive(Default)]
pub(super) struct Hierarchy {
    nodes: BTreeMap<LayerId, Node>,
    order: Vec<LayerId>,
    next: u64,
    modal: Option<LayerId>,
}
impl Hierarchy {
    pub fn contains(&self, id: LayerId) -> bool {
        self.nodes.contains_key(&id)
    }
    pub fn notification(&self, id: LayerId) -> bool {
        self.nodes.get(&id).is_some_and(|node| node.notification)
    }
    pub fn parent(&self, id: LayerId) -> Option<LayerId> {
        self.nodes.get(&id).and_then(|node| node.parent)
    }
    pub fn top_modal(&self) -> Option<LayerId> {
        self.modal
    }
    pub fn top(&self) -> Option<LayerId> {
        self.order.last().copied()
    }
    pub fn ordered(&self) -> &[LayerId] {
        &self.order
    }
    pub fn within(&self, id: LayerId, ancestor: LayerId) -> bool {
        let mut at = Some(id);
        while let Some(id) = at {
            if id == ancestor {
                return true;
            }
            at = self.parent(id);
        }
        false
    }
    pub fn interactive(&self, id: LayerId) -> bool {
        self.contains(id) && self.modal.is_none_or(|modal| self.within(id, modal))
    }
    pub fn insert(&mut self, parent: Option<LayerId>, modal: bool, notification: bool) -> LayerId {
        self.next += 1;
        let id = LayerId(self.next);
        self.nodes.insert(
            id,
            Node {
                parent,
                modal,
                notification,
            },
        );
        self.rebind(&BTreeSet::new());
        self.rebuild();
        id
    }
    /// Notification roots survive a retired host. Ordinary descendants retire
    /// with it, before the host, while notification-owned descendants stay intact.
    pub fn remove(&mut self, id: LayerId) -> Vec<LayerId> {
        if !self.contains(id) {
            return Vec::new();
        }
        let mut removed = BTreeSet::from([id]);
        for id in &self.order {
            let node = &self.nodes[id];
            if !node.notification && node.parent.is_some_and(|parent| removed.contains(&parent)) {
                removed.insert(*id);
            }
        }
        let retired = self
            .order
            .iter()
            .rev()
            .copied()
            .filter(|id| removed.contains(id))
            .collect();
        self.rebind(&removed);
        self.nodes.retain(|id, _| !removed.contains(id));
        self.rebuild();
        retired
    }
    fn rebind(&mut self, removed: &BTreeSet<LayerId>) {
        let modal = self
            .nodes
            .iter()
            .rev()
            .find(|(id, node)| node.modal && !removed.contains(id))
            .map(|(id, _)| *id);
        let changes: Vec<_> = self
            .nodes
            .iter()
            .filter(|(id, node)| node.notification && !removed.contains(id))
            .map(|(id, node)| {
                let parent = if modal.is_some_and(|modal| self.within(modal, *id)) {
                    // A modal opened from a notification cannot own its ancestor.
                    let mut parent = node.parent;
                    while let Some(id) = parent.filter(|id| removed.contains(id)) {
                        parent = self.parent(id);
                    }
                    parent
                } else {
                    modal
                };
                (*id, parent)
            })
            .collect();
        for (id, parent) in changes {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.parent = parent;
            }
        }
        self.modal = modal;
    }
    fn rebuild(&mut self) {
        let mut children: BTreeMap<Option<LayerId>, Vec<LayerId>> = BTreeMap::new();
        for (id, node) in &self.nodes {
            children.entry(node.parent).or_default().push(*id);
        }
        let mut path = BTreeSet::new();
        let mut at = self.modal;
        while let Some(id) = at {
            path.insert(id);
            at = self.parent(id);
        }
        for children in children.values_mut() {
            children.sort_by_key(|id| path.contains(id));
        }
        fn visit(
            parent: Option<LayerId>,
            children: &BTreeMap<Option<LayerId>, Vec<LayerId>>,
            order: &mut Vec<LayerId>,
        ) {
            if let Some(ids) = children.get(&parent) {
                for id in ids {
                    order.push(*id);
                    visit(Some(*id), children, order);
                }
            }
        }
        self.order.clear();
        visit(None, &children, &mut self.order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descendants_outrank_parents_but_later_sibling_branches_stay_above() {
        let mut tree = Hierarchy::default();
        let a = tree.insert(None, false, false);
        let b = tree.insert(None, false, false);
        let c = tree.insert(Some(a), false, false);
        assert_eq!(tree.ordered(), &[a, c, b]);
        assert_eq!(tree.top(), Some(b));
        assert_eq!(tree.remove(a), vec![c, a]);
        assert!(tree.contains(b));
        assert!(!tree.contains(c));
        assert!(tree.insert(None, false, false) > c);
    }
    #[test]
    fn modal_domain_excludes_unrelated_layers_registered_after_it() {
        let mut tree = Hierarchy::default();
        let m = tree.insert(None, true, false);
        let outside = tree.insert(None, false, false);
        let child = tree.insert(Some(m), false, false);
        assert_eq!(tree.top_modal(), Some(m));
        assert_eq!(tree.top(), Some(child));
        assert!(!tree.interactive(outside));
        assert!(tree.interactive(m));
    }
    #[test]
    fn notifications_rebind_without_replacing_descendants_or_registration_order() {
        let mut tree = Hierarchy::default();
        let toast = tree.insert(None, false, true);
        let child = tree.insert(Some(toast), false, false);
        let first = tree.insert(None, true, false);
        assert_eq!(tree.parent(toast), Some(first));
        assert_eq!(tree.parent(child), Some(toast));
        assert_eq!(tree.top(), Some(child));
        let second = tree.insert(None, true, false);
        assert_eq!(tree.parent(toast), Some(second));
        assert_eq!(tree.remove(second), vec![second]);
        assert_eq!(tree.parent(toast), Some(first));
        assert_eq!(tree.remove(first), vec![first]);
        assert_eq!(tree.parent(toast), None);
        assert_eq!(tree.top(), Some(child));
    }
    #[test]
    fn a_modal_opened_from_a_notification_never_becomes_its_own_ancestor() {
        let mut tree = Hierarchy::default();
        let toast = tree.insert(None, false, true);
        let popup = tree.insert(Some(toast), false, false);
        let modal = tree.insert(Some(popup), true, false);
        assert_eq!(tree.parent(toast), None);
        assert_eq!(tree.parent(modal), Some(popup));
        assert_eq!(tree.top(), Some(modal));
        let other = tree.insert(Some(modal), false, true);
        assert_eq!(tree.parent(other), Some(modal));
        tree.remove(modal);
        assert_eq!(tree.parent(other), None);
        assert!(tree.contains(popup));
        assert_eq!(tree.parent(toast), None);
    }
    #[test]
    fn closing_a_host_preserves_a_notification_branch_containing_the_next_modal() {
        let mut tree = Hierarchy::default();
        let host = tree.insert(None, true, false);
        let toast = tree.insert(Some(host), false, true);
        let nested = tree.insert(Some(toast), true, false);
        assert_eq!(tree.parent(toast), Some(host));
        assert_eq!(tree.remove(host), vec![host]);
        assert_eq!(tree.parent(toast), None);
        assert_eq!(tree.top_modal(), Some(nested));
        assert_eq!(tree.top(), Some(nested));
    }
    #[test]
    fn nested_modal_branches_remain_visually_above_later_unrelated_roots() {
        let mut tree = Hierarchy::default();
        let parent = tree.insert(None, false, true);
        let unrelated = tree.insert(None, false, false);
        let modal = tree.insert(Some(parent), true, false);
        assert_eq!(tree.ordered(), &[unrelated, parent, modal]);
        assert_eq!(tree.top(), Some(modal));
    }
}
