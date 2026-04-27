use serde::Serialize;
use kube::{Client, Error};
use frp_operator_api::v1alpha1;
use crate::frp::config::{FrpClientAuthConfig, FrpClientAuthMethod};
use crate::frp::config::FrpConfigResolvable;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrpClientConfig {
    #[serde(rename = "serverAddr")]
    pub server_address: String,
    #[serde(rename = "serverPort")]
    pub server_port: u16,

    pub auth: FrpClientAuthConfig,
}

impl FrpConfigResolvable<v1alpha1::ClientSpec> for FrpClientConfig {
    async fn kube_from(value: v1alpha1::ClientSpec, client: Client) -> Result<FrpClientConfig, Error> {
        Ok(
            Self {
                server_address: value.server_addr,
                server_port: value.server_port,
                auth: FrpClientAuthConfig::kube_from(value.auth, client).await?, // todo: proper mapping is required for auth
            }
        )
    }
}