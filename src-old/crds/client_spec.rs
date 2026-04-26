use k8s_openapi::serde::{Deserialize, Serialize};
use kube::CustomResource;
use schemars::JsonSchema;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "frp.parzival.space", kind = "Client", version = "v1", namespaced)]
pub struct ClientSpec {
    pub name: String,
    pub server_addr: String,
    pub token: String,
}