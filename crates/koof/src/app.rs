use crate::{
    config::{Config, HttpConfig},
    controller::{ControllerContext, run},
    http::HttpClient,
    metrics::Metrics,
    resource::{ManagedResource, ResourceDescriptor},
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

type ControllerFn =
    fn(Arc<ControllerContext>, tokio::sync::watch::Receiver<bool>) -> BoxFuture<'static, ()>;

pub struct App {
    provider: &'static ProviderInfo,
    controllers: Vec<(&'static ResourceDescriptor, ControllerFn)>,
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
            .push((R::descriptor(), |context, shutdown| {
                Box::pin(run::<R>(context, shutdown))
            }));
        self
    }

    /// Initialises logging (`RUST_LOG`, default `info`, with the `subscriber`
    /// feature) and TLS, resolves the [`Config`] (from `KOOF_*` variables with the
    /// `env` feature), connects to Kubernetes and calls [`run_with`](Self::run_with)
    /// until `SIGTERM` or Ctrl-C.
    ///
    /// # Errors
    ///
    /// See [`run_with`](Self::run_with); also when the configuration is invalid or
    /// the Kubernetes client cannot be created.
    pub async fn run(self) -> Result<(), AppError> {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        #[cfg(feature = "subscriber")]
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();

        #[cfg(feature = "env")]
        let config = Config::from_env(self.provider)?;
        #[cfg(not(feature = "env"))]
        let config = Config::defaults(self.provider);
        let kube_client = kube::Client::try_default().await?;
        self.run_with(config, kube_client, shutdown_signal()).await
    }

    /// Serves `/metrics`, `/healthz` and `/readyz` and runs all controllers until
    /// `shutdown` completes, then lets running reconciles finish. Installs no global
    /// logger or TLS provider.
    ///
    /// # Errors
    ///
    /// Returns an error when the HTTP client cannot be created, the metrics address
    /// cannot be bound or the metrics server fails.
    pub async fn run_with(
        self,
        config: Config,
        kube_client: kube::Client,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), AppError> {
        tracing::info!(provider = self.provider.name, ?config, "configuration");
        let http = config.http.client().map_err(AppError::HttpClient)?;
        let reporter = Reporter {
            controller: format!("koof-{}", self.provider.name),
            instance: config.pod_name.clone(),
        };
        let metrics = Arc::new(Metrics::new());
        let descriptors: Vec<&'static ResourceDescriptor> = self
            .controllers
            .iter()
            .map(|(descriptor, _)| *descriptor)
            .collect();
        let context = Arc::new(ControllerContext::new(
            kube_client.clone(),
            HttpClient::new(http, config.base_url.clone()).with_metrics(metrics.clone()),
            Recorder::new(kube_client, reporter),
            metrics,
            descriptors.clone(),
        ));

        let kinds: Vec<&'static str> = descriptors
            .iter()
            .map(|descriptor| descriptor.kind)
            .collect();
        let listener = tokio::net::TcpListener::bind(&config.metrics_addr)
            .await
            .map_err(|source| AppError::Bind {
                address: config.metrics_addr.clone(),
                source,
            })?;

        let (stop, stopped) = tokio::sync::watch::channel(false);
        let stopping = {
            let stopped = stopped.clone();
            move || {
                let mut stopped = stopped.clone();
                async move {
                    let _ = stopped.wait_for(|stopped| *stopped).await;
                }
            }
        };
        let signal = {
            let stop = stop.clone();
            tokio::spawn(async move {
                shutdown.await;
                tracing::info!("shutting down");
                stop.send_replace(true);
            })
        };

        let metrics_server = async {
            let result = axum::serve(listener, router(context.clone(), kinds.clone()))
                .with_graceful_shutdown(stopping())
                .await;
            if let Err(error) = &result {
                tracing::error!(error = crate::error::chain(error), "metrics server failed");
            }
            stop.send_replace(true);
            result
        };

        tracing::info!(controllers = ?kinds, "starting operator");
        let controllers = futures::future::join_all(
            self.controllers
                .iter()
                .map(|(_, controller)| controller(context.clone(), stopped.clone())),
        );
        let (_, served) = tokio::join!(controllers, metrics_server);
        signal.abort();
        served.map_err(AppError::Serve)
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut terminate) = signal(SignalKind::terminate()) {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {}
                _ = terminate.recv() => {}
            }
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[cfg(feature = "env")]
    #[error("invalid configuration")]
    Config(#[from] config::ConfigError),

    #[error("cannot create the HTTP client")]
    HttpClient(#[source] reqwest::Error),

    #[error("cannot connect to Kubernetes")]
    Kubernetes(#[from] kube::Error),

    #[error("cannot listen on {address}")]
    Bind {
        address: String,
        #[source]
        source: std::io::Error,
    },

    #[error("metrics server failed")]
    Serve(#[source] std::io::Error),
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
