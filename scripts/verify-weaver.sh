#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

command -v weaver >/dev/null 2>&1 || {
  echo 'WEAVER=UNSUPPORTED: weaver is not installed' >&2
  exit 69
}

weaver --version

# The replacement registry is the admitted semantic source. Validate it first,
# then require all committed projections to replay from that same source.
bash scripts/verify-semconv-v2.sh

out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

# Weaver's --templates value is the template root. The registry mode and rust
# target resolve beneath templates/registry/rust.
weaver registry generate \
  --registry semantic-conventions-v2/ \
  --templates templates \
  rust \
  "$out"

for projection in attributes.rs span_builders.rs metrics.rs; do
  generated="$out/$projection"
  committed="src/generated/$projection"

  test -f "$generated" || {
    echo "WEAVER=BUILD_BROKEN: missing generated projection $projection from v2 registry" >&2
    exit 1
  }

  if ! cmp -s "$generated" "$committed"; then
    echo "WEAVER=BUILD_BROKEN: v2 projection drift in $committed" >&2
    diff -u "$committed" "$generated" || true
    exit 1
  fi
done

echo 'WEAVER=ALIVE[V2_REPLAY]'
