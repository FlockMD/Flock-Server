/*
main.rs — starts the tokio runtime, binds the TCP/WebSocket listener, creates the Router, hands off each incoming connection.
*/

use axum::{Router, routing::get, extract::{State, ws::{WebSocketUpgrade, WebSocket}}, response::Response};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let router = Arc::new(crate::router::Router::new());
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(router): State<Arc<crate::router::Router>>,
) -> Response {
    ws.on_upgrade(|socket| async move {
        crate::session::Session::new(socket, router).run().await;
    })
}