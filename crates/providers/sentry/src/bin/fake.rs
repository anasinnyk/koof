//! In-memory sentry API for end-to-end tests (see `koof::fake`).

#[tokio::main]
async fn main() -> std::io::Result<()> {
    sentry_provider::generated::fake().run().await
}
