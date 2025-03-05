use plonky2::iop::target::BoolTarget;
use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;

pub fn add_u32<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
    b: &U32Target,
) -> U32Target {
    let (res, _carry) = builder.add_u32(*a, *b);
    res
}

pub fn u32_to_bits_target<F: RichField + Extendable<D>, const D: usize, const B: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
) -> Vec<BoolTarget> {
    let mut res = Vec::new();
    let bit_targets = builder.split_le_base::<B>(a.0, 32);
    for i in (0..32).rev() {
        res.push(BoolTarget::new_unsafe(bit_targets[i]));
    }
    res
}

pub fn bits_to_u32_target<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    bits_target: Vec<BoolTarget>,
) -> U32Target {
    let bit_len = bits_target.len();
    assert_eq!(bit_len, 32);
    U32Target(builder.le_sum(bits_target[0..32].iter().rev()))
}

// x>>y
// Assume: 0 at index 32
pub fn shift32(y: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for _ in 32 - y..32 {
        res.push(32);
    }
    for i in 0..32 - y {
        res.push(i);
    }
    res
}

// define ROTATE(x, y)  (((x)>>(y)) | ((x)<<(32-(y))))
pub fn rotate32(y: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 32 - y..32 {
        res.push(i);
    }
    for i in 0..32 - y {
        res.push(i);
    }
    res
}