use methods::{
    POSEIDON2_BN128_ELF, POSEIDON2_BN128_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use zkhash::{fields::{bn256::FpBN256, utils::random_scalar}/* , poseidon2::poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS*/};
use std::time::Instant;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};


pub fn poseidon2_bn128_bench(mt_depth: usize) {
    
    type Scalar = FpBN256;

    let mut input_scalar: Vec<Vec<u8>> = Vec::new();
    let number_of_leaves: u32 = 1 << mt_depth;
    for _ in 0..number_of_leaves {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    let env = ExecutorEnv::builder().write(&input_scalar).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, POSEIDON2_BN128_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    
    // verify your receipt
    receipt.verify(POSEIDON2_BN128_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    // For example:
    let output: Vec<u8> = receipt.journal.decode().unwrap();

    let hash_final = Scalar::deserialize_uncompressed(&*output).unwrap();

    eprintln!("Hash: {:?}", hash_final);
    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("Verification time: {:?}", elapsed_time2 - elapsed_time);

}