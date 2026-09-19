#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== forbidden Lean proof escapes =="
if grep -R -nE '(^|[^A-Za-z])(sorry|admit|axiom)([^A-Za-z]|$)' \
    --include='*.lean' OpenVMRVR 2>/dev/null; then
  echo "Forbidden Lean proof escape found." >&2
  exit 1
fi

if command -v lake >/dev/null 2>&1; then
  echo "== lake build =="
  lake build
else
  echo "WARN: lake not installed; Lean build skipped." >&2
fi

if [[ -f Cargo.toml ]] && command -v cargo >/dev/null 2>&1; then
  echo "== cargo fmt/check/test =="
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo test --all-targets
fi

if [[ -x scripts/check-results.sh ]]; then
  scripts/check-results.sh
fi

echo "VERIFY=PASS"
