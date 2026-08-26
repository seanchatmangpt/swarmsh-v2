# Weaver v2 semantic-convention migration contract

## Admitted validator

The migration target is OpenTelemetry Weaver `v0.25.1`. The replacement registry uses the `definition/2` language shipped by that exact release. Current-main Weaver behavior is not admission evidence for this migration.

## Source and replacement surfaces

- `semantic-conventions/` is retained as legacy evidence until replay containment is proven.
- `semantic-conventions-v2/` is the replacement definition registry.
- Each v2 definition must preserve a repository-observed telemetry meaning while removing legacy schema constructs that the admitted Weaver rejects.
- Migration may narrow unsupported universal claims; it must not invent success, conflict freedom, economic value, or AI actuation.

## Qualification ladder

1. **S — source admitted:** a replacement definition is traceable to a legacy convention or repository runtime concept.
2. **V — schema valid:** Weaver `v0.25.1` accepts the v2 registry.
3. **R — resolved:** Weaver resolves cross-file attribute references without ambiguity.
4. **G — generated:** admitted templates generate the expected projection set from the v2 registry.
5. **P — replayed:** regenerated projections are byte-identical to the committed exact subject.
6. **ALIVE:** exact subject identity plus S/V/R/G/P all hold in one receipt.

A schema-valid v2 file is not by itself a generated-code provenance crown.

## Authority fence

Semantic conventions and generated telemetry are observation/projection surfaces. They have no ambient authority to actuate work, external AI, Git operations, commerce, deployment, or publication. Advisory AI telemetry describes observations only.

## Falsifiers

The migration remains incomplete if any of the following is observed:

- Weaver rejects `semantic-conventions-v2/` at the admitted version.
- a signal references an undefined attribute;
- the migration changes a telemetry meaning without an explicit compatibility decision;
- generated projections cannot be replayed from the exact registry and template identities;
- generated output is hand-edited to hide generator drift;
- a telemetry field is used as proof of global completion or actuation authority.
