// use openraft::AppData;
// use openraft::AppDataResponse;
// use serde::{Serialize, Deserialize};
// use std::io::Cursor;
// use openraft::BasicNode;
// use openraft::Config;
// use openraft::RaftTypeConfig;

use std::io::Cursor;

// use openraft::storage::Adaptor;
use openraft::BasicNode;

use crate::memstore::Request;
use crate::memstore::Response;

pub type NodeId = u64;

mod memstore;
mod network;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = BasicNode,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);

pub mod types {
    use openraft::BasicNode;

    use crate::NodeId;
    use crate::TypeConfig;

    pub type RaftError<E = openraft::error::Infallible> = openraft::error::RaftError<NodeId, E>;
    pub type RPCError<E = openraft::error::Infallible> =
        openraft::error::RPCError<NodeId, BasicNode, RaftError<E>>;

    pub type ClientWriteError = openraft::error::ClientWriteError<NodeId, BasicNode>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<NodeId, BasicNode>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<NodeId, BasicNode>;
    pub type InitializeError = openraft::error::InitializeError<NodeId, BasicNode>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

fn main() {
    println!("Hello, world!");
}
