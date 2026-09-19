# Agent Operating Rules

This repository uses isolated coding and formal-verification agents.

## Shared invariants

1. Never edit the upstream OpenVM source in place.
2. Pin all upstream claims to commit
   `594b044e705891bcc2abd01578410da7dfcd1efe`.
3. No `sorry`, `admit`, or new `axiom` declarations in Lean.
4. Do not state whole-OpenVM correctness from a W-instruction proof.
5. Every semantic assumption must be written down.
6. Prefer small independently checkable commits.
7. Run `./scripts/verify.sh` before declaring a mission complete.
8. Do not push directly to `main`.

## Coding agent

Primary mission: executable semantics, extraction, differential testing, reproducibility.

## Formal agent

Primary mission: Lean semantics and proofs. Proofs must close under the standard kernel
without `sorry`, `admit`, or custom axioms.

## Merge gate

A branch is mergeable only after:
- its own deterministic checks pass;
- the other discipline reviews its assumptions;
- the project verification script passes after integration.
