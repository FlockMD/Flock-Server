use std::sync::mpsc::Sender;

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

/*
pub enum ClientMsg {
    OpApplied(Op),
    UserJoined(UserId),
    UserLeft(UserId),
    Error(String),
}
*/