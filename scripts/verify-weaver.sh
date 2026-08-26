#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

command -v weaver >/dev/null 2>&1 || {
  echo 'WEAVER=UNSUPPORTED: weaver is not installed' >&2
  exit 69
}

weaver --version

# The replacement registry is now part of the completion subject. Admit it
# before attempting any legacy projection replay so schema migration failures
# cannot be hidden behind generated-code drift.
bash scripts/verify-semconv-v2.sh

out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

# Legacy replay remains required until the committed generated projections are
# demonstrably reproduced from the replacement registry. Keeping this gate
# preserves provenance while the migration is completed.
weaver registry check --registry semantic-conventions/

weaver registry generate \
  --registry semantic-conventions/ \
  --templates templates/registry \
  rust \
  "$out"

for projection in attributes.rs span_builders.rs metrics.rs; do
  generated="$out/$projection"
  committed="src/generated/$projection"

  test -f "$generated" || {
    echo "WEAVER=BUILD_BROKEN: missing generated projection $projection" >&2
    exit 1
  }

  if ! cmp -s "$generated" "$committed"; then
    echo "WEAVER=BUILD_BROKEN: stale projection $committed" >&2
    diff -u "$committed" "$generated" || true
    exit 1
  fi
done

echo 'WEAVER=ALIVE'
