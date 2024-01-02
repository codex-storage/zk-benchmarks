#![allow(non_snake_case)]
use methods::{
    POSEIDON2_BABYBEAR_ELF, 
    POSEIDON2_BABYBEAR_ID
};
use risc0_zkvm::{
    default_prover, 
    ExecutorEnv
};
use zkhash::fields::{
        babybear::FpBabyBear, 
        utils::random_scalar
    };
use std::time::Instant;
use ark_serialize::{
    CanonicalSerialize, 
    CanonicalDeserialize
};


pub fn poseidon2_babybear_bench(mt_depth: usize) {
    
    type Scalar = FpBabyBear;

    let t = (1 << mt_depth) * 8;
    let mut input_scalar: Vec<Vec<u8>> = Vec::new();

    for _ in 0..t {
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
    let receipt = prover.prove_elf(env, POSEIDON2_BABYBEAR_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // For example:
    let output: Vec<Vec<u8>> = receipt.journal.decode().unwrap();

    let mut output_deseralised: Vec<Scalar> = Vec::new();

    for i in 0..output.len() {
        output_deseralised.push(Scalar::deserialize_uncompressed(&**output.get(i).unwrap()).unwrap());
    }

    eprintln!("size: {:?}", output_deseralised);
    // let hash_final = FpBabyBear::deserialize_uncompressed(&*output).unwrap();

    // verify your receipt
    receipt.verify(POSEIDON2_BABYBEAR_ID).unwrap();

    
    eprintln!("Total time: {:?}", elapsed_time);
    // eprintln!("Hash: {:?}", hash_final);


}