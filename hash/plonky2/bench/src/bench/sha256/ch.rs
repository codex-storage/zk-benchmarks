use plonky2::iop::target::BoolTarget;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use crate::arithmetic::u32_arithmetic::{bits_to_u32_target, u32_to_bits_target};


pub fn ch<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
    b: &U32Target,
    c: &U32Target,
) -> U32Target {
    let a_bits = u32_to_bits_target::<F, D, 2>(builder, a);
    let b_bits = u32_to_bits_target::<F, D, 2>(builder, b);
    let c_bits = u32_to_bits_target::<F, D, 2>(builder, c);
    let mut res_bits = Vec::new();
    for i in 0..32 {
        let b_sub_c = builder.sub(b_bits[i].target, c_bits[i].target);
        let a_mul_b_sub_c = builder.mul(a_bits[i].target, b_sub_c);
        let a_mul_b_sub_c_add_c = builder.add(a_mul_b_sub_c, c_bits[i].target);
        res_bits.push(BoolTarget::new_unsafe(a_mul_b_sub_c_add_c));
    }
    bits_to_u32_target(builder, res_bits)
}