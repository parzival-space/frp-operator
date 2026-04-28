use std::sync::Arc;
use k8s_openapi::api::core::v1::Secret;
use kube::{Api, ResourceExt};
use kube::api::{Patch, PatchParams};
use kube::runtime::controller::Action;
use log::{debug, info, warn};
use thiserror::Error;
use frp_operator_api::v1alpha1;
use crate::frp::config::{FrpClientConfig, FrpConfigResolvable};
use crate::frp::resources::ManagedClientConfigSecret;
use crate::OperatorContext;

#[derive(Debug, Error)]
pub enum ClientReconcileError {
    #[error(transparent)]
    Kube(#[from] kube::Error),

    #[error("client {0} is missing namespace")]
    MissingNamespace(String),
}

pub async fn reconcile(client: Arc<v1alpha1::Client>, context: Arc<OperatorContext>) -> Result<Action, ClientReconcileError> {
    debug!("Reconciling client for {:?}", client.name_any());

    // step 1: resolve frp client config
    let client_config = FrpClientConfig::kube_from(client.spec.clone(), context.client.clone()).await?;

    // step 2: build desired managed secret from client + rendered config
    let managed_secret = ManagedClientConfigSecret::from((client.as_ref(), client_config));
    let secret_name = managed_secret.name().unwrap_or_default().to_string();
    let desired_hash = managed_secret.config_hash().map(str::to_string);

    // step 3: compare existing hash and apply only on drift
    let namespace = client
        .namespace()
        .ok_or_else(|| ClientReconcileError::MissingNamespace(client.name_any()))?;
    let secrets: Api<Secret> = Api::namespaced(context.client.clone(), &namespace);
    let existing_hash = secrets
        .get_opt(&secret_name)
        .await?
        .map(ManagedClientConfigSecret::from)
        .and_then(|secret| secret.config_hash().map(str::to_string));

    if existing_hash == desired_hash {
        info!(
            "Config secret {} is up to date for {}",
            secret_name,
            client.name_any()
        );
        return Ok(Action::await_change());
    }

    let secret: Secret = managed_secret.into();
    secrets
        .patch(
            &secret_name,
            &PatchParams::apply("frp-operator").force(),
            &Patch::Apply(&secret),
        )
        .await?;
    info!("Applied config secret {} for {}", secret_name, client.name_any());

    Ok(Action::await_change())
}

pub fn error_policy(client: Arc<v1alpha1::Client>, error: &ClientReconcileError, _context: Arc<OperatorContext>) -> Action {
    warn!("reconcile failed for client {}: {}", client.name_any(), error);
    Action::await_change()
}