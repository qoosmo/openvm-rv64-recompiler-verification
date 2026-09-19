# Upstream snapshot

Repository: https://github.com/openvm-org/openvm.git  
Ref: develop-v2.x.0  
Pinned commit: `594b044e705891bcc2abd01578410da7dfcd1efe`

Primary file:

`extensions/riscv/rvr/src/i/instruction.rs`

At this revision, the RV64I RVR emitter constructs W-operation C expressions using:

- explicit `uint32_t` operand truncation;
- a five-bit shift mask for W shifts;
- signed arithmetic for SRAW;
- final `int32_t` interpretation followed by widening to `uint64_t`.

This repository should cite exact line-level upstream evidence in generated research notes
rather than copying large portions of upstream source.
