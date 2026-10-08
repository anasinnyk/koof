use anyhow::{Context, Result};
use enumset::EnumSet;
use roas_overlay::{
    apply::{Apply, ApplyOptions},
    v1_1::Overlay,
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Reads an `OpenAPI` document and applies overlays in the given order.
///
/// # Errors
///
/// Returns an error when a file cannot be read or parsed, or an overlay action
/// matches nothing.
pub fn load_document(openapi_path: &Path, overlay_paths: &[PathBuf]) -> Result<Value> {
    let source = fs::read_to_string(openapi_path)
        .with_context(|| format!("cannot read {}", openapi_path.display()))?;

    let mut openapi: Value = serde_yaml::from_str(&source).context("cannot parse OpenAPI")?;

    for overlay_path in overlay_paths {
        let source = fs::read_to_string(overlay_path)
            .with_context(|| format!("cannot read {}", overlay_path.display()))?;

        let overlay: Overlay = serde_yaml::from_str(&source)
            .with_context(|| format!("cannot parse {}", overlay_path.display()))?;

        let options: EnumSet<ApplyOptions> = ApplyOptions::ErrorOnZeroMatch.into();

        overlay
            .apply(&mut openapi, options)
            .map_err(|error| anyhow::anyhow!("cannot apply {}: {error}", overlay_path.display()))?;
    }

    Ok(openapi)
}
