use cloudflare_provider::generated::{
    PROVIDER, account::Account, dns_record::DNSRecord, pages_project::PagesProject, zone::Zone,
    zone_ruleset::Ruleset,
};

#[tokio::main]
async fn main() -> Result<(), koof::AppError> {
    koof::App::new(&PROVIDER)
        .controller::<Account>()
        .controller::<Zone>()
        .controller::<DNSRecord>()
        .controller::<Ruleset>()
        .controller::<PagesProject>()
        .run()
        .await
}
