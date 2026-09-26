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
done
