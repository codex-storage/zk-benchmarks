use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;

use crate::arithmetic::u64_arithmetic::U64Target;
use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;

const KECCAK_ROUNDS: usize = 24;

const ROUND_CONSTANTS: [u64; KECCAK_ROUNDS] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

//iota
pub fn iota<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5],
    round: usize
){

    let lo = builder.constant_u32((ROUND_CONSTANTS[round] & 0xFFFFFFFF) as u32);
    let hi = builder.constant_u32(((ROUND_CONSTANTS[round] >> 32)& 0xFFFFFFFF) as u32);
    state[0][0] = builder.xor_u64(state[0][0], U64Target([lo,hi])) ;
}