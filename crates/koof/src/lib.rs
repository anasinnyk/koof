#![doc = include_str!("../README.md")]

pub mod app;
pub mod config;
pub mod controller;
pub mod error;
pub mod external;
pub mod fake;
pub mod managed;
pub mod manifests;
pub mod metrics;
pub mod object;
pub mod reference;
pub mod resource;
pub mod retry;
pub mod schema;
pub mod status;

pub use app::{App, AppError, ProviderInfo};
pub use external::http;
pub use http::Operation;
pub use resource::{ManagedResource, ResourceDescriptor};
