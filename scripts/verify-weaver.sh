#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

command -v weaver >/dev/null 2>&1 || {
  echo 'WEAVER=UNSUPPORTED: weaver is not installed' >&2
  exit 69
}

weaver --version

out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

# Validate the admitted local semantic-convention registry with the same
# upstream Weaver implementation used for generation.
weaver registry check --registry semantic-conventions/

# Generate into an isolated directory. The target-specific configuration under
# templates/registry/rust/weaver.yaml is part of the admitted generator graph.
weaver registry generate \
  --registry semantic-conventions/ \
  --templates templates/registry \
  rust \
  "$out"

# Only projections with an explicit canonical Weaver template are compared.
# Any divergence means the committed projection has no current replay receipt.
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
