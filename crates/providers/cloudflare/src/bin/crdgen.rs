//! Prints every `CustomResourceDefinition` of the provider as a JSON `v1 List`
//! (`kubectl apply -f` accepts it). JSON rather than YAML: YAML 1.1 parsers, such as
//! the API server's, read bare words like `y` or `on` (property names) as booleans.
fn main() -> anyhow::Result<()> {
    let list = serde_json::json!({
        "apiVersion": "v1",
        "kind": "List",
        "items": cloudflare_provider::generated::crds(),
    });
    println!("{}", serde_json::to_string_pretty(&list)?);
    Ok(())
}
