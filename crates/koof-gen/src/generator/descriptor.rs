use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, kind_ident, operation_ident},
        types::GeneratedTypes,
    },
    ir::{
        CredentialSourceIr, NamespacePolicy, ValueExpr,
        resolved::{Auth, Create, Credentials, Operation, Resource},
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_descriptor(resource: &Resource<'_>) -> Result<TokenStream, GeneratorError> {
    let kind = resource.kind;
    let resource_ident = kind_ident(kind)?;
    let group = resource.group;

    let identifiers = resource.identifiers.iter().map(|(identifier, expression)| {
        let expression = value_expr(expression);
        quote!((#identifier, #expression))
    });

    let references = resource
        .references
        .iter()
        .map(|reference| {
            let reference_name = reference.name;
            let from = pointer(reference.from);
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

    let credentials = resource
        .credentials
        .as_ref()
        .map_or_else(|| quote!(None), credentials_tokens);

    let observe = observe_descriptor(resource.observe);
    let delete = optional(resource.delete.map(operation_descriptor));
    let create = optional(resource.create.map(create_descriptor));
    let namespaced = resource.namespaced;
    let late_initialize = resource.late_initialize;
    let update = optional(resource.update.map(update_descriptor));

    Ok(quote! {
        fn api_resource() -> kube::api::ApiResource {
            kube::api::ApiResource::erase::<#resource_ident>(&())
        }

        pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
            kind: #kind,
            api_resource,
            group: #group,
            namespaced: #namespaced,
            identifiers: &[#(#identifiers),*],
            late_initialize: &[#(#late_initialize),*],
            references: &[#(#references),*],
            credentials: #credentials,
            observe: #observe,
            create: #create,
            update: #update,
            delete: #delete,
        };
    })
}

fn optional(tokens: Option<TokenStream>) -> TokenStream {
    tokens.map_or_else(|| quote!(None), |tokens| quote!(Some(#tokens)))
}

fn credentials_tokens(credentials: &Credentials<'_>) -> TokenStream {
    let source = match credentials.source {
        CredentialSourceIr::SecretKeySelector { path } => {
            let path = pointer(&path.0);
            quote!(koof::resource::CredentialSource::Secret { path: #path })
        }
        CredentialSourceIr::Related { via } => {
            quote!(koof::resource::CredentialSource::Related { via: &[#(#via),*] })
        }
    };
    let auth = match credentials.auth {
        Auth::Bearer => quote!(koof::resource::AuthScheme::Bearer),
        Auth::Header(name) => quote!(koof::resource::AuthScheme::Header(#name)),
    };
    quote!(Some(koof::resource::Credentials { source: #source, auth: #auth }))
}

pub fn generate_managed_resource(
    resource: &Resource<'_>,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let resource_ident = kind_ident(resource.kind)?;
    let at_provider_type = types.path(&format!("{}AtProvider", resource.kind))?;
    let observe = operation_ident(resource.observe.id)?;
    let operation_type = |operation: Option<Operation<'_>>| -> Result<TokenStream, GeneratorError> {
        let Some(operation) = operation else {
            return Ok(quote!(koof::http::Unsupported));
        };
        let ident = operation_ident(operation.id)?;
        Ok(quote!(crate::generated::operations::#ident))
    };
    let create = operation_type(resource.create.map(|create| create.operation))?;
    let update = operation_type(resource.update)?;
    let delete = operation_type(resource.delete)?;

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

fn create_descriptor(create: Create<'_>) -> TokenStream {
    let request = request_mapping(create.operation);
    let operation = operation_descriptor(create.operation);
    let external_name = pointer(create.external_name);
    quote! {
        koof::resource::CreateDescriptor {
            operation: #operation,
            request: #request,
            external_name: #external_name,
        }
    }
}

fn update_descriptor(operation: Operation<'_>) -> TokenStream {
    let request = request_mapping(operation);
    let operation = operation_descriptor(operation);
    quote! {
        koof::resource::UpdateDescriptor {
            operation: #operation,
            request: #request,
        }
    }
}

fn request_mapping(operation: Operation<'_>) -> TokenStream {
    let mapping = operation.ir.request_mapping.clone().unwrap_or_default();
    let from = pointer(&mapping.from.0);
    let set = mapping.set.iter().map(|(path, expression)| {
        let path = pointer(path);
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

fn observe_descriptor(operation: Operation<'_>) -> TokenStream {
    let descriptor = operation_descriptor(operation);
    let deleted_when = optional(
        operation
            .ir
            .deleted_when
            .as_deref()
            .map(|path| {
                let path = pointer(path);
                quote!(#path)
            }),
    );
    quote! {
        koof::resource::ObserveDescriptor {
            operation: #descriptor,
            deleted_when: #deleted_when,
        }
    }
}

fn operation_descriptor(operation: Operation<'_>) -> TokenStream {
    let path_parameters = operation
        .ir
        .path_parameters
        .iter()
        .map(|(parameter, expression)| {
            let expression = value_expr(expression);
            quote!((#parameter, #expression))
        });
    let response_path = pointer(operation.ir.response_path.as_deref().unwrap_or_default());
    quote! {
        koof::resource::OperationDescriptor {
            path_parameters: &[#(#path_parameters),*],
            response_path: #response_path,
        }
    }
}

fn value_expr(expression: &ValueExpr) -> TokenStream {
    match expression {
        ValueExpr::Field { path } => {
            let path = pointer(&path.0);
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

fn pointer(path: &str) -> String {
    jsonptr::PointerBuf::from_tokens(path.split('.').filter(|segment| !segment.is_empty()))
        .to_string()
}
