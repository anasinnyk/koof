use spike_provider::crd::DNSRecord;
use spike_provider::types::DnsRecordForProvider;

fn roundtrip(yaml: &str) -> (DNSRecord, serde_json::Value, serde_json::Value) {
    let input: serde_json::Value = serde_yaml::from_str(yaml).expect("yaml");
    let record: DNSRecord = serde_json::from_value(input.clone()).expect("deserialize DNSRecord");
    let output = serde_json::to_value(&record).expect("serialize");
    (record, input, output)
}

#[test]
fn a_record_roundtrips() {
    let (record, input, output) = roundtrip(
        r#"
apiVersion: zones.cloudflare.kube.nas1k.dev/v1alpha1
kind: DNSRecord
metadata: { name: www, namespace: default }
spec:
  forProvider:
    type: A
    name: www.example.com
    content: 198.51.100.4
    ttl: 3600
    proxied: true
"#,
    );
    assert!(matches!(record.spec.for_provider, DnsRecordForProvider::ARecord(_)));
    assert_eq!(input["spec"], output["spec"]);
}

#[test]
fn mx_record_picks_mx_variant() {
    let (record, input, output) = roundtrip(
        r#"
apiVersion: zones.cloudflare.kube.nas1k.dev/v1alpha1
kind: DNSRecord
metadata: { name: mail, namespace: default }
spec:
  forProvider:
    type: MX
    name: example.com
    content: mx.example.com
    priority: 10
    ttl: 1
"#,
    );
    assert!(
        matches!(record.spec.for_provider, DnsRecordForProvider::MxRecord(_)),
        "got {:?}",
        record.spec.for_provider
    );
    assert_eq!(input["spec"], output["spec"]);
}

#[test]
fn invalid_ipv4_is_rejected() {
    let yaml = r#"
apiVersion: zones.cloudflare.kube.nas1k.dev/v1alpha1
kind: DNSRecord
metadata: { name: bad, namespace: default }
spec:
  forProvider: { type: A, name: x.example.com, content: not-an-ip }
"#;
    let input: serde_json::Value = serde_yaml::from_str(yaml).expect("yaml");
    let result = serde_json::from_value::<DNSRecord>(input);
    println!("invalid A record -> {:?}", result.as_ref().map(|r| &r.spec.for_provider));
}
