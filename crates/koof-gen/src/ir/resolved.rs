//! A [`ProviderIr`](super::ProviderIr) that passed [`validate`](super::validate),
//! with every lookup the generator needs already done: lifecycle operations,
//! reference targets, the `apiVersion` split, the auth scheme, the field names of
//! references and credentials. Code generation cannot fail on the IR.

use super::{
    CredentialSourceIr, NamespacePolicy, OperationId, OperationIr, ResourceName, ValidationIr,
    ValueExpr,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Provider<'a> {
    pub name: &'a str,
    pub base_url: &'a str,
    /// In resource name order.
    pub resources: Vec<Resource<'a>>,
    /// Every operation, in id order.
    pub operations: Vec<Operation<'a>>,
}

#[derive(Debug)]
pub struct Resource<'a> {
    /// Also the name of the generated module.
    pub name: &'a ResourceName,
    pub kind: &'a str,
    pub group: &'a str,
    pub version: &'a str,
    pub namespaced: bool,
    pub identifiers: &'a BTreeMap<String, ValueExpr>,
    pub references: Vec<Reference<'a>>,
    pub credentials: Option<Credentials<'a>>,
    pub validations: &'a [ValidationIr],
    pub immutable: &'a [String],
    /// Top-level `forProvider` fields that may be late-initialized.
    pub late_initialize: &'a [String],
    pub observe: Operation<'a>,
    pub create: Option<Create<'a>>,
    pub update: Option<Operation<'a>>,
    pub delete: Option<Operation<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Operation<'a> {
    pub id: &'a OperationId,
    pub ir: &'a OperationIr,
}

#[derive(Debug, Clone, Copy)]
pub struct Create<'a> {
    pub operation: Operation<'a>,
    /// Dot path of the new resource's external name in the response body.
    pub external_name: &'a str,
}

#[derive(Debug)]
pub struct Reference<'a> {
    pub name: &'a str,
    /// Dot path of the reference object, e.g. `spec.forProvider.zoneRef`.
    pub from: &'a str,
    /// Its `forProvider` field, e.g. `zoneRef`.
    pub field: &'a str,
    pub target: &'a ResourceName,
    pub namespace: NamespacePolicy,
}

#[derive(Debug)]
pub struct Credentials<'a> {
    pub source: &'a CredentialSourceIr,
    pub auth: Auth<'a>,
    /// The `forProvider` field of a Secret selector on this resource.
    pub secret_field: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
pub enum Auth<'a> {
    Bearer,
    /// API key in the named header.
    Header(&'a str),
}
