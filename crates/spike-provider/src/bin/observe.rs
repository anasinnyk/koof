//! Calls Account observe through the generic `send`.
//! Usage: CLOUDFLARE_API_TOKEN=... cargo run -p spike-provider --bin observe -- <account_id>

use spike_provider::operation::{AccountObserve, Auth, HttpClient};

#[tokio::main]
async fn main() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let account_id = std::env::args().nth(1).unwrap_or_else(|| "00000000000000000000000000000000".into());
    let token = std::env::var("CLOUDFLARE_API_TOKEN").unwrap_or_else(|_| "invalid".into());
    let client = HttpClient::new(reqwest::Client::new(), "https://api.cloudflare.com/client/v4");
    let auth = Auth::Bearer(token);
    println!("auth = {auth:?}");
    match client.send::<AccountObserve>(&auth, &[("account_id", &account_id)], None).await {
        Ok(Some(response)) => println!("observed: {:?}", response),
        Ok(None) => println!("not found"),
        Err(error) => println!("error: {error}"),
    }
}
