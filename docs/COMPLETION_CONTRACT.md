# SwarmSH v2 Completion Contract

SwarmSH v2 is complete only when the exact admitted source revision has passed the executable gates in `scripts/verify-completion.sh`. Documentation, prior green badges, generated status files, workflow definitions, and successful inspection are not substitutes for execution.

## Standing vocabulary

`UNKNOWN|PARTIAL_ALIVE|ALIVE|BLOCKED|BUILD_BROKEN|UNSUPPORTED`

The project may be called `ALIVE` only for the subject actually executed. A passing library test suite does not imply binary, shell-export, semantic-convention, AI-provider, deployment, or performance standing.

## Admission

The admitted subject is the tuple:

- repository identity;
- exact Git commit SHA;
- `Cargo.lock` identity;
- Rust toolchain identity;
- feature set;
- verifier revision;
- relevant environment/configuration.

Moving any element creates a new subject and invalidates reuse unless equivalence is demonstrated.

## Required gates

1. Formatting is deterministic under `cargo fmt --all -- --check`.
2. Every Cargo target compiles with all features under the pinned lockfile.
3. Clippy accepts every target and feature with warnings denied.
4. Library tests pass.
5. Integration tests pass.
6. `tests/completion_concurrency.rs` executes real concurrent `WorkQueue::get_work_for_agent` calls and proves that all admitted work is claimed exactly once.
7. Rust documentation builds with warnings denied.
8. Shell export is manufactured by the real exporter and every required generated shell file passes `bash -n` plus structural contract checks.
9. Completion-path scripts contain no `|| true` masking.
10. Published Cargo metadata identifies the real repository and does not encode unsupported theorem/marketing language as a package fact.

## Explicit exclusions

The completion gate does **not** manufacture proof of:

- global mathematical conflict-freedom for every possible external filesystem/runtime;
- performance targets that were not benchmarked in the admitted environment;
- production availability of Claude, Ollama, Jaeger, Prometheus, OTLP, Kubernetes, or any external service;
- semantic-convention conformance unless the exact Weaver validator/generator executes successfully;
- shell/Rust semantic equivalence beyond the behaviors explicitly exercised;
- release publication, deployment, merge, or production authority.

Those subjects require their own gates and receipts.

## Falsifiers

Any of the following immediately removes `ALIVE` standing for the affected subject:

- a required completion command exits non-zero;
- a work item is claimed zero times or more than once in the concurrent acceptance test;
- a required shell artifact is absent, empty, or syntactically invalid;
- a completion workflow masks a failed command;
- `Cargo.lock`, toolchain, features, verifier, or source SHA changes without replay;
- a generated/provenance claim cannot be reproduced from its declared generator;
- a production or mathematical claim is broader than the evidence in the associated receipt.

## Receipt minimum

A release/completion receipt records repository, base SHA, head SHA, verifier command, toolchain, lockfile identity, executed gates, exit codes, known exclusions, and resulting scoped standing.
