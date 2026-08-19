/* 
router.rs — owns a HashMap<DocumentId, mpsc::Sender<DocMsg>>. When a new connection arrives, it finds or spawns the right DocumentActor and hands the sender to the new Session. This is the useful core of connection_manager.
*/
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};

use crate::types::{ClientMsg, DocMsg, DocumentId};
use crate::document::actor::DocumentActor;
use crate::repository::DocumentRepository;


// this will be sent to session: session needs to send messages to the client (those messages are received by subscribing to client_broadcast_tx)
// session also needs to be able to send actor messages
#[derive(Clone)]
pub struct DocumentHandle {
    pub doc_operation_tx: mpsc::Sender<DocMsg>, // multiple sessions producing document operations
    pub client_broadcast_tx: broadcast::Sender<ClientMsg>
}

pub struct Router {
    docs: Mutex<HashMap<DocumentId, DocumentHandle>>,
    repo: Arc<DocumentRepository>,
}

impl Router {
    pub fn new(repo: Arc<DocumentRepository>) -> Self {
        Self {
            docs: Mutex::new(HashMap::new()),
            repo,
        }
    }

    pub async fn get_or_spawn(&self, doc_id: DocumentId) -> DocumentHandle {
        // check under lock, drop lock before awaiting
        {
            let docs = self.docs.lock().unwrap();
            if let Some(handle) = docs.get(&doc_id) {
                return handle.clone();
            }
        }
        
        let (doc_operation_tx, doc_operation_rx) = mpsc::channel(256);
        let (client_broadcast_tx, _) = broadcast::channel(256);
        // actor has client_broadcast_tx, session has client_broadcast_rx
        // hand client_broadcast_tx to actor, return client_broadcast_tx from this function inside the document handle so that session can subscribe to it
        let mut actor = DocumentActor::new(doc_id, doc_operation_rx, client_broadcast_tx.clone(), Arc::clone(&self.repo)).await;

        let handle = DocumentHandle {
            doc_operation_tx,
            client_broadcast_tx
        };

        tokio::spawn(async move { actor.run().await });

        self.docs.lock().unwrap().insert(doc_id, handle.clone());
        handle
    }
}