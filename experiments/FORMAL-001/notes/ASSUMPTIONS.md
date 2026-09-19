# FORMAL-001 assumptions and trust boundary

## Source assumptions

1. The source under study is OpenVM commit
   `594b044e705891bcc2abd01578410da7dfcd1efe`.
2. At that commit, `extensions/riscv/rvr/src/i/instruction.rs` formats W
   operations as (lines 351--363 at the pinned revision):
   - `(uint32_t)lhs + (uint32_t)rhs`;
   - `(uint32_t)lhs - (uint32_t)rhs`;
   - `(uint32_t)lhs << ((uint32_t)rhs & 0x1fu)`;
   - `(uint32_t)lhs >> ((uint32_t)rhs & 0x1fu)`;
   - `(uint32_t)((int32_t)(uint32_t)lhs >> ((uint32_t)rhs & 0x1fu))`;
   followed by `(uint64_t)(int32_t)(inner)`.
3. `uint32_t`, `int32_t`, and `uint64_t` exist with widths 32, 32, and 64.
4. C11 leaves conversion from `uint32_t` to `int32_t` implementation-defined
   when the unsigned value is outside the signed range. `CImplementation` makes
   the choice explicit and the five theorems require its
   `uint32ToInt32_twosComplement_choice` law: the implementation preserves the
   32-bit pattern and interprets it as two's complement. This applies both to
   the SRAW operand cast and to the final `(int32_t)(inner)` cast.
5. C11 leaves right shift of a negative signed integer implementation-defined.
   `CImplementation.negativeRightShift_arithmetic_choice` explicitly requires
   sign-filling arithmetic right shift for negative `int32_t` values. Right
   shift of nonnegative values is modeled directly and is not listed as an
   implementation-defined assumption.
6. The emitted bitwise `& 0x1fu` is modeled as 32-bit bitwise AND, not as a
   pre-normalized remainder. A proved bridge lemma establishes that its numeric
   value equals the independent architectural five-bit shift amount, so every
   modeled shift count is in `[0, 31]`.
7. Unsigned 32-bit addition, subtraction, left shift, and logical right shift
   use fixed-width `BitVec` operations. Their wraparound and shift behavior is
   well-defined C behavior, not an implementation assumption.

## Conditional theorem proved by Lean

For every `CImplementation` satisfying the two choice laws above and every pair
of 64-bit operands, Lean proves that each structural model of the exact pinned
ADDW, SUBW, SLLW, SRLW, and SRAW emitted C expression equals its independent
RV64 architectural specification. The architectural instruction definitions do
not call emitter definitions, and emitter instruction definitions do not call
architectural instruction definitions.

## What the theorems do not assume or prove

- No compiler-correctness axiom is introduced.
- The Lean proof does not establish C compiler correctness or that Clang or
  Apple ARM64 executes the source model correctly. CODE-001 separately provides
  executable evidence for the concrete compiler/toolchain.
- The proof does not cover decoding, register allocation, memory/register I/O,
  the full native recompiler, OpenVM as a whole, or zkVM correctness.
- Public source inspection supplies provenance only. The Lean kernel decides
  the stated conditional mathematical equalities.
