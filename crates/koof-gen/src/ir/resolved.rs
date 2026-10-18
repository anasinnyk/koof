use super::{
    CredentialSourceIr, HttpIr, OperationId, OperationIr, ResourceName, TestIr, ValidationIr,
    ValueExpr,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Provider<'a> {
    pub name: &'a str,
    pub base_url: &'a str,
    pub http: &'a HttpIr,
    pub resources: Vec<Resource<'a>>,
    pub operations: Vec<Operation<'a>>,
}

#[derive(Debug, Clone)]
pub struct Resource<'a> {
    pub name: ResourceName,
    pub kind: String,
    pub base_kind: &'a str,
    pub group: &'a str,
    pub version: &'a str,
    pub namespaced: bool,
    pub has_cluster_variant: bool,
    pub identifiers: &'a BTreeMap<String, ValueExpr>,
    pub references: Vec<Reference<'a>>,
    pub credentials: Option<Credentials<'a>>,
    pub validations: &'a [ValidationIr],
    pub immutable: &'a [String],
    pub late_initialize: &'a [String],
    pub test: Option<&'a TestIr>,
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

#[derive(Debug, Clone)]
pub struct Reference<'a> {
    pub name: &'a str,
    pub from: &'a str,
    pub field: &'a str,
    pub target: &'a ResourceName,
    pub target_kind: &'a str,
    pub targets: Vec<ResourceName>,
}

#[derive(Debug, Clone)]
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

#[must_use]
pub fn cluster_name(name: &ResourceName) -> ResourceName {
    ResourceName(format!("cluster_{}", name.0))
}

#[must_use]
pub fn cluster_kind(kind: &str) -> String {
    format!("Cluster{kind}")
}
