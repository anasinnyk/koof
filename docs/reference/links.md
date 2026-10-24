# Further reading

## Concepts

- [Operator pattern](https://kubernetes.io/docs/concepts/extend-kubernetes/operator/):
  controllers that manage applications and external systems through custom resources.
- [Controllers](https://kubernetes.io/docs/concepts/architecture/controller/) and the
  reconcile loop.
- [Custom resources](https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources/)
  and [CustomResourceDefinitions](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/),
  including [CEL validation rules](https://kubernetes.io/docs/reference/using-api/cel/).
- [Finalizers](https://kubernetes.io/docs/concepts/overview/working-with-objects/finalizers/).
- [Leases](https://kubernetes.io/docs/concepts/architecture/leases/) for leader election.

## Crossplane

koof follows Crossplane's managed resource model without depending on Crossplane.

- [Managed resources](https://docs.crossplane.io/latest/concepts/managed-resources/):
  `forProvider`, `atProvider`, external names, late initialization.
- [Management policies](https://docs.crossplane.io/latest/concepts/managed-resources/#managementpolicies)
  and [deletion policy](https://docs.crossplane.io/latest/concepts/managed-resources/#deletionpolicy).
- [Upjet](https://github.com/crossplane/upjet): Crossplane's generator from Terraform
  providers. koof generates from OpenAPI instead.

## Specifications

- [OpenAPI](https://spec.openapis.org/oas/latest.html).
- [OpenAPI Overlay](https://spec.openapis.org/overlay/latest.html), the format of
  `*.overlay.yaml`.
- [JSONPath](https://www.rfc-editor.org/rfc/rfc9535) used by overlay `target`s.

## Libraries and tools

- [kube-rs](https://kube.rs) and its [controller runtime](https://docs.rs/kube/latest/kube/runtime/).
- [typify](https://github.com/oxidecomputer/typify): Rust types from JSON Schema.
- [chainsaw](https://kyverno.github.io/chainsaw): declarative end-to-end tests.
- [kind](https://kind.sigs.k8s.io): local clusters for the e2e tests.
- [prometheus-client](https://github.com/prometheus/client_rust) metrics.

## Provider APIs

- [Cloudflare API](https://developers.cloudflare.com/api/) and its
  [OpenAPI schemas](https://github.com/cloudflare/api-schemas).
- [Sentry API](https://docs.sentry.io/api/).

## These docs

Organised after [Diátaxis](https://diataxis.fr/).
