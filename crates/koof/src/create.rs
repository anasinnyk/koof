//! Creating external resources without duplicates.
//!
//! Like Crossplane, every create is bracketed by annotations under the resource's
//! group:
//! - `external-create-pending` — written (with `resourceVersion`) right before the
//!   request, so two controllers cannot create concurrently;
//! - `external-create-succeeded` + `external-name` — written right after success;
//! - `external-create-failed` — written when the API definitely rejected it.
//!
//! A pending create without a later outcome means the result is unknown (e.g. the
//! connection dropped): creating again could duplicate the resource, so the
//! controller stops and asks for a decision instead.

use crate::{
    error::{ReconcileError, ResolveValueError},
    http::{HttpError, Operation},
    reconciler::ControllerContext,
    reference::value_at,
    resolve::{Node, resolve_credentials, resolve_value},
    resource::{EXTERNAL_NAME, ManagedResource, RequestMapping},
};
use serde_json::{Map, Value};
use std::time::Duration;

pub const CREATE_PENDING: &str = "external-create-pending";
pub const CREATE_SUCCEEDED: &str = "external-create-succeeded";
pub const CREATE_FAILED: &str = "external-create-failed";

/// After a successful create the API may not show the resource for a while; do not
/// create again within this window.
pub const CREATE_GRACE: Duration = Duration::from_mins(1);

/// Create bookkeeping read from the annotations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateState {
    pub pending: Option<jiff::Timestamp>,
    pub succeeded: Option<jiff::Timestamp>,
    pub failed: Option<jiff::Timestamp>,
}

impl CreateState {
    #[must_use]
    pub fn of<R: ManagedResource>(resource: &R) -> Self {
        let descriptor = R::descriptor();
        let annotations = kube::ResourceExt::annotations(resource);
        let timestamp = |name: &str| {
            annotations
                .get(&descriptor.annotation(name))
                .and_then(|value| value.parse::<jiff::Timestamp>().ok())
        };
        Self {
            pending: timestamp(CREATE_PENDING),
            succeeded: timestamp(CREATE_SUCCEEDED),
            failed: timestamp(CREATE_FAILED),
        }
    }

    /// A create was started and neither success nor failure was recorded after it.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.pending.is_some_and(|pending| {
            self.succeeded.is_none_or(|succeeded| succeeded < pending)
                && self.failed.is_none_or(|failed| failed < pending)
        })
    }

    /// A create succeeded less than [`CREATE_GRACE`] ago.
    #[must_use]
    pub fn recently_succeeded(&self, now: jiff::Timestamp) -> bool {
        self.succeeded.is_some_and(|succeeded| {
            now.duration_since(succeeded)
                .unsigned_abs()
                .lt(&CREATE_GRACE)
        })
    }
}

/// Creates the external resource through `R::Create` and records its external name.
/// Returns the external name.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no create operation, the request
/// body cannot be built (unresolved reference, body not matching the request type),
/// the create fails, or the response has no external name.
pub async fn create<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<String, ReconcileError> {
    let node = Node::of(resource)?;
    let descriptor = node.descriptor;
    let create = descriptor.create.as_ref().ok_or_else(|| {
        ReconcileError::Unsupported(format!("{} has no create operation", descriptor.kind))
    })?;

    let mut parameters = Vec::new();
    for (name, expression) in create.operation.path_parameters {
        let value = resolve_value(&context.kube_client, &node, expression)
            .await?
            .ok_or_else(|| {
                ResolveValueError::Missing(format!("could not resolve path parameter {name}"))
            })?;
        parameters.push((*name, value));
    }
    let parameters: Vec<(&str, &str)> = parameters
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect();
    let body: <R::Create as Operation>::Request =
        build_request(&context.kube_client, &node, &create.request).await?;
    let auth = resolve_credentials(&context.kube_client, &node).await?;

    let resource = annotate(
        &context.kube_client,
        resource,
        &[(CREATE_PENDING, Some(now()))],
    )
    .await?;

    let response = match context
        .provider_client
        .send_json::<R::Create>(&auth, &parameters, Some(&body))
        .await
    {
        Ok(response) => response,
        Err(error @ HttpError::Status { .. }) => {
            // The API answered: nothing was created, a later retry is safe.
            annotate(
                &context.kube_client,
                &resource,
                &[(CREATE_FAILED, Some(now()))],
            )
            .await?;
            return Err(error.into());
        }
        // Transport errors leave the create pending: its outcome is unknown.
        Err(error) => return Err(error.into()),
    };

    let external_name = response
        .as_ref()
        .and_then(|body| value_at(body, create.external_name))
        .and_then(|value| match value {
            Value::String(name) => Some(name.clone()),
            Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
        .ok_or_else(|| {
            ReconcileError::CreateIncomplete(format!(
                "create succeeded but the response has no `{}`; set the {} annotation to \
                 the new resource's ID",
                create.external_name,
                descriptor.annotation(EXTERNAL_NAME)
            ))
        })?;

    annotate(
        &context.kube_client,
        &resource,
        &[
            (EXTERNAL_NAME, Some(external_name.clone())),
            (CREATE_SUCCEEDED, Some(now())),
        ],
    )
    .await?;
    Ok(external_name)
}

/// Request body: the object at `mapping.from` with `mapping.set` applied,
/// deserialized into `T` (unknown fields are dropped).
///
/// # Errors
///
/// Returns [`ReconcileError`] when `from` is not an object, a `set` value cannot be
/// resolved, or the JSON does not match `T`.
pub async fn build_request<T: serde::de::DeserializeOwned>(
    client: &kube::Client,
    node: &Node,
    mapping: &RequestMapping,
) -> Result<T, ReconcileError> {
    let mut body = match value_at(&node.value, mapping.from) {
        Some(Value::Object(object)) => Value::Object(object.clone()),
        None | Some(Value::Null) => Value::Object(Map::new()),
        Some(_) => {
            return Err(ResolveValueError::NonScalar {
                path: mapping.from.to_owned(),
            }
            .into());
        }
    };
    for (path, expression) in mapping.set {
        let value = resolve_value(client, node, expression)
            .await?
            .ok_or_else(|| {
                ResolveValueError::Missing(format!("request field {path} could not be resolved"))
            })?;
        set_path(&mut body, path, Value::String(value));
    }
    serde_json::from_value(body).map_err(|error| {
        ReconcileError::InvalidRequest(format!(
            "request body built from {} does not match the API: {error}",
            mapping.from
        ))
    })
}

/// Writes `value` at a dot path, creating intermediate objects.
fn set_path(target: &mut Value, path: &str, value: Value) {
    let mut current = target;
    let mut segments = path.split('.').peekable();
    while let Some(segment) = segments.next() {
        if !current.is_object() {
            *current = Value::Object(Map::new());
        }
        let Value::Object(object) = current else {
            return;
        };
        if segments.peek().is_none() {
            object.insert(segment.to_owned(), value);
            return;
        }
        current = object
            .entry(segment.to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
    }
}

/// Sets (or with `None`, removes) koof annotations; returns the updated object.
/// The patch carries `resourceVersion`, so a concurrent change fails with a
/// conflict instead of racing.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn annotate<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    values: &[(&str, Option<String>)],
) -> Result<R, ReconcileError> {
    let descriptor = R::descriptor();
    let namespace = kube::ResourceExt::namespace(resource).ok_or_else(|| {
        ResolveValueError::Missing(format!("{} has no namespace", descriptor.kind))
    })?;
    let annotations: Map<String, Value> = values
        .iter()
        .map(|(name, value)| {
            (
                descriptor.annotation(name),
                value.clone().map_or(Value::Null, Value::String),
            )
        })
        .collect();
    let patch = serde_json::json!({
        "metadata": {
            "annotations": annotations,
            "resourceVersion": kube::ResourceExt::resource_version(resource),
        }
    });
    Ok(kube::Api::<R>::namespaced(client.clone(), &namespace)
        .patch(
            &kube::ResourceExt::name_any(resource),
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(&patch),
        )
        .await?)
}

fn now() -> String {
    jiff::Timestamp::now().to_string()
}
