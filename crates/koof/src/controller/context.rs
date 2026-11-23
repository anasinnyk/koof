use crate::{
    external::http::HttpClient,
    metrics::Metrics,
    resource::{GroupKind, ResourceDescriptor},
};
use kube::runtime::events::Recorder;
use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex, PoisonError},
    time::{Duration, Instant},
};

pub struct ControllerContext {
    pub kube_client: kube::Client,
    pub provider_client: HttpClient,
    pub recorder: Recorder,
    pub metrics: Arc<Metrics>,
    pub backoff: Backoff,
    pub synced: SyncTracker,
    pub attempts: Attempts,
    pub activity: Activity,
    pub descriptors: Vec<&'static ResourceDescriptor>,
    pub concurrency: u16,
    pub resync: Duration,
    pub reconcile_timeout: Duration,
    pub field_manager: String,
}

impl ControllerContext {
    #[must_use]
    pub fn new(
        kube_client: kube::Client,
        provider_client: HttpClient,
        recorder: Recorder,
        metrics: Arc<Metrics>,
        descriptors: Vec<&'static ResourceDescriptor>,
        concurrency: u16,
    ) -> Self {
        Self {
            kube_client,
            provider_client,
            recorder,
            metrics,
            backoff: Backoff::default(),
            synced: SyncTracker::default(),
            attempts: Attempts::default(),
            activity: Activity::default(),
            descriptors,
            concurrency,
            resync: DEFAULT_RESYNC,
            reconcile_timeout: DEFAULT_RECONCILE_TIMEOUT,
            field_manager: DEFAULT_FIELD_MANAGER.to_owned(),
        }
    }

    /// How often an object is reconciled again once in sync.
    #[must_use]
    pub fn with_resync(self, resync: Duration) -> Self {
        Self { resync, ..self }
    }

    /// How long one reconcile may take before it is abandoned and retried.
    #[must_use]
    pub fn with_reconcile_timeout(self, reconcile_timeout: Duration) -> Self {
        Self {
            reconcile_timeout,
            ..self
        }
    }

    /// Server-side apply field manager that co-owns the external name annotation.
    #[must_use]
    pub fn with_field_manager(self, field_manager: impl Into<String>) -> Self {
        Self {
            field_manager: field_manager.into(),
            ..self
        }
    }
}

pub const DEFAULT_RESYNC: Duration = Duration::from_mins(5);
pub const DEFAULT_RECONCILE_TIMEOUT: Duration = Duration::from_mins(5);

/// When each object's reconcile last started or finished, so objects whose
/// scheduled retry got lost can be found and reconciled again.
#[derive(Debug, Default)]
pub struct Activity {
    last: Mutex<HashMap<String, Instant>>,
}

impl Activity {
    pub fn touched(&self, key: &str) {
        self.last
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(key.to_owned(), Instant::now());
    }

    /// Of `keys`, those without activity for `idle` (or ever); forgets every other key.
    #[must_use]
    pub fn idle(&self, keys: Vec<String>, idle: Duration) -> Vec<String> {
        let mut last = self.last.lock().unwrap_or_else(PoisonError::into_inner);
        let known: BTreeSet<&String> = keys.iter().collect();
        last.retain(|key, _| known.contains(key));
        keys.into_iter()
            .filter(|key| last.get(key).is_none_or(|at| at.elapsed() >= idle))
            .collect()
    }
}
pub const DEFAULT_FIELD_MANAGER: &str = "koof";

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
}

type KnownDrift = (Option<i64>, Vec<String>);

#[derive(Debug, Default)]
pub struct Attempts {
    drift: Mutex<HashMap<String, KnownDrift>>,
    deletes: Mutex<HashMap<String, Instant>>,
}

impl Attempts {
    pub fn drift_persisted(&self, key: &str, generation: Option<i64>, fields: Vec<String>) {
        let mut drift = self.drift.lock().unwrap_or_else(PoisonError::into_inner);
        if fields.is_empty() {
            drift.remove(key);
        } else {
            drift.insert(key.to_owned(), (generation, fields));
        }
    }

    #[must_use]
    pub fn known_drift(&self, key: &str, generation: Option<i64>, fields: &[String]) -> bool {
        self.drift
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(key)
            .is_some_and(|(known_generation, known)| {
                *known_generation == generation && fields.iter().all(|field| known.contains(field))
            })
    }

    pub fn deleted(&self, key: &str) {
        self.deletes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(key.to_owned(), Instant::now());
    }

    #[must_use]
    pub fn delete_pending(&self, key: &str, window: Duration) -> bool {
        self.deletes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(key)
            .is_some_and(|sent| sent.elapsed() < window)
    }

    pub fn forget(&self, key: &str) {
        self.drift
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(key);
        self.deletes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(key);
    }
}

#[derive(Debug, Default)]
pub struct SyncTracker {
    synced: Mutex<BTreeSet<GroupKind>>,
}

impl SyncTracker {
    pub fn mark(&self, kind: GroupKind) {
        let newly = self
            .synced
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(kind);
        if newly {
            tracing::info!(resource = %kind, "initial sync complete");
        }
    }

    #[must_use]
    pub fn all(&self, kinds: &[GroupKind]) -> bool {
        let synced = self.synced.lock().unwrap_or_else(PoisonError::into_inner);
        kinds.iter().all(|kind| synced.contains(kind))
    }
}
