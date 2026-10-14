pub mod operations;
pub mod types;
pub mod access_application;
pub mod account;
pub mod custom_hostname;
pub mod d1_database;
pub mod dns_record;
pub mod email_routing_rule;
pub mod gateway_list;
pub mod gateway_rule;
pub mod healthcheck;
pub mod hyperdrive;
pub mod ip_list;
pub mod kv_namespace;
pub mod lb_monitor;
pub mod lb_pool;
pub mod load_balancer;
pub mod logpush_job;
pub mod notification_policy;
pub mod page_rule;
pub mod pages_project;
pub mod queue;
pub mod r2_bucket;
pub mod spectrum_application;
pub mod tunnel;
pub mod tunnel_cidr_route;
pub mod tunnel_configuration;
pub mod tunnel_hostname_route;
pub mod turnstile_widget;
pub mod vectorize_index;
pub mod waiting_room;
pub mod worker_route;
pub mod zone;
pub mod zone_ruleset;
fn http(http: koof::config::HttpConfig) -> koof::config::HttpConfig {
    http
}
pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
    name: "cloudflare",
    base_url: "https://api.cloudflare.com/client/v4",
    http,
};
/// Registers the controllers of every resource of this provider.
#[must_use]
pub fn controllers(app: koof::App) -> koof::App {
    app.controller::<access_application::AccessApplication>()
        .controller::<account::Account>()
        .controller::<custom_hostname::CustomHostname>()
        .controller::<d1_database::D1Database>()
        .controller::<dns_record::DNSRecord>()
        .controller::<email_routing_rule::EmailRoutingRule>()
        .controller::<gateway_list::GatewayList>()
        .controller::<gateway_rule::GatewayRule>()
        .controller::<healthcheck::Healthcheck>()
        .controller::<hyperdrive::Hyperdrive>()
        .controller::<ip_list::IPList>()
        .controller::<kv_namespace::KVNamespace>()
        .controller::<lb_monitor::LoadBalancerMonitor>()
        .controller::<lb_pool::LoadBalancerPool>()
        .controller::<load_balancer::LoadBalancer>()
        .controller::<logpush_job::LogpushJob>()
        .controller::<notification_policy::NotificationPolicy>()
        .controller::<page_rule::PageRule>()
        .controller::<pages_project::PagesProject>()
        .controller::<queue::Queue>()
        .controller::<r2_bucket::R2Bucket>()
        .controller::<spectrum_application::SpectrumApplication>()
        .controller::<tunnel::Tunnel>()
        .controller::<tunnel_cidr_route::TunnelCidrRoute>()
        .controller::<tunnel_configuration::TunnelConfiguration>()
        .controller::<tunnel_hostname_route::TunnelHostnameRoute>()
        .controller::<turnstile_widget::TurnstileWidget>()
        .controller::<vectorize_index::VectorizeIndex>()
        .controller::<waiting_room::WaitingRoom>()
        .controller::<worker_route::WorkerRoute>()
        .controller::<zone::Zone>()
        .controller::<zone_ruleset::Ruleset>()
}
#[must_use]
pub fn fake() -> koof::fake::Fake {
    koof::fake::Fake::new()
        .resource::<access_application::AccessApplication>()
        .resource::<account::Account>()
        .resource::<custom_hostname::CustomHostname>()
        .resource::<d1_database::D1Database>()
        .resource::<dns_record::DNSRecord>()
        .resource::<email_routing_rule::EmailRoutingRule>()
        .resource::<gateway_list::GatewayList>()
        .resource::<gateway_rule::GatewayRule>()
        .resource::<healthcheck::Healthcheck>()
        .resource::<hyperdrive::Hyperdrive>()
        .resource::<ip_list::IPList>()
        .resource::<kv_namespace::KVNamespace>()
        .resource::<lb_monitor::LoadBalancerMonitor>()
        .resource::<lb_pool::LoadBalancerPool>()
        .resource::<load_balancer::LoadBalancer>()
        .resource::<logpush_job::LogpushJob>()
        .resource::<notification_policy::NotificationPolicy>()
        .resource::<page_rule::PageRule>()
        .resource::<pages_project::PagesProject>()
        .resource::<queue::Queue>()
        .resource::<r2_bucket::R2Bucket>()
        .resource::<spectrum_application::SpectrumApplication>()
        .resource::<tunnel::Tunnel>()
        .resource::<tunnel_cidr_route::TunnelCidrRoute>()
        .resource::<tunnel_configuration::TunnelConfiguration>()
        .resource::<tunnel_hostname_route::TunnelHostnameRoute>()
        .resource::<turnstile_widget::TurnstileWidget>()
        .resource::<vectorize_index::VectorizeIndex>()
        .resource::<waiting_room::WaitingRoom>()
        .resource::<worker_route::WorkerRoute>()
        .resource::<zone::Zone>()
        .resource::<zone_ruleset::Ruleset>()
}
/// `CustomResourceDefinition`s of every resource of this provider.
#[must_use]
pub fn crds() -> Vec<
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
> {
    vec![
        < access_application::AccessApplication as kube::CustomResourceExt > ::crd(), <
        account::Account as kube::CustomResourceExt > ::crd(), <
        custom_hostname::CustomHostname as kube::CustomResourceExt > ::crd(), <
        d1_database::D1Database as kube::CustomResourceExt > ::crd(), <
        dns_record::DNSRecord as kube::CustomResourceExt > ::crd(), <
        email_routing_rule::EmailRoutingRule as kube::CustomResourceExt > ::crd(), <
        gateway_list::GatewayList as kube::CustomResourceExt > ::crd(), <
        gateway_rule::GatewayRule as kube::CustomResourceExt > ::crd(), <
        healthcheck::Healthcheck as kube::CustomResourceExt > ::crd(), <
        hyperdrive::Hyperdrive as kube::CustomResourceExt > ::crd(), < ip_list::IPList as
        kube::CustomResourceExt > ::crd(), < kv_namespace::KVNamespace as
        kube::CustomResourceExt > ::crd(), < lb_monitor::LoadBalancerMonitor as
        kube::CustomResourceExt > ::crd(), < lb_pool::LoadBalancerPool as
        kube::CustomResourceExt > ::crd(), < load_balancer::LoadBalancer as
        kube::CustomResourceExt > ::crd(), < logpush_job::LogpushJob as
        kube::CustomResourceExt > ::crd(), < notification_policy::NotificationPolicy as
        kube::CustomResourceExt > ::crd(), < page_rule::PageRule as
        kube::CustomResourceExt > ::crd(), < pages_project::PagesProject as
        kube::CustomResourceExt > ::crd(), < queue::Queue as kube::CustomResourceExt >
        ::crd(), < r2_bucket::R2Bucket as kube::CustomResourceExt > ::crd(), <
        spectrum_application::SpectrumApplication as kube::CustomResourceExt > ::crd(), <
        tunnel::Tunnel as kube::CustomResourceExt > ::crd(), <
        tunnel_cidr_route::TunnelCidrRoute as kube::CustomResourceExt > ::crd(), <
        tunnel_configuration::TunnelConfiguration as kube::CustomResourceExt > ::crd(), <
        tunnel_hostname_route::TunnelHostnameRoute as kube::CustomResourceExt > ::crd(),
        < turnstile_widget::TurnstileWidget as kube::CustomResourceExt > ::crd(), <
        vectorize_index::VectorizeIndex as kube::CustomResourceExt > ::crd(), <
        waiting_room::WaitingRoom as kube::CustomResourceExt > ::crd(), <
        worker_route::WorkerRoute as kube::CustomResourceExt > ::crd(), < zone::Zone as
        kube::CustomResourceExt > ::crd(), < zone_ruleset::Ruleset as
        kube::CustomResourceExt > ::crd()
    ]
}
