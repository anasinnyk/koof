# Agent guide

koof generates Kubernetes operators from OpenAPI documents: `koof-gen` turns an
OpenAPI document plus overlays into a provider crate, and the `koof` runtime
reconciles its resources Crossplane-style.

## Layout

| Path | What | Edit? |
|---|---|---|
| `crates/koof/` | Runtime: controller, reconcile policy, HTTP client, references, credentials, status, metrics, `fake` API | yes |
| `crates/koof-gen/` | Generator: `spec/` (overlay extensions, schema synthesis), `ir/` (extract + validate), `generator/` (Rust, CRDs, e2e) | yes |
| `crates/providers/<name>/openapi/` | OpenAPI document, overlays (`*.overlay.yaml`) and `sources` (order of application) | yes, this is where resources are defined |
| `crates/providers/<name>/src/bin/` | Provider binaries `koof-<name>`, `koof-<name>-manifests`, `koof-<name>-fake` | rarely |
| `crates/providers/<name>/src/generated/` | Generated Rust | **no**, regenerate |
| `crates/providers/<name>/e2e/generated/` | Generated chainsaw tests | **no**, regenerate |
| `crates/providers/<name>/README.md`, `docs/` | Generated provider docs: kinds table, one page per kind | **no**, regenerate |
| `deploy/providers/<name>/` | Generated CRDs, RBAC, kustomization | **no**, regenerate |
| `deploy/base/` | Deployment, ServiceAccount, bindings | yes |
| `docs/` | User docs ([Diátaxis](https://diataxis.fr/)): tutorials, how-to, explanation, reference | yes, keep in sync with behaviour |

A change to a provider's resources goes into its overlay (`x-kube-resource`,
`x-kube-operation`, `x-kube-field`, `x-kube-provider`), followed by
`scripts/regen.sh <provider>`. Extension keys are camelCase and unknown keys are
rejected; `crates/koof-gen/README.md` documents them.

## Commands

```sh
scripts/check.sh               # everything CI checks except e2e; run before committing
scripts/regen.sh [provider]    # regenerate code, e2e tests and manifests
scripts/check-generated.sh     # fail if generated files differ from a fresh regeneration
scripts/e2e.sh [provider]      # kind cluster + controller + fake API + chainsaw (needs kind, kubectl, chainsaw)
```

`cargo build` alone does not build the `fake` binaries; they need `--features fake`.
A full build of a provider is slow (the Cloudflare types are ~27k lines), so
prefer `cargo clippy -p koof` / `-p koof-gen` while iterating.

## Conventions

- Lints: `clippy::pedantic` is denied workspace-wide, so are `unwrap`/`expect`, `todo!`,
  `dbg!` and warnings. Functions longer than 100 lines fail clippy; split them.
- Formatting: `cargo fmt --all`. Generated modules are `#[rustfmt::skip]`.
- Comments: none beyond what lints require (`# Errors` sections on public
  fallible functions) and user-facing CRD docs. No narrating or header comments.
- Errors: `thiserror` enums; do not repeat a `#[source]`/`#[from]` error in the
  message. Render chains for users with `koof::error::chain`.
- Generator input is strict: `deny_unknown_fields` on every extension struct.
- Commits: short conventional subject (`fix: …`, `chore: …`, `refactor: …`).

## Before you finish

1. `scripts/regen.sh` if you touched `koof-gen`, an overlay or anything that
   affects CRDs, and commit the regenerated files.
2. `scripts/check.sh` passes.
