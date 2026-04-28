use kube::{Client, Error};
use serde::Serialize;
use frp_operator_api::v1alpha1;
use crate::frp::config::FrpConfigResolvable;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FrpClientAuthMethod {
    Token,
    // todo: add other auth configurations
}

impl FrpConfigResolvable<v1alpha1::ClientAuthMethod> for FrpClientAuthMethod {
    async fn kube_from(value: v1alpha1::ClientAuthMethod, _client: Client) -> Result<Self, Error> {
        Ok(
            match value {
                v1alpha1::ClientAuthMethod::Token => FrpClientAuthMethod::Token,
            }
        )
    }
}