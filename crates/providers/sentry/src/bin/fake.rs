#[tokio::main]
async fn main() -> std::io::Result<()> {
    sentry_provider::generated::fake().run().await
}
