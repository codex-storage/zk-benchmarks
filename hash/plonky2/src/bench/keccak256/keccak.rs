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
use super::{chi::chi, iota::iota, pi::pi, rho::rho, theta::theta};
fn generate_data(size: usize) -> Vec<GoldilocksField> {

    let mut data: Vec<GoldilocksField> = Vec::new();
    for _ in 0..(1<<size) {
        let mut rng = rand::thread_rng();
        let random_u64: u64 = rng.gen();
        data.push(GoldilocksField::from_canonical_u64(random_u64));
    }
    data

}


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
    //     let x_u64 = self.get_target(target.0[0].0).to_canonical_u64();
    //     let y_u64 = self.get_target(target.0[1].0).to_canonical_u64();


    //     (x_u64, y_u64)
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
// const KECCAK_CAPACITY: usize = KECCAK_WIDTH - KECCAK_RATE;
// const KECCAK_LANES: usize = KECCAK_WIDTH / 64;

// permutation
fn keccak_f<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
) {
    for i in 0..24 {
        let _ = theta(builder, state);
        let _ = rho(builder, state);
        let _ = pi(builder, state);
        let _ = chi(builder, state);
        let _ = iota(builder, state, i);
    }
}


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
    use rand::RngCore;

    // use plonky2_u32::witness::WitnessU32;
    type F = GoldilocksField;  // Choose the field used in your implementation.
    const D: usize = 2;  // This should match the extension degree used.

    // Create circuit builder.
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Example input message
    let mut input = [0u8; 8];
    let _ = rand::thread_rng().try_fill_bytes(&mut input);

    // let input = b"hello world";
    eprintln!("{:?}", input.len());
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
    
    eprintln!("{:?}", builder.num_gates());

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

    let _ = data.verify(hash_result.clone());
    
    // Extract the hash result from the circuit output.
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

    println!("{:?}",hash_result.get_public_inputs_hash()); 
}


#[cfg(test)]
mod tests {
    use super::*;
    use plonky2::field::goldilocks_field::GoldilocksField;
    // use plonky2::field::types::Field;
    use plonky2::hash::hash_types::RichField;
    use plonky2::iop::witness::{PartialWitness/* , Witness*/};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    // use plonky2_u32::gadgets::arithmetic_u32::U32Target;
    use crate::arithmetic::u64_arithmetic::U64Target;

    fn create_u64_target<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: u64
    ) -> U64Target {
        let lo = value as u32;
        let hi = (value >> 32) as u32;
        U64Target([builder.constant_u32(lo), builder.constant_u32(hi)])
    }

    #[test]
    fn test_theta_function() {
        type F = GoldilocksField;
        const D: usize = 2;

        // Create circuit builder
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Input state
        let input_state: [[U64Target; 5]; 5] = [
            [
                create_u64_target(&mut builder, 0x0000000000000001),
                create_u64_target(&mut builder, 0x0000000000000002),
                create_u64_target(&mut builder, 0x0000000000000003),
                create_u64_target(&mut builder, 0x0000000000000004),
                create_u64_target(&mut builder, 0x0000000000000005)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000006),
                create_u64_target(&mut builder, 0x0000000000000007),
                create_u64_target(&mut builder, 0x0000000000000008),
                create_u64_target(&mut builder, 0x0000000000000009),
                create_u64_target(&mut builder, 0x000000000000000A)
            ],
            [
                create_u64_target(&mut builder, 0x000000000000000B),
                create_u64_target(&mut builder, 0x000000000000000C),
                create_u64_target(&mut builder, 0x000000000000000D),
                create_u64_target(&mut builder, 0x000000000000000E),
                create_u64_target(&mut builder, 0x000000000000000F)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000010),
                create_u64_target(&mut builder, 0x0000000000000011),
                create_u64_target(&mut builder, 0x0000000000000012),
                create_u64_target(&mut builder, 0x0000000000000013),
                create_u64_target(&mut builder, 0x0000000000000014)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000015),
                create_u64_target(&mut builder, 0x0000000000000016),
                create_u64_target(&mut builder, 0x0000000000000017),
                create_u64_target(&mut builder, 0x0000000000000018),
                create_u64_target(&mut builder, 0x0000000000000019)
            ]
        ];

        // Expected output state (after theta)
        let expected_state = [
            [
                create_u64_target(&mut builder, 0x7B7B7B7B7B7B7B7B),
                create_u64_target(&mut builder, 0x8B8B8B8B8B8B8B8B),
                create_u64_target(&mut builder, 0x9B9B9B9B9B9B9B9B),
                create_u64_target(&mut builder, 0xABABABABABABABAB),
                create_u64_target(&mut builder, 0xBBBBBBBBBBBBBBBB)
            ],
            [
                create_u64_target(&mut builder, 0xCBCBCBCBCBCBCBCB),
                create_u64_target(&mut builder, 0xDBDBDBDBDBDBDBDB),
                create_u64_target(&mut builder, 0xEBEBEBEBEBEBEBEB),
                create_u64_target(&mut builder, 0xFBFBFBFBFBFBFBFB),
                create_u64_target(&mut builder, 0x0B0B0B0B0B0B0B0B)
            ],
            [
                create_u64_target(&mut builder, 0x1B1B1B1B1B1B1B1B),
                create_u64_target(&mut builder, 0x2B2B2B2B2B2B2B2B),
                create_u64_target(&mut builder, 0x3B3B3B3B3B3B3B3B),
                create_u64_target(&mut builder, 0x4B4B4B4B4B4B4B4B),
                create_u64_target(&mut builder, 0x5B5B5B5B5B5B5B5B)
            ],
            [
                create_u64_target(&mut builder, 0x6B6B6B6B6B6B6B6B),
                create_u64_target(&mut builder, 0x7B7B7B7B7B7B7B7B),
                create_u64_target(&mut builder, 0x8B8B8B8B8B8B8B8B),
                create_u64_target(&mut builder, 0x9B9B9B9B9B9B9B9B),
                create_u64_target(&mut builder, 0xABABABABABABABAB)
            ],
            [
                create_u64_target(&mut builder, 0xBBBBBBBBBBBBBBBB),
                create_u64_target(&mut builder, 0xCBCBCBCBCBCBCBCB),
                create_u64_target(&mut builder, 0xDBDBDBDBDBDBDBDB),
                create_u64_target(&mut builder, 0xEBEBEBEBEBEBEBEB),
                create_u64_target(&mut builder, 0xFBFBFBFBFBFBFBFB)
            ]
        ];

        // Run the theta function
        let mut state = input_state;
        theta(&mut builder, &mut state);

        
        // Check if the output state matches the expected state
        for x in 0..5 {
            for y in 0..5 {
                for i in 0..2 {
                    println!("Comparing: {:?} and {:?}", state[x][y].0[i].0, expected_state[x][y].0[i].0);
                    let res = builder.is_equal(state[x][y].0[i].0, expected_state[x][y].0[i].0);
                    builder.assert_bool(res);
                }
            }
        }

        println!("{:?}", builder.num_gates());
        // Build the circuit
        let data = builder.build::<PoseidonGoldilocksConfig>();

        // Create witness and prove
        let mut pw = PartialWitness::new();
        pw.set_u64_target(input_state[0][0], 0x0000000000000001);
        pw.set_u64_target(input_state[0][1], 0x0000000000000002);
        pw.set_u64_target(input_state[0][2], 0x0000000000000003);
        pw.set_u64_target(input_state[0][3], 0x0000000000000004);
        pw.set_u64_target(input_state[0][4], 0x0000000000000005);
        pw.set_u64_target(input_state[1][0], 0x0000000000000006);
        pw.set_u64_target(input_state[1][1], 0x0000000000000007);
        pw.set_u64_target(input_state[1][2], 0x0000000000000008);
        pw.set_u64_target(input_state[1][3], 0x0000000000000009);
        pw.set_u64_target(input_state[1][4], 0x000000000000000A);
        pw.set_u64_target(input_state[2][0], 0x000000000000000B);
        pw.set_u64_target(input_state[2][1], 0x000000000000000C);
        pw.set_u64_target(input_state[2][2], 0x000000000000000D);
        pw.set_u64_target(input_state[2][3], 0x000000000000000E);
        pw.set_u64_target(input_state[2][4], 0x000000000000000F);
        pw.set_u64_target(input_state[3][0], 0x0000000000000010);
        pw.set_u64_target(input_state[3][1], 0x0000000000000011);
        pw.set_u64_target(input_state[3][2], 0x0000000000000012);
        pw.set_u64_target(input_state[3][3], 0x0000000000000013);
        pw.set_u64_target(input_state[3][4], 0x0000000000000014);
        pw.set_u64_target(input_state[4][0], 0x0000000000000015);
        pw.set_u64_target(input_state[4][1], 0x0000000000000016);
        pw.set_u64_target(input_state[4][2], 0x0000000000000017);
        pw.set_u64_target(input_state[4][3], 0x0000000000000018);
        pw.set_u64_target(input_state[4][4], 0x0000000000000019);

        let proof = data.prove(pw).unwrap();
        assert!(data.verify(proof).is_ok());
    }


    #[test]
    fn test_rho_function() {
        type F = GoldilocksField;
        const D: usize = 2;

        // Create circuit builder
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Input state (example, should be taken from an authoritative source)
        let input_state = [
            [
                create_u64_target(&mut builder, 0x0000000000000001),
                create_u64_target(&mut builder, 0x0000000000000002),
                create_u64_target(&mut builder, 0x0000000000000003),
                create_u64_target(&mut builder, 0x0000000000000004),
                create_u64_target(&mut builder, 0x0000000000000005)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000006),
                create_u64_target(&mut builder, 0x0000000000000007),
                create_u64_target(&mut builder, 0x0000000000000008),
                create_u64_target(&mut builder, 0x0000000000000009),
                create_u64_target(&mut builder, 0x000000000000000A)
            ],
            [
                create_u64_target(&mut builder, 0x000000000000000B),
                create_u64_target(&mut builder, 0x000000000000000C),
                create_u64_target(&mut builder, 0x000000000000000D),
                create_u64_target(&mut builder, 0x000000000000000E),
                create_u64_target(&mut builder, 0x000000000000000F)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000010),
                create_u64_target(&mut builder, 0x0000000000000011),
                create_u64_target(&mut builder, 0x0000000000000012),
                create_u64_target(&mut builder, 0x0000000000000013),
                create_u64_target(&mut builder, 0x0000000000000014)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000015),
                create_u64_target(&mut builder, 0x0000000000000016),
                create_u64_target(&mut builder, 0x0000000000000017),
                create_u64_target(&mut builder, 0x0000000000000018),
                create_u64_target(&mut builder, 0x0000000000000019)
            ]
        ];

        // Expected state after rho (example, should be taken from an authoritative source)
        let expected_state = [
            [
                create_u64_target(&mut builder, 0x0000000000000001),
                create_u64_target(&mut builder, 0x0000000000000100),
                create_u64_target(&mut builder, 0x0000000000003000),
                create_u64_target(&mut builder, 0x0000000000040000),
                create_u64_target(&mut builder, 0x0000000000500000)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000600000),
                create_u64_target(&mut builder, 0x0000000000700000),
                create_u64_target(&mut builder, 0x0000000008000000),
                create_u64_target(&mut builder, 0x0000000009000000),
                create_u64_target(&mut builder, 0x00000000A0000000)
            ],
            [
                create_u64_target(&mut builder, 0x00000000B0000000),
                create_u64_target(&mut builder, 0x00000000C0000000),
                create_u64_target(&mut builder, 0x0000000D00000000),
                create_u64_target(&mut builder, 0x0000000E00000000),
                create_u64_target(&mut builder, 0x0000000F00000000)
            ],
            [
                create_u64_target(&mut builder, 0x0000001000000000),
                create_u64_target(&mut builder, 0x0000001100000000),
                create_u64_target(&mut builder, 0x0000001200000000),
                create_u64_target(&mut builder, 0x0000001300000000),
                create_u64_target(&mut builder, 0x0000001400000000)
            ],
            [
                create_u64_target(&mut builder, 0x0000001500000000),
                create_u64_target(&mut builder, 0x0000001600000000),
                create_u64_target(&mut builder, 0x0000001700000000),
                create_u64_target(&mut builder, 0x0000001800000000),
                create_u64_target(&mut builder, 0x0000001900000000)
            ]
        ];

        // Run the rho function
        let mut state = input_state;
        rho(&mut builder, &mut state);
        // Check if the output state matches the expected state
        for x in 0..5 {
            for y in 0..5 {
                let res = builder.is_equal(state[x][y].0[0].0, expected_state[x][y].0[0].0);
                let res2 = builder.is_equal(state[x][y].0[1].0, expected_state[x][y].0[1].0);

                builder.assert_bool(res);
                builder.assert_bool(res2);
            }
        }

        println!("{:?}", builder.num_gates());
        // Build the circuit
        let data = builder.build::<PoseidonGoldilocksConfig>();
        
        // Create witness and prove
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        assert!(data.verify(proof).is_ok());
    }


    #[test]
    fn test_keccak256_permutation() {
        type F = GoldilocksField;
        const D: usize = 2;

        // Create circuit builder
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Input state (example, should be taken from an authoritative source)
        let input_state = [
            [
                create_u64_target(&mut builder, 0x0000000000000001),
                create_u64_target(&mut builder, 0x0000000000000002),
                create_u64_target(&mut builder, 0x0000000000000003),
                create_u64_target(&mut builder, 0x0000000000000004),
                create_u64_target(&mut builder, 0x0000000000000005)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000006),
                create_u64_target(&mut builder, 0x0000000000000007),
                create_u64_target(&mut builder, 0x0000000000000008),
                create_u64_target(&mut builder, 0x0000000000000009),
                create_u64_target(&mut builder, 0x000000000000000A)
            ],
            [
                create_u64_target(&mut builder, 0x000000000000000B),
                create_u64_target(&mut builder, 0x000000000000000C),
                create_u64_target(&mut builder, 0x000000000000000D),
                create_u64_target(&mut builder, 0x000000000000000E),
                create_u64_target(&mut builder, 0x000000000000000F)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000010),
                create_u64_target(&mut builder, 0x0000000000000011),
                create_u64_target(&mut builder, 0x0000000000000012),
                create_u64_target(&mut builder, 0x0000000000000013),
                create_u64_target(&mut builder, 0x0000000000000014)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000015),
                create_u64_target(&mut builder, 0x0000000000000016),
                create_u64_target(&mut builder, 0x0000000000000017),
                create_u64_target(&mut builder, 0x0000000000000018),
                create_u64_target(&mut builder, 0x0000000000000019)
            ]
        ];

        // Expected state after keccak256 permutation (example, should be taken from an authoritative source)
        let expected_state = [
            [
                create_u64_target(&mut builder, 0xE69F6BAE986CCF06),
                create_u64_target(&mut builder, 0xDF9D77905A3546B6),
                create_u64_target(&mut builder, 0x7BFBFFF923073CEB),
                create_u64_target(&mut builder, 0xB2D9AB3E200FD999),
                create_u64_target(&mut builder, 0x1A741CAEC020555C)
            ],
            [
                create_u64_target(&mut builder, 0x56D7B52E1442C0AE),
                create_u64_target(&mut builder, 0xEBA39A0E00EF9FE9),
                create_u64_target(&mut builder, 0x2D6FF9BE61A295EE),
                create_u64_target(&mut builder, 0xC82D01AE6E142988),
                create_u64_target(&mut builder, 0xCDEDECBAD32B9246)
            ],
            [
                create_u64_target(&mut builder, 0x7AF13F3C6F6E4AF6),
                create_u64_target(&mut builder, 0xBD469F697CCF7B91),
                create_u64_target(&mut builder, 0xAB4F902ED5B9FD93),
                create_u64_target(&mut builder, 0xFC4F6A6C27E0190B),
                create_u64_target(&mut builder, 0x3D41F5EF85540B06)
            ],
            [
                create_u64_target(&mut builder, 0x2D9F050A3E1600F4),
                create_u64_target(&mut builder, 0x0ED46287DA8AA931),
                create_u64_target(&mut builder, 0xA13AD679DFAA4EA3),
                create_u64_target(&mut builder, 0x70B1D7A7C896E12A),
                create_u64_target(&mut builder, 0xA2CF9C93C5326E0D)
            ],
            [
                create_u64_target(&mut builder, 0x2CE66FBC3AC94F5B),
                create_u64_target(&mut builder, 0x4960E539C1EF3BA7),
                create_u64_target(&mut builder, 0xC7C50305DF46E1BB),
                create_u64_target(&mut builder, 0xEE6FE33D998F8A8B),
                create_u64_target(&mut builder, 0x2A971ED5399DC662)
            ]
        ];

        // Run the keccak256 permutation function
        let mut state = input_state;
        keccak_f(&mut builder, &mut state);

        // Check if the output state matches the expected state
        for x in 0..5 {
            for y in 0..5 {
                let res1 = builder.is_equal(state[x][y].0[0].0, expected_state[x][y].0[0].0);
                let res2 = builder.is_equal(state[x][y].0[1].0, expected_state[x][y].0[1].0);

                builder.assert_bool(res1);
                builder.assert_bool(res2);
            }
        }
        println!("{:?}", builder.num_gates());
        // Build the circuit
        let data = builder.build::<PoseidonGoldilocksConfig>();

        // Create witness and prove
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        assert!(data.verify(proof).is_ok());
    }


    #[test]
    fn test_keccak256_hash() {
        type F = GoldilocksField;
        const D: usize = 2;

        // Create circuit builder
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Input message (example, should be taken from an authoritative source)
        let input_message = vec![
            create_u64_target(&mut builder, 0x0000000000000001),
            create_u64_target(&mut builder, 0x0000000000000002),
            create_u64_target(&mut builder, 0x0000000000000003),
            create_u64_target(&mut builder, 0x0000000000000004)
        ];

        // Expected hash (example, should be taken from an authoritative source)
        let expected_hash = vec![
            create_u64_target(&mut builder, 0xA7FFC6F8BF1ED766),
            create_u64_target(&mut builder, 0x51C14756A061D662),
            create_u64_target(&mut builder, 0xF580FF4DE43B49FA),
            create_u64_target(&mut builder, 0x82D80A4B80F8434A)
        ];

        // Run the keccak256 hash function
        let hash = keccak256(&mut builder, &input_message);

        // Check if the output hash matches the expected hash
        for i in 0..expected_hash.len() {
            let res1 = builder.is_equal(hash[i].0[0].0, expected_hash[i].0[0].0);
            let res2 = builder.is_equal(hash[i].0[1].0, expected_hash[i].0[1].0);

            builder.assert_bool(res1);
            builder.assert_bool(res2);
        }

        println!("{:?}", builder.num_gates());
        // Build the circuit
        let data = builder.build::<PoseidonGoldilocksConfig>();

        // Create witness and prove
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        assert!(data.verify(proof).is_ok());
    }


}
