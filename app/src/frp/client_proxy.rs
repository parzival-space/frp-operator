use kube::{Client, Error};
use serde::Serialize;
use kube::ResourceExt;
use frp_operator_api::v1alpha1;
use frp_operator_api::v1alpha1::TunnelType;
use crate::frp::FrpConfigResolvable;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrpClientProxy {
    pub name: String,
    pub r#type: FrpClientProxyType,
    pub local_ip: String,
    pub local_port: u16,
    pub remote_port: u16,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FrpClientProxyType {
    Tcp,
    Udp,
    // Http, // todo: implement other proxy types
    // Https,
}

impl FrpConfigResolvable<v1alpha1::Tunnel> for FrpClientProxy {
    async fn resolve(value: v1alpha1::Tunnel, client: Client) -> Result<Self, Error>
    where
        Self: Sized
    {
        Ok(Self {
            name: value.name_any(),
            r#type: FrpClientProxyType::resolve(value.spec.r#type, client.clone()).await?,
            local_ip: value.spec.local_address,
            local_port: value.spec.local_port,
            remote_port: value.spec.remote_port
        })
    }
}

impl FrpConfigResolvable<v1alpha1::TunnelType> for FrpClientProxyType {
    async fn resolve(value: TunnelType, client: Client) -> Result<Self, Error>
    where
        Self: Sized
    {
        Ok(match value {
            TunnelType::Tcp => FrpClientProxyType::Tcp,
            TunnelType::Udp => FrpClientProxyType::Udp,
        })
    }
}