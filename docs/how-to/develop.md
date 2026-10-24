# Developing koof

[AGENTS.md](../../AGENTS.md) describes the repository layout, conventions and the
checks to run before committing.

## Regenerate a provider

```sh
scripts/regen.sh cloudflare      # code, CRDs, RBAC, e2e tests and the provider README
scripts/check-generated.sh       # fails if generated files are stale
scripts/check.sh                 # everything CI checks except e2e
```

## End-to-end tests

`x-kube-resource.test` in an overlay gives a resource a fixture: `forProvider`
values for a test object and, optionally, the fields an `update` step changes.
`koof-gen` turns each resource with a fixture (and create + delete operations)
into a [chainsaw](https://kyverno.github.io/chainsaw) test in
`crates/providers/<name>/e2e/generated/`: create it together with everything it
references, wait for `Ready`, update, delete.

```sh
scripts/e2e.sh [provider...]     # kind cluster koof-e2e, controller + fake API on the host
```

The controller talks to `koof::fake`, an in-memory API built from the same
descriptors (the `koof-<provider>-fake` binary, built with the `fake` feature),
instead of the real provider.

## Images

```sh
docker build --build-arg PROVIDER=cloudflare -t koof-cloudflare .
```

Released images: `ghcr.io/anasinnyk/koof-<provider>:<version>` (and `latest`). A pull
request labelled `release` publishes `<version>-alpha.pr<number>.<run>`.

## Release

Push a tag `v<version>` matching `workspace.package.version` in `Cargo.toml`. The
release workflow publishes `koof` and `koof-gen` to crates.io (secret
`CARGO_REGISTRY_TOKEN`), pushes the provider images and creates the GitHub release
with `koof-<provider>.yaml` install bundles pinned to the version.
