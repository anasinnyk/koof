#[tokio::main]
async fn main() -> std::io::Result<()> {
    cloudflare_provider::generated::fake().run().await
}
