use kube::{CustomResource, CustomResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "frp.parzival.space", kind = "FrpClient", version = "v1", namespaced)]
pub struct FrpClientSpec {
    pub name: String,
    pub server_addr: String,
    pub token: String,
}

fn main() {
    println!("{}", serde_yaml::to_string(&FrpClient::crd()).unwrap()
    );
}