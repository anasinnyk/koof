use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, kind_ident},
        types::GeneratedTypes,
    },
    ir::{ValueExpr, resolved::Resource},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_spec(
    resource: &Resource<'_>,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let group = resource.group;
    let version = resource.version;
    let kind = resource.kind;
    let status_name = format!("{kind}Status");
    let spec_ident = kind_ident(&format!("{kind}Spec"))?;
    let for_provider_ident = kind_ident(&format!("{kind}ForProviderSpec"))?;
    let fields_type = types.path(&format!("{kind}ForProvider"))?;

    let validations = validation_tokens(resource);
    let print_columns = print_columns(resource);
    let namespaced = resource.namespaced.then(|| quote!(namespaced,));
    let mut fields = reference_fields(resource)?;
    if let Some(credentials) = credential_field(resource)? {
        fields.push(credentials);
    }

    Ok(quote! {
        /// CRD schema of `spec.forProvider`: unions flattened for kube, then the
        /// overlay's CEL validations.
        fn for_provider_schema(schema: &mut schemars::Schema) {
            koof::schema::flatten_unions(schema);
            koof::schema::add_validations(schema, &[#(#validations),*]);
        }

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
        #[schemars(transform = for_provider_schema)]
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
            #namespaced
            status = #status_name,
            #(#print_columns),*
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

fn print_columns(resource: &Resource<'_>) -> Vec<TokenStream> {
    let column = |name: &str, type_: &str, json_path: &str| {
        let column =
            serde_json::json!({ "name": name, "type": type_, "jsonPath": json_path }).to_string();
        quote!(printcolumn = #column)
    };
    let mut columns = vec![
        column(
            "Ready",
            "string",
            r#".status.conditions[?(@.type=="Ready")].status"#,
        ),
        column(
            "Synced",
            "string",
            r#".status.conditions[?(@.type=="Synced")].status"#,
        ),
    ];
    if resource
        .identifiers
        .values()
        .any(|expression| matches!(expression, ValueExpr::ExternalName))
    {
        let group = resource.group.replace('.', "\\.");
        columns.push(column(
            "External-Name",
            "string",
            &format!(".metadata.annotations.{group}/external-name"),
        ));
    }
    columns.push(column("Age", "date", ".metadata.creationTimestamp"));
    columns
}

fn validation_tokens(resource: &Resource<'_>) -> Vec<TokenStream> {
    let validation = |rule: &str, message: &str, field: &str| {
        quote! {
            koof::schema::Validation { rule: #rule, message: #message, field: #field }
        }
    };
    let mut tokens: Vec<TokenStream> = resource
        .validations
        .iter()
        .map(|rule| validation(&rule.rule, &rule.message, &rule.field))
        .collect();
    for field in resource.immutable {
        let (parent, leaf) = field.rsplit_once('.').unwrap_or(("", field));
        tokens.push(validation(
            "self == oldSelf",
            &format!("{field} is immutable"),
            field,
        ));
        tokens.push(validation(
            &format!("!has(oldSelf.{leaf}) || has(self.{leaf})"),
            &format!("{field} cannot be removed once set"),
            parent,
        ));
    }
    tokens
}

pub fn generate_status(
    resource: &Resource<'_>,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let status_ident = kind_ident(&format!("{}Status", resource.kind))?;
    let at_provider_type = types.path(&format!("{}AtProvider", resource.kind))?;

    Ok(quote! {
        pub type #status_ident = koof::status::ManagedStatus<#at_provider_type>;
    })
}

fn reference_fields(resource: &Resource<'_>) -> Result<Vec<TokenStream>, GeneratorError> {
    resource
        .references
        .iter()
        .map(|reference| {
            let json_name = reference.field;
            let ident = field_ident(json_name)?;

            Ok(quote! {
                #[serde(rename = #json_name, default, skip_serializing_if = "Option::is_none")]
                pub #ident:
                    Option<koof::reference::ResourceReference>,
            })
        })
        .collect()
}

fn credential_field(resource: &Resource<'_>) -> Result<Option<TokenStream>, GeneratorError> {
    let Some(json_name) = resource
        .credentials
        .as_ref()
        .and_then(|credentials| credentials.secret_field)
    else {
        return Ok(None);
    };
    let ident = field_ident(json_name)?;

    Ok(Some(quote! {
        #[serde(rename = #json_name, default, skip_serializing_if = "Option::is_none")]
        pub #ident:
            Option<koof::reference::SecretReference>,
    }))
}
