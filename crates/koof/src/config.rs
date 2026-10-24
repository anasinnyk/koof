use crate::app::ProviderInfo;
use reqwest::redirect::Policy;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
pub struct HttpConfig {
    #[default(Duration::from_secs(30))]
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
    #[default(Duration::from_secs(10))]
    #[serde(with = "humantime_serde")]
    pub connect_timeout: Duration,
    #[default(Duration::from_secs(90))]
    #[serde(with = "humantime_serde")]
    pub pool_idle_timeout: Duration,
    #[default(16)]
    pub pool_max_idle_per_host: usize,
    #[default(format!("koof/{}", env!("CARGO_PKG_VERSION")))]
    pub user_agent: String,
}

impl HttpConfig {
    /// # Errors
    ///
    /// Returns an error when the client cannot be built.
    pub fn client(&self) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .pool_idle_timeout(self.pool_idle_timeout)
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .user_agent(&self.user_agent)
            .redirect(Policy::none())
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
pub struct LeaderElectionConfig {
    pub enabled: bool,
    pub name: String,
    pub namespace: Option<String>,
    #[default(Duration::from_secs(15))]
    #[serde(with = "humantime_serde")]
    pub lease_duration: Duration,
    #[default(Duration::from_secs(5))]
    #[serde(with = "humantime_serde")]
    pub renew_interval: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub metrics_addr: String,
    pub pod_name: Option<String>,
    pub concurrency: u16,
    pub http: HttpConfig,
    pub leader_election: LeaderElectionConfig,
}

impl Config {
    #[must_use]
    pub fn defaults(provider: &ProviderInfo) -> Self {
        let http = HttpConfig {
            user_agent: format!("koof-{}/{}", provider.name, env!("CARGO_PKG_VERSION")),
            ..HttpConfig::default()
        };
        Self {
            base_url: provider.base_url.to_owned(),
            metrics_addr: "0.0.0.0:8080".to_owned(),
            pod_name: None,
            concurrency: 8,
            http: (provider.http)(http),
            leader_election: LeaderElectionConfig {
                name: format!("koof-{}", provider.name),
                ..LeaderElectionConfig::default()
            },
        }
    }

    /// # Errors
    ///
    /// Returns an error when a `KOOF_*` variable cannot be parsed.
    #[cfg(feature = "env")]
    pub fn from_env(provider: &ProviderInfo) -> Result<Self, config::ConfigError> {
        Self::load(provider, environment())
    }

    #[cfg(feature = "env")]
    fn load(
        provider: &ProviderInfo,
        environment: config::Environment,
    ) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Config::try_from(&Self::defaults(provider))?)
            .add_source(environment)
            .build()?
            .try_deserialize()
    }
}

#[cfg(feature = "env")]
fn environment() -> config::Environment {
    config::Environment::with_prefix("KOOF")
        .prefix_separator("_")
        .separator("__")
}

#[cfg(all(test, feature = "env"))]
mod tests {
    use super::*;

    #[test]
    fn leader_election_from_environment() -> Result<(), config::ConfigError> {
        static PROVIDER: ProviderInfo = ProviderInfo::new("test", "http://127.0.0.1");
        let variables = [
            ("KOOF_LEADER_ELECTION__ENABLED", "true"),
            ("KOOF_LEADER_ELECTION__NAMESPACE", "system"),
            ("KOOF_LEADER_ELECTION__LEASE_DURATION", "30s"),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect();
        let config = Config::load(&PROVIDER, environment().source(Some(variables)))?;
        assert_eq!(
            config.leader_election,
            LeaderElectionConfig {
                enabled: true,
                name: "koof-test".to_owned(),
                namespace: Some("system".to_owned()),
                lease_duration: Duration::from_secs(30),
                renew_interval: Duration::from_secs(5),
            }
        );
        Ok(())
    }
}
