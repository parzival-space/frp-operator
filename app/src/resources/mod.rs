mod managed_client_config_secret;
mod managed_client_deployment;

pub use managed_client_config_secret::*;
pub use managed_client_deployment::*;

pub const KEY_FRPC_CONFIG: &str = "frpc.toml";
pub const ANNOTATION_CONFIG_HASH: &str = "frp.parzival.space/config-hash";
