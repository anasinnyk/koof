//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
pub struct AccessApplicationCreate;
impl koof::Operation for AccessApplicationCreate {
    const NAME: &'static str = "access_application.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/access/apps";
    type Request = crate::generated::types::AccessAppRequest;
    type Response = crate::generated::types::AccessApplicationCreateResponse;
}
pub struct AccessApplicationDelete;
impl koof::Operation for AccessApplicationDelete {
    const NAME: &'static str = "access_application.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/access/apps/{app_id}";
    type Request = ();
    type Response = crate::generated::types::AccessIdResponse;
}
pub struct AccessApplicationObserve;
impl koof::Operation for AccessApplicationObserve {
    const NAME: &'static str = "access_application.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/access/apps/{app_id}";
    type Request = ();
    type Response = crate::generated::types::AccessSingleResponse8;
}
pub struct AccessApplicationUpdate;
impl koof::Operation for AccessApplicationUpdate {
    const NAME: &'static str = "access_application.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/access/apps/{app_id}";
    type Request = crate::generated::types::AccessAppRequest;
    type Response = crate::generated::types::AccessApplicationUpdateResponse;
}
pub struct AccountCreate;
impl koof::Operation for AccountCreate {
    const NAME: &'static str = "account.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts";
    type Request = crate::generated::types::IamCreateAccount;
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct AccountDelete;
impl koof::Operation for AccountDelete {
    const NAME: &'static str = "account.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = ();
    type Response = crate::generated::types::IamApiResponseSingleId;
}
pub struct AccountObserve;
impl koof::Operation for AccountObserve {
    const NAME: &'static str = "account.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = ();
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct AccountUpdate;
impl koof::Operation for AccountUpdate {
    const NAME: &'static str = "account.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = crate::generated::types::IamComponentsSchemasAccount;
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct CustomHostnameCreate;
impl koof::Operation for CustomHostnameCreate {
    const NAME: &'static str = "custom_hostname.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/custom_hostnames";
    type Request = crate::generated::types::CustomHostnameCreateRequest;
    type Response = crate::generated::types::TlsCertificatesAndHostnamesCustomHostnameResponseSingle;
}
pub struct CustomHostnameDelete;
impl koof::Operation for CustomHostnameDelete {
    const NAME: &'static str = "custom_hostname.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/custom_hostnames/{custom_hostname_id}";
    type Request = ();
    type Response = crate::generated::types::CustomHostnameDeleteResponse;
}
pub struct CustomHostnameObserve;
impl koof::Operation for CustomHostnameObserve {
    const NAME: &'static str = "custom_hostname.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/custom_hostnames/{custom_hostname_id}";
    type Request = ();
    type Response = crate::generated::types::TlsCertificatesAndHostnamesCustomHostnameResponseSingle;
}
pub struct CustomHostnameUpdate;
impl koof::Operation for CustomHostnameUpdate {
    const NAME: &'static str = "custom_hostname.update";
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/zones/{zone_id}/custom_hostnames/{custom_hostname_id}";
    type Request = crate::generated::types::CustomHostnameUpdateRequest;
    type Response = crate::generated::types::TlsCertificatesAndHostnamesCustomHostnameResponseSingle;
}
pub struct D1DatabaseCreate;
impl koof::Operation for D1DatabaseCreate {
    const NAME: &'static str = "d1_database.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/d1/database";
    type Request = crate::generated::types::D1DatabaseCreateRequest;
    type Response = crate::generated::types::D1DatabaseCreateResponse;
}
pub struct D1DatabaseDelete;
impl koof::Operation for D1DatabaseDelete {
    const NAME: &'static str = "d1_database.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/d1/database/{database_id}";
    type Request = ();
    type Response = crate::generated::types::D1DatabaseDeleteResponse;
}
pub struct D1DatabaseObserve;
impl koof::Operation for D1DatabaseObserve {
    const NAME: &'static str = "d1_database.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/d1/database/{database_id}";
    type Request = ();
    type Response = crate::generated::types::D1DatabaseObserveResponse;
}
pub struct D1DatabaseUpdate;
impl koof::Operation for D1DatabaseUpdate {
    const NAME: &'static str = "d1_database.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/d1/database/{database_id}";
    type Request = crate::generated::types::D1DatabaseUpdateRequestBody;
    type Response = crate::generated::types::D1DatabaseUpdateResponse;
}
pub struct DnsRecordCreate;
impl koof::Operation for DnsRecordCreate {
    const NAME: &'static str = "dns_record.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/dns_records";
    type Request = crate::generated::types::DnsRecordsDnsRecordPost;
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct DnsRecordDelete;
impl koof::Operation for DnsRecordDelete {
    const NAME: &'static str = "dns_record.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = ();
    type Response = crate::generated::types::DnsRecordDeleteResponse;
}
pub struct DnsRecordObserve;
impl koof::Operation for DnsRecordObserve {
    const NAME: &'static str = "dns_record.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = ();
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct DnsRecordUpdate;
impl koof::Operation for DnsRecordUpdate {
    const NAME: &'static str = "dns_record.update";
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = crate::generated::types::DnsRecordsDnsRecordPatch;
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct EmailRoutingRuleCreate;
impl koof::Operation for EmailRoutingRuleCreate {
    const NAME: &'static str = "email_routing_rule.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/email/routing/rules";
    type Request = crate::generated::types::EmailCreateRuleProperties;
    type Response = crate::generated::types::EmailRuleResponseSingle;
}
pub struct EmailRoutingRuleDelete;
impl koof::Operation for EmailRoutingRuleDelete {
    const NAME: &'static str = "email_routing_rule.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/email/routing/rules/{rule_identifier}";
    type Request = ();
    type Response = crate::generated::types::EmailRuleResponseSingle;
}
pub struct EmailRoutingRuleObserve;
impl koof::Operation for EmailRoutingRuleObserve {
    const NAME: &'static str = "email_routing_rule.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/email/routing/rules/{rule_identifier}";
    type Request = ();
    type Response = crate::generated::types::EmailRuleResponseSingle;
}
pub struct EmailRoutingRuleUpdate;
impl koof::Operation for EmailRoutingRuleUpdate {
    const NAME: &'static str = "email_routing_rule.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/email/routing/rules/{rule_identifier}";
    type Request = crate::generated::types::EmailUpdateRuleProperties;
    type Response = crate::generated::types::EmailRuleResponseSingle;
}
pub struct GatewayListCreate;
impl koof::Operation for GatewayListCreate {
    const NAME: &'static str = "gateway_list.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/gateway/lists";
    type Request = crate::generated::types::GatewayListCreateRequest;
    type Response = crate::generated::types::ZeroTrustGatewaySingleResponseWithListItems;
}
pub struct GatewayListDelete;
impl koof::Operation for GatewayListDelete {
    const NAME: &'static str = "gateway_list.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/gateway/lists/{list_id}";
    type Request = ();
    type Response = crate::generated::types::ZeroTrustGatewayEmptyResponse;
}
pub struct GatewayListObserve;
impl koof::Operation for GatewayListObserve {
    const NAME: &'static str = "gateway_list.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/gateway/lists/{list_id}";
    type Request = ();
    type Response = crate::generated::types::ZeroTrustGatewayListSingleResponse;
}
pub struct GatewayListUpdate;
impl koof::Operation for GatewayListUpdate {
    const NAME: &'static str = "gateway_list.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/gateway/lists/{list_id}";
    type Request = crate::generated::types::GatewayListUpdateRequest;
    type Response = crate::generated::types::ZeroTrustGatewayListSingleResponse;
}
pub struct GatewayRuleCreate;
impl koof::Operation for GatewayRuleCreate {
    const NAME: &'static str = "gateway_rule.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/gateway/rules";
    type Request = crate::generated::types::GatewayRuleCreateRequest;
    type Response = crate::generated::types::ZeroTrustGatewaySingleResponse4;
}
pub struct GatewayRuleDelete;
impl koof::Operation for GatewayRuleDelete {
    const NAME: &'static str = "gateway_rule.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/gateway/rules/{rule_id}";
    type Request = ();
    type Response = crate::generated::types::ZeroTrustGatewayEmptyResponse;
}
pub struct GatewayRuleObserve;
impl koof::Operation for GatewayRuleObserve {
    const NAME: &'static str = "gateway_rule.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/gateway/rules/{rule_id}";
    type Request = ();
    type Response = crate::generated::types::ZeroTrustGatewaySingleResponse4;
}
pub struct GatewayRuleUpdate;
impl koof::Operation for GatewayRuleUpdate {
    const NAME: &'static str = "gateway_rule.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/gateway/rules/{rule_id}";
    type Request = crate::generated::types::GatewayRuleUpdateRequest;
    type Response = crate::generated::types::ZeroTrustGatewaySingleResponse4;
}
pub struct HealthcheckCreate;
impl koof::Operation for HealthcheckCreate {
    const NAME: &'static str = "healthcheck.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/healthchecks";
    type Request = crate::generated::types::HealthchecksQueryHealthcheck;
    type Response = crate::generated::types::HealthchecksSingleResponse;
}
pub struct HealthcheckDelete;
impl koof::Operation for HealthcheckDelete {
    const NAME: &'static str = "healthcheck.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/healthchecks/{healthcheck_id}";
    type Request = ();
    type Response = crate::generated::types::HealthchecksIdResponse;
}
pub struct HealthcheckObserve;
impl koof::Operation for HealthcheckObserve {
    const NAME: &'static str = "healthcheck.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/healthchecks/{healthcheck_id}";
    type Request = ();
    type Response = crate::generated::types::HealthchecksSingleResponse;
}
pub struct HealthcheckUpdate;
impl koof::Operation for HealthcheckUpdate {
    const NAME: &'static str = "healthcheck.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/healthchecks/{healthcheck_id}";
    type Request = crate::generated::types::HealthchecksQueryHealthcheck;
    type Response = crate::generated::types::HealthchecksSingleResponse;
}
pub struct HyperdriveCreate;
impl koof::Operation for HyperdriveCreate {
    const NAME: &'static str = "hyperdrive.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/hyperdrive/configs";
    type Request = crate::generated::types::HyperdriveHyperdriveConfig;
    type Response = crate::generated::types::HyperdriveCreateResponse;
}
pub struct HyperdriveDelete;
impl koof::Operation for HyperdriveDelete {
    const NAME: &'static str = "hyperdrive.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/hyperdrive/configs/{hyperdrive_id}";
    type Request = ();
    type Response = crate::generated::types::HyperdriveDeleteResponse;
}
pub struct HyperdriveObserve;
impl koof::Operation for HyperdriveObserve {
    const NAME: &'static str = "hyperdrive.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/hyperdrive/configs/{hyperdrive_id}";
    type Request = ();
    type Response = crate::generated::types::HyperdriveObserveResponse;
}
pub struct HyperdriveUpdate;
impl koof::Operation for HyperdriveUpdate {
    const NAME: &'static str = "hyperdrive.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/hyperdrive/configs/{hyperdrive_id}";
    type Request = crate::generated::types::HyperdriveHyperdriveConfig;
    type Response = crate::generated::types::HyperdriveUpdateResponse;
}
pub struct IpListCreate;
impl koof::Operation for IpListCreate {
    const NAME: &'static str = "ip_list.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/rules/lists";
    type Request = crate::generated::types::IpListCreateRequest;
    type Response = crate::generated::types::ListsListResponseCollection;
}
pub struct IpListDelete;
impl koof::Operation for IpListDelete {
    const NAME: &'static str = "ip_list.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/rules/lists/{list_id}";
    type Request = ();
    type Response = crate::generated::types::ListsListDeleteResponseCollection;
}
pub struct IpListObserve;
impl koof::Operation for IpListObserve {
    const NAME: &'static str = "ip_list.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/rules/lists/{list_id}";
    type Request = ();
    type Response = crate::generated::types::ListsListResponseCollection;
}
pub struct IpListUpdate;
impl koof::Operation for IpListUpdate {
    const NAME: &'static str = "ip_list.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/rules/lists/{list_id}";
    type Request = crate::generated::types::IpListUpdateRequest;
    type Response = crate::generated::types::ListsListResponseCollection;
}
pub struct KvNamespaceCreate;
impl koof::Operation for KvNamespaceCreate {
    const NAME: &'static str = "kv_namespace.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/storage/kv/namespaces";
    type Request = crate::generated::types::WorkersKvCreateNamespaceBody;
    type Response = crate::generated::types::KvNamespaceCreateResponse;
}
pub struct KvNamespaceDelete;
impl koof::Operation for KvNamespaceDelete {
    const NAME: &'static str = "kv_namespace.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/storage/kv/namespaces/{namespace_id}";
    type Request = ();
    type Response = crate::generated::types::WorkersKvApiResponseCommonNoResult;
}
pub struct KvNamespaceObserve;
impl koof::Operation for KvNamespaceObserve {
    const NAME: &'static str = "kv_namespace.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/storage/kv/namespaces/{namespace_id}";
    type Request = ();
    type Response = crate::generated::types::KvNamespaceObserveResponse;
}
pub struct KvNamespaceUpdate;
impl koof::Operation for KvNamespaceUpdate {
    const NAME: &'static str = "kv_namespace.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/storage/kv/namespaces/{namespace_id}";
    type Request = crate::generated::types::WorkersKvCreateRenameNamespaceBody;
    type Response = crate::generated::types::KvNamespaceUpdateResponse;
}
pub struct LbMonitorCreate;
impl koof::Operation for LbMonitorCreate {
    const NAME: &'static str = "lb_monitor.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/monitors";
    type Request = crate::generated::types::LbMonitorCreateRequest;
    type Response = crate::generated::types::LoadBalancingMonitorResponseSingle;
}
pub struct LbMonitorDelete;
impl koof::Operation for LbMonitorDelete {
    const NAME: &'static str = "lb_monitor.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/monitors/{monitor_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingIdResponse;
}
pub struct LbMonitorObserve;
impl koof::Operation for LbMonitorObserve {
    const NAME: &'static str = "lb_monitor.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/monitors/{monitor_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingMonitorResponseSingle;
}
pub struct LbMonitorUpdate;
impl koof::Operation for LbMonitorUpdate {
    const NAME: &'static str = "lb_monitor.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/monitors/{monitor_id}";
    type Request = crate::generated::types::LbMonitorUpdateRequest;
    type Response = crate::generated::types::LoadBalancingMonitorResponseSingle;
}
pub struct LbPoolCreate;
impl koof::Operation for LbPoolCreate {
    const NAME: &'static str = "lb_pool.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/pools";
    type Request = crate::generated::types::LbPoolCreateRequest;
    type Response = crate::generated::types::LoadBalancingSchemasSingleResponse;
}
pub struct LbPoolDelete;
impl koof::Operation for LbPoolDelete {
    const NAME: &'static str = "lb_pool.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/pools/{pool_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingSchemasIdResponse;
}
pub struct LbPoolObserve;
impl koof::Operation for LbPoolObserve {
    const NAME: &'static str = "lb_pool.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/pools/{pool_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingSchemasSingleResponse;
}
pub struct LbPoolUpdate;
impl koof::Operation for LbPoolUpdate {
    const NAME: &'static str = "lb_pool.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/load_balancers/pools/{pool_id}";
    type Request = crate::generated::types::LbPoolUpdateRequest;
    type Response = crate::generated::types::LoadBalancingSchemasSingleResponse;
}
pub struct LoadBalancerCreate;
impl koof::Operation for LoadBalancerCreate {
    const NAME: &'static str = "load_balancer.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/load_balancers";
    type Request = crate::generated::types::LoadBalancerCreateRequest;
    type Response = crate::generated::types::LoadBalancingLoadBalancerComponentsSchemasSingleResponse;
}
pub struct LoadBalancerDelete;
impl koof::Operation for LoadBalancerDelete {
    const NAME: &'static str = "load_balancer.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/load_balancers/{load_balancer_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingComponentsSchemasIdResponse;
}
pub struct LoadBalancerObserve;
impl koof::Operation for LoadBalancerObserve {
    const NAME: &'static str = "load_balancer.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/load_balancers/{load_balancer_id}";
    type Request = ();
    type Response = crate::generated::types::LoadBalancingLoadBalancerComponentsSchemasSingleResponse;
}
pub struct LoadBalancerUpdate;
impl koof::Operation for LoadBalancerUpdate {
    const NAME: &'static str = "load_balancer.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/load_balancers/{load_balancer_id}";
    type Request = crate::generated::types::LoadBalancerUpdateRequest;
    type Response = crate::generated::types::LoadBalancingLoadBalancerComponentsSchemasSingleResponse;
}
pub struct LogpushJobCreate;
impl koof::Operation for LogpushJobCreate {
    const NAME: &'static str = "logpush_job.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/logpush/jobs";
    type Request = crate::generated::types::LogpushJobCreateRequest;
    type Response = crate::generated::types::LogpushLogpushJobResponseSingle;
}
pub struct LogpushJobDelete;
impl koof::Operation for LogpushJobDelete {
    const NAME: &'static str = "logpush_job.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/logpush/jobs/{job_id}";
    type Request = ();
    type Response = crate::generated::types::LogpushJobDeleteResponse;
}
pub struct LogpushJobObserve;
impl koof::Operation for LogpushJobObserve {
    const NAME: &'static str = "logpush_job.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/logpush/jobs/{job_id}";
    type Request = ();
    type Response = crate::generated::types::LogpushLogpushJobResponseSingle;
}
pub struct LogpushJobUpdate;
impl koof::Operation for LogpushJobUpdate {
    const NAME: &'static str = "logpush_job.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/logpush/jobs/{job_id}";
    type Request = crate::generated::types::LogpushJobUpdateRequest;
    type Response = crate::generated::types::LogpushLogpushJobResponseSingle;
}
pub struct NotificationPolicyCreate;
impl koof::Operation for NotificationPolicyCreate {
    const NAME: &'static str = "notification_policy.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/alerting/v3/policies";
    type Request = crate::generated::types::NotificationPolicyCreateRequest;
    type Response = crate::generated::types::AaaIdResponse;
}
pub struct NotificationPolicyDelete;
impl koof::Operation for NotificationPolicyDelete {
    const NAME: &'static str = "notification_policy.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/alerting/v3/policies/{policy_id}";
    type Request = ();
    type Response = crate::generated::types::AaaApiResponseCollection;
}
pub struct NotificationPolicyObserve;
impl koof::Operation for NotificationPolicyObserve {
    const NAME: &'static str = "notification_policy.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/alerting/v3/policies/{policy_id}";
    type Request = ();
    type Response = crate::generated::types::AaaSingleResponse;
}
pub struct NotificationPolicyUpdate;
impl koof::Operation for NotificationPolicyUpdate {
    const NAME: &'static str = "notification_policy.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/alerting/v3/policies/{policy_id}";
    type Request = crate::generated::types::NotificationPolicyUpdateRequest;
    type Response = crate::generated::types::AaaIdResponse;
}
pub struct PageRuleCreate;
impl koof::Operation for PageRuleCreate {
    const NAME: &'static str = "page_rule.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/pagerules";
    type Request = crate::generated::types::PageRuleCreateRequest;
    type Response = crate::generated::types::PageRuleCreateResponse;
}
pub struct PageRuleDelete;
impl koof::Operation for PageRuleDelete {
    const NAME: &'static str = "page_rule.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/pagerules/{pagerule_id}";
    type Request = ();
    type Response = crate::generated::types::ZonesApiResponseSingleId2;
}
pub struct PageRuleObserve;
impl koof::Operation for PageRuleObserve {
    const NAME: &'static str = "page_rule.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/pagerules/{pagerule_id}";
    type Request = ();
    type Response = crate::generated::types::PageRuleObserveResponse;
}
pub struct PageRuleUpdate;
impl koof::Operation for PageRuleUpdate {
    const NAME: &'static str = "page_rule.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/pagerules/{pagerule_id}";
    type Request = crate::generated::types::PageRuleUpdateRequest;
    type Response = crate::generated::types::PageRuleUpdateResponse;
}
pub struct PagesProjectCreate;
impl koof::Operation for PagesProjectCreate {
    const NAME: &'static str = "pages_project.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects";
    type Request = crate::generated::types::PagesProjectCreateRequest;
    type Response = crate::generated::types::PagesProjectCreateResponse;
}
pub struct PagesProjectDelete;
impl koof::Operation for PagesProjectDelete {
    const NAME: &'static str = "pages_project.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = ();
    type Response = crate::generated::types::PagesProjectDeleteResponse;
}
pub struct PagesProjectObserve;
impl koof::Operation for PagesProjectObserve {
    const NAME: &'static str = "pages_project.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = ();
    type Response = crate::generated::types::PagesProjectObserveResponse;
}
pub struct PagesProjectUpdate;
impl koof::Operation for PagesProjectUpdate {
    const NAME: &'static str = "pages_project.update";
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = crate::generated::types::PagesProjectUpdateRequest;
    type Response = crate::generated::types::PagesProjectUpdateResponse;
}
pub struct QueueCreate;
impl koof::Operation for QueueCreate {
    const NAME: &'static str = "queue.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/queues";
    type Request = crate::generated::types::QueueCreateRequest;
    type Response = crate::generated::types::QueueCreateResponse;
}
pub struct QueueDelete;
impl koof::Operation for QueueDelete {
    const NAME: &'static str = "queue.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/queues/{queue_id}";
    type Request = ();
    type Response = crate::generated::types::MqApiV4Success;
}
pub struct QueueObserve;
impl koof::Operation for QueueObserve {
    const NAME: &'static str = "queue.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/queues/{queue_id}";
    type Request = ();
    type Response = crate::generated::types::QueueObserveResponse;
}
pub struct QueueUpdate;
impl koof::Operation for QueueUpdate {
    const NAME: &'static str = "queue.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/queues/{queue_id}";
    type Request = crate::generated::types::MqQueue;
    type Response = crate::generated::types::QueueUpdateResponse;
}
pub struct R2BucketCreate;
impl koof::Operation for R2BucketCreate {
    const NAME: &'static str = "r2_bucket.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/r2/buckets";
    type Request = crate::generated::types::R2BucketCreateRequest;
    type Response = crate::generated::types::R2BucketCreateResponse;
}
pub struct R2BucketDelete;
impl koof::Operation for R2BucketDelete {
    const NAME: &'static str = "r2_bucket.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/r2/buckets/{bucket_name}";
    type Request = ();
    type Response = crate::generated::types::R2V4Response;
}
pub struct R2BucketObserve;
impl koof::Operation for R2BucketObserve {
    const NAME: &'static str = "r2_bucket.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/r2/buckets/{bucket_name}";
    type Request = ();
    type Response = crate::generated::types::R2BucketObserveResponse;
}
pub struct SpectrumApplicationCreate;
impl koof::Operation for SpectrumApplicationCreate {
    const NAME: &'static str = "spectrum_application.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/spectrum/apps";
    type Request = crate::generated::types::SpectrumConfigUpdateAppConfig;
    type Response = crate::generated::types::SpectrumConfigAppConfigSingle;
}
pub struct SpectrumApplicationDelete;
impl koof::Operation for SpectrumApplicationDelete {
    const NAME: &'static str = "spectrum_application.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/spectrum/apps/{app_id}";
    type Request = ();
    type Response = crate::generated::types::SpectrumConfigApiResponseSingleId;
}
pub struct SpectrumApplicationObserve;
impl koof::Operation for SpectrumApplicationObserve {
    const NAME: &'static str = "spectrum_application.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/spectrum/apps/{app_id}";
    type Request = ();
    type Response = crate::generated::types::SpectrumConfigAppConfigSingle;
}
pub struct SpectrumApplicationUpdate;
impl koof::Operation for SpectrumApplicationUpdate {
    const NAME: &'static str = "spectrum_application.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/spectrum/apps/{app_id}";
    type Request = crate::generated::types::SpectrumConfigUpdateAppConfig;
    type Response = crate::generated::types::SpectrumConfigAppConfigSingle;
}
pub struct TunnelCreate;
impl koof::Operation for TunnelCreate {
    const NAME: &'static str = "tunnel.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/cfd_tunnel";
    type Request = crate::generated::types::TunnelCreateRequest;
    type Response = crate::generated::types::TunnelCfdTunnelResponseSingle;
}
pub struct TunnelDelete;
impl koof::Operation for TunnelDelete {
    const NAME: &'static str = "tunnel.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/cfd_tunnel/{tunnel_id}";
    type Request = ();
    type Response = crate::generated::types::TunnelCfdTunnelResponseSingle;
}
pub struct TunnelObserve;
impl koof::Operation for TunnelObserve {
    const NAME: &'static str = "tunnel.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/cfd_tunnel/{tunnel_id}";
    type Request = ();
    type Response = crate::generated::types::TunnelCfdTunnelResponseSingle;
}
pub struct TunnelUpdate;
impl koof::Operation for TunnelUpdate {
    const NAME: &'static str = "tunnel.update";
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/accounts/{account_id}/cfd_tunnel/{tunnel_id}";
    type Request = crate::generated::types::TunnelUpdateRequest;
    type Response = crate::generated::types::TunnelCfdTunnelResponseSingle;
}
pub struct TurnstileWidgetCreate;
impl koof::Operation for TurnstileWidgetCreate {
    const NAME: &'static str = "turnstile_widget.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/challenges/widgets";
    type Request = crate::generated::types::TurnstileWidgetCreateRequest;
    type Response = crate::generated::types::TurnstileWidgetCreateResponse;
}
pub struct TurnstileWidgetDelete;
impl koof::Operation for TurnstileWidgetDelete {
    const NAME: &'static str = "turnstile_widget.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/challenges/widgets/{sitekey}";
    type Request = ();
    type Response = crate::generated::types::TurnstileWidgetDeleteResponse;
}
pub struct TurnstileWidgetObserve;
impl koof::Operation for TurnstileWidgetObserve {
    const NAME: &'static str = "turnstile_widget.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/challenges/widgets/{sitekey}";
    type Request = ();
    type Response = crate::generated::types::TurnstileWidgetObserveResponse;
}
pub struct TurnstileWidgetUpdate;
impl koof::Operation for TurnstileWidgetUpdate {
    const NAME: &'static str = "turnstile_widget.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}/challenges/widgets/{sitekey}";
    type Request = crate::generated::types::TurnstileWidgetUpdateRequest;
    type Response = crate::generated::types::TurnstileWidgetUpdateResponse;
}
pub struct VectorizeIndexCreate;
impl koof::Operation for VectorizeIndexCreate {
    const NAME: &'static str = "vectorize_index.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/vectorize/v2/indexes";
    type Request = crate::generated::types::VectorizeCreateIndexRequest;
    type Response = crate::generated::types::VectorizeIndexCreateResponse;
}
pub struct VectorizeIndexDelete;
impl koof::Operation for VectorizeIndexDelete {
    const NAME: &'static str = "vectorize_index.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/vectorize/v2/indexes/{index_name}";
    type Request = ();
    type Response = crate::generated::types::VectorizeIndexDeleteResponse;
}
pub struct VectorizeIndexObserve;
impl koof::Operation for VectorizeIndexObserve {
    const NAME: &'static str = "vectorize_index.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/vectorize/v2/indexes/{index_name}";
    type Request = ();
    type Response = crate::generated::types::VectorizeIndexObserveResponse;
}
pub struct WaitingRoomCreate;
impl koof::Operation for WaitingRoomCreate {
    const NAME: &'static str = "waiting_room.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/waiting_rooms";
    type Request = crate::generated::types::WaitingroomQueryWaitingroom;
    type Response = crate::generated::types::WaitingroomSingleResponse;
}
pub struct WaitingRoomDelete;
impl koof::Operation for WaitingRoomDelete {
    const NAME: &'static str = "waiting_room.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/waiting_rooms/{waiting_room_id}";
    type Request = ();
    type Response = crate::generated::types::WaitingroomWaitingRoomIdResponse;
}
pub struct WaitingRoomObserve;
impl koof::Operation for WaitingRoomObserve {
    const NAME: &'static str = "waiting_room.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/waiting_rooms/{waiting_room_id}";
    type Request = ();
    type Response = crate::generated::types::WaitingroomSingleResponse;
}
pub struct WaitingRoomUpdate;
impl koof::Operation for WaitingRoomUpdate {
    const NAME: &'static str = "waiting_room.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/waiting_rooms/{waiting_room_id}";
    type Request = crate::generated::types::WaitingroomQueryWaitingroom;
    type Response = crate::generated::types::WaitingroomSingleResponse;
}
pub struct WorkerRouteCreate;
impl koof::Operation for WorkerRouteCreate {
    const NAME: &'static str = "worker_route.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/workers/routes";
    type Request = crate::generated::types::WorkersRoute;
    type Response = crate::generated::types::WorkerRouteCreateResponse;
}
pub struct WorkerRouteDelete;
impl koof::Operation for WorkerRouteDelete {
    const NAME: &'static str = "worker_route.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/workers/routes/{route_id}";
    type Request = ();
    type Response = crate::generated::types::WorkerRouteDeleteResponse;
}
pub struct WorkerRouteObserve;
impl koof::Operation for WorkerRouteObserve {
    const NAME: &'static str = "worker_route.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/workers/routes/{route_id}";
    type Request = ();
    type Response = crate::generated::types::WorkerRouteObserveResponse;
}
pub struct WorkerRouteUpdate;
impl koof::Operation for WorkerRouteUpdate {
    const NAME: &'static str = "worker_route.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/workers/routes/{route_id}";
    type Request = crate::generated::types::WorkersRoute;
    type Response = crate::generated::types::WorkerRouteUpdateResponse;
}
pub struct ZoneCreate;
impl koof::Operation for ZoneCreate {
    const NAME: &'static str = "zone.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones";
    type Request = crate::generated::types::ZoneCreateRequest;
    type Response = crate::generated::types::ZoneCreateResponse;
}
pub struct ZoneDelete;
impl koof::Operation for ZoneDelete {
    const NAME: &'static str = "zone.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = ();
    type Response = crate::generated::types::ZonesApiResponseSingleId;
}
pub struct ZoneObserve;
impl koof::Operation for ZoneObserve {
    const NAME: &'static str = "zone.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = ();
    type Response = crate::generated::types::ZoneObserveResponse;
}
pub struct ZoneUpdate;
impl koof::Operation for ZoneUpdate {
    const NAME: &'static str = "zone.update";
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = crate::generated::types::ZoneUpdateRequest;
    type Response = crate::generated::types::ZoneUpdateResponse;
}
pub struct ZoneRulesetCreate;
impl koof::Operation for ZoneRulesetCreate {
    const NAME: &'static str = "zone_ruleset.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/rulesets";
    type Request = crate::generated::types::ZoneRulesetCreateRequest;
    type Response = crate::generated::types::ZoneRulesetCreateResponse;
}
pub struct ZoneRulesetDelete;
impl koof::Operation for ZoneRulesetDelete {
    const NAME: &'static str = "zone_ruleset.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = ();
    type Response = ();
}
pub struct ZoneRulesetObserve;
impl koof::Operation for ZoneRulesetObserve {
    const NAME: &'static str = "zone_ruleset.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = ();
    type Response = crate::generated::types::ZoneRulesetObserveResponse;
}
pub struct ZoneRulesetUpdate;
impl koof::Operation for ZoneRulesetUpdate {
    const NAME: &'static str = "zone_ruleset.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = crate::generated::types::ZoneRulesetUpdateRequest;
    type Response = crate::generated::types::ZoneRulesetUpdateResponse;
}
