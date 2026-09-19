#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DST="$ROOT/.upstream/openvm"
COMMIT="594b044e705891bcc2abd01578410da7dfcd1efe"
URL="https://github.com/openvm-org/openvm.git"

if [[ ! -d "$DST/.git" ]]; then
  mkdir -p "$(dirname "$DST")"
  git clone --filter=blob:none --no-checkout "$URL" "$DST"
fi

git -C "$DST" fetch --depth=1 origin "$COMMIT"
git -C "$DST" checkout --detach "$COMMIT"

ACTUAL="$(git -C "$DST" rev-parse HEAD)"
[[ "$ACTUAL" == "$COMMIT" ]] || {
  echo "Pinned upstream mismatch: $ACTUAL" >&2
  exit 1
}

echo "Pinned OpenVM checkout ready: $DST @ $ACTUAL"
