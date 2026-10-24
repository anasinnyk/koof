<p align="center">
  <img src="docs/assets/koof.png" alt="koof" width="240">
</p>

# koof

Kubernetes OpenAPI Operator Framework: Kubernetes operators generated from OpenAPI
documents.

Describe an API's resources in a few lines of [OpenAPI overlay](https://spec.openapis.org/overlay/latest.html),
and `koof-gen` generates CRDs, RBAC, end-to-end tests and a controller that manages
them [Crossplane-style](https://docs.crossplane.io/latest/concepts/managed-resources/):
observe, create, update, delete, drift detection, references between resources and
credentials from Secrets.

## Providers

| Provider | Kinds | Install |
|---|---|---|
| [Cloudflare](crates/providers/cloudflare/README.md) | Account, Zone, DNSRecord, Tunnel, Ruleset, … (32) | `koof-cloudflare.yaml` |
| [Sentry](crates/providers/sentry/README.md) | Organization, Team, Project, ProjectKey, Monitor, … (9) | `koof-sentry.yaml` |

```sh
kubectl apply --server-side -f https://github.com/anasinnyk/koof/releases/download/v<version>/koof-<provider>.yaml
```

The bundle contains the CRDs, RBAC and a Deployment in the namespace
`koof-<provider>`. To build your own, use the kustomization:
`github.com/anasinnyk/koof//deploy/providers/<provider>?ref=v<version>`.

## Documentation

- [Tutorial: add a resource to a provider](docs/tutorials/add-a-resource.md)
- [How reconciliation works](docs/explanation/reconciliation.md)
- [Overlay reference](docs/reference/overlays.md)
- [All documentation](docs/README.md)

## Crates

| Crate | |
|---|---|
| [`koof`](crates/koof) | Runtime: reconciliation, references, credentials, status, metrics |
| [`koof-gen`](crates/koof-gen) | Generator: OpenAPI + overlays → provider crate |

Contributing: [Developing koof](docs/how-to/develop.md).

## License

MIT
