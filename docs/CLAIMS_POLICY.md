# Evidence and Claims Policy

SwarmSH v2 separates implementation facts, executed evidence, and aspirations.

## Claim classes

### Executed

A statement about behavior is `Executed` only when the exact admitted revision exercised that behavior. The receipt must identify the command and subject.

### Verified

A statement is `Verified` when a deterministic verifier checked the property on the admitted subject and the verifier itself is identified.

### Inferred

An `Inferred` statement follows from inspected implementation or surrounding evidence but was not executed directly. Inference must never be upgraded to execution evidence.

### Unsupported

An `Unsupported` statement has no adequate local proof. Unsupported is not refused and does not imply the feature can never be built.

### Aspirational

Roadmap and target language is allowed when explicitly marked as a target and kept separate from current standing.

## Restricted language

The following language requires a named proof surface or bounded scope:

- mathematically guaranteed;
- zero conflict in all environments;
- production ready;
- complete parity;
- 100% coverage/compliance;
- revolutionary;
- universal;
- fully autonomous.

Package metadata and executable startup banners should prefer bounded factual descriptions over superlatives.

## Generated-code provenance

A file is called generated only if all of the following are known:

1. generator identity/version;
2. source specification identity;
3. generator configuration/template identity;
4. deterministic output path;
5. replay command;
6. reproduction check or diff.

A `DO NOT EDIT` comment does not establish generation provenance.

## Shell-export claims

`bash -n` proves syntax only. Structural greps prove only the named structural contract. Behavioral parity requires executing equivalent Rust and shell scenarios against the same admitted fixture and comparing outcomes.

## Coordination claims

Concurrent uniqueness for a bounded `WorkQueue` scenario supports the claim that the tested implementation did not duplicate admitted work under that scenario. It is not a universal mathematical proof over every filesystem, process topology, failure model, or external runtime.

## External systems

Claude, Ollama, OpenTelemetry backends, Weaver, Kubernetes, and cloud services are separate subjects. Optional integration code compiling does not imply those external systems were reachable or correctly configured.
