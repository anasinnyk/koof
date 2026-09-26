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
    http::HttpClient,
    reconciler::{ControllerContext, run},
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

    /// Adds the controller for `R`.
    #[must_use]
    pub fn controller<R: ManagedResource>(mut self) -> Self {
        self.controllers
            .push((R::descriptor().kind, |context| Box::pin(run::<R>(context))));
        self
    }

    /// Initialises logging (`RUST_LOG`, default `info`) and TLS, connects to
    /// Kubernetes and runs all controllers until a shutdown signal.
    ///
    /// # Errors
    ///
    /// Returns an error when the Kubernetes client cannot be created.
    pub async fn run(self) -> Result<(), kube::Error> {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();

        let base_url = std::env::var("PROVIDER_BASE_URL")
            .unwrap_or_else(|_| self.provider.base_url.to_owned());
        let kube_client = kube::Client::try_default().await?;
        let reporter = Reporter {
            controller: format!("koof-{}", self.provider.name),
            instance: std::env::var("POD_NAME").ok(),
        };
        let context = Arc::new(ControllerContext::new(
            kube_client.clone(),
            HttpClient::new(reqwest::Client::new(), base_url.clone()),
            Recorder::new(kube_client, reporter),
        ));

        tracing::info!(
            provider = self.provider.name,
            %base_url,
            controllers = ?self.controllers.iter().map(|(kind, _)| *kind).collect::<Vec<_>>(),
            "starting operator"
        );
        futures::future::join_all(
            self.controllers
                .iter()
                .map(|(_, controller)| controller(context.clone())),
        )
        .await;
        Ok(())
    }
}
