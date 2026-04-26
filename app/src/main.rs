use std::sync::Arc;
use std::time::Duration;
use kube::{Api, Client, ResourceExt};
use kube::runtime::Controller;
use kube::runtime::controller::Action;
use log::{info, LevelFilter};
use simplelog::{TermLogger, TerminalMode};
use frp_operator_api::v1alpha1;
use futures_util::stream::StreamExt;
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto
    ).expect("Could not setup logging");

    let client = Client::try_default().await?;

    let context = Arc::new(()); // bad empty context - put client in here
    let api_client: Api<v1alpha1::Client> = Api::all(client);
    Controller::new(api_client, kube::runtime::watcher::Config::default().any_semantic())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => log::info!("reconciled {:?}", o),
                Err(e) => log::error!("reconcile failed: {:?}", e),
            }
        })
        .await;

    Ok(())
}

#[derive(Debug, Error)]
enum Error {}

/// The reconciler that will be called when either object change
async fn reconcile(g: Arc<v1alpha1::Client>, _ctx: Arc<()>) -> Result<Action, Error> {
    let pod = g.name_any();
    let ns = g.namespace().unwrap();
    info!("Reconciling pod {} in {}", pod, ns);
    Ok(Action::await_change())
}
/// an error handler that will be called when the reconciler fails with access to both the
/// object that caused the failure and the actual error
fn error_policy(obj: Arc<v1alpha1::Client>, _error: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
