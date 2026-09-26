pub mod operations;
pub mod types;
pub mod account;
pub mod dns_record;
pub mod pages_project;
pub mod zone;
pub mod zone_ruleset;
pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
    name: "cloudflare",
    base_url: "https://api.cloudflare.com/client/v4",
};
