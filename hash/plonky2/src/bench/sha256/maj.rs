use plonky2::iop::target::BoolTarget;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;

use super::sigma::u32_to_bits_target;
use super::sigma::bits_to_u32_target;

pub fn maj<F: RichField + Extendable<D>, const D: usize>(
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
        let m = builder.mul(b_bits[i].target, c_bits[i].target);
        let two = builder.two();
        let two_m = builder.mul(two, m);
        let b_add_c = builder.add(b_bits[i].target, c_bits[i].target);
        let b_add_c_sub_two_m = builder.sub(b_add_c, two_m);
        let a_mul_b_add_c_sub_two_m = builder.mul(a_bits[i].target, b_add_c_sub_two_m);
        let res = builder.add(a_mul_b_add_c_sub_two_m, m);

        res_bits.push(BoolTarget::new_unsafe(res));
    }
    bits_to_u32_target(builder, res_bits)
}