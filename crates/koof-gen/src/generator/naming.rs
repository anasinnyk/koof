use heck::{ToPascalCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    generator::errors::GeneratorError,
    ir::{OperationId, ResourceName, group_module, resolved::Resource, synthesized_schema},
};

/// `crate::generated::<group>::<version>::<resource>`.
pub fn module_path(
    group: &str,
    version: &str,
    name: &ResourceName,
) -> Result<TokenStream, GeneratorError> {
    let group = field_ident(&group_module(group))?;
    let version = field_ident(version)?;
    let module = field_ident(&name.0)?;
    Ok(quote!(crate::generated::#group::#version::#module))
}

/// `components.schemas` key of the synthesized `role` schema of `resource`.
pub fn schema_name(resource: &Resource<'_>, role: &str) -> String {
    synthesized_schema(
        &format!("{}/{}", resource.group, resource.version),
        resource.base_kind,
        role,
    )
}

pub fn kind_ident(value: &str) -> Result<Ident, GeneratorError> {
    parse_ident(value)
}

pub fn field_ident(value: &str) -> Result<Ident, GeneratorError> {
    let name = value.to_snake_case();

    syn::parse_str::<Ident>(&name)
        .or_else(|_| syn::parse_str::<Ident>(&format!("r#{name}")))
        .map_err(|_| GeneratorError::InvalidIdentifier(value.into()))
}

fn parse_ident(value: &str) -> Result<Ident, GeneratorError> {
    let value = if value
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        format!("Schema{value}")
    } else {
        value.to_owned()
    };

    syn::parse_str(&value).map_err(|_| GeneratorError::InvalidIdentifier(value))
}

pub fn typify_type_name(schema_name: &str) -> String {
    let cleaned = schema_name.replace('\'', "").replace(
        |character: char| !(character.is_alphanumeric() || character == '_'),
        "-",
    );
    let name = cleaned.to_pascal_case();
    match name.chars().next() {
        Some(first) if first.is_alphabetic() || first == '_' => name,
        _ => format!("X{name}"),
    }
}

pub fn operation_ident(id: &OperationId) -> Result<Ident, GeneratorError> {
    parse_ident(&id.0.to_upper_camel_case())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::snake_case_of_a_json_name("apiTokenSecretRef", "api_token_secret_ref")]
    #[case::keywords_become_raw_identifiers("type", "r#type")]
    #[case::kebab_case("dns-record", "dns_record")]
    fn fields_become_rust_identifiers(#[case] name: &str, #[case] ident: &str) {
        assert_eq!(
            field_ident(name).map(|ident| ident.to_string()).ok(),
            Some(ident.to_owned())
        );
    }

    #[rstest]
    #[case::keeps_a_valid_kind("DNSRecord", "DNSRecord")]
    #[case::prefixes_a_leading_digit("3DSecure", "Schema3DSecure")]
    fn kinds_become_rust_type_names(#[case] kind: &str, #[case] ident: &str) {
        assert_eq!(
            kind_ident(kind).map(|ident| ident.to_string()).ok(),
            Some(ident.to_owned())
        );
    }

    #[test]
    fn rejects_a_name_that_cannot_be_an_identifier() {
        assert!(matches!(
            kind_ident("Not A Kind"),
            Err(GeneratorError::InvalidIdentifier(_))
        ));
    }

    #[rstest]
    #[case::pascal_case_of_a_schema_name("dns-records_dns-record-post", "DnsRecordsDnsRecordPost")]
    #[case::drops_quotes("zone's_settings", "ZonesSettings")]
    #[case::prefixes_a_leading_digit("2fa_settings", "X2faSettings")]
    fn schema_names_match_typify(#[case] schema: &str, #[case] type_name: &str) {
        assert_eq!(typify_type_name(schema), type_name);
    }
}
