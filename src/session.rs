/*
session.rs — one instance per WebSocket connection. Two tasks in a select! loop: WS bytes → parse → send DocMsg to the document actor, and broadcast messages from the actor → serialize → send to WS. This is what you currently call document_controller, but the logic is much simpler once the actor owns the document state.
*/
use axum::extract::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use tokio::sync::{oneshot, broadcast};

use std::sync::Arc;
use serde_json;

use crate::types::{UserId, DocumentId, DocMsg, ClientMsg, Op};
use crate::router::Router;

pub struct Session {
    pub socket: WebSocket,
    pub router: Arc<Router>,
    pub user_id: UserId,
    pub doc_id: DocumentId,
}

impl Session {
    pub fn new(
        socket: WebSocket,
        router: Arc<Router>,
        user_id: UserId,
        doc_id: DocumentId,
    ) -> Self {
        Self { socket, router, user_id, doc_id }
    }

    pub async fn run(self) {
        let handle = self.router.get_or_spawn(self.doc_id).await;
        let client_broadcast_rx = handle.client_broadcast_tx.subscribe();


        let (mut ws_tx, mut ws_rx) = self.socket.split(); // our actual channel of communication with client

        loop { // receive messages in a loop and hand off to appropriate task-manager
            tokio::select! {
                msg_from_client = ws_rx.next() => match msg_from_client {
                    Some(Ok(Message::Text(msg))) => match msg {
                        // parse from utf8 to struct via serde first
                        DocMsg::Edit { user, op } => {
                            // hand off to actor
                        },
                        _ => {}
                    },
                    Some(Err(e)) => {},
                    _ => {}
                },
                msg_from_actor = client_broadcast_rx.recv() => match msg_from_actor {
                    Ok(msg) => {
                        // hand back to actor, which will apply changes and broadcast necessary info to all other clients
                    },
                    _ => {}

                }

            }
        }

    }
}