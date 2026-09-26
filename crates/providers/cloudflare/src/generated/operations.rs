//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
pub struct AccountCreate;
impl koof::Operation for AccountCreate {
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts";
    type Request = crate::generated::types::IamCreateAccount;
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct AccountDelete;
impl koof::Operation for AccountDelete {
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = ();
    type Response = crate::generated::types::IamApiResponseSingleId;
}
pub struct AccountObserve;
impl koof::Operation for AccountObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = ();
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct AccountUpdate;
impl koof::Operation for AccountUpdate {
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/accounts/{account_id}";
    type Request = crate::generated::types::IamComponentsSchemasAccount;
    type Response = crate::generated::types::IamResponseSingleAccount;
}
pub struct DnsRecordCreate;
impl koof::Operation for DnsRecordCreate {
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/dns_records";
    type Request = crate::generated::types::DnsRecordsDnsRecordPost;
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct DnsRecordDelete;
impl koof::Operation for DnsRecordDelete {
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = ();
    type Response = crate::generated::types::DnsRecordDeleteResponse;
}
pub struct DnsRecordObserve;
impl koof::Operation for DnsRecordObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = ();
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct DnsRecordUpdate;
impl koof::Operation for DnsRecordUpdate {
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/zones/{zone_id}/dns_records/{dns_record_id}";
    type Request = crate::generated::types::DnsRecordsDnsRecordPatch;
    type Response = crate::generated::types::DnsRecordsDnsResponseSingle;
}
pub struct PagesProjectCreate;
impl koof::Operation for PagesProjectCreate {
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects";
    type Request = crate::generated::types::PagesProjectCreateRequest;
    type Response = crate::generated::types::PagesProjectCreateResponse;
}
pub struct PagesProjectDelete;
impl koof::Operation for PagesProjectDelete {
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = ();
    type Response = crate::generated::types::PagesProjectDeleteResponse;
}
pub struct PagesProjectObserve;
impl koof::Operation for PagesProjectObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = ();
    type Response = crate::generated::types::PagesProjectObserveResponse;
}
pub struct PagesProjectUpdate;
impl koof::Operation for PagesProjectUpdate {
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/accounts/{account_id}/pages/projects/{project_name}";
    type Request = crate::generated::types::PagesProjectUpdateRequest;
    type Response = crate::generated::types::PagesProjectUpdateResponse;
}
pub struct ZoneCreate;
impl koof::Operation for ZoneCreate {
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones";
    type Request = crate::generated::types::ZoneCreateRequest;
    type Response = crate::generated::types::ZoneCreateResponse;
}
pub struct ZoneDelete;
impl koof::Operation for ZoneDelete {
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = ();
    type Response = crate::generated::types::ZonesApiResponseSingleId;
}
pub struct ZoneObserve;
impl koof::Operation for ZoneObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = ();
    type Response = crate::generated::types::ZoneObserveResponse;
}
pub struct ZoneUpdate;
impl koof::Operation for ZoneUpdate {
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    const PATH: &'static str = "/zones/{zone_id}";
    type Request = crate::generated::types::ZoneUpdateRequest;
    type Response = crate::generated::types::ZoneUpdateResponse;
}
pub struct ZoneRulesetCreate;
impl koof::Operation for ZoneRulesetCreate {
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/zones/{zone_id}/rulesets";
    type Request = crate::generated::types::ZoneRulesetCreateRequest;
    type Response = crate::generated::types::ZoneRulesetCreateResponse;
}
pub struct ZoneRulesetDelete;
impl koof::Operation for ZoneRulesetDelete {
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = ();
    type Response = ();
}
pub struct ZoneRulesetObserve;
impl koof::Operation for ZoneRulesetObserve {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = ();
    type Response = crate::generated::types::ZoneRulesetObserveResponse;
}
pub struct ZoneRulesetUpdate;
impl koof::Operation for ZoneRulesetUpdate {
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/zones/{zone_id}/rulesets/{ruleset_id}";
    type Request = crate::generated::types::ZoneRulesetUpdateRequest;
    type Response = crate::generated::types::ZoneRulesetUpdateResponse;
}
