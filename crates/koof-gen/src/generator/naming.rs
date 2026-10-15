use heck::{ToPascalCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::Ident;

use crate::{generator::errors::GeneratorError, ir::OperationId};

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
