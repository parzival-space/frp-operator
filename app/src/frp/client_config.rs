use crate::frp::{FrpClientAuthConfig, FrpClientProxy, FrpConfigResolvable};
use futures_util::{stream, StreamExt, TryStreamExt};
use frp_operator_api::v1alpha1;
use kube::api::ListParams;
use kube::{Api, Client, Error, ResourceExt};
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrpClientConfig {
    #[serde(rename = "serverAddr")]
    pub server_address: String,
    #[serde(rename = "serverPort")]
    pub server_port: u16,
    pub auth: FrpClientAuthConfig,

    pub proxies: Vec<FrpClientProxy>,
}

impl FrpConfigResolvable<v1alpha1::Client> for FrpClientConfig {
    async fn resolve(value: v1alpha1::Client, client: Client) -> Result<FrpClientConfig, Error> {
        let client_name = value.name_any();
        let client_namespace = value.namespace().unwrap_or_default();

        let tunnel_api: Api<v1alpha1::Tunnel> = Api::namespaced(client.clone(), &client_namespace);
        let proxies = stream::iter(
            tunnel_api
                .list(&ListParams::default())
                .await?
                .items
                .into_iter()
                .filter(|tunnel|
                    tunnel.spec.client_ref.name == client_name &&
                        tunnel.spec.client_ref.namespace == client_namespace)
                .sorted_by_key(|tunnel| format!("{}-{}", tunnel.namespace().unwrap_or_default(), tunnel.name_any()))
        )
            .then(|tunnel| FrpClientProxy::resolve(tunnel, client.clone()))
            .try_collect::<Vec<FrpClientProxy>>()
            .await?;

        Ok(Self {
            server_address: value.spec.server_addr,
            server_port: value.spec.server_port,
            auth: FrpClientAuthConfig::resolve(value.spec.auth, client).await?,
            proxies,
        })
    }
}
