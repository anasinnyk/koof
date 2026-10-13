use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("typify failed: {0}")]
    Typify(String),

    #[error("no generated type for schema {0}")]
    UnknownType(String),

    #[error("invalid Rust identifier: {0}")]
    InvalidIdentifier(String),

    #[error(transparent)]
    Syntax(#[from] syn::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}
