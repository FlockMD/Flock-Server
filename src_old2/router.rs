/* 
router.rs — owns a HashMap<DocumentId, mpsc::Sender<DocMsg>>. When a new connection arrives, it finds or spawns the right DocumentActor and hands the sender to the new Session. This is the useful core of connection_manager.
*/

pub struct Router {
    docs: HashMap<DocumentId, mpsc::Sender<DocMsg>>,
}

impl Router {
    pub async fn handle_connection(&self, conn: Connection) {
        let doc_id = /* from handshake */;
        let doc_tx = self.docs.get(&doc_id).unwrap().clone();

        // subscribe to broadcasts
        let (reply_tx, reply_rx) = oneshot::channel();
        doc_tx.send(DocMsg::Join { user, reply: reply_tx }).await;
        let mut broadcast_rx = reply_rx.await.unwrap();

        // spawn two tasks: read from WS --> DocMsg, read broadcast → WS
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(ws_msg) = ws_stream.next() => {
                        let op = parse_op(ws_msg);
                        doc_tx.send(DocMsg::Edit { user, op }).await;
                    }
                    Ok(client_msg) = broadcast_rx.recv() => {
                        ws_stream.send(serialize(client_msg)).await;
                    }
                }
            }
        });
    }
}