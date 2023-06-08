use actix_web::get;
use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::Responder;
use openraft::error::CheckIsLeaderError;
use openraft::error::Infallible;
use openraft::error::RaftError;
use openraft::raft::AppendEntriesRequest;
use openraft::raft::InstallSnapshotRequest;
use openraft::raft::VoteRequest;
use openraft::BasicNode;
use openraft::RaftMetrics;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::memstore::Request;
use crate::App;
use crate::NodeId;
use crate::TypeConfig;

#[post("/add-learner")]
pub async fn add_learner(
    app: Data<App>,
    req: Json<(NodeId, String)>,
) -> actix_web::Result<impl Responder> {
    let node_id = req.0 .0;
    let node = BasicNode {
        addr: req.0 .1.clone(),
    };
    let res = app.raft.add_learner(node_id, node, true).await;
    Ok(Json(res))
}

/// Changes specified learners to members, or remove members.
#[post("/change-membership")]
pub async fn change_membership(
    app: Data<App>,
    req: Json<BTreeSet<NodeId>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.change_membership(req.0, false).await;
    Ok(Json(res))
}

/// Initialize a single-node cluster.
#[post("/init")]
pub async fn init(app: Data<App>) -> actix_web::Result<impl Responder> {
    let mut nodes = BTreeMap::new();
    nodes.insert(
        app.id,
        BasicNode {
            addr: app.addr.clone(),
        },
    );
    let res = app.raft.initialize(nodes).await;
    Ok(Json(res))
}

/// Get the latest metrics of the cluster
#[get("/metrics")]
pub async fn metrics(app: Data<App>) -> actix_web::Result<impl Responder> {
    let metrics = app.raft.metrics().borrow().clone();

    let res: Result<RaftMetrics<NodeId, BasicNode>, Infallible> = Ok(metrics);
    Ok(Json(res))
}

#[post("/raft-vote")]
pub async fn vote(
    app: Data<App>,
    req: Json<VoteRequest<NodeId>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.vote(req.0).await;
    Ok(Json(res))
}

#[post("/raft-append")]
pub async fn append(
    app: Data<App>,
    req: Json<AppendEntriesRequest<TypeConfig>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.append_entries(req.0).await;
    Ok(Json(res))
}

#[post("/raft-snapshot")]
pub async fn snapshot(
    app: Data<App>,
    req: Json<InstallSnapshotRequest<TypeConfig>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.install_snapshot(req.0).await;
    Ok(Json(res))
}

#[post("/write")]
pub async fn write(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let response = app.raft.client_write(req.0).await;
    Ok(Json(response))
}

#[post("/read")]
pub async fn read(app: Data<App>, req: Json<String>) -> actix_web::Result<impl Responder> {
    let state_machine = app.store.state_machine.read().await;
    let key = req.0;
    let value = state_machine.data.get(&key).cloned();

    let res: Result<String, Infallible> = Ok(value.unwrap_or_default());
    Ok(Json(res))
}

#[post("/consistent_read")]
pub async fn consistent_read(
    app: Data<App>,
    req: Json<String>,
) -> actix_web::Result<impl Responder> {
    let ret = app.raft.is_leader().await;

    match ret {
        Ok(_) => {
            let state_machine = app.store.state_machine.read().await;
            let key = req.0;
            let value = state_machine.data.get(&key).cloned();

            let res: Result<String, RaftError<NodeId, CheckIsLeaderError<NodeId, BasicNode>>> =
                Ok(value.unwrap_or_default());
            Ok(Json(res))
        }
        Err(e) => Ok(Json(Err(e))),
    }
}
