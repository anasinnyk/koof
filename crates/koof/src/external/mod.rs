pub mod http;
pub mod request;
pub mod resolve;

use crate::{
    controller::{
        ControllerContext,
        create_state::{self, CREATE_FAILED, CREATE_PENDING, CREATE_SUCCEEDED},
        external_name,
    },
    error::{ReconcileError, ResolveValueError},
    resource::{
        EXTERNAL_NAME, ManagedResource, MergeDescriptor, ObserveDescriptor, OperationDescriptor,
    },
};
use http::{Auth, HttpError, Operation};
use resolve::{Node, resolve_credentials, resolve_value, with_item_references, with_secret_values};
use serde_json::{Map, Value};

pub struct Observation {
    pub exists: bool,
    pub at_provider: Option<Value>,
}

/// A created external resource: its external name and, when the response holds it,
/// the resource as the API returned it.
pub struct Created {
    pub external_name: String,
    pub resource: Option<Value>,
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
            .map_err(|parameter| ResolveValueError::PathParameterNotSet {
                kind: node.descriptor.kind,
                parameter,
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

fn unsupported(kind: &'static str, lifecycle: &'static str) -> ReconcileError {
    ReconcileError::Unsupported { kind, lifecycle }
}

/// Reads the external resource through `R::Observe`; `exists: false` when its
/// path parameters are not set yet, the API answers `404`, no item of a list
/// response matches (`matching`) or the resource is marked deleted (`deleted_when`).
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
    let observe = &node.descriptor.observe;
    let Some(call) = Call::prepare(context, &node, &observe.operation).await? else {
        return Ok(Observation {
            exists: false,
            at_provider: None,
        });
    };
    let matching = match &observe.matching {
        Some(matching) => {
            let Some(value) = resolve_value(&context.kube_client, &node, &matching.value).await?
            else {
                tracing::debug!("value to match the observed item by is not set");
                return Ok(Observation {
                    exists: false,
                    at_provider: None,
                });
            };
            Some((matching.field, value))
        }
        None => None,
    };
    let response = context
        .provider_client
        .send_json::<R::Observe>(&call.auth, &call.parameters(), None)
        .await?;

    let matching = matching
        .as_ref()
        .map(|(field, value)| (*field, value.as_str()));
    let mut observation = observation(response.as_ref(), observe, matching);
    if let (Some(merge), Some(Value::Object(at_provider))) =
        (&node.descriptor.merge, observation.at_provider.as_mut())
    {
        let value = merged::<R>(context, &node, merge).await?;
        at_provider.insert(merge.into.to_owned(), value);
    }
    Ok(observation)
}

async fn merged<R: ManagedResource>(
    context: &ControllerContext,
    node: &Node,
    merge: &MergeDescriptor,
) -> Result<Value, ReconcileError> {
    let Some(call) = Call::prepare(context, node, &merge.operation).await? else {
        return Ok(Value::Null);
    };
    let response = context
        .provider_client
        .send_json::<R::Merge>(&call.auth, &call.parameters(), None)
        .await?;
    Ok(response
        .as_ref()
        .and_then(|response| resolve::non_null(response, merge.operation.response_path))
        .cloned()
        .unwrap_or(Value::Null))
}

fn matched_item<'a>(list: &'a Value, field: &str, value: &str) -> Option<&'a Value> {
    list.as_array()?
        .iter()
        .find(|item| match item.pointer(field) {
            Some(Value::String(text)) => text == value,
            Some(Value::Number(number)) => number.to_string() == value,
            _ => false,
        })
}

fn observation(
    response: Option<&Value>,
    observe: &ObserveDescriptor,
    matching: Option<(&str, &str)>,
) -> Observation {
    let found =
        response.and_then(|response| resolve::non_null(response, observe.operation.response_path));
    let at_provider = match matching {
        Some((field, value)) => found.and_then(|list| matched_item(list, field, value)),
        None => found,
    }
    .cloned();
    let at_provider = match observe.into {
        Some(field) => response.map(|_| {
            Value::Object(Map::from_iter([(
                field.to_owned(),
                at_provider.unwrap_or(Value::Null),
            )]))
        }),
        None => at_provider,
    };
    let exists = match matching {
        Some(_) => at_provider.is_some(),
        None => response.is_some(),
    };
    let deleted = observe.deleted_when.is_some_and(|path| {
        at_provider
            .as_ref()
            .is_some_and(|resource| resolve::non_null(resource, path).is_some())
    });
    if deleted {
        return Observation {
            exists: false,
            at_provider: None,
        };
    }
    Observation {
        exists,
        at_provider,
    }
}

/// Creates the external resource through `R::Create` and records its external name
/// (bracketed by the bookkeeping of [`create_state`]).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no create operation, the request
/// body cannot be built (unresolved reference, body not matching the request type),
/// the create fails, or the response has no external name.
pub async fn create<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Created, ReconcileError> {
    let node = desired_request(context, resource).await?;
    let descriptor = node.descriptor;
    let create = descriptor
        .create
        .as_ref()
        .ok_or_else(|| unsupported(descriptor.kind, "create"))?;

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
        Err(error) if not_created(&error) => {
            create_state::record(
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
        .and_then(|body| body.pointer(create.external_name))
        .and_then(|value| match value {
            Value::String(name) => Some(name.clone()),
            Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
        .ok_or_else(|| ReconcileError::CreateNoIdentifier {
            pointer: create.external_name,
            annotation: descriptor.annotation(EXTERNAL_NAME),
        })?;

    tracing::info!(%external_name, "recording the external name of the created resource");
    let recorded = async {
        external_name::record(
            &context.kube_client,
            &resource,
            &context.field_manager,
            &external_name,
        )
        .await?;
        create_state::record(
            &context.kube_client,
            &resource,
            &[(CREATE_SUCCEEDED, Some(create_state::now()))],
        )
        .await
    };
    recorded
        .await
        .map_err(|source| ReconcileError::CreateNotRecorded {
            external_name: external_name.clone(),
            annotation: descriptor.annotation(EXTERNAL_NAME),
            source: Box::new(source),
        })?;
    let resource = response
        .as_ref()
        .and_then(|body| resolve::non_null(body, create.operation.response_path))
        .cloned();
    Ok(Created {
        external_name,
        resource,
    })
}

async fn desired_request<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Node, ReconcileError> {
    let node = with_item_references(&context.kube_client, &Node::of(resource)?).await?;
    Ok(with_secret_values(&context.kube_client, &node).await?)
}

fn not_created(error: &HttpError) -> bool {
    match error {
        HttpError::Status { status, .. } => status.is_client_error(),
        HttpError::Transport(error) => error.is_connect(),
        HttpError::MissingParameter(_)
        | HttpError::InvalidParameter { .. }
        | HttpError::Decode(_)
        | HttpError::Multipart(_) => false,
    }
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
    let node = desired_request(context, resource).await?;
    let descriptor = node.descriptor;
    let update = descriptor
        .update
        .as_ref()
        .ok_or_else(|| unsupported(descriptor.kind, "update"))?;
    let call = Call::prepare_required(context, &node, &update.operation).await?;
    let body: <R::Update as Operation>::Request =
        request::build(&context.kube_client, &node, &update.request).await?;
    context
        .provider_client
        .send_discarding::<R::Update>(&call.auth, &call.parameters(), Some(&body))
        .await?;
    Ok(())
}

/// Reads the connection details that `R::Connection` returns, at its
/// `response_path`; `None` when the resource has no connection operation, its path
/// parameters are not set yet or the API answers `404`.
///
/// # Errors
///
/// Returns [`ReconcileError`] when a reference or the credentials cannot be
/// resolved, or the request fails.
pub async fn connection<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Option<Value>, ReconcileError> {
    let node = Node::of(resource)?;
    let Some(connection) = &node.descriptor.connection else {
        return Ok(None);
    };
    let Some(call) = Call::prepare(context, &node, &connection.operation).await? else {
        return Ok(None);
    };
    let response = context
        .provider_client
        .send_json::<R::Connection>(&call.auth, &call.parameters(), None)
        .await?;
    Ok(response
        .as_ref()
        .and_then(|response| resolve::non_null(response, connection.operation.response_path))
        .cloned())
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
        .ok_or_else(|| unsupported(descriptor.kind, "delete"))?;
    let Some(call) = Call::prepare(context, &node, operation).await? else {
        return Ok(false);
    };
    Ok(context
        .provider_client
        .send_discarding::<R::Delete>(&call.auth, &call.parameters(), None)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{ResponseMatch, ValueExpr};
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn teams() -> Value {
        json!({ "result": [{ "id": "1", "slug": "a" }, { "id": 2, "slug": "b" }] })
    }

    fn observe() -> ObserveDescriptor {
        ObserveDescriptor::new(OperationDescriptor::new().with_response_path("/result"))
            .with_match(ResponseMatch::new("/id", ValueExpr::Literal("")))
    }

    #[test]
    fn matched_item_of_a_list_is_the_resource() {
        let observation = observation(Some(&teams()), &observe(), Some(("/id", "2")));

        assert_eq!(
            (observation.exists, observation.at_provider),
            (true, Some(json!({ "id": 2, "slug": "b" })))
        );
    }

    #[test]
    fn resource_is_missing_when_no_item_matches() {
        let observation = observation(Some(&teams()), &observe(), Some(("/id", "3")));

        assert_eq!((observation.exists, observation.at_provider), (false, None));
    }

    #[test]
    fn resource_is_missing_when_the_response_is_not_a_list() {
        let response = json!({ "result": { "id": "1" } });
        let observation = observation(Some(&response), &observe(), Some(("/id", "1")));

        assert_eq!((observation.exists, observation.at_provider), (false, None));
    }

    #[test]
    fn a_bare_list_is_the_field_into_of_the_resource() {
        let observe =
            ObserveDescriptor::new(OperationDescriptor::new().with_response_path("/result"))
                .with_into("rules");
        let response = json!({ "result": [{ "id": "1" }] });
        let observation = observation(Some(&response), &observe, None);

        assert_eq!(
            (observation.exists, observation.at_provider),
            (true, Some(json!({ "rules": [{ "id": "1" }] })))
        );
    }

    #[test]
    fn an_empty_bare_list_is_an_unset_field() {
        let observe =
            ObserveDescriptor::new(OperationDescriptor::new().with_response_path("/result"))
                .with_into("rules");
        let response = json!({ "result": null });
        let observation = observation(Some(&response), &observe, None);

        assert_eq!(
            (observation.exists, observation.at_provider),
            (true, Some(json!({ "rules": null })))
        );
    }
}
