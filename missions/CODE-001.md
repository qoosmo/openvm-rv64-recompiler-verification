# CODE-001 — Executable semantic harness

## Goal

Build an auditable executable harness for the pinned OpenVM RV64I W operations:
ADDW, SUBW, SLLW, SRLW, SRAW.

## Required outputs

1. An independent reference model for RV64 W semantics.
2. A second model matching the exact C-expression structure emitted by the pinned OpenVM
   source.
3. Differential tests between the two models.
4. A small compiled-C harness that evaluates the emitted expression shape with Clang and
   compares results with the independent model.
5. Boundary-vector tests covering:
   - 0, 1, -1 patterns;
   - 0x7fffffff, 0x80000000, 0xffffffff;
   - high 32-bit garbage;
   - shift amounts 0, 1, 31, 32, 63, and large 64-bit values.
6. Randomized testing with a deterministic seed.
7. A machine-readable result manifest in `results/`.
8. Documentation of any C implementation-defined or compiler-specific behavior found.

## Constraints

- Do not copy large OpenVM source blocks.
- Do not claim formal proof.
- Do not change Lean proof goals.
- Pin upstream evidence to commit 594b044e705891bcc2abd01578410da7dfcd1efe.
- Run ./scripts/verify.sh.
- Commit only after checks pass.
