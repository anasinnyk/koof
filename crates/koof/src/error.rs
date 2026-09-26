#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),

    #[error("{0}")]
    MissingValue(String),

    #[error("Secret {secret:?} key {key:?} is not valid UTF-8")]
    InvalidSecretUtf8 {
        secret: String,
        key: String,
        #[source]
        source: std::string::FromUtf8Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveValueError {
    #[error("failed to serialize resource: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Missing(String),

    #[error("field {path:?} is not a scalar value")]
    NonScalar { path: String },

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReconcileError {
    #[error(transparent)]
    Credentials(#[from] CredentialError),

    #[error(transparent)]
    ResolveValue(#[from] ResolveValueError),

    #[error(transparent)]
    Http(#[from] crate::http::HttpError),

    #[error("failed to serialize provider response: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}
