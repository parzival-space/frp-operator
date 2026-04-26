use futures_util::stream::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{Api, Client, CustomResource, ResourceExt};
use kube::api::ListParams;
use kube::runtime::{finalizer, Controller};
use kube::runtime::controller::Action;
use kube::runtime::finalizer::Event;
use kube::runtime::watcher::Config;
use log::{info, warn, LevelFilter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, SimpleLogger, TermLogger, TerminalMode};
use thiserror::Error;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "frp.parzival.space", kind = "FrpClient", version = "v1", namespaced)]
pub struct FrpClientSpec {
    pub name: String,
    pub server_addr: String,
    pub token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).expect("Failed to initialize logger");

    let client = Client::try_default().await?;

    let context = Arc::new(()); // bad empty context - put client in here
    let docs:  Api::<FrpClient> = Api::all(client);
    Controller::new(docs, Config::default().any_semantic())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => info!("reconcile failed: {:?}", e),
            }
        })
        .await;

    Ok(())
}

#[derive(Debug, Error)]
enum Error {}

/// The reconciler that will be called when either object change
async fn reconcile(g: Arc<FrpClient>, _ctx: Arc<()>) -> Result<Action, Error> {
    let pod = g.name_any();
    let ns = g.namespace().unwrap();
    info!("Reconciling pod {} in {}", pod, ns);
    Ok(Action::await_change())
}
/// an error handler that will be called when the reconciler fails with access to both the
/// object that caused the failure and the actual error
fn error_policy(obj: Arc<FrpClient>, _error: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(60))
}