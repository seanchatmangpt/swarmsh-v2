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

out="$repo_root/.weaver-replay"
rm -rf "$out"
mkdir -p "$out"

# Keep the exact generated evidence on failure. CI uploads this directory so a
# stale tracked projection can only be replaced from the admitted Weaver run,
# never by hand-editing generated Rust.
weaver registry generate \
  --registry semantic-conventions-v2/ \
  --templates templates \
  --v2 \
  -D 'semconv_version="2.0.0"' \
  -D 'generation_timestamp="1970-01-01T00:00:00Z"' \
  rust \
  "$out"

status=0
for projection in attributes.rs span_builders.rs metrics.rs; do
  generated="$out/$projection"
  committed="src/generated/$projection"

  test -f "$generated" || {
    echo "WEAVER=BUILD_BROKEN: missing generated projection $projection from v2 registry" >&2
    status=1
    continue
  }

  if ! cmp -s "$generated" "$committed"; then
    echo "WEAVER=BUILD_BROKEN: v2 projection drift in $committed" >&2
    diff -u "$committed" "$generated" || true
    status=1
  fi
done

if [ "$status" -ne 0 ]; then
  sha256sum "$out"/*.rs | sort > "$out/SHA256SUMS"
  echo 'WEAVER_REPLAY_EVIDENCE=.weaver-replay'
  exit "$status"
fi

sha256sum "$out"/*.rs | sort > "$out/SHA256SUMS"
echo 'WEAVER=ALIVE[V2_REPLAY]'
