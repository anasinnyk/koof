use crate::{
    generator::{errors::GeneratorError, naming::operation_ident, types::GeneratedTypes},
    ir::{HttpMethod, SchemaName, resolved::Provider},
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
    provider: &Provider<'_>,
    types: &GeneratedTypes,
) -> Result<TokenStream, GeneratorError> {
    let operations = provider
        .operations
        .iter()
        .map(|operation| {
            let id = operation.id;
            let operation = operation.ir;
            let ident = operation_ident(id)?;
            let name = &id.0;
            let path = &operation.path;
            let method = match operation.method {
                HttpMethod::Get => quote!(GET),
                HttpMethod::Post => quote!(POST),
                HttpMethod::Put => quote!(PUT),
                HttpMethod::Patch => quote!(PATCH),
                HttpMethod::Delete => quote!(DELETE),
            };
            let body_type = |schema: Option<&SchemaName>| {
                schema.map_or_else(|| Ok(quote!(())), |schema| types.path(&schema.0))
            };
            let request = body_type(operation.request.as_ref())?;
            let response = body_type(operation.response.as_ref())?;

            Ok(quote! {
                pub struct #ident;

                impl koof::Operation for #ident {
                    const NAME: &'static str = #name;
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
