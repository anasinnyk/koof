#!/usr/bin/env bash
# Runs the end-to-end tests of providers (crates/providers/<provider>/e2e, generated
# from the x-kube-resource test fixtures) with chainsaw against a kind cluster. The
# controller and the fake provider API (koof::fake) run on the host; kind provides
# only the Kubernetes API.
#
# Usage: scripts/e2e.sh [provider...] [-- chainsaw-args...]
#   (default: every provider with e2e tests; e.g. -- --include-test-regex 'zone')
# Needs kind, kubectl and chainsaw (all in the devenv shell). KIND_CLUSTER names
# the cluster (default koof-e2e); it is created from cluster/k8s.yaml when missing.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cluster="${KIND_CLUSTER:-koof-e2e}"
logs="$root/target/e2e"
fake_port=18081
metrics_port=18080
mkdir -p "$logs"

if ! kind get clusters 2>/dev/null | grep -qx "$cluster"; then
  kind create cluster --name "$cluster" --config "$root/cluster/k8s.yaml"
fi
export KUBECONFIG="$logs/kubeconfig"
kind get kubeconfig --name "$cluster" >"$KUBECONFIG"

pids=()
stop() {
  if [[ ${#pids[@]} -gt 0 ]]; then
    kill "${pids[@]}" 2>/dev/null || true
    wait "${pids[@]}" 2>/dev/null || true
  fi
  pids=()
}
trap stop EXIT

run() {
  local provider="$1"
  local bin="$root/target/debug"

  cargo build --quiet --manifest-path "$root/Cargo.toml" -p "$provider-provider" --bins
  "$bin/crdgen" | kubectl apply --server-side --force-conflicts -f - >/dev/null
  kubectl wait --for condition=established crd --all --timeout=60s >/dev/null

  FAKE_ADDR="127.0.0.1:$fake_port" "$bin/fake" >"$logs/$provider-fake.log" 2>&1 &
  pids+=($!)
  PROVIDER_BASE_URL="http://127.0.0.1:$fake_port" METRICS_ADDR="127.0.0.1:$metrics_port" \
    "$bin/main" >"$logs/$provider-controller.log" 2>&1 &
  pids+=($!)

  local attempts=0
  until curl -sf "http://127.0.0.1:$metrics_port/readyz" >/dev/null; do
    if ((++attempts > 60)); then
      echo "$provider controller did not become ready; see $logs/$provider-controller.log" >&2
      return 1
    fi
    sleep 1
  done

  local status=0
  chainsaw test --config "$root/.chainsaw.yaml" --test-dir "$root/crates/providers/$provider/e2e" \
    "${chainsaw_args[@]+"${chainsaw_args[@]}"}" || status=$?
  stop
  if [[ $status -ne 0 ]]; then
    echo "logs: $logs/$provider-controller.log, $logs/$provider-fake.log" >&2
  fi
  return "$status"
}

providers=()
chainsaw_args=()
while [[ $# -gt 0 ]]; do
  if [[ "$1" == "--" ]]; then
    shift
    chainsaw_args=("$@")
    break
  fi
  providers+=("$1")
  shift
done
if [[ ${#providers[@]} -eq 0 ]]; then
  for dir in "$root"/crates/providers/*/; do
    [[ -d "$dir/e2e" ]] && providers+=("$(basename "$dir")")
  done
fi

failed=()
for provider in "${providers[@]}"; do
  run "$provider" || failed+=("$provider")
done
if [[ ${#failed[@]} -gt 0 ]]; then
  echo "e2e failed: ${failed[*]}" >&2
  exit 1
fi
