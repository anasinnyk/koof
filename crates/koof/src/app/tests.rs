use super::*;
use crate::{
    resource::EXTERNAL_NAME,
    testing::{Cluster, PROVIDER, RECORD, Record, account, object, provider, secret, zone},
};
use pretty_assertions::assert_eq;
use serde_json::json;
use std::time::Duration;

struct Running {
    address: String,
    stop: tokio::sync::oneshot::Sender<()>,
    task: tokio::task::JoinHandle<Result<(), AppError>>,
}

impl Running {
    async fn get(&self, path: &str) -> (u16, String) {
        let response = reqwest::get(format!("http://{}{path}", self.address))
            .await
            .expect("response");
        (
            response.status().as_u16(),
            response.text().await.expect("body"),
        )
    }

    async fn shut_down(self) -> Result<(), AppError> {
        let _ = self.stop.send(());
        self.task.await.expect("app task")
    }
}

fn config(base_url: String) -> Config {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("port");
    let metrics_addr = listener.local_addr().expect("address").to_string();
    Config {
        base_url,
        metrics_addr,
        ..Config::defaults(&PROVIDER)
    }
}

fn leading(config: Config) -> Config {
    Config {
        pod_name: Some("pod-a".to_owned()),
        leader_election: crate::config::LeaderElectionConfig {
            enabled: true,
            namespace: Some("koof".to_owned()),
            ..config.leader_election.clone()
        },
        ..config
    }
}

async fn start(app: App, config: Config, cluster: &Cluster) -> Running {
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let address = config.metrics_addr.clone();
    let client = cluster.client();
    let task = tokio::spawn(app.run_with(config, client, async {
        let _ = stopped.await;
    }));
    let running = Running {
        address,
        stop,
        task,
    };
    wait_until(|| async {
        reqwest::get(format!("http://{}/healthz", running.address))
            .await
            .is_ok()
    })
    .await;
    running
}

async fn wait_until<F: Future<Output = bool>>(check: impl Fn() -> F) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while !check().await {
        assert!(
            tokio::time::Instant::now() < deadline,
            "condition not reached in time"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

fn records() -> Cluster {
    Cluster::new(&[
        zone(),
        account(),
        secret("cloudflare", "token", "t0ken"),
        object(
            "Record",
            "www",
            None,
            json!({ "forProvider": { "name": "www", "zoneRef": { "name": "example" } } }),
        ),
    ])
}

fn holder(cluster: &Cluster) -> Option<String> {
    cluster.get("Lease", "koof-example")?["spec"]["holderIdentity"]
        .as_str()
        .map(str::to_owned)
}

#[tokio::test]
async fn reconciles_the_resources_of_its_controllers() {
    let cluster = records();
    let app = App::new(&PROVIDER).controller::<Record>();

    let running = start(app, config(provider().await), &cluster).await;
    wait_until(|| async {
        cluster.get("Record", "www").is_some_and(|record| {
            record["metadata"]["annotations"]
                .get(RECORD.annotation(EXTERNAL_NAME))
                .is_some()
        })
    })
    .await;

    assert_eq!(running.get("/readyz").await, (200, "ok".to_owned()));
    assert!(
        running.get("/metrics").await.1.contains(
            r#"koof_actions_total{group="example.dev",kind="Record",action="created"} 1"#
        )
    );
    assert!(running.shut_down().await.is_ok());
}

#[tokio::test]
async fn is_live_and_ready_once_its_controllers_synced() {
    let running = start(
        App::new(&PROVIDER).controller::<Record>(),
        config(provider().await),
        &Cluster::default(),
    )
    .await;

    assert_eq!(running.get("/healthz").await, (200, "ok".to_owned()));
    assert_eq!(running.get("/readyz").await.0, 200);
    assert!(running.shut_down().await.is_ok());
}

#[tokio::test]
async fn standby_is_not_ready_while_another_instance_leads() {
    let cluster = Cluster::new(&[json!({
        "apiVersion": "coordination.k8s.io/v1",
        "kind": "Lease",
        "metadata": { "name": "koof-example", "namespace": "koof" },
        "spec": { "holderIdentity": "pod-b", "leaseDurationSeconds": 15 },
    })]);

    let running = start(
        App::new(&PROVIDER),
        leading(config(provider().await)),
        &cluster,
    )
    .await;

    assert_eq!(
        running.get("/readyz").await,
        (503, "not leading".to_owned())
    );
    assert!(running.shut_down().await.is_ok());
    assert_eq!(holder(&cluster).as_deref(), Some("pod-b"));
}

#[tokio::test]
async fn leader_releases_the_lease_on_shutdown() {
    let cluster = records();
    let app = App::new(&PROVIDER).controller::<Record>();
    let running = start(app, leading(config(provider().await)), &cluster).await;
    wait_until(|| async { holder(&cluster).as_deref() == Some("pod-a") }).await;

    assert!(running.shut_down().await.is_ok());
    assert_eq!(holder(&cluster), None);
}

#[tokio::test]
async fn metrics_address_in_use_is_an_error() {
    let taken = std::net::TcpListener::bind("127.0.0.1:0").expect("port");
    let config = Config {
        metrics_addr: taken.local_addr().expect("address").to_string(),
        ..config(provider().await)
    };

    let result = App::new(&PROVIDER)
        .run_with(config.clone(), Cluster::default().client(), async {})
        .await;

    assert_eq!(
        result.err().map(|error| error.to_string()),
        Some(format!("cannot listen on {}", config.metrics_addr))
    );
}
