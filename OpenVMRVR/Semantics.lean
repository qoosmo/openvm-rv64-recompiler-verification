import Init.Omega

namespace OpenVMRVR

/-!
Structural model of the five RV64 register-register W expressions emitted by
the pinned OpenVM source. `Spec` gives independent architectural definitions.
`CExpr` follows the emitted C casts and operators in source order.

Only low-level fixed-width word operations are shared. In particular, no
instruction definition in either namespace calls an instruction definition in
the other namespace.
-/

abbrev Word64 := BitVec 64
abbrev Word32 := BitVec 32

/-!
The two fields ending in `_choice` are the implementation-defined C11 choices
needed by the pinned expressions. They are theorem inputs, not axioms.

`uint32ToInt32` records the result representation of a `uint32_t` to `int32_t`
conversion. `negativeRightShift` records right shift of a negative `int32_t`.
Behavior for a nonnegative signed operand is defined by C's ordinary quotient
semantics in `CExpr.int32RightShift`; it is not an implementation assumption.
-/
structure CImplementation where
  uint32ToInt32 : Word32 → Word32
  uint32ToInt32_twosComplement_choice :
    ∀ bits, uint32ToInt32 bits = bits
  negativeRightShift : Word32 → Nat → Word32
  negativeRightShift_arithmetic_choice :
    ∀ bits amount, bits.msb = true →
      negativeRightShift bits amount = bits.sshiftRight amount

namespace Spec

/-! Independent RV64 architectural word semantics. -/

def lowWord (x : Word64) : Word32 := x.truncate 32

def shiftAmount (y : Word64) : Nat := (y.truncate 5).toNat

def finish (result : Word32) : Word64 := result.signExtend 64

def spec_addw (x y : Word64) : Word64 :=
  finish (lowWord x + lowWord y)

def spec_subw (x y : Word64) : Word64 :=
  finish (lowWord x - lowWord y)

def spec_sllw (x y : Word64) : Word64 :=
  finish ((lowWord x).shiftLeft (shiftAmount y))

def spec_srlw (x y : Word64) : Word64 :=
  finish ((lowWord x).ushiftRight (shiftAmount y))

def spec_sraw (x y : Word64) : Word64 :=
  finish ((lowWord x).sshiftRight (shiftAmount y))

end Spec

namespace CExpr

/-!
Typed operator/cast building blocks for the exact emitted expression shape.
Unsigned `BitVec` operators supply C's width-aware modulo and shift semantics.
-/

def castUint32 (x : Word64) : Word32 := x.truncate 32

def bitAndUint32 (x y : Word32) : Word32 := x &&& y

def shiftMask (y : Word64) : Word32 :=
  bitAndUint32 (castUint32 y) (BitVec.ofNat 32 0x1f)

def addUint32 (x y : Word32) : Word32 := x + y

def subUint32 (x y : Word32) : Word32 := x - y

def shiftLeftUint32 (x amount : Word32) : Word32 :=
  x.shiftLeft amount.toNat

def shiftRightUint32 (x amount : Word32) : Word32 :=
  x.ushiftRight amount.toNat

def castInt32 (impl : CImplementation) (bits : Word32) : Word32 :=
  impl.uint32ToInt32 bits

def int32RightShift (impl : CImplementation) (bits amount : Word32) : Word32 :=
  if bits.msb then
    impl.negativeRightShift bits amount.toNat
  else
    bits.ushiftRight amount.toNat

/-! Conversion from `int32_t` to `uint32_t` is reduction modulo 2^32. -/
def castUint32FromInt32 (bits : Word32) : Word32 := bits

/-!
The final emitted `(uint64_t)(int32_t)(inner)`: the first cast uses the explicit
implementation choice; conversion of that signed value to `uint64_t` is the
standard modulo conversion, represented by sign extension of its 32-bit
two's-complement representation.
-/
def castInt32ThenUint64 (impl : CImplementation) (inner : Word32) : Word64 :=
  (castInt32 impl inner).signExtend 64

def emit_addw (impl : CImplementation) (lhs rhs : Word64) : Word64 :=
  castInt32ThenUint64 impl (addUint32 (castUint32 lhs) (castUint32 rhs))

def emit_subw (impl : CImplementation) (lhs rhs : Word64) : Word64 :=
  castInt32ThenUint64 impl (subUint32 (castUint32 lhs) (castUint32 rhs))

def emit_sllw (impl : CImplementation) (lhs rhs : Word64) : Word64 :=
  castInt32ThenUint64 impl
    (shiftLeftUint32 (castUint32 lhs) (shiftMask rhs))

def emit_srlw (impl : CImplementation) (lhs rhs : Word64) : Word64 :=
  castInt32ThenUint64 impl
    (shiftRightUint32 (castUint32 lhs) (shiftMask rhs))

def emit_sraw (impl : CImplementation) (lhs rhs : Word64) : Word64 :=
  castInt32ThenUint64 impl
    (castUint32FromInt32
      (int32RightShift impl (castInt32 impl (castUint32 lhs)) (shiftMask rhs)))

end CExpr

/-! Genuine word-level bridges between the independent models. -/

theorem shiftMask_eq_lowFive (y : Word64) :
    CExpr.shiftMask y = (y.truncate 5).zeroExtend 32 := by
  apply BitVec.eq_of_getLsbD_eq
  intro i
  unfold CExpr.shiftMask CExpr.bitAndUint32 CExpr.castUint32
  intro hi
  have maskBit : Nat.testBit 31 i = decide (i < 5) :=
    calc
      Nat.testBit 31 i = Nat.testBit (2 ^ 5 - 1) i :=
        congrArg (fun n => Nat.testBit n i) (by decide)
      _ = decide (i < 5) := Nat.testBit_two_pow_sub_one 5 i
  simp [hi]
  rw [← BitVec.getLsbD_eq_getElem hi, BitVec.getLsbD_ofNat, maskBit]
  simp [hi, Bool.and_comm]

theorem shiftMask_toNat_eq_spec (y : Word64) :
    (CExpr.shiftMask y).toNat = Spec.shiftAmount y := by
  rw [shiftMask_eq_lowFive]
  simp [Spec.shiftAmount]
  omega

theorem int32RightShift_eq_arithmetic
    (impl : CImplementation) (bits amount : Word32) :
    CExpr.int32RightShift impl bits amount =
      bits.sshiftRight amount.toNat := by
  unfold CExpr.int32RightShift
  cases hmsb : bits.msb
  · exact (BitVec.sshiftRight_eq_of_msb_false hmsb).symm
  · exact impl.negativeRightShift_arithmetic_choice bits amount.toNat hmsb

open CExpr Spec

/-!
The five final preservation theorems are conditional on an explicit
`CImplementation`; its laws expose, rather than hide, the two
implementation-defined C choices.
-/

theorem emit_addw_eq_spec_addw (impl : CImplementation) (x y : Word64) :
    emit_addw impl x y = spec_addw x y := by
  simp [emit_addw, spec_addw, castInt32ThenUint64, castInt32,
    impl.uint32ToInt32_twosComplement_choice, addUint32, castUint32,
    Spec.finish, Spec.lowWord]

theorem emit_subw_eq_spec_subw (impl : CImplementation) (x y : Word64) :
    emit_subw impl x y = spec_subw x y := by
  simp [emit_subw, spec_subw, castInt32ThenUint64, castInt32,
    impl.uint32ToInt32_twosComplement_choice, subUint32, castUint32,
    Spec.finish, Spec.lowWord]

theorem emit_sllw_eq_spec_sllw (impl : CImplementation) (x y : Word64) :
    emit_sllw impl x y = spec_sllw x y := by
  simp [emit_sllw, spec_sllw, castInt32ThenUint64, castInt32,
    impl.uint32ToInt32_twosComplement_choice, shiftLeftUint32, castUint32,
    Spec.finish, Spec.lowWord, shiftMask_toNat_eq_spec]

theorem emit_srlw_eq_spec_srlw (impl : CImplementation) (x y : Word64) :
    emit_srlw impl x y = spec_srlw x y := by
  simp [emit_srlw, spec_srlw, castInt32ThenUint64, castInt32,
    impl.uint32ToInt32_twosComplement_choice, shiftRightUint32, castUint32,
    Spec.finish, Spec.lowWord, shiftMask_toNat_eq_spec]

theorem emit_sraw_eq_spec_sraw (impl : CImplementation) (x y : Word64) :
    emit_sraw impl x y = spec_sraw x y := by
  simp [emit_sraw, spec_sraw, castInt32ThenUint64, castInt32,
    impl.uint32ToInt32_twosComplement_choice, castUint32FromInt32,
    int32RightShift_eq_arithmetic, castUint32, Spec.finish, Spec.lowWord,
    shiftMask_toNat_eq_spec]

end OpenVMRVR
