# FORMAL-001 question and frozen semantic contract

## Falsifiable question

At OpenVM commit `594b044e705891bcc2abd01578410da7dfcd1efe`, does a
structural model of each emitted C expression for `ADDW`, `SUBW`, `SLLW`,
`SRLW`, and `SRAW` produce exactly the independently specified RV64 W result
for every pair of 64-bit source words, conditional on explicit choices for the
two implementation-defined C11 operations used by those expressions?

## Hypothesis

All five equalities hold for the complete finite input domain when out-of-range
`uint32_t` to `int32_t` conversion preserves the two's-complement bit pattern
and negative `int32_t` right shift is arithmetic/sign-filling. The acceptable
evidence is kernel-checked conditional universal Lean theorems
(`PROVED_FORMAL`), not test enumeration or compiler execution.

## Frozen contract

- Inputs and result are unsigned 64-bit bit patterns.
- Each data operand is truncated to its low 32 bits where the pinned expression
  contains a `uint32_t` cast.
- A variable shift count is truncated to 32 bits and bitwise-ANDed with
  `0x1f`, yielding a count in `[0, 31]`.
- Addition, subtraction, and left shift use unsigned 32-bit modular arithmetic.
- `SRLW` is logical right shift of the low 32-bit unsigned value.
- `SRAW` first converts the low 32 bits to `int32_t`; the theorem explicitly
  assumes two's-complement reinterpretation and sign-filling right shift for a
  negative value.
- The low 32-bit result is converted to `int32_t` under the same explicit
  two's-complement implementation choice and then converted to `uint64_t`,
  producing architectural sign extension.
- Functions are pure and combinational: no state, reset, latency, or restricted
  legal-input assumptions.

## Baseline and budget

The immutable baseline is the clean scaffold identified by the hashes in
`FORMAL_LAB.toml`. One proof-structure intervention is permitted. Stop on a
passing full verification or after that intervention fails.
