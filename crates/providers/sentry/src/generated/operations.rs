//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
pub struct DashboardCreate;
impl koof::Operation for DashboardCreate {
    const NAME: &'static str = "dashboard.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/dashboards/";
    type Request = crate::generated::types::DashboardCreateRequest;
    type Response = crate::generated::types::DashboardCreateResponse;
}
pub struct DashboardDelete;
impl koof::Operation for DashboardDelete {
    const NAME: &'static str = "dashboard.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/dashboards/{dashboard_id}/";
    type Request = ();
    type Response = ();
}
pub struct DashboardObserve;
impl koof::Operation for DashboardObserve {
    const NAME: &'static str = "dashboard.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/dashboards/{dashboard_id}/";
    type Request = ();
    type Response = crate::generated::types::DashboardObserveResponse;
}
pub struct DashboardUpdate;
impl koof::Operation for DashboardUpdate {
    const NAME: &'static str = "dashboard.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/dashboards/{dashboard_id}/";
    type Request = crate::generated::types::DashboardUpdateRequest;
    type Response = crate::generated::types::DashboardUpdateResponse;
}
pub struct MonitorCreate;
impl koof::Operation for MonitorCreate {
    const NAME: &'static str = "monitor.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/monitors/";
    type Request = crate::generated::types::MonitorCreateRequest;
    type Response = crate::generated::types::MonitorCreateResponse;
}
pub struct MonitorDelete;
impl koof::Operation for MonitorDelete {
    const NAME: &'static str = "monitor.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/monitors/{monitor_id_or_slug}/";
    type Request = ();
    type Response = ();
}
pub struct MonitorObserve;
impl koof::Operation for MonitorObserve {
    const NAME: &'static str = "monitor.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/monitors/{monitor_id_or_slug}/";
    type Request = ();
    type Response = crate::generated::types::MonitorObserveResponse;
}
pub struct MonitorUpdate;
impl koof::Operation for MonitorUpdate {
    const NAME: &'static str = "monitor.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/monitors/{monitor_id_or_slug}/";
    type Request = crate::generated::types::MonitorUpdateRequest;
    type Response = crate::generated::types::MonitorUpdateResponse;
}
pub struct OrganizationObserve;
impl koof::Operation for OrganizationObserve {
    const NAME: &'static str = "organization.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/";
    type Request = ();
    type Response = crate::generated::types::OrganizationObserveResponse;
}
pub struct OrganizationUpdate;
impl koof::Operation for OrganizationUpdate {
    const NAME: &'static str = "organization.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/";
    type Request = crate::generated::types::OrganizationUpdateRequest;
    type Response = crate::generated::types::OrganizationUpdateResponse;
}
pub struct OrganizationMemberCreate;
impl koof::Operation for OrganizationMemberCreate {
    const NAME: &'static str = "organization_member.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/members/";
    type Request = crate::generated::types::OrganizationMemberCreateRequest;
    type Response = crate::generated::types::OrganizationMemberCreateResponse;
}
pub struct OrganizationMemberDelete;
impl koof::Operation for OrganizationMemberDelete {
    const NAME: &'static str = "organization_member.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/members/{member_id}/";
    type Request = ();
    type Response = ();
}
pub struct OrganizationMemberObserve;
impl koof::Operation for OrganizationMemberObserve {
    const NAME: &'static str = "organization_member.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/members/{member_id}/";
    type Request = ();
    type Response = crate::generated::types::OrganizationMemberObserveResponse;
}
pub struct OrganizationMemberUpdate;
impl koof::Operation for OrganizationMemberUpdate {
    const NAME: &'static str = "organization_member.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/members/{member_id}/";
    type Request = crate::generated::types::OrganizationMemberUpdateRequest;
    type Response = crate::generated::types::OrganizationMemberUpdateResponse;
}
pub struct ProjectCreate;
impl koof::Operation for ProjectCreate {
    const NAME: &'static str = "project.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/teams/{organization_id_or_slug}/{team_id_or_slug}/projects/";
    type Request = crate::generated::types::ProjectCreateRequest;
    type Response = crate::generated::types::ProjectCreateResponse;
}
pub struct ProjectDelete;
impl koof::Operation for ProjectDelete {
    const NAME: &'static str = "project.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/";
    type Request = ();
    type Response = ();
}
pub struct ProjectObserve;
impl koof::Operation for ProjectObserve {
    const NAME: &'static str = "project.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/";
    type Request = ();
    type Response = crate::generated::types::ProjectObserveResponse;
}
pub struct ProjectUpdate;
impl koof::Operation for ProjectUpdate {
    const NAME: &'static str = "project.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/";
    type Request = crate::generated::types::ProjectUpdateRequest;
    type Response = crate::generated::types::ProjectUpdateResponse;
}
pub struct ProjectKeyCreate;
impl koof::Operation for ProjectKeyCreate {
    const NAME: &'static str = "project_key.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/keys/";
    type Request = crate::generated::types::ProjectKeyCreateRequest;
    type Response = crate::generated::types::ProjectKeyCreateResponse;
}
pub struct ProjectKeyDelete;
impl koof::Operation for ProjectKeyDelete {
    const NAME: &'static str = "project_key.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/keys/{key_id}/";
    type Request = ();
    type Response = ();
}
pub struct ProjectKeyObserve;
impl koof::Operation for ProjectKeyObserve {
    const NAME: &'static str = "project_key.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/keys/{key_id}/";
    type Request = ();
    type Response = crate::generated::types::ProjectKeyObserveResponse;
}
pub struct ProjectKeyUpdate;
impl koof::Operation for ProjectKeyUpdate {
    const NAME: &'static str = "project_key.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/keys/{key_id}/";
    type Request = crate::generated::types::ProjectKeyUpdateRequest;
    type Response = crate::generated::types::ProjectKeyUpdateResponse;
}
pub struct ServiceHookCreate;
impl koof::Operation for ServiceHookCreate {
    const NAME: &'static str = "service_hook.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/hooks/";
    type Request = crate::generated::types::ServiceHookCreateRequest;
    type Response = crate::generated::types::ServiceHookCreateResponse;
}
pub struct ServiceHookDelete;
impl koof::Operation for ServiceHookDelete {
    const NAME: &'static str = "service_hook.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/hooks/{hook_id}/";
    type Request = ();
    type Response = ();
}
pub struct ServiceHookObserve;
impl koof::Operation for ServiceHookObserve {
    const NAME: &'static str = "service_hook.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/hooks/{hook_id}/";
    type Request = ();
    type Response = crate::generated::types::ServiceHookObserveResponse;
}
pub struct ServiceHookUpdate;
impl koof::Operation for ServiceHookUpdate {
    const NAME: &'static str = "service_hook.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/projects/{organization_id_or_slug}/{project_id_or_slug}/hooks/{hook_id}/";
    type Request = crate::generated::types::ServiceHookUpdateRequest;
    type Response = crate::generated::types::ServiceHookUpdateResponse;
}
pub struct TeamCreate;
impl koof::Operation for TeamCreate {
    const NAME: &'static str = "team.create";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const PATH: &'static str = "/api/0/organizations/{organization_id_or_slug}/teams/";
    type Request = crate::generated::types::TeamCreateRequest;
    type Response = crate::generated::types::TeamCreateResponse;
}
pub struct TeamDelete;
impl koof::Operation for TeamDelete {
    const NAME: &'static str = "team.delete";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const PATH: &'static str = "/api/0/teams/{organization_id_or_slug}/{team_id_or_slug}/";
    type Request = ();
    type Response = ();
}
pub struct TeamObserve;
impl koof::Operation for TeamObserve {
    const NAME: &'static str = "team.observe";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const PATH: &'static str = "/api/0/teams/{organization_id_or_slug}/{team_id_or_slug}/";
    type Request = ();
    type Response = crate::generated::types::TeamObserveResponse;
}
pub struct TeamUpdate;
impl koof::Operation for TeamUpdate {
    const NAME: &'static str = "team.update";
    const METHOD: reqwest::Method = reqwest::Method::PUT;
    const PATH: &'static str = "/api/0/teams/{organization_id_or_slug}/{team_id_or_slug}/";
    type Request = crate::generated::types::TeamUpdateRequest;
    type Response = crate::generated::types::TeamUpdateResponse;
}
