pub mod operations;
pub mod types;
pub mod cloudflare;
pub mod loadbalancing;
pub mod rulesets;
pub mod storage;
pub mod workers;
pub mod zerotrust;
pub mod zones;
fn http(http: koof::config::HttpConfig) -> koof::config::HttpConfig {
    http
}
pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo::new(
        "cloudflare",
        "https://api.cloudflare.com/client/v4",
    )
    .with_http(http);
/// Registers the controllers of every resource of this provider.
#[must_use]
pub fn controllers(app: koof::App) -> koof::App {
    app.controller::<zerotrust::v1alpha1::access_application::AccessApplication>()
        .controller::<cloudflare::v1alpha1::account::Account>()
        .controller::<cloudflare::v1alpha1::cluster_account::ClusterAccount>()
        .controller::<zones::v1alpha1::custom_hostname::CustomHostname>()
        .controller::<storage::v1alpha1::d1_database::D1Database>()
        .controller::<zones::v1alpha1::dns_record::DNSRecord>()
        .controller::<zones::v1alpha1::email_routing_rule::EmailRoutingRule>()
        .controller::<zerotrust::v1alpha1::gateway_list::GatewayList>()
        .controller::<zerotrust::v1alpha1::gateway_rule::GatewayRule>()
        .controller::<zones::v1alpha1::healthcheck::Healthcheck>()
        .controller::<storage::v1alpha1::hyperdrive::Hyperdrive>()
        .controller::<cloudflare::v1alpha1::ip_list::IPList>()
        .controller::<storage::v1alpha1::kv_namespace::KVNamespace>()
        .controller::<loadbalancing::v1alpha1::lb_monitor::LoadBalancerMonitor>()
        .controller::<loadbalancing::v1alpha1::lb_pool::LoadBalancerPool>()
        .controller::<loadbalancing::v1alpha1::load_balancer::LoadBalancer>()
        .controller::<zones::v1alpha1::logpush_job::LogpushJob>()
        .controller::<cloudflare::v1alpha1::notification_policy::NotificationPolicy>()
        .controller::<zones::v1alpha1::page_rule::PageRule>()
        .controller::<cloudflare::v1alpha1::pages_project::PagesProject>()
        .controller::<storage::v1alpha1::queue::Queue>()
        .controller::<storage::v1alpha1::r2_bucket::R2Bucket>()
        .controller::<zones::v1alpha1::spectrum_application::SpectrumApplication>()
        .controller::<zerotrust::v1alpha1::tunnel::Tunnel>()
        .controller::<zerotrust::v1alpha1::tunnel_cidr_route::TunnelCidrRoute>()
        .controller::<zerotrust::v1alpha1::tunnel_configuration::TunnelConfiguration>()
        .controller::<zerotrust::v1alpha1::tunnel_hostname_route::TunnelHostnameRoute>()
        .controller::<cloudflare::v1alpha1::turnstile_widget::TurnstileWidget>()
        .controller::<storage::v1alpha1::vectorize_index::VectorizeIndex>()
        .controller::<zones::v1alpha1::waiting_room::WaitingRoom>()
        .controller::<workers::v1alpha1::worker_route::WorkerRoute>()
        .controller::<cloudflare::v1alpha1::zone::Zone>()
        .controller::<rulesets::v1alpha1::zone_ruleset::Ruleset>()
}
#[cfg(feature = "fake")]
#[must_use]
pub fn fake() -> koof::fake::Fake {
    koof::fake::Fake::new()
        .resource::<zerotrust::v1alpha1::access_application::AccessApplication>()
        .resource::<cloudflare::v1alpha1::account::Account>()
        .resource::<cloudflare::v1alpha1::cluster_account::ClusterAccount>()
        .resource::<zones::v1alpha1::custom_hostname::CustomHostname>()
        .resource::<storage::v1alpha1::d1_database::D1Database>()
        .resource::<zones::v1alpha1::dns_record::DNSRecord>()
        .resource::<zones::v1alpha1::email_routing_rule::EmailRoutingRule>()
        .resource::<zerotrust::v1alpha1::gateway_list::GatewayList>()
        .resource::<zerotrust::v1alpha1::gateway_rule::GatewayRule>()
        .resource::<zones::v1alpha1::healthcheck::Healthcheck>()
        .resource::<storage::v1alpha1::hyperdrive::Hyperdrive>()
        .resource::<cloudflare::v1alpha1::ip_list::IPList>()
        .resource::<storage::v1alpha1::kv_namespace::KVNamespace>()
        .resource::<loadbalancing::v1alpha1::lb_monitor::LoadBalancerMonitor>()
        .resource::<loadbalancing::v1alpha1::lb_pool::LoadBalancerPool>()
        .resource::<loadbalancing::v1alpha1::load_balancer::LoadBalancer>()
        .resource::<zones::v1alpha1::logpush_job::LogpushJob>()
        .resource::<cloudflare::v1alpha1::notification_policy::NotificationPolicy>()
        .resource::<zones::v1alpha1::page_rule::PageRule>()
        .resource::<cloudflare::v1alpha1::pages_project::PagesProject>()
        .resource::<storage::v1alpha1::queue::Queue>()
        .resource::<storage::v1alpha1::r2_bucket::R2Bucket>()
        .resource::<zones::v1alpha1::spectrum_application::SpectrumApplication>()
        .resource::<zerotrust::v1alpha1::tunnel::Tunnel>()
        .resource::<zerotrust::v1alpha1::tunnel_cidr_route::TunnelCidrRoute>()
        .resource::<zerotrust::v1alpha1::tunnel_configuration::TunnelConfiguration>()
        .resource::<zerotrust::v1alpha1::tunnel_hostname_route::TunnelHostnameRoute>()
        .resource::<cloudflare::v1alpha1::turnstile_widget::TurnstileWidget>()
        .resource::<storage::v1alpha1::vectorize_index::VectorizeIndex>()
        .resource::<zones::v1alpha1::waiting_room::WaitingRoom>()
        .resource::<workers::v1alpha1::worker_route::WorkerRoute>()
        .resource::<cloudflare::v1alpha1::zone::Zone>()
        .resource::<rulesets::v1alpha1::zone_ruleset::Ruleset>()
}
/// `CustomResourceDefinition`s of every resource of this provider.
#[must_use]
pub fn crds() -> Vec<
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
> {
    vec![
        < zerotrust::v1alpha1::access_application::AccessApplication as
        kube::CustomResourceExt > ::crd(), < cloudflare::v1alpha1::account::Account as
        kube::CustomResourceExt > ::crd(), <
        cloudflare::v1alpha1::cluster_account::ClusterAccount as kube::CustomResourceExt
        > ::crd(), < zones::v1alpha1::custom_hostname::CustomHostname as
        kube::CustomResourceExt > ::crd(), < storage::v1alpha1::d1_database::D1Database
        as kube::CustomResourceExt > ::crd(), < zones::v1alpha1::dns_record::DNSRecord as
        kube::CustomResourceExt > ::crd(), <
        zones::v1alpha1::email_routing_rule::EmailRoutingRule as kube::CustomResourceExt
        > ::crd(), < zerotrust::v1alpha1::gateway_list::GatewayList as
        kube::CustomResourceExt > ::crd(), <
        zerotrust::v1alpha1::gateway_rule::GatewayRule as kube::CustomResourceExt >
        ::crd(), < zones::v1alpha1::healthcheck::Healthcheck as kube::CustomResourceExt >
        ::crd(), < storage::v1alpha1::hyperdrive::Hyperdrive as kube::CustomResourceExt >
        ::crd(), < cloudflare::v1alpha1::ip_list::IPList as kube::CustomResourceExt >
        ::crd(), < storage::v1alpha1::kv_namespace::KVNamespace as
        kube::CustomResourceExt > ::crd(), <
        loadbalancing::v1alpha1::lb_monitor::LoadBalancerMonitor as
        kube::CustomResourceExt > ::crd(), <
        loadbalancing::v1alpha1::lb_pool::LoadBalancerPool as kube::CustomResourceExt >
        ::crd(), < loadbalancing::v1alpha1::load_balancer::LoadBalancer as
        kube::CustomResourceExt > ::crd(), < zones::v1alpha1::logpush_job::LogpushJob as
        kube::CustomResourceExt > ::crd(), <
        cloudflare::v1alpha1::notification_policy::NotificationPolicy as
        kube::CustomResourceExt > ::crd(), < zones::v1alpha1::page_rule::PageRule as
        kube::CustomResourceExt > ::crd(), <
        cloudflare::v1alpha1::pages_project::PagesProject as kube::CustomResourceExt >
        ::crd(), < storage::v1alpha1::queue::Queue as kube::CustomResourceExt > ::crd(),
        < storage::v1alpha1::r2_bucket::R2Bucket as kube::CustomResourceExt > ::crd(), <
        zones::v1alpha1::spectrum_application::SpectrumApplication as
        kube::CustomResourceExt > ::crd(), < zerotrust::v1alpha1::tunnel::Tunnel as
        kube::CustomResourceExt > ::crd(), <
        zerotrust::v1alpha1::tunnel_cidr_route::TunnelCidrRoute as
        kube::CustomResourceExt > ::crd(), <
        zerotrust::v1alpha1::tunnel_configuration::TunnelConfiguration as
        kube::CustomResourceExt > ::crd(), <
        zerotrust::v1alpha1::tunnel_hostname_route::TunnelHostnameRoute as
        kube::CustomResourceExt > ::crd(), <
        cloudflare::v1alpha1::turnstile_widget::TurnstileWidget as
        kube::CustomResourceExt > ::crd(), <
        storage::v1alpha1::vectorize_index::VectorizeIndex as kube::CustomResourceExt >
        ::crd(), < zones::v1alpha1::waiting_room::WaitingRoom as kube::CustomResourceExt
        > ::crd(), < workers::v1alpha1::worker_route::WorkerRoute as
        kube::CustomResourceExt > ::crd(), < cloudflare::v1alpha1::zone::Zone as
        kube::CustomResourceExt > ::crd(), < rulesets::v1alpha1::zone_ruleset::Ruleset as
        kube::CustomResourceExt > ::crd()
    ]
}
/// Documentation of every resource of this provider.
#[must_use]
pub fn docs() -> Vec<koof::docs::ResourceDoc> {
    vec![
        koof::docs::ResourceDoc::of:: <
        zerotrust::v1alpha1::access_application::AccessApplication > (),
        koof::docs::ResourceDoc::of:: < cloudflare::v1alpha1::account::Account > (),
        koof::docs::ResourceDoc::of:: <
        cloudflare::v1alpha1::cluster_account::ClusterAccount > (),
        koof::docs::ResourceDoc::of:: < zones::v1alpha1::custom_hostname::CustomHostname
        > (), koof::docs::ResourceDoc::of:: < storage::v1alpha1::d1_database::D1Database
        > (), koof::docs::ResourceDoc::of:: < zones::v1alpha1::dns_record::DNSRecord >
        (), koof::docs::ResourceDoc::of:: <
        zones::v1alpha1::email_routing_rule::EmailRoutingRule > (),
        koof::docs::ResourceDoc::of:: < zerotrust::v1alpha1::gateway_list::GatewayList >
        (), koof::docs::ResourceDoc::of:: <
        zerotrust::v1alpha1::gateway_rule::GatewayRule > (),
        koof::docs::ResourceDoc::of:: < zones::v1alpha1::healthcheck::Healthcheck > (),
        koof::docs::ResourceDoc::of:: < storage::v1alpha1::hyperdrive::Hyperdrive > (),
        koof::docs::ResourceDoc::of:: < cloudflare::v1alpha1::ip_list::IPList > (),
        koof::docs::ResourceDoc::of:: < storage::v1alpha1::kv_namespace::KVNamespace >
        (), koof::docs::ResourceDoc::of:: <
        loadbalancing::v1alpha1::lb_monitor::LoadBalancerMonitor > (),
        koof::docs::ResourceDoc::of:: <
        loadbalancing::v1alpha1::lb_pool::LoadBalancerPool > (),
        koof::docs::ResourceDoc::of:: <
        loadbalancing::v1alpha1::load_balancer::LoadBalancer > (),
        koof::docs::ResourceDoc::of:: < zones::v1alpha1::logpush_job::LogpushJob > (),
        koof::docs::ResourceDoc::of:: <
        cloudflare::v1alpha1::notification_policy::NotificationPolicy > (),
        koof::docs::ResourceDoc::of:: < zones::v1alpha1::page_rule::PageRule > (),
        koof::docs::ResourceDoc::of:: < cloudflare::v1alpha1::pages_project::PagesProject
        > (), koof::docs::ResourceDoc::of:: < storage::v1alpha1::queue::Queue > (),
        koof::docs::ResourceDoc::of:: < storage::v1alpha1::r2_bucket::R2Bucket > (),
        koof::docs::ResourceDoc::of:: <
        zones::v1alpha1::spectrum_application::SpectrumApplication > (),
        koof::docs::ResourceDoc::of:: < zerotrust::v1alpha1::tunnel::Tunnel > (),
        koof::docs::ResourceDoc::of:: <
        zerotrust::v1alpha1::tunnel_cidr_route::TunnelCidrRoute > (),
        koof::docs::ResourceDoc::of:: <
        zerotrust::v1alpha1::tunnel_configuration::TunnelConfiguration > (),
        koof::docs::ResourceDoc::of:: <
        zerotrust::v1alpha1::tunnel_hostname_route::TunnelHostnameRoute > (),
        koof::docs::ResourceDoc::of:: <
        cloudflare::v1alpha1::turnstile_widget::TurnstileWidget > (),
        koof::docs::ResourceDoc::of:: <
        storage::v1alpha1::vectorize_index::VectorizeIndex > (),
        koof::docs::ResourceDoc::of:: < zones::v1alpha1::waiting_room::WaitingRoom > (),
        koof::docs::ResourceDoc::of:: < workers::v1alpha1::worker_route::WorkerRoute >
        (), koof::docs::ResourceDoc::of:: < cloudflare::v1alpha1::zone::Zone > (),
        koof::docs::ResourceDoc::of:: < rulesets::v1alpha1::zone_ruleset::Ruleset > ()
    ]
}
