mod common;

use common::{codegen, widgets, widgets_with};
use koof_gen::{ir::ResourceName, spec};
use pretty_assertions::assert_eq;
use rstest::rstest;
use serde_json::{Value, json};

fn properties(document: &Value, schema: &str) -> Vec<String> {
    document["components"]["schemas"][schema]["properties"]
        .as_object()
        .map(|properties| properties.keys().cloned().collect())
        .unwrap_or_default()
}

#[rstest]
#[case::explicit_for_provider_has_only_listed_fields(
    "shop_v1alpha1_WidgetForProvider",
    &["color", "name", "size", "tags"]
)]
#[case::explicit_at_provider_has_only_listed_fields(
    "shop_v1alpha1_WidgetAtProvider",
    &["color", "created_on", "id", "name", "size", "tags"]
)]
#[case::all_skips_read_only_and_ignored_fields_in_for_provider(
    "widgets_v1alpha1_AccountForProvider",
    &["name"]
)]
#[case::all_keeps_every_field_but_sensitive_ones_in_at_provider(
    "widgets_v1alpha1_AccountAtProvider",
    &["id", "legacy_plan", "name"]
)]
fn picks_the_fields_of_each_resource(#[case] schema: &str, #[case] fields: &[&str]) {
    assert_eq!(properties(&codegen(&widgets()).document, schema), fields);
}

const WEBHOOK: &str = "
        type: object
        required: [url, secret]
        properties:
          url:
            type: string
          secret:
            type: string
            x-kube-field:
              sensitive: true
";

#[rstest]
#[case::inline(format!(
    "
  - target: $.components.schemas.account.properties
    update:
      webhook:{WEBHOOK}"
))]
#[case::through_a_ref(format!(
    "
  - target: $.components.schemas
    update:
      webhook:{WEBHOOK}
  - target: $.components.schemas.account.properties
    update:
      webhook:
        $ref: '#/components/schemas/webhook'
"
))]
fn sensitive_fields_are_left_out_of_nested_objects_in_at_provider(#[case] actions: String) {
    let document = codegen(&widgets_with(&actions)).document;

    assert_eq!(
        document["components"]["schemas"]["widgets_v1alpha1_AccountAtProvider"]["properties"]["webhook"],
        json!({
            "type": "object",
            "required": ["url"],
            "properties": { "url": { "type": "string" } },
        })
    );
}

#[test]
fn for_provider_keeps_required_fields() {
    let document = codegen(&widgets()).document;

    assert_eq!(
        document["components"]["schemas"]["shop_v1alpha1_WidgetForProvider"]["required"],
        json!(["name"])
    );
}

#[test]
fn nullable_becomes_a_null_type() {
    let document = codegen(&widgets()).document;

    assert_eq!(
        document["components"]["schemas"]["shop_v1alpha1_WidgetForProvider"]["properties"]["color"]
            ["type"],
        json!(["string", "null"])
    );
}

#[rstest]
#[case::every_writable_field_by_default("account", &["name"])]
#[case::except_fields_marked_late_initialize_false("widget", &["color", "name", "size"])]
fn late_initializes_writable_fields(#[case] resource: &str, #[case] fields: &[&str]) {
    assert_eq!(
        codegen(&widgets()).late_initialize[&ResourceName(resource.to_owned())],
        fields
    );
}

#[test]
fn unused_paths_and_schemas_are_pruned() {
    let prepared = spec::prepare(&widgets()).expect("prepare");

    assert_eq!(
        (
            prepared.pointer("/paths/~1stats").is_some(),
            prepared.pointer("/paths/~1accounts").is_some()
        ),
        (false, true)
    );
}

#[test]
fn selecting_all_fields_includes_new_api_fields() {
    let document = codegen(&widgets_with(
        "
  - target: $.components.schemas.account.properties
    update:
      region:
        type: string
",
    ))
    .document;

    assert_eq!(
        properties(&document, "widgets_v1alpha1_AccountForProvider"),
        ["name", "region"]
    );
}

#[rstest]
#[case::keeps_string_enums(json!({ "type": "string", "enum": ["a", "b"] }), json!({ "type": "string", "enum": ["a", "b"] }))]
#[case::moves_integer_enums_into_the_description(
    json!({ "type": "integer", "enum": [1, 2] }),
    json!({ "type": "integer", "description": "Allowed values: 1, 2." })
)]
fn enums_typify_cannot_represent_are_dropped(#[case] mut schema: Value, #[case] kept: Value) {
    spec::drop_non_string_enums(&mut schema);
    assert_eq!(schema, kept);
}
