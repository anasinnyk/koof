use crate::{http::Operation, managed::ManagedResourceSpec, status::ManagedStatus};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub trait ManagedResource:
    kube::Resource<DynamicType = ()>
    + Clone
    + Debug
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + 'static
{
    type AtProvider: Serialize
        + DeserializeOwned
        + JsonSchema
        + Clone
        + Debug
        + PartialEq
        + Send
        + Sync;

    type Observe: Operation;

    type Create: Operation<Request: DeserializeOwned>;

    type Update: Operation<Request: DeserializeOwned>;

    type Delete: Operation;

    fn descriptor() -> &'static ResourceDescriptor;

    fn status(&self) -> Option<&ManagedStatus<Self::AtProvider>>;

    fn management(&self) -> &ManagedResourceSpec;
}

/// The identity of a resource type, displayed as `<group>/<kind>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupKind {
    pub group: &'static str,
    pub kind: &'static str,
}

impl std::fmt::Display for GroupKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}/{}", self.group, self.kind)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ResourceDescriptor {
    pub kind: &'static str,
    pub api_resource: fn() -> kube::api::ApiResource,
    pub group: &'static str,
    pub namespaced: bool,
    pub identifiers: &'static [(&'static str, ValueExpr)],
    pub late_initialize: &'static [&'static str],
    pub references: &'static [Reference],
    pub credentials: Option<Credentials>,
    pub observe: ObserveDescriptor,
    pub create: Option<CreateDescriptor>,
    pub update: Option<UpdateDescriptor>,
    pub delete: Option<OperationDescriptor>,
}

impl ResourceDescriptor {
    #[must_use]
    pub const fn new(
        kind: &'static str,
        api_resource: fn() -> kube::api::ApiResource,
        group: &'static str,
        namespaced: bool,
        observe: ObserveDescriptor,
    ) -> Self {
        Self {
            kind,
            api_resource,
            group,
            namespaced,
            identifiers: &[],
            late_initialize: &[],
            references: &[],
            credentials: None,
            observe,
            create: None,
            update: None,
            delete: None,
        }
    }

    #[must_use]
    pub const fn group_kind(&self) -> GroupKind {
        GroupKind {
            group: self.group,
            kind: self.kind,
        }
    }

    #[must_use]
    pub const fn with_identifiers(self, identifiers: &'static [(&'static str, ValueExpr)]) -> Self {
        Self {
            identifiers,
            ..self
        }
    }

    #[must_use]
    pub const fn with_late_initialize(self, late_initialize: &'static [&'static str]) -> Self {
        Self {
            late_initialize,
            ..self
        }
    }

    #[must_use]
    pub const fn with_references(self, references: &'static [Reference]) -> Self {
        Self { references, ..self }
    }

    #[must_use]
    pub const fn with_credentials(self, credentials: Credentials) -> Self {
        Self {
            credentials: Some(credentials),
            ..self
        }
    }

    #[must_use]
    pub const fn with_create(self, create: CreateDescriptor) -> Self {
        Self {
            create: Some(create),
            ..self
        }
    }

    #[must_use]
    pub const fn with_update(self, update: UpdateDescriptor) -> Self {
        Self {
            update: Some(update),
            ..self
        }
    }

    #[must_use]
    pub const fn with_delete(self, delete: OperationDescriptor) -> Self {
        Self {
            delete: Some(delete),
            ..self
        }
    }

    #[must_use]
    pub fn identifier(&self, name: &str) -> Option<&ValueExpr> {
        self.identifiers
            .iter()
            .find(|(identifier, _)| *identifier == name)
            .map(|(_, expression)| expression)
    }

    #[must_use]
    pub fn annotation(&self, name: &str) -> String {
        format!("{}/{name}", self.group)
    }

    #[must_use]
    pub fn finalizer(&self) -> String {
        self.annotation("finalizer")
    }

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

pub const EXTERNAL_NAME: &str = "external-name";

#[derive(Debug)]
pub enum ValueExpr {
    Field(&'static str),
    ExternalName,
    RelatedIdentifier {
        via: &'static [&'static str],
        name: &'static str,
    },
    Coalesce(&'static [ValueExpr]),
    Literal(&'static str),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Reference {
    pub name: &'static str,
    pub from: &'static str,
    /// The kinds the reference can point at, told apart by its `kind` field; the
    /// first one is the default.
    pub targets: &'static [&'static ResourceDescriptor],
}

impl Reference {
    #[must_use]
    pub const fn new(
        name: &'static str,
        from: &'static str,
        targets: &'static [&'static ResourceDescriptor],
    ) -> Self {
        Self {
            name,
            from,
            targets,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Credentials {
    pub source: CredentialSource,
    pub auth: AuthScheme,
}

impl Credentials {
    #[must_use]
    pub const fn new(source: CredentialSource, auth: AuthScheme) -> Self {
        Self { source, auth }
    }
}

#[derive(Debug)]
pub enum CredentialSource {
    Secret { path: &'static str },
    Related { via: &'static [&'static str] },
}

#[derive(Debug, Clone, Copy)]
pub enum AuthScheme {
    Bearer,
    Header(&'static str),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct OperationDescriptor {
    pub path_parameters: &'static [(&'static str, ValueExpr)],
    pub response_path: &'static str,
}

impl OperationDescriptor {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            path_parameters: &[],
            response_path: "",
        }
    }

    #[must_use]
    pub const fn with_path_parameters(
        self,
        path_parameters: &'static [(&'static str, ValueExpr)],
    ) -> Self {
        Self {
            path_parameters,
            ..self
        }
    }

    #[must_use]
    pub const fn with_response_path(self, response_path: &'static str) -> Self {
        Self {
            response_path,
            ..self
        }
    }
}

impl Default for OperationDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ObserveDescriptor {
    pub operation: OperationDescriptor,
    pub deleted_when: Option<&'static str>,
}

impl ObserveDescriptor {
    #[must_use]
    pub const fn new(operation: OperationDescriptor) -> Self {
        Self {
            operation,
            deleted_when: None,
        }
    }

    #[must_use]
    pub const fn with_deleted_when(self, deleted_when: &'static str) -> Self {
        Self {
            deleted_when: Some(deleted_when),
            ..self
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct CreateDescriptor {
    pub operation: OperationDescriptor,
    pub request: RequestMapping,
    pub external_name: &'static str,
}

impl CreateDescriptor {
    #[must_use]
    pub const fn new(
        operation: OperationDescriptor,
        request: RequestMapping,
        external_name: &'static str,
    ) -> Self {
        Self {
            operation,
            request,
            external_name,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct UpdateDescriptor {
    pub operation: OperationDescriptor,
    pub request: RequestMapping,
}

impl UpdateDescriptor {
    #[must_use]
    pub const fn new(operation: OperationDescriptor, request: RequestMapping) -> Self {
        Self { operation, request }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct RequestMapping {
    pub from: &'static str,
    pub set: &'static [(&'static str, ValueExpr)],
}

impl RequestMapping {
    #[must_use]
    pub const fn new(from: &'static str) -> Self {
        Self { from, set: &[] }
    }

    #[must_use]
    pub const fn with_set(self, set: &'static [(&'static str, ValueExpr)]) -> Self {
        Self { set, ..self }
    }
}
