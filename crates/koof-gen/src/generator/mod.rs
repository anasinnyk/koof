use crate::ir::ProviderIr;
use serde_json::Value;

mod errors;
mod naming;
mod operations;
mod resource;
mod types;

pub use errors::GeneratorError;
pub use resource::GeneratedFile;

/// Generates `src/generated/{mod,types,operations,<resource>}.rs`.
///
/// `document` is the schema set from [`crate::spec::for_codegen`].
///
/// # Errors
///
/// Returns [`GeneratorError`] when typify rejects the schemas, a referenced schema has
/// no generated type, or a resource cannot be rendered.
pub fn generate(ir: &ProviderIr, document: &Value) -> Result<Vec<GeneratedFile>, GeneratorError> {
    let types = types::generate_types(document)?;

    let mut files = ir
        .resources
        .iter()
        .map(|(name, res)| resource::generate_resource(ir, document, &types, name, res))
        .collect::<Result<Vec<_>, _>>()?;

    files.push(resource::generate_operations_file(ir, &types)?);
    files.push(resource::generate_types_file(&types)?);
    files.push(resource::generate_mod_file(ir)?);

    Ok(files)
}
