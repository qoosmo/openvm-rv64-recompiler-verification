#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== required toolchain =="
command -v lake >/dev/null 2>&1 || {
  echo "ERROR: lake is required; refusing a false-green verification." >&2
  exit 1
}

echo "== forbidden Lean proof escapes =="
if grep -R -nE '(^|[^A-Za-z])(sorry|admit|axiom)([^A-Za-z]|$)' \
    --include='*.lean' OpenVMRVR 2>/dev/null; then
  echo "Forbidden Lean proof escape found." >&2
  exit 1
fi

echo "== lake build =="
lake build

if [[ -f Cargo.toml ]]; then
  command -v cargo >/dev/null 2>&1 || {
    echo "ERROR: Cargo.toml exists but cargo is unavailable; refusing a false-green verification." >&2
    exit 1
  }
  echo "== cargo fmt/check/test =="
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo test --all-targets
fi

if [[ -x scripts/check-results.sh ]]; then
  echo "== result manifest checks =="
  scripts/check-results.sh
fi

echo "VERIFY=PASS"
