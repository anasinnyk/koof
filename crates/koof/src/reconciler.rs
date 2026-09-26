//! Generic observe / reconcile / controller loop for any [`ManagedResource`].

use crate::{
    error::{ReconcileError, Retry},
    http::{HttpClient, extract},
    policy::{self, Action as PolicyAction, State},
    resolve::{Node, resolve_credentials, resolve_value},
    resource::ManagedResource,
    status::{record_failure, update_status},
};
use futures::StreamExt as _;
use kube::runtime::{
    Controller, Predicate as _, PredicateConfig, WatchStreamExt as _,
    controller::Action,
    events::{Event, EventType, Recorder},
    predicates, reflector, watcher,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError},
    time::Duration,
};
use tracing::Instrument as _;

/// Requeue interval while the external resource is in sync.
const RESYNC: Duration = Duration::from_mins(5);
/// Requeue interval while the external resource does not exist.
const MISSING: Duration = Duration::from_secs(30);
/// Retry interval while waiting for a referenced resource or Secret.
const WAITING: Duration = Duration::from_secs(15);
/// First transient retry; doubles per consecutive failure up to [`RESYNC`].
const BACKOFF_BASE: Duration = Duration::from_secs(5);

pub struct Observation {
    pub exists: bool,
    pub at_provider: Option<serde_json::Value>,
}

pub struct ControllerContext {
    pub kube_client: kube::Client,
    pub provider_client: HttpClient,
    pub recorder: Recorder,
    /// Consecutive failures per object, for exponential backoff.
    failures: Mutex<HashMap<String, u32>>,
}

impl ControllerContext {
    #[must_use]
    pub fn new(kube_client: kube::Client, provider_client: HttpClient, recorder: Recorder) -> Self {
        Self {
            kube_client,
            provider_client,
            recorder,
            failures: Mutex::default(),
        }
    }

    fn failures(&self) -> std::sync::MutexGuard<'_, HashMap<String, u32>> {
        self.failures.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Reads the external resource through `R::Observe`.
///
/// When a path parameter derived from the resource itself (its external name) is
/// not set yet, the resource does not exist externally and no request is made. A
/// referenced resource without an identifier is an error (`ReferenceNotReady`).
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
    let descriptor = node.descriptor;

    let mut parameters = Vec::new();
    for (name, expression) in descriptor.observe.path_parameters {
        let Some(value) = resolve_value(&context.kube_client, &node, expression).await? else {
            tracing::debug!(
                parameter = name,
                "external name not set; resource does not exist yet"
            );
            return Ok(Observation {
                exists: false,
                at_provider: None,
            });
        };
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

/// Observes the external resource and records the result in the status. On failure
/// records `Synced=False` and a warning event before returning the error.
///
/// # Errors
///
/// See [`observe`] and [`update_status`].
pub async fn reconcile<R: ManagedResource>(
    resource: Arc<R>,
    context: Arc<ControllerContext>,
) -> Result<Action, ReconcileError> {
    let span = tracing::info_span!(
        "reconcile",
        kind = R::descriptor().kind,
        namespace = kube::ResourceExt::namespace(resource.as_ref()).unwrap_or_default(),
        name = kube::ResourceExt::name_any(resource.as_ref()),
        generation = resource.meta().generation,
    );
    async {
        match reconcile_inner(resource.as_ref(), &context).await {
            Ok(action) => {
                context.failures().remove(&object_key(resource.as_ref()));
                Ok(action)
            }
            Err(error) => {
                tracing::warn!(%error, retry = ?error.retry(), "reconcile failed");
                if let Err(status_error) =
                    record_failure(&context.kube_client, resource.as_ref(), &error).await
                {
                    tracing::warn!(error = %status_error, "cannot record failure in status");
                }
                publish(
                    &context,
                    resource.as_ref(),
                    EventType::Warning,
                    "ReconcileError",
                    &error.to_string(),
                )
                .await;
                Err(error)
            }
        }
    }
    .instrument(span)
    .await
}

async fn reconcile_inner<R: ManagedResource>(
    resource: &R,
    context: &ControllerContext,
) -> Result<Action, ReconcileError> {
    let observation = observe(context, resource).await?;
    let decision = policy::decide(
        State {
            exists: observation.exists,
            // Drift detection arrives in L4; until then an existing resource is
            // considered up to date.
            up_to_date: true,
            deleting: resource.meta().deletion_timestamp.is_some(),
        },
        resource.management(),
    )?;
    tracing::debug!(?decision, "decided");

    match decision.action {
        PolicyAction::None => {}
        // Executed from L2 (delete, finalize), L3 (create) and L4 (update) on.
        PolicyAction::Create
        | PolicyAction::Update
        | PolicyAction::Delete
        | PolicyAction::Finalize => {
            tracing::info!(action = ?decision.action, "action planned but not implemented yet");
        }
    }

    let change = update_status(&context.kube_client, resource, &observation).await?;
    if change.became_ready {
        publish(
            context,
            resource,
            EventType::Normal,
            "Available",
            "External resource is available",
        )
        .await;
    }

    Ok(Action::requeue(if observation.exists {
        RESYNC
    } else {
        MISSING
    }))
}

/// Requeue delay after a failed reconcile, by [`Retry`] class.
// The signature is fixed by `kube::runtime::Controller::run`.
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn error_policy<R: ManagedResource>(
    resource: Arc<R>,
    error: &ReconcileError,
    context: Arc<ControllerContext>,
) -> Action {
    let attempts = {
        let mut failures = context.failures();
        let attempts = failures.entry(object_key(resource.as_ref())).or_insert(0);
        *attempts = attempts.saturating_add(1);
        *attempts
    };
    let delay = match error.retry() {
        Retry::Transient(Some(retry_after)) => retry_after,
        Retry::Transient(None) => backoff(attempts),
        Retry::Waiting => WAITING,
        Retry::Permanent => RESYNC,
    };
    tracing::debug!(attempts, ?delay, "requeue after failure");
    Action::requeue(delay)
}

fn backoff(attempts: u32) -> Duration {
    let exponent = attempts.saturating_sub(1).min(16);
    BACKOFF_BASE.saturating_mul(1 << exponent).min(RESYNC)
}

fn object_key<R: ManagedResource>(resource: &R) -> String {
    format!(
        "{}/{}/{}",
        R::descriptor().kind,
        kube::ResourceExt::namespace(resource).unwrap_or_default(),
        kube::ResourceExt::name_any(resource)
    )
}

async fn publish<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    type_: EventType,
    reason: &str,
    note: &str,
) {
    let event = Event {
        type_,
        reason: reason.to_owned(),
        note: Some(note.to_owned()),
        action: "Reconcile".to_owned(),
        secondary: None,
    };
    if let Err(error) = context
        .recorder
        .publish(&event, &resource.object_ref(&()))
        .await
    {
        tracing::debug!(%error, "cannot publish event");
    }
}

/// Runs the controller for `R` until a shutdown signal.
pub async fn run<R: ManagedResource>(context: Arc<ControllerContext>) {
    let kind = R::descriptor().kind;
    let api = kube::Api::<R>::all(context.kube_client.clone());
    tracing::info!(resource = kind, "starting controller");

    // Reconcile on spec (generation), annotation (external name), finalizer and
    // deletion changes only — not on the controller's own status updates.
    let deleting = |resource: &R| resource.meta().deletion_timestamp.as_ref().map(|_| 1_u64);
    let (reader, writer) = reflector::store();
    let stream = watcher(api, watcher::Config::default())
        .default_backoff()
        .reflect(writer)
        .applied_objects()
        .predicate_filter(
            predicates::generation
                .combine(predicates::annotations)
                .combine(predicates::finalizers)
                .combine(deleting),
            PredicateConfig::default(),
        );

    Controller::for_stream(stream, reader)
        .shutdown_on_signal()
        .run(reconcile::<R>, error_policy::<R>, context)
        .for_each(|result| async move {
            if let Err(error) = result {
                tracing::debug!(resource = kind, %error, "controller event");
            }
        })
        .await;
}
