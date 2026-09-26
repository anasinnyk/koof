use std::time::Duration;

/// How a failed reconcile should be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retry {
    /// Likely to succeed soon (network, `429`, `5xx`, Kubernetes API): exponential
    /// backoff, or the server's `Retry-After` when given.
    Transient(Option<Duration>),
    /// Waiting for something else to happen (a referenced resource or Secret does not
    /// exist or is not ready yet).
    Waiting,
    /// Will not succeed without a change to the resource (invalid spec, `4xx`).
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

    #[error("failed to serialize provider response: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),

    #[error(transparent)]
    Policy(#[from] crate::policy::PolicyError),
}

impl ReconcileError {
    #[must_use]
    pub fn retry(&self) -> Retry {
        match self {
            Self::Http(error) => error.retry(),
            Self::Kubernetes(error) => kubernetes_retry(error),
            Self::ResolveValue(error) => error.retry(),
            Self::Credentials(error) => error.retry(),
            Self::Serialization(_) | Self::Policy(_) => Retry::Permanent,
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

/// A missing object (referenced resource, Secret) is something to wait for; other
/// Kubernetes API failures are transient.
fn kubernetes_retry(error: &kube::Error) -> Retry {
    match error {
        kube::Error::Api(status) if status.code == 404 => Retry::Waiting,
        _ => Retry::Transient(None),
    }
}
