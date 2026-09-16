use crate::document::{Document, DocumentError};

pub fn make_starter() -> Result<Document, DocumentError> {
    Document::parse(include_str!("../../../tests/vellum/fixtures/forma.vellum"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn starter_keeps_forma_page_layer_counts() {
        let doc = super::make_starter().unwrap();
        assert_eq!(
            doc.data
                .pages
                .iter()
                .map(|page| page.nodes.len())
                .collect::<Vec<_>>(),
            [171, 31, 0]
        );
    }
}
