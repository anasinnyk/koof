pub mod operations;
pub mod types;
pub mod sentry;
fn http(http: koof::config::HttpConfig) -> koof::config::HttpConfig {
    http
}
pub static PROVIDER: koof::ProviderInfo = koof::ProviderInfo::new(
        "sentry",
        "https://sentry.io",
    )
    .with_http(http);
/// Registers the controllers of every resource of this provider.
#[must_use]
pub fn controllers(app: koof::App) -> koof::App {
    app.controller::<sentry::v1alpha1::dashboard::Dashboard>()
        .controller::<sentry::v1alpha1::monitor::Monitor>()
        .controller::<sentry::v1alpha1::organization::Organization>()
        .controller::<sentry::v1alpha1::cluster_organization::ClusterOrganization>()
        .controller::<sentry::v1alpha1::organization_member::OrganizationMember>()
        .controller::<sentry::v1alpha1::project::Project>()
        .controller::<sentry::v1alpha1::project_key::ProjectKey>()
        .controller::<sentry::v1alpha1::service_hook::ServiceHook>()
        .controller::<sentry::v1alpha1::team::Team>()
}
#[cfg(feature = "fake")]
#[must_use]
pub fn fake() -> koof::fake::Fake {
    koof::fake::Fake::new()
        .resource::<sentry::v1alpha1::dashboard::Dashboard>()
        .resource::<sentry::v1alpha1::monitor::Monitor>()
        .resource::<sentry::v1alpha1::organization::Organization>()
        .resource::<sentry::v1alpha1::cluster_organization::ClusterOrganization>()
        .resource::<sentry::v1alpha1::organization_member::OrganizationMember>()
        .resource::<sentry::v1alpha1::project::Project>()
        .resource::<sentry::v1alpha1::project_key::ProjectKey>()
        .resource::<sentry::v1alpha1::service_hook::ServiceHook>()
        .resource::<sentry::v1alpha1::team::Team>()
}
/// `CustomResourceDefinition`s of every resource of this provider.
#[must_use]
pub fn crds() -> Vec<
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
> {
    vec![
        < sentry::v1alpha1::dashboard::Dashboard as kube::CustomResourceExt > ::crd(), <
        sentry::v1alpha1::monitor::Monitor as kube::CustomResourceExt > ::crd(), <
        sentry::v1alpha1::organization::Organization as kube::CustomResourceExt >
        ::crd(), < sentry::v1alpha1::cluster_organization::ClusterOrganization as
        kube::CustomResourceExt > ::crd(), <
        sentry::v1alpha1::organization_member::OrganizationMember as
        kube::CustomResourceExt > ::crd(), < sentry::v1alpha1::project::Project as
        kube::CustomResourceExt > ::crd(), < sentry::v1alpha1::project_key::ProjectKey as
        kube::CustomResourceExt > ::crd(), < sentry::v1alpha1::service_hook::ServiceHook
        as kube::CustomResourceExt > ::crd(), < sentry::v1alpha1::team::Team as
        kube::CustomResourceExt > ::crd()
    ]
}
