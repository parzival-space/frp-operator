# Yet Another FRP Operator
Kubernetes operator for the [FRP project made by fatedier](https://github.com/fatedier/frp).
This Operator allows you to easily deploy and manage the FRP client in you Kubernetes cluster, and use it to expose
services and pods using and FRP server running outside the cluster.

## Running in local Development Environment
1. Create a [minikube](https://minikube.sigs.k8s.io/docs/) cluster and set up kubectl to use it.
2. Start the FRP server on you local machine, a Docker Compose file is provided in the `docker` directory.
3. Run the operator locally using `cargo run -p frp-operator`

## Other FRP Operators
- [zufardhiyaulhaq/frp-operator](https://github.com/zufardhiyaulhaq/frp-operator) - FRP Client Operator written in Go.
- [Aureum-Cloud/frp-Operator](https://github.com/Aureum-Cloud/frp-Operator) - FRP Client Operator by Aureum Cloud, written in Go.
- [nnstd/frp-operator](https://github.com/nnstd/frp-operator) - FRP Operator for managing (frps, frpc) instances, written in Go.