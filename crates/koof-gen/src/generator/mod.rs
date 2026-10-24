use crate::ir::{
    HttpIr, group_module,
    resolved::{Provider, Resource},
    synthesized_prefix,
};
use naming::{field_ident, kind_ident, schema_name};
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
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

type Versions<'r, 'a> = BTreeMap<&'a str, Vec<&'r Resource<'a>>>;

struct Group<'r, 'a> {
    name: &'a str,
    versions: Versions<'r, 'a>,
}

type Aliases = BTreeMap<String, String>;

/// Generates `src/generated/{mod,types,operations}.rs`, the resource modules
/// `src/generated/<group>/<version>/<resource>.rs` with a `mod.rs` per level, and
/// the end-to-end tests `e2e/generated/<resource>/chainsaw-test.yaml`.
///
/// `document` is the schema set from [`crate::spec::for_codegen`].
///
/// # Errors
///
/// Returns [`GeneratorError`] when typify rejects the schemas, a referenced schema has
/// no generated type, a name is not a valid Rust identifier, or two API groups map
/// to the same module.
pub fn generate(
    provider: &Provider<'_>,
    document: &Value,
) -> Result<Vec<GeneratedFile>, GeneratorError> {
    let prefixes = provider
        .resources
        .iter()
        .map(|resource| synthesized_prefix(&format!("{}/{}", resource.group, resource.version)))
        .collect();
    let types = types::generate_types(document, &prefixes)?;
    let groups = groups(provider)?;

    let mut files = provider
        .resources
        .iter()
        .map(|resource| resource_file(resource, &types))
        .collect::<Result<Vec<_>, _>>()?;

    let mut renamed = Aliases::new();
    for (module, group) in &groups {
        files.push(group_file(module, group)?);
        for (version, resources) in &group.versions {
            let aliases = aliases(resources, &types);
            files.push(version_file(module, version, resources, &aliases)?);
            renamed.extend(aliases.into_iter().map(|(alias, name)| (name, alias)));
        }
    }
    files.push(operations_file(provider, &types)?);
    files.push(types_file(&types, &renamed)?);
    files.push(mod_file(provider, &groups)?);
    files.extend(e2e::generate_tests(provider)?);

    Ok(files)
}

fn groups<'r, 'a>(
    provider: &'r Provider<'a>,
) -> Result<BTreeMap<String, Group<'r, 'a>>, GeneratorError> {
    let mut groups: BTreeMap<String, Group<'r, 'a>> = BTreeMap::new();
    for resource in &provider.resources {
        let module = group_module(resource.group);
        let conflict = |conflict: String| GeneratorError::GroupModule {
            group: resource.group.to_owned(),
            module: module.clone(),
            conflict,
        };
        if matches!(module.as_str(), "types" | "operations") {
            return Err(conflict("reserved".to_owned()));
        }
        let group = groups.entry(module.clone()).or_insert_with(|| Group {
            name: resource.group,
            versions: BTreeMap::new(),
        });
        if group.name != resource.group {
            return Err(conflict(format!("already used by {}", group.name)));
        }
        group
            .versions
            .entry(resource.version)
            .or_default()
            .push(resource);
    }
    Ok(groups)
}

fn aliases(resources: &[&Resource<'_>], types: &GeneratedTypes) -> Aliases {
    let mut aliases = Aliases::new();
    for resource in resources {
        for role in ["ForProvider", "AtProvider"] {
            let alias = format!("{}{role}", resource.base_kind);
            for (name, alias) in types.aliases(&schema_name(resource, role), &alias) {
                aliases.insert(alias, name);
            }
        }
    }
    aliases
}

fn resource_file(
    resource: &Resource<'_>,
    types: &GeneratedTypes,
) -> Result<GeneratedFile, GeneratorError> {
    let items = [
        crd::generate_spec(resource, types)?,
        crd::generate_variant_reference(resource)?,
        crd::generate_status(resource, types)?,
        descriptor::generate_descriptor(resource)?,
        descriptor::generate_managed_resource(resource, types)?,
    ];

    Ok(GeneratedFile {
        path: format!(
            "src/generated/{}/{}/{}.rs",
            group_module(resource.group),
            resource.version,
            resource.name.0
        )
        .into(),
        content: format_tokens(generated_file(&quote!(#(#items)*)))?,
    })
}

fn group_file(module: &str, group: &Group<'_, '_>) -> Result<GeneratedFile, GeneratorError> {
    let versions = group
        .versions
        .keys()
        .map(|version| field_ident(version))
        .collect::<Result<Vec<_>, _>>()?;
    let doc = format!(" API group `{}`.", group.name);
    let content = quote! {
        #![doc = #doc]

        #(
            pub mod #versions;
        )*
    };

    Ok(GeneratedFile {
        path: format!("src/generated/{module}/mod.rs").into(),
        content: format_tokens(content)?,
    })
}

fn version_file(
    module: &str,
    version: &str,
    resources: &[&Resource<'_>],
    aliases: &Aliases,
) -> Result<GeneratedFile, GeneratorError> {
    let modules = resources
        .iter()
        .map(|resource| field_ident(&resource.name.0))
        .collect::<Result<Vec<_>, _>>()?;
    let reexports = aliases.iter().map(|(alias, name)| {
        let alias = syn::Ident::new(alias, proc_macro2::Span::call_site());
        let name = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote!(pub use crate::generated::types::#name as #alias;)
    });
    let content = quote! {
        #(
            pub mod #modules;
        )*

        #(#reexports)*
    };

    Ok(GeneratedFile {
        path: format!("src/generated/{module}/{version}/mod.rs").into(),
        content: format_tokens(content)?,
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

fn types_file(types: &GeneratedTypes, renamed: &Aliases) -> Result<GeneratedFile, GeneratorError> {
    let tokens = &types.tokens;
    let content = quote! {
        #![doc = " This file is generated. Do not edit manually."]
        #![allow(dead_code, clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

        #tokens
    };
    let mut content = format_tokens(content)?;
    for (name, alias) in renamed {
        content = content.replace(&format!("///`{name}`\n"), &format!("///`{alias}`\n"));
    }

    Ok(GeneratedFile {
        path: "src/generated/types.rs".into(),
        content,
    })
}

fn mod_file(
    provider: &Provider<'_>,
    groups: &BTreeMap<String, Group<'_, '_>>,
) -> Result<GeneratedFile, GeneratorError> {
    let group_modules = groups
        .keys()
        .map(|module| field_ident(module))
        .collect::<Result<Vec<_>, _>>()?;

    let name = provider.name;
    let base_url = provider.base_url;
    let http = http_overrides(provider.http);
    let kinds = provider
        .resources
        .iter()
        .map(|resource| {
            let group = field_ident(&group_module(resource.group))?;
            let version = field_ident(resource.version)?;
            let module = field_ident(&resource.name.0)?;
            let kind = kind_ident(&resource.kind)?;
            Ok(quote!(#group::#version::#module::#kind))
        })
        .collect::<Result<Vec<_>, GeneratorError>>()?;
    let content = quote! {
        pub mod operations;
        pub mod types;

        #(
            pub mod #group_modules;
        )*

        #http

        pub static PROVIDER: koof::ProviderInfo =
            koof::ProviderInfo::new(#name, #base_url).with_http(http);

        /// Registers the controllers of every resource of this provider.
        #[must_use]
        pub fn controllers(app: koof::App) -> koof::App {
            app #(.controller::<#kinds>())*
        }

        #[cfg(feature = "fake")]
        #[must_use]
        pub fn fake() -> koof::fake::Fake {
            koof::fake::Fake::new() #(.resource::<#kinds>())*
        }

        /// `CustomResourceDefinition`s of every resource of this provider.
        #[must_use]
        pub fn crds() -> Vec<k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition> {
            vec![#(<#kinds as kube::CustomResourceExt>::crd()),*]
        }

        /// Documentation of every resource of this provider.
        #[must_use]
        pub fn docs() -> Vec<koof::docs::ResourceDoc> {
            vec![#(koof::docs::ResourceDoc::of::<#kinds>()),*]
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
