# Managed resources

What every generated kind has in common.

## Spec

| Field | Default | |
|---|---|---|
| `forProvider` | | Desired state, sent to the provider API |
| `managementPolicies` | `["*"]` | Allowed actions: `Observe` (required), `Create`, `Update`, `Delete`, `LateInitialize`, or `*` |
| `deletionPolicy` | `Delete` | `Orphan` keeps the external resource when the object is deleted |

A reference is `{name}` in the same namespace. A reference that can point to a
cluster-scoped kind also takes `kind`, and `apiVersion` when the kind exists in
several groups.

## Status

| Field | |
|---|---|
| `atProvider` | The resource as the provider API returns it |
| `conditions[Ready]` | `True/Available`; `False/Creating`, `Deleting`, `InUse`, `Unavailable` |
| `conditions[Synced]` | `True/ReconcileSuccess`; `False/ReconcileError` with the error chain |

## Annotations

Prefixed with the kind's API group, e.g. `zones.cloudflare.kube.nas1k.dev/`.

| Annotation | |
|---|---|
| `external-name` | ID of the external resource; set after a create, or by you to import |
| `external-create-pending` | A create was sent at this time |
| `external-create-succeeded` | The create succeeded at this time |
| `external-create-failed` | The create failed at this time |

The finalizer is `<group>/finalizer`.

## Events

`Created`, `Updated`, `Deleted`, `Orphaned`, `Available`, `LateInitialized`,
`InUse`, `DriftPersists` (warning), `ReconcileError` (warning).

## Metrics

Served on `/metrics` (`KOOF_METRICS_ADDR`, default `:8080`), labelled by `group`
and `kind`:

| Metric | |
|---|---|
| `koof_reconcile_total` | Reconciles by result |
| `koof_reconcile_duration_seconds` | Reconcile duration |
| `koof_actions_total` | Creates, updates, deletes, orphans |
| `koof_drift_detected_total` | Reconciles that found drift |
| `koof_late_initialized_total` | Late initializations |
| `koof_external_requests_total` | Provider API requests by operation and status class |
| `koof_external_request_duration_seconds` | Provider API request duration |
| `koof_managed_resources` | Objects by `Ready` / `Synced` state |

`/healthz` answers once the process is up; `/readyz` once every controller has
finished its initial sync (and, with leader election, holds the Lease).

## Configuration

Environment variables are listed in the
[`koof` crate README](../../crates/koof/README.md#configuration).
