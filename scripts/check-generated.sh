#!/usr/bin/env bash
# Fails when tracked generated code of any provider differs from a fresh regeneration.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

for dir in "$root"/crates/providers/*/; do
  provider="$(basename "$dir")"
  [[ -f "$dir/openapi/sources" ]] || continue
  "$root/scripts/regen.sh" "$provider" "$tmp/$provider"
  diff -ru "$dir/src/generated" "$tmp/$provider/src/generated"
  if [[ -d "$dir/e2e/generated" || -d "$tmp/$provider/e2e/generated" ]]; then
    diff -ru "$dir/e2e/generated" "$tmp/$provider/e2e/generated"
  fi
  cargo run --quiet --manifest-path "$root/Cargo.toml" -p "$provider-provider" --bin manifests -- \
    "$tmp/$provider/deploy" "ghcr.io/anasinnyk/koof-$provider"
  diff -ru "$root/deploy/providers/$provider" "$tmp/$provider/deploy"
done
