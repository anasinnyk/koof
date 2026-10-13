//! In-memory cloudflare API for end-to-end tests (see `koof::fake`).

#[tokio::main]
async fn main() -> std::io::Result<()> {
    cloudflare_provider::generated::fake().run().await
}
