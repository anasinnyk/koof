use crate::ir::resolved::{Provider, Resource};
use naming::{field_ident, kind_ident};
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::path::PathBuf;
use types::GeneratedTypes;

mod crd;
mod descriptor;
mod errors;
mod naming;
mod operations;
mod types;

pub use errors::GeneratorError;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

/// Generates `src/generated/{mod,types,operations,<resource>}.rs`.
///
/// `document` is the schema set from [`crate::spec::for_codegen`].
///
/// # Errors
///
/// Returns [`GeneratorError`] when typify rejects the schemas, a referenced schema has
/// no generated type, or a name is not a valid Rust identifier.
pub fn generate(
    provider: &Provider<'_>,
    document: &Value,
) -> Result<Vec<GeneratedFile>, GeneratorError> {
    let types = types::generate_types(document)?;

    let mut files = provider
        .resources
        .iter()
        .map(|resource| resource_file(resource, &types))
        .collect::<Result<Vec<_>, _>>()?;

    files.push(operations_file(provider, &types)?);
    files.push(types_file(&types)?);
    files.push(mod_file(provider)?);

    Ok(files)
}

/// A resource module: CRD types, the static descriptor and the
/// `koof::ManagedResource` impl. All behaviour lives in the `koof` runtime.
fn resource_file(
    resource: &Resource<'_>,
    types: &GeneratedTypes,
) -> Result<GeneratedFile, GeneratorError> {
    let items = [
        crd::generate_spec(resource, types)?,
        crd::generate_status(resource, types)?,
        descriptor::generate_descriptor(resource)?,
        descriptor::generate_managed_resource(resource, types)?,
    ];

    Ok(GeneratedFile {
        path: format!("src/generated/{}.rs", resource.name.0).into(),
        content: format_tokens(generated_file(&quote!(#(#items)*)))?,
    })
}

fn operations_file(
    provider: &Provider<'_>,
    types: &GeneratedTypes,
) -> Result<GeneratedFile, GeneratorError> {
    let content = generated_file(&operations::generate_operations(provider, types)?);

    Ok(GeneratedFile {
        path: "src/generated/operations.rs".into(),
        content: format_tokens(content)?,
    })
}

/// `src/generated/types.rs`: typify output. Lints are relaxed for the whole file:
/// it is third-party-generated code with upstream doc strings, and typify compiles
/// `pattern` regexes with `unwrap()`.
fn types_file(types: &GeneratedTypes) -> Result<GeneratedFile, GeneratorError> {
    let tokens = &types.tokens;
    let content = quote! {
        #![doc = " This file is generated. Do not edit manually."]
        #![allow(dead_code, clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

        #tokens
    };

    Ok(GeneratedFile {
        path: "src/generated/types.rs".into(),
        content: format_tokens(content)?,
    })
}

fn mod_file(provider: &Provider<'_>) -> Result<GeneratedFile, GeneratorError> {
    let resource_modules = provider
        .resources
        .iter()
        .map(|resource| field_ident(&resource.name.0))
        .collect::<Result<Vec<_>, _>>()?;

    let name = provider.name;
    let base_url = provider.base_url;
    let kinds = provider
        .resources
        .iter()
        .map(|resource| {
            let module = field_ident(&resource.name.0)?;
            let kind = kind_ident(resource.kind)?;
            Ok(quote!(#module::#kind))
        })
        .collect::<Result<Vec<_>, GeneratorError>>()?;
    let content = quote! {
        pub mod operations;
        pub mod types;

        #(
            pub mod #resource_modules;
        )*

        pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
            name: #name,
            base_url: #base_url,
        };

        /// Registers the controllers of every resource of this provider.
        #[must_use]
        pub fn controllers(app: koof::App) -> koof::App {
            app #(.controller::<#kinds>())*
        }

        /// `CustomResourceDefinition`s of every resource of this provider.
        #[must_use]
        pub fn crds() -> Vec<k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition> {
            vec![#(<#kinds as kube::CustomResourceExt>::crd()),*]
        }
    };

    Ok(GeneratedFile {
        path: "src/generated/mod.rs".into(),
        content: format_tokens(content)?,
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

fn format_tokens(tokens: TokenStream) -> Result<String, GeneratorError> {
    let syntax: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax))
}
