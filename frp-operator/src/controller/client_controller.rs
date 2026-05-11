use crate::OperatorContext;
use crate::frp::{FrpClientConfig, FrpConfigResolvable};
use crate::resources::{ManagedClientConfigSecret, ManagedClientDeployment};
use frp_operator_api::v1alpha1;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Api, ResourceExt};
use log::{debug, info, warn};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientReconcileError {
    #[error(transparent)]
    Kube(#[from] kube::Error),
}

pub async fn reconcile(
    client: Arc<v1alpha1::Client>,
    context: Arc<OperatorContext>,
) -> Result<Action, ClientReconcileError> {
    let client_config = FrpClientConfig::resolve(client.as_ref().clone(), context.client.clone()).await?;

    // ensure client configuration is up to date
    let managed_secret = ManagedClientConfigSecret::from((client.as_ref(), client_config));
    let secrets: Api<Secret> = Api::namespaced(
        context.client.clone(),
        &client.namespace().unwrap_or_default(),
    );
    if let Some(existing_secret) = secrets
        .get_opt(&managed_secret.name().unwrap_or_default())
        .await?
        .map(ManagedClientConfigSecret::from)
    {
        // compare existing hash
        if managed_secret
            .config_hash()
            .eq(&existing_secret.config_hash())
        {
            debug!(
                "Config secret for client {} is up to date, skipping apply",
                client.name_any()
            );
            return Ok(Action::await_change());
        }

        secrets
            .patch(
                &managed_secret.name().unwrap_or_default(),
                &PatchParams::apply("frp-operator").force(),
                &Patch::<&Secret>::Apply(&managed_secret.clone().into()),
            )
            .await?;
        info!(
            "Updated config secret {} for {}",
            managed_secret.name().unwrap_or_default(),
            client.name_any()
        );
    } else {
        // just push new secret
        secrets
            .create(&Default::default(), &managed_secret.clone().into())
            .await?;
        info!(
            "Created config secret {} for {}",
            managed_secret.name().unwrap_or_default(),
            client.name_any()
        );
    }

    // ensure deployment is up to date
    let managed_deployment = ManagedClientDeployment::from((client.as_ref(), managed_secret));
    let deployments: Api<Deployment> = Api::namespaced(
        context.client.clone(),
        &client.namespace().unwrap_or_default(),
    );
    if let Some(existing_deployment) = deployments
        .get_opt(&managed_deployment.name().unwrap_or_default())
        .await?
        .map(ManagedClientDeployment::from)
    {
        // compare existing hash
        if managed_deployment
            .config_hash()
            .eq(&existing_deployment.config_hash())
        {
            debug!(
                "Deployment for client {} is up to date, skipping apply",
                client.name_any()
            );
            return Ok(Action::await_change());
        }

        deployments
            .patch(
                &managed_deployment.name().unwrap_or_default(),
                &PatchParams::apply("frp-operator").force(),
                &Patch::<&Deployment>::Apply(&managed_deployment.clone().into()),
            )
            .await?;
        info!(
            "Updated deployment {} for {}",
            managed_deployment.name().unwrap_or_default(),
            client.name_any()
        );
    } else {
        // just push new deployment
        deployments
            .create(&Default::default(), &managed_deployment.clone().into())
            .await?;
        info!(
            "Created deployment {} for {}",
            managed_deployment.name().unwrap_or_default(),
            client.name_any()
        );
    }

    Ok(Action::await_change())
}

pub fn error_policy(
    client: Arc<v1alpha1::Client>,
    error: &ClientReconcileError,
    _context: Arc<OperatorContext>,
) -> Action {
    warn!(
        "reconcile failed for client {}: {}",
        client.name_any(),
        error
    );
    Action::await_change()
}
