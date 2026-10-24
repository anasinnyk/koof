#!/usr/bin/env bash
# Regenerates providers from their OpenAPI document and overlays, listed in
# crates/providers/<provider>/openapi/sources (document first, then overlays in order).
#
# Usage: scripts/regen.sh [provider [output-dir]]
#   no arguments      — every provider in place
#   provider          — one provider in place
#   provider output   — one provider into output-dir (used by check-generated.sh)
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"

regen() {
  local provider="$1" output="$2"
  local dir="$root/crates/providers/$provider/openapi"
  local sources=()
  while IFS= read -r line; do
    [[ -z "$line" || "$line" == \#* ]] && continue
    sources+=("$dir/$line")
  done < "$dir/sources"

  local args=(--openapi "${sources[0]}")
  for overlay in "${sources[@]:1}"; do
    args+=(--overlay "$overlay")
  done

  cargo run --quiet --manifest-path "$root/Cargo.toml" -p koof-gen -- "${args[@]}" --output "$output"
}

manifests() {
  local provider="$1" output="$2"
  cargo run --quiet --manifest-path "$root/Cargo.toml" -p "$provider-provider" --bin "koof-$provider-manifests" -- \
    "$output" "ghcr.io/anasinnyk/koof-$provider"
}

if [[ $# -ge 2 ]]; then
  regen "$1" "$2"
elif [[ $# -eq 1 ]]; then
  regen "$1" "$root/crates/providers/$1"
  manifests "$1" "$root/deploy/providers/$1"
else
  for dir in "$root"/crates/providers/*/; do
    provider="$(basename "$dir")"
    [[ -f "$dir/openapi/sources" ]] || continue
    regen "$provider" "$dir"
    manifests "$provider" "$root/deploy/providers/$provider"
  done
fi

# Generated files are formatted by prettyplease inside koof-gen; rustfmt is not
# idempotent on typify's block doc comments, so it is intentionally not run here.
