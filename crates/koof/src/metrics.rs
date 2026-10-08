//! Prometheus metrics of the operator, served on `/metrics` by [`crate::App`].

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{Histogram, exponential_buckets},
    },
    registry::Registry,
};
use std::{
    collections::HashMap,
    sync::{Mutex, PoisonError},
    time::Duration,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct KindLabels {
    kind: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct ReconcileLabels {
    kind: String,
    result: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct ActionLabels {
    kind: String,
    action: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct RequestLabels {
    operation: String,
    status_class: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct OperationLabels {
    operation: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct StateLabels {
    kind: String,
    ready: String,
    synced: String,
}

type Histograms<L> = Family<L, Histogram, fn() -> Histogram>;

fn latency_histogram() -> Histogram {
    // 5 ms … ~41 s
    Histogram::new(exponential_buckets(0.005, 2.0, 14))
}

/// `Ready` / `Synced` of one managed resource, for the `managed_resources` gauge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectState {
    pub ready: bool,
    pub synced: bool,
}

/// Last known state of one managed resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TrackedObject {
    kind: &'static str,
    state: ObjectState,
}

pub struct Metrics {
    registry: Registry,
    reconciles: Family<ReconcileLabels, Counter>,
    reconcile_duration: Histograms<KindLabels>,
    actions: Family<ActionLabels, Counter>,
    drift: Family<KindLabels, Counter>,
    late_initialized: Family<KindLabels, Counter>,
    requests: Family<RequestLabels, Counter>,
    request_duration: Histograms<OperationLabels>,
    managed: Family<StateLabels, Gauge>,
    objects: Mutex<HashMap<String, TrackedObject>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Registry::with_prefix("koof");
        let reconciles = Family::default();
        let reconcile_duration =
            Histograms::new_with_constructor(latency_histogram as fn() -> Histogram);
        let actions = Family::default();
        let drift = Family::default();
        let late_initialized = Family::default();
        let requests = Family::default();
        let request_duration =
            Histograms::new_with_constructor(latency_histogram as fn() -> Histogram);
        let managed = Family::default();

        registry.register(
            "reconcile",
            "Reconciles by kind and result",
            reconciles.clone(),
        );
        registry.register(
            "reconcile_duration_seconds",
            "Reconcile duration",
            reconcile_duration.clone(),
        );
        registry.register(
            "actions",
            "External create / update / delete / orphan actions",
            actions.clone(),
        );
        registry.register(
            "drift_detected",
            "Reconciles that found drift",
            drift.clone(),
        );
        registry.register(
            "late_initialized",
            "Late initializations of spec.forProvider",
            late_initialized.clone(),
        );
        registry.register(
            "external_requests",
            "Provider API requests by operation and status class",
            requests.clone(),
        );
        registry.register(
            "external_request_duration_seconds",
            "Provider API request duration",
            request_duration.clone(),
        );
        registry.register(
            "managed_resources",
            "Managed resources by Ready / Synced state",
            managed.clone(),
        );

        Self {
            registry,
            reconciles,
            reconcile_duration,
            actions,
            drift,
            late_initialized,
            requests,
            request_duration,
            managed,
            objects: Mutex::default(),
        }
    }

    /// Prometheus text exposition.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] when encoding fails.
    pub fn encode(&self) -> Result<String, std::fmt::Error> {
        let mut output = String::new();
        prometheus_client::encoding::text::encode(&mut output, &self.registry)?;
        Ok(output)
    }

    pub fn reconciled(&self, kind: &str, result: &str, duration: Duration) {
        self.reconciles
            .get_or_create(&ReconcileLabels {
                kind: kind.to_owned(),
                result: result.to_owned(),
            })
            .inc();
        self.reconcile_duration
            .get_or_create(&KindLabels {
                kind: kind.to_owned(),
            })
            .observe(duration.as_secs_f64());
    }

    pub fn action(&self, kind: &str, action: &str) {
        self.actions
            .get_or_create(&ActionLabels {
                kind: kind.to_owned(),
                action: action.to_owned(),
            })
            .inc();
    }

    pub fn drift_detected(&self, kind: &str) {
        self.drift
            .get_or_create(&KindLabels {
                kind: kind.to_owned(),
            })
            .inc();
    }

    pub fn late_initialized(&self, kind: &str) {
        self.late_initialized
            .get_or_create(&KindLabels {
                kind: kind.to_owned(),
            })
            .inc();
    }

    /// `status_class`: `2xx`, `not_found`, `4xx`, `5xx` or `error` (no response).
    pub fn request(&self, operation: &str, status_class: &str, duration: Duration) {
        self.requests
            .get_or_create(&RequestLabels {
                operation: operation.to_owned(),
                status_class: status_class.to_owned(),
            })
            .inc();
        self.request_duration
            .get_or_create(&OperationLabels {
                operation: operation.to_owned(),
            })
            .observe(duration.as_secs_f64());
    }

    /// Records the Ready / Synced state of object `key`; `None` forgets it (the
    /// managed resource is gone).
    pub fn object_state(&self, key: &str, kind: &'static str, state: Option<ObjectState>) {
        let mut objects = self.objects.lock().unwrap_or_else(PoisonError::into_inner);
        let previous = match state {
            Some(state) => objects.insert(key.to_owned(), TrackedObject { kind, state }),
            None => objects.remove(key),
        };
        let current = objects.get(key).copied();
        if previous == current {
            return;
        }
        if let Some(previous) = previous {
            self.managed.get_or_create(&state_labels(&previous)).dec();
        }
        if let Some(current) = current {
            self.managed.get_or_create(&state_labels(&current)).inc();
        }
    }
}

fn state_labels(object: &TrackedObject) -> StateLabels {
    StateLabels {
        kind: object.kind.to_owned(),
        ready: object.state.ready.to_string(),
        synced: object.state.synced.to_string(),
    }
}
