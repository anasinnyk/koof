# Overlay extensions

Keys are camelCase; unknown keys are rejected. Enum values (`type`, `lifecycle`)
are snake_case.

## `x-kube-provider`

On the document root (`target: $`).

| Key | Type | |
|---|---|---|
| `name` | string | Provider name: binaries `koof-<name>`, namespace `koof-<name>`. Defaults to `info.title` |
| `http.timeout` | duration | Whole request, default `30s` |
| `http.connectTimeout` | duration | Default `10s` |
| `http.poolIdleTimeout` | duration | Default `90s` |
| `http.poolMaxIdlePerHost` | integer | Default `16` |
| `http.userAgent` | string | Default `koof-<name>/<version>` |

## `x-kube-resource`

On the schema that describes the resource's desired state, usually the create
request body.

| Key | Type | |
|---|---|---|
| `name` | string | Internal name, used by `resource`, `target` and `via` |
| `apiVersion` | string | `<group>/<version>` of the CRD |
| `kind` | string | Kind of the CRD; unique per `apiVersion` |
| `cluster` | bool | Also generate the cluster-scoped `Cluster<Kind>` |
| `identifiers` | map of [value](#values) | Named IDs other resources can read with `related_identifier` |
| `references` | map of `{from, target}` | `from`: the `*Ref` field to add (`spec.forProvider.zoneRef`); `target`: a resource `name` |
| `credentials.securityScheme` | string | Key of `components.securitySchemes` (`apiKey` or `http` bearer) |
| `credentials.source` | `{type: secret_key_selector, path}` or `{type: related, via}` | Where the secret comes from |
| `fields.mode` | `all` or `explicit` | `explicit` exposes only the listed properties |
| `fields.forProvider` | list | Properties in `spec.forProvider` |
| `fields.atProvider` | list | Properties in `status.atProvider` |
| `immutable` | list | `forProvider` fields that cannot change once set |
| `validations` | list of `{rule, message, field?}` | CEL rules on `forProvider` |
| `test.forProvider` | object | Fixture for the generated e2e test |
| `test.update` | object | Fields the e2e test changes in its update step |

## `x-kube-operation`

On an operation (`$.paths['…'].get`).

| Key | Type | |
|---|---|---|
| `resource` | string | Resource `name` |
| `lifecycle` | `observe`, `create`, `update`, `delete` | `observe` is required, the others optional |
| `pathParameters` | map of [value](#values) | One per `{placeholder}` in the path |
| `request.from` | path | Body source, default `spec.forProvider` |
| `request.set` | map of dotted body path to [value](#values) | Extra body fields |
| `response.path` | string | Where the resource is in the response (`result`) |
| `response.deletedWhen` | string | Observe only: the resource is gone once this field is set |
| `result.identifiers` | map | Create: where an identifier is in the response (`response.body.result.id`) |

## `x-kube-field`

On a property schema.

| Key | Default | |
|---|---|---|
| `ignored` | `false` | With `fields.mode: all`, leave the property out of `forProvider` |
| `lateInitialize` | `true` | Copy the observed value into an unset spec field |

## Values

| `type` | Keys | Resolves to |
|---|---|---|
| `external_name` | | The `<group>/external-name` annotation |
| `field` | `path` | A field of the object (`metadata.name`, `status.atProvider.type`) |
| `related_identifier` | `via`, `name` | Identifier `name` of the resource reached through the references in `via` |
| `coalesce` | `values` | The first value that is set |
| `literal` | `value` | A fixed string |
