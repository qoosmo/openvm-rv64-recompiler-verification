#!/usr/bin/env bash
set -euo pipefail
WT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/openvm-rv64-recompiler-verification-formal}"
[[ -d "$WT" ]] || { echo "Formal worktree not found: $WT" >&2; exit 1; }
command -v opencode >/dev/null || { echo "opencode not found" >&2; exit 1; }

STAMP="$(date +%Y%m%d-%H%M%S)"
mkdir -p "$WT/logs"
PROMPT="$(cat "$WT/missions/FORMAL-001.md")

Read AGENTS.md first. Work only in this worktree. Inspect the pinned upstream source using
scripts/fetch-upstream.sh when needed. Do not use sorry/admit/axiom. Run scripts/verify.sh.
Commit the branch only when deterministic verification passes."

opencode run \
  --agent formal-lab \
  --dir "$WT" \
  --auto \
  --title "OpenVM RV64 FORMAL-001" \
  "$PROMPT" | tee "$WT/logs/formal-001-$STAMP.log"
