mod client_config;
mod client_auth_config;

pub use client_config::*;

pub trait FrpConfigResolvable<T>: Sized {
    async fn kube_from(value: T, client: kube::Client) -> Result<Self, kube::Error>; // todo: implement proper error type
}