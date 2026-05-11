use crate::resources::{ANNOTATION_CONFIG_HASH, KEY_FRPC_CONFIG, ManagedClientConfigSecret};
use frp_operator_api::v1alpha1;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, PodSpec, PodTemplateSpec, SecretVolumeSource, Volume, VolumeMount,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::ResourceExt;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ManagedClientDeployment {
    deployment: Deployment,
}

impl ManagedClientDeployment {
    fn new() -> Self {
        let labels = BTreeMap::from([
            ("app.kubernetes.io/name".to_string(), "frpc".to_string()),
            (
                "app.kubernetes.io/managed-by".to_string(),
                "frp-operator".to_string(),
            ),
            ("frp.parzival.space/client".to_string(), String::new()),
        ]);

        Self {
            deployment: Deployment {
                metadata: ObjectMeta {
                    labels: Some(labels.clone()),
                    ..Default::default()
                },
                spec: Some(DeploymentSpec {
                    replicas: Some(1),
                    selector: LabelSelector {
                        match_labels: Some(labels.clone()),
                        ..Default::default()
                    },
                    template: PodTemplateSpec {
                        metadata: Some(ObjectMeta {
                            labels: Some(labels),
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
            },
        }
    }

    fn set_namespace(&mut self, namespace: Option<String>) {
        self.deployment.metadata.namespace = namespace;
    }

    fn set_client_name(&mut self, client_name: String) {
        self.deployment.metadata.name = Some(format!("{}-frpc", client_name.clone()));

        self.deployment.metadata.labels
            .get_or_insert_default()
            .insert("frp.parzival.space/client".to_string(), client_name.clone());

        self.deployment.spec.get_or_insert_default()
            .selector.match_labels.get_or_insert_default()
            .insert("frp.parzival.space/client".to_string(), client_name.clone());

        self.deployment.spec.get_or_insert_default()
            .template.metadata.get_or_insert_default()
            .labels.get_or_insert_default()
            .insert("frp.parzival.space/client".to_string(), client_name);
    }

    fn set_config_secret_name(&mut self, secret_name: Option<String>) {
        if let Some(secret) = self
            .deployment
            .spec
            .as_mut()
            .and_then(|spec| spec.template.spec.as_mut())
            .and_then(|spec| spec.volumes.as_mut())
            .and_then(|volumes| volumes.first_mut())
            .and_then(|volume| volume.secret.as_mut())
        {
            secret.secret_name = secret_name;
        }
    }

    fn set_config_hash(&mut self, config_hash: Option<&str>) {
        let config_hash = config_hash.unwrap_or_default().to_string();

        self.deployment.metadata.annotations
            .get_or_insert_default()
            .insert(ANNOTATION_CONFIG_HASH.to_string(), config_hash.clone());

        self.deployment.spec.get_or_insert_default()
            .template.metadata.get_or_insert_default()
            .annotations.get_or_insert_default()
            .insert(ANNOTATION_CONFIG_HASH.to_string(), config_hash.clone());
    }

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

impl From<(&v1alpha1::Client, ManagedClientConfigSecret)> for ManagedClientDeployment {
    fn from(value: (&v1alpha1::Client, ManagedClientConfigSecret)) -> Self {
        let (client, config) = value;
        let mut deployment = Self::new();
        deployment.set_namespace(client.namespace());
        deployment.set_client_name(client.name_any());
        deployment.set_config_secret_name(config.name().map(str::to_string));
        deployment.set_config_hash(config.config_hash());
        deployment
    }
}
