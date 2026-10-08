use crate::{
    generator::generate,
    ir::{extract, validate},
    loader::load_document,
    spec,
};
use anyhow::{Context, Result, bail};
use clap::Parser;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(name = "koof-gen")]
#[command(about = "Generate Kubernetes provider code from OpenAPI")]
pub struct Args {
    /// Source `OpenAPI` YAML or JSON file.
    #[arg(short = 'i', long, value_name = "FILE")]
    openapi: PathBuf,

    /// Overlay files applied in the specified order.
    #[arg(long, value_name = "FILE")]
    overlay: Vec<PathBuf>,

    /// Directory for generated files.
    #[arg(short = 'o', long, value_name = "DIRECTORY")]
    output: PathBuf,

    /// Also write the preprocessed document (pruned, normalized, with synthesized
    /// `forProvider` / `atProvider` schemas) as JSON, for inspection.
    #[arg(long, value_name = "FILE")]
    preprocessed_spec: Option<PathBuf>,
}

/// Runs the generator CLI.
///
/// # Errors
///
/// Returns an error when loading, validation or generation fails, or a generated file
/// cannot be written.
pub fn run() -> Result<()> {
    let args = Args::parse();

    let document = load_document(&args.openapi, &args.overlay)?;
    let prepared = spec::prepare(&document)?;
    let ir = extract(&prepared)?;
    let codegen = spec::for_codegen(&prepared)?;
    if let Some(path) = &args.preprocessed_spec {
        fs::write(path, serde_json::to_string_pretty(&codegen.document)?)
            .with_context(|| format!("cannot write {}", path.display()))?;
    }

    let provider = match validate(&ir, &codegen.late_initialize) {
        Ok(provider) => provider,
        Err(errors) => bail!("invalid ProviderIr:\n{errors:#?}"),
    };
    let files = generate(&provider, &codegen.document)?;

    for file in files {
        write_generated_file(&args.output, &file.path, &file.content)?;
    }

    Ok(())
}

fn write_generated_file(output: &Path, relative_path: &Path, content: &str) -> Result<()> {
    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        bail!(
            "generated path must be relative: {}",
            relative_path.display()
        );
    }

    let path = output.join(relative_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("cannot create {}", parent.display()))?;
    }

    fs::write(&path, content).with_context(|| format!("cannot write {}", path.display()))?;

    Ok(())
}
