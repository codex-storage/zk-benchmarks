use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;

use crate::arithmetic::u64_arithmetic::U64Target;

//pi
pub fn pi<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
){
    let mut new_state = [[builder.zero_u64(); 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            new_state[(2 * x + 3 * y) % 5][y] = state[x][y];
        }
    }
    *state = new_state;
}