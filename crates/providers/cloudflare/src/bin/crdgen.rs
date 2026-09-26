fn main() -> anyhow::Result<()> {
    for crd in cloudflare_provider::generated::crds() {
        println!("---");
        serde_yaml::to_writer(std::io::stdout(), &crd)?;
    }
    Ok(())
}
