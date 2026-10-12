use super::{
    CredentialSourceIr, NamespacePolicy, OperationId, OperationIr, ResourceName, ValidationIr,
    ValueExpr,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Provider<'a> {
    pub name: &'a str,
    pub base_url: &'a str,
    pub resources: Vec<Resource<'a>>,
    pub operations: Vec<Operation<'a>>,
}

#[derive(Debug)]
pub struct Resource<'a> {
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
    pub external_name: &'a str,
}

#[derive(Debug)]
pub struct Reference<'a> {
    pub name: &'a str,
    pub from: &'a str,
    pub field: &'a str,
    pub target: &'a ResourceName,
    pub namespace: NamespacePolicy,
}

#[derive(Debug)]
pub struct Credentials<'a> {
    pub source: &'a CredentialSourceIr,
    pub auth: Auth<'a>,
    pub secret_field: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
pub enum Auth<'a> {
    Bearer,
    Header(&'a str),
}
