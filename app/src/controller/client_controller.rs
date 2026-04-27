use std::sync::Arc;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use kube::ResourceExt;
use kube::runtime::controller::Action;
use log::{debug, info, warn};
use thiserror::Error;
use frp_operator_api::v1alpha1;
use crate::frp::{FrpClientConfig, FrpConfigResolvable};
use crate::OperatorContext;

#[derive(Debug, Error)]
pub enum ClientReconcileError {
    #[error("client {0} is missing namespace")]
    MissingNamespace(String),

    #[error(transparent)]
    Kube(#[from] kube::Error),
}

pub async fn reconcile(client: Arc<v1alpha1::Client>, context: Arc<OperatorContext>) -> Result<Action, ClientReconcileError> {
    debug!("Reconciling client for {:?}", client.name_any());

    let client_config = FrpClientConfig::kube_from(client.spec.clone(), context.client.clone()).await?;
    let client_config_text = toml::to_string(&client_config).unwrap();
    info!("Generated FRP client config for {}:\n{}", client.name_any(), client_config_text);

    // step 1: read info from client
    // step 2: request all tunnel resources for this client
    // step 3: render FRP client config
    // step 4: check for existing deployment
    // step 4.1: compare hash
    // step 5: update deployment


    Ok(Action::await_change())
}

pub fn error_policy(client: Arc<v1alpha1::Client>, error: &ClientReconcileError, context: Arc<OperatorContext>) -> Action {
    warn!("reconcile failed for client {}: {}", client.name_any(), error);
    Action::await_change()
}