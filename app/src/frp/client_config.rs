use crate::frp::{FrpClientAuthConfig, FrpConfigResolvable};
use frp_operator_api::v1alpha1;
use kube::{Client, Error};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrpClientConfig {
    #[serde(rename = "serverAddr")]
    pub server_address: String,
    #[serde(rename = "serverPort")]
    pub server_port: u16,

    pub auth: FrpClientAuthConfig,
}

impl FrpConfigResolvable<v1alpha1::ClientSpec> for FrpClientConfig {
    async fn resolve(
        value: v1alpha1::ClientSpec,
        client: Client,
    ) -> Result<FrpClientConfig, Error> {
        Ok(Self {
            server_address: value.server_addr,
            server_port: value.server_port,
            auth: FrpClientAuthConfig::resolve(value.auth, client).await?, // todo: proper mapping is required for auth
        })
    }
}
