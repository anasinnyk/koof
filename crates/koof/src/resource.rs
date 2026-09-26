//! What `koof-gen` emits per resource: a static [`ResourceDescriptor`] and a
//! [`ManagedResource`] impl. Everything else is generic runtime code.

use crate::{http::Operation, managed::ManagedResourceSpec, status::ManagedStatus};
use kube::core::NamespaceResourceScope;
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// A generated custom resource managed by koof.
pub trait ManagedResource:
    kube::Resource<DynamicType = (), Scope = NamespaceResourceScope>
    + Clone
    + Debug
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + 'static
{
    /// Observed state, stored in `status.atProvider`.
    type AtProvider: Serialize
        + DeserializeOwned
        + JsonSchema
        + Clone
        + Debug
        + PartialEq
        + Send
        + Sync;

    /// Operation that reads the external resource.
    type Observe: Operation<Response: Serialize>;

    /// Operation that creates it; [`crate::http::Unsupported`] when there is none
    /// (then [`ResourceDescriptor::create`] is `None`). The request body is built
    /// as JSON and deserialized into `Request`.
    type Create: Operation<Request: DeserializeOwned>;

    /// Operation that deletes it; [`crate::http::Unsupported`] when there is none
    /// (then [`ResourceDescriptor::delete`] is `None`).
    type Delete: Operation;

    fn descriptor() -> &'static ResourceDescriptor;

    fn status(&self) -> Option<&ManagedStatus<Self::AtProvider>>;

    /// `managementPolicies` and `deletionPolicy` from the spec.
    fn management(&self) -> &ManagedResourceSpec;
}

/// Static description of a resource: how to find its identifiers, related
/// resources, credentials and observe parameters.
#[derive(Debug)]
pub struct ResourceDescriptor {
    pub kind: &'static str,
    /// Used to read this resource as a `DynamicObject` when another resource
    /// references it.
    pub api_resource: fn() -> kube::api::ApiResource,
    /// API group; prefixes koof's annotations and finalizer, e.g.
    /// `zones.cloudflare.kube.nas1k.dev/external-name`.
    pub group: &'static str,
    pub identifiers: &'static [(&'static str, ValueExpr)],
    pub references: &'static [Reference],
    pub credentials: Option<Credentials>,
    pub observe: OperationDescriptor,
    pub create: Option<CreateDescriptor>,
    pub delete: Option<OperationDescriptor>,
}

impl ResourceDescriptor {
    #[must_use]
    pub fn identifier(&self, name: &str) -> Option<&ValueExpr> {
        self.identifiers
            .iter()
            .find(|(identifier, _)| *identifier == name)
            .map(|(_, expression)| expression)
    }

    /// `<group>/<name>`, the key of one of koof's annotations.
    #[must_use]
    pub fn annotation(&self, name: &str) -> String {
        format!("{}/{name}", self.group)
    }

    /// `<group>/finalizer`.
    #[must_use]
    pub fn finalizer(&self) -> String {
        self.annotation("finalizer")
    }

    /// The external name (ID) of `resource` (as JSON) from the
    /// `<group>/external-name` annotation, if set and not empty. The annotation is
    /// the source of truth: setting it on a new resource imports an existing
    /// external resource.
    #[must_use]
    pub fn external_name<'a>(&self, resource: &'a serde_json::Value) -> Option<&'a str> {
        resource
            .pointer("/metadata/annotations")
            .and_then(|annotations| annotations.get(self.annotation(EXTERNAL_NAME).as_str()))
            .and_then(serde_json::Value::as_str)
            .filter(|name| !name.is_empty())
    }

    #[must_use]
    pub fn reference(&self, name: &str) -> Option<&Reference> {
        self.references
            .iter()
            .find(|reference| reference.name == name)
    }
}

/// Annotation (under the resource's group) holding the external name.
pub const EXTERNAL_NAME: &str = "external-name";

/// How a string value (path parameter, identifier) is obtained.
#[derive(Debug)]
pub enum ValueExpr {
    /// Dot path into the resource as JSON, e.g. `status.atProvider.id`.
    Field(&'static str),
    /// The resource's external name annotation.
    ExternalName,
    /// An identifier of a resource reached through the named references.
    RelatedIdentifier {
        via: &'static [&'static str],
        name: &'static str,
    },
    /// The first expression that resolves to a value.
    Coalesce(&'static [ValueExpr]),
    Literal(&'static str),
}

#[derive(Debug)]
pub struct Reference {
    pub name: &'static str,
    /// Dot path of the `{name, namespace}` reference object, e.g.
    /// `spec.forProvider.zoneRef`.
    pub from: &'static str,
    pub target: &'static ResourceDescriptor,
    pub namespace: NamespacePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespacePolicy {
    /// The referenced resource lives in the referencing resource's namespace; a
    /// namespace given in the reference is ignored.
    SameAsResource,
    /// The reference's `namespace`, defaulting to the referencing resource's.
    FromReference,
    /// The referenced resource is cluster-scoped.
    ClusterScoped,
}

#[derive(Debug)]
pub struct Credentials {
    pub source: CredentialSource,
    pub auth: AuthScheme,
}

#[derive(Debug)]
pub enum CredentialSource {
    /// Dot path of a `SecretKeySelector` on this resource.
    Secret { path: &'static str },
    /// Credentials of the resource reached through the named references.
    Related { via: &'static [&'static str] },
}

#[derive(Debug, Clone, Copy)]
pub enum AuthScheme {
    Bearer,
    Header(&'static str),
}

#[derive(Debug)]
pub struct OperationDescriptor {
    pub path_parameters: &'static [(&'static str, ValueExpr)],
    /// Dot path of the resource inside the response body; empty for the whole body.
    pub response_path: &'static str,
}

#[derive(Debug)]
pub struct CreateDescriptor {
    pub operation: OperationDescriptor,
    pub request: RequestMapping,
    /// Dot path of the new resource's external name in the response body, e.g.
    /// `result.id`.
    pub external_name: &'static str,
}

/// How a request body is built: the object at `from` (usually `spec.forProvider`)
/// plus values written at dot paths. Fields the request type does not know are
/// dropped when the JSON is deserialized into it.
#[derive(Debug)]
pub struct RequestMapping {
    pub from: &'static str,
    pub set: &'static [(&'static str, ValueExpr)],
}
