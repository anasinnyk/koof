use sentry_provider::generated::{PROVIDER, crds};

fn main() -> std::io::Result<()> {
    koof::manifests::run(&PROVIDER, &crds())
}
