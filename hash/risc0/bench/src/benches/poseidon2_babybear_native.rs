#![allow(non_snake_case)]
use benchmark_methods::{
    POSEIDON2_BABYBEAR_NATIVE_ELF, 
    POSEIDON2_BABYBEAR_NATIVE_ID
};
use risc0_zkvm::{
    default_prover, 
    ExecutorEnv
};
use std::time::Instant;
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
    let receipt = prover.prove(env, POSEIDON2_BABYBEAR_NATIVE_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // verify your receipt
    receipt.verify(POSEIDON2_BABYBEAR_NATIVE_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    let output: Vec<u32> = receipt.journal.decode().unwrap();

    eprintln!("hash: {:?}", output);
    
    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("Verification time: {:?}", elapsed_time2 - elapsed_time);

}