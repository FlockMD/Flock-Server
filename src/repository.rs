/*
repository.rs — thin async wrapper around MongoDB. Two methods: load(doc_id) -> Vec<Op> and save()
*/

use mongodb::Collection;

use crate::{document::crdt::Document, types::DocumentId};

pub struct DocumentRepository {
    // contains mongodb connection fields
}

impl DocumentRepository {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn load(&self, doc_id: DocumentId) -> Document {
        Document::new(doc_id)
    }

    pub fn save(&self, doc_id: DocumentId, doc: &Document) {
        return
    }
}