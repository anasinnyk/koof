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
