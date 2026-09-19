# Scope and trust boundary

## Claim we want to establish

For each selected RV64I W instruction and for all 64-bit register operands, the value
computed by the modeled OpenVM emitted expression equals the RV64 architectural result.

## Layers

### A. Architectural semantics
Independent RV64 W-instruction specification.

### B. Emitter semantics
A faithful model of the C expression emitted by the pinned OpenVM RVR source.

### C. Executable validation
Compile/run generated C expressions with the compiler configuration used by the harness
and compare against the independent reference semantics over:
- adversarial boundary vectors;
- randomized vectors;
- exhaustive reduced-width analogues where useful.

### D. Lean theorem
Prove A = B in the Lean model.

## Explicit limitations

A proof of B against the Lean C-expression model is not a proof of arbitrary C compiler
correctness. Where C behavior depends on implementation/language-version semantics, the
assumption must be isolated, documented, and tested against the pinned compiler
configuration.

The project must distinguish:
- language-standard semantics;
- compiler-specific behavior;
- OpenVM emitter behavior;
- RISC-V architectural semantics.
