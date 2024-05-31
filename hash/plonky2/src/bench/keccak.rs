use plonky2::field::types::Field;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::keccak::KeccakHash;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use rand::Rng;
use plonky2::field::extension::Extendable;
use plonky2::plonk::config::Hasher;
use plonky2::hash::hash_types::RichField;
use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;
use crate::arithmetic::u64_arithmetic::U64Target;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;


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
    
    let _initial = builder.add_virtual_targets(data.len());
    
    let hash = KeccakHash::<32>::hash_or_noop(&data);
    eprintln!("{:?}", hash);

}


//----------------------------------------------------------


// const KECCAK_WIDTH: usize = 1600;
// const KECCAK_RATE: usize = 1088;
// const KECCAK_CAPACITY: usize = KECCAK_WIDTH - KECCAK_RATE;
// const KECCAK_LANES: usize = KECCAK_WIDTH / 64;
const KECCAK_ROUNDS: usize = 24;

//TODO: remove the dead codes later
#[allow(dead_code)]
const ROUND_CONSTANTS: [u64; KECCAK_ROUNDS] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

//TODO: remove the dead codes later
#[allow(dead_code)]
// Theta
pub fn theta<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
){
    let mut c = [builder.zero_u64(); 5];
    let mut d = [builder.zero_u64(); 5];

    // Compute column parities
    for x in 0..5 {

        let xor_x0_x1 = builder.xor_u64(state[x][0], state[x][1]);
        let xor_x0_x1_x2 = builder.xor_u64(xor_x0_x1, state[x][2]);
        let xor_x0_x1_x2_x3 = builder.xor_u64(xor_x0_x1_x2, state[x][3]);
        c[x] = builder.xor_u64(xor_x0_x1_x2_x3, state[x][4]);
        
    }

    // Compute rotated parities
    for x in 0..5 {
        let c_left = c[(x + 4) % 5];
        let c_right_rot = builder.rotate_left_u64(c[(x + 1) % 5], 1);
        d[x] = builder.xor_u64(c_left, c_right_rot);
    }

    // Modify the state
    for x in 0..5 {
        for y in 0..5 {
            state[x][y] = builder.xor_u64(state[x][y], d[x]);
        }
    }
}

//TODO: remove the dead codes later
#[allow(dead_code)]
//rho
fn rho<F: RichField + Extendable<D>, const D: usize>(
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

//TODO: remove the dead codes later
#[allow(dead_code)]
//pi
fn pi<F: RichField + Extendable<D>, const D: usize>(
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

//TODO: remove the dead codes later
#[allow(dead_code)]
//iota
fn iota<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5],
    round: usize
){

    let lo = builder.constant_u32((ROUND_CONSTANTS[round] & 0xFFFFFFFF) as u32);
    let hi = builder.constant_u32(((ROUND_CONSTANTS[round] >> 32)& 0xFFFFFFFF) as u32);
    state[0][0] = builder.xor_u64(state[0][0], U64Target([lo,hi])) ;
}

//TODO: remove the dead codes later
#[allow(dead_code)]
fn chi<F: RichField + Extendable<D>, const D: usize>(
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

//TODO: remove the dead codes later
#[allow(dead_code)]
// permutation
fn keccak_permutation<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
) {
    for i in 0..24 {
        theta(builder, state);
        rho(builder, state);
        pi(builder, state);
        chi(builder, state);
        iota(builder, state, ROUND_CONSTANTS[i] as usize)
    }
}