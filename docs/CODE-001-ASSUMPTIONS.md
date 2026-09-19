# CODE-001 semantic and compiler assumptions

## Experiment declaration

- **Question:** On the declared boundary vectors and 100,000 deterministic pseudorandom
  vectors, do the independent RV64 W model, the emitter-shape Rust model, and the
  Clang-compiled emitter expressions return the same 64-bit bit pattern?
- **Hypothesis:** All three implementations agree on every tested vector.
- **Primary metric:** mismatch count (pass criterion: zero).
- **Secondary metrics:** vectors evaluated, elapsed wall-clock time, compiler and host,
  and whether UndefinedBehaviorSanitizer reports a failure.
- **Evidence class and stopping rule:** one baseline `PASS_SIMULATION` run of all declared
  vectors, or stop immediately on the first counterexample. No repair intervention was
  budgeted for the baseline.

## Upstream evidence

This harness is scoped only to OpenVM commit
`594b044e705891bcc2abd01578410da7dfcd1efe`. The inspected source is
`extensions/riscv/rvr/src/i/instruction.rs` (SHA-256
`ad9fe1b1739243afbadc324605185e66d3f9d083cfa3e4392a258bf47713dae1`). At that
revision, lines 351–364 define `alu_expr`: lines 354–360 construct the five W-operation
inners, and line 363 applies the final signed-32 interpretation and unsigned-64 conversion.
The small C file in this repository transcribes only those five expression shapes rather
than copying the surrounding OpenVM source.

## Semantic contract

- Inputs are arbitrary 64-bit register bit patterns.
- ADDW and SUBW compute modulo 2^32; shifts use only the low five bits of `rhs`.
- SLLW and SRLW operate on the low unsigned 32-bit word. SRAW performs a sign-filling
  right shift of that word.
- Every result is the 64-bit sign extension of the resulting low 32-bit word.
- The independent Rust model implements SRAW sign fill with unsigned masks. The second
  Rust model follows the emitter's cast/operator order.

## C11 behavior isolated by the harness

The unsigned arithmetic and shifts have defined C11 behavior: operands are `uint32_t`,
and the mask keeps every shift count in 0–31. Two properties are not portable across all
C implementations:

1. Converting a `uint32_t` value greater than `INT32_MAX` to `int32_t` is
   implementation-defined when the value is not representable.
2. Right-shifting a negative `int32_t` is implementation-defined. SRAW requires an
   arithmetic (sign-filling) shift.

The compiled harness therefore checks at startup that conversion of `UINT32_MAX` gives
`-1` and that `-2 >> 1` gives `-1`. It also requires exact-width `uint32_t` and `int32_t`.
The recorded run uses the Clang version and host named in `results/code-001.json`, C11,
optimization level `-O2`, warnings as errors, and UndefinedBehaviorSanitizer. These checks
are compiler/target evidence, not verification of Clang or a claim about every conforming
C implementation.

## Evidence limits

The result is deterministic concrete differential testing (`PASS_SIMULATION`), not a
formal proof. It establishes neither all-input correctness, whole-OpenVM correctness,
nor compiler correctness. The C harness tests the expression shape in isolation; it does
not test surrounding register allocation, control flow, or generated-program execution.

The manifest identifies the pseudorandom generator as SplitMix64 and records a SHA-256
digest of the ordered text corpus sent to C. The result checker regenerates that corpus,
validates its digest, structurally parses the JSON, and rejects unknown manifest fields.
