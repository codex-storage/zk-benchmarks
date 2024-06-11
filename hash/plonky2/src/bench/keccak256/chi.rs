use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;

use crate::arithmetic::u64_arithmetic::U64Target;

pub fn chi<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
){
    for x in 0..5 {
        let mut temp = [builder.zero_u64(); 5];
        for y in 0..5 {
            temp[y] = state[x][y];
        }

        
        for y in 0..5 {
            let t1 = builder.not_u64(temp[(y + 1) % 5]);
            let t2 = builder.and_u64(t1, temp[(y + 2) % 5]);
            state[x][y] = builder.xor_u64(state[x][y], t2);
        }
    }
}