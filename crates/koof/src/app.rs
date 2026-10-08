//! Operator entry point: everything a provider binary needs in one builder.
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     koof::App::new(&my_provider::generated::PROVIDER)
//!         .controller::<my_provider::generated::zone::Zone>()
//!         .run()
//!         .await
//! }
//! ```

use crate::{
    controller::{ControllerContext, run},
    http::HttpClient,
    metrics::Metrics,
    resource::ManagedResource,
};
use futures::future::BoxFuture;
use kube::runtime::events::{Recorder, Reporter};
use std::sync::Arc;

/// Generated per provider (`crate::generated::PROVIDER`).
#[derive(Debug)]
pub struct ProviderInfo {
    /// Short provider name, used as the event reporter (`koof-<name>`).
    pub name: &'static str,
    /// Default API base URL (first `servers[].url`); `PROVIDER_BASE_URL` overrides it.
    pub base_url: &'static str,
}

type ControllerFn = fn(Arc<ControllerContext>) -> BoxFuture<'static, ()>;

pub struct App {
    provider: &'static ProviderInfo,
    controllers: Vec<(&'static str, ControllerFn)>,
}

impl App {
    #[must_use]
    pub fn new(provider: &'static ProviderInfo) -> Self {
        Self {
            provider,
            controllers: Vec::new(),
        }
    }

    /// Applies a registration function, e.g. a provider's generated
    /// `controllers` (all its resources).
    #[must_use]
    pub fn with(self, register: fn(Self) -> Self) -> Self {
        register(self)
    }

    /// Adds the controller for `R`.
    #[must_use]
    pub fn controller<R: ManagedResource>(mut self) -> Self {
        self.controllers
            .push((R::descriptor().kind, |context| Box::pin(run::<R>(context))));
        self
    }

    /// Initialises logging (`RUST_LOG`, default `info`) and TLS, connects to
    /// Kubernetes, serves `/metrics`, `/healthz` and `/readyz` on `METRICS_ADDR`
    /// (default `0.0.0.0:8080`) and runs all controllers until a shutdown signal.
    ///
    /// # Errors
    ///
    /// Returns an error when the Kubernetes client cannot be created or the metrics
    /// address cannot be bound.
    pub async fn run(self) -> Result<(), AppError> {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();

        let base_url = std::env::var("PROVIDER_BASE_URL")
            .unwrap_or_else(|_| self.provider.base_url.to_owned());
        let metrics_addr =
            std::env::var("METRICS_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
        let kube_client = kube::Client::try_default().await?;
        let reporter = Reporter {
            controller: format!("koof-{}", self.provider.name),
            instance: std::env::var("POD_NAME").ok(),
        };
        let metrics = Arc::new(Metrics::new());
        let context = Arc::new(ControllerContext::new(
            kube_client.clone(),
            HttpClient::new(reqwest::Client::new(), base_url.clone()).with_metrics(metrics.clone()),
            Recorder::new(kube_client, reporter),
            metrics,
        ));

        let kinds: Vec<&'static str> = self.controllers.iter().map(|(kind, _)| *kind).collect();
        let listener = tokio::net::TcpListener::bind(&metrics_addr)
            .await
            .map_err(|source| AppError::Bind {
                address: metrics_addr.clone(),
                source,
            })?;
        let server = axum::serve(listener, router(context.clone(), kinds.clone()));

        tracing::info!(
            provider = self.provider.name,
            %base_url,
            %metrics_addr,
            controllers = ?kinds,
            "starting operator"
        );
        let controllers = futures::future::join_all(
            self.controllers
                .iter()
                .map(|(_, controller)| controller(context.clone())),
        );
        // Controllers stop on SIGINT / SIGTERM; the server stops with them.
        tokio::select! {
            _ = controllers => {}
            result = server => {
                if let Err(error) = result {
                    tracing::error!(%error, "metrics server failed");
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("cannot connect to Kubernetes: {0}")]
    Kubernetes(#[from] kube::Error),

    #[error("cannot listen on {address}: {source}")]
    Bind {
        address: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Clone)]
struct ServerState {
    context: Arc<ControllerContext>,
    kinds: Arc<[&'static str]>,
}

fn router(context: Arc<ControllerContext>, kinds: Vec<&'static str>) -> axum::Router {
    use axum::{extract::State, http::StatusCode, routing::get};

    async fn metrics(State(state): State<ServerState>) -> (StatusCode, String) {
        match state.context.metrics.encode() {
            Ok(body) => (StatusCode::OK, body),
            Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        }
    }
    async fn readyz(State(state): State<ServerState>) -> (StatusCode, &'static str) {
        if state.context.synced.all(&state.kinds) {
            (StatusCode::OK, "ok")
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, "initial sync in progress")
        }
    }

    axum::Router::new()
        .route("/metrics", get(metrics))
        .route("/healthz", get(|| async { "ok" }))
        .route("/readyz", get(readyz))
        .with_state(ServerState {
            context,
            kinds: kinds.into(),
        })
}
