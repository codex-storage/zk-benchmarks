use methods::{
    SHA256_ELF, SHA256_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
// use rand::RngCore;
use std::time::Instant;

pub fn sha_bench(input: Vec<u8>) {
   
    let start_time = Instant::now();

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, SHA256_ELF).unwrap();

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();
   
    // verify your receipt
    receipt.verify(SHA256_ID).unwrap();

    let elapsed_time = start_time.elapsed();
    eprintln!("Total time: {:?}", elapsed_time);
}