use kube::core::crd::v1::CustomResourceExt;
use std::env;
use frp_operator_api::v1alpha1::{Client, Tunnel};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let generated_crds = format!(
        "{}",
        vec![
            serde_yaml::to_string(&Client::crd()).unwrap(),
            serde_yaml::to_string(&Tunnel::crd()).unwrap(),
        ].join("---\n")
    );

    if let Some(output_path) = args.get(1) {
        println!("Writing generated CRDs to {}", output_path);
        std::fs::write(output_path, generated_crds).expect("Failed to write CRDs to file");
    } else {
        println!("{}", generated_crds);
    }
}
