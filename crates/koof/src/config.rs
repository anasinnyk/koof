use crate::app::ProviderInfo;
use std::{fmt, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpDefaults {
    pub timeout: Option<Duration>,
    pub connect_timeout: Option<Duration>,
    pub pool_idle_timeout: Option<Duration>,
    pub pool_max_idle_per_host: Option<usize>,
    pub user_agent: Option<&'static str>,
}

impl HttpDefaults {
    pub const NONE: Self = Self {
        timeout: None,
        connect_timeout: None,
        pool_idle_timeout: None,
        pool_max_idle_per_host: None,
        user_agent: None,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpConfig {
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub pool_idle_timeout: Duration,
    pub pool_max_idle_per_host: usize,
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
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Default,
    Provider,
    Environment,
}

impl fmt::Display for Source {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Default => "default",
            Self::Provider => "provider",
            Self::Environment => "environment",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub base_url: String,
    pub metrics_addr: String,
    pub pod_name: Option<String>,
    pub http: HttpConfig,
    pub sources: Vec<(&'static str, Source)>,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid {variable}={value:?}: {reason}")]
pub struct ConfigError {
    pub variable: &'static str,
    pub value: String,
    pub reason: String,
}

impl Config {
    /// # Errors
    ///
    /// Returns [`ConfigError`] when a `KOOF_*` variable cannot be parsed.
    pub fn from_env(provider: &ProviderInfo) -> Result<Self, ConfigError> {
        Self::resolve(provider, |name| std::env::var(name).ok())
    }

    /// # Errors
    ///
    /// Returns [`ConfigError`] when a variable cannot be parsed.
    pub fn resolve(
        provider: &ProviderInfo,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, ConfigError> {
        let defaults = &provider.http;
        let mut layers = Layers {
            env,
            sources: Vec::new(),
        };
        let base_url = layers.value(
            "KOOF_BASE_URL",
            String::new(),
            Some(provider.base_url.to_owned()),
            parse_string,
        )?;
        let metrics_addr = layers.value(
            "KOOF_METRICS_ADDR",
            "0.0.0.0:8080".to_owned(),
            None,
            parse_string,
        )?;
        let pod_name = layers.optional("KOOF_POD_NAME");
        let http = HttpConfig {
            timeout: layers.value(
                "KOOF_HTTP_TIMEOUT",
                Duration::from_secs(30),
                defaults.timeout,
                parse_duration,
            )?,
            connect_timeout: layers.value(
                "KOOF_HTTP_CONNECT_TIMEOUT",
                Duration::from_secs(10),
                defaults.connect_timeout,
                parse_duration,
            )?,
            pool_idle_timeout: layers.value(
                "KOOF_HTTP_POOL_IDLE_TIMEOUT",
                Duration::from_secs(90),
                defaults.pool_idle_timeout,
                parse_duration,
            )?,
            pool_max_idle_per_host: layers.value(
                "KOOF_HTTP_POOL_MAX_IDLE_PER_HOST",
                16,
                defaults.pool_max_idle_per_host,
                |value| value.parse::<usize>().map_err(|error| error.to_string()),
            )?,
            user_agent: layers.value(
                "KOOF_HTTP_USER_AGENT",
                format!("koof-{}/{}", provider.name, env!("CARGO_PKG_VERSION")),
                defaults.user_agent.map(str::to_owned),
                parse_string,
            )?,
        };
        Ok(Self {
            base_url,
            metrics_addr,
            pod_name,
            http,
            sources: layers.sources,
        })
    }

    #[must_use]
    pub fn source(&self, variable: &str) -> Option<Source> {
        self.sources
            .iter()
            .find(|(name, _)| *name == variable)
            .map(|(_, source)| *source)
    }
}

struct Layers<E> {
    env: E,
    sources: Vec<(&'static str, Source)>,
}

impl<E: Fn(&str) -> Option<String>> Layers<E> {
    fn value<T>(
        &mut self,
        variable: &'static str,
        default: T,
        provider: Option<T>,
        parse: impl Fn(&str) -> Result<T, String>,
    ) -> Result<T, ConfigError> {
        let (value, source) = match (self.env)(variable) {
            Some(value) => (
                parse(value.trim()).map_err(|reason| ConfigError {
                    variable,
                    value,
                    reason,
                })?,
                Source::Environment,
            ),
            None => match provider {
                Some(value) => (value, Source::Provider),
                None => (default, Source::Default),
            },
        };
        self.sources.push((variable, source));
        Ok(value)
    }

    fn optional(&mut self, variable: &'static str) -> Option<String> {
        let value = (self.env)(variable).filter(|value| !value.is_empty());
        if value.is_some() {
            self.sources.push((variable, Source::Environment));
        }
        value
    }
}

fn parse_string(value: &str) -> Result<String, String> {
    if value.is_empty() {
        Err("must not be empty".to_owned())
    } else {
        Ok(value.to_owned())
    }
}

/// # Errors
///
/// Returns a message when `value` is not a duration or is negative.
pub fn parse_duration(value: &str) -> Result<Duration, String> {
    let duration: jiff::SignedDuration = value
        .parse()
        .map_err(|error| format!("expected a duration like 30s or 1m30s: {error}"))?;
    Duration::try_from(duration).map_err(|_| "must not be negative".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    const PROVIDER: ProviderInfo = ProviderInfo {
        name: "example",
        base_url: "https://api.example.com",
        http: HttpDefaults {
            timeout: Some(Duration::from_mins(1)),
            ..HttpDefaults::NONE
        },
    };

    fn resolve(variables: &[(&str, &str)]) -> Result<Config, ConfigError> {
        let variables: BTreeMap<String, String> = variables
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect();
        Config::resolve(&PROVIDER, |name| variables.get(name).cloned())
    }

    #[test]
    fn environment_overrides_provider_overrides_default() -> Result<(), ConfigError> {
        let config = resolve(&[])?;
        assert_eq!(config.http.timeout, Duration::from_mins(1));
        assert_eq!(config.source("KOOF_HTTP_TIMEOUT"), Some(Source::Provider));
        assert_eq!(config.http.connect_timeout, Duration::from_secs(10));
        assert_eq!(config.source("KOOF_HTTP_CONNECT_TIMEOUT"), Some(Source::Default));
        assert_eq!(config.base_url, "https://api.example.com");
        assert_eq!(config.http.user_agent, format!("koof-example/{}", env!("CARGO_PKG_VERSION")));

        let config = resolve(&[
            ("KOOF_HTTP_TIMEOUT", "1m30s"),
            ("KOOF_BASE_URL", "http://127.0.0.1:8081"),
        ])?;
        assert_eq!(config.http.timeout, Duration::from_secs(90));
        assert_eq!(config.source("KOOF_HTTP_TIMEOUT"), Some(Source::Environment));
        assert_eq!(config.base_url, "http://127.0.0.1:8081");
        Ok(())
    }

    #[test]
    fn durations_accept_friendly_units() {
        assert_eq!(parse_duration("500ms"), Ok(Duration::from_millis(500)));
        assert_eq!(parse_duration("2m"), Ok(Duration::from_mins(2)));
        assert!(parse_duration("-5s").is_err());
        assert!(parse_duration("soon").is_err());
    }

    #[test]
    fn invalid_variable_is_an_error() {
        let error = resolve(&[("KOOF_HTTP_POOL_MAX_IDLE_PER_HOST", "many")])
            .err()
            .map(|error| error.variable);
        assert_eq!(error, Some("KOOF_HTTP_POOL_MAX_IDLE_PER_HOST"));
    }
}
