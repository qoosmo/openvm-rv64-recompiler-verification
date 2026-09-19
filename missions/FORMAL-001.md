# FORMAL-001 — Lean semantic preservation for RV64 W operations

## Goal

Create a machine-checked Lean model proving semantic equivalence between:

A. independent RV64 W-operation semantics, and
B. a faithful abstract model of the C expression emitted by the pinned OpenVM RVR code

for ADDW, SUBW, SLLW, SRLW, and SRAW.

## Required theorems

For all 64-bit operands x and y:

- emit_addw x y = spec_addw x y
- emit_subw x y = spec_subw x y
- emit_sllw x y = spec_sllw x y
- emit_srlw x y = spec_srlw x y
- emit_sraw x y = spec_sraw x y

The formalization must make these operations explicit:

- truncate operand to low 32 bits;
- mask variable shift amount to low 5 bits;
- unsigned 32-bit modular add/sub/left-shift behavior;
- logical right shift;
- signed 32-bit arithmetic right shift;
- sign extension of the 32-bit result to 64 bits.

## Proof quality requirements

- No `sorry`.
- No `admit`.
- No new `axiom` declarations.
- Prefer transparent definitions and small lemmas.
- `lake build` must pass.
- Record assumptions separately from theorems.
- Do not model Clang correctness as an axiom.

## Research-quality requirement

The emitter model and RISC-V spec model must be visibly independent definitions. Avoid
defining one as an alias of the other and closing the main theorem by reflexivity.

## Completion

Run ./scripts/verify.sh and commit the passing result to the formal-agent branch.
