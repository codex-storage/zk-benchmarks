#![allow(non_snake_case)]
use methods::{
    POSEIDON2_BABYBEAR_NATIVE_ELF, 
    POSEIDON2_BABYBEAR_NATIVE_ID
};
use risc0_zkvm::{
    default_prover, 
    ExecutorEnv
};
use std::time::Instant;
// use risc0_core::field::baby_bear::BabyBearElem;
use rand::Rng;

pub fn poseidon2_babybear_native_bench(mt_depth: usize) {
    
    let t = (1 << mt_depth) * 8;
    let mut input: Vec<u32> = Vec::new();

    for _ in 0..t {
        
        let mut rng = rand::thread_rng();
        let random_u32: u32 = rng.gen();
        input.push(random_u32);
    }

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, POSEIDON2_BABYBEAR_NATIVE_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // For example:
    let output: Vec<u32> = receipt.journal.decode().unwrap();

    // let mut output_deseralised: Vec<Scalar> = Vec::new();

    // for i in 0..output.len() {
    //     output_deseralised.push(Scalar::deserialize_uncompressed(&**output.get(i).unwrap()).unwrap());
    // }

    eprintln!("hash: {:?}", output);
    // let hash_final = FpBabyBear::deserialize_uncompressed(&*output).unwrap();

    // verify your receipt
    receipt.verify(POSEIDON2_BABYBEAR_NATIVE_ID).unwrap();

    
    eprintln!("Total time: {:?}", elapsed_time);
    // eprintln!("Hash: {:?}", hash_final);


}