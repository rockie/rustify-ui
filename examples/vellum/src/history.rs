use crate::document::{Document, DocumentData};
use std::collections::VecDeque;

const HISTORY_LIMIT: usize = 80;

#[derive(Clone, Debug)]
struct Entry {
    label: String,
    data: DocumentData,
}

impl Entry {
    fn capture(doc: &Document, label: &str) -> Self {
        // Asset strings stay shared while geometry and metadata form an independent snapshot.
        Self {
            label: label.into(),
            data: doc.data.clone(),
        }
    }

    fn apply(self, doc: &mut Document) -> String {
        doc.data = self.data;
        doc.refresh();
        self.label
    }
}

#[derive(Clone, Debug, Default)]
pub struct History {
    undo_stack: VecDeque<Entry>,
    redo_stack: Vec<Entry>,
    pending: Option<Entry>,
}

impl History {
    pub fn begin(&mut self, doc: &Document, label: &str) {
        if self.pending.is_none() {
            self.pending = Some(Entry::capture(doc, label));
        }
    }

    pub fn commit(&mut self, doc: &Document) -> bool {
        let Some(entry) = self.pending.take() else {
            return false;
        };
        if entry.data == doc.data {
            return false;
        }
        self.undo_stack.push_back(entry);
        if self.undo_stack.len() > HISTORY_LIMIT {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear();
        true
    }

    pub fn cancel(&mut self, doc: &mut Document) {
        if let Some(entry) = self.pending.take() {
            entry.apply(doc);
        }
    }

    /// Abandons an invalidated edit without overwriting the external document update.
    pub fn discard_pending(&mut self) {
        self.pending = None;
    }

    pub fn undo(&mut self, doc: &mut Document) -> Option<String> {
        self.commit(doc);
        let entry = self.undo_stack.pop_back()?;
        self.redo_stack.push(Entry::capture(doc, &entry.label));
        Some(entry.apply(doc))
    }

    pub fn redo(&mut self, doc: &mut Document) -> Option<String> {
        let entry = self.redo_stack.pop()?;
        self.undo_stack.push_back(Entry::capture(doc, &entry.label));
        Some(entry.apply(doc))
    }

    pub fn undo_len(&self) -> usize {
        self.undo_stack.len()
    }
    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }
    pub fn is_pending(&self) -> bool {
        self.pending.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Document, Node};
    use std::rc::Rc;

    #[test]
    fn unchanged_transaction_does_not_enter_history() {
        let doc = Document::empty("Original");
        let mut history = History::default();
        history.begin(&doc, "Edit");
        assert!(!history.commit(&doc));
        assert_eq!(history.undo_len(), 0);
    }

    #[test]
    fn retains_only_80_committed_steps() {
        let mut doc = Document::empty("0");
        let mut history = History::default();
        for i in 1..=81 {
            history.begin(&doc, "Rename");
            doc.data.name = i.to_string();
            history.commit(&doc);
        }
        assert_eq!(history.undo_len(), 80);
        for _ in 0..80 {
            assert_eq!(history.undo(&mut doc).as_deref(), Some("Rename"));
        }
        assert_eq!(doc.data.name, "1");
        assert!(history.undo(&mut doc).is_none());
    }

    #[test]
    fn cancel_restores_pending_geometry_and_assets() {
        let mut doc = Document::empty("Original");
        let id = doc.add(Node::new("rect"));
        let mut history = History::default();
        history.begin(&doc, "Move");
        doc.get_mut(&id).unwrap().x = 200.0;
        doc.data
            .assets
            .insert("temporary".into(), Rc::from("data:image/png;base64,new"));
        history.cancel(&mut doc);
        assert_eq!(doc.get(&id).unwrap().x, 0.0);
        assert!(doc.data.assets.is_empty());
        assert_eq!(history.undo_len(), 0);
    }

    #[test]
    fn undo_and_redo_restore_replaced_documents_and_asset_keys() {
        let mut doc = Document::empty("Original");
        let image: Rc<str> = Rc::from("data:image/png;base64,original");
        let font: Rc<str> = Rc::from("data:font/woff2;base64,font");
        doc.data.assets.insert("original".into(), Rc::clone(&image));
        doc.data.fonts.insert("Inter".into(), Rc::clone(&font));
        let original = doc.data.clone();
        let mut history = History::default();
        history.begin(&doc, "Open document");
        doc = Document::empty("Replacement");
        doc.data.assets.insert(
            "replacement".into(),
            Rc::from("data:image/png;base64,replacement"),
        );
        assert!(history.commit(&doc));
        let replacement = doc.data.clone();
        assert_eq!(history.undo(&mut doc).as_deref(), Some("Open document"));
        assert_eq!(doc.data, original);
        assert!(Rc::ptr_eq(&doc.data.assets["original"], &image));
        assert!(Rc::ptr_eq(&doc.data.fonts["Inter"], &font));
        assert_eq!(history.redo(&mut doc).as_deref(), Some("Open document"));
        assert_eq!(doc.data, replacement);
    }

    #[test]
    fn changing_only_an_asset_is_undoable() {
        let mut doc = Document::empty("Original");
        let mut history = History::default();
        history.begin(&doc, "Asset");
        doc.data
            .assets
            .insert("image".into(), Rc::from("data:image/png;base64,new"));
        assert!(history.commit(&doc));
        history.undo(&mut doc);
        assert!(doc.data.assets.is_empty());
    }

    #[test]
    fn nested_begin_retains_original_snapshot_and_commit_discards_redo() {
        let mut doc = Document::empty("Original");
        let mut history = History::default();
        history.begin(&doc, "Outer");
        doc.data.name = "Middle".into();
        history.begin(&doc, "Inner");
        doc.data.name = "Final".into();
        history.commit(&doc);
        assert_eq!(history.undo(&mut doc).as_deref(), Some("Outer"));
        assert_eq!(doc.data.name, "Original");
        assert_eq!(history.redo_len(), 1);
        history.begin(&doc, "New");
        doc.data.name = "New".into();
        history.commit(&doc);
        assert_eq!(history.redo_len(), 0);
    }

    #[test]
    fn undo_commits_pending_edit_before_reverting_it() {
        let mut doc = Document::empty("Original");
        let mut history = History::default();
        history.begin(&doc, "Rename");
        doc.data.name = "New".into();
        assert_eq!(history.undo(&mut doc).as_deref(), Some("Rename"));
        assert_eq!(doc.data.name, "Original");
    }

    #[test]
    fn discarded_text_edit_preserves_external_update_and_previous_history() {
        let mut doc = Document::empty("Original");
        let mut text = Node::new("text");
        text.text = "Initial text".into();
        let id = doc.add(text);
        let mut history = History::default();
        history.begin(&doc, "Rename document");
        doc.data.name = "Named".into();
        history.commit(&doc);

        history.begin(&doc, "Edit text");
        doc.get_mut(&id).unwrap().text = "External update".into();
        doc.touch(Some(&id));
        let external = doc.data.clone();
        history.discard_pending();
        history.cancel(&mut doc);
        assert_eq!(doc.data, external);
        assert!(!history.is_pending());
        assert!(!history.commit(&doc));
        assert_eq!(history.undo_len(), 1);

        history.begin(&doc, "Next edit");
        doc.get_mut(&id).unwrap().text = "Next edit".into();
        history.commit(&doc);
        history.undo(&mut doc);
        assert_eq!(doc.data, external);
    }
}
