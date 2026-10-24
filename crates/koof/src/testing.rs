use crate::{
    ManagedResource, ProviderInfo, ResourceDescriptor,
    http::{Operation, Unsupported},
    managed::ManagedResourceSpec,
    reference::SecretKeySelector,
    resource::{
        AuthScheme, CreateDescriptor, CredentialSource, Credentials, ObserveDescriptor,
        OperationDescriptor, Reference, RequestMapping, UpdateDescriptor, ValueExpr,
    },
    status::ManagedStatus,
};
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{CustomResource, client::Body};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, convert::Infallible, sync::Arc};

pub const GROUP: &str = "example.dev";
pub const NAMESPACE: &str = "default";

pub static PROVIDER: ProviderInfo = ProviderInfo::new("example", "https://api.example.dev");

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    pub name: String,
}

#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[kube(group = "example.dev", version = "v1", kind = "Account", namespaced)]
#[kube(status = "ManagedStatus<Value>")]
pub struct AccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccountForProvider,
    #[serde(flatten)]
    pub management: ManagedResourceSpec,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountForProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_token_secret_ref: Option<SecretKeySelector>,
}

#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[kube(group = "example.dev", version = "v1", kind = "Zone", namespaced)]
#[kube(status = "ManagedStatus<Value>")]
pub struct ZoneSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ZoneForProvider,
    #[serde(flatten)]
    pub management: ManagedResourceSpec,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ZoneForProvider {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ref: Option<ObjectReference>,
}

/// A DNS record of a zone.
#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[kube(group = "example.dev", version = "v1", kind = "Record", namespaced)]
#[kube(status = "ManagedStatus<RecordAtProvider>")]
pub struct RecordSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: RecordForProvider,
    #[serde(flatten)]
    pub management: ManagedResourceSpec,
}

fn record_schema(schema: &mut schemars::Schema) {
    crate::schema::add_validations(
        schema,
        &[crate::schema::Validation {
            rule: "!has(self.ttl) || self.ttl == 1 || self.ttl >= 30",
            message: "ttl must be 1 (automatic) or at least 30 seconds",
            field: "",
        }],
    );
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(transform = record_schema)]
pub struct RecordForProvider {
    /// Fully qualified record name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Time to live in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<RecordSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone_ref: Option<ObjectReference>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct RecordAtProvider {
    pub id: String,
    pub name: String,
    pub ttl: u32,
    /// When the record was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct RecordSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flatten_cname: Option<bool>,
}

const EXTERNAL_ID: &[(&str, ValueExpr)] = &[("external_id", ValueExpr::ExternalName)];
pub const ZONE_ID: ValueExpr = ValueExpr::RelatedIdentifier {
    via: &["zone"],
    name: "external_id",
};

pub static ACCOUNT: ResourceDescriptor = ResourceDescriptor::new(
    "Account",
    || kube::api::ApiResource::erase::<Account>(&()),
    GROUP,
    true,
    ObserveDescriptor::new(OperationDescriptor::new()),
)
.with_identifiers(EXTERNAL_ID)
.with_credentials(Credentials::new(
    CredentialSource::Secret {
        path: "/spec/forProvider/apiTokenSecretRef",
    },
    AuthScheme::Bearer,
));

pub static ZONE: ResourceDescriptor = ResourceDescriptor::new(
    "Zone",
    || kube::api::ApiResource::erase::<Zone>(&()),
    GROUP,
    true,
    ObserveDescriptor::new(OperationDescriptor::new()),
)
.with_identifiers(EXTERNAL_ID)
.with_references(&[Reference::new(
    "account",
    "/spec/forProvider/accountRef",
    &[&ACCOUNT],
)])
.with_credentials(Credentials::new(
    CredentialSource::Related { via: &["account"] },
    AuthScheme::Header("X-Auth-Key"),
));

pub static RECORD: ResourceDescriptor = ResourceDescriptor::new(
    "Record",
    || kube::api::ApiResource::erase::<Record>(&()),
    GROUP,
    true,
    ObserveDescriptor::new(RECORD_OPERATION.with_response_path("/result")),
)
.with_identifiers(EXTERNAL_ID)
.with_late_initialize(&["name", "ttl", "tags", "settings"])
.with_references(&[Reference::new(
    "zone",
    "/spec/forProvider/zoneRef",
    &[&ZONE],
)])
.with_credentials(Credentials::new(
    CredentialSource::Related {
        via: &["zone", "account"],
    },
    AuthScheme::Bearer,
))
.with_create(CreateDescriptor::new(
    OperationDescriptor::new().with_path_parameters(&[("zone_id", ZONE_ID)]),
    RequestMapping::new("/spec/forProvider"),
    "/result/id",
))
.with_update(UpdateDescriptor::new(
    RECORD_OPERATION.with_response_path("/result"),
    RequestMapping::new("/spec/forProvider"),
))
.with_delete(RECORD_OPERATION.with_response_path("/result"));

const RECORD_OPERATION: OperationDescriptor = OperationDescriptor::new()
    .with_path_parameters(&[("zone_id", ZONE_ID), ("record_id", ValueExpr::ExternalName)]);

macro_rules! operation {
    ($name:ident, $method:ident, $path:literal) => {
        pub struct $name;

        impl Operation for $name {
            const NAME: &'static str = stringify!($name);
            const METHOD: reqwest::Method = reqwest::Method::$method;
            const PATH: &'static str = $path;
            type Request = Value;
            type Response = Value;
        }
    };
}

operation!(RecordObserve, GET, "/zones/{zone_id}/records/{record_id}");
operation!(RecordCreate, POST, "/zones/{zone_id}/records");
operation!(RecordUpdate, PATCH, "/zones/{zone_id}/records/{record_id}");
operation!(RecordDelete, DELETE, "/zones/{zone_id}/records/{record_id}");

macro_rules! managed {
    ($kind:ident, $descriptor:ident, $at_provider:ty, $observe:ty, $create:ty, $update:ty, $delete:ty) => {
        impl ManagedResource for $kind {
            type AtProvider = $at_provider;
            type Observe = $observe;
            type Create = $create;
            type Update = $update;
            type Delete = $delete;

            fn descriptor() -> &'static ResourceDescriptor {
                &$descriptor
            }

            fn status(&self) -> Option<&ManagedStatus<$at_provider>> {
                self.status.as_ref()
            }

            fn management(&self) -> &ManagedResourceSpec {
                &self.spec.management
            }
        }
    };
}

managed!(
    Account,
    ACCOUNT,
    Value,
    Unsupported,
    Unsupported,
    Unsupported,
    Unsupported
);
managed!(
    Zone,
    ZONE,
    Value,
    Unsupported,
    Unsupported,
    Unsupported,
    Unsupported
);
managed!(
    Record,
    RECORD,
    RecordAtProvider,
    RecordObserve,
    RecordCreate,
    RecordUpdate,
    RecordDelete
);

/// A managed object of `kind` in [`NAMESPACE`] with `spec` and, when given, the
/// external-name annotation.
pub fn object(kind: &str, name: &str, external_name: Option<&str>, spec: Value) -> Value {
    let mut object = json!({
        "apiVersion": format!("{GROUP}/v1"),
        "kind": kind,
        "metadata": {
            "name": name,
            "namespace": NAMESPACE,
            "annotations": external_name.map_or_else(
                || json!({}),
                |external_name| json!({ format!("{GROUP}/external-name"): external_name }),
            ),
        },
    });
    object["spec"] = spec;
    object
}

pub fn record(external_name: Option<&str>, for_provider: Value) -> Record {
    let mut spec = json!({});
    spec["forProvider"] = for_provider;
    serde_json::from_value(object("Record", "www", external_name, spec)).expect("record")
}

/// `Record` www → `Zone` example (zone-1) → `Account` main (secret `cloudflare`).
pub fn zone() -> Value {
    object(
        "Zone",
        "example",
        Some("zone-1"),
        json!({ "forProvider": { "name": "example.com", "accountRef": { "name": "main" } } }),
    )
}

pub fn account() -> Value {
    object(
        "Account",
        "main",
        Some("account-1"),
        json!({ "forProvider": { "apiTokenSecretRef": { "name": "cloudflare", "key": "token" } } }),
    )
}

pub fn secret(name: &str, key: &str, value: &str) -> Value {
    serde_json::to_value(Secret {
        metadata: kube::api::ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(NAMESPACE.to_owned()),
            ..kube::api::ObjectMeta::default()
        },
        data: Some(BTreeMap::from([(
            key.to_owned(),
            ByteString(value.as_bytes().to_vec()),
        )])),
        ..Secret::default()
    })
    .expect("secret")
}

/// A Kubernetes API that serves `GET` of the given objects and `404` for anything else.
pub fn kubernetes(objects: &[Value]) -> kube::Client {
    let objects: Arc<BTreeMap<String, Value>> = Arc::new(
        objects
            .iter()
            .map(|object| (object_path(object), object.clone()))
            .collect(),
    );
    let service = tower::service_fn(move |request: http::Request<Body>| {
        let objects = objects.clone();
        async move {
            let (status, body) = match objects.get(request.uri().path()) {
                Some(object) => (200, object.clone()),
                None => (
                    404,
                    json!({ "kind": "Status", "apiVersion": "v1", "status": "Failure", "reason": "NotFound", "code": 404 }),
                ),
            };
            let mut response =
                http::Response::new(Body::from(serde_json::to_vec(&body).unwrap_or_default()));
            *response.status_mut() = http::StatusCode::from_u16(status).unwrap_or_default();
            Ok::<_, Infallible>(response)
        }
    });
    kube::Client::new(service, NAMESPACE)
}

fn object_path(object: &Value) -> String {
    let field = |pointer: &str| {
        object
            .pointer(pointer)
            .and_then(Value::as_str)
            .unwrap_or_default()
    };
    let plural = format!("{}s", field("/kind").to_lowercase());
    let prefix = match field("/apiVersion") {
        "v1" => "/api/v1".to_owned(),
        api_version => format!("/apis/{api_version}"),
    };
    format!(
        "{prefix}/namespaces/{}/{plural}/{}",
        field("/metadata/namespace"),
        field("/metadata/name")
    )
}
