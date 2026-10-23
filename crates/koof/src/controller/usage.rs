use super::ControllerContext;
use crate::{
    error::ReconcileError,
    external::resolve,
    resource::{ManagedResource, Reference},
};
use kube::{
    ResourceExt as _,
    api::{Api, DynamicObject, ListParams},
};

pub(super) async fn dependents<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
) -> Result<Vec<String>, ReconcileError> {
    let target = R::descriptor();
    let name = resource.name_any();
    let namespace = resource.namespace();
    let mut dependents = Vec::new();
    for descriptor in &context.descriptors {
        let references: Vec<&Reference> = descriptor
            .references
            .iter()
            .filter(|reference| {
                reference
                    .targets
                    .iter()
                    .any(|candidate| std::ptr::eq(*candidate, target))
            })
            .collect();
        if references.is_empty() {
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
            let value = serde_json::to_value(&object)?;
            for reference in &references {
                if let Ok(Some((kind, referenced))) = resolve::target(&value, reference)
                    && std::ptr::eq(kind, target)
                    && referenced == name
                    && !(std::ptr::eq(*descriptor, target)
                        && object.name_any() == name
                        && object.namespace() == namespace)
                {
                    dependents.push(format!(
                        "{} {}/{}",
                        descriptor.kind,
                        object.namespace().unwrap_or_default(),
                        object.name_any()
                    ));
                    break;
                }
            }
        }
    }
    Ok(dependents)
}
