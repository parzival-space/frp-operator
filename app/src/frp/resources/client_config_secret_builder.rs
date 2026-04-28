use std::collections::BTreeMap;

use frp_operator_api::v1alpha1;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::ResourceExt;
use log::warn;

use crate::frp::config::FrpClientConfig;

pub const FRPC_CONFIG_KEY: &str = "frpc.toml";
pub const CONFIG_HASH_ANNOTATION: &str = "frp.parzival.space/config-hash";

#[derive(Debug, Clone)]
pub struct ManagedClientConfigSecret {
    secret: Secret,
}

impl ManagedClientConfigSecret {
    pub fn name(&self) -> Option<&str> {
        self.secret.metadata.name.as_deref()
    }

    pub fn config_hash(&self) -> Option<&str> {
        self.secret
            .metadata
            .annotations
            .as_ref()?
            .get(CONFIG_HASH_ANNOTATION)
            .map(String::as_str)
    }
}

impl Into<Secret> for ManagedClientConfigSecret {
    fn into(self) -> Secret {
        self.secret
    }
}

impl From<Secret> for ManagedClientConfigSecret {
    fn from(secret: Secret) -> Self {
        Self { secret }
    }
}

impl From<(&v1alpha1::Client, FrpClientConfig)> for ManagedClientConfigSecret {
    fn from(value: (&v1alpha1::Client, FrpClientConfig)) -> Self {
        let (client, client_config) = value;
        let client_name = client.name_any();
        let mut secret = Secret {
            metadata: ObjectMeta {
                name: Some(format!("{}-frpc-config", client_name)),
                namespace: client.namespace(),
                labels: Some(BTreeMap::from([
                    ("app.kubernetes.io/name".to_string(), "frpc".to_string()),
                    (
                        "app.kubernetes.io/managed-by".to_string(),
                        "frp-operator".to_string(),
                    ),
                    ("frp.parzival.space/client".to_string(), client_name),
                ])),
                ..ObjectMeta::default()
            },
            type_: Some("Opaque".to_string()),
            ..Secret::default()
        };

        match toml::to_string(&client_config) {
            Ok(rendered_config) => {
                secret.string_data = Some(BTreeMap::from([(
                    FRPC_CONFIG_KEY.to_string(),
                    rendered_config.clone(),
                )]));

                let config_hash = blake3::hash(rendered_config.as_bytes()).to_string();
                secret
                    .metadata
                    .annotations
                    .get_or_insert_default()
                    .insert(CONFIG_HASH_ANNOTATION.to_string(), config_hash);
            }
            Err(err) => {
                warn!(
                    "Failed to serialize client config to TOML for {}: {}",
                    client.name_any(),
                    err
                );
            }
        }

        Self { secret }
    }
}