mod common;

use common::{
    LABELS, NOTES, PARTS, TOKEN_OPERATION, TOKEN_SECRET, generated, widgets, widgets_with,
};

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

#[test]
fn item_references_reach_the_descriptor_and_the_crd_schema() {
    let files = generated(&widgets_with(PARTS)).expect("generated");

    insta::assert_snapshot!(
        "widget_with_item_references",
        &files["src/generated/shop/v1alpha1/widget.rs"]
    );
}

#[test]
fn required_nullable_fields_may_be_missing() {
    let files = generated(&widgets_with(
        "
  - target: $.components.schemas.widget
    update:
      required: [note, lastSeen]
      properties:
        note:
          type: string
          nullable: true
        lastSeen:
          type: string
          nullable: true
",
    ))
    .expect("generated");
    let types = &files["src/generated/types.rs"];

    assert!(!types.contains("Option::deserialize"));
    assert!(types.contains(
        "#[serde(default)]\n    pub note: ::std::option::Option<::std::string::String>,"
    ));
    assert!(types.contains("#[serde(rename = \"lastSeen\", default)]\n    pub last_seen:"));
}

#[test]
fn connection_details_can_come_from_a_connection_operation() {
    let files =
        generated(&widgets_with(&format!("{TOKEN_OPERATION}{TOKEN_SECRET}"))).expect("generated");
    let widget = &files["src/generated/shop/v1alpha1/widget.rs"];

    let descriptor: String = widget.split_whitespace().collect();

    assert!(descriptor.contains(r#".with_connection_secret(&[("widget_id","/id")])"#));
    assert!(descriptor.contains(r#"&[("token","")]"#));
    assert!(descriptor.contains("typeConnection=crate::generated::operations::WidgetConnection;"));
}

#[test]
fn a_bare_list_is_managed_as_a_field() {
    let files = generated(&widgets_with(LABELS)).expect("generated");
    let labels: String = files["src/generated/widgets/v1alpha1/labels.rs"]
        .split_whitespace()
        .collect();
    let types = &files["src/generated/types.rs"];

    assert!(labels.contains(r#"plural="labels","#));
    assert!(labels.contains(r#".with_into("labels")"#));
    assert!(labels.contains(r#"RequestMapping::new("/spec/forProvider/labels")"#));
    assert!(types.contains("pub struct WidgetsV1alpha1LabelsAtProvider {"));
    assert!(types.contains("pub labels: ::std::vec::Vec<Label>,"));
}

#[test]
fn a_merge_read_fills_a_field_and_a_body_can_be_multipart() {
    let files = generated(&widgets_with(NOTES)).expect("generated");
    let widget: String = files["src/generated/shop/v1alpha1/widget.rs"]
        .split_whitespace()
        .collect();
    let operations: String = files["src/generated/operations.rs"]
        .split_whitespace()
        .collect();
    let types = &files["src/generated/types.rs"];

    assert!(widget.contains(".with_merge(koof::resource::MergeDescriptor::new("));
    assert!(widget.contains(r#"],),"notes",),)"#));
    assert!(widget.contains("typeMerge=crate::generated::operations::WidgetMerge;"));
    assert!(operations.contains(
        r#"koof::multipart::Multipart::new(&["metadata"],"notes","application/octet-stream",)"#
    ));
    assert!(operations.contains("typeRequest=serde_json::Value;"));
    let at_provider = &types[types
        .find("pub struct ShopV1alpha1WidgetAtProvider")
        .expect("atProvider")..];
    assert!(at_provider[..at_provider.find('}').expect("end")].contains("pub notes:"));
}

#[test]
fn one_operation_can_serve_several_lifecycles() {
    let files = generated(&widgets_with(
        "
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].delete['x-kube-operation']
    remove: true
  - target: $.paths['/accounts/{account_id}/widgets/{widget_id}'].patch['x-kube-operation']
    update:
      lifecycle: [update, delete]
",
    ))
    .expect("generated");
    let operations: String = files["src/generated/operations.rs"]
        .split_whitespace()
        .collect();

    assert!(operations.contains(
        r#"pubstructWidgetDelete;implkoof::OperationforWidgetDelete{constNAME:&'staticstr="widget.delete";constMETHOD:reqwest::Method=reqwest::Method::PATCH;"#
    ));
}
