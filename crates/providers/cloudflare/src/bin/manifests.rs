use cloudflare_provider::generated::{PROVIDER, crds, docs};

fn main() -> std::io::Result<()> {
    koof::manifests::run(&PROVIDER, &crds(), &docs())
}
