/*
session.rs — one instance per WebSocket connection. Forwards edits from the client to the document actor, and forwards ops the actor broadcasts back to the client.
*/
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures::{StreamExt, SinkExt};
use log::error;

use crate::document::actor::DocumentHandle;
use crate::types::DocMsg;

pub async fn run(socket: WebSocket, handle: DocumentHandle) {
    let mut client_broadcast_rx = handle.client_broadcast_tx.subscribe();
    let (mut ws_tx, mut ws_rx) = socket.split();

    loop {
        tokio::select! {
            from_client = ws_rx.next() => {
                let text = match from_client {
                    Some(Ok(Message::Text(text))) => text,
                    Some(Ok(_)) => continue,
                    Some(Err(_)) | None => break,
                };
                match serde_json::from_str::<DocMsg>(text.as_str()) {
                    Ok(msg @ DocMsg::Edit { .. }) => {
                        let _ = handle.doc_operation_tx.send(msg).await;
                    }
                    Ok(_) => {}
                    Err(e) => error!("Invalid message from client: {e}"),
                }
            }
            from_actor = client_broadcast_rx.recv() => {
                let Ok(msg) = from_actor else { break };
                let Ok(text) = serde_json::to_string(&msg) else { continue };
                if ws_tx.send(Message::Text(Utf8Bytes::from(text))).await.is_err() {
                    break;
                }
            }
        }
    }
}
