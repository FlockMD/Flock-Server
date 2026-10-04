/*
document/crdt.rs — the CRDT itself: an RGA-style tree of nodes, kept as
plain (non-atomic, non-mutex) state.

A Document is only ever mutated from inside its owning DocumentActor's
single-threaded message loop, so there is exactly one writer and no need
for interior mutability here. All synchronization concerns belong to the
actor, not to the data structure.
*/
use std::collections::{BTreeSet, HashMap};

use crate::document::lamport::Lamport;
use crate::types::{DocumentId, Node, NodeId, Op};

pub struct Document {
    doc_id: DocumentId,
    lamport: Lamport,
    nodes: HashMap<NodeId, Node>,
    // parent -> children, ordered so render() walks them in CRDT order
    children: HashMap<Option<NodeId>, BTreeSet<NodeId>>,
}

impl Document {
    pub fn new(doc_id: DocumentId) -> Self {
        Self {
            doc_id,
            lamport: Lamport::new(),
            nodes: HashMap::new(),
            children: HashMap::new(),
        }
    }

    /// Applies one operation, mutating local state in place
    /// 
    /// # Arguments
    ///
    /// * `op` - The operation to apply
    ///
    /// # Returns
    ///
    /// The operation that was applied, including the new node id if it was modified (by lamport clock, etc.)
    pub fn apply(&mut self, op: &Op) -> Option<Op> {
        match op {
            Op::Insert { request_time, user, content, parent } => {
                let lamport_id = self.lamport.tick(*request_time);
                let id = NodeId { lamport_id, user: *user };
                let node = Node {
                    id: id.clone(),
                    parent: parent.clone(),
                    content: *content,
                    tombstone: false,
                };

                self.children.entry(parent.clone()).or_default().insert(id.clone());
                self.nodes.insert(id, node);
                Some(
                    Op::Insert {
                        request_time: *request_time,
                        user: *user,
                        content: *content,
                        parent: parent.clone(),
                    }
                )
            }
            Op::Delete { id } => {
                if let Some(node) = self.nodes.get_mut(id) {
                    node.tombstone = true;
                }
                Some(op.clone())
            }
        }
    }

    fn walk(&self) -> String {
        let mut out = String::new();
        let mut stack: Vec<NodeId> = Vec::new();

        // By pushing in ascending order, we'll pop the child with the largest lamport clock id first
        if let Some(roots) = self.children.get(&None) {
            stack.extend(roots.iter().copied());
        }

        while let Some(id) = stack.pop() {
            let node = &self.nodes[&id];
            if !node.tombstone {
                out.push(node.content);
            }
            if let Some(children) = self.children.get(&Some(id)) {
                stack.extend(children.iter().copied());
            }
        }
        out
    }
}