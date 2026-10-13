fn main() -> anyhow::Result<()> {
    let list = serde_json::json!({
        "apiVersion": "v1",
        "kind": "List",
        "items": sentry_provider::generated::crds(),
    });
    println!("{}", serde_json::to_string_pretty(&list)?);
    Ok(())
}
