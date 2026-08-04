/*
use std::sync::mpsc::Sender;
use tokio::sync::broadcast;

pub type UserId = u64;
pub type DocumentId = u64;
pub enum DocMsg {
    Join { user: UserId, reply: Sender<broadcast::Receiver<ClientMsg>> },
    Leave { user: UserId },
    Edit { user: UserId, op: Op },
    Save { user: UserId },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId {
    pub user: UserId,
    pub lamport_id: u64,
}

#[derive(Clone, Debug)]
pub enum Op {
    // maybe make insert take in just the value and parent, and have document produce the id itself
    Insert {
        request_time: u64,
        user: UserId,
        content: char,
        parent: Option<NodeId>,
    },

    Delete {
        id: NodeId,
    },
}

#[derive(Clone, Debug)]
pub enum ClientMsg {
    OpApplied(Op),
    UserJoined(UserId),
    UserLeft(UserId),
    Error(String),
}
*/

/*
use std::sync::mpsc::Sender;
use tokio::sync::broadcast;



#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId {
    pub user: UserId,
    pub lamport_id: u64,
}

*/
use tokio::sync::{oneshot, broadcast};
use serde::{Serialize, Deserialize};

pub type UserId = u64;
pub type DocumentId = u64;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId {
    pub user: UserId,
    pub lamport_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Op {
    // maybe make insert take in just the value and parent, and have document produce the id itself
    Insert {
        request_time: u64,
        user: UserId,
        content: char,
        parent: Option<NodeId>,
    },

    Delete {
        id: NodeId,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub content: char,
    pub tombstone: bool,
}

pub enum DocMsg {
    Join {
        user: UserId,
        reply: oneshot::Sender<broadcast::Receiver<ClientMsg>>,
    },
    Leave { user: UserId },
    Edit  { user: UserId, op: Op },
    Save  { user: UserId },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    OpApplied(Op),
    UserJoined(UserId),
    UserLeft(UserId),
    Error(String),
}