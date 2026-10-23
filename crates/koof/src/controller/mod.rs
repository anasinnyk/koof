pub mod conditions;
mod context;
pub mod create_state;
pub mod drift;
pub mod finalizer;
pub mod late_init;
pub mod policy;
pub mod status;
mod usage;

pub use context::{Backoff, ControllerContext, SyncTracker};

use crate::{
    error::ReconcileError,
    external,
    resource::{EXTERNAL_NAME, ManagedResource},
    retry::{self, RetryPolicy},
};
use conditions::ConditionSpec;
use create_state::CreateState;
use drift::drift;
use futures::{StreamExt as _, TryStreamExt as _};
use kube::runtime::{
    Controller, Predicate as _, PredicateConfig, WatchStreamExt as _,
    controller::Action,
    events::{Event, EventType},
    predicates, reflector, watcher,
};
use policy::{Action as PolicyAction, State};
use status::{record_failure, set_conditions, update_status};
use std::{sync::Arc, time::Duration};
use tracing::Instrument as _;

const RESYNC: Duration = Duration::from_mins(5);
const MISSING: Duration = Duration::from_secs(30);
const WAITING: Duration = Duration::from_secs(15);
const CONFIRM: Duration = Duration::from_secs(5);
const ORPHANED: &str = "External resource kept (orphaned)";
const NOT_DELETABLE: &str = "External resource kept: the API has no delete operation";
const RECONCILE_RETRY: RetryPolicy = RetryPolicy {
    base: Duration::from_secs(5),
    max: RESYNC,
    attempts: None,
    waiting: Some(WAITING),
    permanent: Some(RESYNC),
};

/// Observes the external resource and records the result in the status. On failure
/// records `Synced=False` and a warning event before returning the error.
///
/// # Errors
///
/// See [`external::observe`] and [`update_status`].
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
        let started = std::time::Instant::now();
        let result = reconcile_inner(resource.as_ref(), &context).await;
        context.metrics.reconciled(
            R::descriptor().kind,
            if result.is_ok() { "success" } else { "error" },
            started.elapsed(),
        );
        match result {
            Ok(action) => {
                context.backoff.succeeded(&object_key(resource.as_ref()));
                Ok(action)
            }
            Err(error) => {
                tracing::warn!(%error, failure = ?retry::classify(&error), "reconcile failed");
                if let Err(status_error) = record_failure(&context, resource.as_ref(), &error).await
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
        ensured = finalizer::ensure(&context.kube_client, resource).await?;
        &ensured
    };
    if deleting {
        let dependents = usage::dependents(context, resource).await?;
        if !dependents.is_empty() {
            return in_use(context, resource, &dependents).await;
        }
    }
    if deleting && R::descriptor().delete.is_none() {
        return release(context, resource, Some(NOT_DELETABLE)).await;
    }
    if deleting && policy::orphans_on_delete(resource.management()) {
        return release(context, resource, Some(ORPHANED)).await;
    }

    let observation = external::observe(context, resource).await?;
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
    if !drifted_fields.is_empty() {
        context.metrics.drift_detected(R::descriptor().kind);
    }

    if decision.late_initialize
        && let Some(observed) = &observation.at_provider
    {
        let fields = late_init::candidates(resource, observed)?;
        if !fields.is_empty() {
            late_init::apply(&context.kube_client, resource, &fields).await?;
            let names: Vec<&str> = fields.keys().map(String::as_str).collect();
            tracing::info!(fields = ?names, "late-initialized");
            context.metrics.late_initialized(R::descriptor().kind);
            publish(
                context,
                resource,
                EventType::Normal,
                "LateInitialized",
                &format!("Filled from the provider: {}", names.join(", ")),
            )
            .await;
            return Ok(Action::requeue(CONFIRM));
        }
    }

    match decision.action {
        PolicyAction::None => {}
        PolicyAction::Delete => return delete_external(context, resource).await,
        PolicyAction::Finalize => {
            return release(context, resource, observation.exists.then_some(ORPHANED)).await;
        }
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

    let change = update_status(context, resource, &observation).await?;
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

async fn delete_external<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Action, ReconcileError> {
    let existed = external::delete(context, resource).await?;
    tracing::info!(existed, "external resource deleted");
    context.metrics.action(R::descriptor().kind, "deleted");
    set_conditions(
        context,
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

async fn in_use<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    dependents: &[String],
) -> Result<Action, ReconcileError> {
    let message = format!(
        "Deletion waits until these resources are deleted: {}",
        dependents.join(", ")
    );
    tracing::info!(?dependents, "deletion blocked by dependents");
    set_conditions(
        context,
        resource,
        vec![
            ConditionSpec::in_use(message.clone()),
            ConditionSpec::reconcile_success(),
        ],
    )
    .await?;
    publish(context, resource, EventType::Normal, "InUse", &message).await;
    Ok(Action::requeue(WAITING))
}

async fn release<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    orphaned: Option<&str>,
) -> Result<Action, ReconcileError> {
    if let Some(note) = orphaned {
        tracing::info!("external resource orphaned");
        context.metrics.action(R::descriptor().kind, "orphaned");
        publish(context, resource, EventType::Normal, "Orphaned", note).await;
    }
    finalizer::remove(&context.kube_client, resource).await?;
    context
        .metrics
        .object_state(&object_key(resource), R::descriptor().kind, None);
    Ok(Action::await_change())
}

async fn update_external<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    drifted_fields: &[String],
) -> Result<Action, ReconcileError> {
    external::update(context, resource).await?;
    tracing::info!(?drifted_fields, "external resource updated");
    context.metrics.action(R::descriptor().kind, "updated");
    publish(
        context,
        resource,
        EventType::Normal,
        "Updated",
        &format!("External resource updated: {}", drifted_fields.join(", ")),
    )
    .await;

    let observation = external::observe(context, resource).await?;
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
    update_status(context, resource, &observation).await?;
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
            descriptor.annotation(create_state::CREATE_PENDING),
            descriptor.annotation(EXTERNAL_NAME),
            descriptor.annotation(create_state::CREATE_PENDING),
        )));
    }
    if state.recently_succeeded(jiff::Timestamp::now()) {
        tracing::debug!("created recently and not visible yet; waiting");
        set_conditions(
            context,
            resource,
            vec![
                ConditionSpec::creating(),
                ConditionSpec::reconcile_success(),
            ],
        )
        .await?;
        return Ok(Action::requeue(CONFIRM));
    }

    let external_name = external::create(context, resource).await?;
    tracing::info!(%external_name, "external resource created");
    context.metrics.action(R::descriptor().kind, "created");
    publish(
        context,
        resource,
        EventType::Normal,
        "Created",
        &format!("External resource created: {external_name}"),
    )
    .await;
    Ok(Action::requeue(CONFIRM))
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn error_policy<R: ManagedResource>(
    resource: Arc<R>,
    error: &ReconcileError,
    context: Arc<ControllerContext>,
) -> Action {
    let attempts = context.backoff.failed(&object_key(resource.as_ref()));
    let delay = RECONCILE_RETRY
        .delay(retry::classify(error), attempts)
        .unwrap_or(RESYNC);
    tracing::debug!(attempts, ?delay, "requeue after failure");
    Action::requeue(delay)
}

pub(crate) fn object_key<R: ManagedResource>(resource: &R) -> String {
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

pub async fn run<R: ManagedResource>(context: Arc<ControllerContext>) {
    let kind = R::descriptor().kind;
    let api = kube::Api::<R>::all(context.kube_client.clone());
    tracing::info!(resource = kind, "starting controller");

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
    let sync_context = context.clone();
    let stream = watcher(api, watcher::Config::default())
        .default_backoff()
        .reflect(writer)
        .inspect_ok(move |event| {
            if matches!(event, watcher::Event::InitDone) {
                sync_context.synced.mark(kind);
            }
        })
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
