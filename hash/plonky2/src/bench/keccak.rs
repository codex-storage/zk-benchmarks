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
use plonky2::iop::witness::Witness;
use plonky2::field::types::PrimeField64;

fn generate_data(size: usize) -> Vec<GoldilocksField> {

    let mut data: Vec<GoldilocksField> = Vec::new();
    for _ in 0..(1<<size) {
        let mut rng = rand::thread_rng();
        let random_u64: u64 = rng.gen();
        data.push(GoldilocksField::from_canonical_u64(random_u64));
    }
    data

}

//TODO: remove the dead codes later
#[allow(dead_code)]
pub trait WitnessU64<F: PrimeField64>: Witness<F> {
    fn set_u64_target(&mut self, target: U64Target, value: u64);
    // fn get_u64_target(&self, target: U64Target) -> (u64, u64);
}

impl<T: Witness<F>, F: PrimeField64> WitnessU64<F> for T {
    fn set_u64_target(&mut self, target: U64Target, value: u64) {
        self.set_target(target.0[0].0, F::from_canonical_u32((value & 0xFFFFFFFF) as u32));
        self.set_target(target.0[1].0, F::from_canonical_u32(((value >> 32) & 0xFFFFFFFF) as u32));
    }

    // fn get_u64_target(&self, target: U64Target) -> (u64, u64) {
    //     let x_u64 = self.get_target(target.0).to_canonical_u64();
    //     let low = x_u64 as u32;
    //     let high = (x_u64 >> 32) as u32;
    //     (low, high)
    // }
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
const KECCAK_RATE: usize = 1088;
// const KECCAK_CsAPACITY: usize = KECCAK_WIDTH - KECCAK_RATE;
// const KECCAKs_LANES: usize = KECCAK_WIDTH / 64;
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
fn keccak_f<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
) {
    for i in 0..24 {
        theta(builder, state);
        rho(builder, state);
        pi(builder, state);
        chi(builder, state);
        iota(builder, state, i)
    }
}

//TODO: remove the dead codes later
#[allow(dead_code)]
fn absorb<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5],
    message: &[U64Target],
    rate: usize
) {
    let chunks = message.chunks(rate / 64);
    for block in chunks {
        for (i, &word) in block.iter().enumerate() {
            let x = i % 5;
            let y = i / 5;
            state[x][y] = builder.xor_u64(state[x][y], word);
        }
        keccak_f(builder, state); // Assume keccak_f is implemented as a circuit
    }
}

//TODO: remove the dead codes later
#[allow(dead_code)]
fn squeeze<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5],
    rate: usize,
    output_length: usize
) -> Vec<U64Target> {
    let mut hash = Vec::new();
    while hash.len() * 8 < output_length {
        for y in 0..5 {
            for x in 0..5 {
                if (x + 5 * y) * 64 < rate {
                    hash.push(state[x][y]);
                    if hash.len() * 8 >= output_length {
                        return hash;
                    }
                }
            }
        }
        keccak_f(builder, state); // Assume keccak_f is implemented as a circuit
    }
    hash
}

//TODO: remove the dead codes later
#[allow(dead_code)]
fn keccak_pad<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[U64Target],
    rate: usize
) -> Vec<U64Target> {
    let mut padded_message = message.to_vec();
    let rate_words = rate / 64;
    let mut pad_len = rate_words - (message.len() % rate_words);
    
    if pad_len == 0 {
        pad_len = rate_words;
    }

    if pad_len == 1 {
        padded_message.push(U64Target([builder.one_u32(), builder.one_u32()]));
    } else {
        padded_message.push(U64Target([builder.one_u32(), builder.zero_u32()]));
        for _ in 1..(pad_len - 1) {
            padded_message.push(U64Target([builder.zero_u32(), builder.zero_u32()]));
        }
        padded_message.push(U64Target([builder.zero_u32(), builder.one_u32()]));
    }

    padded_message
}

//TODO: remove the dead codes later
#[allow(dead_code)]
fn keccak256<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[U64Target]
) -> Vec<U64Target> {
    let mut state = [[builder.zero_u64(); 5]; 5];
    let rate = KECCAK_RATE;

    // Padding
    let padded_message = keccak_pad(builder, message, rate);

    // Absorbing
    absorb(builder, &mut state, &padded_message, rate);

    // Squeezing
    let hash = squeeze(builder, &mut state, rate, 256);

    hash
}

#[test]
fn test_keccak256() {
    // use plonky2_u32::gadgets::arithmetic_u32::U32Target;
    // use plonky2::iop::target::Target;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    // use plonky2_u32::witness::WitnessU32;
    type F = GoldilocksField;  // Choose the field used in your implementation.
    const D: usize = 2;  // This should match the extension degree used.

    // Create circuit builder.
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Example input message
    let input = b"hello";
    
    // Convert input to U64Target format used in your Keccak circuit.
    let input_u64: Vec<U64Target> = input.chunks(8)
        .map(|chunk| {
            let mut chunk_padded = [0u8; 8];
            chunk_padded[..chunk.len()].copy_from_slice(chunk);
            let value = u64::from_le_bytes(chunk_padded);
            U64Target([
                builder.constant_u32(value as u32),
                builder.constant_u32((value >> 32) as u32),
            ])
        })
        .collect();

    

    // Build the Keccak-256 circuit.
    let _ = keccak256(&mut builder, &input_u64);

    // Generate the circuit and witness.
    let data = builder.build::<PoseidonGoldilocksConfig>();
    let mut pw = PartialWitness::new();

    let input_u64_plain: Vec<u64> = input.chunks(8)
        .map(|chunk| {
            let mut chunk_padded = [0u8; 8];
            chunk_padded[..chunk.len()].copy_from_slice(chunk);
            u64::from_le_bytes(chunk_padded)
        })
        .collect();

    for (i, &byte) in input_u64_plain.iter().enumerate() {
        pw.set_u64_target(input_u64[i], byte as u64);
    }

    // Run the circuit.
    let hash_result = data.prove(pw).unwrap();

    let _ = data.verify(hash_result);

    // // Extract the hash result from the circuit output.
    // let mut output_bytes = Vec::new();
    // for target in hash_targets {
    //     let lo = hash_result.get_u32(target.0[0]) as u64;
    //     let hi = (hash_result.get_u32(target.0[1]) as u64) << 32;
    //     let combined = lo | hi;
    //     output_bytes.extend_from_slice(&combined.to_le_bytes());
    // }

    // // Truncate to 256 bits (32 bytes).
    // output_bytes.truncate(32);

    // // Compute the expected hash using a reference implementation.
    // let expected_hash = keccak256_reference(input);

    // // Compare the circuit output with the expected hash.
    // assert_eq!(output_bytes, expected_hash, "Keccak-256 hash mismatch");
    }

