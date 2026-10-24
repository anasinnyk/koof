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

    mod requests {
        use super::*;
        use crate::testing::{RecordCreate, RecordDelete, RecordObserve, RecordUpdate};
        use pretty_assertions::assert_eq;
        use serde_json::{Value, json};
        use wiremock::{
            Mock, MockServer, ResponseTemplate,
            matchers::{body_json, header, method, path},
        };

        const RECORD: &[(&str, &str)] = &[("zone_id", "zone-1"), ("record_id", "record-1")];
        const ZONE: &[(&str, &str)] = &[("zone_id", "zone-1")];

        fn token() -> Auth {
            Auth::Bearer("t0ken".to_owned())
        }

        async fn provider(
            verb: &str,
            url: &str,
            response: ResponseTemplate,
        ) -> (MockServer, HttpClient) {
            let server = MockServer::start().await;
            Mock::given(method(verb))
                .and(path(url))
                .respond_with(response)
                .expect(1)
                .mount(&server)
                .await;
            let client = HttpClient::new(reqwest::Client::new(), server.uri());
            (server, client)
        }

        fn status_of(error: &HttpError) -> Option<(u16, Option<Duration>)> {
            match error {
                HttpError::Status {
                    status,
                    retry_after,
                    ..
                } => Some((status.as_u16(), *retry_after)),
                _ => None,
            }
        }

        #[tokio::test]
        async fn observe_returns_the_decoded_resource() {
            let record = json!({ "result": { "id": "record-1", "ttl": 300 } });
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(200).set_body_json(&record),
            )
            .await;

            let response = client.send::<RecordObserve>(&token(), RECORD, None).await;

            assert_eq!(response.ok(), Some(Some(record)));
        }

        #[tokio::test]
        async fn bearer_token_is_sent_in_the_authorization_header() {
            let server = MockServer::start().await;
            Mock::given(header("authorization", "Bearer t0ken"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
                .expect(1)
                .mount(&server)
                .await;
            let client = HttpClient::new(reqwest::Client::new(), server.uri());

            client
                .send::<RecordObserve>(&token(), RECORD, None)
                .await
                .expect("authorized");
        }

        #[tokio::test]
        async fn api_key_is_sent_in_its_own_header() {
            let server = MockServer::start().await;
            Mock::given(header("x-auth-key", "k3y"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
                .expect(1)
                .mount(&server)
                .await;
            let client = HttpClient::new(reqwest::Client::new(), server.uri());
            let auth = Auth::Header {
                name: "X-Auth-Key".to_owned(),
                value: "k3y".to_owned(),
            };

            client
                .send::<RecordObserve>(&auth, RECORD, None)
                .await
                .expect("authorized");
        }

        #[tokio::test]
        async fn create_sends_the_body_as_json() {
            let server = MockServer::start().await;
            Mock::given(method("POST"))
                .and(path("/zones/zone-1/records"))
                .and(body_json(json!({ "name": "www", "ttl": 300 })))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(json!({ "result": { "id": "record-1" } })),
                )
                .expect(1)
                .mount(&server)
                .await;
            let client = HttpClient::new(reqwest::Client::new(), server.uri());

            let created = client
                .send_json::<RecordCreate>(
                    &token(),
                    ZONE,
                    Some(&json!({ "name": "www", "ttl": 300 })),
                )
                .await;

            assert_eq!(
                created
                    .ok()
                    .flatten()
                    .and_then(|body| body.pointer("/result/id").cloned()),
                Some(json!("record-1"))
            );
        }

        #[tokio::test]
        async fn missing_resource_is_none_on_observe() {
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(404),
            )
            .await;

            assert_eq!(
                client
                    .send::<RecordObserve>(&token(), RECORD, None)
                    .await
                    .ok(),
                Some(None)
            );
        }

        #[tokio::test]
        async fn missing_resource_is_none_on_delete() {
            let (_server, client) = provider(
                "DELETE",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(404),
            )
            .await;

            assert_eq!(
                client
                    .send_discarding::<RecordDelete>(&token(), RECORD, None)
                    .await
                    .ok(),
                Some(false)
            );
        }

        #[tokio::test]
        async fn missing_parent_is_an_error_on_create() {
            let (_server, client) =
                provider("POST", "/zones/zone-1/records", ResponseTemplate::new(404)).await;

            let error = client
                .send_json::<RecordCreate>(&token(), ZONE, Some(&json!({})))
                .await
                .expect_err("404");

            assert_eq!(status_of(&error), Some((404, None)));
        }

        #[tokio::test]
        async fn empty_success_body_is_null() {
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(204),
            )
            .await;

            assert_eq!(
                client
                    .send::<RecordObserve>(&token(), RECORD, None)
                    .await
                    .ok(),
                Some(Some(Value::Null))
            );
        }

        #[tokio::test]
        async fn update_ignores_a_body_that_is_not_json() {
            let (_server, client) = provider(
                "PATCH",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(200).set_body_string("OK"),
            )
            .await;

            let updated = client
                .send_discarding::<RecordUpdate>(&token(), RECORD, Some(&json!({ "ttl": 60 })))
                .await;

            assert_eq!(updated.ok(), Some(true));
        }

        #[tokio::test]
        async fn observe_fails_on_a_body_that_is_not_json() {
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(200).set_body_string("<html>"),
            )
            .await;

            let error = client
                .send::<RecordObserve>(&token(), RECORD, None)
                .await
                .expect_err("not JSON");

            assert!(matches!(error, HttpError::Decode(_)));
        }

        #[tokio::test]
        async fn rate_limit_keeps_the_retry_after_delay() {
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(429).insert_header("retry-after", "7"),
            )
            .await;

            let error = client
                .send::<RecordObserve>(&token(), RECORD, None)
                .await
                .expect_err("429");

            assert_eq!(status_of(&error), Some((429, Some(Duration::from_secs(7)))));
        }

        #[tokio::test]
        async fn failure_reports_the_provider_message() {
            let (_server, client) = provider(
                "POST",
                "/zones/zone-1/records",
                ResponseTemplate::new(400)
                    .set_body_json(json!({ "errors": [{ "message": "Invalid TTL" }] })),
            )
            .await;

            let error = client
                .send_json::<RecordCreate>(&token(), ZONE, Some(&json!({})))
                .await
                .expect_err("400");

            assert_eq!(
                error.to_string(),
                "POST /zones/zone-1/records returned 400 Bad Request: Invalid TTL"
            );
        }

        #[tokio::test]
        async fn requests_are_counted_by_status_class() {
            let (_server, client) = provider(
                "GET",
                "/zones/zone-1/records/record-1",
                ResponseTemplate::new(404),
            )
            .await;
            let metrics = std::sync::Arc::new(crate::metrics::Metrics::new());
            let client = client.with_metrics(metrics.clone());

            client
                .send::<RecordObserve>(&token(), RECORD, None)
                .await
                .expect("404 is not an error");

            assert!(
                metrics
                    .encode()
                    .expect("encode")
                    .contains(r#"koof_external_requests_total{operation="RecordObserve",status_class="not_found"} 1"#)
            );
        }
    }
}
