use serde_json::{Map, Value, json};
use std::fmt::Write as _;

/// How an operation's request body is sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Json,
    Multipart(Multipart),
}

/// A `multipart/form-data` body made from the JSON body: each field named in
/// `json` becomes a part holding that field as JSON, and each item
/// `{name, content}` of the list `files` a file part of type `file_type`. Other
/// fields are not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multipart {
    pub json: &'static [&'static str],
    pub files: &'static str,
    pub file_type: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub name: String,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum MultipartError {
    #[error("multipart content type has no boundary")]
    MissingBoundary,

    #[error("malformed multipart body: {0}")]
    Malformed(&'static str),

    #[error("part {0} is not UTF-8 text")]
    NotText(String),

    #[error("{0} must be a list of {{name, content}}")]
    NotFiles(String),

    #[error("part {name} is not JSON")]
    Json {
        name: String,
        #[source]
        source: serde_json::Error,
    },
}

impl Multipart {
    #[must_use]
    pub const fn new(
        json: &'static [&'static str],
        files: &'static str,
        file_type: &'static str,
    ) -> Self {
        Self {
            json,
            files,
            file_type,
        }
    }

    /// The parts of `body`.
    ///
    /// # Errors
    ///
    /// Returns [`MultipartError`] when `files` is not a list of `{name, content}`.
    pub fn parts(&self, body: &Value) -> Result<Vec<Part>, MultipartError> {
        let mut parts: Vec<Part> = self
            .json
            .iter()
            .filter_map(|name| {
                let value = body.get(*name).filter(|value| !value.is_null())?;
                Some(Part {
                    name: (*name).to_owned(),
                    filename: None,
                    content_type: Some("application/json".to_owned()),
                    body: value.to_string().into_bytes(),
                })
            })
            .collect();
        let files = match body.get(self.files) {
            None | Some(Value::Null) => &Vec::new(),
            Some(Value::Array(files)) => files,
            Some(_) => return Err(MultipartError::NotFiles(self.files.to_owned())),
        };
        for file in files {
            let (Some(name), Some(content)) = (
                file.get("name").and_then(Value::as_str),
                file.get("content").and_then(Value::as_str),
            ) else {
                return Err(MultipartError::NotFiles(self.files.to_owned()));
            };
            parts.push(Part {
                name: name.to_owned(),
                filename: Some(name.to_owned()),
                content_type: Some(self.file_type.to_owned()),
                body: content.as_bytes().to_vec(),
            });
        }
        Ok(parts)
    }

    /// The JSON body `parts` were made from: the inverse of [`parts`](Self::parts).
    ///
    /// # Errors
    ///
    /// Returns [`MultipartError`] when a `json` part is not JSON or a file is not
    /// UTF-8 text.
    pub fn body(&self, parts: Vec<Part>) -> Result<Value, MultipartError> {
        let mut body = Map::new();
        let mut files = Vec::new();
        for part in parts {
            if self.json.contains(&part.name.as_str()) {
                let value =
                    serde_json::from_slice(&part.body).map_err(|source| MultipartError::Json {
                        name: part.name.clone(),
                        source,
                    })?;
                body.insert(part.name, value);
            } else {
                files.push(file(part)?);
            }
        }
        body.insert(self.files.to_owned(), Value::Array(files));
        Ok(Value::Object(body))
    }
}

/// `parts` as a JSON list of `{name, content}`.
///
/// # Errors
///
/// Returns [`MultipartError`] when a part is not UTF-8 text.
pub fn files(parts: Vec<Part>) -> Result<Value, MultipartError> {
    parts
        .into_iter()
        .map(file)
        .collect::<Result<_, _>>()
        .map(Value::Array)
}

fn file(part: Part) -> Result<Value, MultipartError> {
    let content =
        String::from_utf8(part.body).map_err(|_| MultipartError::NotText(part.name.clone()))?;
    Ok(json!({ "name": part.name, "content": content }))
}

#[must_use]
pub fn is_multipart(content_type: &str) -> bool {
    content_type
        .split(';')
        .next()
        .is_some_and(|media| media.trim().eq_ignore_ascii_case("multipart/form-data"))
}

/// The `Content-Type` and body of a `multipart/form-data` message of `parts`.
#[must_use]
pub fn encode(parts: &[Part]) -> (String, Vec<u8>) {
    let boundary = boundary(parts);
    let mut body = Vec::new();
    for part in parts {
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        let mut disposition = format!(
            "Content-Disposition: form-data; name=\"{}\"",
            quoted(&part.name)
        );
        if let Some(filename) = &part.filename {
            let _ = write!(disposition, "; filename=\"{}\"", quoted(filename));
        }
        body.extend_from_slice(disposition.as_bytes());
        body.extend_from_slice(b"\r\n");
        if let Some(content_type) = &part.content_type {
            body.extend_from_slice(format!("Content-Type: {content_type}\r\n").as_bytes());
        }
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(&part.body);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    (format!("multipart/form-data; boundary={boundary}"), body)
}

fn quoted(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn boundary(parts: &[Part]) -> String {
    use std::hash::{BuildHasher as _, Hasher as _};
    let state = std::collections::hash_map::RandomState::new();
    (0_u32..=u32::MAX)
        .map(|attempt| {
            let mut hasher = state.build_hasher();
            hasher.write_u32(attempt);
            format!("koof-{:016x}", hasher.finish())
        })
        .find(|boundary| {
            !parts
                .iter()
                .any(|part| find(&part.body, boundary.as_bytes(), 0).is_some())
        })
        .unwrap_or_default()
}

/// The parts of a `multipart/form-data` `body` with `content_type`.
///
/// # Errors
///
/// Returns [`MultipartError`] when the content type has no boundary or the body is
/// not a multipart message.
pub fn decode(content_type: &str, body: &[u8]) -> Result<Vec<Part>, MultipartError> {
    let boundary = content_type
        .split(';')
        .filter_map(|parameter| parameter.trim().split_once('='))
        .find(|(key, _)| key.trim().eq_ignore_ascii_case("boundary"))
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
        .filter(|boundary| !boundary.is_empty())
        .ok_or(MultipartError::MissingBoundary)?;
    let delimiter = format!("--{boundary}");
    let delimiter = delimiter.as_bytes();

    let mut start = find(body, delimiter, 0).ok_or(MultipartError::Malformed("no boundary"))?;
    let mut parts = Vec::new();
    loop {
        let after = start + delimiter.len();
        if body[after..].starts_with(b"--") {
            return Ok(parts);
        }
        let end =
            find(body, delimiter, after).ok_or(MultipartError::Malformed("no closing boundary"))?;
        parts.push(part(&body[after..end])?);
        start = end;
    }
}

fn part(raw: &[u8]) -> Result<Part, MultipartError> {
    let raw = raw.strip_prefix(b"\r\n").unwrap_or(raw);
    let raw = raw.strip_suffix(b"\r\n").unwrap_or(raw);
    let split =
        find(raw, b"\r\n\r\n", 0).ok_or(MultipartError::Malformed("part without headers"))?;
    let headers = std::str::from_utf8(&raw[..split])
        .map_err(|_| MultipartError::Malformed("headers are not UTF-8"))?;
    let mut name = None;
    let mut filename = None;
    let mut content_type = None;
    for line in headers.split("\r\n") {
        let Some((header, value)) = line.split_once(':') else {
            continue;
        };
        if header.trim().eq_ignore_ascii_case("content-disposition") {
            name = parameter(value, "name");
            filename = parameter(value, "filename");
        } else if header.trim().eq_ignore_ascii_case("content-type") {
            content_type = Some(value.trim().to_owned());
        }
    }
    Ok(Part {
        name: name.ok_or(MultipartError::Malformed("part without a name"))?,
        filename,
        content_type,
        body: raw[split + 4..].to_vec(),
    })
}

fn parameter(header: &str, key: &str) -> Option<String> {
    header.split(';').skip(1).find_map(|parameter| {
        let (name, value) = parameter.trim().split_once('=')?;
        name.trim().eq_ignore_ascii_case(key).then(|| {
            let value = value.trim();
            value
                .strip_prefix('"')
                .and_then(|value| value.strip_suffix('"'))
                .unwrap_or(value)
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
        })
    })
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack
        .get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|position| position + from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SNIPPET: Multipart =
        Multipart::new(&["metadata"], "files", "application/javascript+module");

    fn snippet() -> Value {
        json!({
            "metadata": { "main_module": "main.js" },
            "files": [{ "name": "main.js", "content": "export default {};\r\n" }],
            "snippet_name": "not sent",
        })
    }

    #[test]
    fn a_body_round_trips_through_a_multipart_message() {
        let parts = SNIPPET.parts(&snippet()).expect("parts");
        let (content_type, message) = encode(&parts);
        let decoded = decode(&content_type, &message).expect("decoded");

        assert_eq!(decoded, parts);
        assert_eq!(
            SNIPPET.body(decoded).expect("body"),
            json!({
                "metadata": { "main_module": "main.js" },
                "files": [{ "name": "main.js", "content": "export default {};\r\n" }],
            })
        );
    }

    #[test]
    fn decodes_a_message_from_another_client() {
        let message = b"preamble\r\n--b\r\nContent-Disposition: form-data; name=\"main.js\"; filename=\"main.js\"\r\nContent-Type: application/javascript+module\r\n\r\nexport default {};\r\n--b--\r\n";

        assert_eq!(
            decode("multipart/form-data; boundary=\"b\"", message)
                .and_then(files)
                .expect("files"),
            json!([{ "name": "main.js", "content": "export default {};" }])
        );
    }

    #[test]
    fn files_must_be_a_list_of_names_and_contents() {
        let body = json!({ "files": [{ "name": "main.js" }] });

        assert_eq!(
            SNIPPET.parts(&body).map_err(|error| error.to_string()),
            Err("files must be a list of {name, content}".to_owned())
        );
    }

    #[test]
    fn a_message_needs_a_boundary() {
        assert_eq!(
            decode("multipart/form-data", b"").map_err(|error| error.to_string()),
            Err("multipart content type has no boundary".to_owned())
        );
    }
}
