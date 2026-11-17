use super::*;
use crate::testing::{GROUP, record};
use rstest::rstest;
use serde_json::{Value, json};

const MANAGER: &str = "koof-example";

fn entry(manager: &str, operation: &str, subresource: Option<&str>, key: &str) -> Value {
    json!({
        "manager": manager,
        "operation": operation,
        "subresource": subresource,
        "fieldsType": "FieldsV1",
        "fieldsV1": { "f:metadata": { "f:annotations": { format!("f:{key}"): {} } } },
    })
}

fn external_name() -> String {
    format!("{GROUP}/external-name")
}

#[rstest]
#[case::applied_by_koof(entry(MANAGER, "Apply", None, &external_name()), true)]
#[case::applied_by_another_manager(entry("gitops", "Apply", None, &external_name()), false)]
#[case::merge_patched_by_koof(entry(MANAGER, "Update", None, &external_name()), false)]
#[case::on_the_status(entry(MANAGER, "Apply", Some("status"), &external_name()), false)]
#[case::another_annotation(entry(MANAGER, "Apply", None, &format!("{GROUP}/other")), false)]
fn ownership_needs_an_apply_of_the_key_by_the_manager(#[case] entry: Value, #[case] owns: bool) {
    let mut resource = record(Some("id-1"), json!({}));
    resource.metadata.managed_fields =
        Some(serde_json::from_value(json!([entry])).expect("managed fields"));

    assert_eq!(owned(&resource, MANAGER), owns);
}

#[test]
fn object_without_managed_fields_is_not_owned() {
    assert!(!owned(&record(Some("id-1"), json!({})), MANAGER));
}
