pub mod operations;
pub mod types;
pub mod dashboard;
pub mod monitor;
pub mod organization;
pub mod organization_member;
pub mod project;
pub mod project_key;
pub mod service_hook;
pub mod team;
pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo {
    name: "sentry",
    base_url: "https://sentry.io",
};
/// Registers the controllers of every resource of this provider.
#[must_use]
pub fn controllers(app: koof::App) -> koof::App {
    app.controller::<dashboard::Dashboard>()
        .controller::<monitor::Monitor>()
        .controller::<organization::Organization>()
        .controller::<organization_member::OrganizationMember>()
        .controller::<project::Project>()
        .controller::<project_key::ProjectKey>()
        .controller::<service_hook::ServiceHook>()
        .controller::<team::Team>()
}
/// `CustomResourceDefinition`s of every resource of this provider.
#[must_use]
pub fn crds() -> Vec<
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
> {
    vec![
        < dashboard::Dashboard as kube::CustomResourceExt > ::crd(), < monitor::Monitor
        as kube::CustomResourceExt > ::crd(), < organization::Organization as
        kube::CustomResourceExt > ::crd(), < organization_member::OrganizationMember as
        kube::CustomResourceExt > ::crd(), < project::Project as kube::CustomResourceExt
        > ::crd(), < project_key::ProjectKey as kube::CustomResourceExt > ::crd(), <
        service_hook::ServiceHook as kube::CustomResourceExt > ::crd(), < team::Team as
        kube::CustomResourceExt > ::crd()
    ]
}
