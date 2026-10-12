pub mod http;
pub mod request;
pub mod resolve;

use crate::{
    controller::{
        ControllerContext,
        create_state::{self, CREATE_FAILED, CREATE_PENDING, CREATE_SUCCEEDED},
    },
    error::{ReconcileError, ResolveValueError},
    json_path,
    resource::{EXTERNAL_NAME, ManagedResource, OperationDescriptor},
};
use http::{Auth, HttpError, Operation};
use resolve::{Node, resolve_credentials, resolve_value};
use serde_json::Value;

pub struct Observation {
    pub exists: bool,
    pub at_provider: Option<Value>,
}

struct Call {
    parameters: Vec<(&'static str, String)>,
    auth: Auth,
}

impl Call {
    async fn prepare(
        context: &ControllerContext,
        node: &Node,
        operation: &OperationDescriptor,
    ) -> Result<Option<Self>, ReconcileError> {
        let Ok(parameters) = resolve_parameters(context, node, operation).await? else {
            return Ok(None);
        };
        let auth = resolve_credentials(&context.kube_client, node).await?;
        Ok(Some(Self { parameters, auth }))
    }

    async fn prepare_required(
        context: &ControllerContext,
        node: &Node,
        operation: &OperationDescriptor,
    ) -> Result<Self, ReconcileError> {
        let parameters = resolve_parameters(context, node, operation)
            .await?
            .map_err(|parameter| {
                ResolveValueError::Missing(format!(
                    "{}: could not resolve path parameter {parameter}",
                    node.descriptor.kind
                ))
            })?;
        let auth = resolve_credentials(&context.kube_client, node).await?;
        Ok(Self { parameters, auth })
    }

    fn parameters(&self) -> Vec<(&'static str, &str)> {
        self.parameters
            .iter()
            .map(|(name, value)| (*name, value.as_str()))
            .collect()
    }
}

async fn resolve_parameters(
    context: &ControllerContext,
    node: &Node,
    operation: &OperationDescriptor,
) -> Result<Result<Vec<(&'static str, String)>, &'static str>, ReconcileError> {
    let mut parameters = Vec::new();
    for (name, expression) in operation.path_parameters {
        let Some(value) = resolve_value(&context.kube_client, node, expression).await? else {
            tracing::debug!(parameter = name, "path parameter not set");
            return Ok(Err(name));
        };
        parameters.push((*name, value));
    }
    Ok(Ok(parameters))
}

fn unsupported(kind: &str, lifecycle: &str, hint: &str) -> ReconcileError {
    ReconcileError::Unsupported(format!("{kind} has no {lifecycle} operation{hint}"))
}

/// Reads the external resource through `R::Observe`; `exists: false` when its
/// path parameters are not set yet or the API answers `404`.
///
/// # Errors
///
/// Returns [`ReconcileError`] when a reference or the credentials cannot be
/// resolved, or the request fails.
pub async fn observe<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Observation, ReconcileError> {
    let node = Node::of(resource)?;
    let operation = &node.descriptor.observe;
    let Some(call) = Call::prepare(context, &node, operation).await? else {
        return Ok(Observation {
            exists: false,
            at_provider: None,
        });
    };
    let response = context
        .provider_client
        .send_json::<R::Observe>(&call.auth, &call.parameters(), None)
        .await?;

    Ok(Observation {
        exists: response.is_some(),
        at_provider: response
            .as_ref()
            .and_then(|response| json_path::get_non_null(response, operation.response_path))
            .cloned(),
    })
}

/// Creates the external resource through `R::Create` and records its external name
/// (bracketed by the bookkeeping of [`create_state`]). Returns the external name.
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
    let create = descriptor
        .create
        .as_ref()
        .ok_or_else(|| unsupported(descriptor.kind, "create", ""))?;

    let call = Call::prepare_required(context, &node, &create.operation).await?;
    let body: <R::Create as Operation>::Request =
        request::build(&context.kube_client, &node, &create.request).await?;

    let resource = create_state::annotate(
        &context.kube_client,
        resource,
        &[(CREATE_PENDING, Some(create_state::now()))],
    )
    .await?;

    let response = match context
        .provider_client
        .send_json::<R::Create>(&call.auth, &call.parameters(), Some(&body))
        .await
    {
        Ok(response) => response,
        Err(error @ HttpError::Status { .. }) => {
            create_state::annotate(
                &context.kube_client,
                &resource,
                &[(CREATE_FAILED, Some(create_state::now()))],
            )
            .await?;
            return Err(error.into());
        }
        Err(error) => return Err(error.into()),
    };

    let external_name = response
        .as_ref()
        .and_then(|body| json_path::get(body, create.external_name))
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

    create_state::annotate(
        &context.kube_client,
        &resource,
        &[
            (EXTERNAL_NAME, Some(external_name.clone())),
            (CREATE_SUCCEEDED, Some(create_state::now())),
        ],
    )
    .await?;
    Ok(external_name)
}

/// Sends the desired state through `R::Update` (response body ignored).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no update operation, a path
/// parameter is not set, the body cannot be built, a reference or the credentials
/// cannot be resolved, or the request fails.
pub async fn update<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<(), ReconcileError> {
    let node = Node::of(resource)?;
    let descriptor = node.descriptor;
    let update = descriptor
        .update
        .as_ref()
        .ok_or_else(|| unsupported(descriptor.kind, "update", ""))?;
    let call = Call::prepare_required(context, &node, &update.operation).await?;
    let body: <R::Update as Operation>::Request =
        request::build(&context.kube_client, &node, &update.request).await?;
    context
        .provider_client
        .send_discarding::<R::Update>(&call.auth, &call.parameters(), Some(&body))
        .await?;
    Ok(())
}

/// Deletes the external resource through `R::Delete`. `Ok(false)` when it was
/// already gone (or its path parameters are not set).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no delete operation, a
/// reference or the credentials cannot be resolved, or the request fails.
pub async fn delete<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<bool, ReconcileError> {
    let node = Node::of(resource)?;
    let descriptor = node.descriptor;
    let operation = descriptor
        .delete
        .as_ref()
        .ok_or_else(|| unsupported(descriptor.kind, "delete", "; set deletionPolicy: Orphan"))?;
    let Some(call) = Call::prepare(context, &node, operation).await? else {
        return Ok(false);
    };
    Ok(context
        .provider_client
        .send_discarding::<R::Delete>(&call.auth, &call.parameters(), None)
        .await?)
}
