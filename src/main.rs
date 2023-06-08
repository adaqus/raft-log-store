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

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = BasicNode,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);


fn main() {
    println!("Hello, world!");
}
