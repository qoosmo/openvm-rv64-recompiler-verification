#!/usr/bin/env bash
set -euo pipefail
WT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/openvm-rv64-recompiler-verification-code}"
[[ -d "$WT" ]] || { echo "Coding worktree not found: $WT" >&2; exit 1; }
command -v goose >/dev/null || { echo "goose not found" >&2; exit 1; }

STAMP="$(date +%Y%m%d-%H%M%S)"
mkdir -p "$WT/logs"
PROMPT="$(cat "$WT/missions/CODE-001.md")

Read AGENTS.md and .goosehints first. Work only in this worktree. Inspect the pinned
upstream source using scripts/fetch-upstream.sh when needed. Run scripts/verify.sh.
Commit the branch only when deterministic verification passes."

(
  cd "$WT"
  goose run \
    --text "$PROMPT" \
    --name "openvm-rv64-code-001" \
    --max-turns 80 \
    --output-format text \
    | tee "$WT/logs/code-001-$STAMP.log"
)
