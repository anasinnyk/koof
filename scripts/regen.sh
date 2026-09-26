#!/usr/bin/env bash
# Regenerates the Cloudflare provider from OpenAPI + overlays.
# Usage: scripts/regen.sh [output-dir]   (default: crates/providers/cloudflare)
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
output="${1:-$root/crates/providers/cloudflare}"

cargo run --quiet --manifest-path "$root/Cargo.toml" -p koof-gen -- \
  --openapi "$root/openapi/cloudflare.yaml" \
  --overlay "$root/openapi/cloudflare.account.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.zone.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.dns-record.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.ruleset.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.pages-project.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.typify.overlay.yaml" \
  --output "$output"

# Generated files are formatted by prettyplease inside koof-gen; rustfmt is not
# idempotent on typify's block doc comments, so it is intentionally not run here.
