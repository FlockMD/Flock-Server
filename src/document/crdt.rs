use std::sync::Mutex;
use std::collections::{HashMap, BTreeSet};
use crate::types::{NodeId, Op};
use crate::document::lamport;

#[derive(Clone)]
struct Node {
    id: NodeId,
    // RGA anchor
    parent: Option<NodeId>,
    content: char,
    tombstone: bool,
}

pub struct Document {
    doc_id: u32,
    // source of truth
    ops: Vec<Op>,
    // optional cache
    lamport: lamport::Lamport,

    // full document state:
    nodes: Mutex<HashMap<NodeId, Node>>,
    // parent -> children
    children: Mutex<HashMap<Option<NodeId>, BTreeSet<NodeId>>>,
}

fn node_from_op(op: &Op, lamport: &lamport::Lamport) -> Result<Node, String> {
    match op {
        Op::Insert {
            request_time,
            user,
            content,
            parent,
        } => {
            let lamport_id = lamport.tick(*request_time);
            let id = NodeId {
                user: user.clone(),
                lamport_id,
            };

            Ok(Node {
                id,
                parent: parent.clone(),
                content: *content,
                tombstone: false,
            })
        },
        Op::Delete { id: _ } => {
            Err(String::from("Delete operation not supported to create new node"))
        }
    }

}

impl Document {
    fn insert(&mut self, node: &Node) {
        self.nodes.lock().unwrap()
            .insert(node.id.clone(), node.clone());

        self.children.lock().unwrap()
            .entry(node.parent.clone())
            .or_default()
            .insert(node.id.clone());

    }

    fn delete(&mut self, id: &NodeId) {
        // ideally we only lock the node we're currently 'deleting', but for ease of development for now we will lock entire structure
        let mut nodes = self.nodes.lock().unwrap();
        if let Some(node) = nodes.get_mut(id) {
            node.tombstone = true;
        };

    }

    /*
    we will move this to when message is received, so on reception we can 
    (if message is node write) create internal node representation, call on publisher
    to broadcast to other clients, and then proceed with insertion in server

    fn apply(&mut self, op: &Op)  {
        match op {
            Op::Insert { .. } => {

                let res = node_from_op(op, &(self.lamport));

                let node = match res {
                    Ok(node) => {
                        node
                    }
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                };

                
            }

            Op::Delete { id } => {
               
            }
        }
    }
    */

    /*

    fn render(&self) -> String {
        let mut out = String::new();

        fn walk(
            parent: Option<NodeId>,
            doc: &Document,
            out: &mut String,
        ) {
            if let Some(children) = doc.children.get(&parent) {
                for id in children {
                    let node = &doc.nodes[id];

                    if !node.tombstone {
                        out.push(node.content);
                    }

                    walk(Some(id.clone()), doc, out);
                }
            }
        }

        walk(None, self, &mut out);

        out
    }
     */
}