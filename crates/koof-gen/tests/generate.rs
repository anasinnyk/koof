mod common;

use common::{generated, widgets};

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
