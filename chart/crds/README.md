# Generating CRDs
The custom resource definitions (CRDs) for this directory need to be generated using `crdgen` binary, which is part
of the `frp-operator-api` crate.

You can generate the CRDs by running the following command in this directory:
```bash
cargo run --bin crdgen -p frp-operator-api -- ./generated.yaml
```