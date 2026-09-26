use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("typify failed: {0}")]
    Typify(String),

    #[error("no generated type for schema {0}")]
    UnknownType(String),

    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),

    #[error("invalid Rust identifier: {0}")]
    InvalidIdentifier(String),

    #[error(transparent)]
    Syntax(#[from] syn::Error),

    #[error("invalid Kubernetes apiVersion: {0}")]
    InvalidApiVersion(String),

    #[error("unsupported credential: {0}")]
    UnsupportedCredential(String),

    #[error("invalid IR: {0}")]
    InvalidIr(String),
}
