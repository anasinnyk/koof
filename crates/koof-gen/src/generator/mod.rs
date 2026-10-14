use crate::ir::{
    HttpIr,
    resolved::{Provider, Resource},
};
use naming::{field_ident, kind_ident};
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::path::PathBuf;
use types::GeneratedTypes;

mod crd;
mod descriptor;
mod e2e;
mod errors;
mod naming;
mod operations;
mod types;

pub use errors::GeneratorError;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

/// Generates `src/generated/{mod,types,operations,<resource>}.rs` and the
/// end-to-end tests `e2e/generated/<resource>/chainsaw-test.yaml`.
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
    files.extend(e2e::generate_tests(provider)?);

    Ok(files)
}

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
    let http = http_overrides(provider.http);
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

        #http

        pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
            name: #name,
            base_url: #base_url,
            http,
        };

        /// Registers the controllers of every resource of this provider.
        #[must_use]
        pub fn controllers(app: koof::App) -> koof::App {
            app #(.controller::<#kinds>())*
        }

        #[must_use]
        pub fn fake() -> koof::fake::Fake {
            koof::fake::Fake::new() #(.resource::<#kinds>())*
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

fn http_overrides(http: &HttpIr) -> TokenStream {
    let duration = |name: &str, value: Option<std::time::Duration>| {
        let value = value?;
        let field = quote::format_ident!("{name}");
        let millis = u64::try_from(value.as_millis()).unwrap_or(u64::MAX);
        let duration = match millis {
            0 => quote!(::std::time::Duration::ZERO),
            _ if millis % 3_600_000 == 0 => {
                let hours = millis / 3_600_000;
                quote!(::std::time::Duration::from_hours(#hours))
            }
            _ if millis % 60_000 == 0 => {
                let minutes = millis / 60_000;
                quote!(::std::time::Duration::from_mins(#minutes))
            }
            _ if millis % 1_000 == 0 => {
                let seconds = millis / 1_000;
                quote!(::std::time::Duration::from_secs(#seconds))
            }
            _ => quote!(::std::time::Duration::from_millis(#millis)),
        };
        Some(quote!(http.#field = #duration;))
    };
    let fields: Vec<TokenStream> = [
        duration("timeout", http.timeout),
        duration("connect_timeout", http.connect_timeout),
        duration("pool_idle_timeout", http.pool_idle_timeout),
        http.pool_max_idle_per_host
            .map(|value| quote!(http.pool_max_idle_per_host = #value;)),
        http.user_agent
            .as_deref()
            .map(|value| quote!(#value.clone_into(&mut http.user_agent);)),
    ]
    .into_iter()
    .flatten()
    .collect();
    let binding = if fields.is_empty() {
        quote!(http)
    } else {
        quote!(mut http)
    };
    quote! {
        fn http(#binding: koof::config::HttpConfig) -> koof::config::HttpConfig {
            #(#fields)*
            http
        }
    }
}

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
