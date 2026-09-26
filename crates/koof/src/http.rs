//! Generic HTTP access to a provider API. Generated code describes each operation
//! with an [`Operation`] impl (data only); [`HttpClient::send`] does the rest.

use serde::{Serialize, de::DeserializeOwned};
use std::fmt;

/// One provider API operation, as emitted by `koof-gen`.
pub trait Operation {
    const METHOD: reqwest::Method;
    /// Path template relative to the base URL, e.g. `/zones/{zone_id}`.
    const PATH: &'static str;
    /// Request body; `()` when the operation takes none.
    type Request: Serialize;
    /// Successful response body.
    type Response: DeserializeOwned;
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
    },

    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct HttpClient {
    http: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(http: reqwest::Client, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into(),
        }
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
        let bytes = response.bytes().await?;
        if !status.is_success() {
            return Err(HttpError::Status {
                method: O::METHOD,
                path,
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }
        let bytes: &[u8] = if bytes.is_empty() { b"null" } else { &bytes };
        Ok(Some(serde_json::from_slice(bytes)?))
    }
}

/// Value at a dot path (e.g. `result`) inside a response serialized to JSON; the
/// empty path returns the whole value. `None` when a segment is missing or `null`.
///
/// # Errors
///
/// Returns [`serde_json::Error`] when `response` cannot be serialized.
pub fn extract(
    response: &impl Serialize,
    path: &str,
) -> Result<Option<serde_json::Value>, serde_json::Error> {
    let value = serde_json::to_value(response)?;
    let found = path
        .split('.')
        .filter(|segment| !segment.is_empty())
        .try_fold(&value, |current, segment| current.get(segment));
    Ok(found.filter(|value| !value.is_null()).cloned())
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
