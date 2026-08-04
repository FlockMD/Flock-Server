/*
session.rs — one instance per WebSocket connection. Two tasks in a select! loop: WS bytes → parse → send DocMsg to the document actor, and broadcast messages from the actor → serialize → send to WS. This is what you currently call document_controller, but the logic is much simpler once the actor owns the document state.
*/
use axum::extract::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use tokio::sync::{mpsc, broadcast, oneshot};
use std::sync::Arc;

use crate::types::{UserId, DocumentId, DocMsg, ClientMsg, Op};
use crate::router::Router;

pub struct Session {
    pub socket: WebSocket,
    pub router: Arc<Router>,
    pub user_id: UserId,
    pub doc_id: DocumentId,
}

impl Session {
    pub async fn run(self) {
        let doc_tx = self.router.get_or_spawn(self.doc_id).await;

        // ask the actor for a broadcast receiver
        let (reply_tx, reply_rx) = oneshot::channel();
        let _ = doc_tx.send(DocMsg::Join {
            user: self.user_id.clone(),
            reply: reply_tx,
        }).await;
        let mut broadcast_rx = reply_rx.await.unwrap();

        let (mut ws_tx, mut ws_rx) = self.socket.split();

        loop {
            tokio::select! {
                msg = ws_rx.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            match serde_json::from_str::<Op>(&text) {
                                Ok(op) => {
                                    let _ = doc_tx.send(DocMsg::Edit {
                                        user: self.user_id.clone(),
                                        op,
                                    }).await;
                                }
                                Err(_) => {
                                    // malformed op — could send ClientMsg::Error back
                                }
                            }
                        }

                        // clean disconnect or socket dropped
                        Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                            let _ = doc_tx.send(DocMsg::Leave {
                                user: self.user_id.clone(),
                            }).await;
                            break;
                        }

                        // axum handles Ping/Pong automatically, safe to ignore
                        Some(Ok(_)) => {}
                    }
                }

                result = broadcast_rx.recv() => {
                    match result {
                        Ok(client_msg) => {
                            let text = serde_json::to_string(&client_msg).unwrap();
                            if ws_tx.send(Message::Text(text)).await.is_err() {
                                let _ = doc_tx.send(DocMsg::Leave {
                                    user: self.user_id.clone(),
                                }).await;
                                break;
                            }
                        }

                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            // this session fell behind by n messages
                            // for a CRDT you can recover by sending a full
                            // snapshot of the current document state
                        }

                        Err(broadcast::error::RecvError::Closed) => {
                            // document actor shut down
                            break;
                        }
                    }
                }
            }
        }
    }
}