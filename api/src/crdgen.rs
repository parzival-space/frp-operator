use api::v1alpha1::Client;
use kube::CustomResourceExt;

fn main() {
    println!("{}",
        serde_yaml::to_string(&Client::crd()).unwrap()
    )
}