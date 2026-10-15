use crate::app::ProviderInfo;
use k8s_openapi::api::rbac::v1::{ClusterRole, PolicyRule};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::Serialize;
use std::{collections::BTreeMap, fs, io, path::Path};

pub const NAME: &str = "provider";

#[must_use]
pub fn cluster_role(crds: &[CustomResourceDefinition]) -> ClusterRole {
    let mut groups: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for crd in crds {
        groups
            .entry(crd.spec.group.as_str())
            .or_default()
            .push(crd.spec.names.plural.as_str());
    }
    let rule = |group: &str, resources: Vec<String>, verbs: &[&str]| PolicyRule {
        api_groups: Some(vec![group.to_owned()]),
        resources: Some(resources),
        verbs: verbs.iter().map(|verb| (*verb).to_owned()).collect(),
        ..PolicyRule::default()
    };
    let mut rules = Vec::new();
    for (group, plurals) in groups {
        let with = |suffix: &str| -> Vec<String> {
            plurals
                .iter()
                .map(|plural| format!("{plural}{suffix}"))
                .collect()
        };
        rules.push(rule(
            group,
            with(""),
            &["get", "list", "watch", "patch", "update"],
        ));
        rules.push(rule(group, with("/status"), &["get", "patch", "update"]));
        rules.push(rule(group, with("/finalizers"), &["update"]));
    }
    rules.push(rule("", vec!["secrets".to_owned()], &["get"]));
    rules.push(rule("events.k8s.io", vec!["events".to_owned()], &["create", "patch"]));
    ClusterRole {
        metadata: ObjectMeta {
            name: Some(NAME.to_owned()),
            ..ObjectMeta::default()
        },
        rules: Some(rules),
        ..ClusterRole::default()
    }
}

/// # Errors
///
/// Returns an error when a document cannot be serialized.
pub fn documents<T: Serialize>(items: &[T]) -> Result<String, serde_json::Error> {
    let mut out = String::new();
    for item in items {
        out.push_str("---\n");
        out.push_str(&serde_json::to_string_pretty(item)?);
        out.push('\n');
    }
    Ok(out)
}

#[must_use]
pub fn kustomization(provider: &ProviderInfo, image: &str) -> String {
    let name = provider.name;
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
namespace: koof-{name}
namePrefix: koof-{name}-
labels:
  - pairs:
      app.kubernetes.io/instance: koof-{name}
    includeSelectors: true
resources:
  - ../../base
  - crds.yaml
  - rbac.yaml
images:
  - name: {NAME}
    newName: {image}
    newTag: \"{version}\"
"
    )
}

/// # Errors
///
/// Returns an error when a manifest cannot be serialized or written.
pub fn write(
    directory: &Path,
    provider: &ProviderInfo,
    crds: &[CustomResourceDefinition],
    image: &str,
) -> io::Result<()> {
    fs::create_dir_all(directory)?;
    fs::write(directory.join("crds.yaml"), documents(crds).map_err(io::Error::other)?)?;
    fs::write(
        directory.join("rbac.yaml"),
        documents(&[cluster_role(crds)]).map_err(io::Error::other)?,
    )?;
    fs::write(
        directory.join("kustomization.yaml"),
        kustomization(provider, image),
    )?;
    Ok(())
}

/// # Errors
///
/// Returns an error on wrong arguments or when the manifests cannot be written.
pub fn run(provider: &ProviderInfo, crds: &[CustomResourceDefinition]) -> io::Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.as_slice() {
        [] => {
            print!("{}", documents(crds).map_err(io::Error::other)?);
            Ok(())
        }
        [directory, image] => write(Path::new(directory), provider, crds, image),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: manifests [<directory> <image>]",
        )),
    }
}
