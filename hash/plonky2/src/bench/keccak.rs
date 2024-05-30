// use anyhow::Result;
use plonky2::field::types::Field;
// use plonky2::gates::poseidon::PoseidonGate;
// use plonky2::hash::hash_types::{HashOutTarget, RichField};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::keccak::{KeccakHash, KeccakPermutation, /*KeccakPermutation */};
// use plonky2::hash::keccak;
use plonky2::hash::poseidon::PoseidonHash;
// use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{/*AlgebraicHasher,*/ GenericConfig,/* PoseidonGoldilocksConfig, */ KeccakGoldilocksConfig};
use rand::Rng;
// use plonky2::iop::target::Target;
// use plonky2::iop::target::BoolTarget;
use plonky2::field::extension::Extendable;
// use std::marker::PhantomData;
use plonky2::plonk::config::Hasher;

use plonky2_u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use plonky2::field::types::PrimeField64;
use plonky2::iop::witness::Witness;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;

fn generate_data(size: usize) -> Vec<GoldilocksField> {

    let mut data: Vec<GoldilocksField> = Vec::new();
    for _ in 0..(1<<size) {
        let mut rng = rand::thread_rng();
        let random_u64: u64 = rng.gen();
        data.push(GoldilocksField::from_canonical_u64(random_u64));
    }
    data

}

// TODO: Circuit needs to be implemented
pub fn keccak_bench(_size: usize) {

    let data = generate_data(2);
    const D: usize = 2;
    type C = KeccakGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let initial = builder.add_virtual_targets(data.len());
    
    let hash = KeccakHash::<32>::hash_or_noop(&data);
    eprintln!("{:?}", hash);

}


//----------------------------------------------------------

// const KECCAK_WIDTH: usize = 1600;
// const KECCAK_RATE: usize = 1088;
// const KECCAK_CAPACITY: usize = KECCAK_WIDTH - KECCAK_RATE;
// const KECCAK_LANES: usize = KECCAK_WIDTH / 64;
// const KECCAK_ROUNDS: usize = 24;

// const ROUND_CONSTANTS: [u64; KECCAK_ROUNDS] = [
//     0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
//     0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
//     0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
//     0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
//     0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
//     0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
// ];

// fn initialize_state() -> [u64; KECCAK_LANES] {
//     [0; KECCAK_LANES]
// }
// pub struct U64Target([U32Target;2]);

// // copied from sha256 circuit
// // TODO: move to some common place
// pub fn u32_to_bits_target<F: RichField + Extendable<D>, const D: usize, const B: usize>(
//     builder: &mut CircuitBuilder<F, D>,
//     a: &U32Target,
// ) -> Vec<BoolTarget> {
//     let mut res = Vec::new();
//     let bit_targets = builder.split_le_base::<B>(a.0, 32);
//     for i in (0..32).rev() {
//         res.push(BoolTarget::new_unsafe(bit_targets[i]));
//     }
//     res
// }

// // copied from sha256 circuit
// // TODO: move to some common place
// pub fn bits_to_u32_target<F: RichField + Extendable<D>, const D: usize>(
//     builder: &mut CircuitBuilder<F, D>,
//     bits_target: Vec<BoolTarget>,
// ) -> U32Target {
//     let bit_len = bits_target.len();
//     assert_eq!(bit_len, 32);
//     U32Target(builder.le_sum(bits_target[0..32].iter().rev()))
// }

// //TODO: not tested
// pub fn xor_u64<F: RichField + Extendable<D>, const D: usize>(
//     builder: &mut CircuitBuilder<F, D>,
//     x: U64Target,
//     y: U64Target,
// ) -> U64Target {
//     let xor_x0_y0 = xor_u32(builder, x.0[0], y.0[0]);
//     let xor_x1_y1 = xor_u32(builder, x.0[1], y.0[1]);

//     U64Target([xor_x0_y0,xor_x1_y1])
    
// }

// pub fn xor_u32<F: RichField + Extendable<D>, const D: usize>(
//     builder: &mut CircuitBuilder<F, D>,
//     x: U32Target,
//     y: U32Target,
// ) -> U32Target {
    
//     let bits_target_x = u32_to_bits_target::<F, D, 2>(builder, &x);
//     let bits_target_y = u32_to_bits_target::<F, D, 2>(builder, &y);

//     assert_eq!(bits_target_x.len(), bits_target_y.len());

//     let mut xor_result_final = Vec::<BoolTarget>::new();
//     for i in 0..bits_target_x.len() {
//         let a_plus_b = builder.add(bits_target_x.get(i).unwrap().target, bits_target_y.get(i).unwrap().target);
//         let ab = builder.mul(bits_target_x.get(i).unwrap().target, bits_target_y.get(i).unwrap().target);
//         let two_ab = builder.mul_const(F::from_canonical_u64(2), ab);
//         let xor_result = builder.sub(a_plus_b, two_ab);
//         xor_result_final.push(BoolTarget::new_unsafe(xor_result));
//     }
//     let result = bits_to_u32_target(builder, xor_result_final);
//     result

// }

// Theta
// pub fn theta<F: RichField + Extendable<D>, const D: usize>(
//     builder: &mut CircuitBuilder<F, D>,
//     state: &mut [U64Target; KECCAK_LANES]
// ) {

//     let mut c = [0u64; 5];
//     for x in 0..5 {
//         c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
//     }
//     for x in 0..5 {
//         let d = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
//         for y in 0..5 {
//             state[x + 5 * y] ^= d;
//         }
//     }

// }