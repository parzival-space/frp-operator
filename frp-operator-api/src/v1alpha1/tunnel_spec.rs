use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(group = "frp.parzival.space", kind = "Tunnel", version = "v1alpha1")]
#[kube(namespaced)]
#[serde(rename_all = "camelCase")]
pub struct TunnelSpec {
    pub r#type: TunnelType,
    pub local_address: String,
    pub local_port: u16,
    
    #[serde(default)]
    pub remote_port: u16,
    
    pub client_ref: ClientRef,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum TunnelType {
    #[default]
    Tcp,
    Udp,
    // Http, // will be implemented in later versions
    // Https,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ClientRef {
    pub namespace: String,
    pub name: String,
}