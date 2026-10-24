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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const POLICY: RetryPolicy = RetryPolicy {
        base: Duration::from_secs(5),
        max: Duration::from_secs(60),
        attempts: None,
        waiting: Some(Duration::from_secs(15)),
        permanent: Some(Duration::from_secs(300)),
    };

    const LIMITED: RetryPolicy = RetryPolicy {
        attempts: Some(3),
        waiting: None,
        permanent: None,
        ..POLICY
    };

    fn secs(seconds: u64) -> Duration {
        Duration::from_secs(seconds)
    }

    fn api(status: u16) -> ReconcileError {
        api_retry_after(status, None)
    }

    fn api_retry_after(status: u16, retry_after: Option<u64>) -> ReconcileError {
        HttpError::Status {
            method: reqwest::Method::GET,
            path: "/zones/1".to_owned(),
            status: reqwest::StatusCode::from_u16(status).expect("valid status"),
            body: String::new(),
            retry_after: retry_after.map(secs),
        }
        .into()
    }

    fn kubernetes(code: u16) -> kube::Error {
        kube::Error::Api(kube::core::Status::failure("", "").with_code(code).boxed())
    }

    fn reference_not_ready() -> ResolveValueError {
        ResolveValueError::ReferenceNotReady {
            kind: "Zone",
            via: "zoneRef".to_owned(),
            identifier: "external_id",
        }
    }

    #[rstest]
    #[case::rate_limited(api(429), Failure::Transient(None))]
    #[case::rate_limited_with_retry_after(
        api_retry_after(429, Some(7)),
        Failure::Transient(Some(secs(7)))
    )]
    #[case::server_error(api(503), Failure::Transient(None))]
    #[case::bad_request(api(400), Failure::Permanent)]
    #[case::forbidden(api(403), Failure::Permanent)]
    #[case::conflict(api(409), Failure::Permanent)]
    #[case::kubernetes_object_missing(kubernetes(404).into(), Failure::Waiting)]
    #[case::kubernetes_unavailable(kubernetes(500).into(), Failure::Transient(None))]
    #[case::reference_not_ready(reference_not_ready().into(), Failure::Waiting)]
    #[case::referenced_secret_missing(CredentialError::Kubernetes(kubernetes(404)).into(), Failure::Waiting)]
    #[case::secret_key_missing(
        CredentialError::SecretKeyMissing { secret: "token".to_owned(), key: "api".to_owned() }.into(),
        Failure::Permanent
    )]
    #[case::reference_not_set(ResolveValueError::ReferenceNotSet { from: "zoneRef" }.into(), Failure::Permanent)]
    #[case::unsupported_operation(ReconcileError::Unsupported { kind: "Zone", lifecycle: "update" }, Failure::Permanent)]
    fn classifies_errors_by_how_to_retry(#[case] error: ReconcileError, #[case] failure: Failure) {
        assert_eq!(classify(&error), failure);
    }

    #[rstest]
    #[case::first_retry_waits_the_base_delay(Failure::Transient(None), 1, Some(secs(5)))]
    #[case::each_retry_doubles(Failure::Transient(None), 3, Some(secs(20)))]
    #[case::backoff_is_capped(Failure::Transient(None), 10, Some(secs(60)))]
    #[case::huge_attempt_counts_do_not_overflow(Failure::Transient(None), u32::MAX, Some(secs(60)))]
    #[case::retry_after_wins_over_backoff(Failure::Transient(Some(secs(42))), 3, Some(secs(42)))]
    #[case::waiting_uses_its_own_delay(Failure::Waiting, 7, Some(secs(15)))]
    #[case::permanent_uses_its_own_delay(Failure::Permanent, 1, Some(secs(300)))]
    fn delays_retries(
        #[case] failure: Failure,
        #[case] attempt: u32,
        #[case] delay: Option<Duration>,
    ) {
        assert_eq!(POLICY.delay(failure, attempt), delay);
    }

    #[rstest]
    #[case::retries_transient_failures_within_the_limit(
        Failure::Transient(None),
        2,
        Some(secs(10))
    )]
    #[case::gives_up_after_the_last_attempt(Failure::Transient(None), 3, None)]
    #[case::never_retries_permanent_failures(Failure::Permanent, 1, None)]
    #[case::never_retries_waiting_failures(Failure::Waiting, 1, None)]
    fn limited_policy_gives_up(
        #[case] failure: Failure,
        #[case] attempt: u32,
        #[case] delay: Option<Duration>,
    ) {
        assert_eq!(LIMITED.delay(failure, attempt), delay);
    }

    #[tokio::test(start_paused = true)]
    async fn with_retries_repeats_until_success() {
        let mut responses = [Err(api(503)), Err(api(429)), Ok("created")].into_iter();
        let started = tokio::time::Instant::now();

        let result = with_retries(&LIMITED, || {
            std::future::ready(responses.next().expect("response"))
        })
        .await;

        assert_eq!(result.ok(), Some("created"));
        assert_eq!(started.elapsed(), secs(5 + 10));
    }

    #[tokio::test(start_paused = true)]
    async fn with_retries_returns_the_last_error_when_giving_up() {
        let mut responses = [Err(api(503)), Err(api(502)), Err(api(500)), Ok("never")].into_iter();

        let result = with_retries(&LIMITED, || {
            std::future::ready(responses.next().expect("response"))
        })
        .await;

        assert_eq!(
            result.err().map(|error| error.to_string()),
            Some(api(500).to_string())
        );
    }

    #[tokio::test(start_paused = true)]
    async fn with_retries_stops_at_a_failure_the_policy_does_not_retry() {
        let mut responses = [Err(api(400)), Ok("never")].into_iter();

        let result = with_retries(&LIMITED, || {
            std::future::ready(responses.next().expect("response"))
        })
        .await;

        assert_eq!(
            result.err().map(|error| error.to_string()),
            Some(api(400).to_string())
        );
    }
}
