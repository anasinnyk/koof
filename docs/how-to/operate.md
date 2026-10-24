# Operating a provider

## Install

```sh
kubectl apply --server-side -f https://github.com/anasinnyk/koof/releases/download/v<version>/koof-cloudflare.yaml
```

This installs the CRDs, RBAC and a Deployment in the namespace `koof-cloudflare`.

## Import an existing resource

Set the external name and allow only observation:

```yaml
metadata:
  annotations:
    cloudflare.kube.nas1k.dev/external-name: <ID at the provider>
spec:
  managementPolicies: [Observe]
```

The annotation prefix is the API group of the kind. Once `Ready` is `True`, widen
`managementPolicies` (for example to `["*"]`) to let koof manage it.

## Keep the external resource when deleting the object

```yaml
spec:
  deletionPolicy: Orphan
```

## Recover from an interrupted create

`Synced=False` with "a previous create may have succeeded" means the controller
stopped between sending a create and recording its result. Check the provider:

- the resource exists: set the `<group>/external-name` annotation to its ID;
- it does not: remove the `<group>/external-create-pending` annotation.

## Run more than one replica

Set `KOOF_LEADER_ELECTION__ENABLED=true`. Only the holder of the Lease reconciles;
the others report not ready until they take over. See the
[configuration reference](../../crates/koof/README.md#configuration).

## Run locally against a real API

```sh
cargo run -p cloudflare-provider --bin koof-cloudflare-manifests | kubectl apply --server-side -f -
RUST_LOG=info,koof=debug cargo run -p cloudflare-provider --bin koof-cloudflare
```

`koof=debug` logs every provider request and every decision.

## Run the end-to-end tests

```sh
scripts/e2e.sh cloudflare
```

This needs kind, kubectl and chainsaw. It creates the kind cluster `koof-e2e` and
runs the controller and the fake API on the host. If CRDs changed in a way the
cluster rejects (for example the scope of a kind), delete the cluster first:
`kind delete cluster --name koof-e2e`.
