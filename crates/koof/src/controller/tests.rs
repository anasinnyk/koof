use super::*;
use crate::{
    external::http::HttpClient,
    metrics::Metrics,
    testing::{
        ACCOUNT, Cluster, RECORD, Record, ZONE, Zone, account, object, provider, secret, zone,
    },
};
use kube::runtime::events::{Recorder, Reporter};
use pretty_assertions::assert_eq;
use serde_json::{Value, json};

struct World {
    cluster: Cluster,
    context: Arc<ControllerContext>,
    provider: String,
}

async fn world(record: Value) -> World {
    world_with(&[
        zone(),
        account(),
        secret("cloudflare", "token", "t0ken"),
        record,
    ])
    .await
}

async fn world_with(objects: &[Value]) -> World {
    let provider = provider().await;
    let cluster = Cluster::new(objects);
    let client = cluster.client();
    let reporter = Reporter {
        controller: "koof-test".to_owned(),
        instance: None,
    };
    let context = Arc::new(ControllerContext::new(
        client.clone(),
        HttpClient::new(reqwest::Client::new(), provider.clone()),
        Recorder::new(client, reporter),
        Arc::new(Metrics::new()),
        vec![&ACCOUNT, &ZONE, &RECORD],
        1,
    ));
    World {
        cluster,
        context,
        provider,
    }
}

fn www(spec: Value) -> Value {
    object("Record", "www", None, spec)
}

fn for_provider(fields: &Value) -> Value {
    let mut for_provider = json!({ "zoneRef": { "name": "example" } });
    json_patch::merge(&mut for_provider, fields);
    json!({ "forProvider": for_provider })
}

impl World {
    async fn reconcile_record(&self) -> Result<Action, ReconcileError> {
        reconcile(
            Arc::new(self.cluster.typed::<Record>("Record", "www")),
            self.context.clone(),
        )
        .await
    }

    async fn reconcile_zone(&self) -> Result<Action, ReconcileError> {
        reconcile(
            Arc::new(self.cluster.typed::<Zone>("Zone", "example")),
            self.context.clone(),
        )
        .await
    }

    async fn reconcile_record_times(&self, times: usize) {
        for _ in 0..times {
            self.reconcile_record().await.expect("reconciled");
        }
    }

    fn record(&self) -> Option<Value> {
        self.cluster.get("Record", "www")
    }

    fn condition(&self, kind: &str, name: &str, type_: &str) -> Option<(String, String)> {
        let object = self.cluster.get(kind, name)?;
        object["status"]["conditions"]
            .as_array()?
            .iter()
            .find(|condition| condition["type"] == type_)
            .map(|condition| {
                (
                    condition["status"].as_str().unwrap_or_default().to_owned(),
                    condition["reason"].as_str().unwrap_or_default().to_owned(),
                )
            })
    }

    fn ready(&self) -> Option<(String, String)> {
        self.condition("Record", "www", "Ready")
    }

    fn external_name(&self) -> Option<String> {
        self.record()?["metadata"]["annotations"][RECORD.annotation(EXTERNAL_NAME)]
            .as_str()
            .map(str::to_owned)
    }

    async fn at_provider(&self) -> Value {
        let state: Value = reqwest::get(format!("{}/__fake/state", self.provider))
            .await
            .expect("state")
            .json()
            .await
            .expect("JSON");
        state["example.dev/Record"].clone()
    }

    fn set_spec(&self, fields: &Value) {
        let mut record = self.record().expect("record");
        json_patch::merge(&mut record["spec"]["forProvider"], fields);
        record["metadata"]["generation"] = json!(2);
        self.cluster.put(record);
    }
}

fn pair(status: &str, reason: &str) -> (String, String) {
    (status.to_owned(), reason.to_owned())
}

const ID: &str = "00000000000000000000000000000001";

#[tokio::test]
async fn creates_a_missing_record_and_reports_it_ready() {
    let world = world(www(for_provider(
        &json!({ "name": "www.example.com", "ttl": 300 }),
    )))
    .await;

    world.reconcile_record_times(2).await;

    assert_eq!(world.external_name().as_deref(), Some(ID));
    assert_eq!(world.ready(), Some(pair("True", "Available")));
    assert_eq!(world.cluster.event_reasons(), ["Created", "Available"]);
    assert_eq!(
        world.at_provider().await[format!("zone-1/{ID}")]["ttl"],
        json!(300)
    );
}

#[tokio::test]
async fn adds_its_finalizer_before_touching_the_provider() {
    let world = world(www(for_provider(
        &json!({ "name": "www.example.com", "ttl": 300 }),
    )))
    .await;

    world.reconcile_record().await.expect("reconciled");

    assert_eq!(
        world
            .record()
            .map(|record| record["metadata"]["finalizers"].clone()),
        Some(json!(["example.dev/finalizer"]))
    );
}

#[tokio::test]
async fn copies_fields_the_user_left_unset_from_the_provider() {
    let world = world(www(for_provider(&json!({ "ttl": 300 })))).await;

    world.reconcile_record_times(2).await;

    assert_eq!(
        world
            .record()
            .map(|record| record["spec"]["forProvider"]["name"].clone()),
        Some(json!(ID))
    );
    assert_eq!(
        world.cluster.event_reasons(),
        ["Created", "LateInitialized"]
    );
}

#[tokio::test]
async fn corrects_drift_by_updating_the_provider() {
    let world = world(www(for_provider(
        &json!({ "name": "www.example.com", "ttl": 300 }),
    )))
    .await;
    world.reconcile_record_times(2).await;

    world.set_spec(&json!({ "ttl": 60 }));
    world.reconcile_record().await.expect("reconciled");

    assert_eq!(
        world.at_provider().await[format!("zone-1/{ID}")]["ttl"],
        json!(60)
    );
    assert_eq!(
        world.cluster.events().last().cloned(),
        Some((
            "Updated".to_owned(),
            "External resource updated: ttl".to_owned()
        ))
    );
}

#[tokio::test]
async fn deleting_the_object_deletes_the_external_record() {
    let world = world(www(for_provider(
        &json!({ "name": "www.example.com", "ttl": 300 }),
    )))
    .await;
    world.reconcile_record_times(2).await;

    world.cluster.delete("Record", "www");
    world.reconcile_record_times(2).await;

    assert_eq!(
        (world.record(), world.at_provider().await),
        (None, Value::Null)
    );
    assert_eq!(
        world.cluster.event_reasons(),
        ["Created", "Available", "Deleted"]
    );
}

#[tokio::test]
async fn orphan_policy_keeps_the_external_record() {
    let mut spec = for_provider(&json!({ "name": "www.example.com", "ttl": 300 }));
    spec["deletionPolicy"] = json!("Orphan");
    let world = world(www(spec)).await;
    world.reconcile_record_times(2).await;

    world.cluster.delete("Record", "www");
    world.reconcile_record().await.expect("reconciled");

    assert_eq!(world.record(), None);
    assert_eq!(
        world.at_provider().await[format!("zone-1/{ID}")]["ttl"],
        json!(300)
    );
    assert_eq!(
        world.cluster.events().last().cloned(),
        Some((
            "Orphaned".to_owned(),
            "External resource kept (orphaned)".to_owned()
        ))
    );
}

#[tokio::test]
async fn an_import_only_observes() {
    let mut spec = for_provider(&json!({ "name": "www.example.com" }));
    spec["managementPolicies"] = json!(["Observe"]);
    let world = world(www(spec)).await;

    world.reconcile_record().await.expect("reconciled");

    assert_eq!(world.at_provider().await, Value::Null);
    assert_eq!(world.ready(), Some(pair("False", "Unavailable")));
}

#[tokio::test]
async fn zone_deletion_waits_for_its_records() {
    let mut deleting_zone = zone();
    deleting_zone["metadata"]["finalizers"] = json!(["example.dev/finalizer"]);
    deleting_zone["metadata"]["deletionTimestamp"] = json!("2026-01-01T00:00:00Z");
    let world = world_with(&[deleting_zone, www(for_provider(&json!({})))]).await;

    world.reconcile_zone().await.expect("reconciled");

    assert_eq!(
        world.condition("Zone", "example", "Ready"),
        Some(pair("False", "InUse"))
    );
    assert_eq!(
        world.cluster.events(),
        [(
            "InUse".to_owned(),
            "Deletion waits until these resources are deleted: Record default/www".to_owned()
        )]
    );
}

#[tokio::test]
async fn an_interrupted_create_is_not_repeated() {
    let mut record = www(for_provider(&json!({ "name": "www.example.com" })));
    record["metadata"]["annotations"][RECORD.annotation(create_state::CREATE_PENDING)] =
        json!("2026-01-01T00:00:00Z");
    let world = world(record).await;

    let error = world.reconcile_record().await.expect_err("create pending");

    assert!(matches!(error, ReconcileError::CreatePending { .. }));
    assert_eq!(world.at_provider().await, Value::Null);
    assert_eq!(
        world.condition("Record", "www", "Synced"),
        Some(pair("False", "ReconcileError"))
    );
}

#[tokio::test]
async fn waits_until_the_zone_has_an_id() {
    let mut zone_without_id = zone();
    zone_without_id["metadata"]["annotations"] = json!({});
    let world = world_with(&[
        zone_without_id,
        account(),
        secret("cloudflare", "token", "t0ken"),
        www(for_provider(&json!({ "name": "www" }))),
    ])
    .await;

    let error = world.reconcile_record().await.expect_err("zone not ready");

    assert_eq!(retry::classify(&error), retry::Failure::Waiting);
    assert_eq!(
        world.condition("Record", "www", "Synced"),
        Some(pair("False", "ReconcileError"))
    );
}
