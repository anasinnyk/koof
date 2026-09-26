use kube::CustomResourceExt;
use spike_provider::crd::{Account, DNSRecord, PagesProject, Ruleset, Zone};

fn main() -> Result<(), serde_yaml::Error> {
    for crd in [Account::crd(), Zone::crd(), DNSRecord::crd(), Ruleset::crd(), PagesProject::crd()] {
        println!("---\n{}", serde_yaml::to_string(&crd)?);
    }
    Ok(())
}
