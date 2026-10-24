use crate::{
    error::{CredentialError, ReconcileError, ResolveValueError},
    http::HttpError,
};
use std::{future::Future, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure {
    Transient(Option<Duration>),
    Waiting,
    Permanent,
}

#[must_use]
pub fn classify(error: &ReconcileError) -> Failure {
    match error {
        ReconcileError::Http(error) => http(error),
        ReconcileError::Kubernetes(error) => kubernetes(error),
        ReconcileError::ResolveValue(error) => resolve_value(error),
        ReconcileError::Credentials(error) => credentials(error),
        ReconcileError::Serialization(_)
        | ReconcileError::Policy(_)
        | ReconcileError::Unsupported { .. }
        | ReconcileError::RequestPath { .. }
        | ReconcileError::RequestAssign { .. }
        | ReconcileError::InvalidRequest { .. }
        | ReconcileError::CreatePending { .. }
        | ReconcileError::CreateNoIdentifier { .. }
        | ReconcileError::CreateNotRecorded { .. } => Failure::Permanent,
    }
}

fn http(error: &HttpError) -> Failure {
    match error {
        HttpError::Transport(_) => Failure::Transient(None),
        HttpError::Status {
            status,
            retry_after,
            ..
        } if *status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error() => {
            Failure::Transient(*retry_after)
        }
        HttpError::Status { .. }
        | HttpError::MissingParameter(_)
        | HttpError::InvalidParameter { .. }
        | HttpError::Decode(_) => Failure::Permanent,
    }
}

fn resolve_value(error: &ResolveValueError) -> Failure {
    match error {
        ResolveValueError::ReferenceNotReady { .. } => Failure::Waiting,
        ResolveValueError::Kubernetes(error) => kubernetes(error),
        ResolveValueError::Serialization(_)
        | ResolveValueError::RequestFieldNotSet { .. }
        | ResolveValueError::PathParameterNotSet { .. }
        | ResolveValueError::UnknownIdentifier { .. }
        | ResolveValueError::UnknownReference { .. }
        | ResolveValueError::ReferenceNotSet { .. }
        | ResolveValueError::ReferenceKind { .. }
        | ResolveValueError::AmbiguousReference { .. }
        | ResolveValueError::ClusterScopedReference { .. }
        | ResolveValueError::NonScalar { .. } => Failure::Permanent,
    }
}

fn credentials(error: &CredentialError) -> Failure {
    match error {
        CredentialError::Kubernetes(error) => kubernetes(error),
        CredentialError::Resolve(error) => resolve_value(error),
        CredentialError::NoCredentials { .. }
        | CredentialError::NotSecret { .. }
        | CredentialError::FieldNotSet { .. }
        | CredentialError::SecretKeyMissing { .. }
        | CredentialError::InvalidSecretUtf8 { .. } => Failure::Permanent,
    }
}

fn kubernetes(error: &kube::Error) -> Failure {
    match error {
        kube::Error::Api(status) if status.code == 404 => Failure::Waiting,
        _ => Failure::Transient(None),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub base: Duration,
    pub max: Duration,
    pub attempts: Option<u32>,
    pub waiting: Option<Duration>,
    pub permanent: Option<Duration>,
}

impl RetryPolicy {
    #[must_use]
    pub fn delay(&self, failure: Failure, attempt: u32) -> Option<Duration> {
        if self.attempts.is_some_and(|attempts| attempt >= attempts) {
            return None;
        }
        match failure {
            Failure::Transient(Some(retry_after)) => Some(retry_after),
            Failure::Transient(None) => Some(self.backoff(attempt)),
            Failure::Waiting => self.waiting,
            Failure::Permanent => self.permanent,
        }
    }

    fn backoff(&self, attempt: u32) -> Duration {
        let exponent = attempt.saturating_sub(1).min(16);
        self.base.saturating_mul(1 << exponent).min(self.max)
    }
}

/// # Errors
///
/// Returns the error of the last attempt.
pub async fn with_retries<T, F, Fut>(
    policy: &RetryPolicy,
    mut operation: F,
) -> Result<T, ReconcileError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, ReconcileError>>,
{
    let mut attempt = 0;
    loop {
        let error = match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => error,
        };
        attempt += 1;
        match policy.delay(classify(&error), attempt) {
            Some(delay) => tokio::time::sleep(delay).await,
            None => return Err(error),
        }
    }
}
