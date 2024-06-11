use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;

use crate::arithmetic::u64_arithmetic::U64Target;

//rho
pub fn rho<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
){
    const RHO_OFFSETS: [[usize; 5]; 5] = [
        [0,  1,  62, 28, 27],
        [36, 44,  6, 55, 20],
        [3, 10, 43, 25, 39],
        [41, 45, 15, 21,  8],
        [18, 2,  61, 56, 14],
    ];

    for x in 0..5 {
        for y in 0..5 {
            let rotation = RHO_OFFSETS[x][y];
            state[x][y] = builder.rotate_left_u64(state[x][y], rotation as u8);
        }
    }
}