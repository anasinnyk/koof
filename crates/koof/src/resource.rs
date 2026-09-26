//! What `koof-gen` emits per resource: a static [`ResourceDescriptor`] and a
//! [`ManagedResource`] impl. Everything else is generic runtime code.

use crate::{http::Operation, status::ManagedStatus};
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

    fn descriptor() -> &'static ResourceDescriptor;

    fn status(&self) -> Option<&ManagedStatus<Self::AtProvider>>;
}

/// Static description of a resource: how to find its identifiers, related
/// resources, credentials and observe parameters.
#[derive(Debug)]
pub struct ResourceDescriptor {
    pub kind: &'static str,
    /// Used to read this resource as a `DynamicObject` when another resource
    /// references it.
    pub api_resource: fn() -> kube::api::ApiResource,
    pub identifiers: &'static [(&'static str, ValueExpr)],
    pub references: &'static [Reference],
    pub credentials: Option<Credentials>,
    pub observe: OperationDescriptor,
}

impl ResourceDescriptor {
    #[must_use]
    pub fn identifier(&self, name: &str) -> Option<&ValueExpr> {
        self.identifiers
            .iter()
            .find(|(identifier, _)| *identifier == name)
            .map(|(_, expression)| expression)
    }

    #[must_use]
    pub fn reference(&self, name: &str) -> Option<&Reference> {
        self.references
            .iter()
            .find(|reference| reference.name == name)
    }
}

/// How a string value (path parameter, identifier) is obtained.
#[derive(Debug)]
pub enum ValueExpr {
    /// Dot path into the resource as JSON, e.g. `status.atProvider.id`.
    Field(&'static str),
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
