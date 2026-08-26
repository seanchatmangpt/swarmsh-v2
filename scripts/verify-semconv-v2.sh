#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

command -v weaver >/dev/null 2>&1 || {
  echo 'SEMCONV_V2=UNSUPPORTED[WEAVER_MISSING]' >&2
  exit 69
}

version="$(weaver --version)"
echo "$version"
case "$version" in
  *0.25.1*) ;;
  *)
    echo "SEMCONV_V2=REFUSED[WEAVER_IDENTITY]: expected 0.25.1" >&2
    exit 1
    ;;
esac

registry="semantic-conventions-v2"
test -d "$registry"

# Replacement definitions must use Weaver's v2 language and must not smuggle
# legacy inline-signal attributes into the new registry.
python3 - <<'PY'
from pathlib import Path

root = Path('semantic-conventions-v2')
files = sorted(root.glob('*.yaml'))
assert files, 'replacement registry is empty'
for path in files:
    text = path.read_text(encoding='utf-8')
    if not text.startswith('file_format: definition/2\n'):
        raise SystemExit(f'{path}: missing definition/2 identity')
    for forbidden in ('requirement_level: optional', 'stability: experimental', 'prefix:'):
        if forbidden in text:
            raise SystemExit(f'{path}: legacy construct retained: {forbidden}')
print(f'REPLACEMENT_DEFINITIONS={len(files)}')
PY

weaver registry check --registry "$registry/"

echo 'SEMCONV_V2=ALIVE[SCHEMA_ADMISSION]'
