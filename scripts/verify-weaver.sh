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

# Weaver's --v2 switch is a boolean flag (not a key/value option). Passing a
# literal `true` shifts TARGET/OUTPUT and means generation never starts. Keep
# the flag positional contract exact so this court exercises the admitted
# registry rather than only proving schema admission.
weaver registry generate \
  --registry semantic-conventions-v2/ \
  --templates templates \
  --v2 \
  -D 'semconv_version="2.0.0"' \
  -D 'generation_timestamp="1970-01-01T00:00:00Z"' \
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
