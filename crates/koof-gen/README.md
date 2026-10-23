# koof-gen

Generates a Kubernetes operator (provider) from an OpenAPI document and
[OpenAPI overlays](https://spec.openapis.org/overlay/latest.html). The generated
code runs on the [`koof`](https://crates.io/crates/koof) runtime.

```sh
cargo install koof-gen
```

## Usage

```sh
koof-gen \
  --openapi openapi/api.yaml \
  --overlay openapi/provider.overlay.yaml \
  --overlay openapi/widget.overlay.yaml \
  --output .
```

`--output` is the provider crate: `src/generated/` receives the CRD types, resource
descriptors, operations and typify-generated API types. Overlays are applied in
order; `--preprocessed-spec FILE` writes the schema set used for type generation.

## Overlays

The provider:

```yaml
actions:
  - target: $
    update:
      x-kube-provider:
        name: example
        # Optional HTTP client defaults; KOOF_HTTP__* variables override them.
        http:
          timeout: 30s
          connectTimeout: 10s
          poolIdleTimeout: 90s
          poolMaxIdlePerHost: 16
          userAgent: example-operator
```

A resource: `x-kube-resource` on its schema (usually the create request body)
and `x-kube-operation` on each lifecycle operation:

```yaml
actions:
  - target: $.paths['/accounts/{account_id}/widgets'].post.requestBody.content['application/json'].schema
    update:
      x-kube-resource:
        name: widget
        apiVersion: example.kube.dev/v1alpha1
        kind: Widget
        identifiers:
          external_id:
            type: external_name
        references:
          account:
            from: spec.forProvider.accountRef
            target: account
        credentials:
          securityScheme: api_token
          source:
            type: related
            via: [account]
        fields:
          mode: all

  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].get
    update:
      x-kube-operation:
        resource: widget
        lifecycle: observe
        response:
          path: result
        pathParameters:
          account_id:
            type: related_identifier
            via: [account]
            name: external_id
          widget_id:
            type: external_name
```

Every resource is namespaced: references point at resources in the same
namespace and the credentials Secret is read from it. `cluster: true` on
`x-kube-resource` also generates a cluster-scoped `Cluster<Kind>` (its Secret
reference names a namespace); references to the resource then take
`kind: <Kind> | Cluster<Kind>` (default `<Kind>`), and a cluster variant can only
reference resources that have one too.

`lifecycle` is one of `observe` (required), `create`, `update` and `delete`.
For APIs that soft-delete (the observe endpoint keeps answering `200`),
`response.deletedWhen: deleted_at` on `observe` treats the resource as gone once
that field is set.
Request bodies default to `spec.forProvider`; `request.set` adds values such as
identifiers of referenced resources. `x-kube-field` on a property sets `ignored`
or `lateInitialize: false`.

## Provider crate

The generated code uses `koof`, `kube`, `k8s-openapi` (feature `schemars`),
`schemars`, `serde` and `serde_json`; depending on the schemas also `regress`
(patterns), `chrono` (`date-time`) and `uuid` (`uuid`). The binary needs `tokio`.

Use the `koof` release of the same minor version as `koof-gen`.

## License

MIT
