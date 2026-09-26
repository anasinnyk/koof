//! Prints every CRD that kube can derive; reports the ones whose derive panics.
use kube::CustomResourceExt;
use spike_provider::crd::{Account, DNSRecord, PagesProject, Ruleset, Zone};

type CrdFn = fn() -> k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let all: [(&str, CrdFn); 5] = [
        ("Account", Account::crd),
        ("Zone", Zone::crd),
        ("DNSRecord", DNSRecord::crd),
        ("Ruleset", Ruleset::crd),
        ("PagesProject", PagesProject::crd),
    ];
    for (kind, crd) in all {
        match std::panic::catch_unwind(crd) {
            Ok(crd) => match serde_yaml::to_string(&crd) {
                Ok(yaml) => println!("---\n{yaml}"),
                Err(e) => eprintln!("{kind}: yaml error {e}"),
            },
            Err(panic) => {
                let message = panic
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                    .unwrap_or_default();
                let short: String = message.chars().take(300).collect();
                eprintln!("{kind}: derive panicked: {short}");
            }
        }
    }
}
