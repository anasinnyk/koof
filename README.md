# koof

Kubernetes OpenAPI Operator Framework: Kubernetes operators generated from OpenAPI documents.

| Crate | |
|---|---|
| [`koof`](crates/koof) | Runtime: reconciliation, references, credentials, status, metrics |
| [`koof-gen`](crates/koof-gen) | Code generator: OpenAPI + overlays → provider crate |
| [`crates/providers/*`](crates/providers) | Providers (Cloudflare, Sentry) |

## Providers

Each provider lives in `crates/providers/<name>`: the OpenAPI document and its
overlays in `openapi/` (listed in `openapi/sources`), generated code in
`src/generated/`, a controller binary and `manifests`, which prints the CRDs or
writes the provider's kustomization (`deploy/providers/<name>`).

```sh
scripts/regen.sh cloudflare                                  # regenerate code and manifests
cargo run -p cloudflare-provider --bin manifests | kubectl apply -f -
cargo run -p cloudflare-provider --bin main                  # run against the current kubeconfig
```

## Installation

`deploy/base` holds the Deployment, ServiceAccount, ClusterRoleBinding and metrics
Service; `deploy/providers/<name>` adds the generated CRDs and ClusterRole and puts
everything in the namespace `koof-<name>`.

```sh
kubectl apply --server-side -f https://github.com/anasinnyk/openapi-operator-framework/releases/download/v<version>/koof-cloudflare.yaml
kubectl apply --server-side -k 'github.com/anasinnyk/openapi-operator-framework//deploy/providers/cloudflare?ref=v<version>'
```

## End-to-end tests

`x-kube-resource.test` in an overlay gives a resource a fixture: `forProvider`
values for a test object and, optionally, the fields an `update` step changes.
`koof-gen` turns each resource with a fixture (and create + delete operations)
into a [chainsaw](https://kyverno.github.io/chainsaw) test in
`crates/providers/<name>/e2e/generated/`: create it together with everything it
references, wait for `Ready`, update, delete.

```sh
scripts/e2e.sh [provider...]   # kind cluster koof-e2e, controller + fake API on the host
```

The controller talks to `koof::fake`, an in-memory API built from the same
descriptors (`fake` binary of each provider, built with the `fake` feature),
instead of the real provider.

## Images

```sh
docker build --build-arg PROVIDER=cloudflare -t koof-cloudflare .
```

Released images: `ghcr.io/anasinnyk/koof-<provider>:<version>` (and `latest`). A pull
request labelled `release` publishes `<version>-alpha.pr<number>.<run>`.

## Releasing

Push a tag `v<version>` matching `workspace.package.version` in `Cargo.toml`. The
release workflow publishes `koof` and `koof-gen` to crates.io (secret
`CARGO_REGISTRY_TOKEN`), pushes the provider images and creates the GitHub release
with `koof-<provider>.yaml` install bundles pinned to the version.

## License

MIT
