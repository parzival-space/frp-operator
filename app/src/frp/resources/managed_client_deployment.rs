use std::collections::BTreeMap;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{Container, PodSpec, PodTemplateSpec, SecretVolumeSource, Volume, VolumeMount};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use frp_operator_api::v1alpha1;
use kube::ResourceExt;
use crate::frp::resources::{ManagedClientConfigSecret, ANNOTATION_CONFIG_HASH, KEY_FRPC_CONFIG};

#[derive(Debug, Clone)]
pub struct ManagedClientDeployment {
    deployment: Deployment,
}

impl ManagedClientDeployment {
    pub fn name(&self) -> Option<&str> {
        self.deployment.metadata.name.as_deref()
    }

    pub fn config_hash(&self) -> Option<&str> {
        self.deployment
            .metadata
            .annotations
            .as_ref()?
            .get(ANNOTATION_CONFIG_HASH)
            .map(String::as_str)
    }
}

impl Into<Deployment> for ManagedClientDeployment {
    fn into(self) -> Deployment {
        self.deployment
    }
}

impl From<Deployment> for ManagedClientDeployment {
    fn from(deployment: Deployment) -> Self {
        Self { deployment }
    }
}

// implementation for v1alpha1
impl From<(&v1alpha1::Client, ManagedClientConfigSecret)> for ManagedClientDeployment {
    fn from(value: (&v1alpha1::Client, ManagedClientConfigSecret)) -> Self {
        let (client, config) = value;
        let client_name = client.name_any();
        let selector_labels = BTreeMap::from([
            ("app.kubernetes.io/name".to_string(), "frpc".to_string()),
            (
                "app.kubernetes.io/managed-by".to_string(),
                "frp-operator".to_string(),
            ),
            ("frp.parzival.space/client".to_string(), client_name.clone()),
        ]);

        let mut annotations = BTreeMap::new();
        if let Some(config_hash) = config.config_hash() {
            annotations.insert(ANNOTATION_CONFIG_HASH.to_string(), config_hash.to_string());
        }

        Self {
            deployment: Deployment {
                metadata: ObjectMeta {
                    name: Some(format!("{}-frpc", client_name)),
                    namespace: client.namespace(),
                    labels: Some(selector_labels.clone()),
                    annotations: (!annotations.is_empty()).then_some(annotations.clone()),
                    ..Default::default()
                },
                spec: Some(DeploymentSpec {
                    replicas: Some(1),
                    selector: LabelSelector {
                        match_labels: Some(selector_labels.clone()),
                        ..Default::default()
                    },
                    template: PodTemplateSpec {
                        metadata: Some(ObjectMeta {
                            labels: Some(selector_labels),
                            annotations: (!annotations.is_empty()).then_some(annotations),
                            ..Default::default()
                        }),
                        spec: Some(PodSpec {
                            containers: vec![Container {
                                name: "frpc".to_string(),
                                image: Some("snowdreamtech/frpc".to_string()),
                                volume_mounts: Some(vec![VolumeMount {
                                    name: "config".to_string(),
                                    mount_path: "/etc/frp/frpc.toml".to_string(),
                                    sub_path: Some(KEY_FRPC_CONFIG.to_string()),
                                    read_only: Some(true),
                                    ..Default::default()
                                }]),
                                ..Default::default()
                            }],
                            volumes: Some(vec![Volume {
                                name: "config".to_string(),
                                secret: Some(SecretVolumeSource {
                                    secret_name: config.name().map(str::to_string),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }),
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
    }
}