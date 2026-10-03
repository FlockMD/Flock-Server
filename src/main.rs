/*
main.rs — starts the tokio runtime, binds the WebSocket listener, and wires each connection up:
connection -> find or spawn the document's actor -> run a session against it.
*/
pub mod types;
pub mod repository;
pub mod session;
pub mod document;

use axum::{
    extract::{ws::WebSocketUpgrade, Query, State},
    response::Response,
    routing::get,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::document::actor::{self, DocumentHandle};
use crate::repository::DocumentRepository;
use crate::types::DocumentId;

struct AppState {
    docs: Mutex<HashMap<DocumentId, DocumentHandle>>,
    repo: Arc<DocumentRepository>,
}

impl AppState {
    fn get_or_spawn(&self, doc_id: DocumentId) -> DocumentHandle {
        self.docs
            .lock()
            .unwrap()
            .entry(doc_id)
            .or_insert_with(|| actor::spawn(doc_id, Arc::clone(&self.repo)))
            .clone()
    }
}

// TODO: replace with real auth once it exists — for now the client just
// tells us who it is and which document it wants.
#[derive(Deserialize, Clone, Copy)]
struct Connect {
    user_id: u64,
    doc_id: u64,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        docs: Mutex::new(HashMap::new()),
        repo: Arc::new(DocumentRepository::new()),
    });
    let app = axum::Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(conn): Query<Connect>,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| async move {
        let handle = state.get_or_spawn(conn.doc_id);
        session::run(socket, handle).await;
    })
}
