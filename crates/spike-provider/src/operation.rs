//! Prototype of the phase 4 runtime: one generic `send` instead of a generated client.

use serde::{Serialize, de::DeserializeOwned};
use std::fmt;

pub trait Operation {
    const METHOD: reqwest::Method;
    const PATH: &'static str;
    type Request: Serialize;
    type Response: DeserializeOwned;
}

/// Credentials; `Debug` never prints the secret.
pub enum Auth {
    Bearer(String),
    Header { name: &'static str, value: String },
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::Bearer(_) => f.write_str("Auth::Bearer(***)"),
            Auth::Header { name, .. } => write!(f, "Auth::Header({name}: ***)"),
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
        Self { http, base_url: base_url.into() }
    }

    /// `404` becomes `Ok(None)`; any other non-2xx is an error carrying the body.
    pub async fn send<O: Operation>(
        &self,
        auth: &Auth,
        params: &[(&'static str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<O::Response>, HttpError> {
        let path = render_path(O::PATH, params)?;
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let mut request = self.http.request(O::METHOD, url);
        request = match auth {
            Auth::Bearer(token) => request.bearer_auth(token),
            Auth::Header { name, value } => request.header(*name, value),
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

fn render_path(template: &str, params: &[(&'static str, &str)]) -> Result<String, HttpError> {
    let mut path = template.to_owned();
    for (name, value) in params {
        path = path.replace(&format!("{{{name}}}"), &urlencoding::encode(value));
    }
    if path.contains('{') {
        return Err(HttpError::MissingParameter(path));
    }
    Ok(path)
}

// ---- what koof-gen would emit per x-kube-operation: data only ----

pub struct AccountObserve;

impl Operation for AccountObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = ();
    type Response = crate::types::IamResponseSingleAccount;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_and_encodes_path() {
        let path = render_path("/zones/{zone_id}/dns_records/{id}", &[("zone_id", "a/b"), ("id", "1")]);
        assert_eq!(path.ok().as_deref(), Some("/zones/a%2Fb/dns_records/1"));
    }

    #[test]
    fn missing_parameter_is_an_error() {
        assert!(render_path("/accounts/{account_id}", &[]).is_err());
    }

    #[test]
    fn auth_debug_hides_secret() {
        let auth = Auth::Bearer("s3cr3t".into());
        assert!(!format!("{auth:?}").contains("s3cr3t"));
    }
}
