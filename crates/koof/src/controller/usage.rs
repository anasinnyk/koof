use super::ControllerContext;
use crate::{
    error::ReconcileError,
    external::resolve,
    resource::{ItemReference, ManagedResource, Reference},
};
use kube::{
    ResourceExt as _,
    api::{Api, DynamicObject, ListParams},
};
use serde_json::Value;

pub(super) async fn dependents<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Vec<String>, ReconcileError> {
    let target = R::descriptor();
    let name = resource.name_any();
    let namespace = resource.namespace();
    let mut dependents = Vec::new();
    for descriptor in &context.descriptors {
        let points_at_target = |reference: &&Reference| {
            reference
                .targets
                .iter()
                .any(|candidate| std::ptr::eq(*candidate, target))
        };
        let references: Vec<&Reference> = descriptor
            .references
            .iter()
            .filter(points_at_target)
            .collect();
        let item_references: Vec<&ItemReference> = descriptor
            .item_references
            .iter()
            .filter(|item_reference| points_at_target(&&item_reference.reference))
            .collect();
        if references.is_empty() && item_references.is_empty() {
            continue;
        }
        let api_resource = (descriptor.api_resource)();
        let api: Api<DynamicObject> = match (descriptor.namespaced, namespace.as_deref()) {
            (true, Some(namespace)) => {
                Api::namespaced_with(context.kube_client.clone(), namespace, &api_resource)
            }
            (false, Some(_)) => continue,
            (_, None) => Api::all_with(context.kube_client.clone(), &api_resource),
        };
        for object in api.list(&ListParams::default()).await? {
            let is_target = std::ptr::eq(*descriptor, target)
                && object.name_any() == name
                && object.namespace() == namespace;
            let value = serde_json::to_value(&object)?;
            let refers = |item: &Value, reference: &Reference| {
                matches!(
                    resolve::target(item, reference),
                    Ok(Some((kind, object_name))) if std::ptr::eq(kind, target) && object_name == name
                )
            };
            let uses_target = references.iter().any(|reference| refers(&value, reference))
                || item_references.iter().any(|item_reference| {
                    value
                        .pointer(item_reference.list)
                        .and_then(Value::as_array)
                        .is_some_and(|items| {
                            items
                                .iter()
                                .any(|item| refers(item, &item_reference.reference))
                        })
                });
            if uses_target && !is_target {
                dependents.push(format!(
                    "{} {}/{}",
                    descriptor.kind,
                    object.namespace().unwrap_or_default(),
                    object.name_any()
                ));
            }
        }
    }
    Ok(dependents)
}
