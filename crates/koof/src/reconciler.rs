//! Generic observe / reconcile / controller loop for any [`ManagedResource`].

use crate::{
    error::{ReconcileError, ResolveValueError},
    http::{HttpClient, extract},
    resolve::{Node, resolve_credentials, resolve_value},
    resource::ManagedResource,
    status::update_status,
};
use futures::StreamExt as _;
use kube::runtime::{Controller, controller::Action, watcher};
use std::{sync::Arc, time::Duration};

pub struct Observation {
    pub exists: bool,
    pub at_provider: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct ControllerContext {
    pub kube_client: kube::Client,
    pub provider_client: HttpClient,
}

impl ControllerContext {
    #[must_use]
    pub fn new(kube_client: kube::Client, provider_client: HttpClient) -> Self {
        Self {
            kube_client,
            provider_client,
        }
    }
}

/// Reads the external resource through `R::Observe`.
///
/// # Errors
///
/// Returns [`ReconcileError`] when a path parameter or the credentials cannot be
/// resolved, or the request fails.
pub async fn observe<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Observation, ReconcileError> {
    let node = Node::of(resource)?;
    let descriptor = node.descriptor;

    let mut parameters = Vec::new();
    for (name, expression) in descriptor.observe.path_parameters {
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

    let auth = resolve_credentials(&context.kube_client, &node).await?;
    let response = context
        .provider_client
        .send::<R::Observe>(&auth, &parameters, None)
        .await?;

    let at_provider = match &response {
        Some(response) => extract(response, descriptor.observe.response_path)?,
        None => None,
    };
    Ok(Observation {
        exists: response.is_some(),
        at_provider,
    })
}

/// Observes the external resource and records the result in the status.
///
/// # Errors
///
/// See [`observe`] and [`update_status`].
pub async fn reconcile<R: ManagedResource>(
    resource: Arc<R>,
    context: Arc<ControllerContext>,
) -> Result<Action, ReconcileError> {
    let observation = observe(&context, resource.as_ref()).await?;
    update_status(&context.kube_client, resource.as_ref(), &observation).await?;

    // TODO(create): a missing external resource is only reported for now.
    let requeue_after = if observation.exists {
        Duration::from_mins(5)
    } else {
        Duration::from_secs(30)
    };
    Ok(Action::requeue(requeue_after))
}

#[must_use]
pub fn error_policy<R: ManagedResource>(
    _resource: Arc<R>,
    _error: &ReconcileError,
    _context: Arc<ControllerContext>,
) -> Action {
    Action::requeue(Duration::from_secs(30))
}

/// Runs the controller for `R` until a shutdown signal.
pub async fn run<R: ManagedResource>(context: Arc<ControllerContext>) {
    let kind = R::descriptor().kind;
    let api = kube::Api::<R>::all(context.kube_client.clone());

    Controller::new(api, watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile::<R>, error_policy::<R>, context)
        .for_each(|result| async move {
            match result {
                Ok((object, action)) => {
                    tracing::debug!(
                        resource = kind,
                        ?object,
                        ?action,
                        "reconciliation completed"
                    );
                }
                Err(error) => {
                    tracing::error!(resource = kind, ?error, "reconciliation failed");
                }
            }
        })
        .await;
}
