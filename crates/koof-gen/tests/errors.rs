mod common;

use common::{generated, widgets_with};
use pretty_assertions::assert_eq;
use rstest::rstest;

fn error(actions: &str) -> Option<String> {
    generated(&widgets_with(actions)).err()
}

#[rstest]
#[case::observe_is_required(
    "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].get['x-kube-operation']
    remove: true
",
    "failed to build ProviderIr: resource widget has no observe operation"
)]
#[case::one_operation_per_lifecycle(
    "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].patch['x-kube-operation']
    update:
      lifecycle: create
",
    "failed to build ProviderIr: resource widget has duplicate create operation"
)]
#[case::operation_of_an_unknown_resource(
    "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].delete['x-kube-operation']
    update:
      resource: gadget
",
    "failed to build ProviderIr: paths./accounts/{account_id}/widgets/{widget_id}.delete references unknown resource gadget"
)]
#[case::misspelled_key(
    "
  - target: $.components.schemas.widget['x-kube-resource']
    update:
      apiVerison: shop.widgets.example.dev/v1
",
    "invalid x-kube-resource at components.schemas.widget: unknown field `apiVerison`, expected one of `name`, `apiVersion`, `kind`, `cluster`, `fields`, `identifiers`, `references`, `credentials`, `validations`, `immutable`, `test`"
)]
#[case::soft_delete_only_on_observe(
    "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].delete['x-kube-operation']
    update:
      response:
        path: result
        deletedWhen: deleted_at
",
    "failed to build ProviderIr: resource widget: response.deleted_when is only allowed on the observe operation"
)]
#[case::durations_need_a_unit(
    "
  - target: $['x-kube-provider'].http
    update:
      timeout: soon
",
    "failed to build ProviderIr: x-kube-provider.http.timeout: \"soon\" is not a duration like 30s: expected number at 0"
)]
#[case::every_path_parameter_is_bound(
    "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].get['x-kube-operation'].pathParameters.widget_id
    remove: true
",
    "operations.widget.observe.path_parameters: path expects {\"account_id\", \"widget_id\"}, but bindings are {\"account_id\"}"
)]
#[case::create_returns_the_external_name(
    "
  - target: $.paths['/accounts/{account_id}/widgets'].post['x-kube-operation'].result
    remove: true
",
    "resources.widget.lifecycle.create: create operation widget.create must declare result.identifiers.external_id: response.body.<path>"
)]
#[case::api_version_has_a_group(
    "
  - target: $.components.schemas.widget['x-kube-resource']
    update:
      apiVersion: v1
",
    "resources.widget.api_version: invalid Kubernetes apiVersion \"v1\": expected <group>/<version>"
)]
#[case::listed_fields_exist(
    "
  - target: $.components.schemas.widget['x-kube-resource'].fields.forProvider
    update: [colour]
",
    "resource widget: shop_v1alpha1_WidgetForProvider: fields [\"colour\"] not found in widget"
)]
#[case::kinds_are_unique_per_api_version(
    "
  - target: $.components.schemas.widget['x-kube-resource']
    update:
      apiVersion: widgets.example.dev/v1alpha1
      kind: Account
",
    "resource widget: kind Account in widgets.example.dev/v1alpha1 is already used by another resource"
)]
#[case::security_scheme_exists(
    "
  - target: $.components.schemas.widget['x-kube-resource'].credentials
    update:
      securityScheme: oauth
",
    "resources.widget.credentials.security_scheme: security scheme \"oauth\" does not exist"
)]
#[case::cluster_variant_references_only_cluster_kinds(
    "
  - target: $.components.schemas.account['x-kube-resource']
    update:
      references:
        widget:
          from: spec.forProvider.widgetRef
          target: widget
",
    "resources.account.references.widget: ClusterAccount cannot reference widget: it has no cluster variant (set cluster: true on it)"
)]
#[case::references_live_in_for_provider(
    "
  - target: $.components.schemas.widget['x-kube-resource'].references.account
    update:
      from: spec.accountRef
",
    "resources.widget.references.account: path must start with spec.forProvider.: spec.accountRef"
)]
fn rejects_overlay_mistakes(#[case] actions: &str, #[case] message: &str) {
    assert_eq!(error(actions), Some(message.to_owned()));
}

#[test]
fn reports_every_use_of_an_unknown_reference_target() {
    let message = error(
        "
  - target: $.components.schemas.widget['x-kube-resource'].references.account
    update:
      target: shop
",
    );

    assert_eq!(
        message.unwrap_or_default().lines().collect::<Vec<_>>(),
        [
            "resources.widget.references.account: unknown target resource shop",
            "resources.widget.credentials: unknown resource shop",
            "operations.widget.observe.path_parameters.account_id: unknown resource shop",
            "operations.widget.create.path_parameters.account_id: unknown resource shop",
            "operations.widget.update.path_parameters.account_id: unknown resource shop",
            "operations.widget.delete.path_parameters.account_id: unknown resource shop",
        ]
    );
}

#[test]
fn e2e_fixtures_cannot_depend_on_each_other_in_a_cycle() {
    let message = error(
        "
  - target: $.components.schemas.account['x-kube-resource']
    update:
      cluster: false
      references:
        widget:
          from: spec.forProvider.widgetRef
          target: widget
",
    );

    assert_eq!(
        message.as_deref(),
        Some(
            "e2e dependencies form a cycle: account -> widget -> account; remove `test` or break the cycle"
        )
    );
}
