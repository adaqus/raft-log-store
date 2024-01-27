use actix_web::middleware;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::HttpServer;
use clap::Parser;
use network::Network;
use openraft::storage::Adaptor;
use openraft::Config;
use std::fmt::Display;
use std::io::Cursor;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

// use crate::store::rocksdbstore::RocksDbStore;
// use store::memstore::MemStore;
use crate::store::rocksdbstore::RocksDbStore;
use crate::store::Request;
use crate::store::Response;

pub type NodeId = u64;
// Currently used storage (RocksDbStore or MemStore)
pub type Store = RocksDbStore;

mod http_handler;
mod network;
mod store;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub addr: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ addr: {} }}", self.addr)
    }
}

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);

pub type LogStore = Adaptor<TypeConfig, Arc<Store>>;
pub type StateMachineStore = Adaptor<TypeConfig, Arc<Store>>;
pub type Raft = openraft::Raft<TypeConfig, Network, LogStore, StateMachineStore>;

pub mod types {
    use crate::Node;
    use crate::NodeId;
    use crate::TypeConfig;

    pub type RaftError<E = openraft::error::Infallible> = openraft::error::RaftError<NodeId, E>;
    pub type RPCError<E = openraft::error::Infallible> =
        openraft::error::RPCError<NodeId, Node, RaftError<E>>;

    pub type ClientWriteError = openraft::error::ClientWriteError<NodeId, Node>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<NodeId, Node>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<NodeId, Node>;
    pub type InitializeError = openraft::error::InitializeError<NodeId, Node>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

pub struct App {
    pub id: NodeId,
    pub addr: String,
    pub raft: Raft,
    pub store: Arc<Store>,
    pub config: Arc<Config>,
}

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,
}

#[actix_web::main]
async fn main() {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse the parameters passed by arguments.
    let options = Opt::parse();

    let node_id = options.id;
    let http_addr = options.http_addr;

    // Create a configuration for the raft instance.
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    // Create a instance of where the Raft data will be stored.
    let store = RocksDbStore::new(format!("./db-{node_id}")).await;

    let (log_store, state_machine) = Adaptor::new(store.clone());

    // Create the network layer that will connect and communicate the raft instances and
    // will be used in conjunction with the store created above.
    let network = Network::new();

    // Create a local raft instance.
    let raft = openraft::Raft::new(node_id, config.clone(), network, log_store, state_machine)
        .await
        .unwrap();

    // Create an application that will store all the instances created above, this will
    // be later used on the actix-web services.
    let app_data = Data::new(App {
        id: node_id,
        addr: http_addr.clone(),
        raft,
        store,
        config,
    });

    // Start the actix-web server.
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .app_data(app_data.clone())
            // raft internal RPC
            .service(http_handler::append)
            .service(http_handler::snapshot)
            .service(http_handler::vote)
            // admin API
            .service(http_handler::init)
            .service(http_handler::add_learner)
            .service(http_handler::change_membership)
            .service(http_handler::metrics)
            // application API
            .service(http_handler::write)
            .service(http_handler::read)
            .service(http_handler::consistent_read)
    });

    let x = server.bind(http_addr).unwrap();

    x.run().await.unwrap();
}
