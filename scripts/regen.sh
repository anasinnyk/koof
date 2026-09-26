#!/usr/bin/env bash
# Regenerates the Cloudflare provider from OpenAPI + overlays.
# Usage: scripts/regen.sh [output-dir]   (default: crates/providers/cloudflare)
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
provider="$root/crates/providers/cloudflare"
output="${1:-$provider}"

cargo run --quiet --manifest-path "$root/Cargo.toml" -p koof-gen -- \
  --openapi "$provider/openapi/cloudflare.yaml" \
  --overlay "$provider/openapi/cloudflare.account.overlay.yaml" \
  --overlay "$provider/openapi/cloudflare.zone.overlay.yaml" \
  --overlay "$provider/openapi/cloudflare.dns-record.overlay.yaml" \
  --overlay "$provider/openapi/cloudflare.ruleset.overlay.yaml" \
  --overlay "$provider/openapi/cloudflare.pages-project.overlay.yaml" \
  --overlay "$provider/openapi/cloudflare.typify.overlay.yaml" \
  --output "$output"

# Generated files are formatted by prettyplease inside koof-gen; rustfmt is not
# idempotent on typify's block doc comments, so it is intentionally not run here.
