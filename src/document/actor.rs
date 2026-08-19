/*
document/actor.rs — the async wrapper around crdt.rs. Owns a Document and a DocumentRepository, sits in a loop receiving DocMsgs, applies ops to the CRDT, persists to Mongo, broadcasts to sessions. This is the serialization point for all concurrent writes to a document.
*/

use tokio::sync::{mpsc, broadcast};
use std::sync::Arc;

use crate::types::{UserId, DocumentId, DocMsg, ClientMsg, Op};
use crate::document::crdt::Document;
use crate::repository::DocumentRepository;

pub struct DocumentActor {
    doc_id: DocumentId,
    doc: Document,
    repo: Arc<DocumentRepository>,
    doc_operation_rx: mpsc::Receiver<DocMsg>, // receives from Session
    client_broadcast_tx: broadcast::Sender<ClientMsg>, // transmits to Session
}

impl DocumentActor {
    pub async fn new(
        doc_id: DocumentId,
        doc_operation_rx: mpsc::Receiver<DocMsg>,
        client_broadcast_tx: broadcast::Sender<ClientMsg>,
        repo: Arc<DocumentRepository>,
    ) -> Self {

        // load document here. Probably will store materialized doc state instead of ops in db
        let mut doc = Document::new(doc_id);
        /*
        let ops = repo.load(doc_id).await.unwrap_or_default();
    

        for op in ops {
            doc.apply(&op);
        }
        */

        Self { doc_id, doc, repo, doc_operation_rx, client_broadcast_tx }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.doc_operation_rx.recv().await {
            match msg {
                _ => ()
            }
        }
    }
}
