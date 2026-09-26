use crate::error::{CredentialError, ResolveValueError};
use k8s_openapi::api::core::v1::Secret;
use kube::Api;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A key of a Secret, for credentials. `namespace` defaults to the namespace of the
/// resource holding the reference; cluster-scoped resources must set it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SecretReference {
    pub name: String,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Reads `key` of Secret `name` in `namespace` as UTF-8.
///
/// # Errors
///
/// Returns [`CredentialError`] when the Secret cannot be read, has no such key, or
/// the value is not UTF-8.
pub async fn resolve_secret_key(
    client: &kube::Client,
    namespace: &str,
    name: &str,
    key: &str,
) -> Result<String, CredentialError> {
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    let secret = secrets.get(name).await?;

    if let Some(value) = secret.string_data.as_ref().and_then(|data| data.get(key)) {
        return Ok(value.clone());
    }

    let bytes = secret
        .data
        .as_ref()
        .and_then(|data| data.get(key))
        .ok_or_else(|| {
            CredentialError::MissingValue(format!(
                "Secret {namespace}/{name} does not contain key {key:?}"
            ))
        })?
        .0
        .clone();

    String::from_utf8(bytes).map_err(|source| CredentialError::InvalidSecretUtf8 {
        secret: format!("{namespace}/{name}"),
        key: key.to_owned(),
        source,
    })
}

/// Value at a dot path such as `spec.forProvider.zoneRef`.
#[must_use]
pub fn value_at<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    path.split('.')
        .try_fold(value, |current, segment| current.get(segment))
}

/// Scalar at a dot path as a string; `Ok(None)` when missing or `null`.
///
/// # Errors
///
/// Returns [`ResolveValueError::NonScalar`] when the value is an object or array.
pub fn scalar_at(
    value: &serde_json::Value,
    path: &str,
) -> Result<Option<String>, ResolveValueError> {
    let Some(value) = value_at(value, path) else {
        return Ok(None);
    };

    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(value) => Ok(Some(value.clone())),
        serde_json::Value::Number(value) => Ok(Some(value.to_string())),
        serde_json::Value::Bool(value) => Ok(Some(value.to_string())),
        _ => Err(ResolveValueError::NonScalar {
            path: path.to_owned(),
        }),
    }
}
