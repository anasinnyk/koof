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

#[derive(Debug)]
pub struct ResourceDescriptor {
    pub kind: &'static str,
    pub api_resource: fn() -> kube::api::ApiResource,
    pub group: &'static str,
    pub namespaced: bool,
    pub identifiers: &'static [(&'static str, ValueExpr)],
    pub late_initialize: &'static [&'static str],
    pub references: &'static [Reference],
    pub credentials: Option<Credentials>,
    pub observe: OperationDescriptor,
    pub create: Option<CreateDescriptor>,
    pub update: Option<UpdateDescriptor>,
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
pub struct Reference {
    pub name: &'static str,
    pub from: &'static str,
    pub target: &'static ResourceDescriptor,
    pub namespace: NamespacePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespacePolicy {
    SameAsResource,
    FromReference,
    ClusterScoped,
}

#[derive(Debug)]
pub struct Credentials {
    pub source: CredentialSource,
    pub auth: AuthScheme,
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
pub struct OperationDescriptor {
    pub path_parameters: &'static [(&'static str, ValueExpr)],
    pub response_path: &'static str,
}

#[derive(Debug)]
pub struct CreateDescriptor {
    pub operation: OperationDescriptor,
    pub request: RequestMapping,
    pub external_name: &'static str,
}

#[derive(Debug)]
pub struct UpdateDescriptor {
    pub operation: OperationDescriptor,
    pub request: RequestMapping,
}

#[derive(Debug)]
pub struct RequestMapping {
    pub from: &'static str,
    pub set: &'static [(&'static str, ValueExpr)],
}
