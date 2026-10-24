use crate::config::LeaderElectionConfig;
use k8s_openapi::api::coordination::v1::{Lease, LeaseSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{MicroTime, ObjectMeta};
use kube::api::{Api, PostParams};
use std::time::Duration;
use tokio::time::Instant;

const SERVICE_ACCOUNT_NAMESPACE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/namespace";

#[derive(Debug, thiserror::Error)]
pub enum LeaderError {
    #[error(
        "cannot determine the leader election namespace; set KOOF_LEADER_ELECTION__NAMESPACE or \
         POD_NAMESPACE"
    )]
    Namespace,

    #[error("lost the leader election lease {namespace}/{name}")]
    Lost { namespace: String, name: String },
}

/// # Errors
///
/// Returns [`LeaderError::Namespace`] when neither the configuration, `POD_NAMESPACE`
/// nor the service account namespace file names a namespace.
pub fn namespace(config: &LeaderElectionConfig) -> Result<String, LeaderError> {
    config
        .namespace
        .clone()
        .or_else(|| std::env::var("POD_NAMESPACE").ok())
        .or_else(|| std::fs::read_to_string(SERVICE_ACCOUNT_NAMESPACE).ok())
        .map(|namespace| namespace.trim().to_owned())
        .filter(|namespace| !namespace.is_empty())
        .ok_or(LeaderError::Namespace)
}

#[must_use]
pub fn identity(pod_name: Option<&str>) -> String {
    pod_name
        .map(str::to_owned)
        .or_else(|| std::env::var("HOSTNAME").ok())
        .filter(|identity| !identity.is_empty())
        .unwrap_or_else(|| format!("koof-{}", std::process::id()))
}

pub struct LeaderElector {
    api: Api<Lease>,
    namespace: String,
    name: String,
    identity: String,
    lease_duration: Duration,
    renew_interval: Duration,
    observed: Option<(String, Instant)>,
}

impl LeaderElector {
    #[must_use]
    pub fn new(
        client: kube::Client,
        config: &LeaderElectionConfig,
        namespace: String,
        identity: String,
    ) -> Self {
        Self {
            api: Api::namespaced(client, &namespace),
            namespace,
            name: config.name.clone(),
            identity,
            lease_duration: config.lease_duration,
            renew_interval: config.renew_interval,
            observed: None,
        }
    }

    pub async fn acquire(&mut self) {
        tracing::info!(
            lease = self.name,
            namespace = self.namespace,
            identity = self.identity,
            "waiting for leadership"
        );
        loop {
            match self.try_acquire_or_renew().await {
                Ok(Outcome::Leading) => {
                    tracing::info!(lease = self.name, identity = self.identity, "leading");
                    return;
                }
                Ok(Outcome::Standby | Outcome::Conflict) => {}
                Err(error) => tracing::warn!(
                    lease = self.name,
                    error = crate::error::chain(&error),
                    "cannot acquire the lease"
                ),
            }
            tokio::time::sleep(self.renew_interval).await;
        }
    }

    pub async fn keep(&mut self) -> LeaderError {
        let deadline = self.lease_duration.saturating_sub(self.renew_interval);
        let retry = self.renew_interval / 5;
        let mut renewed = Instant::now();
        let mut wait = self.renew_interval;
        loop {
            tokio::time::sleep(wait).await;
            match self.try_acquire_or_renew().await {
                Ok(Outcome::Leading) => {
                    renewed = Instant::now();
                    wait = self.renew_interval;
                    continue;
                }
                Ok(Outcome::Standby) => break,
                Ok(Outcome::Conflict) => {}
                Err(error) => tracing::warn!(
                    lease = self.name,
                    error = crate::error::chain(&error),
                    "cannot renew the lease"
                ),
            }
            if renewed.elapsed() >= deadline {
                break;
            }
            wait = retry;
        }
        tracing::error!(
            lease = self.name,
            identity = self.identity,
            "lost leadership"
        );
        LeaderError::Lost {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
        }
    }

    pub async fn release(&self) {
        let result = async {
            let Some(mut lease) = self.api.get_opt(&self.name).await? else {
                return Ok(());
            };
            let spec = lease.spec.get_or_insert_with(LeaseSpec::default);
            if spec.holder_identity.as_deref() != Some(self.identity.as_str()) {
                return Ok(());
            }
            spec.holder_identity = None;
            spec.lease_duration_seconds = Some(1);
            spec.renew_time = Some(now());
            self.api
                .replace(&self.name, &PostParams::default(), &lease)
                .await
                .map(|_| ())
        }
        .await;
        match result {
            Ok(()) => tracing::info!(lease = self.name, "released the lease"),
            Err(error) => tracing::warn!(
                lease = self.name,
                error = crate::error::chain(&error),
                "cannot release the lease"
            ),
        }
    }

    async fn try_acquire_or_renew(&mut self) -> Result<Outcome, kube::Error> {
        let Some(mut lease) = self.api.get_opt(&self.name).await? else {
            let lease = Lease {
                metadata: ObjectMeta {
                    name: Some(self.name.clone()),
                    namespace: Some(self.namespace.clone()),
                    ..ObjectMeta::default()
                },
                spec: Some(self.spec(None)),
            };
            return conflict(self.api.create(&PostParams::default(), &lease).await);
        };
        let spec = lease.spec.clone().unwrap_or_default();
        let held = spec.holder_identity.as_deref() == Some(self.identity.as_str());
        if !held && !self.expired(&lease, &spec) {
            return Ok(Outcome::Standby);
        }
        let renewed = if held {
            LeaseSpec {
                renew_time: Some(now()),
                lease_duration_seconds: Some(self.lease_seconds()),
                ..spec
            }
        } else {
            self.spec(Some(&spec))
        };
        lease.spec = Some(renewed);
        conflict(
            self.api
                .replace(&self.name, &PostParams::default(), &lease)
                .await,
        )
    }

    fn expired(&mut self, lease: &Lease, spec: &LeaseSpec) -> bool {
        if spec.holder_identity.as_deref().is_none_or(str::is_empty) {
            return true;
        }
        let version = lease.metadata.resource_version.clone().unwrap_or_default();
        let observed = match &self.observed {
            Some((seen, at)) if *seen == version => *at,
            _ => {
                let at = Instant::now();
                self.observed = Some((version, at));
                at
            }
        };
        let duration = spec
            .lease_duration_seconds
            .and_then(|seconds| u64::try_from(seconds).ok())
            .map_or(self.lease_duration, Duration::from_secs);
        observed.elapsed() >= duration
    }

    fn spec(&self, previous: Option<&LeaseSpec>) -> LeaseSpec {
        let time = now();
        LeaseSpec {
            holder_identity: Some(self.identity.clone()),
            lease_duration_seconds: Some(self.lease_seconds()),
            acquire_time: Some(time.clone()),
            renew_time: Some(time),
            lease_transitions: Some(
                previous
                    .and_then(|spec| spec.lease_transitions)
                    .map_or(0, |transitions| transitions + 1),
            ),
            ..LeaseSpec::default()
        }
    }

    fn lease_seconds(&self) -> i32 {
        i32::try_from(self.lease_duration.as_secs()).unwrap_or(i32::MAX)
    }
}

fn now() -> MicroTime {
    MicroTime(jiff::Timestamp::now())
}

enum Outcome {
    Leading,
    Standby,
    Conflict,
}

fn conflict<T>(result: Result<T, kube::Error>) -> Result<Outcome, kube::Error> {
    match result {
        Ok(_) => Ok(Outcome::Leading),
        Err(kube::Error::Api(status)) if status.code == 409 => Ok(Outcome::Conflict),
        Err(error) => Err(error),
    }
}
