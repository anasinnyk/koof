use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("typify failed: {0}")]
    Typify(String),

    #[error("no generated type for schema {0}")]
    UnknownType(String),

    #[error("e2e dependencies form a cycle: {0}; remove `test` or break the cycle")]
    ReferenceCycle(String),

    #[error("invalid Rust identifier: {0}")]
    InvalidIdentifier(String),

    #[error("API group {group} maps to module {module}, which is {conflict}")]
    GroupModule {
        group: String,
        module: String,
        conflict: String,
    },

    #[error(transparent)]
    Syntax(#[from] syn::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}
