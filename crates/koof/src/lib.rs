#![doc = include_str!("../README.md")]

pub mod app;
pub mod config;
pub mod controller;
pub mod docs;
pub mod error;
pub mod external;
#[cfg(feature = "fake")]
pub mod fake;
pub mod leader;
pub mod managed;
pub mod manifests;
pub mod metrics;
pub mod object;
pub mod reference;
pub mod resource;
pub mod retry;
pub mod schema;
pub mod status;
#[cfg(test)]
mod testing;

pub use app::{App, AppError, ProviderInfo};
pub use external::http;
pub use http::Operation;
pub use resource::{ManagedResource, ResourceDescriptor};
