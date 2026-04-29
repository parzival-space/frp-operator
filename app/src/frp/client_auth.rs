use crate::frp::FrpConfigResolvable;
use frp_operator_api::v1alpha1;
use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client, Error};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrpClientAuthConfig {
    pub method: FrpClientAuthMethod,
    pub token: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FrpClientAuthMethod {
    Token,
    // todo: add other auth configurations
}

impl FrpConfigResolvable<v1alpha1::ClientAuthSpec> for FrpClientAuthConfig {
    async fn resolve(value: v1alpha1::ClientAuthSpec, client: Client) -> Result<Self, Error> {
        let secrets = Api::<Secret>::default_namespaced(client.clone());
        let secret = secrets.get(&value.token_secret_ref.name).await?;
        let token_data = secret.data.unwrap();
        let token_value_bytes = token_data
            .get(&value.token_secret_ref.key)
            .unwrap()
            .0
            .clone();
        let token_value = String::from_utf8(token_value_bytes).unwrap();
        // todo: implement proper token secret reading

        Ok(Self {
            method: FrpClientAuthMethod::resolve(value.method, client).await?,
            token: (*token_value.clone()).parse().unwrap(),
        })
    }
}

impl FrpConfigResolvable<v1alpha1::ClientAuthMethod> for FrpClientAuthMethod {
    async fn resolve(value: v1alpha1::ClientAuthMethod, _client: Client) -> Result<Self, Error> {
        Ok(match value {
            v1alpha1::ClientAuthMethod::Token => FrpClientAuthMethod::Token,
        })
    }
}
