use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retry {
    Transient(Option<Duration>),
    Waiting,
    Permanent,
}

#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("Kubernetes API request failed: {0}")]
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
    #[error("failed to serialize resource: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Missing(String),

    #[error("field {path:?} is not a scalar value")]
    NonScalar { path: String },

    #[error("reference not ready: {0}")]
    ReferenceNotReady(String),

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

    #[error("JSON (de)serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed: {0}")]
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

impl ReconcileError {
    #[must_use]
    pub fn retry(&self) -> Retry {
        match self {
            Self::Http(error) => error.retry(),
            Self::Kubernetes(error) => kubernetes_retry(error),
            Self::ResolveValue(error) => error.retry(),
            Self::Credentials(error) => error.retry(),
            Self::Serialization(_)
            | Self::Policy(_)
            | Self::Unsupported(_)
            | Self::InvalidRequest(_)
            | Self::CreateIncomplete(_) => Retry::Permanent,
        }
    }
}

impl ResolveValueError {
    #[must_use]
    pub fn retry(&self) -> Retry {
        match self {
            Self::ReferenceNotReady(_) => Retry::Waiting,
            Self::Kubernetes(error) => kubernetes_retry(error),
            Self::Serialization(_) | Self::Missing(_) | Self::NonScalar { .. } => Retry::Permanent,
        }
    }
}

impl CredentialError {
    #[must_use]
    pub fn retry(&self) -> Retry {
        match self {
            Self::Kubernetes(error) => kubernetes_retry(error),
            Self::Resolve(error) => error.retry(),
            Self::MissingValue(_) | Self::InvalidSecretUtf8 { .. } => Retry::Permanent,
        }
    }
}

fn kubernetes_retry(error: &kube::Error) -> Retry {
    match error {
        kube::Error::Api(status) if status.code == 404 => Retry::Waiting,
        _ => Retry::Transient(None),
    }
}
