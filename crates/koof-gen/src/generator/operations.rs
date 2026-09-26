use crate::{
    generator::{errors::GeneratorError, naming::operation_ident, types::GeneratedTypes},
    ir::{HttpMethod, ProviderIr},
};
use proc_macro2::TokenStream;
use quote::quote;

/// One `impl koof::Operation` per operation: data only, no function bodies.
///
/// # Errors
///
/// Returns [`GeneratorError`] when an operation name is not a valid identifier or its
/// request / response schema has no generated type.
pub fn generate_operations(
    ir: &ProviderIr,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let operations = ir
        .operations
        .iter()
        .map(|(id, operation)| {
            let ident = operation_ident(id)?;
            let path = &operation.path;
            let method = match operation.method {
                HttpMethod::GET => quote!(GET),
                HttpMethod::POST => quote!(POST),
                HttpMethod::PUT => quote!(PUT),
                HttpMethod::PATCH => quote!(PATCH),
                HttpMethod::DELETE => quote!(DELETE),
            };
            let body_type = |schema: Option<&crate::ir::SchemaName>| {
                schema.map_or_else(|| Ok(quote!(())), |schema| types.path(&schema.0))
            };
            let request = body_type(operation.request.as_ref())?;
            let response = body_type(operation.response.as_ref())?;

            Ok(quote! {
                pub struct #ident;

                impl koof::Operation for #ident {
                    const METHOD: reqwest::Method = reqwest::Method::#method;
                    const PATH: &'static str = #path;
                    type Request = #request;
                    type Response = #response;
                }
            })
        })
        .collect::<Result<Vec<_>, GeneratorError>>()?;

    Ok(quote!(#(#operations)*))
}
