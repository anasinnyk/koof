#!/usr/bin/env bash
# Measures line coverage of the unit tests of koof and koof-gen with cargo-llvm-cov
# (needs llvm-cov and llvm-profdata: rustup component llvm-tools-preview, or
# LLVM_COV / LLVM_PROFDATA).
#
# Usage: scripts/coverage.sh [badges-dir]
#   prints the coverage per crate; with badges-dir also writes <crate>.json there
#   for https://shields.io/badges/endpoint-badge
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
crates=(koof koof-gen)

summary="$(cargo llvm-cov --manifest-path "$root/Cargo.toml" --all-features --locked \
  -p koof -p koof-gen --ignore-filename-regex 'src/testing\.rs$' --json --summary-only)"

percent() {
  jq -r --arg crate "/crates/$1/src/" '
    [.data[0].files[] | select(.filename | contains($crate)) | .summary.lines]
    | (map(.covered) | add) as $covered | (map(.count) | add) as $count
    | if $count > 0 then $covered * 100 / $count | floor else 0 end
  ' <<<"$summary"
}

color() {
  if (( $1 >= 80 )); then echo brightgreen
  elif (( $1 >= 60 )); then echo yellowgreen
  elif (( $1 >= 40 )); then echo yellow
  else echo red
  fi
}

for crate in "${crates[@]}"; do
  value="$(percent "$crate")"
  echo "$crate: $value% of lines"
  if [[ $# -ge 1 ]]; then
    mkdir -p "$1"
    jq -n --arg label "$crate coverage" --arg message "$value%" --arg color "$(color "$value")" \
      '{schemaVersion: 1, label: $label, message: $message, color: $color}' >"$1/$crate.json"
  fi
done
