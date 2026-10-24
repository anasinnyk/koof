#![allow(dead_code, clippy::expect_used)]

use enumset::EnumSet;
use koof_gen::{
    generator::generate,
    ir::{extract, validate},
    loader::load_document,
    spec,
};
use roas_overlay::{
    apply::{Apply, ApplyOptions},
    v1_1::Overlay,
};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/widgets")
        .join(name)
}

/// The widgets API with its provider, account and widget overlays.
pub fn widgets() -> Value {
    load_document(
        &fixture("openapi.yaml"),
        &[
            fixture("provider.overlay.yaml"),
            fixture("account.overlay.yaml"),
            fixture("widget.overlay.yaml"),
        ],
    )
    .expect("fixture")
}

/// [`widgets`] with one more overlay, given as its `actions` list in YAML.
pub fn widgets_with(actions: &str) -> Value {
    let overlay: Overlay = serde_yaml::from_str(&format!(
        "overlay: 1.1.0\ninfo:\n  title: test\n  version: 0.0.1\nactions:\n{actions}"
    ))
    .expect("overlay");
    let mut document = widgets();
    overlay
        .apply(&mut document, EnumSet::only(ApplyOptions::ErrorOnZeroMatch))
        .expect("overlay applies");
    document
}

/// Runs the generator like the CLI and returns the generated files by path, or
/// the error with its causes.
pub fn generated(document: &Value) -> Result<BTreeMap<String, String>, String> {
    let prepared =
        spec::prepare(document).map_err(|error| format!("{:#}", anyhow::Error::from(error)))?;
    let ir = extract(&prepared).map_err(|error| format!("{:#}", anyhow::Error::from(error)))?;
    let codegen = spec::for_codegen(&prepared)
        .map_err(|error| format!("{:#}", anyhow::Error::from(error)))?;
    let provider = validate(&ir, &codegen.late_initialize).map_err(|errors| {
        errors
            .iter()
            .map(|error| format!("{}: {}", error.location, error.message))
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let files = generate(&provider, &codegen.document)
        .map_err(|error| format!("{:#}", anyhow::Error::from(error)))?;
    Ok(files
        .into_iter()
        .map(|file| (file.path.display().to_string(), file.content))
        .collect())
}

/// The schema set typify generates the API types from.
pub fn codegen(document: &Value) -> spec::CodegenSpec {
    spec::for_codegen(&spec::prepare(document).expect("prepare")).expect("codegen")
}
