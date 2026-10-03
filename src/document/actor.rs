/*
document/actor.rs — the async wrapper around crdt.rs. Owns a Document and a DocumentRepository, sits in a loop receiving DocMsgs, applies ops to the CRDT, persists to Mongo, broadcasts to sessions. This is the serialization point for all concurrent writes to a document.
*/

use tokio::sync::{mpsc, broadcast};
use std::sync::Arc;

use crate::types::{DocumentId, DocMsg, ClientMsg, Op};
use crate::document::crdt::Document;
use crate::repository::DocumentRepository;

/// What a session holds to talk to a running actor: send ops in, subscribe to applied ops out.
#[derive(Clone)]
pub struct DocumentHandle {
    pub doc_operation_tx: mpsc::Sender<DocMsg>,
    pub client_broadcast_tx: broadcast::Sender<ClientMsg>,
}

struct DocumentActor {
    doc_id: DocumentId,
    doc: Document,
    repo: Arc<DocumentRepository>,
    doc_operation_rx: mpsc::Receiver<DocMsg>,
    client_broadcast_tx: broadcast::Sender<ClientMsg>,
}

/// Loads the document, starts its actor task, and returns a handle to it.
pub fn spawn(doc_id: DocumentId, repo: Arc<DocumentRepository>) -> DocumentHandle {
    let (doc_operation_tx, doc_operation_rx) = mpsc::channel(256);
    let (client_broadcast_tx, _) = broadcast::channel(256);

    let actor = DocumentActor {
        doc_id,
        doc: repo.load(doc_id),
        repo,
        doc_operation_rx,
        client_broadcast_tx: client_broadcast_tx.clone(),
    };
    tokio::spawn(actor.run());

    DocumentHandle { doc_operation_tx, client_broadcast_tx }
}

impl DocumentActor {
    async fn run(mut self) {
        while let Some(msg) = self.doc_operation_rx.recv().await {
            match msg {
                DocMsg::Edit { op, .. } => {
                    // can't broadcast the same op out, need to update node id in the case of a write (since lamport clock takes max of request time and current counter)
                    let new_op: Op = self.doc.apply(&op).unwrap_or(op);
                    let _ = self.client_broadcast_tx.send(ClientMsg::OpApplied(new_op));
                }
                DocMsg::Save { .. } => {
                    // should there be another mpsc for repo to enqueue save operations?
                    self.repo.save(self.doc_id, &self.doc)
                }
                DocMsg::Leave { .. } => {}
            }
        }
    }
}
