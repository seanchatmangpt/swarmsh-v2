#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

fail() {
  echo "CLAIM_HYGIENE=BLOCKED: $*" >&2
  exit 1
}

# Acceptance paths must never turn a failed command into success.
if grep -nF '|| true' .github/workflows/completion.yml scripts/verify-completion.sh scripts/verify-shell-export.sh; then
  fail 'masked failure found in completion path'
fi

# Package metadata is a published interface; it must identify this repository.
grep -q '^repository = "https://github.com/seanchatmangpt/swarmsh-v2"$' Cargo.toml \
  || fail 'Cargo.toml repository URL is stale or placeholder'
grep -q '^homepage = "https://github.com/seanchatmangpt/swarmsh-v2"$' Cargo.toml \
  || fail 'Cargo.toml homepage URL is stale or placeholder'

# A package description is not a theorem. Keep proof language out of registry metadata.
if grep '^description = ' Cargo.toml | grep -Eiq 'mathematical|guarantee|revolutionary'; then
  fail 'Cargo package description contains an unscoped proof/marketing claim'
fi

# Completion claims must carry status vocabulary and falsifiers.
grep -q 'PARTIAL_ALIVE\|ALIVE\|BLOCKED\|BUILD_BROKEN\|UNSUPPORTED' docs/COMPLETION_CONTRACT.md \
  || fail 'completion contract is missing standing vocabulary'
grep -qi 'falsifier' docs/COMPLETION_CONTRACT.md \
  || fail 'completion contract is missing falsifiers'

echo 'CLAIM_HYGIENE=ALIVE'
