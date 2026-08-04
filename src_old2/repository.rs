/*
repository.rs — thin async wrapper around MongoDB. Two methods: load(doc_id) -> Vec<Op> and append_op(op). Nothing else. This is what you currently call document_model.
*/

use mongodb::Collection;

pub struct DocumentRepository {
    collection: Collection<OpRecord>,
}

impl DocumentRepository {
    pub async fn load(&self, doc_id: u32) -> Vec<Op> { 

     }
    pub async fn append_op(&self, op: &Op) -> Result<()> { 

    }
}