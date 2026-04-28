mod client_auth_config;
mod client_config;
mod client_auth_method;

pub use client_auth_config::*;
pub use client_auth_method::*;
pub use client_config::*;

pub trait FrpConfigResolvable<T> {
    async fn resolve(value: T, client: kube::Client) -> Result<Self, kube::Error> where Self: Sized;
}