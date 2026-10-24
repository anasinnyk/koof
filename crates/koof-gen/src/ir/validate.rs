use std::collections::{BTreeMap, BTreeSet};

use super::{
    ApiKeyLocationIr, CredentialSourceIr, FieldPath, OperationId, OperationIr, ProviderIr,
    ResourceIr, ResourceName, SecuritySchemeIr, ValueExpr,
    resolved::{
        Auth, Create, Credentials, Operation, Provider, Reference, Resource, cluster_kind,
        cluster_name,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct ValidationError {
    pub location: String,
    pub message: String,
}

/// Checks `ir` and resolves it for code generation. `late_initialize` holds the
/// late-initializable fields per resource (see [`crate::spec::CodegenSpec`]).
///
/// # Errors
///
/// Returns every problem found: duplicate kinds, unknown references, operations or
/// security schemes, unsupported credentials, path parameters that do not match
/// the path, a create operation that does not return the external name, and
/// reference or credential fields outside `spec.forProvider`.
pub fn validate<'a>(
    ir: &'a ProviderIr,
    late_initialize: &'a BTreeMap<ResourceName, Vec<String>>,
) -> Result<Provider<'a>, Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut kinds = BTreeMap::new();
    let mut resources = Vec::new();

    for (name, resource) in &ir.resources {
        validate_unique_kinds(name, resource, &mut kinds, &mut errors);
        let mut checker = Checker {
            ir,
            name,
            resource,
            errors: &mut errors,
        };
        let fields = late_initialize.get(name).map_or(&[][..], Vec::as_slice);
        if let Some(namespaced) = checker.resolve(fields) {
            let cluster = resource
                .cluster
                .then(|| checker.cluster_variant(&namespaced))
                .flatten();
            resources.push(namespaced);
            resources.extend(cluster);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Provider {
        name: &ir.provider,
        base_url: &ir.base_url,
        http: &ir.http,
        resources,
        operations: ir
            .operations
            .iter()
            .map(|(id, ir)| Operation { id, ir })
            .collect(),
    })
}

fn validate_unique_kinds(
    name: &ResourceName,
    resource: &ResourceIr,
    kinds: &mut BTreeMap<(String, String), ResourceName>,
    errors: &mut Vec<ValidationError>,
) {
    let mut variants = vec![(resource.kind.clone(), name.clone())];
    if resource.cluster {
        variants.push((cluster_kind(&resource.kind), cluster_name(name)));
    }
    for (kind, variant) in variants {
        if let Some(previous) = kinds.insert((resource.api_version.clone(), kind), variant.clone())
        {
            errors.push(ValidationError {
                location: format!("resource.{}", variant.0),
                message: format!("apiVersion/kind already used by resource {}", previous.0),
            });
        }
    }
}

struct Checker<'a, 'e> {
    ir: &'a ProviderIr,
    name: &'a ResourceName,
    resource: &'a ResourceIr,
    errors: &'e mut Vec<ValidationError>,
}

impl<'a> Checker<'a, '_> {
    fn resolve(&mut self, late_initialize: &'a [String]) -> Option<Resource<'a>> {
        let resource = self.resource;
        let errors_before = self.errors.len();

        let api_version = resource.api_version.split_once('/');
        if api_version.is_none() {
            self.error(
                "api_version",
                format!(
                    "invalid Kubernetes apiVersion {:?}: expected <group>/<version>",
                    resource.api_version
                ),
            );
        }
        let references = self.references();
        let credentials = self.credentials();
        let observe = self.operation("observe", Some(&resource.lifecycle.observe));
        let create = self.create();
        let update = self.operation("update", resource.lifecycle.update.as_ref());
        let delete = self.operation("delete", resource.lifecycle.delete.as_ref());

        if self.errors.len() != errors_before {
            return None;
        }
        let (group, version) = api_version?;
        Some(Resource {
            name: self.name.clone(),
            kind: resource.kind.clone(),
            base_kind: &resource.kind,
            group,
            version,
            namespaced: true,
            has_cluster_variant: resource.cluster,
            identifiers: &resource.identifiers,
            references,
            credentials,
            validations: &resource.validations,
            immutable: &resource.immutable,
            late_initialize,
            test: resource.test.as_ref(),
            observe: observe?,
            create,
            update,
            delete,
        })
    }

    fn cluster_variant(&mut self, namespaced: &Resource<'a>) -> Option<Resource<'a>> {
        let errors_before = self.errors.len();
        let mut references = namespaced.references.clone();
        for reference in &mut references {
            let has_cluster_variant = self
                .ir
                .resources
                .get(reference.target)
                .is_some_and(|target| target.cluster);
            if !has_cluster_variant {
                self.error(
                    &format!("references.{}", reference.name),
                    format!(
                        "{} cannot reference {}: it has no cluster variant (set cluster: true on it)",
                        cluster_kind(namespaced.base_kind),
                        reference.target.0
                    ),
                );
            }
            reference.targets = vec![cluster_name(reference.target)];
        }
        if self.errors.len() != errors_before {
            return None;
        }
        Some(Resource {
            name: cluster_name(self.name),
            kind: cluster_kind(namespaced.base_kind),
            namespaced: false,
            has_cluster_variant: false,
            references,
            ..namespaced.clone()
        })
    }

    fn error(&mut self, location: &str, message: impl Into<String>) {
        self.errors.push(ValidationError {
            location: format!("resources.{}.{location}", self.name.0),
            message: message.into(),
        });
    }

    fn references(&mut self) -> Vec<Reference<'a>> {
        let mut references = Vec::new();
        for (name, reference) in &self.resource.references {
            let location = format!("references.{name}");
            let Some(target) = self.ir.resources.get(&reference.target) else {
                self.error(
                    &location,
                    format!("unknown target resource {}", reference.target.0),
                );
                continue;
            };
            if self.resource.test.is_some() && target.test.is_none() {
                self.error(
                    "test",
                    format!(
                        "the test needs a {0} object; add a test to resource {0}",
                        reference.target.0
                    ),
                );
            }
            let Some(field) = self.for_provider_field(&location, &reference.from) else {
                continue;
            };
            let mut targets = vec![reference.target.clone()];
            if target.cluster {
                targets.push(cluster_name(&reference.target));
            }
            let (target_group, target_version) =
                target.api_version.split_once('/').unwrap_or_default();
            references.push(Reference {
                name,
                from: &reference.from.0,
                field,
                target: &reference.target,
                target_kind: &target.kind,
                target_group,
                target_version,
                targets,
            });
        }
        references
    }

    fn credentials(&mut self) -> Option<Credentials<'a>> {
        let credentials = self.resource.credentials.as_ref()?;

        let auth = match self.ir.security_schemes.get(&credentials.security_scheme) {
            None => {
                self.error(
                    "credentials.security_scheme",
                    format!(
                        "security scheme {:?} does not exist",
                        credentials.security_scheme.0,
                    ),
                );
                None
            }
            Some(SecuritySchemeIr::Http { scheme }) if scheme == "bearer" => Some(Auth::Bearer),
            Some(SecuritySchemeIr::Http { scheme }) => {
                self.error(
                    "credentials.security_scheme",
                    format!("HTTP authentication scheme {scheme:?} is not supported yet"),
                );
                None
            }
            Some(SecuritySchemeIr::ApiKey {
                name,
                location: ApiKeyLocationIr::Header,
            }) => Some(Auth::Header(name)),
            Some(SecuritySchemeIr::ApiKey { .. }) => {
                self.error(
                    "credentials.security_scheme",
                    "query and cookie API keys are not supported yet",
                );
                None
            }
        };

        let secret_field = match &credentials.source {
            CredentialSourceIr::SecretKeySelector { path } => {
                self.for_provider_field("credentials", path)
            }
            CredentialSourceIr::Related { via } => {
                match resolve_relation(self.ir, self.name, via) {
                    Ok(target) => {
                        let has_direct_credentials =
                            target.credentials.as_ref().is_some_and(|credentials| {
                                matches!(
                                    &credentials.source,
                                    CredentialSourceIr::SecretKeySelector { .. }
                                )
                            });
                        if !has_direct_credentials {
                            self.error(
                                "credentials",
                                "credential relation must end at a resource with direct Secret \
                                 credentials",
                            );
                        }
                    }
                    Err(message) => self.error("credentials", message),
                }
                None
            }
        };

        Some(Credentials {
            source: &credentials.source,
            auth: auth?,
            secret_field,
        })
    }

    fn for_provider_field(&mut self, location: &str, path: &'a FieldPath) -> Option<&'a str> {
        let Some(field) = path.0.strip_prefix("spec.forProvider.") else {
            self.error(
                location,
                format!("path must start with spec.forProvider.: {}", path.0),
            );
            return None;
        };
        if field.contains('.') || field.is_empty() {
            self.error(
                location,
                format!("nested paths are not supported: {}", path.0),
            );
            return None;
        }
        Some(field)
    }

    fn create(&mut self) -> Option<Create<'a>> {
        let operation = self.operation("create", self.resource.lifecycle.create.as_ref())?;

        let Some(identifier) = self
            .resource
            .identifiers
            .iter()
            .find(|(_, expression)| matches!(expression, ValueExpr::ExternalName))
            .map(|(identifier, _)| identifier)
        else {
            self.error(
                "identifiers",
                "resource can be created but has no identifier of type external_name",
            );
            return None;
        };
        let Some(external_name) = operation
            .ir
            .result_identifiers
            .get(identifier)
            .and_then(|path| path.strip_prefix("response.body."))
        else {
            self.error(
                "lifecycle.create",
                format!(
                    "create operation {} must declare result.identifiers.{identifier}: \
                     response.body.<path>",
                    operation.id.0
                ),
            );
            return None;
        };
        Some(Create {
            operation,
            external_name,
        })
    }

    fn operation(&mut self, action: &str, id: Option<&'a OperationId>) -> Option<Operation<'a>> {
        let id = id?;
        let Some(operation) = self.ir.operations.get(id) else {
            self.error(
                &format!("lifecycle.{action}"),
                format!("unknown operation {}", id.0),
            );
            return None;
        };

        validate_path_parameters(id, operation, self.errors);
        for (parameter, expression) in &operation.path_parameters {
            validate_expression(
                self.ir,
                self.name,
                expression,
                &format!("operations.{}.path_parameters.{parameter}", id.0),
                self.errors,
            );
        }
        Some(Operation { id, ir: operation })
    }
}

fn resolve_relation<'a>(
    ir: &'a ProviderIr,
    start: &ResourceName,
    via: &[String],
) -> Result<&'a ResourceIr, String> {
    let mut current = ir
        .resources
        .get(start)
        .ok_or_else(|| format!("unknown resource {}", start.0))?;

    for relation_name in via {
        let relation = current
            .references
            .get(relation_name)
            .ok_or_else(|| format!("unknown relation {relation_name}"))?;

        current = ir
            .resources
            .get(&relation.target)
            .ok_or_else(|| format!("unknown resource {}", relation.target.0))?;
    }

    Ok(current)
}

fn validate_path_parameters(
    id: &OperationId,
    operation: &OperationIr,
    errors: &mut Vec<ValidationError>,
) {
    let expected = placeholders(&operation.path);
    let actual = operation
        .path_parameters
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    if expected != actual {
        errors.push(ValidationError {
            location: format!("operations.{}.path_parameters", id.0),
            message: format!("path expects {expected:?}, but bindings are {actual:?}"),
        });
    }
}

fn validate_expression(
    ir: &ProviderIr,
    resource: &ResourceName,
    expression: &ValueExpr,
    location: &str,
    errors: &mut Vec<ValidationError>,
) {
    match expression {
        ValueExpr::RelatedIdentifier { via, name } => match resolve_relation(ir, resource, via) {
            Ok(target) if !target.identifiers.contains_key(name) => {
                errors.push(ValidationError {
                    location: location.into(),
                    message: format!("target resource has no identifier {name}"),
                });
            }
            Err(message) => errors.push(ValidationError {
                location: location.into(),
                message,
            }),
            _ => {}
        },

        ValueExpr::Coalesce { values } => {
            if values.is_empty() {
                errors.push(ValidationError {
                    location: location.into(),
                    message: "coalesce cannot be empty".into(),
                });
            }

            for value in values {
                validate_expression(ir, resource, value, location, errors);
            }
        }

        ValueExpr::Field { .. } | ValueExpr::ExternalName | ValueExpr::Literal { .. } => {}
    }
}

fn placeholders(path: &str) -> BTreeSet<&str> {
    path.split('{')
        .skip(1)
        .filter_map(|part| part.split_once('}'))
        .map(|(name, _)| name)
        .collect()
}
