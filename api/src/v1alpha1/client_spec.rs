use std::collections::BTreeMap;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(group = "frp.parzival.space", kind = "Client", version = "v1alpha1")]
#[kube(namespaced)]
#[serde(rename_all = "camelCase")]
pub struct ClientSpec {
    pub server_addr: String,
    pub server_port: u16,
    pub auth: ClientAuthSpec,
    #[serde(default)]
    pub metadatas: BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientAuthSpec {
    pub method: ClientAuthMethod,
    pub token_secret_ref: SecretKeyRef,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ClientAuthMethod {
    #[default]
    Token,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecretKeyRef {
    pub name: String,
    pub key: String,
}