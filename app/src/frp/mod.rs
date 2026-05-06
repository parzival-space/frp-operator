mod client_auth;
mod client_config;
mod client_proxy;

pub use client_auth::*;
pub use client_config::*;
pub use client_proxy::*;

pub trait FrpConfigResolvable<T> {
    async fn resolve(value: T, client: kube::Client) -> Result<Self, kube::Error>
    where
        Self: Sized;
}
