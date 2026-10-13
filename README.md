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
`src/generated/`, a controller binary and `crdgen`, which prints the CRDs.

```sh
scripts/regen.sh cloudflare                              # regenerate
cargo run -p cloudflare-provider --bin crdgen | kubectl apply -f -
cargo run -p cloudflare-provider --bin main              # run against the current kubeconfig
```

## Images

```sh
docker build --build-arg PROVIDER=cloudflare -t koof-cloudflare .
```

Released images: `ghcr.io/anasinnyk/koof-<provider>:<version>`; CRDs are attached
to each GitHub release as `<provider>-crds.json`.

## Releasing

Push a tag `v<version>` matching `workspace.package.version` in `Cargo.toml`. The
release workflow publishes `koof` and `koof-gen` to crates.io (secret
`CARGO_REGISTRY_TOKEN`), pushes the provider images and creates the GitHub release.

## License

MIT
