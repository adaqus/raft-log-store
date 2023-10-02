use openraft::SnapshotMeta;
use serde::Deserialize;
use serde::Serialize;

use crate::Node;
use crate::NodeId;

// pub mod memstore;
pub mod rocksdbstore;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Set { key: String, value: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredSnapshot {
    pub meta: SnapshotMeta<NodeId, Node>,

    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}
