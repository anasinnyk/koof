#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("Kubernetes API request failed")]
    Kubernetes(#[from] kube::Error),

    #[error("{0}")]
    MissingValue(String),

    #[error(transparent)]
    Resolve(#[from] ResolveValueError),

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
    #[error("failed to serialize resource")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Missing(String),

    #[error("field {path:?} is not a scalar value")]
    NonScalar { path: String },

    #[error("reference not ready: {0}")]
    ReferenceNotReady(String),

    #[error("Kubernetes API request failed")]
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

    #[error("JSON (de)serialization failed")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed")]
    Kubernetes(#[from] kube::Error),

    #[error(transparent)]
    Policy(#[from] crate::controller::policy::PolicyError),

    #[error("{0}")]
    Unsupported(String),

    #[error("{0}")]
    InvalidRequest(String),

    #[error("{0}")]
    CreateIncomplete(String),
}

#[must_use]
pub fn chain(error: &dyn std::error::Error) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        let cause_message = cause.to_string();
        if !message.contains(&cause_message) {
            message.push_str(": ");
            message.push_str(&cause_message);
        }
        source = cause.source();
    }
    message
}
