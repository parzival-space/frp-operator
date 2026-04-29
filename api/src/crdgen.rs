use frp_operator_api::v1alpha1::Client;
use kube::CustomResourceExt;

fn main() {
    println!(
        "{}",
        vec![serde_yaml::to_string(&Client::crd()).unwrap()].join("\n---\n")
    )
}
