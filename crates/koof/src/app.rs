use crate::{
    config::{Config, HttpConfig},
    controller::{ControllerContext, run},
    http::HttpClient,
    metrics::Metrics,
    resource::ManagedResource,
};
use futures::future::BoxFuture;
use kube::runtime::events::{Recorder, Reporter};
use std::sync::Arc;

#[derive(Debug)]
pub struct ProviderInfo {
    pub name: &'static str,
    pub base_url: &'static str,
    pub http: fn(HttpConfig) -> HttpConfig,
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

    #[must_use]
    pub fn with(self, register: fn(Self) -> Self) -> Self {
        register(self)
    }

    #[must_use]
    pub fn controller<R: ManagedResource>(mut self) -> Self {
        self.controllers
            .push((R::descriptor().kind, |context| Box::pin(run::<R>(context))));
        self
    }

    /// Initialises logging (`RUST_LOG`, default `info`) and TLS, resolves the
    /// [`Config`], connects to Kubernetes, serves `/metrics`, `/healthz` and
    /// `/readyz` and runs all controllers until a shutdown signal.
    ///
    /// # Errors
    ///
    /// Returns an error when the configuration is invalid, the HTTP or Kubernetes
    /// client cannot be created, or the metrics address cannot be bound.
    pub async fn run(self) -> Result<(), AppError> {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();

        let config = Config::from_env(self.provider)?;
        tracing::info!(provider = self.provider.name, ?config, "configuration");
        let http = config.http.client().map_err(AppError::HttpClient)?;
        let kube_client = kube::Client::try_default().await?;
        let reporter = Reporter {
            controller: format!("koof-{}", self.provider.name),
            instance: config.pod_name.clone(),
        };
        let metrics = Arc::new(Metrics::new());
        let context = Arc::new(ControllerContext::new(
            kube_client.clone(),
            HttpClient::new(http, config.base_url.clone()).with_metrics(metrics.clone()),
            Recorder::new(kube_client, reporter),
            metrics,
        ));

        let kinds: Vec<&'static str> = self.controllers.iter().map(|(kind, _)| *kind).collect();
        let listener = tokio::net::TcpListener::bind(&config.metrics_addr)
            .await
            .map_err(|source| AppError::Bind {
                address: config.metrics_addr.clone(),
                source,
            })?;
        let server = axum::serve(listener, router(context.clone(), kinds.clone()));

        tracing::info!(controllers = ?kinds, "starting operator");
        let controllers = futures::future::join_all(
            self.controllers
                .iter()
                .map(|(_, controller)| controller(context.clone())),
        );
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
    #[error("invalid configuration: {0}")]
    Config(#[from] config::ConfigError),

    #[error("cannot create the HTTP client: {0}")]
    HttpClient(#[source] reqwest::Error),

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
