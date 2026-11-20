# Overlay recipes

Short answers for API shapes that come up when adding a resource. Run
`scripts/regen.sh <provider>` after each change. The
[overlay reference](../reference/overlays.md) lists every key.

## Read credentials from a Secret

The root resource of a provider (an account, an organization) holds the token:

```yaml
credentials:
  securityScheme: api_token          # a key of components.securitySchemes
  source:
    type: secret_key_selector
    path: spec.forProvider.apiTokenSecretRef
```

`apiTokenSecretRef: {name, key}` is added to `forProvider` and must name a Secret in
the object's namespace.

## Inherit credentials from a parent

```yaml
credentials:
  securityScheme: api_token
  source:
    type: related
    via: [zone, account]             # follow zoneRef, then the zone's accountRef
```

## Use a parent's ID in the path

```yaml
references:
  account:
    from: spec.forProvider.accountRef
    target: account
# on each operation:
pathParameters:
  account_id:
    type: related_identifier
    via: [account]
    name: external_id                # an identifier declared by the target
```

## Put a parent's ID in the request body

```yaml
- target: $.paths['/zones'].post
  update:
    x-kube-operation:
      resource: zone
      lifecycle: create
      request:
        set:
          account.id:
            type: related_identifier
            via: [account]
            name: external_id
```

## Name-addressed resources

Some APIs address a resource by a name chosen by the user, not by a generated ID.
Fall back from the external name to the object's name:

```yaml
pathParameters:
  project_name:
    type: coalesce
    values:
      - type: external_name
      - type: field
        path: metadata.name
result:
  identifiers:
    external_id: response.body.result.name
```

## Update needs the full object (PUT)

Add the fields that are required by the API but are not part of `forProvider`:

```yaml
request:
  set:
    id:
      type: external_name
    type:
      type: field
      path: status.atProvider.type
```

## Soft delete

When `GET` keeps answering `200` after a delete and marks the resource instead:

```yaml
lifecycle: observe
response:
  path: result
  deletedWhen: deleted_at
```

## Observe through a list

When the API has no endpoint for one resource, only a list (a team's access to a
project: `GET /projects/{org}/{project}/teams/`), observe the list and pick the
item:

```yaml
lifecycle: observe
response:
  match:
    field: id
    value:
      type: related_identifier
      via: [team]
      name: external_id
```

Missing when no item matches. Only the first page of a paginated list is read.

## Immutable fields and validation

```yaml
immutable: [name]
validations:
  - rule: "self.ttl == 1 || (self.ttl >= 30 && self.ttl <= 86400)"
    message: "ttl must be 1 or between 30 and 86400"
```

Both become CEL rules in the CRD
([`x-kubernetes-validations`](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#validation-rules)),
so the API server rejects invalid changes. A reference used in the observe path
(`zoneRef` of a DNSRecord) is immutable without being listed: pointing it at
another parent would make koof look for the resource there.

## A field koof should not touch

Properties marked `readOnly` never reach `forProvider`. With `fields.mode: all`,
set `x-kube-field` on the property schema (with `explicit`, just do not list it):

```yaml
- target: $.components.schemas.zones_zone.properties.status
  update:
    x-kube-field:
      ignored: true              # not in forProvider; still reported in atProvider
- target: $.components.schemas.zones_zone.properties.plan
  update:
    x-kube-field:
      lateInitialize: false      # never copied from the API into the spec
- target: $.components.schemas.turnstile_secret
  update:
    x-kube-field:
      sensitive: true            # a secret the API returns; never in atProvider
```

Anyone who can read the object can read its status, so mark every secret the
observe response contains as `sensitive`, nested ones included.

## A list the API returns in its own order

When the API keeps a set of objects and returns them sorted its own way, match the
items by key so that the order is not drift:

```yaml
- target: $.components.schemas.member.properties.teamRoles
  update:
    x-kube-field:
      listMapKeys: [teamSlug]    # each key must be a required property of the items
```

## Reference objects from list items

When a list names other resources by ID, let each item reference the object
instead. The identifier of the referenced object fills the item's ID field before
the request is sent:

```yaml
x-kube-resource:
  itemReferences:
    team:
      from: spec.forProvider.teamRoles[].teamRef   # field added to every item
      target: team
      into: teamSlug                                # filled from the reference
      identifier: slug                              # an identifier of the target
```

Each item sets exactly one of `teamSlug` and `teamRef`. A `listMapKeys` key may be
the `into` field; the list is then compared by key, but not declared a map in the
CRD (Kubernetes requires map keys to be set). Deleting a referenced object waits
while items reference it.

`into` may be nested (`group.id`). When the items are a union (`oneOf` of rule
objects, like Cloudflare Access rules), the reference becomes one more variant,
`{groupRef: {name}}`, which is replaced by `{group: {id}}`:

```yaml
x-kube-resource:
  itemReferences:
    include_group:
      from: spec.forProvider.include[].groupRef
      target: access_group
      into: group.id
      identifier: external_id
```

## Take a field's value from a Secret

When a request field holds a secret the user should not write into the object (a
Pages environment variable of type `secret_text`), mark the property:

```yaml
- target: $.components.schemas.pages_secret_text_env_var.properties.value
  update:
    x-kube-field:
      secretKeyRef: true
```

The object can then set `valueFrom.secretKeyRef: {name, key}` next to `value`, at
any depth (lists and maps included); exactly one of the two when the API
requires the field. The Secret is read from the object's namespace before each
create and update. The field is not compared with the observation (APIs return
such values empty or masked), so a changed Secret is sent with the next update,
not on its own.

## Publish values the API returns

Values clients need (a token's client secret, a DSN) go to a Secret instead of
the status:

```yaml
x-kube-resource:
  connectionSecret:
    client_id: client_id             # Secret key: dotted path in the resource
    client_secret: client_secret
```

The object names the Secret in `spec.writeConnectionSecretToRef`. Keys are taken
from the create response (set `response.path` on the create operation when the
resource is wrapped, e.g. `result`) and from every observation; a key the
observation lacks keeps its value, so a secret the API returns only on create
survives. Mark such fields `sensitive` as well when the observe response has them.

When the API returns a value from a separate endpoint (a tunnel token), mark that
`GET` operation `lifecycle: connection` and read the key from its response:

```yaml
x-kube-resource:
  connectionSecret:
    tunnel_id: id
    token:
      from: connection               # the whole value at the operation's response.path
      # path: some.field             # or a dotted path in it
---
# on GET /accounts/{account_id}/cfd_tunnel/{tunnel_id}/token
x-kube-operation:
  resource: tunnel
  lifecycle: connection
  response:
    path: result
  pathParameters: { … }              # as on observe
```

koof calls it after each observation of an object that has
`writeConnectionSecretToRef`, so a rotated value reaches the Secret.

## Share a resource across namespaces

`cluster: true` on `x-kube-resource` also generates a cluster-scoped
`Cluster<Kind>`. References to it accept `kind: Cluster<Kind>`.

## A type typify cannot handle

Put schema fixes that only matter for code generation in
`<provider>.typify.overlay.yaml`, last in `sources`, so resource overlays stay
about resources.
