# Status

## Phase 0 — scaffold

- [x] Pin upstream OpenVM revision.
- [x] Define narrow semantic-preservation target.
- [x] Split coding and formal-verification work into isolated agent worktrees.
- [x] CODE-001: executable differential semantics harness.
- [x] FORMAL-001: Lean model + checked theorems for `ADDW/SUBW/SLLW/SRLW/SRAW`.
- [x] REVIEW-001: cross-review assumptions between code and proofs.
- [ ] CI green on a clean checkout.
- [ ] Preprint claims frozen from verified artifacts only.

## Non-goals for v0

- Whole-compiler verification.
- Verification of Clang/LLVM.
- Verification of the STARK/SWIRL proof system.
- Whole-zkVM correctness.
- Performance claims.
