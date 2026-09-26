use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, kind_ident, operation_ident},
        operations::generate_operations,
        types::GeneratedTypes,
    },
    ir::{
        ApiKeyLocationIr, CredentialSourceIr, FieldPath, NamespacePolicy, OperationId, ProviderIr,
        ResourceIr, ResourceName, SecuritySchemeIr, ValueExpr,
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

/// A resource module: CRD types, the static descriptor and the
/// `koof::ManagedResource` impl. All behaviour lives in the `koof` runtime.
pub fn generate_resource(
    ir: &ProviderIr,
    types: &GeneratedTypes,
    name: &ResourceName,
    resource: &ResourceIr,
) -> Result<GeneratedFile, GeneratorError> {
    let items = [
        generate_spec(resource, types)?,
        generate_status(resource, types)?,
        generate_descriptor(ir, name, resource)?,
        generate_managed_resource(ir, resource, types)?,
    ];

    Ok(GeneratedFile {
        path: format!("src/generated/{}.rs", name.0).into(),
        content: format_tokens(generated_file(&quote!(#(#items)*)))?,
    })
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

fn generate_status(
    resource: &ResourceIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let status_ident = kind_ident(&format!("{}Status", resource.kind))?;
    let at_provider_type = types.path(&format!("{}AtProvider", resource.kind))?;

    Ok(quote! {
        pub type #status_ident = koof::status::ManagedStatus<#at_provider_type>;
    })
}

fn generate_descriptor(
    ir: &ProviderIr,
    name: &ResourceName,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let kind = &resource.kind;
    let resource_ident = kind_ident(kind)?;
    let (group, _) = resource
        .api_version
        .split_once('/')
        .ok_or_else(|| GeneratorError::InvalidApiVersion(resource.api_version.clone()))?;

    let identifiers = resource.identifiers.iter().map(|(identifier, expression)| {
        let expression = value_expr(expression);
        quote!((#identifier, #expression))
    });

    let references = resource
        .references
        .iter()
        .map(|(reference_name, reference)| {
            if !ir.resources.contains_key(&reference.target) {
                return Err(GeneratorError::InvalidIr(format!(
                    "resource {} references unknown resource {}",
                    name.0, reference.target.0
                )));
            }
            let from = &reference.from.0;
            let target_module = field_ident(&reference.target.0)?;
            let namespace = match reference.namespace {
                NamespacePolicy::SameAsResource => quote!(SameAsResource),
                NamespacePolicy::FromReference => quote!(FromReference),
                NamespacePolicy::ClusterScoped => quote!(ClusterScoped),
            };
            Ok(quote! {
                koof::resource::Reference {
                    name: #reference_name,
                    from: #from,
                    target: &crate::generated::#target_module::DESCRIPTOR,
                    namespace: koof::resource::NamespacePolicy::#namespace,
                }
            })
        })
        .collect::<Result<Vec<_>, GeneratorError>>()?;

    let credentials = credentials_tokens(ir, resource)?;

    let observe = operation_descriptor(ir, &resource.lifecycle.observe)?;
    let delete = if let Some(id) = &resource.lifecycle.delete {
        let delete = operation_descriptor(ir, id)?;
        quote!(Some(#delete))
    } else {
        quote!(None)
    };
    let create = create_descriptor(ir, name, resource)?;
    let update = update_descriptor(ir, resource)?;

    Ok(quote! {
        fn api_resource() -> kube::api::ApiResource {
            kube::api::ApiResource::erase::<#resource_ident>(&())
        }

        pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
            kind: #kind,
            api_resource,
            group: #group,
            identifiers: &[#(#identifiers),*],
            references: &[#(#references),*],
            credentials: #credentials,
            observe: #observe,
            create: #create,
            update: #update,
            delete: #delete,
        };
    })
}

fn credentials_tokens(
    ir: &ProviderIr,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let tokens = match &resource.credentials {
        None => quote!(None),
        Some(credentials) => {
            let source = match &credentials.source {
                CredentialSourceIr::SecretKeySelector { path } => {
                    let path = &path.0;
                    quote!(koof::resource::CredentialSource::Secret { path: #path })
                }
                CredentialSourceIr::Related { via } => {
                    quote!(koof::resource::CredentialSource::Related { via: &[#(#via),*] })
                }
            };
            let auth = match ir.security_schemes.get(&credentials.security_scheme) {
                Some(SecuritySchemeIr::Http { scheme, .. }) if scheme == "bearer" => {
                    quote!(koof::resource::AuthScheme::Bearer)
                }
                Some(SecuritySchemeIr::ApiKey {
                    name,
                    location: ApiKeyLocationIr::Header,
                }) => quote!(koof::resource::AuthScheme::Header(#name)),
                _ => {
                    return Err(GeneratorError::UnsupportedCredential(format!(
                        "security scheme {} is not supported",
                        credentials.security_scheme.0,
                    )));
                }
            };
            quote!(Some(koof::resource::Credentials { source: #source, auth: #auth }))
        }
    };

    Ok(tokens)
}

fn generate_managed_resource(
    ir: &ProviderIr,
    resource: &ResourceIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let resource_ident = kind_ident(&resource.kind)?;
    let at_provider_type = types.path(&format!("{}AtProvider", resource.kind))?;
    if !ir.operations.contains_key(&resource.lifecycle.observe) {
        return Err(GeneratorError::InvalidIr(format!(
            "unknown observe operation {}",
            resource.lifecycle.observe.0
        )));
    }
    let observe = operation_ident(&resource.lifecycle.observe)?;
    let operation_type = |id: Option<&OperationId>| -> Result<TokenStream, GeneratorError> {
        let Some(id) = id else {
            return Ok(quote!(koof::http::Unsupported));
        };
        let ident = operation_ident(id)?;
        Ok(quote!(crate::generated::operations::#ident))
    };
    let create = operation_type(resource.lifecycle.create.as_ref())?;
    let update = operation_type(resource.lifecycle.update.as_ref())?;
    let delete = operation_type(resource.lifecycle.delete.as_ref())?;

    Ok(quote! {
        impl koof::ManagedResource for #resource_ident {
            type AtProvider = #at_provider_type;
            type Observe = crate::generated::operations::#observe;
            type Create = #create;
            type Update = #update;
            type Delete = #delete;

            fn descriptor() -> &'static koof::ResourceDescriptor {
                &DESCRIPTOR
            }

            fn status(&self) -> Option<&koof::status::ManagedStatus<Self::AtProvider>> {
                self.status.as_ref()
            }

            fn management(&self) -> &koof::managed::ManagedResourceSpec {
                &self.spec.management
            }
        }
    })
}

/// `Some(koof::resource::CreateDescriptor { .. })` or `None`.
fn create_descriptor(
    ir: &ProviderIr,
    name: &ResourceName,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let Some(id) = &resource.lifecycle.create else {
        return Ok(quote!(None));
    };
    let operation = ir
        .operations
        .get(id)
        .ok_or_else(|| GeneratorError::InvalidIr(format!("unknown operation {}", id.0)))?;

    // The identifier backed by the external name annotation must come back in the
    // create response, otherwise the new resource cannot be tracked.
    let identifier = resource
        .identifiers
        .iter()
        .find(|(_, expression)| matches!(expression, ValueExpr::ExternalName))
        .map(|(identifier, _)| identifier)
        .ok_or_else(|| {
            GeneratorError::InvalidIr(format!(
                "resource {} can be created but has no identifier of type external_name",
                name.0
            ))
        })?;
    let external_name = operation
        .result_identifiers
        .get(identifier)
        .and_then(|path| path.strip_prefix("response.body."))
        .ok_or_else(|| {
            GeneratorError::InvalidIr(format!(
                "create operation {} must declare result.identifiers.{identifier}: \
                 response.body.<path>",
                id.0
            ))
        })?;

    let request = request_mapping(operation);
    let operation = operation_descriptor(ir, id)?;

    Ok(quote! {
        Some(koof::resource::CreateDescriptor {
            operation: #operation,
            request: #request,
            external_name: #external_name,
        })
    })
}

/// `Some(koof::resource::UpdateDescriptor { .. })` or `None`.
fn update_descriptor(
    ir: &ProviderIr,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let Some(id) = &resource.lifecycle.update else {
        return Ok(quote!(None));
    };
    let operation = ir
        .operations
        .get(id)
        .ok_or_else(|| GeneratorError::InvalidIr(format!("unknown operation {}", id.0)))?;
    let request = request_mapping(operation);
    let operation = operation_descriptor(ir, id)?;
    Ok(quote! {
        Some(koof::resource::UpdateDescriptor {
            operation: #operation,
            request: #request,
        })
    })
}

/// `koof::resource::RequestMapping { .. }`; defaults to `from: spec.forProvider`.
fn request_mapping(operation: &crate::ir::OperationIr) -> TokenStream {
    let mapping = operation.request_mapping.clone().unwrap_or_default();
    let from = &mapping.from.0;
    let set = mapping.set.iter().map(|(path, expression)| {
        let expression = value_expr(expression);
        quote!((#path, #expression))
    });
    quote! {
        koof::resource::RequestMapping {
            from: #from,
            set: &[#(#set),*],
        }
    }
}

/// `koof::resource::OperationDescriptor { .. }` for an operation.
fn operation_descriptor(ir: &ProviderIr, id: &OperationId) -> Result<TokenStream, GeneratorError> {
    let operation = ir
        .operations
        .get(id)
        .ok_or_else(|| GeneratorError::InvalidIr(format!("unknown operation {}", id.0)))?;
    let path_parameters = operation
        .path_parameters
        .iter()
        .map(|(parameter, expression)| {
            let expression = value_expr(expression);
            quote!((#parameter, #expression))
        });
    let response_path = operation.response_path.clone().unwrap_or_default();
    Ok(quote! {
        koof::resource::OperationDescriptor {
            path_parameters: &[#(#path_parameters),*],
            response_path: #response_path,
        }
    })
}

fn value_expr(expression: &ValueExpr) -> TokenStream {
    match expression {
        ValueExpr::Field { path } => {
            let path = &path.0;
            quote!(koof::resource::ValueExpr::Field(#path))
        }
        ValueExpr::RelatedIdentifier { via, name } => {
            quote!(koof::resource::ValueExpr::RelatedIdentifier { via: &[#(#via),*], name: #name })
        }
        ValueExpr::Coalesce { values } => {
            let values = values.iter().map(value_expr);
            quote!(koof::resource::ValueExpr::Coalesce(&[#(#values),*]))
        }
        ValueExpr::Literal { value } => quote!(koof::resource::ValueExpr::Literal(#value)),
        ValueExpr::ExternalName => quote!(koof::resource::ValueExpr::ExternalName),
    }
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

fn format_tokens(tokens: TokenStream) -> Result<String, GeneratorError> {
    let syntax: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax))
}

pub fn generate_mod_file(ir: &ProviderIr) -> Result<GeneratedFile, GeneratorError> {
    let resource_modules = ir
        .resources
        .keys()
        .map(|name| field_ident(&name.0))
        .collect::<Result<Vec<_>, _>>()?;

    let provider = &ir.provider;
    let base_url = &ir.base_url;
    let content = quote! {
        pub mod operations;
        pub mod types;

        #(
            pub mod #resource_modules;
        )*

        pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
            name: #provider,
            base_url: #base_url,
        };
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
