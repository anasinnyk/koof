mod common;

use common::{generated, widgets, widgets_with};

#[test]
fn generates_the_provider_crate() {
    let files = generated(&widgets()).expect("generated");

    insta::assert_snapshot!(files.keys().cloned().collect::<Vec<_>>().join("\n"));
}

#[test]
fn generates_rust_for_each_resource() {
    let files = generated(&widgets()).expect("generated");

    insta::assert_snapshot!("widget", &files["src/generated/shop/v1alpha1/widget.rs"]);
    insta::assert_snapshot!(
        "account",
        &files["src/generated/widgets/v1alpha1/account.rs"]
    );
    insta::assert_snapshot!(
        "cluster_account",
        &files["src/generated/widgets/v1alpha1/cluster_account.rs"]
    );
}

#[test]
fn generates_the_provider_entry_points() {
    let files = generated(&widgets()).expect("generated");

    insta::assert_snapshot!("mod", &files["src/generated/mod.rs"]);
    insta::assert_snapshot!("operations", &files["src/generated/operations.rs"]);
}

#[test]
fn generates_an_e2e_test_per_resource_with_a_fixture() {
    let files = generated(&widgets()).expect("generated");

    insta::assert_snapshot!(
        "e2e_widget",
        &files["e2e/generated/widget/chainsaw-test.yaml"]
    );
}

#[test]
fn keyed_lists_reach_the_descriptor_and_the_crd_schema() {
    let files = generated(&widgets_with(
        "
  - target: $.components.schemas.account.properties
    update:
      members:
        type: array
        items:
          type: object
          required: [email]
          properties:
            email:
              type: string
        x-kube-field:
          listMapKeys: [email]
",
    ))
    .expect("generated");
    let account = &files["src/generated/widgets/v1alpha1/account.rs"];

    assert!(account.contains(r#".with_list_map_keys(&[("members", &["email"])])"#));
    assert!(account.contains("koof::schema::add_list_maps("));
}
