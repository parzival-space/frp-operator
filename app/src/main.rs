mod controller;
mod frp;

use std::sync::Arc;
use frp_operator_api::v1alpha1;
use futures_util::stream::StreamExt;
use kube::runtime::Controller;
use kube::{Api, Client};
use kube::runtime::watcher::Config;
use log::LevelFilter;
use simplelog::{TermLogger, TerminalMode};

#[derive(Clone)]
pub struct OperatorContext {
    pub client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto
    ).expect("Could not setup logging");

    let kube = Client::try_default().await?;
    let context = Arc::new(OperatorContext { client: kube.clone() }); // bad empty context - put client in here

    let client_api: Api<v1alpha1::Client> = Api::all(kube);
    // todo: add tunnel api
    // todo: add visitor api

    tokio::join!(
        Controller::new(client_api, Config::default().any_semantic())
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
