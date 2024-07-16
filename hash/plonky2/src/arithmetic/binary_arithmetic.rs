use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::iop::target::BoolTarget;

pub fn xor_circuit<F, const D: usize>(
    a: BoolTarget,
    b: BoolTarget,
    builder: &mut CircuitBuilder<F, D>,
) -> BoolTarget
where
    F: RichField + Extendable<D>,
{
   
    // xor(a, b) = a*(1-b) + (1-a)*b = a + b - 2*ab
    let b_minus_2ab = builder.arithmetic(-F::TWO, F::ONE, a.target, b.target, b.target);
    let a_plus_b_minus_2ab = builder.add(a.target, b_minus_2ab);

    BoolTarget::new_unsafe(a_plus_b_minus_2ab)
}

pub fn xor_const_circuit<F, const D: usize>(
    a: BoolTarget,
    b: bool,
    builder: &mut CircuitBuilder<F, D>,
) -> BoolTarget
where
    F: RichField + Extendable<D>,
{
    // b = 0 => xor(a, b) = a
    // b = 1 => xor(a, b) = 1 - a = not(a)
    if b {
        builder.not(a)
    } else {
        a
    }
}

// reffered to https://github.com/polymerdao/plonky2-sha256
/// 0 -> [0, 1, 2, ..., 63]
/// 1 -> [63, 0, 1, ..., 62]
/// 63 -> [1, 2, ..., 63, 0]
pub fn rotate_u64(y: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 64 - y..64 {
        res.push(i);
    }
    for i in 0..64 - y {
        res.push(i);
    }
    res
}

#[allow(dead_code)]
pub fn from_bits_to_u64(bools: &[bool]) -> u64 {
    let mut result: u64 = 0;
    let mut shift = 0;
    for &bit in bools {
        if bit {
            result |= 1 << shift;
        }
        shift += 1;
        if shift == 64 {
            break;
        }
    }
    result
}

pub fn u64_to_bits(num: u64) -> Vec<bool> {
    let mut result = Vec::with_capacity(64);
    let mut n = num;
    for _ in 0..64 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}