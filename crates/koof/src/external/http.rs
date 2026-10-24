use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, time::Duration};
use tracing::Instrument as _;

pub trait Operation {
    const NAME: &'static str;
    const METHOD: reqwest::Method;
    const PATH: &'static str;
    type Request: Serialize + Send + Sync;
    type Response: DeserializeOwned + Send + Sync;
}

pub struct Unsupported;

impl Operation for Unsupported {
    const NAME: &'static str = "unsupported";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "";
    type Request = ();
    type Response = ();
}

#[derive(Clone)]
pub enum Auth {
    Bearer(String),
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

    #[error("invalid path parameter {name}: {value:?}")]
    InvalidParameter { name: String, value: String },

    #[error("HTTP request failed")]
    Transport(#[from] reqwest::Error),

    #[error("{method} {path} returned {status}: {}", summary(.body))]
    Status {
        method: reqwest::Method,
        path: String,
        status: reqwest::StatusCode,
        body: String,
        retry_after: Option<Duration>,
    },

    #[error("failed to decode response")]
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

    #[must_use]
    pub fn with_metrics(mut self, metrics: std::sync::Arc<crate::metrics::Metrics>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Sends `O` with the given path parameters and optional body.
    ///
    /// `404 Not Found` on `GET` or `DELETE` becomes `Ok(None)`; an empty successful
    /// body decodes as JSON `null`.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError`] when a path parameter is missing, the request fails, the
    /// status is neither successful nor a `GET`/`DELETE` `404` (the body is kept), or
    /// the response cannot be decoded.
    pub async fn send<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<O::Response>, HttpError> {
        self.send_raw::<O>(auth, parameters, body)
            .await?
            .map(|bytes| {
                let bytes: &[u8] = if bytes.is_empty() { b"null" } else { &bytes };
                serde_json::from_slice(bytes)
            })
            .transpose()
            .map_err(HttpError::from)
    }

    async fn send_raw<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<Vec<u8>>, HttpError> {
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
            match &result {
                Err(HttpError::Status { body, .. }) => {
                    tracing::debug!(%body, "provider request failed");
                }
                Err(error) => {
                    tracing::debug!(
                        error = crate::error::chain(error),
                        "provider request failed"
                    );
                }
                Ok(_) => {}
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

    /// Like [`send`](Self::send) but does not decode the response body: `Ok(true)` on
    /// success, `Ok(false)` on a `GET`/`DELETE` `404`. For operations whose body is
    /// irrelevant (update, delete), so a non-JSON body cannot fail the call.
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
        Ok(self.send_raw::<O>(auth, parameters, body).await?.is_some())
    }

    async fn execute<O: Operation>(
        &self,
        auth: &Auth,
        parameters: &[(&str, &str)],
        body: Option<&O::Request>,
    ) -> Result<Option<Vec<u8>>, HttpError> {
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
        if status == reqwest::StatusCode::NOT_FOUND
            && matches!(O::METHOD, reqwest::Method::GET | reqwest::Method::DELETE)
        {
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
        Ok(Some(bytes.to_vec()))
    }
}

const MAX_SUMMARY: usize = 256;

fn summary(body: &str) -> String {
    let message = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|json| provider_message(&json))
        .unwrap_or_else(|| body.to_owned());
    let message = message.split_whitespace().collect::<Vec<_>>().join(" ");
    if message.chars().count() <= MAX_SUMMARY {
        return message;
    }
    let mut short: String = message.chars().take(MAX_SUMMARY).collect();
    short.push('…');
    short
}

fn provider_message(json: &serde_json::Value) -> Option<String> {
    use serde_json::Value;
    let messages: Vec<&str> = json
        .get("errors")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|error| {
            error
                .get("message")
                .and_then(Value::as_str)
                .or_else(|| error.as_str())
        })
        .collect();
    if !messages.is_empty() {
        return Some(messages.join("; "));
    }
    ["message", "detail", "error_description", "error"]
        .iter()
        .find_map(|key| match json.get(key)? {
            Value::String(message) => Some(message.clone()),
            Value::Object(error) => error
                .get("message")
                .and_then(Value::as_str)
                .map(str::to_owned),
            _ => None,
        })
}

fn render_path(template: &str, parameters: &[(&str, &str)]) -> Result<String, HttpError> {
    let mut path = template.to_owned();
    for (name, value) in parameters {
        if matches!(*value, "." | "..") {
            return Err(HttpError::InvalidParameter {
                name: (*name).to_owned(),
                value: (*value).to_owned(),
            });
        }
        path = path.replace(&format!("{{{name}}}"), &urlencoding::encode(value));
    }
    if path.contains('{') {
        return Err(HttpError::MissingParameter(path));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::fills_parameters(
        "/zones/{zone_id}/dns_records/{record_id}",
        &[("zone_id", "z1"), ("record_id", "r1")],
        "/zones/z1/dns_records/r1"
    )]
    #[case::encodes_reserved_characters("/projects/{name}", &[("name", "a/b c?")], "/projects/a%2Fb%20c%3F")]
    #[case::allows_dots_inside_a_value("/zones/{zone}", &[("zone", "example.com")], "/zones/example.com")]
    #[case::ignores_unused_parameters("/accounts", &[("account_id", "a1")], "/accounts")]
    fn renders_request_paths(
        #[case] template: &str,
        #[case] parameters: &[(&str, &str)],
        #[case] path: &str,
    ) {
        assert_eq!(
            render_path(template, parameters).ok(),
            Some(path.to_owned())
        );
    }

    #[rstest]
    #[case::current_directory(".")]
    #[case::parent_directory("..")]
    fn rejects_values_that_would_change_the_path(#[case] value: &str) {
        assert!(matches!(
            render_path("/zones/{zone_id}/dns_records", &[("zone_id", value)]),
            Err(HttpError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn rejects_a_path_with_a_missing_parameter() {
        assert!(matches!(
            render_path("/zones/{zone_id}", &[]),
            Err(HttpError::MissingParameter(path)) if path == "/zones/{zone_id}"
        ));
    }

    #[rstest]
    #[case::cloudflare_errors(
        r#"{"success":false,"errors":[{"code":1003,"message":"Invalid zone"}]}"#,
        "Invalid zone"
    )]
    #[case::several_errors(
        r#"{"errors":[{"message":"name is required"},{"message":"ttl is invalid"}]}"#,
        "name is required; ttl is invalid"
    )]
    #[case::errors_as_strings(r#"{"errors":["name is required"]}"#, "name is required")]
    #[case::top_level_message(r#"{"message":"Not Found","documentation_url":"…"}"#, "Not Found")]
    #[case::sentry_detail(
        r#"{"detail":"You do not have permission"}"#,
        "You do not have permission"
    )]
    #[case::oauth_error(
        r#"{"error":"invalid_token","error_description":"Token expired"}"#,
        "Token expired"
    )]
    #[case::nested_error_object(r#"{"error":{"code":403,"message":"Forbidden"}}"#, "Forbidden")]
    #[case::json_without_a_known_field(r#"{"status":"failed"}"#, r#"{"status":"failed"}"#)]
    #[case::plain_text("Bad Gateway", "Bad Gateway")]
    #[case::html_whitespace_collapsed(
        "<html>\n  <body>Bad   Gateway</body>\n</html>",
        "<html> <body>Bad Gateway</body> </html>"
    )]
    fn summarizes_the_provider_error(#[case] body: &str, #[case] summary_text: &str) {
        assert_eq!(summary(body), summary_text);
    }

    #[test]
    fn summary_of_a_huge_body_is_cut() {
        assert_eq!(
            summary(&"x".repeat(10_000)),
            format!("{}…", "x".repeat(256))
        );
    }

    #[test]
    fn status_error_shows_the_summary_not_the_whole_body() {
        let error = HttpError::Status {
            method: reqwest::Method::POST,
            path: "/zones".to_owned(),
            status: reqwest::StatusCode::BAD_REQUEST,
            body: r#"{"errors":[{"message":"Invalid zone"}],"result":null}"#.to_owned(),
            retry_after: None,
        };
        assert_eq!(
            error.to_string(),
            "POST /zones returned 400 Bad Request: Invalid zone"
        );
    }
}
