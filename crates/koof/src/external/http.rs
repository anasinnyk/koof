//! Generic HTTP access to a provider API. Generated code describes each operation
//! with an [`Operation`] impl (data only); [`HttpClient::send`] does the rest.

use crate::error::Retry;
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, time::Duration};
use tracing::Instrument as _;

/// One provider API operation, as emitted by `koof-gen`.
pub trait Operation {
    /// `<resource>.<lifecycle>`, e.g. `dns_record.observe`; used in traces and metrics.
    const NAME: &'static str;
    const METHOD: reqwest::Method;
    /// Path template relative to the base URL, e.g. `/zones/{zone_id}`.
    const PATH: &'static str;
    /// Request body; `()` when the operation takes none.
    type Request: Serialize + Send + Sync;
    /// Successful response body.
    type Response: DeserializeOwned + Send + Sync;
}

/// Stand-in for a lifecycle operation the API does not have (e.g. no delete).
/// The runtime checks the descriptor before using it and never sends it.
pub struct Unsupported;

impl Operation for Unsupported {
    const NAME: &'static str = "unsupported";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "";
    type Request = ();
    type Response = ();
}

/// Credentials for one request. `Debug` never prints the secret.
#[derive(Clone)]
pub enum Auth {
    /// `Authorization: Bearer <token>`.
    Bearer(String),
    /// A custom header, e.g. an API key.
    Header { name: String, value: String },
}

impl fmt::Debug for Auth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bearer(_) => formatter.write_str("Auth::Bearer(***)"),
            Self::Header { name, .. } => write!(formatter, "Auth::Header({name}: ***)"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("unresolved path parameter in {0}")]
    MissingParameter(String),

    #[error("HTTP request failed: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("{method} {path} returned {status}: {body}")]
    Status {
        method: reqwest::Method,
        path: String,
        status: reqwest::StatusCode,
        body: String,
        /// Parsed `Retry-After` (seconds form) of a `429` / `503`.
        retry_after: Option<Duration>,
    },

    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct HttpClient {
    http: reqwest::Client,
    base_url: String,
    metrics: Option<std::sync::Arc<crate::metrics::Metrics>>,
}

impl HttpClient {
    pub fn new(http: reqwest::Client, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into(),
            metrics: None,
        }
    }

    /// Records every request in `metrics`.
    #[must_use]
    pub fn with_metrics(mut self, metrics: std::sync::Arc<crate::metrics::Metrics>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Sends `O` with the given path parameters and optional body.
    ///
    /// `404 Not Found` becomes `Ok(None)`; an empty successful body decodes as JSON
    /// `null`.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError`] when a path parameter is missing, the request fails, the
    /// status is neither successful nor `404` (the body is kept), or the response
    /// cannot be decoded.
    pub async fn send<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<O::Response>, HttpError> {
        let span = tracing::debug_span!(
            "http",
            operation = O::NAME,
            method = %O::METHOD,
            path = O::PATH,
            status = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
        );
        async {
            let started = std::time::Instant::now();
            let result = self.execute::<O>(auth, parameters, body).await;
            let span = tracing::Span::current();
            let elapsed = started.elapsed();
            span.record("duration_ms", elapsed.as_millis());
            if let Some(metrics) = &self.metrics {
                let class = match &result {
                    Ok(Some(_)) => "2xx",
                    Ok(None) => "not_found",
                    Err(HttpError::Status { status, .. }) if status.is_server_error() => "5xx",
                    Err(HttpError::Status { .. }) => "4xx",
                    Err(_) => "error",
                };
                metrics.request(O::NAME, class, elapsed);
            }
            match &result {
                Ok(Some(_)) => span.record("status", "ok"),
                Ok(None) => span.record("status", "not_found"),
                Err(HttpError::Status { status, .. }) => span.record("status", status.as_u16()),
                Err(_) => span.record("status", "error"),
            };
            if let Err(error) = &result {
                tracing::debug!(%error, "provider request failed");
            }
            result
        }
        .instrument(span)
        .await
    }

    /// Like [`send`](Self::send) but returns the response body as raw JSON instead
    /// of `O::Response`, so a body that does not match the generated type cannot
    /// lose data (used for create, whose response carries the new ID).
    ///
    /// # Errors
    ///
    /// See [`send`](Self::send).
    pub async fn send_json<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<serde_json::Value>, HttpError> {
        struct Json<O>(std::marker::PhantomData<O>);
        impl<O: Operation> Operation for Json<O> {
            const NAME: &'static str = O::NAME;
            const METHOD: reqwest::Method = O::METHOD;
            const PATH: &'static str = O::PATH;
            type Request = O::Request;
            type Response = serde_json::Value;
        }
        self.send::<Json<O>>(auth, parameters, body).await
    }

    /// Like [`send`](Self::send) but ignores the response body: `Ok(true)` on
    /// success, `Ok(false)` on `404`. For operations whose body is irrelevant
    /// (delete), so an unexpected body shape cannot fail the call.
    ///
    /// # Errors
    ///
    /// See [`send`](Self::send), except decoding.
    pub async fn send_discarding<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<bool, HttpError> {
        struct Discard<O>(std::marker::PhantomData<O>);
        impl<O: Operation> Operation for Discard<O> {
            const NAME: &'static str = O::NAME;
            const METHOD: reqwest::Method = O::METHOD;
            const PATH: &'static str = O::PATH;
            type Request = O::Request;
            type Response = serde::de::IgnoredAny;
        }
        Ok(self
            .send::<Discard<O>>(auth, parameters, body)
            .await?
            .is_some())
    }

    async fn execute<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<O::Response>, HttpError> {
        let path = render_path(O::PATH, parameters)?;
        let url = format!("{}{path}", self.base_url.trim_end_matches('/'));

        let mut request = match auth {
            Auth::Bearer(token) => self.http.request(O::METHOD, url).bearer_auth(token),
            Auth::Header { name, value } => self
                .http
                .request(O::METHOD, url)
                .header(name.as_str(), value.as_str()),
        };
        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let status = response.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.trim().parse::<u64>().ok())
            .map(Duration::from_secs);
        let bytes = response.bytes().await?;
        if !status.is_success() {
            return Err(HttpError::Status {
                method: O::METHOD,
                path,
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
                retry_after,
            });
        }
        let bytes: &[u8] = if bytes.is_empty() { b"null" } else { &bytes };
        Ok(Some(serde_json::from_slice(bytes)?))
    }
}

impl HttpError {
    /// `429` and `5xx` (and transport failures) are transient; other statuses and
    /// decoding errors are permanent.
    #[must_use]
    pub fn retry(&self) -> Retry {
        match self {
            Self::Transport(_) => Retry::Transient(None),
            Self::Status {
                status,
                retry_after,
                ..
            } if *status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error() => {
                Retry::Transient(*retry_after)
            }
            Self::Status { .. } | Self::MissingParameter(_) | Self::Decode(_) => Retry::Permanent,
        }
    }
}

fn render_path(template: &str, parameters: &[(&str, &str)]) -> Result<String, HttpError> {
    let mut path = template.to_owned();
    for (name, value) in parameters {
        path = path.replace(&format!("{{{name}}}"), &urlencoding::encode(value));
    }
    if path.contains('{') {
        return Err(HttpError::MissingParameter(path));
    }
    Ok(path)
}
