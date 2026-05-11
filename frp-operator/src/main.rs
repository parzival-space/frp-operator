mod controller;
mod frp;
mod resources;

use frp_operator_api::v1alpha1;
use futures_util::stream::StreamExt;
use kube::runtime::Controller;
use kube::runtime::watcher::Config;
use kube::{Api, Client};
use log::LevelFilter;
use simplelog::{TermLogger, TerminalMode};
use std::sync::Arc;
use kube::runtime::reflector::ObjectRef;

#[derive(Clone)]
pub struct OperatorContext {
    pub client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Could not setup logging");

    let kube = Client::try_default().await?;
    let context = Arc::new(OperatorContext {
        client: kube.clone(),
    }); // bad empty context - put client in here

    let client_api: Api<v1alpha1::Client> = Api::all(kube.clone());
    let tunnel_api: Api<v1alpha1::Tunnel> = Api::all(kube.clone());
    // todo: add visitor api
    
    tokio::join!(
        // v1alpha1::Client, v1alpha1::Tunnel
        Controller::new(client_api, Config::default().any_semantic())
            .watches(
                tunnel_api,
                Config::default().any_semantic(),
                |tunnel: v1alpha1::Tunnel| {
                    // map tunnel changes to client to trigger client reconcile
                    vec![
                        ObjectRef::new(&tunnel.spec.client_ref.name)
                            .within(&tunnel.spec.client_ref.namespace)
                    ]
                }
            )
            .shutdown_on_signal()
            .run(
                controller::client_controller::reconcile,
                controller::client_controller::error_policy,
                context.clone()
            )
            .for_each(|res| async move {
                match res {
                    Ok(o) => log::info!("reconciled {:?}", o),
                    Err(e) => log::error!("reconcile failed: {:?}", e),
                }
            })
    );
    Ok(())
}
