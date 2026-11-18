mod common;

use common::{PARTS, codegen, widgets, widgets_with};
use koof_gen::{
    ir::{ListMapIr, ResourceName, SecretValueIr},
    spec,
};
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
        codegen(&widgets()).fields[&ResourceName(resource.to_owned())].late_initialize,
        fields
    );
}

#[test]
fn keyed_lists_are_collected_per_resource() {
    let fields = codegen(&widgets_with(
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
            role:
              type: string
        x-kube-field:
          listMapKeys: [email]
",
    ))
    .fields;

    assert_eq!(
        fields[&ResourceName("account".to_owned())].list_map_keys,
        [(
            "members".to_owned(),
            ListMapIr {
                keys: vec!["email".to_owned()],
                in_schema: true
            }
        )]
        .into()
    );
}

#[test]
fn item_references_add_their_field_to_the_items() {
    let codegen = codegen(&widgets_with(PARTS));

    assert_eq!(
        (
            &codegen.document["components"]["schemas"]["shop_v1alpha1_WidgetForProvider"]["properties"]
                ["parts"]["items"],
            &codegen.fields[&ResourceName("widget".to_owned())].list_map_keys["parts"],
        ),
        (
            &json!({
                "type": "object",
                "required": [],
                "properties": {
                    "widgetId": { "type": "string" },
                    "count": { "type": "integer" },
                    "widgetRef": {
                        "type": "object",
                        "description": "A reference to an object of kind `Widget` in the namespace of the resource holding the reference; sets `widgetId`.",
                        "required": ["name"],
                        "properties": { "name": { "type": "string" } },
                    },
                },
            }),
            &ListMapIr {
                keys: vec!["widgetId".to_owned()],
                in_schema: false
            },
        )
    );
}

const RULES: &str = "
  - target: $.components.schemas
    update:
      email_rule:
        type: object
        required: [email]
        properties:
          email:
            type: string
      widget_rule:
        type: object
        required: [widget]
        properties:
          widget:
            type: object
            required: [id]
            properties:
              id:
                type: string
      rule:
        type: object
        oneOf:
          - $ref: '#/components/schemas/email_rule'
          - $ref: '#/components/schemas/widget_rule'
  - target: $.components.schemas.widget.properties
    update:
      include:
        type: array
        items:
          $ref: '#/components/schemas/rule'
  - target: $.components.schemas.widget['x-kube-resource'].fields.forProvider
    update: [include]
  - target: $.components.schemas.widget['x-kube-resource']
    update:
      itemReferences:
        widget:
          from: spec.forProvider.include[].widgetRef
          target: widget
          into: widget.id
          identifier: external_id
";

#[test]
fn item_references_into_a_union_add_a_variant() {
    let codegen = codegen(&widgets_with(RULES));

    assert_eq!(
        (
            &codegen.document["components"]["schemas"]["shop_v1alpha1_WidgetForProvider"]["properties"]
                ["include"]["items"]["oneOf"][2],
            &codegen.fields[&ResourceName("widget".to_owned())].union_items,
        ),
        (
            &json!({
                "type": "object",
                "required": ["widgetRef"],
                "properties": {
                    "widgetRef": {
                        "type": "object",
                        "description": "A reference to an object of kind `Widget` in the namespace of the resource holding the reference; sets `widget.id`.",
                        "required": ["name"],
                        "properties": { "name": { "type": "string" } },
                    },
                },
            }),
            &["include".to_owned()].into(),
        )
    );
}

const ENV: &str = "
  - target: $.components.schemas
    update:
      plain_var:
        type: object
        required: [type, value]
        properties:
          type:
            type: string
            enum: [plain]
          value:
            type: string
            x-kube-field:
              secretKeyRef: true
      secret_var:
        type: object
        required: [type]
        properties:
          type:
            type: string
            enum: [secret]
          value:
            type: string
            x-kube-field:
              secretKeyRef: true
      env:
        type: object
        additionalProperties:
          oneOf:
            - $ref: '#/components/schemas/plain_var'
            - $ref: '#/components/schemas/secret_var'
  - target: $.components.schemas.widget.properties
    update:
      config:
        type: object
        properties:
          env:
            $ref: '#/components/schemas/env'
  - target: $.components.schemas.widget['x-kube-resource'].fields.forProvider
    update: [config]
";

#[test]
fn secret_key_ref_fields_can_be_read_from_a_secret() {
    let codegen = codegen(&widgets_with(ENV));
    let variants = &codegen.document["components"]["schemas"]["shop_v1alpha1_WidgetForProvider"]["properties"]
        ["config"]["properties"]["env"]["additionalProperties"]["oneOf"];

    assert_eq!(
        (
            &variants[0]["required"],
            &variants[0]["properties"]["valueFrom"]["properties"]["secretKeyRef"]["required"],
            &variants[1]["properties"]["valueFrom"]["required"],
            &codegen.document["components"]["schemas"]["plain_var"]["required"],
            &codegen.fields[&ResourceName("widget".to_owned())].secret_values,
        ),
        (
            &json!(["type"]),
            &json!(["name", "key"]),
            &json!(["secretKeyRef"]),
            &json!(["type", "value"]),
            &vec![
                SecretValueIr {
                    objects: vec!["config".to_owned(), "env".to_owned(), "*".to_owned()],
                    field: "value".to_owned(),
                    required: false,
                },
                SecretValueIr {
                    objects: vec!["config".to_owned(), "env".to_owned(), "*".to_owned()],
                    field: "value".to_owned(),
                    required: true,
                },
            ],
        )
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
