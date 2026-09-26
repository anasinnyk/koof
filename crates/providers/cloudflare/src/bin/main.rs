use cloudflare_provider::generated::{PROVIDER, account::Account};

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    koof::App::new(&PROVIDER)
        .controller::<Account>()
        // .controller::<cloudflare_provider::generated::zone::Zone>()
        // .controller::<cloudflare_provider::generated::dns_record::DNSRecord>()
        // .controller::<cloudflare_provider::generated::zone_ruleset::Ruleset>()
        // .controller::<cloudflare_provider::generated::pages_project::PagesProject>()
        .run()
        .await
}
