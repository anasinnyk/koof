use koof_gen::loader::load_document;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn overlays_are_applied_in_order() {
    let document = load_document(
        &fixture("widgets/openapi.yaml"),
        &[fixture("widgets/provider.overlay.yaml")],
    )
    .expect("loaded");

    assert_eq!(document["x-kube-provider"]["name"], "widgets");
}

#[test]
fn overlay_that_matches_nothing_is_an_error() {
    let error = load_document(
        &fixture("widgets/openapi.yaml"),
        &[fixture("stale.overlay.yaml")],
    )
    .expect_err("stale overlay");

    assert!(format!("{error:#}").contains("stale.overlay.yaml"));
}

#[test]
fn missing_document_is_named_in_the_error() {
    let error = load_document(&fixture("missing.yaml"), &[]).expect_err("missing");

    assert!(error.to_string().starts_with("cannot read "));
}
