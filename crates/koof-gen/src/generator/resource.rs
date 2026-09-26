use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, kind_ident, operation_ident},
        operations::generate_operations,
        types::GeneratedTypes,
    },
    ir::{
        ApiKeyLocationIr, CredentialSourceIr, FieldPath, OperationIr, ProviderIr, ResourceIr,
        ResourceName, SecuritySchemeIr, ValueExpr,
    },
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

pub fn generate_resource(
    ir: &ProviderIr,
    types: &GeneratedTypes,
    name: &ResourceName,
    resource: &ResourceIr,
) -> Result<GeneratedFile, GeneratorError> {
    let custom_resource = generate_custom_resource(&[
        generate_spec(resource, types)?,
        generate_status(resource, types)?,
        generate_observe(ir, name, resource)?,
        generate_resolve_credentials(ir, name, resource)?,
        generate_update_status(resource, types)?,
        generate_reconciler(resource)?,
        generate_run_controller(resource)?,
    ]);

    let content = format_tokens(custom_resource)?;

    Ok(GeneratedFile {
        path: format!("src/generated/{}.rs", name.0).into(),
        content,
    })
}

fn generate_custom_resource(tokens: &[TokenStream]) -> TokenStream {
    generated_file(&quote!(#(#tokens)*))
}

/// Wraps generated items with the common file header. Pedantic lints are allowed
/// because doc comments come verbatim from the upstream `OpenAPI` document.
fn generated_file(items: &TokenStream) -> TokenStream {
    quote! {
        #![doc = " This file is generated. Do not edit manually."]
        #![allow(clippy::pedantic)]

        #items
    }
}

pub struct GeneratedPathParameters {
    pub bindings: Vec<TokenStream>,
    pub arguments: Vec<TokenStream>,
}

pub fn generate_run_controller(resource: &ResourceIr) -> Result<TokenStream, GeneratorError> {
    let resource_ident = kind_ident(&resource.kind)?;
    let controller_name = &resource.kind;

    Ok(quote! {
        pub async fn run_controller(
            context: std::sync::Arc<
                koof::reconciler::ControllerContext<koof::http::HttpClient>
            >,
        ) {
            use futures::StreamExt as _;

            let api: kube::Api<#resource_ident> =
                kube::Api::all(
                    context.kube_client.clone(),
                );

            kube::runtime::Controller::new(
                api,
                kube::runtime::watcher::Config::default(),
            )
            .shutdown_on_signal()
            .run(
                reconcile,
                error_policy,
                context,
            )
            .for_each(|result| async move {
                match result {
                    Ok((object, action)) => {
                        tracing::debug!(
                            resource = #controller_name,
                            ?object,
                            ?action,
                            "reconciliation completed",
                        );
                    }

                    Err(error) => {
                        tracing::error!(
                            resource = #controller_name,
                            ?error,
                            "reconciliation failed",
                        );
                    }
                }
            })
            .await;
        }
    })
}

pub fn generate_path_parameters(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    operation: &OperationIr,
) -> Result<GeneratedPathParameters, GeneratorError> {
    let mut bindings = Vec::new();
    let mut arguments = Vec::new();

    for (parameter_name, expression) in &operation.path_parameters {
        let parameter_ident = field_ident(parameter_name)?;

        let value = generate_value_expr(ir, resource_name, &quote!(resource), expression)?;

        bindings.push(quote! {
            let #parameter_ident = (#value)
                .await?
                .ok_or_else(|| {
                    koof::error::ResolveValueError::Missing(
                        format!(
                            "could not resolve path parameter {}",
                            #parameter_name,
                        ),
                    )
                })?;
        });

        arguments.push(quote!((#parameter_name, #parameter_ident.as_str())));
    }

    Ok(GeneratedPathParameters {
        bindings,
        arguments,
    })
}

pub fn generate_observe(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let operation_id = &resource.lifecycle.observe;

    let operation = ir.operations.get(operation_id).ok_or_else(|| {
        GeneratorError::InvalidIr(format!("unknown observe operation {}", operation_id.0))
    })?;

    if resource.credentials.is_none() {
        return Err(GeneratorError::UnsupportedCredential(format!(
            "resource {} has no credentials",
            resource_name.0
        )));
    }

    let GeneratedPathParameters {
        bindings,
        arguments,
    } = generate_path_parameters(ir, resource_name, operation)?;

    let resource_ident = kind_ident(&resource.kind)?;
    let operation_type = operation_ident(operation_id)?;
    let response_path = operation.response_path.clone().unwrap_or_default();

    Ok(quote! {
        pub async fn observe(
            kube_client: &kube::Client,
            http: &koof::http::HttpClient,
            resource: &#resource_ident,
        ) -> Result<
            koof::reconciler::Observation,
            koof::error::ReconcileError,
        > {
            #(#bindings)*

            let auth = resolve_credentials(kube_client, resource).await?;

            let response = http
                .send::<crate::generated::operations::#operation_type>(
                    &auth,
                    &[#(#arguments),*],
                    None,
                )
                .await?;

            let at_provider = match &response {
                Some(response) => koof::http::extract(response, #response_path)?,
                None => None,
            };

            Ok(koof::reconciler::Observation {
                exists: response.is_some(),
                at_provider,
            })
        }
    })
}

fn generate_value_expr(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    resource_value: &TokenStream,
    expression: &ValueExpr,
) -> Result<TokenStream, GeneratorError> {
    match expression {
        ValueExpr::Field { path } => {
            let path = &path.0;

            Ok(quote! {
                async {
                    koof::reference::resolve_field_value(
                        #resource_value,
                        #path,
                    )
                }
            })
        }

        ValueExpr::Literal { value } => Ok(quote! {
            async {
                Ok::<
                    Option<String>,
                    koof::error::ResolveValueError,
                >(Some(#value.to_owned()))
            }
        }),

        ValueExpr::Coalesce { values } => {
            let values = values
                .iter()
                .map(|value| generate_value_expr(ir, resource_name, &resource_value.clone(), value))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(quote! {
                async {
                    let mut resolved: Option<String> = None;

                    #(
                        if resolved.is_none() {
                            resolved = (#values).await?;
                        }
                    )*

                    Ok::<
                        Option<String>,
                        koof::error::ResolveValueError,
                    >(resolved)
                }
            })
        }

        ValueExpr::RelatedIdentifier { via, name } => {
            generate_related_identifier(ir, resource_name, &resource_value.clone(), via, name)
        }
    }
}

fn generate_related_identifier(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    resource_value: &TokenStream,
    via: &[String],
    identifier_name: &str,
) -> Result<TokenStream, GeneratorError> {
    let mut current_resource_name = resource_name;
    let mut current_resource = ir.resources.get(resource_name).ok_or_else(|| {
        GeneratorError::InvalidIr(format!("unknown resource {}", resource_name.0))
    })?;

    let mut current_value = resource_value.clone();
    let mut current_namespace = quote!(root_namespace);
    let mut traversal = Vec::new();

    for (index, relation_name) in via.iter().enumerate() {
        let relation = current_resource
            .references
            .get(relation_name)
            .ok_or_else(|| {
                GeneratorError::InvalidIr(format!(
                    "resource {} has no relation {}",
                    resource_name.0, relation_name,
                ))
            })?;

        let target = ir.resources.get(&relation.target).ok_or_else(|| {
            GeneratorError::InvalidIr(format!("unknown resource {}", relation.target.0))
        })?;

        let reference_access = field_path_access(current_value.clone(), &relation.from)?;

        let reference_ident = format_ident!("reference_{index}");
        let namespace_ident = format_ident!("namespace_{index}");
        let api_ident = format_ident!("api_{index}");
        let related_ident = format_ident!("related_{index}");

        let target_module = field_ident(&relation.target.0)?;
        let target_ident = kind_ident(&target.kind)?;

        let reference_path = &relation.from.0;

        traversal.push(quote! {
            let #reference_ident = (#reference_access)
                .as_ref()
                .ok_or_else(|| {
                    koof::error::ResolveValueError::Missing(
                        format!(
                            "resource reference {} is not set",
                            #reference_path,
                        ),
                    )
                })?;

            let #namespace_ident = #reference_ident
                .namespace
                .clone()
                .unwrap_or_else(|| #current_namespace.clone());

            let #api_ident: kube::Api<
                crate::generated::#target_module::#target_ident
            > = kube::Api::namespaced(
                kube_client.clone(),
                &#namespace_ident,
            );

            let #related_ident = #api_ident
                .get(&#reference_ident.name)
                .await?;
        });

        current_resource = target;
        current_resource_name = &relation.target;
        current_value = quote!(&#related_ident);
        current_namespace = quote!(#namespace_ident);
    }

    let identifier_expression = current_resource
        .identifiers
        .get(identifier_name)
        .ok_or_else(|| {
            GeneratorError::InvalidIr(format!(
                "resource {} has no identifier {}",
                current_resource.kind, identifier_name,
            ))
        })?;

    let identifier_value = generate_value_expr(
        ir,
        current_resource_name,
        &current_value,
        identifier_expression,
    )?;

    Ok(quote! {
        async {
            let root_namespace =
                kube::ResourceExt::namespace(#resource_value)
                    .ok_or_else(|| {
                        koof::error::ResolveValueError::Missing(
                            "resource has no namespace".into(),
                        )
                    })?;

            #(#traversal)*

            (#identifier_value).await
        }
    })
}

fn format_tokens(tokens: TokenStream) -> Result<String, GeneratorError> {
    let syntax: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax))
}

fn generate_spec(
    resource: &ResourceIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let (group, version) = resource
        .api_version
        .split_once('/')
        .ok_or_else(|| GeneratorError::InvalidApiVersion(resource.api_version.clone()))?;

    let kind = &resource.kind;
    let status_name = format!("{kind}Status");
    let spec_ident = kind_ident(&format!("{kind}Spec"))?;
    let for_provider_ident = kind_ident(&format!("{kind}ForProviderSpec"))?;
    let fields_type = types.path(&format!("{kind}ForProvider"))?;

    let mut fields = generate_reference_fields(resource)?;
    if let Some(credentials) = generate_credential_field(resource)? {
        fields.push(credentials);
    }

    Ok(quote! {
        /// `spec.forProvider`: the generated API fields plus references and
        /// credentials.
        #[derive(
            Clone,
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        #[schemars(transform = koof::schema::flatten_object_unions)]
        pub struct #for_provider_ident {
            #[serde(flatten)]
            pub fields: #fields_type,

            #(#fields)*
        }

        #[derive(
            kube::CustomResource,
            Clone,
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        #[kube(
            group = #group,
            version = #version,
            kind = #kind,
            namespaced,
            status = #status_name
        )]
        pub struct #spec_ident {
            #[serde(rename = "forProvider")]
            pub for_provider: #for_provider_ident,

            #[serde(flatten)]
            pub management:
                koof::managed::ManagedResourceSpec,
        }
    })
}

pub fn generate_update_status(
    resource: &ResourceIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let resource_ident = kind_ident(&resource.kind)?;
    let status_ident = kind_ident(&format!("{}Status", resource.kind))?;
    let at_provider_ident = types.path(&format!("{}AtProvider", resource.kind))?;

    Ok(quote! {
        pub async fn update_status(
            client: &kube::Client,
            resource: &#resource_ident,
            observation: &koof::reconciler::Observation,
        ) -> Result<(), koof::error::ReconcileError> {
            let namespace =
                kube::ResourceExt::namespace(resource)
                    .ok_or_else(|| {
                        koof::error::ResolveValueError::Missing(
                            "resource has no namespace".into(),
                        )
                    })?;

            let name =
                kube::ResourceExt::name_any(resource);

            let at_provider: Option<#at_provider_ident> =
                observation
                    .at_provider
                    .clone()
                    .map(serde_json::from_value)
                    .transpose()?;

            let condition_status = if observation.exists {
                "True"
            } else {
                "False"
            };

            let reason = if observation.exists {
                "Available"
            } else {
                "NotFound"
            };

            let message = if observation.exists {
                "External resource exists"
            } else {
                "External resource does not exist"
            };

            let previous_condition = resource
                .status
                .as_ref()
                .and_then(|status| {
                    status.conditions.iter().find(|condition| {
                        condition.type_ == "Ready"
                    })
                });

            let last_transition_time = previous_condition
                .filter(|condition| {
                    condition.status == condition_status
                        && condition.reason == reason
                })
                .map(|condition| {
                    condition.last_transition_time.clone()
                })
                .unwrap_or_else(|| {
                    k8s_openapi::apimachinery::pkg::apis::meta::v1::Time::from(
                        jiff::Timestamp::now(),
                    )
                });

            let condition =
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition {
                    type_: "Ready".into(),
                    status: condition_status.into(),
                    reason: reason.into(),
                    message: message.into(),
                    observed_generation: resource.metadata.generation,
                    last_transition_time,
                };

            let status = #status_ident {
                at_provider,
                observed_generation: resource.metadata.generation,
                conditions: vec![condition],
            };

            let patch = serde_json::json!({
                "status": status,
            });

            let api: kube::Api<#resource_ident> =
                kube::Api::namespaced(
                    client.clone(),
                    &namespace,
                );

            let current_status = serde_json::to_value(&resource.status)?;
            let desired_status = serde_json::to_value(Some(&status))?;

            if current_status == desired_status {
                return Ok(());
            }

            api.patch_status(
                &name,
                &kube::api::PatchParams::default(),
                &kube::api::Patch::Merge(&patch),
            )
            .await?;

            Ok(())
        }
    })
}

pub fn generate_reconciler(resource: &ResourceIr) -> Result<TokenStream, GeneratorError> {
    let resource_ident = kind_ident(&resource.kind)?;

    Ok(quote! {
        pub async fn reconcile(
            resource: std::sync::Arc<#resource_ident>,
            context: std::sync::Arc<
                koof::reconciler::ControllerContext<koof::http::HttpClient>
            >,
        ) -> Result<
            kube::runtime::controller::Action,
            koof::error::ReconcileError,
        > {
            let observation = observe(
                &context.kube_client,
                &context.provider_client,
                resource.as_ref(),
            )
            .await?;

            update_status(
                &context.kube_client,
                resource.as_ref(),
                &observation,
            )
            .await?;

            let requeue_after = if observation.exists {
                std::time::Duration::from_secs(300)
            } else {
                // @TODO: IMPLEMENT CREATE
                std::time::Duration::from_secs(30)
            };

            Ok(
                kube::runtime::controller::Action::requeue(
                    requeue_after,
                ),
            )
        }

        pub fn error_policy(
            _resource: std::sync::Arc<#resource_ident>,
            _error: &koof::error::ReconcileError,
            _context: std::sync::Arc<
                koof::reconciler::ControllerContext<koof::http::HttpClient>
            >,
        ) -> kube::runtime::controller::Action {
            kube::runtime::controller::Action::requeue(
                std::time::Duration::from_secs(30),
            )
        }
    })
}

fn generate_status(
    resource: &ResourceIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let status_ident = kind_ident(&format!("{}Status", resource.kind))?;
    let at_provider_type = types.path(&format!("{}AtProvider", resource.kind))?;

    Ok(quote! {
        #[derive(
            Clone,
            Debug,
            Default,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        pub struct #status_ident {
            #[serde(
                rename = "atProvider",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            #[schemars(transform = koof::schema::flatten_object_unions)]
            pub at_provider: Option<#at_provider_type>,

            #[serde(
                rename = "observedGeneration",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub observed_generation: Option<i64>,

            #[serde(default)]
            pub conditions: Vec<
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition
            >,
        }
    })
}

fn generate_reference_fields(resource: &ResourceIr) -> Result<Vec<TokenStream>, GeneratorError> {
    resource
        .references
        .values()
        .map(|reference| {
            let json_name = for_provider_field(&reference.from)?;
            let ident = field_ident(json_name)?;

            Ok(quote! {
                #[serde(rename = #json_name, default, skip_serializing_if = "Option::is_none")]
                pub #ident:
                    Option<koof::reference::ResourceReference>,
            })
        })
        .collect()
}

fn generate_credential_field(resource: &ResourceIr) -> Result<Option<TokenStream>, GeneratorError> {
    let Some(credentials) = &resource.credentials else {
        return Ok(None);
    };

    let CredentialSourceIr::SecretKeySelector { path } = &credentials.source else {
        return Ok(None);
    };

    let json_name = for_provider_field(path)?;
    let ident = field_ident(json_name)?;

    Ok(Some(quote! {
        #[serde(rename = #json_name, default, skip_serializing_if = "Option::is_none")]
        pub #ident:
            Option<koof::reference::SecretReference>,
    }))
}

fn for_provider_field(path: &FieldPath) -> Result<&str, GeneratorError> {
    let field = path.0.strip_prefix("spec.forProvider.").ok_or_else(|| {
        GeneratorError::UnsupportedSchema(format!(
            "reference path must start with \
                 spec.forProvider.: {}",
            path.0
        ))
    })?;

    if field.contains('.') {
        return Err(GeneratorError::UnsupportedSchema(format!(
            "nested reference paths are not supported: {}",
            path.0
        )));
    }

    Ok(field)
}

fn field_path_access(root: TokenStream, path: &FieldPath) -> Result<TokenStream, GeneratorError> {
    path.0.split('.').try_fold(root, |access, segment| {
        let field = field_ident(segment)?;
        Ok(quote!(#access.#field))
    })
}

// Duplicates the reference traversal of `generate_related_identifier`; both move into
// the generic runtime in phase 5 of docs/refactor-plan.md.
#[allow(clippy::too_many_lines)]
pub fn generate_resolve_credentials(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let Some(credentials) = &resource.credentials else {
        return Ok(TokenStream::new());
    };

    let resource_ident = kind_ident(&resource.kind)?;

    let auth = match ir.security_schemes.get(&credentials.security_scheme) {
        Some(SecuritySchemeIr::Http { scheme, .. }) if scheme == "bearer" => {
            quote!(koof::http::Auth::Bearer(value))
        }
        Some(SecuritySchemeIr::ApiKey {
            name,
            location: ApiKeyLocationIr::Header,
        }) => quote!(koof::http::Auth::Header {
            name: #name.to_owned(),
            value,
        }),
        _ => {
            return Err(GeneratorError::UnsupportedCredential(format!(
                "security scheme {} is not supported",
                credentials.security_scheme.0,
            )));
        }
    };

    let mut traversal = Vec::new();
    let mut current_resource = resource;
    let mut current_value = quote!(resource);
    let mut current_namespace = quote!(namespace);

    match &credentials.source {
        CredentialSourceIr::SecretKeySelector { .. } => {}

        CredentialSourceIr::Related { via } => {
            for (index, relation_name) in via.iter().enumerate() {
                let relation = current_resource
                    .references
                    .get(relation_name)
                    .ok_or_else(|| {
                        GeneratorError::UnsupportedCredential(format!(
                            "resource {} has no relation {}",
                            resource_name.0, relation_name,
                        ))
                    })?;

                let target = ir.resources.get(&relation.target).ok_or_else(|| {
                    GeneratorError::UnsupportedCredential(format!(
                        "unknown related resource {}",
                        relation.target.0,
                    ))
                })?;

                let reference_access = field_path_access(current_value.clone(), &relation.from)?;

                let reference_ident = syn::Ident::new(
                    &format!("reference_{index}"),
                    proc_macro2::Span::call_site(),
                );

                let namespace_ident = syn::Ident::new(
                    &format!("namespace_{index}"),
                    proc_macro2::Span::call_site(),
                );

                let api_ident =
                    syn::Ident::new(&format!("api_{index}"), proc_macro2::Span::call_site());

                let value_ident =
                    syn::Ident::new(&format!("related_{index}"), proc_macro2::Span::call_site());

                let target_module = field_ident(&relation.target.0)?;

                let target_ident = kind_ident(&target.kind)?;

                let reference_path = &relation.from.0;

                let target_type = quote! {
                    crate::generated::#target_module::#target_ident
                };

                traversal.push(quote! {
                    let #reference_ident =
                        (#reference_access)
                            .as_ref()
                            .ok_or_else(|| {
                                koof::error::CredentialError::MissingValue(
                                    format!(
                                        "resource reference {} is not set",
                                        #reference_path,
                                    ),
                                )
                            })?;

                    let #namespace_ident = #reference_ident
                        .namespace
                        .clone()
                        .unwrap_or_else(|| #current_namespace.clone());

                    let #api_ident: kube::Api<#target_type> =
                        kube::Api::namespaced(
                            client.clone(),
                            &#namespace_ident,
                        );

                    let #value_ident = #api_ident
                        .get(&#reference_ident.name)
                        .await?;
                });

                current_resource = target;
                current_value = quote!(#value_ident);
                current_namespace = quote!(#namespace_ident);
            }
        }
    }

    let terminal_credentials = current_resource.credentials.as_ref().ok_or_else(|| {
        GeneratorError::UnsupportedCredential(format!(
            "credential relation from {} ends at a resource \
                 without credentials",
            resource_name.0,
        ))
    })?;

    let CredentialSourceIr::SecretKeySelector { path: secret_path } = &terminal_credentials.source
    else {
        return Err(GeneratorError::UnsupportedCredential(format!(
            "credential relation from {} does not end at \
                 direct Secret credentials",
            resource_name.0,
        )));
    };

    let selector_access = field_path_access(current_value, secret_path)?;

    let secret_path_string = &secret_path.0;
    let resource_kind = &resource.kind;

    Ok(quote! {
        pub async fn resolve_credentials(
            client: &kube::Client,
            resource: &#resource_ident,
        ) -> Result<koof::http::Auth, koof::error::CredentialError> {
            let namespace =
                kube::ResourceExt::namespace(resource)
                    .ok_or_else(|| {
                        koof::error::CredentialError::MissingValue(
                            format!(
                                "{} has no namespace",
                                #resource_kind,
                            ),
                        )
                    })?;

            #(#traversal)*

            let selector = (#selector_access)
                .as_ref()
                .ok_or_else(|| {
                    koof::error::CredentialError::MissingValue(
                        format!(
                            "credential field {} is not set",
                            #secret_path_string,
                        ),
                    )
                })?;

            let value = koof::reference::resolve_secret_key(
                client,
                &#current_namespace,
                selector,
            )
            .await?;

            Ok(#auth)
        }
    })
}

pub fn generate_mod_file(ir: &ProviderIr) -> Result<GeneratedFile, GeneratorError> {
    let resource_modules = ir
        .resources
        .keys()
        .map(|name| field_ident(&name.0))
        .collect::<Result<Vec<_>, _>>()?;

    let content = quote! {
        pub mod operations;
        pub mod types;

        #(
            pub mod #resource_modules;
        )*
    };

    Ok(GeneratedFile {
        path: "src/generated/mod.rs".into(),
        content: format_tokens(content)?,
    })
}

/// `src/generated/types.rs`: typify output. Lints are relaxed for the whole file:
/// it is third-party-generated code with upstream doc strings, and typify compiles
/// `pattern` regexes with `unwrap()`.
pub fn generate_types_file(types: &GeneratedTypes) -> Result<GeneratedFile, GeneratorError> {
    let tokens = &types.tokens;
    let content = quote! {
        #![doc = " This file is generated. Do not edit manually."]
        #![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

        #tokens
    };

    Ok(GeneratedFile {
        path: "src/generated/types.rs".into(),
        content: format_tokens(content)?,
    })
}

pub fn generate_operations_file(
    ir: &ProviderIr,
    types: &GeneratedTypes,
) -> Result<GeneratedFile, GeneratorError> {
    let content = generated_file(&generate_operations(ir, types)?);

    Ok(GeneratedFile {
        path: "src/generated/operations.rs".into(),
        content: format_tokens(content)?,
    })
}
