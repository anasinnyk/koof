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
pub struct AccountForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::AccountForProvider,
    #[serde(
        rename = "apiTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_token_secret_ref: Option<koof::reference::SecretReference>,
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
    group = "cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "Account",
    namespaced,
    status = "AccountStatus"
)]
pub struct AccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccountForProviderSpec,
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
pub struct AccountStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    #[schemars(transform = koof::schema::flatten_object_unions)]
    pub at_provider: Option<crate::generated::types::AccountAtProvider>,
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
    resource: &Account,
) -> Result<koof::reconciler::Observation, koof::error::ReconcileError> {
    let account_id = (async {
        koof::reference::resolve_field_value(resource, "status.atProvider.id")
    })
        .await?
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "account_id",),
            )
        })?;
    let auth = resolve_credentials(kube_client, resource).await?;
    let response = http
        .send::<
            crate::generated::operations::AccountObserve,
        >(&auth, &[("account_id", account_id.as_str())], None)
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
    resource: &Account,
) -> Result<koof::http::Auth, koof::error::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!("{} has no namespace", "Account",),
            )
        })?;
    let selector = (resource.spec.for_provider.api_token_secret_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = koof::reference::resolve_secret_key(client, &namespace, selector).await?;
    Ok(koof::http::Auth::Bearer(value))
}
pub async fn update_status(
    client: &kube::Client,
    resource: &Account,
    observation: &koof::reconciler::Observation,
) -> Result<(), koof::error::ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing("resource has no namespace".into())
        })?;
    let name = kube::ResourceExt::name_any(resource);
    let at_provider: Option<crate::generated::types::AccountAtProvider> = observation
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
    let status = AccountStatus {
        at_provider,
        observed_generation: resource.metadata.generation,
        conditions: vec![condition],
    };
    let patch = serde_json::json!({ "status" : status, });
    let api: kube::Api<Account> = kube::Api::namespaced(client.clone(), &namespace);
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
    resource: std::sync::Arc<Account>,
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
    _resource: std::sync::Arc<Account>,
    _error: &koof::error::ReconcileError,
    _context: std::sync::Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) -> kube::runtime::controller::Action {
    kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(30))
}
pub async fn run_controller(
    context: std::sync::Arc<koof::reconciler::ControllerContext<koof::http::HttpClient>>,
) {
    use futures::StreamExt as _;
    let api: kube::Api<Account> = kube::Api::all(context.kube_client.clone());
    kube::runtime::Controller::new(api, kube::runtime::watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok((object, action)) => {
                    tracing::debug!(
                        resource = "Account", ? object, ? action,
                        "reconciliation completed",
                    );
                }
                Err(error) => {
                    tracing::error!(
                        resource = "Account", ? error, "reconciliation failed",
                    );
                }
            }
        })
        .await;
}
