/*
document/actor.rs — the async wrapper around crdt.rs. Owns a Document and a DocumentRepository, sits in a loop receiving DocMsgs, applies ops to the CRDT, persists to Mongo, broadcasts to sessions. This is the serialization point for all concurrent writes to a document.
*/

use std::sync::mpsc;
use crate::document::crdt::Document;
use crate::types::{DocMsg};
use crate::repository::DocumentRepository;



pub struct DocumentActor {
    doc: Document,
    repo: DocumentRepository,
    rx: mpsc::Receiver<DocMsg>,
    broadcast: broadcast::Sender<ClientMsg>,
}

impl DocumentActor {
    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                DocMsg::Edit { user, op } => {
                    self.doc.apply(&op);
                    self.repo.append_op(&op).await;
                    let _ = self.broadcast.send(ClientMsg::OpApplied(op));
                }
                DocMsg::Join { user, reply } => {
                    let _ = reply.send(self.broadcast.subscribe());
                }
                // ...
            }
        }
    }
}