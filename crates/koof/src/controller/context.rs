use super::{BACKOFF_BASE, RESYNC};
use crate::{external::http::HttpClient, metrics::Metrics};
use kube::runtime::events::Recorder;
use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex, PoisonError},
    time::Duration,
};

pub struct ControllerContext {
    pub kube_client: kube::Client,
    pub provider_client: HttpClient,
    pub recorder: Recorder,
    pub metrics: Arc<Metrics>,
    pub backoff: Backoff,
    pub synced: SyncTracker,
}

impl ControllerContext {
    #[must_use]
    pub fn new(
        kube_client: kube::Client,
        provider_client: HttpClient,
        recorder: Recorder,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            kube_client,
            provider_client,
            recorder,
            metrics,
            backoff: Backoff::default(),
            synced: SyncTracker::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Backoff {
    failures: Mutex<HashMap<String, u32>>,
}

impl Backoff {
    pub fn failed(&self, key: &str) -> u32 {
        let mut failures = self.failures.lock().unwrap_or_else(PoisonError::into_inner);
        let attempts = failures.entry(key.to_owned()).or_insert(0);
        *attempts = attempts.saturating_add(1);
        *attempts
    }

    pub fn succeeded(&self, key: &str) {
        self.failures
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(key);
    }

    #[must_use]
    pub fn delay(attempts: u32) -> Duration {
        let exponent = attempts.saturating_sub(1).min(16);
        BACKOFF_BASE.saturating_mul(1 << exponent).min(RESYNC)
    }
}

#[derive(Debug, Default)]
pub struct SyncTracker {
    synced: Mutex<BTreeSet<&'static str>>,
}

impl SyncTracker {
    pub fn mark(&self, kind: &'static str) {
        let newly = self
            .synced
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(kind);
        if newly {
            tracing::info!(resource = kind, "initial sync complete");
        }
    }

    #[must_use]
    pub fn all(&self, kinds: &[&'static str]) -> bool {
        let synced = self.synced.lock().unwrap_or_else(PoisonError::into_inner);
        kinds.iter().all(|kind| synced.contains(kind))
    }
}
