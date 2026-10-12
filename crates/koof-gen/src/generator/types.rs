use crate::generator::{errors::GeneratorError, naming::typify_type_name};
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::collections::BTreeSet;

pub struct GeneratedTypes {
    pub tokens: TokenStream,
    names: BTreeSet<String>,
}

impl GeneratedTypes {
    /// `crate::generated::types::<Type>` for a `components.schemas` name.
    ///
    /// # Errors
    ///
    /// Returns [`GeneratorError::UnknownType`] when typify produced no such type.
    pub fn path(&self, schema_name: &str) -> Result<TokenStream, GeneratorError> {
        let name = typify_type_name(schema_name);
        if !self.names.contains(&name) {
            return Err(GeneratorError::UnknownType(schema_name.to_owned()));
        }
        let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
        Ok(quote!(crate::generated::types::#ident))
    }
}

/// Runs typify over `components.schemas` of a document prepared by
/// [`crate::spec::for_codegen`].
///
/// # Errors
///
/// Returns [`GeneratorError`] when a schema cannot be read as JSON Schema or typify
/// rejects the schema set.
pub fn generate_types(document: &Value) -> Result<GeneratedTypes, GeneratorError> {
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| GeneratorError::Typify("missing components.schemas".into()))?;

    let definitions = schemas
        .iter()
        .map(|(name, schema)| {
            serde_json::from_value::<schemars08::schema::Schema>(schema.clone())
                .map(|schema| (name.clone(), schema))
                .map_err(|error| GeneratorError::Typify(format!("schema {name}: {error}")))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut settings = typify::TypeSpaceSettings::default();
    settings
        .with_struct_builder(false)
        .with_derive("schemars::JsonSchema".to_owned())
        .with_derive("PartialEq".to_owned());
    let mut space = typify::TypeSpace::new(&settings);
    if let Err(error) = space.add_ref_types(definitions.clone()) {
        return Err(GeneratorError::Typify(explain(
            &settings,
            &definitions,
            &error,
        )));
    }

    let names = space.iter_types().map(|ty| ty.name()).collect();
    Ok(GeneratedTypes {
        tokens: space.to_stream(),
        names,
    })
}

fn explain(
    settings: &typify::TypeSpaceSettings,
    definitions: &[(String, schemars08::schema::Schema)],
    error: &typify::Error,
) -> String {
    let without_defaults: Vec<(String, schemars08::schema::Schema)> = definitions
        .iter()
        .map(|(name, schema)| (name.clone(), strip_defaults(schema)))
        .collect();
    if typify::TypeSpace::new(settings)
        .add_ref_types(without_defaults.clone())
        .is_err()
    {
        return error.to_string();
    }
    let rejects = |keep: &[usize]| {
        let mut attempt = without_defaults.clone();
        for &index in keep {
            attempt[index] = definitions[index].clone();
        }
        typify::TypeSpace::new(settings)
            .add_ref_types(attempt)
            .is_err()
    };
    let candidates: Vec<usize> = definitions
        .iter()
        .enumerate()
        .filter(|(index, (_, schema))| *schema != without_defaults[*index].1)
        .map(|(index, _)| index)
        .collect();
    let mut culprits = Vec::new();
    let mut queue = vec![candidates];
    while let Some(group) = queue.pop() {
        if group.is_empty() || !rejects(&group) {
            continue;
        }
        if let [single] = group.as_slice() {
            culprits.push(definitions[*single].0.as_str());
            continue;
        }
        let (left, right) = group.split_at(group.len() / 2);
        queue.push(left.to_vec());
        queue.push(right.to_vec());
    }
    culprits.sort_unstable();
    format!(
        "{error}; rejected `default` values in: {} (remove them with an overlay)",
        culprits.join(", ")
    )
}

fn strip_defaults(schema: &schemars08::schema::Schema) -> schemars08::schema::Schema {
    fn walk(value: &mut Value) {
        match value {
            Value::Object(map) => {
                map.remove("default");
                map.values_mut().for_each(walk);
            }
            Value::Array(items) => items.iter_mut().for_each(walk),
            _ => {}
        }
    }
    let mut value = serde_json::to_value(schema).unwrap_or(Value::Null);
    walk(&mut value);
    serde_json::from_value(value).unwrap_or_else(|_| schema.clone())
}
