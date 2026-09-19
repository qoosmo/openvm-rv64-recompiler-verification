//! Executable models for the five RV64 register-register W operations in CODE-001.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    Addw,
    Subw,
    Sllw,
    Srlw,
    Sraw,
}

impl Op {
    pub const ALL: [Self; 5] = [Self::Addw, Self::Subw, Self::Sllw, Self::Srlw, Self::Sraw];

    pub const fn id(self) -> u8 {
        match self {
            Self::Addw => 0,
            Self::Subw => 1,
            Self::Sllw => 2,
            Self::Srlw => 3,
            Self::Sraw => 4,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Addw => "ADDW",
            Self::Subw => "SUBW",
            Self::Sllw => "SLLW",
            Self::Srlw => "SRLW",
            Self::Sraw => "SRAW",
        }
    }
}

/// Independent architectural model: calculate the low 32-bit word, then sign-extend it.
/// SRAW's sign fill is expressed with unsigned operations rather than a signed host shift.
pub fn reference(op: Op, lhs: u64, rhs: u64) -> u64 {
    let a = lhs as u32;
    let shamt = (rhs & 0x1f) as u32;
    let word = match op {
        Op::Addw => a.wrapping_add(rhs as u32),
        Op::Subw => a.wrapping_sub(rhs as u32),
        Op::Sllw => a << shamt,
        Op::Srlw => a >> shamt,
        Op::Sraw => arithmetic_right_word(a, shamt),
    };
    sign_extend_word(word)
}

fn arithmetic_right_word(value: u32, shamt: u32) -> u32 {
    let logical = value >> shamt;
    if value & 0x8000_0000 != 0 && shamt != 0 {
        logical | (u32::MAX << (32 - shamt))
    } else {
        logical
    }
}

fn sign_extend_word(word: u32) -> u64 {
    if word & 0x8000_0000 == 0 {
        u64::from(word)
    } else {
        u64::from(word) | 0xffff_ffff_0000_0000
    }
}

/// Model of the pinned emitter's cast/operator structure. Rust's wrapping operations and
/// signed shift make explicit the behavior expected from the tested Clang target.
pub fn emitted_shape(op: Op, lhs: u64, rhs: u64) -> u64 {
    let shamt = (rhs as u32) & 0x1f;
    let inner: u32 = match op {
        Op::Addw => (lhs as u32).wrapping_add(rhs as u32),
        Op::Subw => (lhs as u32).wrapping_sub(rhs as u32),
        Op::Sllw => (lhs as u32) << shamt,
        Op::Srlw => (lhs as u32) >> shamt,
        Op::Sraw => (((lhs as u32) as i32) >> shamt) as u32,
    };
    ((inner as i32) as i64) as u64
}

pub const BOUNDARY_WORDS: [u64; 10] = [
    0,
    1,
    u64::MAX,
    0x0000_0000_7fff_ffff,
    0x0000_0000_8000_0000,
    0x0000_0000_ffff_ffff,
    0xdead_beef_0000_0000,
    0x0123_4567_7fff_ffff,
    0x89ab_cdef_8000_0000,
    0xffff_ffff_0000_0001,
];

pub const BOUNDARY_SHIFTS: [u64; 9] = [
    0,
    1,
    31,
    32,
    63,
    u64::MAX,
    0x0000_0001_0000_0000,
    0xdead_beef_0000_0020,
    0x8000_0000_0000_001f,
];

pub fn boundary_vectors() -> Vec<(Op, u64, u64)> {
    let mut vectors = Vec::new();
    for op in [Op::Addw, Op::Subw] {
        for lhs in BOUNDARY_WORDS {
            for rhs in BOUNDARY_WORDS {
                vectors.push((op, lhs, rhs));
            }
        }
    }
    for op in [Op::Sllw, Op::Srlw, Op::Sraw] {
        for lhs in BOUNDARY_WORDS {
            for rhs in BOUNDARY_SHIFTS {
                vectors.push((op, lhs, rhs));
            }
        }
    }
    vectors
}

pub const RANDOM_SEED: u64 = 0x5eed_c001_594b_044e;
pub const RANDOM_CASES_PER_OP: usize = 20_000;

pub fn random_vectors(seed: u64, cases_per_op: usize) -> Vec<(Op, u64, u64)> {
    let mut state = seed;
    let mut vectors = Vec::with_capacity(cases_per_op * Op::ALL.len());
    for op in Op::ALL {
        for _ in 0..cases_per_op {
            vectors.push((op, splitmix64(&mut state), splitmix64(&mut state)));
        }
    }
    vectors
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(vectors: impl IntoIterator<Item = (Op, u64, u64)>) {
        for (op, lhs, rhs) in vectors {
            assert_eq!(
                reference(op, lhs, rhs),
                emitted_shape(op, lhs, rhs),
                "{} lhs={lhs:#018x} rhs={rhs:#018x}",
                op.name()
            );
        }
    }

    #[test]
    fn boundary_models_agree() {
        compare(boundary_vectors());
    }

    #[test]
    fn deterministic_random_models_agree() {
        compare(random_vectors(RANDOM_SEED, RANDOM_CASES_PER_OP));
    }

    #[test]
    fn boundary_inventory_is_present() {
        for required in [0, 1, 31, 32, 63, u64::MAX] {
            assert!(BOUNDARY_SHIFTS.contains(&required));
        }
        for required in [0, 1, u64::MAX, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff] {
            assert!(BOUNDARY_WORDS.contains(&required));
        }
        assert!(
            BOUNDARY_WORDS
                .iter()
                .any(|x| x >> 32 != 0 && *x as u32 == 0)
        );
    }
}
