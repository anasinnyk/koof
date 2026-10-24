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

## Immutable fields and validation

```yaml
immutable: [name]
validations:
  - rule: "self.ttl == 1 || (self.ttl >= 30 && self.ttl <= 86400)"
    message: "ttl must be 1 or between 30 and 86400"
```

Both become CEL rules in the CRD
([`x-kubernetes-validations`](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#validation-rules)),
so the API server rejects invalid changes.

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
```

## Share a resource across namespaces

`cluster: true` on `x-kube-resource` also generates a cluster-scoped
`Cluster<Kind>`. References to it accept `kind: Cluster<Kind>`.

## A type typify cannot handle

Put schema fixes that only matter for code generation in
`<provider>.typify.overlay.yaml`, last in `sources`, so resource overlays stay
about resources.
