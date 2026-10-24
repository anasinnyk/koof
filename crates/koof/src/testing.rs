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
use std::{
    collections::BTreeMap,
    convert::Infallible,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

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
    Cluster::new(objects).client()
}

/// An in-memory Kubernetes API: get, list (per namespace or across namespaces),
/// create, merge-patch and JSON-patch of objects and their status, with
/// `resourceVersion` conflicts and deletion once the last finalizer is gone.
#[derive(Clone, Default)]
pub struct Cluster {
    objects: Arc<Mutex<BTreeMap<String, Value>>>,
    version: Arc<AtomicU64>,
}

#[cfg_attr(not(feature = "fake"), allow(dead_code))]
impl Cluster {
    pub fn new(objects: &[Value]) -> Self {
        let cluster = Self::default();
        for object in objects {
            cluster.put(object.clone());
        }
        cluster
    }

    pub fn client(&self) -> kube::Client {
        let cluster = self.clone();
        let service = tower::service_fn(move |request: http::Request<Body>| {
            let cluster = cluster.clone();
            async move {
                let (parts, body) = request.into_parts();
                let body = http_body_util::BodyExt::collect(body)
                    .await
                    .map(http_body_util::Collected::to_bytes)
                    .unwrap_or_default();
                let content_type = parts
                    .headers
                    .get(http::header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or_default()
                    .to_owned();
                if parts
                    .uri
                    .query()
                    .is_some_and(|query| query.contains("watch=true"))
                {
                    std::future::pending::<()>().await;
                }
                let (status, body) =
                    cluster.handle(&parts.method, parts.uri.path(), &content_type, &body);
                let mut response =
                    http::Response::new(Body::from(serde_json::to_vec(&body).unwrap_or_default()));
                *response.status_mut() = http::StatusCode::from_u16(status).unwrap_or_default();
                Ok::<_, Infallible>(response)
            }
        });
        kube::Client::new(service, NAMESPACE)
    }

    /// Stores `object` as is, with a new `resourceVersion`.
    pub fn put(&self, mut object: Value) {
        object["metadata"]["resourceVersion"] = json!(self.next_version());
        self.lock().insert(object_path(&object), object);
    }

    pub fn get(&self, kind: &str, name: &str) -> Option<Value> {
        self.find(kind, name)
    }

    pub fn typed<R: serde::de::DeserializeOwned>(&self, kind: &str, name: &str) -> R {
        serde_json::from_value(self.find(kind, name).expect("object exists")).expect("typed object")
    }

    /// Deletes like `kubectl delete`: an object with finalizers only gets a
    /// `deletionTimestamp`.
    pub fn delete(&self, kind: &str, name: &str) {
        let mut objects = self.lock();
        let Some((path, object)) = objects
            .iter_mut()
            .find(|(_, object)| object["kind"] == kind && object["metadata"]["name"] == name)
        else {
            return;
        };
        let path = path.clone();
        if has_finalizers(object) {
            object["metadata"]["deletionTimestamp"] = json!("2026-01-01T00:00:00Z");
        } else {
            objects.remove(&path);
        }
    }

    /// `(reason, note)` of every published event, oldest first.
    pub fn events(&self) -> Vec<(String, String)> {
        let mut events: Vec<(u64, String, String)> = self
            .lock()
            .values()
            .filter(|object| object["kind"] == "Event")
            .map(|event| {
                (
                    event["metadata"]["resourceVersion"]
                        .as_str()
                        .and_then(|version| version.parse().ok())
                        .unwrap_or_default(),
                    event["reason"].as_str().unwrap_or_default().to_owned(),
                    event["note"].as_str().unwrap_or_default().to_owned(),
                )
            })
            .collect();
        events.sort();
        events
            .into_iter()
            .map(|(_, reason, note)| (reason, note))
            .collect()
    }

    pub fn event_reasons(&self) -> Vec<String> {
        self.events()
            .into_iter()
            .map(|(reason, _)| reason)
            .collect()
    }

    fn find(&self, kind: &str, name: &str) -> Option<Value> {
        self.lock()
            .values()
            .find(|object| object["kind"] == kind && object["metadata"]["name"] == name)
            .cloned()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BTreeMap<String, Value>> {
        self.objects
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn next_version(&self) -> String {
        (self.version.fetch_add(1, Ordering::Relaxed) + 1).to_string()
    }

    fn handle(
        &self,
        method: &http::Method,
        path: &str,
        content_type: &str,
        body: &[u8],
    ) -> (u16, Value) {
        let body: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        match *method {
            http::Method::GET => self.read(path),
            http::Method::POST => self.create(path, body),
            http::Method::PUT => self.replace(path, body),
            http::Method::PATCH => match path.strip_suffix("/status") {
                Some(path) => self.patch(path, content_type, &json!({ "status": null }), body),
                None => self.patch(path, content_type, &Value::Null, body),
            },
            _ => status(405, "MethodNotAllowed"),
        }
    }

    fn read(&self, path: &str) -> (u16, Value) {
        let objects = self.lock();
        if let Some(object) = objects.get(path) {
            return (200, object.clone());
        }
        let items: Vec<Value> = objects
            .iter()
            .filter(|(candidate, _)| in_collection(candidate, path))
            .map(|(_, object)| object.clone())
            .collect();
        if items.is_empty() && !is_collection(path) {
            return status(404, "NotFound");
        }
        (
            200,
            json!({ "kind": "List", "apiVersion": "v1", "metadata": { "resourceVersion": "1" }, "items": items }),
        )
    }

    fn create(&self, path: &str, mut object: Value) -> (u16, Value) {
        let name = object["metadata"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let key = format!("{path}/{name}");
        if self.lock().contains_key(&key) {
            return status(409, "AlreadyExists");
        }
        object["metadata"]["resourceVersion"] = json!(self.next_version());
        self.lock().insert(key, object.clone());
        (201, object)
    }

    fn replace(&self, path: &str, mut object: Value) -> (u16, Value) {
        let mut objects = self.lock();
        let Some(current) = objects.get_mut(path) else {
            return status(404, "NotFound");
        };
        if object["metadata"]["resourceVersion"] != current["metadata"]["resourceVersion"] {
            return status(409, "Conflict");
        }
        object["metadata"]["resourceVersion"] = json!(self.next_version());
        *current = object.clone();
        (200, object)
    }

    fn patch(&self, path: &str, content_type: &str, only: &Value, changes: Value) -> (u16, Value) {
        let mut objects = self.lock();
        let Some(object) = objects.get_mut(path) else {
            return status(404, "NotFound");
        };
        let expected = changes.pointer("/metadata/resourceVersion").cloned();
        if expected
            .as_ref()
            .is_some_and(|expected| *expected != object["metadata"]["resourceVersion"])
        {
            return status(409, "Conflict");
        }
        let mut patched = object.clone();
        let applied = if content_type.contains("json-patch") {
            serde_json::from_value::<json_patch::Patch>(changes)
                .map_err(|error| error.to_string())
                .and_then(|patch| {
                    json_patch::patch(&mut patched, &patch).map_err(|error| error.to_string())
                })
        } else {
            json_patch::merge(&mut patched, &changes);
            Ok(())
        };
        if applied.is_err() {
            return status(422, "Invalid");
        }
        if only.get("status").is_some() {
            object["status"] = patched["status"].clone();
        } else {
            let status = object.get("status").cloned();
            *object = patched;
            if let Some(status) = status {
                object["status"] = status;
            }
        }
        object["metadata"]["resourceVersion"] = json!(self.next_version());
        let result = object.clone();
        if object["metadata"].get("deletionTimestamp").is_some() && !has_finalizers(object) {
            objects.remove(path);
        }
        (200, result)
    }
}

fn has_finalizers(object: &Value) -> bool {
    object["metadata"]["finalizers"]
        .as_array()
        .is_some_and(|finalizers| !finalizers.is_empty())
}

fn is_collection(path: &str) -> bool {
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    let after_version = match segments.first() {
        Some(&"api") => 2,
        _ => 3,
    };
    match segments.get(after_version) {
        Some(&"namespaces") => segments.len() == after_version + 3,
        _ => segments.len() == after_version + 1,
    }
}

fn in_collection(object_path: &str, collection: &str) -> bool {
    let (prefix, plural) = collection.rsplit_once('/').unwrap_or_default();
    object_path.starts_with(&format!("{collection}/"))
        || (object_path.starts_with(&format!("{prefix}/namespaces/"))
            && object_path.contains(&format!("/{plural}/"))
            && !collection.contains("/namespaces/"))
}

fn status(code: u16, reason: &str) -> (u16, Value) {
    (
        code,
        json!({ "kind": "Status", "apiVersion": "v1", "status": "Failure", "reason": reason, "code": code }),
    )
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
    match field("/metadata/namespace") {
        "" => format!("{prefix}/{plural}/{}", field("/metadata/name")),
        namespace => format!(
            "{prefix}/namespaces/{namespace}/{plural}/{}",
            field("/metadata/name")
        ),
    }
}

/// `koof::fake` serving [`Record`]s on a local port; returns its base URL.
#[cfg(feature = "fake")]
pub async fn provider() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("port");
    let url = format!("http://{}", listener.local_addr().expect("address"));
    tokio::spawn(
        axum::serve(
            listener,
            crate::fake::Fake::new().resource::<Record>().router(),
        )
        .into_future(),
    );
    url
}
