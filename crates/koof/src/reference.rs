use crate::error::{CredentialError, ResolveValueError};
use k8s_openapi::api::core::v1::{Secret, SecretKeySelector};
use kube::Api;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

pub type SecretReference = SecretKeySelector;

/// # Errors
///
/// Can return errors related to kubernets secret references
pub async fn resolve_secret_key(
    client: &kube::Client,
    namespace: &str,
    selector: &SecretKeySelector,
) -> Result<String, CredentialError> {
    let secret_name = selector.name.clone();

    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);

    let secret = secrets.get(&secret_name).await?;

    if let Some(value) = secret
        .string_data
        .as_ref()
        .and_then(|data| data.get(&selector.key))
    {
        return Ok(value.clone());
    }

    let bytes = secret
        .data
        .as_ref()
        .and_then(|data| data.get(&selector.key))
        .ok_or_else(|| {
            CredentialError::MissingValue(format!(
                "Secret {secret_name:?} does not contain key {:?}",
                selector.key,
            ))
        })?
        .0
        .clone();

    String::from_utf8(bytes).map_err(|source| CredentialError::InvalidSecretUtf8 {
        secret: secret_name.clone(),
        key: selector.key.clone(),
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
