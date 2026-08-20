#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

command -v otel-weaver >/dev/null 2>&1 || {
  echo 'WEAVER=UNSUPPORTED: otel-weaver is not installed' >&2
  exit 69
}

otel-weaver --version
otel-weaver validate semantic-conventions/
otel-weaver generate --config weaver.yaml

# Generated projections must be reproducible. A generation command that changes
# committed output means the repository is stale, even if generation itself exits 0.
git diff --exit-code -- src/generated

echo 'WEAVER=ALIVE'
