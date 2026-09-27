use sentry_provider::generated::{PROVIDER, controllers};

#[tokio::main]
async fn main() -> Result<(), koof::AppError> {
    koof::App::new(&PROVIDER).with(controllers).run().await
}
