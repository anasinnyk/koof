pub mod error;
pub mod http;
pub mod managed;
pub mod reconciler;
pub mod reference;
pub mod resolve;
pub mod resource;
pub mod schema;
pub mod status;

pub use http::Operation;
pub use resource::{ManagedResource, ResourceDescriptor};
