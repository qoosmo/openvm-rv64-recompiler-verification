#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CODE_WT="${ROOT}-code"
FORMAL_WT="${ROOT}-formal"

echo "Launching coding and formal agents in isolated worktrees."
"$ROOT/scripts/run-coding-agent.sh" "$CODE_WT" &
CODE_PID=$!
"$ROOT/scripts/run-formal-agent.sh" "$FORMAL_WT" &
FORMAL_PID=$!

set +e
wait "$CODE_PID"; CODE_RC=$?
wait "$FORMAL_PID"; FORMAL_RC=$?
set -e

echo "Coding agent exit: $CODE_RC"
echo "Formal agent exit: $FORMAL_RC"
[[ "$CODE_RC" -eq 0 && "$FORMAL_RC" -eq 0 ]]
