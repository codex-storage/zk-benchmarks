use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use crate::arithmetic::u32_arithmetic::{rotate32, shift32};
use crate::arithmetic::u32_arithmetic::u32_to_bits_target;
use crate::arithmetic::u32_arithmetic::bits_to_u32_target;
use super::xor3::xor3;

pub fn sigma0<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
) -> U32Target {
    let mut a_bits = u32_to_bits_target::<F, D, 2>(builder, a);
    a_bits.push(builder.constant_bool(false));
    let rotate7 = rotate32(7);
    let rotate18 = rotate32(18);
    let shift3 = shift32(3);
    let mut res_bits = Vec::new();
    for i in 0..32 {
        res_bits.push(xor3(
            builder,
            a_bits[rotate7[i]],
            a_bits[rotate18[i]],
            a_bits[shift3[i]],
        ));
    }
    bits_to_u32_target(builder, res_bits)
}

pub fn sigma1<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
) -> U32Target {
    let mut a_bits = u32_to_bits_target::<F, D, 2>(builder, a);
    a_bits.push(builder.constant_bool(false));
    let rotate17 = rotate32(17);
    let rotate19 = rotate32(19);
    let shift10 = shift32(10);
    let mut res_bits = Vec::new();
    for i in 0..32 {
        res_bits.push(xor3(
            builder,
            a_bits[rotate17[i]],
            a_bits[rotate19[i]],
            a_bits[shift10[i]],
        ));
    }
    bits_to_u32_target(builder, res_bits)
}

//#define Sigma0(x)    (ROTATE((x), 2) ^ ROTATE((x),13) ^ ROTATE((x),22))
pub fn big_sigma0<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
) -> U32Target {
    let a_bits = u32_to_bits_target::<F, D, 2>(builder, a);
    let rotate2 = rotate32(2);
    let rotate13 = rotate32(13);
    let rotate22 = rotate32(22);
    let mut res_bits = Vec::new();
    for i in 0..32 {
        res_bits.push(xor3(
            builder,
            a_bits[rotate2[i]],
            a_bits[rotate13[i]],
            a_bits[rotate22[i]],
        ));
    }
    bits_to_u32_target(builder, res_bits)
}

pub fn big_sigma1<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &U32Target,
) -> U32Target {
    let a_bits = u32_to_bits_target::<F, D, 2>(builder, a);
    let rotate6 = rotate32(6);
    let rotate11 = rotate32(11);
    let rotate25 = rotate32(25);
    let mut res_bits = Vec::new();
    for i in 0..32 {
        res_bits.push(xor3(
            builder,
            a_bits[rotate6[i]],
            a_bits[rotate11[i]],
            a_bits[rotate25[i]],
        ));
    }
    bits_to_u32_target(builder, res_bits)
}