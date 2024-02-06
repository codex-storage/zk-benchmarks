use benchmark_methods::{
    BLAKE3_ELF, BLAKE3_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
// use rand::RngCore;
use std::time::Instant;

pub fn blake3_bench(input: Vec<u8>) {
   
    

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();
    let start_time = Instant::now();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, BLAKE3_ELF).unwrap();

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();

    let elapsed_time1 = start_time.elapsed();
    // verify your receipt
    receipt.verify(BLAKE3_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("verification time: {:?}", elapsed_time2 -  elapsed_time1);

}