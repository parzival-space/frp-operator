use std::sync::Arc;
use kube::ResourceExt;
use kube::runtime::controller::Action;
use log::{debug, warn};
use thiserror::Error;
use frp_operator_api::v1alpha1;

#[derive(Debug, Error)]
pub enum ClientReconcileError {
    #[error("client {0} is missing namespace")]
    MissingNamespace(String),

    #[error(transparent)]
    Kube(#[from] kube::Error),
}

pub async fn reconcile(client: Arc<v1alpha1::Client>, _ctx: Arc<()>) -> Result<Action, ClientReconcileError> {
    debug!("Reconciling client for {:?}", client.name_any());

    Ok(Action::await_change())
}

pub fn error_policy(client: Arc<v1alpha1::Client>, error: &ClientReconcileError, _ctx: Arc<()>) -> Action {
    warn!("reconcile failed for client {}: {}", client.name_any(), error);
    Action::await_change()
}