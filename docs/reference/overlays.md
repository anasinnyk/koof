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
| `itemReferences` | map of `{from, target, into, identifier}` | A reference in each item of a list. `from`: `spec.forProvider.<list>[].<field>`, the field added to the items; `into`: the (dotted) item field that receives identifier `identifier` of the target; each item sets exactly one of the two. When the items are a union, the reference is one more variant and an item sets at most one of them |
| `credentials.securityScheme` | string | Key of `components.securitySchemes` (`apiKey` or `http` bearer) |
| `credentials.source` | `{type: secret_key_selector, path}` or `{type: related, via}` | Where the secret comes from |
| `connectionSecret` | map of Secret key to dotted path or `{from: connectionDetails, path?}` | Values written to the Secret in `spec.writeConnectionSecretToRef`. A dotted path is a field of the resource as the API returns it, read from the create response and every observation. `from: connectionDetails` reads the response of the observe operation with `connectionDetails: true` (at `path`, the whole value by default) after every observation |
| `fields.mode` | `all` or `explicit` | `explicit` exposes only the listed properties |
| `fields.forProvider` | list | Properties in `spec.forProvider` |
| `fields.atProvider` | list | Properties in `status.atProvider` |
| `immutable` | list | `forProvider` fields that cannot change once set. References used by the observe path are immutable without being listed |
| `validations` | list of `{rule, message, field?}` | CEL rules on `forProvider` |
| `test.forProvider` | object | Fixture for the generated e2e test |
| `test.update` | object | Fields the e2e test changes in its update step |

## `x-kube-operation`

On an operation (`$.paths['…'].get`).

| Key | Type | |
|---|---|---|
| `resource` | string | Resource `name` |
| `lifecycle` | `observe`, `create`, `update`, `delete` | `observe` is required, the others optional |
| `connectionDetails` | bool | With `lifecycle: observe`: not the observation but an extra `GET` after it, whose response only feeds `connectionSecret` keys with `from: connectionDetails` |
| `pathParameters` | map of [value](#values) | One per `{placeholder}` in the path |
| `request.from` | path | Body source, default `spec.forProvider` |
| `request.set` | map of dotted body path to [value](#values) | Extra body fields |
| `response.path` | string | Where the resource is in the response (`result`); on create, where `connectionSecret` paths start |
| `response.deletedWhen` | string | Observe only: the resource is gone once this field is set |
| `response.match` | `{field, value}` | Observe only: the response is a list and the resource is the item whose `field` equals the value expression `value`; missing when no item matches |
| `result.identifiers` | map | Create: where an identifier is in the response (`response.body.result.id`) |

## `x-kube-field`

On a property schema.

| Key | Default | |
|---|---|---|
| `ignored` | `false` | With `fields.mode: all`, leave the property out of `forProvider` |
| `lateInitialize` | `true` | Copy the observed value into an unset spec field |
| `sensitive` | `false` | Never copy the property into `status.atProvider`, at any depth (secrets the API returns) |
| `secretKeyRef` | `false` | `forProvider` gets `<property>From.secretKeyRef: {name, key}` next to the property, at any depth: its value is read from a Secret in the object's namespace before create and update, and not compared for drift. Not for resources with `cluster: true` |
| `listMapKeys` | | Top-level list of objects: match items by these keys, in any order, when comparing with the observation. Keys must be required (the CRD then gets `x-kubernetes-list-type: map`) or filled by an item reference |

## Values

| `type` | Keys | Resolves to |
|---|---|---|
| `external_name` | | The `<group>/external-name` annotation |
| `field` | `path` | A field of the object (`metadata.name`, `status.atProvider.type`) |
| `related_identifier` | `via`, `name` | Identifier `name` of the resource reached through the references in `via` |
| `coalesce` | `values` | The first value that is set |
| `literal` | `value` | A fixed string |
