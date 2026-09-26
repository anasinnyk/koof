//! Generic observe / reconcile / controller loop for any [`ManagedResource`].

use crate::{
    conditions::ConditionSpec,
    create::{self, CreateState},
    drift::drifted,
    error::{ReconcileError, ResolveValueError, Retry},
    finalizer,
    http::{HttpClient, extract},
    late_init,
    policy::{self, Action as PolicyAction, State},
    reference::value_at,
    resolve::{Node, resolve_credentials, resolve_value},
    resource::{EXTERNAL_NAME, ManagedResource, OperationDescriptor},
    status::{record_failure, set_conditions, update_status},
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
/// Delay before confirming the outcome of a create or delete.
const CONFIRM: Duration = Duration::from_secs(5);
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

/// Resolves the path parameters of `operation`. `Ok(None)` when one derived from the
/// resource itself (its external name) is not set: the external resource does not
/// exist yet. A referenced resource without an identifier is an error
/// (`ReferenceNotReady`).
async fn resolve_parameters(
    context: &ControllerContext,
    node: &Node,
    operation: &OperationDescriptor,
) -> Result<Option<Vec<(&'static str, String)>>, ReconcileError> {
    let mut parameters = Vec::new();
    for (name, expression) in operation.path_parameters {
        let Some(value) = resolve_value(&context.kube_client, node, expression).await? else {
            tracing::debug!(parameter = name, "external name not set");
            return Ok(None);
        };
        parameters.push((*name, value));
    }
    Ok(Some(parameters))
}

fn as_pairs<'a>(parameters: &'a [(&'static str, String)]) -> Vec<(&'static str, &'a str)> {
    parameters
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect()
}

/// Reads the external resource through `R::Observe`; see [`resolve_parameters`] for
/// a resource without an external name.
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

    let Some(parameters) = resolve_parameters(context, &node, &descriptor.observe).await? else {
        return Ok(Observation {
            exists: false,
            at_provider: None,
        });
    };
    let auth = resolve_credentials(&context.kube_client, &node).await?;
    let response = context
        .provider_client
        .send::<R::Observe>(&auth, &as_pairs(&parameters), None)
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

/// Sends the desired state through `R::Update` (response body ignored).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no update operation, the body
/// cannot be built, a reference or the credentials cannot be resolved, or the
/// request fails.
pub async fn update<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<(), ReconcileError> {
    let node = Node::of(resource)?;
    let operation = node.descriptor.update.as_ref().ok_or_else(|| {
        ReconcileError::Unsupported(format!("{} has no update operation", node.descriptor.kind))
    })?;
    let parameters = resolve_parameters(context, &node, &operation.operation)
        .await?
        .ok_or_else(|| {
            ResolveValueError::Missing(format!("{} has no external name", node.descriptor.kind))
        })?;
    let body: <R::Update as crate::http::Operation>::Request =
        crate::request::build(&context.kube_client, &node, &operation.request).await?;
    let auth = resolve_credentials(&context.kube_client, &node).await?;
    context
        .provider_client
        .send_discarding::<R::Update>(&auth, &as_pairs(&parameters), Some(&body))
        .await?;
    Ok(())
}

/// Desired fields that differ from the observation (see [`crate::drift`]).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource cannot be serialized.
pub fn drift<R: ManagedResource>(
    resource: &R,
    observation: &Observation,
) -> Result<Vec<String>, ReconcileError> {
    let Some(observed) = &observation.at_provider else {
        return Ok(Vec::new());
    };
    let from = R::descriptor()
        .update
        .as_ref()
        .map_or("spec.forProvider", |update| update.request.from);
    let value = serde_json::to_value(resource)?;
    Ok(value_at(&value, from).map_or_else(Vec::new, |desired| drifted(desired, observed)))
}

/// Deletes the external resource through `R::Delete`. `Ok(false)` when it was
/// already gone.
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
    let operation = node.descriptor.delete.as_ref().ok_or_else(|| {
        ResolveValueError::Missing(format!(
            "{} has no delete operation; set deletionPolicy: Orphan",
            node.descriptor.kind
        ))
    })?;
    let Some(parameters) = resolve_parameters(context, &node, operation).await? else {
        return Ok(false);
    };
    let auth = resolve_credentials(&context.kube_client, &node).await?;
    Ok(context
        .provider_client
        .send_discarding::<R::Delete>(&auth, &as_pairs(&parameters), None)
        .await?)
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
    let deleting = resource.meta().deletion_timestamp.is_some();
    let ensured;
    let resource = if deleting {
        resource
    } else {
        // Before anything can be created externally, so deletion is never missed.
        ensured = finalizer::ensure(&context.kube_client, resource).await?;
        &ensured
    };
    if deleting && policy::orphans_on_delete(resource.management()) {
        return release(context, resource, true).await;
    }

    let observation = observe(context, resource).await?;
    let drifted_fields = if observation.exists && !deleting {
        drift(resource, &observation)?
    } else {
        Vec::new()
    };
    let decision = policy::decide(
        State {
            exists: observation.exists,
            up_to_date: drifted_fields.is_empty(),
            deleting,
        },
        resource.management(),
    )?;
    tracing::debug!(?decision, ?drifted_fields, "decided");

    if decision.late_initialize
        && let Some(observed) = &observation.at_provider
    {
        let fields = late_init::candidates(resource, observed)?;
        if !fields.is_empty() {
            late_init::apply(&context.kube_client, resource, &fields).await?;
            let names: Vec<&str> = fields.keys().map(String::as_str).collect();
            tracing::info!(fields = ?names, "late-initialized");
            publish(
                context,
                resource,
                EventType::Normal,
                "LateInitialized",
                &format!("Filled from the provider: {}", names.join(", ")),
            )
            .await;
            // The spec change (new generation) triggers the next reconcile, which
            // continues with the updated spec.
            return Ok(Action::requeue(CONFIRM));
        }
    }

    match decision.action {
        PolicyAction::None => {}
        PolicyAction::Delete => return delete_external(context, resource).await,
        PolicyAction::Finalize => return release(context, resource, observation.exists).await,
        PolicyAction::Create if R::descriptor().create.is_some() => {
            return create_external(context, resource).await;
        }
        PolicyAction::Create => {
            tracing::debug!("external resource missing and the API has no create operation");
        }
        PolicyAction::Update if R::descriptor().update.is_some() => {
            return update_external(context, resource, &drifted_fields).await;
        }
        PolicyAction::Update => {
            tracing::debug!(
                ?drifted_fields,
                "drift detected but the API has no update operation"
            );
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

/// Deletes, then confirms on the next reconcile that the resource is gone before
/// releasing the managed resource.
async fn delete_external<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Action, ReconcileError> {
    let existed = delete(context, resource).await?;
    tracing::info!(existed, "external resource deleted");
    set_conditions(
        &context.kube_client,
        resource,
        vec![
            ConditionSpec::deleting(),
            ConditionSpec::reconcile_success(),
        ],
    )
    .await?;
    publish(
        context,
        resource,
        EventType::Normal,
        "Deleted",
        "External resource deleted",
    )
    .await;
    Ok(Action::requeue(CONFIRM))
}

/// Removes the finalizer; `orphaned` when the external resource is kept.
async fn release<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    orphaned: bool,
) -> Result<Action, ReconcileError> {
    if orphaned {
        tracing::info!("external resource orphaned");
        publish(
            context,
            resource,
            EventType::Normal,
            "Orphaned",
            "External resource kept (orphaned)",
        )
        .await;
    }
    finalizer::remove(&context.kube_client, resource).await?;
    Ok(Action::await_change())
}

/// Updates, then observes again in the same reconcile so the status is fresh. If
/// the drift persists (the API normalises a value, or ignores a field) the next
/// attempt waits for the regular resync instead of looping.
async fn update_external<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    drifted_fields: &[String],
) -> Result<Action, ReconcileError> {
    update(context, resource).await?;
    tracing::info!(?drifted_fields, "external resource updated");
    publish(
        context,
        resource,
        EventType::Normal,
        "Updated",
        &format!("External resource updated: {}", drifted_fields.join(", ")),
    )
    .await;

    let observation = observe(context, resource).await?;
    let remaining = drift(resource, &observation)?;
    if !remaining.is_empty() {
        tracing::warn!(?remaining, "drift persists after update");
        publish(
            context,
            resource,
            EventType::Warning,
            "DriftPersists",
            &format!(
                "Fields still differ after update (normalised or ignored by the API): {}",
                remaining.join(", ")
            ),
        )
        .await;
    }
    update_status(&context.kube_client, resource, &observation).await?;
    Ok(Action::requeue(RESYNC))
}

async fn create_external<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Action, ReconcileError> {
    let state = CreateState::of(resource);
    let descriptor = R::descriptor();
    if state.is_incomplete() {
        return Err(ReconcileError::CreateIncomplete(format!(
            "a previous create may have succeeded ({} is set, no outcome recorded); if the \
             external resource exists set {} to its ID, otherwise remove {}",
            descriptor.annotation(create::CREATE_PENDING),
            descriptor.annotation(EXTERNAL_NAME),
            descriptor.annotation(create::CREATE_PENDING),
        )));
    }
    if state.recently_succeeded(jiff::Timestamp::now()) {
        tracing::debug!("created recently and not visible yet; waiting");
        set_conditions(
            &context.kube_client,
            resource,
            vec![
                ConditionSpec::creating(),
                ConditionSpec::reconcile_success(),
            ],
        )
        .await?;
        return Ok(Action::requeue(CONFIRM));
    }

    let external_name = create::create(context, resource).await?;
    tracing::info!(%external_name, "external resource created");
    publish(
        context,
        resource,
        EventType::Normal,
        "Created",
        &format!("External resource created: {external_name}"),
    )
    .await;
    // The annotation patch triggers the next reconcile, which observes it.
    Ok(Action::requeue(CONFIRM))
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

    // Reconcile on spec (generation), external name, finalizer and deletion changes
    // only — not on the controller's own status updates or create bookkeeping
    // annotations (those would retrigger a failing create immediately instead of
    // honouring the error policy's backoff).
    let external_name_key = R::descriptor().annotation(EXTERNAL_NAME);
    let external_name = move |resource: &R| {
        kube::ResourceExt::annotations(resource)
            .get(&external_name_key)
            .map(|name| {
                let mut hasher = std::hash::DefaultHasher::new();
                std::hash::Hash::hash(name, &mut hasher);
                std::hash::Hasher::finish(&hasher)
            })
    };
    let deleting = |resource: &R| resource.meta().deletion_timestamp.as_ref().map(|_| 1_u64);
    let (reader, writer) = reflector::store();
    let stream = watcher(api, watcher::Config::default())
        .default_backoff()
        .reflect(writer)
        .applied_objects()
        .predicate_filter(
            predicates::generation
                .combine(external_name)
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
