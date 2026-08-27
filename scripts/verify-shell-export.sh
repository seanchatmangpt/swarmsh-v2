#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

out="${TMPDIR:-/tmp}/swarmsh-v2-shell-export-$$"
trap 'rm -rf "$out"' EXIT

cargo run --locked --all-features --bin swarmsh-exporter -- full --output "$out"

required=(
  coordination_helper.sh
  agent_swarm_orchestrator.sh
  real_agent_coordinator.sh
  telemetry_spans.sh
  health_monitor.sh
  8020_automation.sh
)

for script in "${required[@]}"; do
  path="$out/$script"
  test -s "$path"
  bash -n "$path"
done

grep -Eq 'register_agent|claim_work|coordinate' "$out/coordination_helper.sh"
grep -Eq 'agent|work|coordination' "$out/agent_swarm_orchestrator.sh"

echo "SHELL_EXPORT_SYNTAX=ALIVE"
echo "SHELL_EXPORT_DIRECTORY=$out"
