# SwarmSH v2 Release Readiness

This document is an operational checklist, not a release claim. The executable authority is `scripts/verify-completion.sh` and the exact-head completion workflow.

## Identity

A candidate release records repository identity, exact base/head SHAs, `Cargo.lock` identity, Rust toolchain, feature set, and verifier revision. Any source or verifier change after evidence collection requires replay.

## Core coordination

Required evidence:

- work IDs are unique at admission;
- work priorities are finite;
- incompatible agents do not consume work;
- highest-priority compatible work is selected by deterministic fallback;
- concurrent work claiming is exhaustive and duplicate-free for the admitted stress fixture;
- invalid agent identity/capacity is refused;
- external AI awaits occur outside registry and queue critical sections.

## AI boundary

Core behavior is network-independent by default. Ollama is opt-in through `SWARMSH_ENABLE_OLLAMA`; coordinator AI participation is opt-in through `SWARMSH_ENABLE_AI`. A configured Claude endpoint is represented explicitly but is not described as an implemented Claude transport.

Provider-specific standing requires a reachable endpoint, model identity, request/response fixture, timeout policy, and provider-specific execution receipt.

## Shell export

The actual exporter must manufacture a fresh output directory; required scripts must be non-empty and pass `bash -n`. These checks establish syntax/structure only, not complete Rust/shell semantic equivalence.

## OpenTelemetry Weaver

Release standing requires the current Weaver CLI to validate the registry and reproduce committed generated files without a diff. Until exact replay is green, generated-code provenance is not release-qualified.

## Binary smoke surface

Every declared binary must build. The exporter, coordinator, and agent command surfaces must execute without masked non-zero exits.

## Release decision

A candidate may move from `PARTIAL_ALIVE` to `ALIVE` only when every required completion job succeeds at the same exact head. External provider, performance, deployment, and universal mathematical claims remain separately scoped.
