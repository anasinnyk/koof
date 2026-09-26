//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
/// `spec.forProvider`: the generated API fields plus references and
/// credentials.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
#[schemars(transform = koof::schema::flatten_object_unions)]
pub struct RulesetForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::RulesetForProvider,
    #[serde(rename = "zoneRef", default, skip_serializing_if = "Option::is_none")]
    pub zone_ref: Option<koof::reference::ResourceReference>,
}
#[derive(
    kube::CustomResource,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
#[kube(
    group = "rulesets.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "Ruleset",
    namespaced,
    status = "RulesetStatus"
)]
pub struct RulesetSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: RulesetForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    #[schemars(transform = koof::schema::flatten_object_unions)]
    pub at_provider: Option<crate::generated::types::RulesetAtProvider>,
    #[serde(
        rename = "observedGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_generation: Option<i64>,
    #[serde(default)]
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}
pub async fn observe(
    kube_client: &kube::Client,
    http: &koof::http::HttpClient,
    resource: &Ruleset,
) -> Result<koof::reconciler::Observation, koof::error::ReconcileError> {
    let ruleset_id = (async {
        koof::reference::resolve_field_value(resource, "status.atProvider.id")
    })
        .await?
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "ruleset_id",),
            )
        })?;
    let zone_id = (async {
        let root_namespace = kube::ResourceExt::namespace(resource)
            .ok_or_else(|| {
                koof::error::ResolveValueError::Missing(
                    "resource has no namespace".into(),
                )
            })?;
        let reference_0 = (resource.spec.for_provider.zone_ref)
            .as_ref()
            .ok_or_else(|| {
                koof::error::ResolveValueError::Missing(
                    format!(
                        "resource reference {} is not set", "spec.forProvider.zoneRef",
                    ),
                )
            })?;
        let namespace_0 = reference_0
            .namespace
            .clone()
            .unwrap_or_else(|| root_namespace.clone());
        let api_0: kube::Api<crate::generated::zone::Zone> = kube::Api::namespaced(
            kube_client.clone(),
            &namespace_0,
        );
        let related_0 = api_0.get(&reference_0.name).await?;
        (async {
            koof::reference::resolve_field_value(&related_0, "status.atProvider.id")
        })
            .await
    })
        .await?
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "zone_id",),
            )
        })?;
    let auth = resolve_credentials(kube_client, resource).await?;
    let response = http
        .send::<
            crate::generated::operations::ZoneRulesetObserve,
        >(
            &auth,
            &[("ruleset_id", ruleset_id.as_str()), ("zone_id", zone_id.as_str())],
            None,
        )
        .await?;
    let at_provider = match &response {
        Some(response) => koof::http::extract(response, "result")?,
        None => None,
    };
    Ok(koof::reconciler::Observation {
        exists: response.is_some(),
        at_provider,
    })
}
pub async fn resolve_credentials(
    client: &kube::Client,
    resource: &Ruleset,
) -> Result<koof::http::Auth, koof::error::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!("{} has no namespace", "Ruleset",),
            )
        })?;
    let reference_0 = (resource.spec.for_provider.zone_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!("resource reference {} is not set", "spec.forProvider.zoneRef",),
            )
        })?;
    let namespace_0 = reference_0.namespace.clone().unwrap_or_else(|| namespace.clone());
    let api_0: kube::Api<crate::generated::zone::Zone> = kube::Api::namespaced(
        client.clone(),
        &namespace_0,
    );
    let related_0 = api_0.get(&reference_0.name).await?;
    let reference_1 = (related_0.spec.for_provider.account_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!(
                    "resource reference {} is not set", "spec.forProvider.accountRef",
                ),
            )
        })?;
    let namespace_1 = reference_1
        .namespace
        .clone()
        .unwrap_or_else(|| namespace_0.clone());
    let api_1: kube::Api<crate::generated::account::Account> = kube::Api::namespaced(
        client.clone(),
        &namespace_1,
    );
    let related_1 = api_1.get(&reference_1.name).await?;
    let selector = (related_1.spec.for_provider.api_token_secret_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = koof::reference::resolve_secret_key(client, &namespace_1, selector)
        .await?;
    Ok(koof::http::Auth::Bearer(value))
}
pub async fn update_status(
    client: &kube::Client,
    resource: &Ruleset,
    observation: &koof::reconciler::Observation,
) -> Result<(), koof::error::ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing("resource has no namespace".into())
        })?;
    let name = kube::ResourceExt::name_any(resource);
    let at_provider: Option<crate::generated::types::RulesetAtProvider> = observation
        .at_provider
        .clone()
        .map(serde_json::from_value)
        .transpose()?;
    let condition_status = if observation.exists { "True" } else { "False" };
    let reason = if observation.exists { "Available" } else { "NotFound" };
    let message = if observation.exists {
        "External resource exists"
    } else {
        "External resource does not exist"
    };
    let previous_condition = resource
        .status
        .as_ref()
        .and_then(|status| {
            status.conditions.iter().find(|condition| { condition.type_ == "Ready" })
        });
    let last_transition_time = previous_condition
        .filter(|condition| {
            condition.status == condition_status && condition.reason == reason
        })
        .map(|condition| { condition.last_transition_time.clone() })
        .unwrap_or_else(|| {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Time::from(
                jiff::Timestamp::now(),
            )
        });
    let condition = k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition {
        type_: "Ready".into(),
        status: condition_status.into(),
        reason: reason.into(),
        message: message.into(),
        observed_generation: resource.metadata.generation,
        last_transition_time,
    };
    let status = RulesetStatus {
        at_provider,
        observed_generation: resource.metadata.generation,
        conditions: vec![condition],
    };
    let patch = serde_json::json!({ "status" : status, });
    let api: kube::Api<Ruleset> = kube::Api::namespaced(client.clone(), &namespace);
    let current_status = serde_json::to_value(&resource.status)?;
    let desired_status = serde_json::to_value(Some(&status))?;
    if current_status == desired_status {
        return Ok(());
    }
    api.patch_status(
            &name,
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(&patch),
        )
        .await?;
    Ok(())
}
pub async fn reconcile(
    resource: std::sync::Arc<Ruleset>,
    context: std::sync::Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) -> Result<kube::runtime::controller::Action, koof::error::ReconcileError> {
    let observation = observe(
            &context.kube_client,
            &context.provider_client,
            resource.as_ref(),
        )
        .await?;
    update_status(&context.kube_client, resource.as_ref(), &observation).await?;
    let requeue_after = if observation.exists {
        std::time::Duration::from_secs(300)
    } else {
        std::time::Duration::from_secs(30)
    };
    Ok(kube::runtime::controller::Action::requeue(requeue_after))
}
pub fn error_policy(
    _resource: std::sync::Arc<Ruleset>,
    _error: &koof::error::ReconcileError,
    _context: std::sync::Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) -> kube::runtime::controller::Action {
    kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(30))
}
pub async fn run_controller(
    context: std::sync::Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) {
    use futures::StreamExt as _;
    let api: kube::Api<Ruleset> = kube::Api::all(context.kube_client.clone());
    kube::runtime::Controller::new(api, kube::runtime::watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok((object, action)) => {
                    tracing::debug!(
                        resource = "Ruleset", ? object, ? action,
                        "reconciliation completed",
                    );
                }
                Err(error) => {
                    tracing::error!(
                        resource = "Ruleset", ? error, "reconciliation failed",
                    );
                }
            }
        })
        .await;
}
